use super::path::PROJECT_TARGET_DIR;
#[allow(unused_imports)]
use super::{
    path::PROJECT_BIN_SRC, path::PROJECT_LIB_HEADER, path::PROJECT_LIB_SRC, path::PROJECT_SRC_DIR,
    path::PROJECT_TOML, DependencyConfig, EntryConfig, PackageConfig, ProjectConfig,
    WorkSpaceConfig,
};
use crate::util;
use crate::{cmake, errors::ErrorTag};

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet};

use chrono;

use toml;

impl ProjectConfig {
    pub fn version_date() -> String {
        let local_now = chrono::Local::now();
        local_now.format("%Y.%m.%d").to_string()
    }

    pub fn validate(&self) -> bool {
        if self.workspace.is_none() {
            if self.package.is_none() {
                tracing::error!(
                    func = "self.package.is_none",
                    error_tag = ErrorTag::InvalidProjectPackageError.as_ref()
                );
                return false;
            }
        } else {
            let mut errors = vec![];
            if self.package.is_some() {
                errors.push("package");
            }
            if self.bins.is_some() {
                errors.push("bins");
            }
            if self.libs.is_some() {
                errors.push("libs");
            }
            if self.dependencies.is_some() {
                errors.push("dependencies");
            }
            if self.features.is_some() {
                errors.push("features");
            }

            if !errors.is_empty() {
                tracing::error!(
                    func = "!errors.is_empty",
                    error_tag = ErrorTag::InvalidProjectWorkspaceError.as_ref(),
                    error_str = errors.join(", ") + "conflicts",
                );
            }

            return errors.is_empty();
        }

        return true;
    }

    pub fn load(path: &str) -> Option<Self> {
        match std::fs::read_to_string(path) {
            Ok(text) => Self::loads(&text),
            Err(e) => {
                tracing::error!(
                    func = "std::fs::read_to_string",
                    path = path,
                    error_tag = ErrorTag::ReadFileError.as_ref(),
                    error_str = e.to_string(),
                );
                None
            }
        }
    }

    pub fn loads(text: &str) -> Option<Self> {
        match toml::from_str(text) {
            Ok(c) => Some(c),
            Err(e) => {
                tracing::error!(
                    func = "toml::from_str",
                    error_tag = ErrorTag::TomlDeserializeError.as_ref(),
                    error_str = e.to_string(),
                    message = text,
                );
                None
            }
        }
    }

    pub fn dump(&self, path: &str) -> bool {
        let text = self.dumps();
        if text.is_empty() {
            return false;
        }

        match std::fs::write(path, text.as_bytes()) {
            Ok(_) => true,
            Err(e) => {
                tracing::error!(
                    func = "std::fs::write",
                    path = path,
                    error_tag = ErrorTag::WriteFileError.as_ref(),
                    error_str = e.to_string(),
                    messsage = text,
                );
                false
            }
        }
    }

    pub fn dumps(&self) -> String {
        match toml::to_string_pretty(self) {
            Ok(text) => text,
            Err(e) => {
                tracing::error!(
                    func = "toml::to_string_pretty",
                    error_tag = ErrorTag::TomlSerializeError.as_ref(),
                    error_str = e.to_string(),
                );
                String::new()
            }
        }
    }

    pub fn is_project_inited(ignore: bool) -> bool {
        if util::fs::is_file_exists(PROJECT_TOML) {
            if ignore {
                tracing::warn!(
                    func = "util::fs::is_file_exists",
                    path = PROJECT_TOML,
                    error_tag = ErrorTag::FileExistsError.as_ref(),
                    message = "skip"
                );
            }
            return true;
        } else {
            if !ignore {
                tracing::error!(
                    func = "util::fs::is_file_exists",
                    path = PROJECT_TOML,
                    error_tag = ErrorTag::FileNotFoundError.as_ref(),
                    message = "please run asc init first"
                );
            }
            return false;
        }
    }

    pub fn read_project_conf() -> Option<Self> {
        Self::load(PROJECT_TOML)
    }

    pub fn write_project_conf(&self) -> bool {
        self.dump(PROJECT_TOML)
    }

    pub fn is_source_scaned() -> bool {
        if util::fs::is_file_exists(cmake::path::CMAKE_LISTS_PATH)
            && util::fs::is_dir_exists(PROJECT_TARGET_DIR)
        {
            return true;
        } else {
            tracing::error!(
                func = "util::fs::is_file_exists && util::fs::is_dir_exists",
                file = cmake::path::CMAKE_LISTS_PATH,
                dir = PROJECT_TARGET_DIR,
                error_tag = ErrorTag::PathNotFoundError.as_ref(),
                message = "please run asc scan first"
            );
            return false;
        }
    }

    pub fn get_target_name_src(
        &self,
        name: &Option<String>,
        shared_lib: bool,
        static_lib: bool,
    ) -> (String, String) {
        let mut package_name = String::new();
        if self.package.is_some() {
            package_name = self.package.as_ref().unwrap().name.clone();
        }

        if !shared_lib && !static_lib {
            // bin
            return self.get_target_name_src_inner(
                name,
                &self.bins,
                &package_name,
                &format!("{}/{}", PROJECT_SRC_DIR, PROJECT_BIN_SRC),
            );
        } else {
            // lib
            return self.get_target_name_src_inner(
                name,
                &self.libs,
                &package_name,
                &format!("{}/{}", PROJECT_SRC_DIR, PROJECT_LIB_SRC),
            );
        }
    }

    fn get_target_name_src_inner(
        &self,
        name: &Option<String>,
        entries: &Option<BTreeSet<EntryConfig>>,
        default_name: &str,
        default_path: &str,
    ) -> (String, String) {
        // no bins and libs, use package
        if entries.is_none() || entries.as_ref().unwrap().is_empty() {
            if default_name.is_empty() {
                return (String::new(), String::new());
            }
            return (default_name.to_string(), default_path.to_string());
        } else {
            // try to use bins/libs
            if name.is_none() {
                return (String::new(), String::new());
            }
            let name = name.as_ref().unwrap();
            // validate name
            if name.is_empty() {
                return (String::new(), String::new());
            }
            // validate bins/libs
            let mut path = String::new();
            for entry in entries.as_ref().unwrap() {
                if &entry.name == name {
                    path = entry.path.clone();
                    break;
                }
            }
            if path.is_empty() {
                return (String::new(), String::new());
            }

            return (name.clone(), path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT_WORKSPACE: &str = r#"[workspace]
members = [
    "clap_bench",
    "clap_builder",
    "clap_complete",
    "clap_complete_fig",
    "clap_complete_nushell",
    "clap_derive",
    "clap_lex",
    "clap_mangen",
]
"#;

    const TEXT_PACKAGE: &str = r#"[package]
name = "test"
version = "2024.10.21"
edition = "2024"

[[bin]]
name = "a"
path = "src/main.cpp"

[features]
debug = [
    "clap_builder/debug",
    "clap_derive?/debug",
]
default = [
    "color",
    "error-context",
    "help",
    "std",
    "suggestions",
    "usage",
]
"#;

    const TEXT_CONFLICTS: &str = r#"[workspace]
members = [
    "clap_bench",
    "clap_builder",
    "clap_complete",
    "clap_complete_fig",
    "clap_complete_nushell",
    "clap_derive",
    "clap_lex",
    "clap_mangen",
]

[package]
name = "test"
version = "2024.10.21"
edition = "2024"

[[bin]]
name = "a"
path = "src/main.cpp"

[features]
debug = [
    "clap_builder/debug",
    "clap_derive?/debug",
]
default = [
    "color",
    "error-context",
    "help",
    "std",
    "suggestions",
    "usage",
]
"#;

    #[test]
    fn test_loads() {
        let data = ProjectConfig::loads(TEXT_CONFLICTS);
        assert!(data.is_some());
    }

    #[test]
    fn test_dumps() {
        let mut data = ProjectConfig::default();

        data.package = Some(PackageConfig {
            name: String::from("test"),
            version: String::from("2024.10.21"),
            edition: String::from("2024"),
        });

        let mut bins = BTreeSet::new();
        bins.insert(EntryConfig {
            name: String::from("a"),
            path: String::from("src/main.cpp"),
        });
        data.bins = Some(bins);

        data.workspace = Some(WorkSpaceConfig {
            members: [
                String::from("clap_bench"),
                String::from("clap_builder"),
                String::from("clap_derive"),
                String::from("clap_lex"),
                String::from("clap_complete"),
                String::from("clap_complete_fig"),
                String::from("clap_complete_nushell"),
                String::from("clap_mangen"),
            ]
            .into(),
        });

        let mut features = BTreeMap::new();
        features.insert(String::from("default"), {
            [
                String::from("std"),
                String::from("color"),
                String::from("help"),
                String::from("usage"),
                String::from("error-context"),
                String::from("suggestions"),
            ]
            .into()
        });
        features.insert(String::from("debug"), {
            [
                String::from("clap_builder/debug"),
                String::from("clap_derive?/debug"),
            ]
            .into()
        });
        data.features = Some(features);

        let mut dependencies = BTreeMap::new();
        dependencies.insert(
            String::from("chrono"),
            DependencyConfig {
                version: String::from("0.4.38"),
                features: None,
            },
        );
        dependencies.insert(
            String::from("clang-sys"),
            DependencyConfig {
                version: String::from("1.8.1"),
                features: Some([String::from("derive")].into()),
            },
        );
        dependencies.insert(
            String::from("tracing-subscriber"),
            DependencyConfig {
                version: String::from("clang_10_0"),
                features: Some(
                    [
                        String::from("env-filter"),
                        String::from("time"),
                        String::from("local-time"),
                        String::from("json"),
                    ]
                    .into(),
                ),
            },
        );

        let text = data.dumps();
        assert!(text == TEXT_CONFLICTS);
    }

    #[test]
    fn test_conflicts_workspace() {
        let data = ProjectConfig::loads(TEXT_CONFLICTS);
        assert_eq!(data.unwrap().validate(), false);
    }

    #[test]
    fn test_empty_package() {
        let data = ProjectConfig::default();
        assert_eq!(data.validate(), false);
    }

    #[test]
    fn test_valid_workspace() {
        let data = ProjectConfig::loads(TEXT_WORKSPACE);
        assert_eq!(data.unwrap().validate(), true);
    }

    #[test]
    fn test_valid_package() {
        let data = ProjectConfig::loads(TEXT_PACKAGE);
        assert_eq!(data.unwrap().validate(), true);
    }
}

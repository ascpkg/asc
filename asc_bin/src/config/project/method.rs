#[allow(unused_imports)]
use super::data::{DependencyConfig, EntryConfig, PackageConfig, ProjectConfig, WorkSpaceConfig};
use crate::config::relative_paths::{ASC_TARGET_DIR_NAME, ASC_TOML_FILE_NAME};
use crate::{config::relative_paths, errors::ErrorTag, util};

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet};

use chrono::{self, Datelike};

impl ProjectConfig {
    pub fn version_date() -> String {
        let local_now = chrono::Local::now();
        format!(
            "{}.{}.{}",
            local_now.year(),
            local_now.month(),
            local_now.day()
        )
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
            if self.bins.is_empty() {
                errors.push("bins");
            }
            if self.libs.is_empty() {
                errors.push("libs");
            }
            if !self.dependencies.is_empty() {
                errors.push("dependencies");
            }
            if !self.features.is_empty() {
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

    pub fn is_project_inited(ignore_error: bool) -> bool {
        if util::fs::is_file_exists(ASC_TOML_FILE_NAME) {
            if ignore_error {
                tracing::warn!(
                    func = "util::fs::is_file_exists",
                    path = ASC_TOML_FILE_NAME,
                    error_tag = ErrorTag::FileExistsError.as_ref(),
                    message = "skip"
                );
            }
            return true;
        } else {
            if !ignore_error {
                tracing::error!(
                    func = "util::fs::is_file_exists",
                    path = ASC_TOML_FILE_NAME,
                    error_tag = ErrorTag::FileNotFoundError.as_ref(),
                    message = "please run asc init first"
                );
            }
            return false;
        }
    }

    pub fn read_project_conf() -> Option<Self> {
        Self::load(ASC_TOML_FILE_NAME, false)
    }

    pub fn write_project_conf(&mut self) -> bool {
        if self.path.is_empty() {
            self.path = ASC_TOML_FILE_NAME.to_string();
        }
        self.dump(false)
    }

    pub fn is_source_scaned() -> bool {
        if util::fs::is_file_exists(&format!(
            "{}/{}",
            relative_paths::ASC_PROJECT_DIR_NAME,
            relative_paths::CMAKE_LISTS_TXT_FILE_NAME
        )) && util::fs::is_dir_exists(ASC_TARGET_DIR_NAME)
        {
            return true;
        } else {
            tracing::error!(
                func = "util::fs::is_file_exists && util::fs::is_dir_exists",
                file = relative_paths::CMAKE_LISTS_TXT_FILE_NAME,
                dir = ASC_TARGET_DIR_NAME,
                error_tag = ErrorTag::PathNotFoundError.as_ref(),
                message = "please run asc scan first"
            );
            return false;
        }
    }
}

impl WorkSpaceConfig {
    pub fn get_members(&self) -> String {
        self.members
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(", ")
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
description = ""
license = "LGPL-3.0+"
repository = ""

[[bin]]
name = "a"
source_dir = "src"
source_file = "main.cpp"

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
description = ""
license = "LGPL-3.0+"
repository = ""

[[bin]]
name = "a"
source_dir = "src"
source_file = "main.cpp"

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
        let data = ProjectConfig::loads(TEXT_CONFLICTS, false);
        assert!(data.is_some());
    }

    #[test]
    fn test_dumps() {
        let mut data = ProjectConfig::default();

        data.package = Some(PackageConfig {
            name: String::from("test"),
            version: String::from("2024.10.21"),
            edition: String::from("2024"),
            description: String::new(),
            license: String::from("LGPL-3.0+"),
            repository: String::from(""),
        });

        data.bins.insert(EntryConfig {
            name: String::from("a"),
            source_dir: String::from("src"),
            source_file: String::from("main.cpp"),
            shared: None,
        });

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
        data.features = features;

        let mut dependencies = BTreeMap::new();
        dependencies.insert(
            String::from("chrono"),
            DependencyConfig {
                version: String::from("0.4.38"),
                find_packages: BTreeSet::new(),
                link_libraries: BTreeSet::new(),
                features: BTreeSet::new(),
            },
        );
        dependencies.insert(
            String::from("clang-sys"),
            DependencyConfig {
                version: String::from("1.8.1"),
                find_packages: BTreeSet::new(),
                link_libraries: BTreeSet::new(),
                features: [String::from("derive")].into(),
            },
        );
        dependencies.insert(
            String::from("tracing-subscriber"),
            DependencyConfig {
                version: String::from("clang_10_0"),
                find_packages: BTreeSet::new(),
                link_libraries: BTreeSet::new(),
                features: [
                    String::from("env-filter"),
                    String::from("time"),
                    String::from("local-time"),
                    String::from("json"),
                ]
                .into(),
            },
        );

        let text = data.dumps(false);
        assert!(text == TEXT_CONFLICTS);
    }

    #[test]
    fn test_conflicts_workspace() {
        let data = ProjectConfig::loads(TEXT_CONFLICTS, false);
        assert_eq!(data.unwrap().validate(), false);
    }

    #[test]
    fn test_empty_package() {
        let data = ProjectConfig::default();
        assert_eq!(data.validate(), false);
    }

    #[test]
    fn test_valid_workspace() {
        let data = ProjectConfig::loads(TEXT_WORKSPACE, false);
        assert_eq!(data.unwrap().validate(), true);
    }

    #[test]
    fn test_valid_package() {
        let data = ProjectConfig::loads(TEXT_PACKAGE, false);
        assert_eq!(data.unwrap().validate(), true);
    }
}

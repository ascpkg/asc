use std::collections::{BTreeMap, BTreeSet};

use chrono;

use serde::{Deserialize, Serialize};

use toml;

pub static PROJECT_TOML: &str = "asc.toml";
pub static PROJECT_EDITION: &str = "2024";
pub static PROJECT_SRC_DIR: &str = "src";
pub static PROJECT_BIN_SRC: &str = "main.cpp";
pub static PROJECT_LIB_SRC: &str = "lib.cpp";
pub static PROJECT_EXPORT_SRC: &str = "export.h";

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntryConfig {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct DependencyConfig {
    pub version: String,
    pub features: Option<BTreeSet<String>>,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkSpaceConfig {
    pub members: BTreeSet<String>,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub workspace: Option<WorkSpaceConfig>,
    pub package: Option<PackageConfig>,
    #[serde(rename = "bin")]
    pub bins: Option<BTreeSet<EntryConfig>>,
    #[serde(rename = "lib")]
    pub libs: Option<BTreeSet<EntryConfig>>,
    pub dependencies: Option<BTreeMap<String, DependencyConfig>>,
    pub features: Option<BTreeMap<String, BTreeSet<String>>>,
}

impl ProjectConfig {
    pub fn version_date() -> String {
        let local_now = chrono::Local::now();
        local_now.format("%Y.%m.%d").to_string()
    }

    pub fn validate(&self) -> bool {
        if self.workspace.is_some() {
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
                    message = "workspace conflicts",
                    conflicts = errors.join(", ")
                );
            }

            return errors.is_empty();
        } else {
            if self.package.is_none() {
                tracing::error!(message = "package empty");
                return false;
            }
        }

        return true;
    }

    pub fn load(path: &str) -> Option<Self> {
        match std::fs::read_to_string(path) {
            Ok(text) => Self::loads(&text),
            Err(e) => {
                tracing::error!(
                    messsage = "std::fs::read_to_string",
                    path = path,
                    error = e.to_string()
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
                    messsage = "toml::from_str",
                    text = text,
                    error = e.to_string()
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
                    messsage = "std::fs::write",
                    path = path,
                    text = text,
                    error = e.to_string()
                );
                false
            }
        }
    }

    pub fn dumps(&self) -> String {
        match toml::to_string_pretty(self) {
            Ok(text) => text,
            Err(e) => {
                tracing::error!(messsage = "toml::to_string_pretty", error = e.to_string());
                String::new()
            }
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
                version: String::from("clang_18_0"),
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

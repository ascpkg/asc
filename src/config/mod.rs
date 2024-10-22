use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub static PROJECT_TOML: &str = "asc.toml";
pub static PROJECT_EDITION: &str = "2024";
pub static PROJECT_TARGET_DIR: &str = "target";
pub static PROJECT_SRC_DIR: &str = "src";
pub static PROJECT_BIN_SRC: &str = "main.cpp";
pub static PROJECT_LIB_SRC: &str = "lib.hpp";
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

pub mod method;

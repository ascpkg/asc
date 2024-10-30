use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;

use crate::config;

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct EntryConfig {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct DependencyConfig {
    pub version: String,
    pub find_packages: BTreeSet<String>,
    pub link_libraries: BTreeSet<String>,
    pub features: BTreeSet<String>,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct WorkSpaceConfig {
    pub members: BTreeSet<String>,
}

#[derive(
    Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize, ConfigFile,
)]
#[config_file_ext("toml")]
pub struct ProjectConfig {
    pub workspace: Option<WorkSpaceConfig>,
    pub package: Option<PackageConfig>,
    #[serde(rename = "bin")]
    pub bins: Option<BTreeSet<EntryConfig>>,
    #[serde(rename = "lib")]
    pub libs: Option<BTreeSet<EntryConfig>>,
    pub features: BTreeMap<String, BTreeSet<String>>,
    pub dependencies: BTreeMap<String, DependencyConfig>,

    #[serde(skip)]
    pub path: String,
}

#[derive(Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct InstalledFiles {
    pub prefix: String,
    pub files: Vec<String>,
    #[serde(skip)]
    pub path: String,
}

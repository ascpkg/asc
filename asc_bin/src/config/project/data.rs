use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub description: String,
    pub license: String,
    pub repository: String,
    pub branch: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub supports: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct EntryConfig {
    pub name: String,
    pub source_dir: String,
    pub source_file: String,
    pub shared: Option<bool>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub std_c: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub std_cxx: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct DependencyConfig {
    pub version: String,
    pub find_packages: BTreeSet<String>,
    pub include_directories: BTreeSet<String>,
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
    #[serde(rename = "bin", default, skip_serializing_if = "BTreeSet::is_empty")]
    pub bins: BTreeSet<EntryConfig>,
    #[serde(rename = "lib", default, skip_serializing_if = "BTreeSet::is_empty")]
    pub libs: BTreeSet<EntryConfig>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub features: BTreeMap<String, BTreeSet<String>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
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

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

#[derive(
    Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize, ConfigFile,
)]
#[config_file_ext("json")]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortJson {
    pub name: String,
    pub version: String,
    pub port_version: String,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub supports: String,
    pub dependencies: Vec<BTreeMap<String, String>>,

    #[serde(skip)]
    path: String,
}


#[derive(Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgDependency {
    pub dependencies: Vec<VcpkgDependencyDesc>,
    pub overrides: Vec<BTreeMap<String, String>>,

    #[serde(skip)]
    pub path: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgDependencyDesc {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_features: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<BTreeMap<String, String>>,
}
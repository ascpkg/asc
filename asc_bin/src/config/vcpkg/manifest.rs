use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

#[derive(Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgConfiguration {
    pub default_registry: VcpkgDefaultRegistry,
    pub registries: Vec<VcpkgRegistry>,

    #[serde(skip)]
    pub path: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VcpkgRegistry {
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub location: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub baseline: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub repository: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reference: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VcpkgDefaultRegistry {
    pub kind: String,
    pub baseline: String,
    pub repository: String,
}

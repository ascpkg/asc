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
    pub name: String,
    pub location: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VcpkgDefaultRegistry {
    pub kind: String,
    pub baseline: String,
    pub repository: String,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

// from vcpkg (versions/baseline.json)
#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgBaseline {
    pub default: HashMap<String, VcpkgPortVersion>,

    #[serde(skip)]
    path: String,
}

// from vcpkg
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortVersion {
    pub baseline: String,
    pub port_version: u32,
}

impl VcpkgPortVersion {
    pub fn format_version_text(&self) -> String {
        if self.port_version == 0 {
            self.baseline.clone()
        } else {
            format!("{}#{}", self.baseline, self.port_version)
        }
    }
}

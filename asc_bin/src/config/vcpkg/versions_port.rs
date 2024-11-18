use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

// from vcpkg
#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgPortVersions {
    pub versions: Vec<VcpkgPortTreeVersion>,

    #[serde(skip)]
    path: String,
}

// from vcpkg
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortTreeVersion {
    pub git_tree: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_semver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_string: Option<String>,
    pub port_version: u32,
}

impl VcpkgPortTreeVersion {
    pub fn format_version_text(&self) -> String {
        let mut s = String::new();
        if let Some(v) = &self.version {
            s = v.clone();
        } else if let Some(v) = &self.version_date {
            s = v.clone();
        } else if let Some(v) = &self.version_string {
            s = v.clone();
        } else if let Some(v) = &self.version_semver {
            s = v.clone();
        }

        if self.port_version == 0 {
            s
        } else {
            format!("{}#{}", s, self.port_version)
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VcpkgGitTreeInfo {
    pub port_name: String,
    pub commit_hash: String,
    pub commit_date_time: String,
}

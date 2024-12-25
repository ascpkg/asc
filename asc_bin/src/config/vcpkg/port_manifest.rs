use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use config_file_derives::ConfigFile;
use config_file_types;

use crate::util;

static SOURCE_PREFIX: &str = "Source:";
static VERSION_PREFIX: &str = "Version:";
static VERSION_DATE_PREFIX: &str = "Version-Date:";
static VERSION_SEMVER_PREFIX: &str = "Version-Semver:";
static VERSION_STRING_PREFIX: &str = "Version-String:";
static PORT_VERSION_PREFIX: &str = "Port-Version:";
static BUILD_DEPENDS_PREFIX: &str = "Build-Depends:";

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgPortManifest {
    #[serde(skip)]
    path: String,

    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_semver: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_string: Option<String>,

    #[serde(default)]
    pub port_version: u32,

    #[serde(skip_serializing_if = "String::is_empty")]
    homepage: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    description: Vec<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    supports: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    default_features: Vec<String>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    features: BTreeMap<String, VcpkgPortFeature>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<VcpkgPortDependency>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct VcpkgPortFeature {
    description: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<VcpkgPortDependency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum VcpkgPortDependency {
    Simple(String),
    Complex(ComplexDependency),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct ComplexDependency {
    name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    features: Vec<String>,
    default_features: bool,
    platform: Option<String>,
}

impl VcpkgPortManifest {
    pub fn update_vcpkg_json_file(
        path: &str,
        all_port_versions: &BTreeMap<String, String>,
    ) -> String {
        if !util::fs::is_file_exists(&path) {
            return String::new();
        }

        let mut data = Self::load(path, false).unwrap();
        let mut version = vec![];
        if let Some(v) = &data.version {
            version.push(v.as_str());
        }
        if let Some(v) = &data.version_date {
            version.push(v.as_str());
        }
        if let Some(v) = &data.version_semver {
            version.push(v.as_str());
        }
        if let Some(v) = &data.version_string {
            version.push(v.as_str());
        }

        let version = version.join("-").replace("_", "-").replace(".", "-");
        data.name = format!("{}-{version}-{}", data.name, data.port_version);

        data.dump(true, false);

        return version;
    }

    pub fn update_control_file(path: &str, all_port_versions: &BTreeMap<String, String>) -> String {
        if !util::fs::is_file_exists(&path) {
            return String::new();
        }

        let text = std::fs::read_to_string(path).unwrap();
        let version = Self::get_version_from_control_file(&text);

        let mut lines = text
            .lines()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for no in 0..lines.len() {
            let line = &lines[no];
            if line.starts_with(SOURCE_PREFIX) {
                lines[no] = format!("{line}-{version}");
            } else if line.starts_with(BUILD_DEPENDS_PREFIX) {
                let mut features = line
                    .split_at(BUILD_DEPENDS_PREFIX.len() + 1)
                    .1
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                for index in 0..features.len() {
                    let feature = &features[index];
                    if feature.contains("[") && feature.contains("]") {
                        continue;
                    }
                    if let Some(version) = all_port_versions.get(feature) {
                        features[index] = format!("{feature}-{version}");
                    }
                }
                lines[no] = features.join(", ");
            }
        }

        std::fs::write(path, (lines.join("\n") + "\n").as_bytes()).unwrap();

        return version;
    }

    pub fn get_version_from_vcpkg_json_file(text: &str) -> String {
        let data = VcpkgPortManifest::loads(text, false).unwrap();
        let mut version = vec![];
        if let Some(v) = &data.version {
            version.push(v.as_str());
        }
        if let Some(v) = &data.version_date {
            version.push(v.as_str());
        }
        if let Some(v) = &data.version_semver {
            version.push(v.as_str());
        }
        if let Some(v) = &data.version_string {
            version.push(v.as_str());
        }

        let version = version.join("-").replace("_", "-").replace(".", "-");
        return format!("{}-{version}-{}", data.name, data.port_version);
    }

    pub fn get_version_from_control_file(text: &str) -> String {
        let mut version = vec![];
        for line in text.lines() {
            if line.starts_with(VERSION_PREFIX) {
                version.push(line.split_at(VERSION_PREFIX.len()).1.trim());
            } else if line.starts_with(VERSION_DATE_PREFIX) {
                version.push(line.split_at(VERSION_DATE_PREFIX.len()).1.trim());
            } else if line.starts_with(VERSION_SEMVER_PREFIX) {
                version.push(line.split_at(VERSION_SEMVER_PREFIX.len()).1.trim());
            } else if line.starts_with(VERSION_STRING_PREFIX) {
                version.push(line.split_at(VERSION_STRING_PREFIX.len()).1.trim());
            } else if line.starts_with(PORT_VERSION_PREFIX) {
                version.push(line.split_at(PORT_VERSION_PREFIX.len()).1.trim());
            }
        }

        return version.join("-").replace("_", "-").replace(".", "-");
    }
}

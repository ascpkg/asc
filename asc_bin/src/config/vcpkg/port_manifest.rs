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
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortManifest {
    #[serde(skip)]
    path: String,

    name: String,

    pub version: Option<String>,
    pub version_date: Option<String>,
    pub version_semver: Option<String>,
    pub version_string: Option<String>,

    #[serde(default)]
    pub port_version: u32,

    #[serde(skip_serializing_if = "String::is_empty")]
    homepage: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    description: Vec<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    supports: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    default_features: Vec<String>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    features: BTreeMap<String, VcpkgPortFeature>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

        let (name, version) = Self::build_version_suffix_name(
            &data.name,
            &data.version,
            &data.version_date,
            &data.version_semver,
            &data.version_string,
            data.port_version,
        );
        data.name = name;

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

        let (_name, version) = Self::build_version_suffix_name(
            &data.name,
            &data.version,
            &data.version_date,
            &data.version_semver,
            &data.version_string,
            data.port_version,
        );
        return version;
    }

    fn build_version_suffix_name(
        name: &str,
        version: &Option<String>,
        version_date: &Option<String>,
        version_semver: &Option<String>,
        version_string: &Option<String>,
        port_version: u32,
    ) -> (String, String) {
        let mut versions = vec![];
        if let Some(v) = version {
            versions.push(v.clone());
        }
        if let Some(v) = version_date {
            versions.push(v.clone());
        }
        if let Some(v) = version_semver {
            versions.push(v.clone());
        }
        if let Some(v) = version_string {
            versions.push(v.clone());
        }
        versions.push(format!("{port_version}"));

        let v = versions.join("-").replace("_", "-").replace(".", "-");
        return (format!("{name}-{v}"), v);
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

#[cfg(test)]
mod tests {
    use super::*;

    const VCPKG_REPO_DIR: &str = "D:/asc/data/vcpkg";
    const FFMPEG_CONTROL_COMMIT_ID: &str = "373915929eac1d0219474c18a6e8a3134783dfc5";
    const FFMPEG_VCPKG_JSON_COMMIT_ID: &str = "44e8841e065a1b14340c6c0bb90210b11d7c3d4d";

    fn get_all_port_versions(commit_id: &str) -> BTreeMap<String, String> {
        let mut all_port_versions = BTreeMap::new();
        for (port, (control_file_text, vcpkg_json_file_text)) in
            crate::git::ls_tree::list_ports(commit_id, VCPKG_REPO_DIR, true)
        {
            if !control_file_text.is_empty() {
                all_port_versions.insert(
                    port,
                    VcpkgPortManifest::get_version_from_control_file(&control_file_text),
                );
            } else if !vcpkg_json_file_text.is_empty() {
                all_port_versions.insert(
                    port,
                    VcpkgPortManifest::get_version_from_vcpkg_json_file(&vcpkg_json_file_text),
                );
            }
        }
        return all_port_versions;
    }
    
    fn get_ffmpeg_control() -> String {
        crate::git::show::file_content(VCPKG_REPO_DIR, FFMPEG_CONTROL_COMMIT_ID)
    }

    fn get_ffmpeg_vcpkg_json() -> String {
        crate::git::show::file_content(VCPKG_REPO_DIR, FFMPEG_VCPKG_JSON_COMMIT_ID)
    }

    #[test]
    fn test_get_version_from_control_file() {
        assert_eq!(
            String::from("4-3-2-11"),
            VcpkgPortManifest::get_version_from_control_file(&get_ffmpeg_control())
        );
    }

    #[test]
    fn test_get_version_from_vcpkg_json_file() {
        assert_eq!(
            String::from("4-4-0"),
            VcpkgPortManifest::get_version_from_vcpkg_json_file(&get_ffmpeg_vcpkg_json())
        );
    }

    #[test]
    fn test_update_control_file() {
        let path = "ffmpeg.CONTROL";
        std::fs::write(path, get_ffmpeg_control().as_bytes()).unwrap();

        // let all_port_versions = get_all_port_versions(FFMPEG_CONTROL_COMMIT_ID);
        // VcpkgPortManifest::update_control_file(path, &all_port_versions);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_update_vcpkg_json_file() {
        let path = "ffmpeg.vcpkg.json";
        std::fs::write(path, get_ffmpeg_vcpkg_json().as_bytes()).unwrap();

        // let all_port_versions = get_all_port_versions(FFMPEG_VCPKG_JSON_COMMIT_ID);
        // VcpkgPortManifest::update_vcpkg_json_file(path, &all_port_versions);

        std::fs::remove_file(path).unwrap();
    }
}


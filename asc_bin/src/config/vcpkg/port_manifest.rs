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
static HOMEPAGE_PREFIX: &str = "Homepage:";
static DESCRIPTION_PREFIX: &str = "Description:";
static SUPPORTS_PREFIX: &str = "Supports:";
static DEFAULT_FEATURES_PREFIX: &str = "Default-Features:";
static FEATURES_PREFIX: &str = "Features:";
static BUILD_DEPENDS_PREFIX: &str = "Build-Depends:";

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
struct VcpkgPortManifest {
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
    pub fn from_file(path: &str) -> Option<VcpkgPortManifest> {
        if path.ends_with(".json") {
            return Self::load(path, false);
        } else {
            return Self::from_control_file(path);
        }
    }

    pub fn from_control_file(path: &str) -> Option<VcpkgPortManifest> {
        if !util::fs::is_file_exists(&path) {
            return None;
        }

        let mut manifest = VcpkgPortManifest::default();
        let text = std::fs::read_to_string(path).unwrap();
        for lines in text.split("\n\n\n") {
            // source section
            if lines.starts_with(SOURCE_PREFIX) {
                let mut entry = "";
                for line in lines.lines() {
                    if line.starts_with(SOURCE_PREFIX) {
                        entry = SOURCE_PREFIX;
                        manifest.name = line.split_at(SOURCE_PREFIX.len()).1.trim().to_string();
                    } else if line.starts_with(VERSION_PREFIX) {
                        entry = VERSION_PREFIX;
                        manifest.version =
                            Some(line.split_at(VERSION_PREFIX.len()).1.trim().to_string());
                    } else if line.starts_with(VERSION_DATE_PREFIX) {
                        entry = VERSION_DATE_PREFIX;
                        manifest.version_date =
                            Some(line.split_at(VERSION_DATE_PREFIX.len()).1.trim().to_string());
                    } else if line.starts_with(VERSION_SEMVER_PREFIX) {
                        entry = VERSION_SEMVER_PREFIX;
                        manifest.version_semver = Some(
                            line.split_at(VERSION_SEMVER_PREFIX.len())
                                .1
                                .trim()
                                .to_string(),
                        );
                    } else if line.starts_with(VERSION_STRING_PREFIX) {
                        entry = VERSION_STRING_PREFIX;
                        manifest.version_string = Some(
                            line.split_at(VERSION_STRING_PREFIX.len())
                                .1
                                .trim()
                                .to_string(),
                        );
                    } else if line.starts_with(PORT_VERSION_PREFIX) {
                        entry = PORT_VERSION_PREFIX;
                        manifest.port_version = line
                            .split_at(PORT_VERSION_PREFIX.len())
                            .1
                            .trim()
                            .parse::<u32>()
                            .unwrap();
                    } else if line.starts_with(HOMEPAGE_PREFIX) {
                        entry = HOMEPAGE_PREFIX;
                        manifest.homepage = line.split_at(HOMEPAGE_PREFIX.len()).1.trim().to_string();
                    } else if line.starts_with(DESCRIPTION_PREFIX) {
                        entry = DESCRIPTION_PREFIX;
                        manifest
                            .description
                            .push(line.split_at(DESCRIPTION_PREFIX.len()).1.trim().to_string());
                    } else if line.starts_with(DEFAULT_FEATURES_PREFIX) {
                        entry = DEFAULT_FEATURES_PREFIX;
                        for part in line
                            .split_at(DEFAULT_FEATURES_PREFIX.len())
                            .1
                            .trim()
                            .split(",")
                        {
                            manifest.default_features.push(part.trim().to_string());
                        }
                    } else if line.starts_with(SUPPORTS_PREFIX) {
                        manifest.supports = line.split_at(SUPPORTS_PREFIX.len()).1.trim().to_string();
                    } else if lines.starts_with(BUILD_DEPENDS_PREFIX) {
                        for part in lines
                            .split_at(BUILD_DEPENDS_PREFIX.len())
                            .1
                            .trim()
                            .split_whitespace()
                        {
                            let is_complex_feature = part.contains("[") && part.contains("]");
                            let has_platform = part.contains("(") && part.contains(")");
                            if !is_complex_feature && !has_platform {
                                manifest
                                    .dependencies
                                    .push(VcpkgPortDependency::Simple(part.to_string()));
                            } else {
                                let mut dep = ComplexDependency::default();
                                let begin = part.find("[").unwrap();
                                let end = part.find("]").unwrap();
                                let name = part.split_at(begin).0;
                                dep.name = name.to_string();
                                dep.default_features = false;
                                for feature in part.split_at(begin).1.split_at(end).0.split(",") {
                                    dep.features.push(feature.to_string());
                                }
                                manifest
                                    .dependencies
                                    .push(VcpkgPortDependency::Complex(dep));
                            }
                        }
                    } else {
                        if entry == DESCRIPTION_PREFIX {
                            manifest.description.push(line.trim().to_string());
                        }
                    }
                }
            }
            // feature section
            else if lines.starts_with(FEATURES_PREFIX) {
                for line in lines.lines() {
                    if line.starts_with(FEATURES_PREFIX) {
                    } else if line.starts_with(BUILD_DEPENDS_PREFIX) {
                    } else if line.starts_with(DESCRIPTION_PREFIX) {
                    } else if line.starts_with(SUPPORTS_PREFIX) {
                    }
                }
            }
        }

        return Some(manifest);
    }
}

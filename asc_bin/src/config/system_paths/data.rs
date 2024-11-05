use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

use crate::config::relative_paths::{
    self, VCPKG_BASELINE_JSON_FILE_NAME, VCPKG_BUILD_SYSTEMS_DIR_NAME, VCPKG_CMAKE_FILE_NAME,
    VCPKG_DIR_NAME, VCPKG_SCRIPTS_DIR_NAME, VCPKG_VERSIONS_DIR_NAME,
};
use crate::util;

pub struct DataPath {}

impl DataPath {
    fn prefix() -> String {
        if let Some(dir) = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let data_dir = dir.data_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&data_dir) {
                if !util::fs::create_dirs(&data_dir) {
                    return String::new();
                }
            }
            return data_dir;
        }
        return String::new();
    }

    pub fn vcpkg_clone_dir() -> String {
        build(&Self::prefix(), VCPKG_DIR_NAME)
    }

    pub fn vcpkg_versions_port_json_path(vcpkg_clone_dir: &str, port_name: &str) -> String {
        format!(
            "{}/{}/{}-/{}.json",
            vcpkg_clone_dir,
            VCPKG_VERSIONS_DIR_NAME,
            port_name.chars().nth(0).unwrap(),
            port_name
        )
    }

    pub fn vcpkg_versions_baseline_json_path(vcpkg_clone_dir: &str) -> String {
        format!(
            "{}/{}/{}",
            vcpkg_clone_dir, VCPKG_VERSIONS_DIR_NAME, VCPKG_BASELINE_JSON_FILE_NAME
        )
    }

    pub fn vcpkg_scripts_build_systems_cmake_path(vcpkg_clone_dir: &str) -> String {
        format!(
            "{}/{}/{}/{}",
            vcpkg_clone_dir,
            VCPKG_SCRIPTS_DIR_NAME,
            VCPKG_BUILD_SYSTEMS_DIR_NAME,
            VCPKG_CMAKE_FILE_NAME
        )
    }

    pub fn vcpkg_search_index_json() -> String {
        build(
            &Self::prefix(),
            &format!(
                "{}/{}",
                relative_paths::VCPKG_INDEX_DIR_NAME,
                relative_paths::VCPKG_SEARCH_INDEX_JSON_FILE_NAME
            ),
        )
    }

    pub fn vcpkg_tree_index_json() -> String {
        build(
            &Self::prefix(),
            &format!(
                "{}/{}",
                relative_paths::VCPKG_INDEX_DIR_NAME,
                relative_paths::VCPKG_TREE_INDEX_JSON_FILE_NAME
            ),
        )
    }

    pub fn vcpkg_binary_cache_dir() -> String {
        build(&Self::prefix(), relative_paths::VCPKG_BINARY_CACHE_DIR_NAME)
    }
}

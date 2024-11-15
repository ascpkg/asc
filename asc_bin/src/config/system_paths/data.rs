use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

use crate::config::relative_paths::{
    self, VCPKG_BASELINE_JSON_FILE_NAME, VCPKG_BUILD_SYSTEMS_DIR_NAME, VCPKG_CMAKE_FILE_NAME,
    VCPKG_DIR_NAME, VCPKG_JSON_FILE_NAME, VCPKG_PORTS_DIR_NAME, VCPKG_PORT_FILE_CMAKE_FILE_NAME,
    VCPKG_SCRIPTS_DIR_NAME, VCPKG_VERSIONS_DIR_NAME,
};

pub struct DataPath {}

impl DataPath {
    fn prefix() -> String {
        if let Some(dir) = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            return dir.data_dir().to_str().unwrap().replace(r"\", "/");
        }
        return String::new();
    }

    pub fn vcpkg_default_clone_dir() -> String {
        build(
            &Self::prefix(),
            vec![String::from(VCPKG_DIR_NAME)],
            false,
            true,
        )
    }

    pub fn vcpkg_port_json_path(vcpkg_clone_dir: &str, port_name: &str) -> String {
        build(
            vcpkg_clone_dir,
            vec![
                String::from(VCPKG_VERSIONS_DIR_NAME),
                VCPKG_PORTS_DIR_NAME[..VCPKG_PORTS_DIR_NAME.len() - 1].to_string(),
                format!("{port_name}/{VCPKG_JSON_FILE_NAME}"),
            ],
            false,
            false,
        )
    }

    pub fn vcpkg_port_file_cmake_path(vcpkg_clone_dir: &str, port_name: &str) -> String {
        build(
            vcpkg_clone_dir,
            vec![
                String::from(VCPKG_VERSIONS_DIR_NAME),
                VCPKG_PORTS_DIR_NAME[..VCPKG_PORTS_DIR_NAME.len() - 1].to_string(),
                format!("{port_name}/{VCPKG_PORT_FILE_CMAKE_FILE_NAME}"),
            ],
            false,
            false,
        )
    }

    pub fn vcpkg_versions_port_json_path(vcpkg_clone_dir: &str, port_name: &str) -> String {
        build(
            vcpkg_clone_dir,
            vec![
                String::from(VCPKG_VERSIONS_DIR_NAME),
                format!("{}-", port_name.chars().nth(0).unwrap()),
                format!("{}.json", port_name),
            ],
            false,
            false,
        )
    }

    pub fn vcpkg_versions_baseline_json_path(vcpkg_clone_dir: &str) -> String {
        build(
            vcpkg_clone_dir,
            vec![
                String::from(VCPKG_VERSIONS_DIR_NAME),
                String::from(VCPKG_BASELINE_JSON_FILE_NAME),
            ],
            false,
            false,
        )
    }

    pub fn vcpkg_scripts_build_systems_cmake_path(vcpkg_clone_dir: &str) -> String {
        build(
            vcpkg_clone_dir,
            vec![
                String::from(VCPKG_SCRIPTS_DIR_NAME),
                String::from(VCPKG_BUILD_SYSTEMS_DIR_NAME),
                String::from(VCPKG_CMAKE_FILE_NAME),
            ],
            false,
            false,
        )
    }

    pub fn vcpkg_default_index_dir() -> String {
        build(
            &Self::prefix(),
            vec![String::from(relative_paths::VCPKG_INDEX_DIR_NAME)],
            true,
            true,
        )
    }

    pub fn vcpkg_search_index_json(index_dir: &str) -> String {
        build(
            index_dir,
            vec![String::from(
                relative_paths::VCPKG_SEARCH_INDEX_JSON_FILE_NAME,
            )],
            true,
            false,
        )
    }

    pub fn vcpkg_tree_index_json(index_dir: &str) -> String {
        build(
            index_dir,
            vec![String::from(
                relative_paths::VCPKG_TREE_INDEX_JSON_FILE_NAME,
            )],
            true,
            false,
        )
    }

    pub fn vcpkg_default_downloads_dir() -> String {
        build(
            &Self::prefix(),
            vec![String::from(relative_paths::VPCKG_DOWNLOADS_DIR_NAME)],
            true,
            true,
        )
    }

    pub fn vcpkg_default_binary_cache_dir() -> String {
        build(
            &Self::prefix(),
            vec![String::from(relative_paths::VCPKG_BINARY_CACHE_DIR_NAME)],
            true,
            true,
        )
    }
}

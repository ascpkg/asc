use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

use crate::config::relative_paths::{
    VCPKG_BASELINE_JSON_FILE_NAME, VCPKG_DIR_NAME, VCPKG_VERSIONS_DIR_NAME,
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

    pub fn vcpkg_search_index_json() -> String {
        build(&Self::prefix(), "vcpkg.index/search_index.json")
    }

    pub fn vcpkg_tree_index_json() -> String {
        build(&Self::prefix(), "vcpkg.index/tree_index.json")
    }
}

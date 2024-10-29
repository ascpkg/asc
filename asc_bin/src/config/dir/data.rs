use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

use crate::util;

pub struct DataDir {}

impl DataDir {
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
        build(&Self::prefix(), "vcpkg")
    }

    pub fn vcpkg_search_index_json() -> String {
        build(&Self::prefix(), "vcpkg.index/search_index.json")
    }

    pub fn vcpkg_tree_index_json() -> String {
        build(&Self::prefix(), "vcpkg.index/tree_index.json")
    }
}

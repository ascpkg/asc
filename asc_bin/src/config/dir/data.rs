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

    pub fn vcpkg_search_version_index_json() -> String {
        // port name <-> index
        // commit hash <-> index
        // commit date time <-> index
        // port version <-> index
        // port name index -> (port version index, commit hash index)
        build(&Self::prefix(), "vcpkg.index/version_index.json")
    }

    pub fn vcpkg_git_tree_json() -> String {
        build(&Self::prefix(), "vcpkg.index/git_tree.json")
    }
}

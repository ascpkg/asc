use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

use crate::util;

pub struct DataDir {}

impl DataDir {
    fn prefix() -> String {
        if let Some(dir) = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let data_dir = dir.data_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&data_dir) {
                if !util::fs::create_dir(&data_dir) {
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

    pub fn vcpkg_search_prefix_index_file() -> String {
        build(&Self::prefix(), "vcpkg.index/search_prefix.json")
    }

    pub fn vcpkg_search_postfix_index_file() -> String {
        build(&Self::prefix(), "vcpkg.index/search_postfix.json")
    }

    pub fn vcpkg_search_baseline_file() -> String {
        build(&Self::prefix(), "vcpkg.index/baseline.json")
    }

    pub fn vcpkg_port_versions_file() -> String {
        build(&Self::prefix(), "vcpkg.index/port_versions.dat")
    }

    pub fn vcpkg_check_point_file() -> String {
        build(&Self::prefix(), "vcpkg.index/check_point.toml")
    }
}

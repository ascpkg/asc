use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

use crate::util;

pub struct ConfigPath {}

impl ConfigPath {
    fn prefix() -> String {
        if let Some(dir) = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let config_dir = dir.config_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&config_dir) {
                if !util::fs::create_dirs(&config_dir) {
                    return String::new();
                }
            }
            return config_dir;
        }
        return String::new();
    }

    pub fn vcpkg_toml() -> String {
        build(&Self::prefix(), "vcpkg.toml")
    }
}

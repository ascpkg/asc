use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

use crate::util;

pub struct ConfigDir {}

impl ConfigDir {
    fn prefix() -> String {
        if let Some(dir) = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let config_dir = dir.config_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&config_dir) {
                if !util::fs::create_dir(&config_dir) {
                    return String::new();
                }
            }
            return config_dir;
        }
        return String::new();
    }

    pub fn vcpkg() -> String {
        build(&Self::prefix(), "vcpkg.toml")
    }
}

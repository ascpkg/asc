use crate::config::relative_paths;

use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

pub struct ConfigPath {}

impl ConfigPath {
    fn prefix() -> String {
        if let Some(dir) = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            return dir.config_dir().to_str().unwrap().replace(r"\", "/");
        }
        return String::new();
    }

    pub fn vcpkg_toml() -> String {
        build(
            &Self::prefix(),
            vec![String::from(relative_paths::VCPKG_TOML_FILE_NAME)],
            true,
            false,
        )
    }

    pub fn arch_os_to_vcpkg_triplet() -> String {
        build(
            &Self::prefix(),
            vec![String::from(
                relative_paths::ARCH_OS_TO_VCPKG_TRIPLET_FILE_NAME,
            )],
            true,
            false,
        )
    }
}

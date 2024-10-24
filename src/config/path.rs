use super::data::InstalledFiles;
use crate::{types::toml::TomlContainer, util};

pub static PROJECT_TOML: &str = "asc.toml";
pub static PROJECT_EDITION: &str = "2024";
pub static PROJECT_TARGET_DIR: &str = "target";
pub static PROJECT_INSTALL_DIR: &str = "target/installed";
pub static INSTALL_FILES_PATH: &str = "target/install_files.toml";
pub static INSTALL_BIN_DIR: &str = "bin";
pub static INSTALL_LIB_DIR: &str = "lib";
pub static INSTALL_INCLUDE_DIR: &str = "include";
pub static INSTALL_SHARE_DIR: &str = "share";
pub static PROJECT_SRC_DIR: &str = "src";
pub static PROJECT_BIN_SRC: &str = "main.cpp";
pub static PROJECT_LIB_HEADER: &str = "lib.hpp";
pub static PROJECT_LIB_SRC: &str = "lib.cpp";
pub static PROJECT_EXPORT_SRC: &str = "export.h";

pub fn clean() -> bool {
    if util::fs::is_dir_exists(PROJECT_TARGET_DIR) {
        let mut temp = None;
        if let Some(data) = TomlContainer::<InstalledFiles>::load(INSTALL_FILES_PATH, false) {
            if data.prefix != PROJECT_INSTALL_DIR {
                temp = Some(data);
            }
        }

        let result = util::fs::remove_dirs(PROJECT_TARGET_DIR);

        if let Some(data) = temp {
            if util::fs::create_dir(PROJECT_TARGET_DIR) {
                TomlContainer::new(data, INSTALL_FILES_PATH).dump();
            }
        }

        return result;
    }
    return true;
}

pub fn uninstall() -> bool {
    let mut has_error = false;

    if let Some(data) = TomlContainer::<InstalledFiles>::load(INSTALL_FILES_PATH, false) {
        let mut dir_paths = std::collections::HashSet::new();

        for path in &data.files {
            dir_paths.insert(
                std::path::Path::new(path)
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
            has_error &= util::fs::remove_file(path);
        }

        for dir_name in [INSTALL_INCLUDE_DIR, INSTALL_SHARE_DIR] {
            for dir_path in dir_paths.iter() {
                if dir_path.contains(&format!("/{dir_name}")) {
                    if util::fs::remove_dirs(dir_path) {
                        tracing::info!(func = "util::fs::remove_dirs", path = dir_path);
                    }
                }
            }
        }

        has_error &= util::fs::remove_file(INSTALL_FILES_PATH);
    }

    return has_error;
}

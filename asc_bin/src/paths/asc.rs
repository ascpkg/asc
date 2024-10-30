use crate::{config::project::InstalledFiles, util};

use super::{INSTALL_INCLUDE_DIR_NAME, INSTALL_SHARE_DIR_NAME};

pub static ASC_TOML_FILE_NAME: &str = "asc.toml";
pub static ASC_EDITION: &str = "2024";
pub static ASC_TARGET_DIR_NAME: &str = "target";
pub static ASC_TARGET_INSTALLED_DIR: &str = "target/installed";
pub static ASC_TARGET_INSTALLED_FILES_TOML_PATH: &str = "target/installed_files.toml";

pub static SRC_DIR_NAME: &str = "src";
pub static MAIN_CPP_FILE_NAME: &str = "main.cpp";
pub static LIB_HPP_FILE_NAME: &str = "lib.hpp";
pub static LIB_CPP_FILE_NAME: &str = "lib.cpp";
pub static EXPORT_H_FILE_NAME: &str = "export.h";

pub fn clean_asc_files() -> bool {
    if util::fs::is_dir_exists(ASC_TARGET_DIR_NAME) {
        let mut config = None;
        if let Some(data) = InstalledFiles::load(ASC_TARGET_INSTALLED_FILES_TOML_PATH, false) {
            if data.prefix != ASC_TARGET_INSTALLED_DIR {
                config = Some(data);
            }
        }

        let result = util::fs::remove_dirs(ASC_TARGET_DIR_NAME);

        if let Some(data) = config {
            if util::fs::create_dir(ASC_TARGET_DIR_NAME) {
                data.dump(false);
            }
        }

        return result;
    }
    return true;
}

pub fn uninstall_installed_files() -> bool {
    let mut has_error = false;

    if let Some(data) = InstalledFiles::load(ASC_TARGET_INSTALLED_FILES_TOML_PATH, false) {
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

        for dir_name in [INSTALL_INCLUDE_DIR_NAME, INSTALL_SHARE_DIR_NAME] {
            for dir_path in dir_paths.iter() {
                if dir_path.contains(&format!("/{dir_name}")) {
                    if util::fs::remove_dirs(dir_path) {
                        tracing::info!(func = "util::fs::remove_dirs", path = dir_path);
                    }
                }
            }
        }

        has_error &= util::fs::remove_file(ASC_TARGET_INSTALLED_FILES_TOML_PATH);
    }

    return has_error;
}

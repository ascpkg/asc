use crate::{config::project::InstalledFiles, util};

use super::{CMAKE_INSTALL_INCLUDE_DIR_NAME, CMAKE_INSTALL_SHARE_DIR_NAME};

pub static ASC_TOML_FILE_NAME: &str = "asc.toml";
pub static ASC_EDITION: &str = "2024";
pub static ASC_PROJECT_DIR_NAME: &str = ".asc";
pub static ASC_PROJECT_INSTALLED_FILES_TOML_PATH: &str = ".asc/installed_files.toml";
pub static ASC_TARGET_DIR_NAME: &str = "target";
pub static ASC_TARGET_INSTALLED_DIR: &str = "target/installed";

pub static SRC_DIR_NAME: &str = "src";
pub static MAIN_CPP_FILE_NAME: &str = "main.cpp";
pub static LIB_HPP_FILE_NAME: &str = "lib.hpp";
pub static LIB_CPP_FILE_NAME: &str = "lib.cpp";
pub static EXPORT_H_FILE_NAME: &str = "export.h";

pub fn clean_target_files() -> bool {
    if util::fs::is_dir_exists(ASC_TARGET_DIR_NAME) {
        if !util::fs::remove_dirs(ASC_TARGET_DIR_NAME) {
            return false;
        }
    }

    util::fs::remove_dir(ASC_PROJECT_DIR_NAME);

    return true;
}

pub fn uninstall_installed_files() -> bool {
    let mut has_error = false;

    if let Some(data) = InstalledFiles::load(ASC_PROJECT_INSTALLED_FILES_TOML_PATH, false) {
        let mut dir_paths = std::collections::HashSet::new();

        for path in &data.files {
            dir_paths.insert(util::fs::get_parent_dir(path));
            has_error &= util::fs::remove_file(path);
        }

        for dir_name in [CMAKE_INSTALL_INCLUDE_DIR_NAME, CMAKE_INSTALL_SHARE_DIR_NAME] {
            for dir_path in dir_paths.iter() {
                if dir_path.contains(&format!("/{dir_name}")) {
                    if util::fs::remove_dirs(dir_path) {
                        tracing::info!(func = "util::fs::remove_dirs", path = dir_path);
                    }
                }
            }
        }

        has_error &= util::fs::remove_file(ASC_PROJECT_INSTALLED_FILES_TOML_PATH);
    }

    return has_error;
}

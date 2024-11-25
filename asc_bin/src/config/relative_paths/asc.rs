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

pub static VCPKG_INSTALLED_DIR_NAME: &str = "vcpkg_installed";
pub static VCPKG_BIN_DIR_NAME: &str = "bin";
pub static VCPKG_LIB_DIR_NAME: &str = "lib";

pub static VERSION_TXT_FILE_NAME: &str = "version.txt";
pub static INNO_SETUP_ISS_FILE_NAME: &str = "setup.iss";

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

pub fn vcpkg_installed_bin_dir_path(profile: &str, triplet: &str) -> String {
    format!(
        "{ASC_TARGET_DIR_NAME}/{VCPKG_INSTALLED_DIR_NAME}/{triplet}/{}{VCPKG_BIN_DIR_NAME}",
        if profile == "release" { "" } else { "debug/" }
    )
}

pub fn vcpkg_installed_lib_dir_path(profile: &str, triplet: &str) -> String {
    format!(
        "{ASC_TARGET_DIR_NAME}/{VCPKG_INSTALLED_DIR_NAME}/{triplet}/{}{VCPKG_LIB_DIR_NAME}",
        if profile == "release" { "" } else { "debug/" }
    )
}

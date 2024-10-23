use crate::util;

pub static PROJECT_TOML: &str = "asc.toml";
pub static PROJECT_EDITION: &str = "2024";
pub static PROJECT_TARGET_DIR: &str = "target";
pub static INSTALL_FILES_PATH: &str = "target/install.files.txt";
pub static INSTALL_BIN_DIR: &str = "bin";
pub static INSTALL_LIB_DIR: &str = "lib";
pub static INSTALL_INCLUDE_DIR: &str = "include";
pub static INSTALL_SHARE_DIR: &str = "share";
pub static PROJECT_SRC_DIR: &str = "src";
pub static PROJECT_BIN_SRC: &str = "main.cpp";
pub static PROJECT_LIB_HDR: &str = "lib.hpp";
pub static PROJECT_LIB_SRC: &str = "lib.cpp";
pub static PROJECT_EXPORT_SRC: &str = "export.h";

pub fn clean() -> bool {
    if util::fs::is_dir_exists(PROJECT_TARGET_DIR) {
        return util::fs::remove_dirs(PROJECT_TARGET_DIR);
    }
    return true;
}

pub fn uninstall() -> bool {
    let mut has_error = false;

    let mut dir_paths = std::collections::HashSet::new();
    if let Ok(text) = std::fs::read_to_string(INSTALL_FILES_PATH) {
        for path in text.split("\n") {
            if !path.is_empty() {
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

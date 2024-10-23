use crate::util;

pub static PROJECT_TOML: &str = "asc.toml";
pub static PROJECT_EDITION: &str = "2024";
pub static PROJECT_TARGET_DIR: &str = "target";
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

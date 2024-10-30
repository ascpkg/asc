use crate::templates;
use crate::util;

pub static INSTALL_BIN_DIR_NAME: &str = "bin";
pub static INSTALL_LIB_DIR_NAME: &str = "lib";
pub static INSTALL_INCLUDE_DIR_NAME: &str = "include";
pub static INSTALL_SHARE_DIR_NAME: &str = "share";

pub static CMAKE_LISTS_TXT_FILE_NAME: &str = "CMakeLists.txt";
pub static VERSION_H_IN_FILE_NAME: &str = "version.h.in";
pub static CONFIG_H_CM_FILE_NAME: &str = "config.h.cm";
pub static USER_CMAKE_FILE_NAME: &str = "user.cmake";

pub fn get_config_cmake_in_file_name(project: &str) -> String {
    format!("{}-config.cmake.in", project)
}

pub fn clean_cmake_files(name: &str) -> bool {
    let mut has_error = false;

    for path in [CMAKE_LISTS_TXT_FILE_NAME, VERSION_H_IN_FILE_NAME] {
        if util::fs::is_file_exists(path) {
            has_error &= util::fs::remove_file(path);
        }
    }

    if !name.is_empty() {
        has_error &= util::fs::remove_file(&get_config_cmake_in_file_name(name));
    }

    if let Ok(text) = std::fs::read_to_string(CONFIG_H_CM_FILE_NAME) {
        if text == templates::CONFIG_H_CM_HBS {
            has_error &= util::fs::remove_file(CONFIG_H_CM_FILE_NAME);
        }
    }

    if let Ok(text) = std::fs::read_to_string(USER_CMAKE_FILE_NAME) {
        if text == templates::USER_CMAKE_HBS {
            has_error &= util::fs::remove_file(USER_CMAKE_FILE_NAME);
        }
    }

    return has_error;
}
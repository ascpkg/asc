use crate::templates;
use crate::util;

pub static CMAKE_LISTS_PATH: &str = "CMakeLists.txt";
pub static VERSION_H_IN_PATH: &str = "version.h.in";
pub static CONFIG_H_CM_PATH: &str = "config.h.cm";
pub static USER_CMAKE_PATH: &str = "user.cmake";

pub fn config_cmake_in_path(project: &str) -> String {
    format!("{}-config.cmake.in", project)
}

pub fn clean(name: &str) -> bool {
    let mut has_error = false;

    for path in [CMAKE_LISTS_PATH, VERSION_H_IN_PATH] {
        if util::fs::is_file_exists(path) {
            has_error &= util::fs::remove_file(path);
        }
    }

    if !name.is_empty() {
        has_error &= util::fs::remove_file(&config_cmake_in_path(name));
    }

    if let Ok(text) = std::fs::read_to_string(CONFIG_H_CM_PATH) {
        if text == templates::CONFIG_H_CM_HBS {
            has_error &= util::fs::remove_file(CONFIG_H_CM_PATH);
        }
    }

    if let Ok(text) = std::fs::read_to_string(USER_CMAKE_PATH) {
        if text == templates::USER_CMAKE_HBS {
            has_error &= util::fs::remove_file(USER_CMAKE_PATH);
        }
    }

    return has_error;
}

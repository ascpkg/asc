use crate::util;

pub fn cmake_lists_path(options: &util::cli::Options) -> String {
    format!("{}/CMakeLists.txt", &options.project_dir)
}

pub fn version_h_in_path(options: &util::cli::Options) -> String {
    format!("{}/version.h.in", &options.project_dir)
}

pub fn config_cmake_in_path(options: &util::cli::Options) -> String {
    format!(
        "{}/{}-config.cmake.in",
        &options.project_dir, &options.project
    )
}

pub fn config_h_cm_path(options: &util::cli::Options) -> String {
    format!("{}/config.h.cm", &options.project_dir)
}

pub fn check_cmake_path(options: &util::cli::Options) -> String {
    format!("{}/check.cmake", &options.project_dir)
}

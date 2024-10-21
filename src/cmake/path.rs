use crate::cli;

pub fn cmake_lists_path(_options: &cli::commands::scan::ScanOptions) -> String {
    String::from("CMakeLists.txt")
}

pub fn version_h_in_path(_options: &cli::commands::scan::ScanOptions) -> String {
    String::from("version.h.in")
}

pub fn config_cmake_in_path(options: &cli::commands::scan::ScanOptions) -> String {
    format!("{}-config.cmake.in", &options.project)
}

pub fn config_h_cm_path(_options: &cli::commands::scan::ScanOptions) -> String {
    String::from("config.h.cm")
}

pub fn check_cmake_path(_options: &cli::commands::scan::ScanOptions) -> String {
    String::from("check.cmake")
}

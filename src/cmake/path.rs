use crate::cli;

pub static CMAKE_LISTS_PATH: &str = "CMakeLists.txt";
pub static VERSION_H_IN_PATH: &str = "version.h.in";
pub static CONFIG_H_CM_PATH: &str = "config.h.cm";
pub static CHECK_CMAKE_PATH: &str = "check.cmake";

pub fn config_cmake_in_path(options: &cli::commands::scan::ScanOptions) -> String {
    format!("{}-config.cmake.in", &options.project)
}

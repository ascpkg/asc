// project
pub static VCPKG_JSON_FILE_NAME: &str = "vcpkg.json";
pub static VCPKG_CONFIGURATION_JSON_FILE_NAME: &str = "vcpkg-configuration.json";

// vcpkg
pub static VCPKG_DIR_NAME: &str = "vcpkg";
pub static VCPKG_VERSIONS_DIR_NAME: &str = "versions";
pub static VCPKG_BASELINE_JSON_FILE_NAME: &str = "baseline.json";
pub static VCPKG_PORTS_DIR_NAME: &str = "ports/";
pub static VCPKG_SCRIPTS_DIR_NAME: &str = "scripts";
pub static VCPKG_BUILD_SYSTEMS_DIR_NAME: &str = "buildsystems";
pub static VCPKG_CMAKE_FILE_NAME: &str = "vcpkg.cmake";

pub fn vcpkg_versions_baseline_json() -> String {
    format!("{VCPKG_VERSIONS_DIR_NAME}/{VCPKG_BASELINE_JSON_FILE_NAME}")
}

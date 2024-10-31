// project
pub static VCPKG_JSON_FILE_NAME: &str = "vcpkg.json";
pub static VCPKG_CONFIGURATION_JSON_FILE_NAME: &str = "vcpkg-configuration.json";

// vcpkg
pub static VCPKG_DIR_NAME: &str = "vcpkg";
pub static VCPKG_VERSIONS_DIR_NAME: &str = "versions";
pub static VCPKG_BASELINE_JSON_FILE_NAME: &str = "baseline.json";
pub static VCPKG_PORTS_DIR_NAME: &str = "ports/";

pub fn vcpkg_versions_baseline_json() -> String {
    format!("{VCPKG_VERSIONS_DIR_NAME}/{VCPKG_BASELINE_JSON_FILE_NAME}")
}

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

// vcpkg.index
pub static VCPKG_INDEX_DIR_NAME: &str = "vcpkg.index";
pub static VCPKG_SEARCH_INDEX_JSON_FILE_NAME: &str = "search_index.json";
pub static VCPKG_TREE_INDEX_JSON_FILE_NAME: &str = "tree_index.json";

// vcpkg.binary.cache
pub static VCPKG_BINARY_CACHE_DIR_NAME: &str = "vcpkg.binary.cache";

pub fn vcpkg_versions_baseline_json() -> String {
    format!("{VCPKG_VERSIONS_DIR_NAME}/{VCPKG_BASELINE_JSON_FILE_NAME}")
}

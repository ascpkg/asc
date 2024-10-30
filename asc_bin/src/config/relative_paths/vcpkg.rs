pub static VCPKG_JSON: &str = "vcpkg.json";
pub static VCPKG_CONFIGURATION_JSON: &str = "vcpkg-configuration.json";
static VCPKG_VERSIONS_DIR_NAME: &str = "versions";
static VCPKG_VERSIONS_BASELINE_JSON_PATH: &str = "versions/baseline.json";

pub fn get_versions_port_json_path(vcpkg_root_dir: &str, port_name: &str) -> String {
    format!(
        "{vcpkg_root_dir}/{VCPKG_VERSIONS_DIR_NAME}/{}-/{port_name}.json",
        port_name.chars().nth(0).unwrap()
    )
}

pub fn get_versions_baseline_json_path(vcpkg_root_dir: &str) -> String {
    format!("{vcpkg_root_dir}/{VCPKG_VERSIONS_BASELINE_JSON_PATH}")
}

use crate::{
    cli::commands::VcpkgArgs,
    config::relative_paths::{
        VCPKG_BOOTSTRAP_BAT_FILE_NAME, VCPKG_BOOTSTRAP_SH_FILE_NAME, VCPKG_DIR_NAME,
    },
    util,
};

pub fn run() {
    let vcpkg_conf = VcpkgArgs::load_or_default();
    let vcpkg_clone_dir = vcpkg_conf.get_public_registry().3;
    if vcpkg_clone_dir.is_empty() || !util::fs::is_dir_exists(&vcpkg_clone_dir) {
        return;
    }

    let (bootstrap_script_path, vcpkg_path) = if cfg!(target_os = "windows") {
        (
            format!("{vcpkg_clone_dir}/{VCPKG_BOOTSTRAP_BAT_FILE_NAME}"),
            format!("{vcpkg_clone_dir}/{VCPKG_DIR_NAME}.exe"),
        )
    } else {
        (
            format!("{vcpkg_clone_dir}/{VCPKG_BOOTSTRAP_SH_FILE_NAME}"),
            format!("{vcpkg_clone_dir}/{VCPKG_DIR_NAME}"),
        )
    };
    if util::fs::is_file_exists(&vcpkg_path) || !util::fs::is_file_exists(&bootstrap_script_path) {
        return;
    }

    let _output =
        util::shell::run(&bootstrap_script_path, &vec![], ".", false, false, false).unwrap();
}

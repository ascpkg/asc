use crate::{config::relative_paths::VCPKG_PORTS_DIR_NAME, util};

pub fn run(rev: &str, port_name: &str, repo_root_dir: &str) -> String {
    let output = util::shell::run(
        "git",
        &vec![
            "rev-parse",
            &format!("{rev}:{VCPKG_PORTS_DIR_NAME}{port_name}"),
        ],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

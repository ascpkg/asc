use crate::{config::relative_paths, util};

pub fn run(repo_root_dir: &str, hash: &str) -> String {
    let output = util::shell::run(
        "git",
        &vec![
            "show",
            &format!(
                "{}:{}",
                hash,
                relative_paths::vcpkg_versions_baseline_json()
            ),
        ],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn file_content(repo_root_dir: &str, hash: &str) -> String {
    let output =
        util::shell::run("git", &vec!["show", hash], repo_root_dir, true, false, true).unwrap();
    String::from_utf8_lossy(&output.stdout).to_string()
}

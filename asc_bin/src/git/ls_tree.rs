use crate::{config::relative_paths::VCPKG_PORTS_DIR_NAME, util};

pub fn run(git_commit_hash: &str, repo_root_dir: &str, silent: bool) -> Vec<(String, String)> {
    let mut results = vec![];

    let output = util::shell::run(
        "git",
        &vec![
            "ls-tree",
            "-d",
            "-r",
            "--full-tree",
            git_commit_hash,
            VCPKG_PORTS_DIR_NAME,
        ],
        repo_root_dir,
        true,
        false,
        silent,
    )
    .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    for line in stdout.split("\n") {
        let s = line.trim();
        if !s.is_empty() {
            let right = s.split_once(" tree ").unwrap().1;
            let parts: Vec<&str> = right
                .split(VCPKG_PORTS_DIR_NAME)
                .map(|s| s.trim())
                .collect();
            if parts.len() == 2 {
                results.push((parts[0].to_string(), parts[1].to_string()));
            }
        }
    }

    return results;
}

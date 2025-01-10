use std::collections::BTreeMap;

use crate::{
    config::relative_paths::{VCPKG_CONTROL_FILE_NAME, VCPKG_JSON_FILE_NAME, VCPKG_PORTS_DIR_NAME},
    util,
};

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

pub fn list_ports(
    git_commit_hash: &str,
    repo_root_dir: &str,
    silent: bool,
) -> BTreeMap<String, (String, String)> {
    let output = util::shell::run(
        "git",
        &vec!["ls-tree", "-r", git_commit_hash, VCPKG_PORTS_DIR_NAME],
        repo_root_dir,
        true,
        false,
        silent,
    )
    .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    let control_file_delimiter = format!("/{VCPKG_CONTROL_FILE_NAME}");
    let vcpkg_json_file_delimiter = format!("/{VCPKG_JSON_FILE_NAME}");
    let mut port_manifest_text = BTreeMap::new();
    for line in stdout.lines() {
        if line.ends_with(VCPKG_CONTROL_FILE_NAME) {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let text = super::show::tree_file_content(repo_root_dir, parts[2]);
            let name = parts[3]
                .split_once(VCPKG_PORTS_DIR_NAME)
                .unwrap()
                .1
                .rsplit_once(&control_file_delimiter)
                .unwrap()
                .0
                .to_string();
            port_manifest_text.insert(name, (text, String::new()));
        } else if line.ends_with(VCPKG_JSON_FILE_NAME) {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let text = super::show::tree_file_content(repo_root_dir, parts[2]);
            let name = parts[3]
                .split_once(VCPKG_PORTS_DIR_NAME)
                .unwrap()
                .1
                .rsplit_once(&vcpkg_json_file_delimiter)
                .unwrap()
                .0
                .to_string();
            port_manifest_text.insert(name, (String::new(), text));
        }
    }
    return port_manifest_text;
}

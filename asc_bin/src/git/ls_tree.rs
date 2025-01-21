use std::collections::{BTreeMap, HashMap};

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
    manifest_text_cache: &HashMap<String, String>,
    silent: bool,
) -> (i32, i32, BTreeMap<String, (String, String, String)>) {
    let mut caches = 0;
    let mut missings = 0;

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
            let tree_hash = parts[2].trim().to_string();
            let text = match manifest_text_cache.get(&tree_hash) {
                Some(t) => {
                    caches += 1;
                    t.clone()
                }
                None => {
                    missings += 1;
                    super::show::tree_file_content(repo_root_dir, &tree_hash)
                }
            };
            let name = parts[3]
                .split_once(VCPKG_PORTS_DIR_NAME)
                .unwrap()
                .1
                .rsplit_once(&control_file_delimiter)
                .unwrap()
                .0
                .to_string();
            port_manifest_text.insert(name, (tree_hash, text, String::new()));
        } else if line.ends_with(VCPKG_JSON_FILE_NAME) {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let tree_hash = parts[2].trim().to_string();
            let text = match manifest_text_cache.get(&tree_hash) {
                Some(t) => {
                    caches += 1;
                    t.clone()
                }
                None => {
                    missings += 1;
                    super::show::tree_file_content(repo_root_dir, &tree_hash)
                }
            };
            let name = parts[3]
                .split_once(VCPKG_PORTS_DIR_NAME)
                .unwrap()
                .1
                .rsplit_once(&vcpkg_json_file_delimiter)
                .unwrap()
                .0
                .to_string();
            port_manifest_text.insert(name, (tree_hash, String::new(), text));
        }
    }
    return (caches, missings, port_manifest_text);
}

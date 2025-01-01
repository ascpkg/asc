use crate::util;

pub fn commit_file_content(repo_root_dir: &str, commit_hash: &str, path: &str) -> String {
    let output = util::shell::run(
        "git",
        &vec!["show", &format!("{commit_hash}:{path}")],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn tree_file_content(repo_root_dir: &str, tree_hash: &str) -> String {
    let output = util::shell::run(
        "git",
        &vec!["show", tree_hash],
        repo_root_dir,
        true,
        false,
        true,
    )
    .unwrap();
    String::from_utf8_lossy(&output.stdout).to_string()
}

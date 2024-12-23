use crate::util;

pub fn run(repo_root_dir: &str, branch: &str, commit: &str) -> bool {
    let remote_branch = format!("origin/{branch}");
    util::shell::run(
        "git",
        &vec![
            "reset",
            "--hard",
            if commit.is_empty() {
                &remote_branch
            } else {
                commit
            },
        ],
        repo_root_dir,
        false,
        false,
        false,
    )
    .is_ok()
}

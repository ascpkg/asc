use crate::util;

pub fn run(repo_root_dir: &str, branch: &str) -> bool {
    util::shell::run(
        "git",
        &vec!["reset", "--hard", &format!("origin/{branch}")],
        repo_root_dir,
        false,
        false,
        false,
    )
    .is_ok()
}

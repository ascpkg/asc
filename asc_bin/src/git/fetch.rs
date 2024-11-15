use crate::util;

pub fn run(repo_root_dir: &str) -> bool {
    util::shell::run("git", &vec!["fetch"], repo_root_dir, false, false, false).is_ok()
}

use crate::util;

pub fn run(message: String, repo_root_dir: &String) {
    let _output = util::shell::run(
        "git",
        &vec!["commit", "-m", &message],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();
}

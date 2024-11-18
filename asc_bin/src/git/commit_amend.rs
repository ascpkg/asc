use crate::util;

pub fn run(repo_root_dir: &String) {
    let _output = util::shell::run(
        "git",
        &vec!["commit", "--amend", "--no-edit"],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();
}

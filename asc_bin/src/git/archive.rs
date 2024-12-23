use crate::util;

pub fn run(repo_root_dir: &str, format: &str, output: &str, rev: &str, path: &str) {
    let _output = util::shell::run(
        "git",
        &vec![
            "archive",
            "--format",
            format,
            "--output",
            output,
            &format!("{rev}:{path}"),
        ],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();
}

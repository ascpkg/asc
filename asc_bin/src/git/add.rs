use crate::util;

pub fn run(targets: &Vec<String>, repo_root_dir: &String) {
    for target in targets {
        let _output = util::shell::run(
            "git",
            &vec![
                "add",
                &util::fs::remove_prefix(target, repo_root_dir, repo_root_dir),
            ],
            repo_root_dir,
            true,
            false,
            false,
        )
        .unwrap();
    }
}

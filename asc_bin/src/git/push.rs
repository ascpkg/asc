use crate::util;

pub fn run(repo_root_dir: &String, force: bool) {
    let mut args = vec!["push"];
    if force {
        args.push("-f");
    }
    let _output = util::shell::run("git", &args, repo_root_dir, true, false, false).unwrap();
}

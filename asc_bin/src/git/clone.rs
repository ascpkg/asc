use crate::util;

pub fn run(url: &str, branch: &str, directory: &str, arguments: &Vec<String>) -> bool {
    let mut args = vec!["clone", "-b", branch, url, directory];
    for a in arguments {
        args.push(a);
    }

    return util::shell::run("git", &args, ".", false, false, false).is_ok();
}

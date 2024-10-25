use crate::util;

use super::VcpkgManager;

impl VcpkgManager {
    pub fn update(&mut self) -> bool {
        self.get();
        if !self.validate() {
            return false;
        }

        // clone if not exists
        if !util::fs::is_dir_exists(self.args.directory.as_ref().unwrap()) {
            let mut args = vec![
                "clone",
                "-b",
                self.args.branch.as_ref().unwrap(),
                self.args.repo.as_ref().unwrap(),
                self.args.directory.as_ref().unwrap(),
            ];
            for a in &self.args.args {
                args.push(&a);
            }

            return util::shell::run("git", &args, false, false).is_ok();
        } else {
            // fetch and reset
            let cwd = util::fs::get_cwd();
            util::fs::set_cwd(self.args.directory.as_ref().unwrap());

            let result = util::shell::run("git", &vec!["fetch"], false, false).is_ok()
                && util::shell::run(
                    "git",
                    &vec![
                        "reset",
                        "--hard",
                        &format!("origin/{}", self.args.branch.as_ref().unwrap()),
                    ],
                    false,
                    false,
                )
                .is_ok();

            util::fs::set_cwd(&cwd);
            return result;
        }
    }
}

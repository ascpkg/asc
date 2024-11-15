use crate::{git, util};

use super::VcpkgManager;

impl VcpkgManager {
    pub fn update(&mut self) -> bool {
        self.config_get(true);

        // clone if not exists
        if !util::fs::is_dir_exists(self.args.directory.as_ref().unwrap()) {
            return git::clone::run(
                self.args.repo.as_ref().unwrap(),
                self.args.branch.as_ref().unwrap(),
                self.args.directory.as_ref().unwrap(),
                &self.args.args,
            );
        } else {
            // fetch and reset
            let repo_root_dir = self.args.directory.as_ref().unwrap();
            let mut result = git::fetch::run(repo_root_dir);
            result &= git::reset::run(repo_root_dir, self.args.branch.as_ref().unwrap());

            return result;
        }
    }
}

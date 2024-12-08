use crate::{git, util};

use super::VcpkgManager;

impl VcpkgManager {
    pub fn update(&mut self) -> bool {
        self.config_get(true);

        let mut result = true;
        for (_name, url, branch, directory) in self.args.flatten_registry() {
            // clone if not exists
            if !util::fs::is_dir_exists(&directory) {
                result &= git::clone::run(&url, &branch, &directory, &self.args.args);
            } else {
                // fetch and reset
                result &= git::fetch::run(&directory);
                result &= git::reset::run(&directory, &branch);
            }
        }
        return result;
    }
}

use crate::cli::commands::VcpkgArgs;

use super::VcpkgManager;

impl VcpkgManager {
    pub fn config_set(&self) -> bool {
        // write conf to file
        let mut conf = VcpkgArgs::load_or_default();
        return conf.update(&self.args, true, true);
    }

    pub fn config_get(&mut self, silent: bool) {
        // read conf from file or set with defaults
        let conf = VcpkgArgs::load_or_default();
        self.args.update(&conf, false, false);

        if !silent {
            tracing::info!("{:#?}", self.args);
        }
    }
}

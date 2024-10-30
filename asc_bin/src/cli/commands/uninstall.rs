use clap::Args;

use crate::{config::relative_paths, util};

#[derive(Args, Debug, Clone)]
pub struct UninstallArgs {}

impl UninstallArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "uninstall", name = util::fs::get_cwd_name());

        relative_paths::uninstall_installed_files()
    }
}

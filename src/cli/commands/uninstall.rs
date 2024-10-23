use clap::Args;

use super::ConfigType;
use crate::{config, util};

#[derive(Args, Debug, Clone)]
pub struct UninstallArgs {
    #[clap(long, default_value = "debug")]
    config: ConfigType,
}

impl UninstallArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "uninstall", name = util::fs::get_cwd_name());

        config::path::uninstall()
    }
}

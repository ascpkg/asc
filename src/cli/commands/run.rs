use super::ConfigType;

use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    #[clap(long, default_value = "Debug")]
    config: ConfigType,
}

impl RunArgs {
    pub fn exec(&self) -> bool {
        tracing::info!("run");

        false
    }
}

use crate::{cli::config, cmake};

use super::{scan::ScanOptions, ConfigType};

use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct BuildArgs {
    pub name: Option<String>,
    #[clap(long, default_value = "Debug")]
    config: ConfigType,
}

impl BuildArgs {
    pub fn exec(&self) -> bool {
        if self.name.is_none() || self.name.as_ref().unwrap().is_empty() {
            return false;
        }

        let options = ScanOptions {
            build_dir: config::PROJECT_TARGET_DIR.to_string(),
            cmake_config: self.config.as_ref().to_string(),
            ..Default::default()
        };
        cmake::build::run(&options);

        return true;
    }
}

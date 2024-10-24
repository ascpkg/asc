use clap::Args;

use super::{scan::ScanOptions, ConfigType};
use crate::{cmake, config, util};

#[derive(Args, Debug, Default, Clone)]
pub struct BuildArgs {
    pub name: Option<String>,
    
    #[clap(long)]
    config: ConfigType,
}

impl BuildArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "build", name = util::fs::get_cwd_name());

        if !config::data::ProjectConfig::is_project_inited(false) {
            return false;
        }

        if !config::data::ProjectConfig::is_source_scaned() {
            return false;
        }

        let options = ScanOptions {
            target_dir: config::path::PROJECT_TARGET_DIR.to_string(),
            cmake_config: self.config.as_ref().to_string(),
            ..Default::default()
        };
        cmake::build::exec(&options);

        return true;
    }
}

use clap::Args;

use super::{scan::ScanOptions, ConfigType};
use crate::{cmake, config, config::relative_paths, util};

#[derive(Args, Debug, Default, Clone)]
pub struct BuildArgs {
    pub name: Option<String>,

    #[clap(long, default_value = ConfigType::Debug.as_ref())]
    config: ConfigType,
}

impl BuildArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "build", name = util::fs::get_cwd_name());

        if !config::project::ProjectConfig::is_project_inited(false) {
            return false;
        }

        if !config::project::ProjectConfig::is_source_scaned() {
            return false;
        }

        let options = ScanOptions {
            target_dir: relative_paths::ASC_TARGET_DIR_NAME.to_string(),
            cmake_config: self.config.as_ref().to_string(),
            ..Default::default()
        };
        cmake::build::exec(&options);

        return true;
    }
}

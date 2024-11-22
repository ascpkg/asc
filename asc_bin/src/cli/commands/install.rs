use clap::Args;

use super::{scan::ScanOptions, ConfigType};
use crate::{cmake, config, config::relative_paths, util};

#[derive(Args, Debug, Default, Clone)]
/// install executable/headers/libraries
pub struct InstallArgs {
    /// install prefix
    #[clap(long, default_value = relative_paths::ASC_TARGET_INSTALLED_DIR)]
    pub prefix: String,

    /// release mode (default false)
    #[clap(long, default_value_t = false)]
    release: bool,

    /// package cli (7z, tar, issc, auto .7z on windows .tar.xz on others)
    #[clap(long, default_value = "")]
    pack_cli: String,
}

impl InstallArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "install", name = util::fs::get_cwd_name());

        if !config::project::ProjectConfig::is_project_inited(false) {
            return false;
        }

        if !config::project::ProjectConfig::is_source_scaned() {
            return false;
        }

        let options = ScanOptions {
            target_dir: relative_paths::ASC_TARGET_DIR_NAME.to_string(),
            cmake_config: ConfigType::from(self.release).as_ref().to_string(),
            ..Default::default()
        };
        cmake::install::exec(&options, &self.prefix, &self.pack_cli);

        return true;
    }
}

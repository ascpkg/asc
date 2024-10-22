use clap::Args;

use super::{scan::ScanOptions, ConfigType};
use crate::errors::ErrorTag;
use crate::{config, cmake};

#[derive(Args, Debug, Clone)]
pub struct BuildArgs {
    pub name: Option<String>,
    #[clap(long, default_value = "debug")]
    config: ConfigType,
}

impl BuildArgs {
    pub fn exec(&self) -> bool {
        match config::ProjectConfig::read_project_conf() {
            None => {
                tracing::error!(message = "please run asc init, asc scan first");
                false
            }
            Some(project_conf) => {
                if project_conf.package.is_none()
                    && project_conf.bins.is_none()
                    && project_conf.libs.is_none()
                {
                    tracing::error!(
                        error_tag = ErrorTag::InvalidProjectPackageError.as_ref(),
                        message = "package, bins, libs were not found"
                    );
                    return false;
                }

                let (target_name, _target_src) =
                    project_conf.get_target_name_src(&self.name, false, false);

                if target_name.is_empty() {
                    tracing::error!(
                        call = "target_name.is_empty",
                        error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                        message = "invalid name"
                    );
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
    }
}

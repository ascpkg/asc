use clap::Args;

use tracing;

use crate::{cmake, config, errors::ErrorTag, util};

#[derive(Args, Debug, Clone)]
pub struct CleanArgs {}

impl CleanArgs {
    pub fn exec(&self) -> bool {
        match config::project::ProjectConfig::read_project_conf() {
            None => {
                tracing::error!(error_tag = ErrorTag::InvalidProjectError.as_ref(),);
                return false;
            }
            Some(project_conf) => {
                if project_conf.workspace.is_some() {
                    return self.clean_workspace(&project_conf);
                } else {
                    if let Some(package) = project_conf.package {
                        return self.clean_package(&package.name);
                    } else {
                        tracing::error!(error_tag = ErrorTag::InvalidProjectPackageError.as_ref());
                        return false;
                    }
                }
            }
        }
    }

    fn clean_package(&self, name: &str) -> bool {
        tracing::info!(message = "clean package", name = name);

        // cmake
        let mut has_error = cmake::path::clean(name);

        // target
        has_error &= config::project::path::clean();

        return has_error;
    }

    fn clean_workspace(&self, package_conf: &config::project::ProjectConfig) -> bool {
        tracing::info!(message = "clean workspace", name = util::fs::get_cwd_name());

        // cmake
        let mut has_error = cmake::path::clean("");

        // target
        has_error &= config::project::path::clean();

        // members
        let cwd = util::fs::get_cwd();
        match &package_conf.workspace {
            None => {
                has_error = true;
                tracing::error!(error_tag = ErrorTag::InvalidProjectWorkspaceError.as_ref(),);
            }
            Some(workspace_config) => {
                if workspace_config.members.is_empty() {
                    has_error = true;
                    tracing::error!(error_tag = ErrorTag::InvalidProjectWorkspaceError.as_ref(),);
                }
                for m in &workspace_config.members {
                    util::fs::set_cwd(m);
                    has_error &= self.clean_package(&m);
                    util::fs::set_cwd(&cwd);
                }
            }
        }

        return !has_error;
    }
}

use std::collections::BTreeSet;

use clap::Args;

use crate::{
    config::{self, project::ProjectConfig, relative_paths::ASC_TOML_FILE_NAME},
    errors::ErrorTag,
    util, vcpkg,
};

#[derive(Args, Debug, Default, Clone)]
pub struct PublishArgs {
    edit: bool,
    package: Option<String>,
}

impl PublishArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "run");

        match config::project::ProjectConfig::read_project_conf() {
            None => {
                tracing::error!(
                    error_tag = ErrorTag::InvalidProjectError.as_ref(),
                    path = ASC_TOML_FILE_NAME
                );
                return false;
            }
            Some(project_conf) => match project_conf.workspace {
                None => {
                    return self.publish_package(&project_conf);
                }
                Some(workspace) => match &self.package {
                    None => {
                        tracing::error!(
                            error_tag = ErrorTag::InvalidProjectWorkspaceError.as_ref(),
                            path = ASC_TOML_FILE_NAME
                        );
                        return false;
                    }
                    Some(p) => {
                        if !workspace.members.contains(p) {
                            tracing::error!(
                                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                                packages = workspace.get_members()
                            );
                            return false;
                        } else {
                            let cwd = util::fs::get_cwd();
                            util::fs::set_cwd(p);
                            let result = if let Some(project_conf) =
                                config::project::ProjectConfig::read_project_conf()
                            {
                                self.publish_package(&project_conf)
                            } else {
                                tracing::error!(
                                    error_tag = ErrorTag::InvalidProjectWorkspaceError.as_ref(),
                                    packages = workspace.get_members()
                                );
                                false
                            };
                            util::fs::set_cwd(&cwd);
                            return result;
                        }
                    }
                },
            },
        }
    }

    fn publish_package(&self, project_conf: &ProjectConfig) -> bool {
        match &project_conf.package {
            None => {
                tracing::error!(
                    error_tag = ErrorTag::InvalidProjectPackageError.as_ref(),
                    path = project_conf.path,
                );
                return false;
            }
            Some(pkg) => {
                let mut result = vcpkg::json::gen_port_json(pkg, &project_conf.dependencies);
                result &= vcpkg::cmake::gen_port_file_cmake(pkg);
                if !self.edit {
                    result &= vcpkg::json::gen_port_versions();
                }
                return result;
            }
        }
    }
}

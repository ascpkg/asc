use clap::Args;

use crate::{config, errors::ErrorTag, util};

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    dependency: String,

    #[clap(long)]
    package: Option<String>,

    #[clap(long)]
    recurse: bool,
}

impl RemoveArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "remove", dependency = self.dependency);

        match config::project::ProjectConfig::read_project_conf() {
            None => {
                return false;
            }
            Some(mut project_conf) => match project_conf.workspace {
                None => {
                    return self.remove_pakcage(&mut project_conf);
                }
                Some(workspace) => match &self.package {
                    None => {
                        tracing::error!(
                            error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                            packages = workspace.get_members()
                        );
                        return false;
                    }
                    Some(member) => {
                        if !workspace.members.contains(member) {
                            tracing::error!(
                                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                                packages = workspace.get_members()
                            );
                            return false;
                        } else {
                            let cwd = util::fs::get_cwd();
                            util::fs::set_cwd(member);
                            let result = match config::project::ProjectConfig::read_project_conf() {
                                None => false,
                                Some(mut project_conf) => self.remove_pakcage(&mut project_conf),
                            };
                            util::fs::set_cwd(&cwd);
                            return result;
                        }
                    }
                },
            },
        }
    }

    fn remove_pakcage(&self, project_conf: &mut config::project::ProjectConfig) -> bool {
        if self.dependency.is_empty() {
            return false;
        } else {
            return project_conf.dependencies.remove(&self.dependency).is_some()
                && project_conf.dump(false);
        }
    }
}

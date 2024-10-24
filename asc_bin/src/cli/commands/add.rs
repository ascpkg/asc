use std::collections::BTreeSet;

use clap::Args;

use crate::{config, errors::ErrorTag, util};

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    name: String,

    #[clap(long)]
    member: Option<String>,

    #[clap(long)]
    ver: String,

    #[clap(long)]
    pub find_pkg: Vec<String>,

    #[clap(long)]
    pub link_lib: Vec<String>,

    #[clap(long)]
    features: Vec<String>,
}

impl AddArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "add", name = self.name);

        match config::project::ProjectConfig::read_project_conf() {
            None => {
                return false;
            }
            Some(mut project_conf) => match project_conf.workspace {
                None => {
                    return self.add_for_pakcage(&mut project_conf);
                }
                Some(workspace) => match &self.member {
                    None => {
                        tracing::error!(
                            error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                            members = workspace.get_members()
                        );
                        return false;
                    }
                    Some(member) => {
                        if !workspace.members.contains(member) {
                            tracing::error!(
                                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                                members = workspace.get_members()
                            );
                            return false;
                        } else {
                            let cwd = util::fs::get_cwd();
                            util::fs::set_cwd(member);
                            let result = match config::project::ProjectConfig::read_project_conf() {
                                None => false,
                                Some(mut project_conf) => self.add_for_pakcage(&mut project_conf),
                            };
                            util::fs::set_cwd(&cwd);
                            return result;
                        }
                    }
                },
            },
        }
    }

    fn add_for_pakcage(&self, project_conf: &mut config::project::ProjectConfig) -> bool {
        if self.name.is_empty() {
            return false;
        } else {
            project_conf.dependencies.insert(
                self.name.clone(),
                config::project::DependencyConfig {
                    version: self.ver.clone(),
                    find_pkg: self
                        .find_pkg
                        .iter()
                        .map(|s| s.clone())
                        .collect::<BTreeSet<String>>(),
                    link_lib: self
                        .link_lib
                        .iter()
                        .map(|s| s.clone())
                        .collect::<BTreeSet<String>>(),
                    features: self
                        .features
                        .iter()
                        .map(|s| s.clone())
                        .collect::<BTreeSet<String>>(),
                },
            );
            return project_conf.write_project_conf();
        }
    }
}

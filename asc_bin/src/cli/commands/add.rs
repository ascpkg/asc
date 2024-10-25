use std::collections::BTreeSet;

use clap::Args;

use crate::{config, errors::ErrorTag, util};

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    dependency: String,

    #[clap(long)]
    package: Option<String>,

    #[clap(long, default_value = "")]
    version: String,

    #[clap(long, help = "--find-package=a --find-package=b")]
    pub find_package: Vec<String>,

    #[clap(long, help = "--find-library=c --find-library=d")]
    pub link_library: Vec<String>,

    #[clap(long, help = "--feature=a --feature=b")]
    feature: Vec<String>,
}

impl AddArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "add", dependency = self.dependency);

        match config::project::ProjectConfig::read_project_conf() {
            None => {
                return false;
            }
            Some(mut project_conf) => match project_conf.workspace {
                None => {
                    return self.add_for_pakcage(&mut project_conf);
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
        if self.dependency.is_empty() {
            return false;
        } else {
            project_conf.dependencies.insert(
                self.dependency.clone(),
                config::project::DependencyConfig {
                    version: self.version.clone(),
                    find_packages: self
                        .find_package
                        .iter()
                        .map(|s| s.clone())
                        .collect::<BTreeSet<String>>(),
                    link_libraries: self
                        .link_library
                        .iter()
                        .map(|s| s.clone())
                        .collect::<BTreeSet<String>>(),
                    features: self
                        .feature
                        .iter()
                        .map(|s| s.clone())
                        .collect::<BTreeSet<String>>(),
                },
            );
            return project_conf.write_project_conf();
        }
    }
}

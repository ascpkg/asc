use clap::Args;

use crate::{
    config::{self, project::ProjectConfig, relative_paths::ASC_TOML_FILE_NAME},
    errors::ErrorTag,
    git, util, vcpkg,
};

use super::VcpkgArgs;

#[derive(Args, Debug, Default, Clone)]
/// publish package to vcpkg registry
pub struct PublishArgs {
    /// your private registry name
    #[clap(long)]
    registry: String,

    /// package or workspace member name
    #[clap(long)]
    package: Option<String>,

    /// git push or not
    #[clap(long, default_value_t = false)]
    push: bool,
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
                            error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                            packages = workspace.get_members()
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
                let latest_commit =
                    git::log::get_latest_commit(".", git::log::GIT_LOG_FORMAT_COMMIT_HASH_DATE);

                let vcpkg_conf = VcpkgArgs::load_or_default();
                let (_name, _url, _branch, repo_root_dir) =
                    vcpkg_conf.get_private_registry(&self.registry);
                if repo_root_dir.is_empty() {
                    return false;
                }

                // add version suffix to port name
                let mut pkg = pkg.clone();
                pkg.name = format!("{}-{}", pkg.name, pkg.version.replace(".", "-"));

                let dir =
                    config::system_paths::DataPath::vcpkg_ports_dir_path(&repo_root_dir, &pkg.name);
                let action = if util::fs::is_dir_exists(&dir) {
                    "update"
                } else {
                    "add"
                };

                let (mut result, port_version) = vcpkg::json::gen_port_json(
                    &repo_root_dir,
                    &pkg,
                    &project_conf.dependencies,
                    &latest_commit,
                );
                result &= vcpkg::cmake::gen_port_file_cmake(&repo_root_dir, &pkg, &latest_commit);
                if result {
                    git::add::run(&vec![dir], &repo_root_dir);
                    git::commit::run(
                        format!(
                            "[{}] {} {}#{} ({})",
                            &pkg.name, action, pkg.version, port_version, latest_commit.hash
                        ),
                        &repo_root_dir,
                    );
                }

                result &= vcpkg::json::gen_port_versions(
                    &repo_root_dir,
                    &pkg.name,
                    &Some(pkg.version.clone()),
                    &None,
                    &None,
                    &None,
                    port_version,
                );
                if result {
                    git::add::run(
                        &vec![
                            config::system_paths::DataPath::vcpkg_versions_port_json_path(
                                &repo_root_dir,
                                &pkg.name,
                            ),
                            config::system_paths::DataPath::vcpkg_versions_baseline_json_path(
                                &repo_root_dir,
                            ),
                        ],
                        &repo_root_dir,
                    );
                    git::commit_amend::run(&repo_root_dir);

                    if self.push {
                        git::push::run(&repo_root_dir);
                    }
                }

                return result;
            }
        }
    }
}

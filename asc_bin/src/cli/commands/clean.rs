use clap::Args;

use tracing;

use crate::{
    config::{self, project::ProjectConfig, relative_paths},
    errors::ErrorTag,
    util,
};

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
                    return self.clean_package(&project_conf);
                }
            }
        }
    }

    fn clean_package(&self, project_conf: &ProjectConfig) -> bool {
        let mut has_error = false;

        let cwd = util::fs::get_cwd();
        if let Some(bins) = &project_conf.bins {
            for bin in bins {
                util::fs::set_cwd(&format!(
                    "{}/{}",
                    relative_paths::ASC_PROJECT_DIR_NAME,
                    bin.name
                ));
                tracing::info!(message = "clean bin", name = bin.name);

                // cmake
                has_error &= relative_paths::clean_cmake_files(&bin.name);

                // graph
                has_error &= relative_paths::clean_graph_files();

                util::fs::set_cwd(&cwd);
            }
        }
        if let Some(libs) = &project_conf.libs {
            for lib in libs {
                util::fs::set_cwd(&format!(
                    "{}/{}",
                    relative_paths::ASC_PROJECT_DIR_NAME,
                    lib.name
                ));
                tracing::info!(message = "clean lib", name = lib.name);

                // cmake
                has_error &= relative_paths::clean_cmake_files(&lib.name);

                // graph
                has_error &= relative_paths::clean_graph_files();

                util::fs::set_cwd(&cwd);
            }
        }

        // cmake
        util::fs::set_cwd(relative_paths::ASC_PROJECT_DIR_NAME);
        has_error = relative_paths::clean_cmake_files("");
        util::fs::set_cwd(&cwd);

        // target
        has_error &= relative_paths::clean_asc_files();

        return has_error;
    }

    fn clean_workspace(&self, package_conf: &config::project::ProjectConfig) -> bool {
        tracing::info!(message = "clean workspace", name = util::fs::get_cwd_name());

        let cwd = util::fs::get_cwd();
        util::fs::set_cwd(relative_paths::ASC_PROJECT_DIR_NAME);

        // cmake
        let mut has_error = relative_paths::clean_cmake_files("");

        // target
        has_error &= relative_paths::clean_asc_files();

        // members
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
                let c: String = util::fs::get_cwd();
                for m in &workspace_config.members {
                    util::fs::set_cwd(m);

                    if let Some(project_conf) = config::project::ProjectConfig::read_project_conf()
                    {
                        has_error &= self.clean_package(&project_conf);
                    }

                    util::fs::set_cwd(&c);
                }
            }
        }

        util::fs::set_cwd(&cwd);

        return !has_error;
    }
}

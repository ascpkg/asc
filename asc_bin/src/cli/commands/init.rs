use std::collections::BTreeSet;

use clap::Args;

use crate::config::project::EntryConfig;
use crate::errors::ErrorTag;
use crate::util;
use crate::{config, config::relative_paths};

#[derive(Args, Debug, Clone, Default)]
pub struct InitArgs {
    #[clap(long, default_value_t = false)]
    pub lib: bool,

    #[clap(long, default_value_t = false)]
    pub shared: bool,

    #[clap(long, default_value_t = false)]
    pub workspace: bool,

    #[clap(long)]
    pub member: Vec<String>,
}

impl InitArgs {
    pub fn exec(&self) -> bool {
        if self.workspace {
            return self.init_workspace();
        } else if !self.lib {
            return self.init_bin(&self.name());
        } else {
            return self.init_lib(&self.name());
        }
    }

    pub fn name(&self) -> String {
        util::fs::get_cwd_name()
    }

    pub fn init_bin(&self, name: &str) -> bool {
        tracing::info!(message = "init bin", name = name);
        return self.init_package(name);
    }

    pub fn init_lib(&self, name: &str) -> bool {
        tracing::info!(message = "init bin", name = name);
        return self.init_package(name);
    }

    pub fn init_package(&self, name: &str) -> bool {
        tracing::info!(message = "init package", name = name);
        // validate args
        if name.is_empty() {
            tracing::error!(
                func = "name.is_empty",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
            );
            return false;
        }

        // skip if exists
        if config::project::ProjectConfig::is_project_inited(true) {
            return false;
        }

        let mut project = config::project::ProjectConfig::default();
        let mut package = config::project::PackageConfig::default();
        package.name = name.to_string();
        package.version = config::project::ProjectConfig::version_date();
        package.edition = relative_paths::ASC_EDITION.to_string();
        project.package = Some(package);

        if self.lib {
            project.libs = BTreeSet::from([EntryConfig {
                name: name.to_string(),
                source_dir: relative_paths::SRC_DIR_NAME.to_string(),
                source_file: relative_paths::LIB_CPP_FILE_NAME.to_string(),
                shared: if self.shared { Some(true) } else { Some(false) },
            }]);
        } else {
            project.bins = BTreeSet::from([EntryConfig {
                name: name.to_string(),
                source_dir: relative_paths::SRC_DIR_NAME.to_string(),
                source_file: relative_paths::MAIN_CPP_FILE_NAME.to_string(),
                shared: None,
            }]);
        }

        // write asc.toml
        return project.write_project_conf();
    }

    pub fn init_workspace(&self) -> bool {
        tracing::info!(message = "init workspace", name = util::fs::get_cwd());

        // validate args
        if self.member.is_empty() {
            tracing::error!(
                func = "self.member.is_empty",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
            );
            return false;
        }

        let cwd = util::fs::get_cwd();

        // init members
        let mut has_error = false;
        let mut workspace = config::project::WorkSpaceConfig::default();
        for m in &self.member {
            if workspace.members.insert(m.clone()) {
                let mut args = Self::default();
                args.lib = self.lib;

                util::fs::set_cwd(m);
                if !args.init_package(m) {
                    has_error = true;
                }
                util::fs::set_cwd(&cwd);
            }
        }
        let mut project = config::project::ProjectConfig::default();
        project.workspace = Some(workspace);

        // skip if exists
        if config::project::ProjectConfig::is_project_inited(true) {
            return false;
        }

        // write asc.toml
        return !has_error && project.validate() && project.write_project_conf();
    }
}

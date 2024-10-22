use clap::Args;

use crate::config;
use crate::errors::ErrorTag;
use crate::util;

#[derive(Args, Debug, Clone, Default)]
pub struct InitArgs {
    #[clap(long, default_value_t = false)]
    pub lib: bool,
    #[clap(long, default_value_t = false)]
    pub workspace: bool,
    pub member: Option<Vec<String>>,
}

impl InitArgs {
    pub fn exec(&self) -> bool {
        if self.workspace && self.member.is_some() {
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
        tracing::info!("init bin");
        return self.init_package(name);
    }

    pub fn init_lib(&self, name: &str) -> bool {
        tracing::info!("init bin");
        return self.init_package(name);
    }

    pub fn init_package(&self, name: &str) -> bool {
        tracing::info!("init package");
        // validate args
        if name.is_empty() {
            tracing::error!(
                call = "name.is_empty",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
            );
            return false;
        }

        // skip if exists
        if config::ProjectConfig::is_project_inited(true) {
            return false;
        }

        let mut project = config::ProjectConfig::default();
        let mut package = config::PackageConfig::default();
        package.name = name.to_string();
        package.version = config::ProjectConfig::version_date();
        package.edition = config::path::PROJECT_EDITION.to_string();
        project.package = Some(package);

        // write asc.toml
        return project.write_project_conf();
    }

    pub fn init_workspace(&self) -> bool {
        tracing::info!("init workspace");

        // validate args
        let members = self.member.as_ref().unwrap();
        if members.is_empty() {
            tracing::error!(
                call = "members.is_empty",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
            );
            return false;
        }

        let cwd = util::fs::get_cwd();

        // init members
        let mut has_error = false;
        let mut workspace = config::WorkSpaceConfig::default();
        for m in members {
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
        let mut project = config::ProjectConfig::default();
        project.workspace = Some(workspace);

        // skip if exists
        if config::ProjectConfig::is_project_inited(true) {
            return false;
        }

        // write asc.toml
        return !has_error && project.validate() && project.write_project_conf();
    }
}
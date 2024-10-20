use crate::cli::config;

use clap::Args;

#[derive(Args, Debug, Clone, Default)]
pub struct InitArgs {
    pub name: Option<String>,
    #[clap(long, default_value_t = false)]
    pub lib: bool,
    #[clap(long, default_value_t = false)]
    pub workspace: bool,
    pub member: Option<Vec<String>>,
}

impl InitArgs {
    pub fn exec(&self) -> bool {
        if self.name.is_some() {
            if self.workspace && self.member.is_some() {
                return self.init_workspace();
            } else if !self.lib {
                return self.init_bin(self.name.as_ref().unwrap());
            } else {
                return self.init_lib(self.name.as_ref().unwrap());
            }
        }
        return false;
    }

    pub fn init_bin(&self, name: &str) -> bool {
        return self.init_package(name);
    }

    pub fn init_lib(&self, name: &str) -> bool {
        return self.init_package(name);
    }

    pub fn init_package(&self, name: &str) -> bool {
        // validate args
        if name.is_empty() {
            return false;
        }

        // skip if exists
        if std::fs::metadata(config::PROJECT_TOML).is_ok() {
            tracing::error!(
                message = "std::fs::metadata",
                path = config::PROJECT_TOML,
                error = "exits"
            );
            return false;
        }

        let mut project = config::ProjectConfig::default();
        let mut package = config::PackageConfig::default();
        package.name = name.to_string();
        package.version = config::ProjectConfig::version_date();
        package.edition = config::PROJECT_EDITION.to_string();
        project.package = Some(package);

        // write asc.toml
        return project.dump(config::PROJECT_TOML);
    }

    pub fn init_workspace(&self) -> bool {
        // validate args
        let name = self.name.as_ref().unwrap();
        let members = self.member.as_ref().unwrap();
        if name.is_empty() || members.is_empty() {
            return false;
        }

        let cwd = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // init members
        let mut has_error = false;
        let mut workspace = config::WorkSpaceConfig::default();
        for m in members {
            if workspace.members.insert(m.clone()) {
                let mut args = Self::default();
                args.name = Some(m.clone());
                args.lib = self.lib;

                std::env::set_current_dir(m).unwrap();
                if !args.init_package(m) {
                    has_error = true;
                }
                std::env::set_current_dir(&cwd).unwrap();
            }
        }
        let mut project = config::ProjectConfig::default();
        project.workspace = Some(workspace);

        // skip if exists
        if std::fs::metadata(config::PROJECT_TOML).is_ok() {
            tracing::error!(
                message = "std::fs::metadata",
                path = config::PROJECT_TOML,
                error = "exits"
            );
            return false;
        }

        // write asc.toml
        return !has_error && project.validate() && project.dump(config::PROJECT_TOML);
    }
}

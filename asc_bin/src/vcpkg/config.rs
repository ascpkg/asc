use crate::{cli::commands::VcpkgArgs, config, errors::ErrorTag};

use super::VcpkgManager;

impl VcpkgManager {
    pub fn config_set(&self) -> bool {
        let config_path = config::system_paths::ConfigPath::vcpkg_toml();

        // write conf to file
        let mut conf = VcpkgArgs::load(&config_path, true).unwrap();
        if let Some(repo) = &self.args.repo {
            conf.repo = Some(repo.clone());
        }
        if let Some(branch) = &self.args.branch {
            conf.branch = Some(branch.clone());
        }
        if let Some(directory) = &self.args.directory {
            conf.directory = Some(directory.clone());
        }
        if let Some(env_default_binary_cache) = &self.args.env_download_dir {
            conf.env_download_dir = Some(env_default_binary_cache.clone());
        }

        return conf.dump(false);
    }

    pub fn config_get(&mut self, silent: bool) {
        let config_path = config::system_paths::ConfigPath::vcpkg_toml();

        match VcpkgArgs::load(&config_path, false) {
            None => {}
            Some(conf) => {
                // read conf from file
                if self.args.repo.is_none() {
                    if let Some(repo) = conf.repo {
                        self.args.repo = Some(repo);
                    }
                }
                if self.args.branch.is_none() {
                    if let Some(branch) = conf.branch {
                        self.args.branch = Some(branch);
                    }
                }
                if self.args.directory.is_none() {
                    if let Some(directory) = conf.directory {
                        self.args.directory = Some(directory);
                    }
                }
                // default directory
                if self.args.directory.is_none() {
                    self.args.directory = Some(config::system_paths::DataPath::vcpkg_clone_dir())
                }
            }
        }

        if !silent {
            tracing::info!("{:#?}", self.args);
        }
    }

    pub fn validate(&self) -> bool {
        if self.args.repo.is_none() && self.args.branch.is_none() && self.args.directory.is_none() {
            tracing::error!(
                call = "self.repo.is_none && self.branch.is_none && self.directory.is_none",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref()
            );
            return false;
        }
        return true;
    }
}

use directories;

use crate::{cli::commands::VcpkgArgs, errors::ErrorTag, util};

use super::VcpkgManager;

impl VcpkgManager {
    pub fn set(&self) -> bool {
        let config_path = self.config_path();
        if config_path.is_empty() {
            return false;
        }

        // write conf to file
        let mut conf = VcpkgArgs::load(&config_path, true).unwrap_or_default();
        if let Some(repo) = &self.args.repo {
            conf.repo = Some(repo.clone());
        }
        if let Some(branch) = &self.args.branch {
            conf.branch = Some(branch.clone());
        }
        if let Some(directory) = &self.args.directory {
            conf.directory = Some(directory.clone());
        }

        return conf.dump(false);
    }

    pub fn get(&mut self) {
        let config_path = self.config_path();
        if config_path.is_empty() {
            return;
        }

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
                    self.args.directory = Some(format!("{}/vcpkg", self.data_directory()))
                }
            }
        }

        tracing::info!("{:#?}", self.args);
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

    fn config_path(&self) -> String {
        if let Some(dir) = directories::ProjectDirs::from("", "", "asc") {
            let config_dir = dir.config_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&config_dir) {
                if !util::fs::create_dirs(&config_dir) {
                    return String::new();
                }
            }
            return format!("{config_dir}/vcpkg.toml");
        }
        return String::new();
    }

    fn data_directory(&self) -> String {
        if let Some(dir) = directories::ProjectDirs::from("", "", "asc") {
            let data_dir = dir.data_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&data_dir) {
                if !util::fs::create_dirs(&data_dir) {
                    return String::new();
                }
            }
            return data_dir;
        }
        return String::new();
    }
}

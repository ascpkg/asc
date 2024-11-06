pub mod config;
pub mod index;
pub mod json;
pub mod search;
pub mod update;

use std::collections::HashMap;

use struct_iterable::Iterable;

use crate::{
    cli::commands::vcpkg::VcpkgArgs,
    config::{relative_paths, system_paths},
};

pub struct VcpkgManager {
    args: VcpkgArgs,
}

impl VcpkgManager {
    pub fn new(args: VcpkgArgs) -> Self {
        Self { args: args }
    }
}

impl VcpkgArgs {
    pub fn load_or_default() -> Self {
        return VcpkgArgs::load(&system_paths::ConfigPath::vcpkg_toml(), false).unwrap_or_else(
            || {
                let mut default_conf: VcpkgArgs = VcpkgArgs::default();
                default_conf.path = system_paths::ConfigPath::vcpkg_toml();
                default_conf.set_defaults();
                default_conf.dump(false);

                default_conf
            },
        );
    }

    pub fn set_defaults(&mut self) {
        if self.repo.is_none() {
            self.repo = Some(relative_paths::VCPKG_MICROSOFT_REPO_URL.to_string());
        }

        if self.branch.is_none() {
            self.branch = Some(relative_paths::VCPKG_MICROSOFT_REPO_BRANCH_NAME.to_string());
        }

        if self.directory.is_none() {
            self.directory = Some(system_paths::DataPath::vcpkg_default_clone_dir())
        }

        if self.index_directory.is_none() {
            self.index_directory = Some(system_paths::DataPath::vcpkg_default_index_dir())
        }

        if self.env_downloads.is_none() {
            self.env_downloads = Some(system_paths::DataPath::vcpkg_default_downloads_dir())
        }

        if self.env_default_binary_cache.is_none() {
            self.env_default_binary_cache =
                Some(system_paths::DataPath::vcpkg_default_binary_cache_dir())
        }
    }

    pub fn update(&mut self, other: &Self, force: bool, dump: bool) -> bool {
        if force || self.repo.is_none() {
            if let Some(repo) = &other.repo {
                self.repo = Some(repo.clone());
            }
        }
        if force || self.branch.is_none() {
            if let Some(branch) = &other.branch {
                self.branch = Some(branch.clone());
            }
        }
        if force || self.directory.is_none() {
            if let Some(directory) = &other.directory {
                self.directory = Some(directory.clone());
            }
        }
        if force || self.index_directory.is_none() {
            if let Some(index_directory) = &other.index_directory {
                self.index_directory = Some(index_directory.clone());
            }
        }
        if force || self.env_downloads.is_none() {
            if let Some(env_downloads) = &other.env_downloads {
                self.env_downloads = Some(env_downloads.clone());
            }
        }
        if force || self.env_default_binary_cache.is_none() {
            if let Some(env_default_binary_cache) = &other.env_default_binary_cache {
                self.env_default_binary_cache = Some(env_default_binary_cache.clone());
            }
        }

        if dump {
            return self.dump(false);
        }

        return true;
    }

    pub fn get_envs(&self) -> HashMap<String, String> {
        let mut envs = HashMap::new();
        for (key, value) in self.iter() {
            if key.starts_with("env_") {
                if let Some(r) = value.downcast_ref::<Option<String>>() {
                    if let Some(e) = r {
                        envs.insert(key.replace("env_", "vcpkg_").to_uppercase(), e.clone());
                    }
                }
            }
        }
        return envs;
    }
}

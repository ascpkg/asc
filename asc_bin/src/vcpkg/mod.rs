pub mod cmake;
pub mod config;
pub mod index;
pub mod json;
pub mod search;
pub mod update;

use std::collections::{BTreeMap, HashMap};

use struct_iterable::Iterable;

use crate::{
    cli::commands::vcpkg::VcpkgArgs,
    config::{relative_paths, system_paths},
};

pub struct VcpkgManager {
    pub args: VcpkgArgs,
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
                default_conf.dump(true, false);

                default_conf
            },
        );
    }

    pub fn set_defaults(&mut self) {
        if self.registry.is_empty() {
            self.registry.push(format!(
                "{}?branch={}&directory={}",
                relative_paths::VCPKG_MICROSOFT_REPO_URL,
                relative_paths::VCPKG_MICROSOFT_REPO_BRANCH_NAME,
                system_paths::DataPath::vcpkg_default_clone_dir()
            ));
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
        let mut registries = BTreeMap::new();
        for (registry, url, branch, vcpkg_root_dir) in self.flatten_registry() {
            registries.insert(registry, (url, branch, vcpkg_root_dir));
        }
        for (registry, url, branch, vcpkg_root_dir) in other.flatten_registry() {
            if force {
                registries.insert(registry, (url, branch, vcpkg_root_dir));
            } else {
                if !registries.contains_key(&registry) {
                    registries.insert(registry, (url, branch, vcpkg_root_dir));
                }
            }
        }
        self.registry.clear();
        for (_registry, (url, branch, vcpkg_root_dir)) in registries {
            self.registry.push(format!(
                "{}?branch={}&directory={}",
                url, branch, vcpkg_root_dir
            ));
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
            return self.dump(true, false);
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

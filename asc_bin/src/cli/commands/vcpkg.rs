use clap::Args;

use config_file_derives::ConfigFile;
use config_file_types;

use struct_iterable::Iterable;

use serde::{Deserialize, Serialize};

use url::Url;

use crate::{config, vcpkg::VcpkgManager};

use super::VcpkgAction;

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, Iterable, ConfigFile)]
#[config_file_ext("toml")]
/// update vcpkg source, build vcpkg versions index, set/get vcpkg configurations
pub struct VcpkgArgs {
    /// update/index/set/get
    #[serde(skip)]
    action: VcpkgAction,

    /// sync registries
    #[clap(long, default_value_t = false)]
    #[serde(skip, default)]
    pub sync: bool,

    /// update args
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    /// vcpkg registry url?branch=&directory=
    #[clap(long)]
    pub registry: Vec<String>,

    /// vcpkg.index path
    #[clap(long)]
    pub index_directory: Option<String>,

    /// vcpkg.downloads path
    #[clap(long)]
    pub env_downloads: Option<String>,

    /// vcpkg.archives path
    #[clap(long)]
    pub env_default_binary_cache: Option<String>,

    #[clap(long, default_value = "")]
    #[serde(skip)]
    pub path: String,
}

impl VcpkgArgs {
    pub fn exec(&mut self) -> bool {
        tracing::info!(message = "vcpkg", registry = self.registry.join(", "));

        let mut manager = VcpkgManager::new(self.clone());
        match self.action {
            VcpkgAction::Update => manager.update(),
            VcpkgAction::Set => manager.config_set(),
            VcpkgAction::Get => {
                manager.config_get(false);
                true
            }
            VcpkgAction::Index => manager.index(),
            VcpkgAction::Flatten => manager.flatten(),
        }
    }

    pub fn flatten_registry(&self) -> Vec<(String, String, String, String)> {
        let mut results = vec![];

        for reg in &self.registry {
            match Url::parse(reg) {
                Err(e) => {
                    tracing::error!(
                        message = "Url::parse error",
                        repo = reg,
                        error = e.to_string()
                    );
                }
                Ok(u) => {
                    let url = u.as_str().split('?').next().unwrap();

                    let name = u.path().rsplit_once("/").unwrap().1.replace(".git", "");
                    let mut branch = String::new();
                    let mut directory = String::new();

                    for (key, value) in u.query_pairs() {
                        match key.as_ref() {
                            "branch" => branch = value.to_string(),
                            "directory" => directory = value.to_string(),
                            _ => {}
                        }
                    }

                    if directory.is_empty() {
                        directory = config::system_paths::DataPath::vcpkg_registry_clone_dir(&name);
                    }

                    results.push((name, url.to_string(), branch, directory));
                }
            }
        }
        return results;
    }

    pub fn get_registry(&self, name: &str) -> (String, String, String, String) {
        for (n, url, branch, directory) in self.flatten_registry() {
            if &n == name {
                return (n, url, branch, directory);
            }
        }

        tracing::error!(message = "vcpkg regitry was not found", name = name);
        return (String::new(), String::new(), String::new(), String::new());
    }

    pub fn get_private_registry(&self, name: &str) -> (String, String, String, String) {
        if name == config::relative_paths::VCPKG_DIR_NAME {
            tracing::error!("public vcpkg regitry was not allowed");
            return (String::new(), String::new(), String::new(), String::new());
        }

        for (n, url, branch, directory) in self.flatten_registry() {
            if &n == name {
                return (n, url, branch, directory);
            }
        }

        tracing::error!(message = "private vcpkg regitry was not found", name = name);
        return (String::new(), String::new(), String::new(), String::new());
    }

    pub fn get_public_registry(&self) -> (String, String, String, String) {
        for (n, url, branch, directory) in self.flatten_registry() {
            if &n == config::relative_paths::VCPKG_DIR_NAME {
                return (n, url, branch, directory);
            }
        }

        tracing::error!(message = "public vcpkg regitry was not found");
        return (String::new(), String::new(), String::new(), String::new());
    }
}

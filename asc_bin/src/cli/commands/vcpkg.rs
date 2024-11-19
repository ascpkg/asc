use clap::Args;

use config_file_derives::ConfigFile;
use config_file_types;

use struct_iterable::Iterable;

use serde::{Deserialize, Serialize};

use crate::vcpkg::VcpkgManager;

use super::VcpkgAction;

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, Iterable, ConfigFile)]
#[config_file_ext("toml")]
/// update vcpkg source, build vcpkg versions index, set/get vcpkg configurations
pub struct VcpkgArgs {
    /// update/index/set/get
    #[serde(skip)]
    action: VcpkgAction,

    /// update args
    pub args: Vec<String>,

    /// vcpkg repo url
    #[clap(long)]
    pub repo: Option<String>,

    /// vcpkg repo branch
    #[clap(long)]
    pub branch: Option<String>,

    /// vcpkg path
    #[clap(long)]
    pub directory: Option<String>,

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
        tracing::info!(
            message = "vcpkg",
            repo = self.repo,
            branch = self.branch,
            directory = self.directory
        );

        let mut manager = VcpkgManager::new(self.clone());
        match self.action {
            VcpkgAction::Update => manager.update(),
            VcpkgAction::Set => manager.config_set(),
            VcpkgAction::Get => {
                manager.config_get(false);
                true
            }
            VcpkgAction::Index => manager.index(),
        }
    }
}

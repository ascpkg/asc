use clap::Args;

use config_file_derives::ConfigFile;
use config_file_types;

use struct_iterable::Iterable;

use serde::{Deserialize, Serialize};

use crate::vcpkg::VcpkgManager;

use super::VcpkgAction;

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, Iterable, ConfigFile)]
#[config_file_ext("toml")]
pub struct VcpkgArgs {
    #[serde(skip)]
    action: VcpkgAction,

    pub args: Vec<String>,

    #[clap(long)]
    pub repo: Option<String>,

    #[clap(long)]
    pub branch: Option<String>,

    #[clap(long)]
    pub directory: Option<String>,

    #[clap(long)]
    pub index_directory: Option<String>,

    #[clap(long)]
    pub env_downloads: Option<String>,

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

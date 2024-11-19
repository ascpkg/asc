use clap::Args;

use crate::dependency;

#[derive(Args, Debug, Clone)]
/// remove dependency from package or workspace memeber's asc.toml
pub struct RemoveArgs {
    /// dependency name
    pub dependency: String,

    /// workspace member name
    #[clap(long)]
    pub package: Option<String>,
}

impl RemoveArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "remove", dependency = self.dependency);

        return dependency::remove::dependency_from_config_file(self);
    }
}

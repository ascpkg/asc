use clap::Args;

use crate::dependency;

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    pub dependency: String,

    #[clap(long)]
    pub package: Option<String>,

    #[clap(long)]
    pub recurse: bool,
}

impl RemoveArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "remove", dependency = self.dependency);

        return dependency::remove::dependency_from_config_file(self);
    }
}

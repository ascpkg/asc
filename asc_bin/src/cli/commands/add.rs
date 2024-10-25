use clap::Args;

use crate::dependency;

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    pub dependency: String,

    #[clap(long)]
    pub package: Option<String>,

    #[clap(long, default_value = "")]
    pub version: String,

    #[clap(long, help = "--find-package=a --find-package=b")]
    pub find_package: Vec<String>,

    #[clap(long, help = "--find-library=c --find-library=d")]
    pub link_library: Vec<String>,

    #[clap(long, help = "--feature=a --feature=b")]
    pub feature: Vec<String>,
}

impl AddArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "add", dependency = self.dependency);

        return dependency::add::dependency_to_config_file(self);
    }
}

use clap::Args;

use crate::dependency;

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    pub dependency: String,

    #[clap(long)]
    pub package: Option<String>,

    #[clap(long, default_value = "")]
    pub version: String,

    #[clap(long, help = "--find-package=a --find-package=b@!windows")]
    pub find_package: Vec<String>,

    #[clap(long, help = "--include-directory=c -include-directory=d")]
    pub include_directory: Vec<String>,

    #[clap(long, help = "--link-library=e --link-library=f")]
    pub link_library: Vec<String>,

    #[clap(long, help = "--feature=g --feature=h")]
    pub feature: Vec<String>,
}

impl AddArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "add", dependency = self.dependency);

        return dependency::add::dependency_to_config_file(self);
    }
}

use clap::Args;

use crate::dependency;

#[derive(Args, Debug, Clone)]
/// add dependency to package or workspace memeber's asc.toml
pub struct AddArgs {
    /// dependency name
    pub dependency: String,

    /// workspace member name
    #[clap(long)]
    pub package: Option<String>,

    /// dependency version (default latest)
    #[clap(long, default_value = "")]
    pub version: String,

    /// for cmake find_package (--find-package=a --find-package=b@!windows)
    #[clap(long)]
    pub find_package: Vec<String>,

    /// for cmake target_include_directories (--include-directory=c -include-directory=d)
    #[clap(long)]
    pub include_directory: Vec<String>,

    /// for cmake target_link_libraries (--link-library=e --link-library=f)
    #[clap(long)]
    pub link_library: Vec<String>,

    /// for vcpkg manifest (--feature=g --feature=h)
    #[clap(long)]
    pub feature: Vec<String>,
}

impl AddArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "add", dependency = self.dependency);

        return dependency::add::dependency_to_config_file(self);
    }
}

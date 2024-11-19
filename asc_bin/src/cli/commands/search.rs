use clap::Args;

use crate::vcpkg;

#[derive(Args, Debug, Clone)]
/// search package with extractly name or startswith/endswith/contains text
pub struct SearchArgs {
    /// extractly match (spdlog), startswith (log*), endswith (*log), contains (*log*)
    pub name: String,

    /// list all versions
    #[clap(long, default_value_t = false)]
    pub list: bool,
}

impl SearchArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "search", name = self.name);

        let results = vcpkg::search::from_index_file(&self.name, self.list);
        for res in &results {
            tracing::info!("{}", res);
        }

        return !results.is_empty();
    }
}

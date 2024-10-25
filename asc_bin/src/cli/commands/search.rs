use clap::Args;

use crate::vcpkg;

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    pub name: String,
}

impl SearchArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "search", name = self.name);

        return vcpkg::search::from_index_file(self);
    }
}

pub mod config;
pub mod index;
pub mod path;
pub mod search;
pub mod update;

use crate::cli::commands::vcpkg::VcpkgArgs;

pub struct VcpkgManager {
    args: VcpkgArgs,
}

impl VcpkgManager {
    pub fn new(args: VcpkgArgs) -> Self {
        Self { args: args }
    }
}

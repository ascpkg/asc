use clap::Args;

use crate::vcpkg;

#[derive(Args, Debug, Default, Clone)]
pub struct PublishArgs {
    edit: bool,
}

impl PublishArgs {
    pub fn exec(&self) -> bool {
        let mut result = vcpkg::json::gen_port_data();
        if !self.edit {
            result &= vcpkg::json::gen_port_versions();
        }

        result
    }
}

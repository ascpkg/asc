use clap::Args;

use crate::vcpkg;

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    // abc - equals with abc
    // ad* - startswith ad
    // *bc - endswith bc
    // *test* - contains test
    pub name: String,

    // 1.0 - equals to 1.0
    // <1.0 - less than 1.0
    // <=1.0 - less or equal than 1.0
    // >1.0 - greater than 1.0
    // >=1.0 - greater or equal than 1.0
    // 1.0<2.0 - greater than 1.0 and less than 2.0
    // 1.0=<2.0 - greater or equal than 1.0 and less than 2.0
    // 1.0<=2.0 - greater than 1.0 and less or equal than 2.0
    // 1.0=<=2.0 - greater or equal than 1.0 and less or equal than 2.0
    pub version: Option<String>,

    // same as version
    pub date: Option<String>,

    #[clap(long, default_value_t = false)]
    pub list: bool,
}

impl SearchArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "search", name = self.name);

        let results = vcpkg::search::from_index_file(self);
        for res in &results {
            tracing::info!("{}", res);
        }

        return !results.is_empty();
    }
}

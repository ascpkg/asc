use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    name: String,
}

impl SearchArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "search", name = self.name);

        false
    }
}

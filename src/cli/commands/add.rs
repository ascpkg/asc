use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    name: String,

    #[clap(long)]
    version: String,

    #[clap(long)]
    features: Vec<String>,
}

impl AddArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "add", name = self.name);

        false
    }
}

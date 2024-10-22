use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    #[clap(long)]
    name: String,
}

impl SearchArgs {
    pub fn exec(&self) -> bool {
        tracing::info!("search");

        false
    }
}

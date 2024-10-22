use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    #[clap(long)]
    name: String,
}

impl RemoveArgs {
    pub fn exec(&self) -> bool {
        tracing::info!("remove");
        
        false
    }
}

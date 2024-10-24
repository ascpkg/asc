use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    name: String,
    
    #[clap(long)]
    recurse: bool,
}

impl RemoveArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "remove", name = self.name);
        
        false
    }
}

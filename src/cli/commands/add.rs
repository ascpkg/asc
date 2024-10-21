use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    #[clap(long)]
    name: String,
    #[clap(long)]
    features: Vec<String>,
}

impl AddArgs {
    pub fn exec(&self) -> bool {
        false
    }
}

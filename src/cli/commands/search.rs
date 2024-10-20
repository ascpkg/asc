use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    #[clap(long)]
    name: String,
}
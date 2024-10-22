use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct CleanArgs {}

impl CleanArgs {
    pub fn exec(&self) -> bool {
        tracing::info!("clean");

        false
    }
}

pub mod add;
pub use add::AddArgs;
pub mod build;
pub use build::BuildArgs;
pub mod clean;
pub use clean::CleanArgs;
pub mod init;
pub use init::InitArgs;
pub mod new;
pub use new::NewArgs;
pub mod remove;
pub use remove::RemoveArgs;
pub mod run;
pub use run::RunArgs;
pub mod search;
pub use search::SearchArgs;
pub mod scan;
pub use scan::ScanArgs;

use clap::{Parser, Subcommand, ValueEnum};

use strum_macros::{AsRefStr, FromRepr};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New(NewArgs),
    Init(InitArgs),

    Search(SearchArgs),
    Add(AddArgs),
    Remove(RemoveArgs),

    Scan(ScanArgs),
    Build(BuildArgs),
    
    Run(RunArgs),
    Clean(CleanArgs),
}


#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr, FromRepr)]
#[clap(rename_all = "snake_case")]
pub enum ConfigType {
    Debug = 0,
    Release = 1,
}

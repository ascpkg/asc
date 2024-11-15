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
pub mod install;
pub use install::InstallArgs;
pub mod publish;
pub use publish::PublishArgs;
pub mod uninstall;
pub use uninstall::UninstallArgs;
pub mod vcpkg;
pub use vcpkg::VcpkgArgs;

use clap::{Parser, Subcommand, ValueEnum};

use strum_macros::{AsRefStr, FromRepr};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New(NewArgs),
    Init(InitArgs),

    Vcpkg(VcpkgArgs),
    Search(SearchArgs),
    Add(AddArgs),
    Remove(RemoveArgs),

    Scan(ScanArgs),
    Build(BuildArgs),

    Run(RunArgs),
    Clean(CleanArgs),

    Install(InstallArgs),
    Uninstall(UninstallArgs),

    Publish(PublishArgs),
}

#[derive(Clone, Debug, Default, PartialEq, ValueEnum, AsRefStr, FromRepr)]
#[clap(rename_all = "PascalCase")]
pub enum ConfigType {
    #[default]
    Debug,
    Release,
}

#[derive(Clone, Debug, Default, PartialEq, ValueEnum, AsRefStr, FromRepr)]
#[clap(rename_all = "snake_case")]
pub enum VcpkgAction {
    #[default]
    Update,
    Set,
    Get,
    Index,
}

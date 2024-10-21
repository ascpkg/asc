use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    Build(BuildArgs),
    Clean(CleanArgs),
    New(NewArgs),
    Remove(RemoveArgs),
    Run(RunArgs),
    Search(SearchArgs),
    Scan(ScanArgs),
}

#[derive(Args, Debug, Clone)]
struct AddArgs {
    #[clap(long)]
    pub include: Vec<String>,
}

#[derive(Args, Debug, Clone)]
struct BuildArgs {}

#[derive(Args, Debug, Clone)]
struct CleanArgs {}

#[derive(Args, Debug, Clone)]
struct NewArgs {}

#[derive(Args, Debug, Clone)]
struct RemoveArgs {}

#[derive(Args, Debug, Clone)]
struct RunArgs {}

#[derive(Args, Debug, Clone)]
struct SearchArgs {}

#[derive(Args, Debug, Clone)]
struct ScanArgs {}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add(options) => {
            println!("Add: {:?}", options);
        }
        Commands::Build(options) => {
            println!("Build: {:?}", options);
        }
        Commands::Clean(options) => {
            println!("Clean: {:?}", options);
        }
        Commands::New(options) => {
            println!("New: {:?}", options);
        }
        Commands::Remove(options) => {
            println!("Remove: {:?}", options);
        }
        Commands::Run(options) => {
            println!("Run: {:?}", options);
        }
        Commands::Search(options) => {
            println!("Search: {:?}", options);
        }
        Commands::Scan(options) => {
            println!("Scan: {:?}", options);
        }
    }
}

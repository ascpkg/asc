use clap::Parser;

use time::{macros::format_description, UtcOffset};

use tracing;
use tracing_subscriber::{self, fmt::time::OffsetTime};

pub mod clang;
pub mod cli;
pub mod cmake;
pub mod config;
pub mod errors;
pub mod graph;
pub mod config_file_types;
pub mod util;

fn main() {
    // init stdout tracing log
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_line_number(true)
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
        ))
        .init();

    // dispatch commands
    let mut cli = cli::Cli::parse();
    match &mut cli.command {
        // new bin/lib/workspace
        cli::Commands::New(options) => {
            options.exec();
        }
        // init bin/lib/workspace
        cli::Commands::Init(options) => {
            options.exec();
        }

        // operate vcpkg
        cli::Commands::Vcpkg(options) => {
            options.exec();
        }
        // search lib
        cli::Commands::Search(options) => {
            options.exec();
        }
        // add lib
        cli::Commands::Add(options) => {
            options.exec();
        }
        // remove lib
        cli::Commands::Remove(options) => {
            options.exec();
        }

        // scan source tree
        cli::Commands::Scan(options) => {
            options.exec();
        }
        // build source tree
        cli::Commands::Build(options) => {
            options.exec();
        }
        // install target
        cli::Commands::Install(options) => {
            options.exec();
        }
        // remove target
        cli::Commands::Uninstall(options) => {
            options.exec();
        }

        // clean target
        cli::Commands::Clean(options) => {
            options.exec();
        }
        // run target
        cli::Commands::Run(options) => {
            options.exec();
        }
    }
}

use clap::Parser;

use time::{macros::format_description, UtcOffset};

use tracing;
use tracing_subscriber::{self, fmt::time::OffsetTime};

pub mod clang;
pub mod cli;
pub mod cmake;
pub mod errors;
pub mod graph;
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

    let cli = cli::Cli::parse();
    match &cli.command {
        // new/init bin/lib/workspace
        cli::Commands::New(options) => {
            options.exec();
        }
        cli::Commands::Init(options) => {
            options.exec();
        }

        // search/add/remove lib
        cli::Commands::Search(options) => {
            options.exec();
        }
        cli::Commands::Add(options) => {
            options.exec();
        }
        cli::Commands::Remove(options) => {
            options.exec();
        }

        // scan/build source tree
        cli::Commands::Scan(options) => {
            options.exec();
        }
        cli::Commands::Build(options) => {
            options.exec();
        }

        // clean/run target
        cli::Commands::Clean(options) => {
            options.exec();
        }
        cli::Commands::Run(options) => {
            options.exec();
        }
    }
}

// use clap::Parser;

// use time::{macros::format_description, UtcOffset};

// use tracing;
// use tracing_subscriber::{self, fmt::time::OffsetTime};

// fn main() {
//     // init stdout tracing log
//     tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::INFO)
//         .with_line_number(true)
//         .with_timer(OffsetTime::new(
//             UtcOffset::from_hms(8, 0, 0).unwrap(),
//             format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
//         ))
//         .init();

//     // parse command lines
//     tracing::warn!("parse command lines");
//     let mut options = cli::Options::parse();
//     options.normalize();
//     tracing::info!("{:#?}", options);

//     // scan source dependencies with clang ir
//     if options.action_type == cli::types::ActionType::Scan
//         || options.action_type == cli::types::ActionType::All
//     {
//         // write empty files
//         std::fs::create_dir(&options.build_dir).unwrap_or(());
//         std::fs::write(format!("{}/config.h", &options.build_dir), b"").unwrap_or(());
//         std::fs::write(format!("{}/version.h", &options.build_dir), b"").unwrap_or(());

//         tracing::warn!("scan source dependencies with clang ir");
//         let source_mappings = clang::parser::SourceMappings::scan(&options);

//         tracing::warn!("output flow chart {}", graph::flowchart::path(&options));
//         let mermaid_flowchart = graph::flowchart::gen(&options, &source_mappings);
//         tracing::info!("\n{mermaid_flowchart}");

//         tracing::warn!("output {}", cmake::path::cmake_lists_path(&options));
//         cmake::lists::gen(&options, &source_mappings);
//     }

//     // output CMakeLists.txt
//     if options.action_type == cli::types::ActionType::Configure
//         || options.action_type == cli::types::ActionType::All
//     {
//         tracing::warn!("generate a build system with cmake");
//         cmake::project::gen(&options);
//     }

//     // build with cmake
//     if options.action_type == cli::types::ActionType::Build
//         || options.action_type == cli::types::ActionType::All
//     {
//         tracing::warn!("build with cmake");
//         cmake::build::run(&options);
//     }

//     // install with cmake
//     if options.action_type == cli::types::ActionType::Install
//         || options.action_type == cli::types::ActionType::All
//     {
//         tracing::warn!("install with cmake");
//         cmake::install::run(&options);
//     }
// }

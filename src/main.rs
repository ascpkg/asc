use clap::Parser;

use time::{macros::format_description, UtcOffset};

use tracing;
use tracing_subscriber::{self, fmt::time::OffsetTime};

pub mod clang;
pub mod cmake;
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

    // parse command lines
    tracing::warn!("parse command lines");
    let mut options = util::cli::CommandLines::parse();
    options.normalize();
    tracing::info!("{:#?}", options);

    // scan source dependencies with clang ir
    if options.action_type == util::cli::ActionType::Scan
        || options.action_type == util::cli::ActionType::All
    {
        tracing::warn!("scan source dependencies with clang ir");
        let source_mappings = clang::parser::SourceMappings::scan(&options);

        tracing::warn!("output flow chart {}", graph::flowchart::path(&options));
        let mermaid_flowchart = graph::flowchart::gen(&options, &source_mappings);
        tracing::info!("\n{mermaid_flowchart}");

        tracing::warn!("output {}", cmake::lists::path(&options));
        cmake::lists::gen(&options, &source_mappings);
    }

    // output CMakeLists.txt
    if options.action_type == util::cli::ActionType::Configure
        || options.action_type == util::cli::ActionType::All
    {
        tracing::warn!("generate a build system with cmake");
        cmake::project::gen(&options);
    }

    // build with cmake
    if options.action_type == util::cli::ActionType::Build
        || options.action_type == util::cli::ActionType::All
    {
        tracing::warn!("build with cmake");
        cmake::build::run(&options);
    }

    // install with cmake
    if options.action_type == util::cli::ActionType::Install
        || options.action_type == util::cli::ActionType::All
    {
        tracing::warn!("install with cmake");
        cmake::install::run(&options);
    }
}

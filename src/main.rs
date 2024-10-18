use clap::Parser;

use time::{macros::format_description, UtcOffset};

use tracing;
use tracing_subscriber::{self, fmt::time::OffsetTime};

pub mod clang;
pub mod cmake;
pub mod graph;
pub mod util;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_line_number(true)
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
        ))
        .init();

    tracing::warn!("parse command lines");
    let mut options = util::cli::CommandLines::parse();
    options.normalize();
    tracing::info!("{:#?}", options);

    tracing::warn!("scan source dependences with clang ir");
    let source_mappings = clang::parser::SourceMappings::scan(&options);

    tracing::warn!("output mermaid flowchat of source dependences");
    let mermaid_flowchart =
        graph::flowchart::gen(&options.source_dir, &source_mappings);
    tracing::info!("{mermaid_flowchart}");
    std::fs::write(
        format!("{}.md", options.project),
        format!("```mermaid\n{}\n```", mermaid_flowchart).as_bytes(),
    )
    .unwrap();

    let project_dir = std::path::Path::new(&options.source_dir)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    tracing::warn!("output CMakeLists.txt");
    let txt = cmake::lists::gen(&options, &source_mappings, &project_dir);
    std::fs::write(format!("{}/CMakeLists.txt", &project_dir,), txt.as_bytes()).unwrap();

    tracing::warn!("generate a build system with cmake");
    let build_dir = format!("{}/build", &project_dir);
    cmake::project::gen(&build_dir, &project_dir);

    tracing::warn!("build with cmake");
    cmake::build::compile(&build_dir);
}

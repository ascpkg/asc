use clap::Parser;

use tracing;
use tracing_subscriber;

pub mod clang;
pub mod cmake;
pub mod graph;
pub mod util;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .with_line_number(true)
        .init();

    tracing::error!("parse command line options");
    let mut options = util::cli::CommandLines::parse();
    options.replace();
    tracing::error!("{:#?}", options);

    tracing::error!("generate source dependences");
    let source_mappings = clang::SourceMappings::parse(&options);

    tracing::error!("generate markdown mermaid flowchat");
    let markdown_mermaid_flowchart =
        graph::gen::gen_dependency_flowchat(&options.source_dir, &source_mappings);
    tracing::error!("{markdown_mermaid_flowchart}");
    std::fs::write("README.md", markdown_mermaid_flowchart.as_bytes()).unwrap();

    tracing::error!("generate CMakeLists.txt");
    let project_dir = std::path::Path::new(&options.source_dir)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    let txt = cmake::lists::gen(&options, &source_mappings, &project_dir);
    std::fs::write(format!("{}/CMakeLists.txt", &project_dir,), txt.as_bytes()).unwrap();

    // run cmake generate
    tracing::error!("cmake generate project");
    let build_dir = format!("{}/build", &project_dir);
    cmake::project::gen(&build_dir, &project_dir);

    // run cmake build
    tracing::error!("cmake build");
    cmake::build::compile(&build_dir);
}

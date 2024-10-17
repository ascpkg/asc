use clap::Parser;

pub mod graph;
pub mod ir;
pub mod util;

fn main() {
    let options = util::cli::CommandLines::parse();
    let source_mappings = ir::parser::SourceMappings::parse(options);

    println!("\n{:#?}\n", &source_mappings.source_include_headers);
    println!("\n{:#?}\n", &source_mappings.header_include_headers);

    let markdown_mermaid_flowchart =
        graph::builder::build_dependency_flowchat(&source_mappings);
    println!("{markdown_mermaid_flowchart}");
    std::fs::write("deps.md", markdown_mermaid_flowchart.as_bytes()).unwrap();
}

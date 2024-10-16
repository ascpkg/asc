use clap::Parser;

pub mod graph;
pub mod ir;
pub mod util;

fn main() {
    let options = util::cli::CommandLines::parse();
    let (source_headers, source_functions, header_sources, header_functions) =
        ir::parser::parse(options);

    println!("\n{:#?}\n", source_headers);
    println!("\n{:#?}\n", header_sources);

    let markdown_mermaid_flowchart =
        graph::builder::build_dependency_flowchat(&header_sources, &source_headers);
    println!("{markdown_mermaid_flowchart}");
    std::fs::write("deps.md", markdown_mermaid_flowchart.as_bytes()).unwrap();
}

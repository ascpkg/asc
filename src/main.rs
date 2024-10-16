use clap::Parser;

pub mod graph;
pub mod ir;
pub mod util;



fn main() {
    let options = util::cli::CommandLines::parse();
    let (source_headers, source_functions, header_sources, header_functions) =
        ir::parser::parse(options);

    let flowchart = graph::builder::build_dependency_flowchat(&header_sources, &source_headers);
    println!("```mermaid");
    println!("{flowchart}");
    println!("```");
}

use clap::Parser;

use petgraph_graphml;

pub mod graph;
pub mod ir;
pub mod util;

fn main() {
    let options = util::cli::CommandLines::parse();
    let (source_headers, source_functions, header_sources, header_functions) =
        ir::parser::parse(options);

    let dependency_graph = graph::builder::build_dependency_graph(&header_sources, &source_headers);
    let graphml = petgraph_graphml::GraphMl::new(&dependency_graph)
        .pretty_print(true)
        .export_node_weights_display()
        .to_string();
    println!("{graphml}");
}

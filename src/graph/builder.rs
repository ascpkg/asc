use petgraph;

pub fn build_dependency_graph(
    header_sources: &std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    source_headers: &std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
) -> petgraph::graph::DiGraph<String, ()> {
    let mut graph = petgraph::graph::DiGraph::<String, ()>::new();

    for (source, dependencies) in header_sources.iter() {
        let source_index = graph.add_node(source.clone());
        for dependency in dependencies {
            let dep_index = graph.add_node(dependency.clone());
            graph.add_edge(source_index, dep_index, ());
        }
    }

    for (source, dependents) in source_headers.iter() {
        let source_index = graph.add_node(source.clone());
        for dependent in dependents {
            let dep_index = graph.add_node(dependent.clone());
            graph.add_edge(dep_index, source_index, ());
        }
    }

    remove_unused_nodes(&mut graph);

    graph
}

fn remove_unused_nodes(graph: &mut petgraph::graph::DiGraph<String, ()>) {
    let mut nodes_to_remove: Vec<petgraph::graph::NodeIndex> = vec![];

    for node_index in graph.node_indices() {
        if graph
            .edges_directed(node_index, petgraph::Direction::Outgoing)
            .count()
            == 0
            && graph
                .edges_directed(node_index, petgraph::Direction::Incoming)
                .count()
                == 0
        {
            nodes_to_remove.push(node_index);
        }
    }

    for node_index in nodes_to_remove {
        graph.remove_node(node_index);
    }
}

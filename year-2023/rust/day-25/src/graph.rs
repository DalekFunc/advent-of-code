use std::{collections::HashSet, fs::File, io::Write};

use petgraph::{
    dot::{Config, Dot},
    stable_graph::NodeIndex,
    visit::IntoNodeReferences,
    Graph, Undirected,
};

pub type Components = Graph<&'static str, (), Undirected>;

pub fn get_node_index(graph: &mut Components, name: &'static str) -> NodeIndex {
    match graph
        .node_references()
        .find(|(_idx, named)| **named == name)
    {
        Some((idx, _)) => idx,
        None => graph.add_node(name),
    }
}

pub fn print_graph(graph: &Components) {
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    println!("{:?}", dot);
}

pub fn output_dot_file(graph: &Components, file_name: &str) {
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    let mut file = File::create(file_name).expect("file created");
    // file.write_all(dot)?;

    file.write_fmt(format_args!("{:?}", dot)).expect("write ok");
}

pub fn are_interconnected(graph: &Components, nodes: &Vec<&NodeIndex>) -> bool {
    let result = true;
    for node in nodes {
        let others = nodes.into_iter().filter(|other| *other != node).map(|r| *r);
        if !is_connected_to_all(graph, **node, others) {
            return false;
        }
    }
    result
}

// node is connected to at least (2) others
pub fn is_connected_to_all<'a>(
    graph: &Components,
    node: NodeIndex,
    others: impl Iterator<Item = &'a NodeIndex>,
) -> bool {
    others
        .into_iter()
        .filter(|&&other| graph.edges_connecting(node, other).count() > 0)
        .count()
        >= 2
}

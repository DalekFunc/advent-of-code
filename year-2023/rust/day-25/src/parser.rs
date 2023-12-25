use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use petgraph::Graph;

use crate::graph::{get_node_index, Components};

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)).parse(input)
}

pub fn parse_file(input: &'static str) -> IResult<&'static str, Components> {
    let (rest, connections) = separated_list1(line_ending, parse_line)(input)?;

    let mut graph = Graph::new_undirected();
    for (name, nbrs) in connections {
        let node = get_node_index(&mut graph, name);

        for name in nbrs {
            let nbr = get_node_index(&mut graph, name);

            graph.update_edge(node, nbr, ());
        }
    }

    Ok((rest, graph))
}

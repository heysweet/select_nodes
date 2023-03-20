#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

mod node;
mod types;
mod graph;
mod search_methods;

use std::collections::{HashMap};

use node::Node;
use graph::{Graph, UniqueId};
use search_methods::SearchMethod;

fn generate_node_hash_map(nodes: Vec<&Node>) -> HashMap<UniqueId, &Node> {
    let mut result: HashMap<UniqueId, &Node>;

    todo!();

    result
}

pub fn select_nodes(graph: Graph, nodes: Vec<&Node>, selector: String) -> Vec<&Node> {
    let node_map = generate_node_hash_map(nodes);

    SearchMethod::search(graph, node_map, selector);

    nodes
}

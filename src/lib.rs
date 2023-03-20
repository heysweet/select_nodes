#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

mod graph;
mod selector;

use std::collections::{HashMap};

use graph::{ParsedGraph, UniqueId, node::ParsedNode};

pub fn generate_node_hash_map(nodes: Vec<&ParsedNode>) -> HashMap<UniqueId, &ParsedNode> {
    let mut result: HashMap<UniqueId, &ParsedNode>;

    todo!();

    result
}

pub fn select_nodes(graph: ParsedGraph, nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> Vec<&ParsedNode> {

    // SearchMethod::search(graph, node_map, selector);

    nodes
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

mod node;
use node::Node;

pub fn select_nodes(nodes: Vec<Node>, selector: String) -> Vec<Node> {
    let nodes = nodes.clone();
    nodes
}
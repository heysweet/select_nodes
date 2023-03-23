#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

mod graph;
mod selector;

use std::collections::{HashMap};

use graph::{ParsedGraph, UniqueId, node::ParsedNode};

use crate::selector::{spec::SelectionCriteria, SearchMethod};

pub fn generate_node_hash_map<Iter>(nodes: Iter) -> HashMap<UniqueId, ParsedNode> where Iter: Iterator<Item = ParsedNode> {
    let mut result: HashMap<UniqueId, ParsedNode> = HashMap::new();

    nodes.for_each(|node| { result.insert(node.unique_id.clone(), node); });

    result
}

pub fn select_nodes(graph: ParsedGraph, raw_selector: impl Into<String>) -> Result<std::slice::Iter<UniqueId>, String> {
    let raw_select: &str = raw_selector.into().as_str();

    let selection_criteria = SelectionCriteria::from_single_raw_spec(String::from(raw_select))?;

    Ok(selection_criteria.method.search(&graph, raw_select))
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

mod graph;
mod selector;

use std::collections::HashMap;

use graph::{node::ParsedNode, UniqueId};
use selector::methods::SearchError;

pub use crate::graph::ParsedGraph;

pub use crate::selector::spec::SelectionCriteria;

pub fn generate_node_hash_map(nodes: Vec<ParsedNode>) -> HashMap<UniqueId, ParsedNode> {
    nodes
        .iter()
        .map(|node| (node.unique_id.clone(), node.clone()))
        .collect()
}

pub fn select_nodes(
    graph: ParsedGraph,
    raw_selector: impl Into<String>,
) -> Result<Vec<UniqueId>, SearchError> {
    let binding = raw_selector.into();
    let raw_select: &str = binding.as_str();

    let selection_criteria = SelectionCriteria::from_single_raw_spec(String::from(raw_select));

    match selection_criteria {
        Err(selection_error) => Err(SearchError::SelectionError { selection_error }),
        Ok(selection_criteria) => Ok(selection_criteria.method.search(&graph, raw_select)?),
    }
}

// #[cfg(test)]
// #[path = "lib_tests.rs"]
// mod lib_tests;

// #[macro_use]
// extern crate lazy_static;

wai_bindgen_rust::export!("src/interface.wai");

mod graph;
mod selector;

use crate::graph::UniqueId;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use graph::node::GraphNode;
use wai_bindgen_rust::Handle;

use interface::SelectionError::*;
use interface::{Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError};

// use graph::UniqueId;

use crate::graph::ParsedGraph;
// use crate::selector::spec::SelectionCriteria;
// use crate::graph::node::GraphNode;

// pub use graph::node_selector::NodeSelector;

//core/dbt/graph/selector.py
// only expose ResourceTypeSelector(nodes, edges, previous_state: Option<PreviousState>, resource_types)
//
// review all Selector types and see if we can just make it NodeSelector with configuration

pub struct Interface;

impl interface::Interface for Interface {}

impl Display for SelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoMatchingResourceType(selector) => {
                write!(f, "Invalid resource_type selector '{}'", selector)
            }
            // TODO: get rid of this?
            MissingField(field) => write!(f, "Missing required field '{}'", field),
            MissingValueError(input) => {
                write!(f, "'{}' is not a valid method name", input)
            }
            ParentsDepthParseIntError(input) => {
                write!(f, "Failed to parse parents depth in '{}'.", input)
            }
            ChildrensDepthParseIntError(input) => {
                write!(f, "Failed to parse childrens depth in '{}'.", input)
            }
            InvalidMethodError(method_name) => {
                write!(f, "'{}' is not a valid method name", method_name)
            }
            IncompatiblePrefixAndSuffixError(input) => {
                write!(
                    f,
                    "Invalid node spec '{}' - '@' prefix and '+' suffix are incompatible",
                    input
                )
            }
            FailedRegexMatchError(input) => {
                write!(f, "Failed to match regex for '{}'", input)
            }
            MatchedEmptyMethodError => {
                write!(f, "Matched empty method name")
            }
            InvalidIndirectSelectionError => {
                write!(f, "Invalid IndirectSelection input")
            }
            BoolInputError(key) => {
                write!(
                    f,
                    "'{}' field was provided and was not string literal `true` or `false`",
                    key
                )
            }
            NodeNotInGraph(id) => write!(f, "Node with id '{}' was not found in graph.", id),
        }
    }
}

pub struct NodeSelector {
    graph: ParsedGraph,
    previous_state: Option<ParsedGraph>,
}

impl interface::NodeSelector for NodeSelector {
    fn new(
        nodes: Vec<Node>,
        edges: Vec<Edge>,
    ) -> Result<Handle<NodeSelector>, SelectorCreateError> {
        let mut node_map = HashMap::<UniqueId, GraphNode>::new();
        for node in nodes.iter() {
            node_map.insert(node.unique_id, GraphNode::from(node)?);
        }

        let mut parent_map = HashMap::<UniqueId, HashSet<UniqueId>>::new();
        for edge in edges.iter() {
            let parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents);
            parent_map.insert(edge.unique_id, parents);
        }
        Ok(NodeSelector {
            graph: ParsedGraph::from_parents(node_map, parent_map),
            previous_state: None,
        }
        .into())
    }

    fn update(
        &self,
        nodes: Vec<interface::Node>,
        edges: Vec<interface::Edge>,
    ) -> Result<Handle<NodeSelector>, SelectorCreateError> {
        let mut node_map = HashMap::<UniqueId, GraphNode>::new();
        for node in nodes.iter() {
            node_map.insert(node.unique_id, GraphNode::from(node)?);
        }

        let mut parent_map = HashMap::<UniqueId, HashSet<UniqueId>>::new();
        for edge in edges.iter() {
            let parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents);
            parent_map.insert(edge.unique_id, parents);
        }
        Ok(NodeSelector {
            graph: ParsedGraph::from_parents(node_map, parent_map),
            previous_state: Some(self.graph),
        }
        .into())
    }

    fn select(&self, selector: String) -> Result<Vec<String>, SelectionError> {}

    fn select_type(
        &self,
        selector: UniqueId,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<String>, SelectionError> {
    }

    fn select_included(
        &self,
        included_nodes: Vec<String>,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<String>, SelectionError> {
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

wai_bindgen_rust::export!("src/interface.wai");

mod graph;
mod selector;

use crate::graph::UniqueId;
use std::collections::{HashMap, HashSet};

use graph::node::GraphNode;
use wai_bindgen_rust::Handle;

use interface::{Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError};

use crate::graph::ParsedGraph;

pub struct Interface;

impl interface::Interface for Interface {}

pub struct NodeSelector {
    graph: ParsedGraph,
    previous_state: Option<ParsedGraph>,
}

//core/dbt/graph/selector.py
impl interface::NodeSelector for NodeSelector {
    fn new(
        nodes: Vec<Node>,
        edges: Vec<Edge>,
    ) -> Result<Handle<NodeSelector>, SelectorCreateError> {
        let mut node_map = HashMap::<UniqueId, GraphNode>::new();
        for node in nodes.iter() {
            node_map.insert(node.unique_id.to_owned(), GraphNode::from(node)?);
        }

        let mut parent_map = HashMap::<UniqueId, HashSet<UniqueId>>::new();
        for edge in edges.iter() {
            let mut parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents.to_owned());
            parent_map.insert(edge.unique_id.to_owned(), parents);
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
            node_map.insert(node.unique_id.clone(), GraphNode::from(node)?);
        }

        let mut parent_map = HashMap::<UniqueId, HashSet<UniqueId>>::new();
        for edge in edges.iter() {
            let mut parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents.to_owned());
            parent_map.insert(edge.unique_id.clone(), parents);
        }
        Ok(NodeSelector {
            graph: ParsedGraph::from_parents(node_map, parent_map),
            previous_state: Some(self.graph.to_owned()),
        }
        .into())
    }

    fn select(&self, selector: String) -> Result<Vec<UniqueId>, SelectionError> {
        todo!()
    }

    fn select_type(
        &self,
        selector: UniqueId,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        todo!()
    }

    fn select_included(
        &self,
        included_nodes: Vec<UniqueId>,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        todo!()
    }
}

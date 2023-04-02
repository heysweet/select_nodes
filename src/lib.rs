#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

wai_bindgen_rust::export!("src/interface.wai");

mod graph;
mod selector;

use crate::graph::UniqueId;

use graph::node::GraphNode;
use selector::{
    node_selector::NodeSelector,
    spec::{IndirectSelection, SelectionCriteria},
};
use wai_bindgen_rust::Handle;

use interface::{Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError};

pub struct Interface;

impl interface::Interface for Interface {}

//core/dbt/graph/selector.py
impl interface::NodeSelector for NodeSelector {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Handle<Self>, SelectorCreateError> {
        NodeSelector::from(nodes, edges).and_then(|s| Ok(s.into()))
    }

    fn select(&self, selector: String) -> Result<Vec<UniqueId>, SelectionError> {
        self.select_and_filter(None, &selector, &ResourceTypeFilter::All)
    }

    fn select_type(
        &self,
        selector: UniqueId,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        self.select_and_filter(None, &selector, &resource_type_filter)
    }

    fn select_included(
        &self,
        included_nodes: Vec<UniqueId>,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        self.select_and_filter(
            Some(included_nodes.into_iter().collect()),
            &selector,
            &resource_type_filter,
        )
    }
}

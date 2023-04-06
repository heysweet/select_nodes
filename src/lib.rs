#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

wai_bindgen_rust::export!("dbt-node-selector.wai");

mod graph;
mod selector;

use std::collections::HashSet;

use crate::graph::UniqueId;

use graph::node::GraphNode;
use selector::{
    node_selector::NodeSelector,
    spec::{IndirectSelection, SelectionCriteria, SelectionGroup},
};
use wai_bindgen_rust::Handle;

use dbt_node_selector::{Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError};

pub struct DbtNodeSelector;

impl dbt_node_selector::DbtNodeSelector for DbtNodeSelector {}

//core/dbt/graph/selector.py
impl dbt_node_selector::NodeSelector for NodeSelector {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Handle<Self>, SelectorCreateError> {
        Self::_new(nodes, edges)
    }

    fn update(
        &self,
        nodes: Vec<Node>,
        edges: Vec<Edge>,
    ) -> Result<Handle<Self>, SelectorCreateError> {
        self._update(nodes, edges)
    }

    fn select(&self, selector: String) -> Result<Vec<UniqueId>, SelectionError> {
        self._select(selector)
    }

    fn select_type(
        &self,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        self._select_type(selector, resource_type_filter)
    }

    fn select_included(
        &self,
        included_nodes: Vec<UniqueId>,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        self._select_included(included_nodes, selector, resource_type_filter)
    }
}

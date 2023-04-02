#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

wai_bindgen_rust::export!("src/interface.wai");

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

use interface::{Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError};

pub struct Interface;

impl interface::Interface for Interface {}

//core/dbt/graph/selector.py
impl interface::NodeSelector for NodeSelector {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Handle<Self>, SelectorCreateError> {
        NodeSelector::from(nodes, edges).and_then(|s| Ok(s.into()))
    }

    fn select(&self, selector: String) -> Result<Vec<UniqueId>, SelectionError> {
        let selection_criteria = SelectionCriteria::from_single_raw_spec(selector)?;
        let selection_group = SelectionGroup::from_criteria(selection_criteria);

        let selected_set: HashSet<String> = self.get_selected(&selection_group)?;

        Ok(selected_set.into_iter().collect())
    }

    fn select_type(
        &self,
        selector: UniqueId,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        let selection_criteria = SelectionCriteria::from_single_raw_spec(selector)?;
        let selection_group = SelectionGroup::from_criteria(selection_criteria);

        let selected_set: HashSet<String> =
            self.get_selected_type(&selection_group, &resource_type_filter)?;

        Ok(selected_set.into_iter().collect())
    }

    fn select_included(
        &self,
        included_nodes: Vec<UniqueId>,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        todo!()
        // let selected_set: HashSet<String> = self.select_type(selector, resource_type_filter)?;

        // Ok(selected_set.into_iter().collect())
    }
}

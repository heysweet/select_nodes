#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

wai_bindgen_rust::export!("src/interface.wai");

mod graph;
mod selector;

use crate::graph::UniqueId;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use graph::node::GraphNode;
use selector::spec::SelectionCriteria;
use wai_bindgen_rust::Handle;

use interface::{Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError};

use crate::graph::ParsedGraph;

pub struct Interface;

impl interface::Interface for Interface {}

pub struct NodeSelector {
    graph: Rc<ParsedGraph>,
    previous_state: Option<Rc<ParsedGraph>>,
}

impl NodeSelector {
    fn select_and_filter(
        &self,
        included_nodes: Option<HashSet<UniqueId>>,
        selector: &String,
        resource_type_filter: &ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        let selection_criteria = SelectionCriteria::from_single_raw_spec(selector)?;
        let unfiltered_result = selection_criteria.method.search(&self.previous_state.clone(), &self.graph, selector)?;
        Ok(unfiltered_result
            .iter()
            .filter(|id| match &included_nodes {
                Some(included_nodes) => self.graph.is_node(id, &|n| {
                    included_nodes.contains(*id)
                        && resource_type_filter.should_include(n.resource_type)
                }),
                None => self.graph.is_node(id, &|n| {
                    resource_type_filter.should_include(n.resource_type)
                }),
            })
            .map(|s| s.to_owned())
            .collect())
    }

    fn from(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Self, SelectorCreateError> {
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
        Ok(Self {
            graph: ParsedGraph::from_parents(node_map, parent_map).into(),
            previous_state: None.into(), //previous_state.and_then(|s| Some(s.graph.clone())),
        })
    }
}

//core/dbt/graph/selector.py
impl interface::NodeSelector for NodeSelector {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Handle<Self>, SelectorCreateError> {
        NodeSelector::from(nodes, edges).and_then(|s| Ok(s.into()))
    }

    fn select(&self, selector: String) -> Result<Vec<UniqueId>, SelectionError> {
        let selection_criteria = SelectionCriteria::from_single_raw_spec(&selector)?;
        Ok(selection_criteria.method.search(&self.previous_state, &self.graph, &selector)?)
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

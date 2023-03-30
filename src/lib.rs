// #[cfg(test)]
// #[path = "lib_tests.rs"]
// mod lib_tests;

// #[macro_use]
// extern crate lazy_static;

use crate::graph::UniqueId;
use std::collections::{HashMap, HashSet};

use graph::node::GraphNode;
use interface::{ResourceTypeFilter, NodeCreateError};
use wai_bindgen_rust::Handle;

wai_bindgen_rust::export!("src/interface.wai");

mod graph;
// mod selector;

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

pub struct NodeSelector {
    graph: ParsedGraph,
    previous_state: Option<ParsedGraph>
}

impl interface::NodeSelector for NodeSelector {
    fn new(nodes: Vec<interface::Node>, edges: Vec<interface::Edge>) -> Result<Handle<NodeSelector>, NodeCreateError> {
        let mut node_map = HashMap::<UniqueId, GraphNode>::new();
        for node in nodes.iter(){
            node_map.insert(node.unique_id, GraphNode::from(node)?);
        };

        let mut parent_map = HashMap::<UniqueId, HashSet::<UniqueId>>::new();
        for edge in edges.iter() {
            let parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents);
            parent_map.insert(edge.unique_id, parents);
        }
        Ok(NodeSelector{ graph: ParsedGraph::from_parents(node_map, parent_map), previous_state: None }.into())
    }

    fn update(&self, nodes:Vec<interface::Node>, edges:Vec<interface::Edge>) -> Result<Handle<NodeSelector>, NodeCreateError> {
        let mut node_map = HashMap::<UniqueId, GraphNode>::new();
        for node in nodes.iter(){
            node_map.insert(node.unique_id, GraphNode::from(node)?);
        };

        let mut parent_map = HashMap::<UniqueId, HashSet::<UniqueId>>::new();
        for edge in edges.iter() {
            let parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents);
            parent_map.insert(edge.unique_id, parents);
        }
        Ok(NodeSelector{ graph: ParsedGraph::from_parents(node_map, parent_map), previous_state: Some(self.graph) }.into())
    }

    fn select(&self,selector:String,) -> Vec<String> {
        
    }

    fn select_type(&self,selector:String,resource_type_filter:ResourceTypeFilter,) -> Vec<String> {
        
    }
    
    fn select_included(&self,included_nodes:Vec<String>,selector:String,) -> Vec<String> {
        
    }
}
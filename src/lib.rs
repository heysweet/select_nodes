// #[cfg(test)]
// #[path = "lib_tests.rs"]
// mod lib_tests;

// #[macro_use]
// extern crate lazy_static;

use interface::ResourceTypeFilter;

wai_bindgen_rust::export!("src/interface.wai");

// mod graph;
// mod selector;

// use graph::UniqueId;

// use crate::graph::ParsedGraph;
// use crate::selector::spec::SelectionCriteria;
// use crate::graph::node::GraphNode;

// pub use graph::node_selector::NodeSelector;

//core/dbt/graph/selector.py
// only expose ResourceTypeSelector(nodes, edges, previous_state: Option<PreviousState>, resource_types)
// 
// review all Selector types and see if we can just make it NodeSelector with configuration

pub struct Interface;

impl interface::Interface for Interface {}

pub struct NodeSelector;

impl interface::NodeSelector for NodeSelector {
    fn new(nodes:Vec<interface::Node>, edges:Vec<interface::Edge>,) -> wai_bindgen_rust::Handle<crate::NodeSelector> {
        
    }

    fn select(&self,selector:String,) -> Vec<String> {
        
    }

    fn select_type(&self,selector:String,resource_type_filter:ResourceTypeFilter,) -> Vec<String> {
        
    }
    
    fn select_included(&self,included_nodes:Vec<String>,selector:String,) -> Vec<String> {
        
    }
}
#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

#[macro_use]
extern crate lazy_static;

mod graph;
mod selector;

use graph::UniqueId;

use crate::graph::ParsedGraph;
use crate::selector::spec::SelectionCriteria;
use crate::graph::node::GraphNode;

pub use graph::node_selector::NodeSelector;

//core/dbt/graph/selector.py
// only expose ResourceTypeSelector(nodes, edges, previous_state: Option<PreviousState>, resource_types)
// 
// review all Selector types and see if we can just make it NodeSelector with configuration
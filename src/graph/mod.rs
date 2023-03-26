pub mod node;
pub mod selector;
pub mod types;

/// https://github.com/dbt-labs/dbt-core/blob/4186f99b742b47e0e95aca4f604cc09e5c67a449/core/dbt/graph/graph.py
use std::collections::{HashMap, HashSet};

use crate::selector::methods::SearchError;

use self::node::GraphNode;

pub use String as UniqueId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedGraph {
    pub node_map: HashMap<UniqueId, GraphNode>,
    pub children_map: HashMap<UniqueId, HashSet<UniqueId>>,
    /// A map of nodes to its set of parents
    pub parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
}

enum SelectionError {
    NoMatchingResourceType(String),
    NodeNotInGraph{ id: String },
    SearchError(SearchError)
}

use SelectionError::*;

impl ParsedGraph {
    fn reverse_edges(
        edge_map: &HashMap<UniqueId, HashSet<UniqueId>>,
    ) -> HashMap<UniqueId, HashSet<UniqueId>> {
        let mut target_map: HashMap<UniqueId, HashSet<UniqueId>> = HashMap::new();

        for (source_id, target_ids) in edge_map.clone().iter() {
            for target_id in target_ids {
                let value = target_map.get_mut(target_id);
                match value {
                    Some(targets) => {
                        targets.insert(source_id.clone());
                    }
                    None => {
                        let mut targets = HashSet::new();
                        targets.insert(source_id.clone());
                        target_map.insert(source_id.clone(), targets);
                    }
                }
            }
        }
        target_map
    }

    // Returns a subset of the Graph, does not modify original Graph.
    pub fn filter(&self, included: &HashSet<UniqueId>) -> Self {
        let node_map = self
        .node_map
        .clone();
        node_map.retain(|id, _node| included.contains(id));
        ParsedGraph {
            node_map,
            children_map: self.children_map.clone(),
            parents_map: self.children_map.clone(),
        }
    }

    pub fn from_children(
        node_map: HashMap<UniqueId, GraphNode>,
        children_map: HashMap<UniqueId, HashSet<UniqueId>>,
    ) -> Self {
        let parents_map = Self::reverse_edges(&children_map);
        ParsedGraph {
            node_map: node_map,
            parents_map,
            children_map,
        }
    }

    pub fn from_parents(
        node_map: HashMap<UniqueId, GraphNode>,
        parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
    ) -> Self {
        let children_map = Self::reverse_edges(&parents_map);
        ParsedGraph {
            node_map: node_map,
            parents_map,
            children_map,
        }
    }

    pub fn select_childrens_parents(&self, selected: &HashSet<UniqueId>) -> HashSet<UniqueId> {
        let ancestors_for = self.select_children()
    }

    fn bfs_edges(&self, selected: &mut HashSet<UniqueId>, node_id: &UniqueId, max_depth: Option<usize>, reverse: bool) -> Result<HashSet<UniqueId>, SelectionError> {
        match (self.node_map.contains_key(node_id), max_depth) {
            (false, _) => Err(NodeNotInGraph { id: node_id.to_string() }),
            (true, None) | (true, Some(0)) => {
                Ok(*selected)
            },
            (true, Some(max_depth)) => {
                let edges = if reverse { self.parents_map } else { self.children_map };
                let vanguard = edges.get(node_id).unwrap_or(&HashSet::new());
                let to_traverse = edges.iter().filter(|(id, edges)| !selected.contains(*id));
                for (next_id, _edges) in to_traverse {
                    self.descendants(selected, next_id, Some(max_depth - 1));
                }
                Ok(*selected)
            }
        }
    }

    fn descendants(&self, selected: &mut HashSet<UniqueId>, node_id: &UniqueId, max_depth: Option<usize>) -> Result<HashSet<UniqueId>, SelectionError> {
        self.bfs_edges(selected, node_id, max_depth, false)
    }

    fn ancestors(&self, selected: &mut HashSet<UniqueId>, node_id: &UniqueId, max_depth: Option<usize>) -> Result<HashSet<UniqueId>, SelectionError> {
        self.bfs_edges(selected, node_id, max_depth, true)
    }

    pub fn select_children(&self, selected: &HashSet<UniqueId>, max_depth: Option<usize>) {
        let descendants: HashSet<UniqueId> = HashSet::new();
        for node_id in selected {
            descendants.insert(self.descendants(node_id, max_depth))
        }
    }
}

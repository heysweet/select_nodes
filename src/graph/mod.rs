pub mod node;
pub mod node_selector;
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

pub enum SelectionError {
    NoMatchingResourceType(String),
    NodeNotInGraph { id: String },
    SearchError(SearchError),
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
        let mut node_map = self.node_map.clone();
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

    fn bfs_edges(
        &self,
        selected: &HashSet<UniqueId>,
        output: &mut HashSet<UniqueId>,
        node_id: &UniqueId,
        max_depth: Option<usize>,
        reverse: bool,
    ) {
        match max_depth {
            Some(0) => (),
            None | Some(_) => {
                let immutable_output = output.clone();
                let edges = if reverse {
                    &self.parents_map
                } else {
                    &self.children_map
                };
                let empty_set = HashSet::new();
                let vanguard = edges.get(node_id).unwrap_or(&empty_set);
                let to_traverse = vanguard
                    .iter()
                    .filter(|id| !selected.contains(*id) && !immutable_output.contains(*id));
                for next_id in to_traverse {
                    output.insert(next_id.to_string());
                    self.bfs_edges(
                        selected,
                        output,
                        next_id,
                        max_depth.and_then(|d| Some(d - 1)),
                        reverse
                    );
                }
            }
        }
    }

    /// Returns all nodes reachable from `node` in `graph`
    fn descendants(
        &self,
        selected: &HashSet<UniqueId>,
        output: &mut HashSet<UniqueId>,
        node_id: &UniqueId,
        max_depth: Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        match self.node_map.contains_key(node_id) {
            false => Err(NodeNotInGraph {
                id: node_id.to_string(),
            }),
            true => {
                self.bfs_edges(selected, output, node_id, max_depth, false);
                Ok(output.clone())
            },
        }
    }

    /// Returns all nodes having a path to `node` in `graph`
    fn ancestors(
        &self,
        selected: &HashSet<UniqueId>,
        output: &mut HashSet<UniqueId>,
        node_id: &UniqueId,
        max_depth: Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        match self.node_map.contains_key(node_id) {
            false => Err(NodeNotInGraph {
                id: node_id.to_string(),
            }),
            true => {
                self.bfs_edges(selected, output, node_id, max_depth, true);
                Ok(output.clone())
            },
        }
    }

    /// Returns set of all descendents up to a max-depth
    pub fn select_children(
        &self,
        selected: &HashSet<UniqueId>,
        max_depth: Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut descendants: HashSet<UniqueId> = HashSet::new();
        for node_id in selected.iter() {
            self.descendants(selected, &mut descendants, node_id, max_depth)?;
        }
        Ok(descendants)
    }

    /// Returns set of all ancestors up to a max-depth
    pub fn select_parents(
        &self,
        selected: &HashSet<UniqueId>,
        max_depth: Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut ancestors: HashSet<UniqueId> = HashSet::new();
        for node_id in selected.iter() {
            self.ancestors(selected, &mut ancestors, node_id, max_depth)?;
        }
        Ok(ancestors)
    }

    /// For the current selected nodes and the current selected nodes'
    /// descendents, select all ancestors.
    pub fn select_childrens_parents(
        &self,
        selected: &HashSet<UniqueId>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut ancestors_for = self.select_children(selected, None)?;
        ancestors_for.extend(ancestors_for.clone().into_iter());
        self.select_parents(&mut ancestors_for, None)
    }

}

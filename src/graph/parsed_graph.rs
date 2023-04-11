/// https://github.com/dbt-labs/dbt-core/blob/4186f99b742b47e0e95aca4f604cc09e5c67a449/core/dbt/graph/graph.py
use std::collections::HashMap;

use std::collections::HashSet;

use crate::dbt_node_selector::SelectionError;
use crate::dbt_node_selector::SelectionError::*;

pub use String as UniqueId;

use super::node::NodeTypeKey;
use super::node::WrapperNode;
use super::node::WrapperNodeExt;

#[derive(Clone, Debug)]
pub struct ParsedGraph {
    pub node_map: HashMap<UniqueId, WrapperNode>,
    pub children_map: HashMap<UniqueId, HashSet<UniqueId>>,
    /// A map of nodes to its set of parents
    pub parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
    pub sources: HashSet<UniqueId>,
    pub exposures: HashSet<UniqueId>,
    pub metrics: HashSet<UniqueId>,
    pub macros: HashSet<UniqueId>,
}

impl ParsedGraph {
    fn get_subset(
        &self,
        subset_ids: &HashSet<UniqueId>
    ) -> HashMap<UniqueId, WrapperNode>
    {
        subset_ids
            .iter()
            .filter_map(|id| {
                let Some(node) = self.node_map.get(id) else { return None };
                let Some(target_node) = Some(node.clone()) else { return None };
                Some((id.to_string(), target_node))
            })
            .collect()
    }

    pub fn get_sources(&self) -> HashMap<UniqueId, WrapperNode> {
        self.get_subset(&self.sources)
    }

    pub fn get_exposures(&self) -> HashMap<UniqueId, WrapperNode> {
        self.get_subset(&self.exposures)
    }

    pub fn get_metrics(&self) -> HashMap<UniqueId, WrapperNode> {
        self.get_subset(&self.metrics)
    }

    pub fn get_macros(&self) -> HashMap<UniqueId, WrapperNode> {
        self.get_subset(&self.macros)
    }

    fn reverse_edges(
        edge_map: &HashMap<UniqueId, HashSet<UniqueId>>,
    ) -> HashMap<UniqueId, HashSet<UniqueId>> {
        let mut target_map: HashMap<UniqueId, HashSet<UniqueId>> = HashMap::new();

        for (source_id, target_ids) in edge_map.iter() {
            for target_id in target_ids {
                let value = target_map.get_mut(target_id);
                match value {
                    Some(targets) => {
                        targets.insert(source_id.clone());
                    }
                    None => {
                        let mut targets = HashSet::new();
                        targets.insert(source_id.clone());
                        target_map.insert(target_id.clone(), targets);
                    }
                }
            }
        }
        target_map
    }

    pub fn get_node_if(
        &self,
        node_id: &UniqueId,
        is_match: &dyn Fn(&WrapperNode) -> bool,
    ) -> Option<&WrapperNode> {
        let node = self.node_map.get(node_id)?;
        is_match(node).then_some(node)
    }

    pub fn is_node(&self, node_id: &UniqueId, is_match: &dyn Fn(&WrapperNode) -> bool) -> bool {
        self.get_node_if(node_id, is_match).is_some()
    }

    fn filter_by_resource_type(
        included: &HashMap<UniqueId, WrapperNode>,
        resource_type: NodeTypeKey,
    ) -> HashSet<UniqueId> {
        included
            .iter()
            .filter_map(
                |(id, node)| match node.resource_type().key() == resource_type {
                    true => Some(id.to_string()),
                    false => None,
                },
            )
            .collect()
    }

    // Returns a subset of the Graph, does not modify original Graph.
    pub fn filter(&self, included: &HashSet<UniqueId>) -> Self {
        let mut node_map = self.node_map.clone();
        node_map.retain(|id, _node| included.contains(id));
        let node_set: HashSet<&UniqueId> = HashSet::from_iter(node_map.keys());
        ParsedGraph::from(
            node_map,
            self.children_map.clone(),
            self.children_map.clone(),
        )
    }

    fn from(
        node_map: HashMap<UniqueId, WrapperNode>,
        children_map: HashMap<UniqueId, HashSet<UniqueId>>,
        parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
    ) -> Self {
        ParsedGraph {
            sources: Self::filter_by_resource_type(&node_map, NodeTypeKey::Source),
            exposures: Self::filter_by_resource_type(&node_map, NodeTypeKey::Exposure),
            metrics: Self::filter_by_resource_type(&node_map, NodeTypeKey::Metric),
            macros: Self::filter_by_resource_type(&node_map, NodeTypeKey::Macro),
            parents_map,
            children_map,
            node_map,
        }
    }

    pub fn from_children(
        node_map: HashMap<UniqueId, WrapperNode>,
        children_map: HashMap<UniqueId, HashSet<UniqueId>>,
    ) -> Self {
        let parents_map = Self::reverse_edges(&children_map);
        ParsedGraph::from(node_map, children_map, parents_map)
    }

    pub fn from_parents(
        node_map: HashMap<UniqueId, WrapperNode>,
        parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
    ) -> Self {
        let children_map = Self::reverse_edges(&parents_map);
        ParsedGraph::from(node_map, children_map, parents_map)
    }

    fn bfs_edges(
        &self,
        selected: &HashSet<UniqueId>,
        output: &mut HashSet<UniqueId>,
        node_id: &UniqueId,
        max_depth: &Option<usize>,
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
                        &max_depth.and_then(|d| Some(d - 1)),
                        reverse,
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
        max_depth: &Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        match self.node_map.contains_key(node_id) {
            false => Err(NoMatchingResourceType(node_id.to_string())),
            true => {
                self.bfs_edges(selected, output, node_id, max_depth, false);
                Ok(output.clone())
            }
        }
    }

    /// Returns all nodes having a path to `node` in `graph`
    fn ancestors(
        &self,
        selected: &HashSet<UniqueId>,
        output: &mut HashSet<UniqueId>,
        node_id: &UniqueId,
        max_depth: &Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        match self.node_map.contains_key(node_id) {
            false => Err(NodeNotInGraph(node_id.to_string())),
            true => {
                self.bfs_edges(selected, output, node_id, max_depth, true);
                Ok(output.clone())
            }
        }
    }

    /// Returns set of all descendants up to a max-depth
    pub fn select_children(
        &self,
        selected: &HashSet<UniqueId>,
        max_depth: &Option<usize>,
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
        max_depth: &Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut ancestors: HashSet<UniqueId> = HashSet::new();
        for node_id in selected.iter() {
            self.ancestors(selected, &mut ancestors, node_id, max_depth)?;
        }
        Ok(ancestors)
    }

    /// Adds parents to the selected set
    pub fn and_select_parents(
        &self,
        selected: &HashSet<UniqueId>,
        max_depth: &Option<usize>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut parents: HashSet<UniqueId> = self.select_parents(selected, max_depth)?;
        parents.extend(selected.clone());
        Ok(parents)
    }

    /// For the current selected nodes and the current selected nodes'
    /// descendants, select all ancestors.
    pub fn select_childrens_parents(
        &self,
        selected: &HashSet<UniqueId>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut ancestors_for = self.select_children(selected, &None)?;
        ancestors_for.extend(ancestors_for.clone().into_iter());
        self.select_parents(&mut ancestors_for, &None)
    }
}

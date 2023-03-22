pub mod node;
pub mod types;

/// https://github.com/dbt-labs/dbt-core/blob/4186f99b742b47e0e95aca4f604cc09e5c67a449/core/dbt/graph/graph.py

use std::{collections::{HashMap, HashSet}};

use self::node::ParsedNode;

pub use String as UniqueId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedGraph {
    node_map: HashMap<UniqueId, ParsedNode>,
    children_map: HashMap<UniqueId, HashSet<UniqueId>>,
    /// A map of nodes to its set of parents
    parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
}

impl ParsedGraph {
    fn reverse_edges(edge_map: &HashMap<UniqueId, HashSet<UniqueId>>) -> HashMap<UniqueId, HashSet<UniqueId>> {
        let mut target_map: HashMap<UniqueId, HashSet<UniqueId>> = HashMap::new();

        for (source_id, target_ids) in edge_map.clone().iter() {
            for target_id in target_ids {
                let value = target_map.get_mut(target_id);
                match value {
                    Some(targets) => {
                        targets.insert(source_id.clone());
                    },
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

    pub fn new(node_map: HashMap<UniqueId, ParsedNode>, parents_map: HashMap<UniqueId, HashSet<UniqueId>>) -> Self {
        let children_map = Self::reverse_edges(&parents_map);
        ParsedGraph { node_map: node_map, parents_map, children_map }
    }
}
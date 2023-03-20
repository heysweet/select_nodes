/// https://github.com/dbt-labs/dbt-core/blob/4186f99b742b47e0e95aca4f604cc09e5c67a449/core/dbt/graph/graph.py

use std::collections::{HashMap, HashSet};


#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UniqueId(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph {
    children_map: HashMap<UniqueId, HashSet<UniqueId>>,
    /// A map of nodes to its set of parents
    parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
}

impl Graph {
    fn get_children_from_parents(parents_map: &HashMap<UniqueId, HashSet<UniqueId>>) -> HashMap<UniqueId, HashSet<UniqueId>> {
        let mut children_map: HashMap<UniqueId, HashSet<UniqueId>> = HashMap::new();

        for (child_id, parent_ids) in parents_map.clone().iter() {
            for parent_id in parent_ids {
                let value = children_map.get_mut(&parent_id);
                match value {
                    Some(children) => {
                        children.insert(child_id.clone());
                    },
                    None => {
                        let mut children = HashSet::new();
                        children.insert(child_id.clone());
                        children_map.insert(child_id.clone(), children);
                    }
                }
            }
        }
        children_map
    }

    pub fn new(parents_map: HashMap<UniqueId, HashSet<UniqueId>>) -> Self {
        let children_map = Self::get_children_from_parents(&parents_map);
        Graph { parents_map, children_map }
    }
}
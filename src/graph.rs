/// https://github.com/dbt-labs/dbt-core/blob/4186f99b742b47e0e95aca4f604cc09e5c67a449/core/dbt/graph/graph.py

use std::collections::{HashMap, HashSet};


#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UniqueId(String);

pub struct Graph {
    /// A map of nodes to its set of parents
    parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
}
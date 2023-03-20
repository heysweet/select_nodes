use std::collections::{HashMap, HashSet};


#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UniqueId(String);

pub struct Graph {
    /// A map of nodes to its set of parents
    parents_map: HashMap<UniqueId, HashSet<UniqueId>>,
}
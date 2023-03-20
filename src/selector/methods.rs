use std::collections::HashMap;

use crate::graph::{ParsedGraph, UniqueId, node::{ParsedNode, NodeType}};

use super::{SearchMethod, FileMethod, ResourceTypeMethod};

impl SearchMethod for ResourceTypeMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        let maybe_resource_type = NodeType::from_string(&selector);
        todo!()
    }
}

impl SearchMethod for FileMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}
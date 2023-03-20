use std::collections::HashMap;

use crate::graph::{ParsedGraph, UniqueId, node::{ParsedNode, NodeType}};

use super::{SearchMethod, FileMethod, ResourceTypeMethod, FQNMethod, TagMethod, GroupMethod, SourceMethod, PathMethod, PackageMethod, ConfigMethod, TestNameMethod, TestTypeMethod, StateMethod, ExposureMethod, MetricMethod, ResultMethod, SourceStatusMethod, WildcardMethod};

impl SearchMethod for FQNMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for TagMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for GroupMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for SourceMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for PathMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for FileMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for PackageMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for ConfigMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for TestNameMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for TestTypeMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for ResourceTypeMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        let maybe_resource_type = NodeType::from_string(&selector);
        todo!()
    }
}

impl SearchMethod for StateMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for ExposureMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for MetricMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for ResultMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for SourceStatusMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}

impl SearchMethod for WildcardMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &ParsedNode>, selector: String) -> std::slice::Iter<UniqueId> {
        todo!()
    }
}
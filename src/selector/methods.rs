use std::collections::HashMap;

use crate::graph::{ParsedGraph, UniqueId, node::{ParsedNode, NodeType}};

use super::{SearchMethod, FileMethod, ResourceTypeMethod, FQNMethod, TagMethod, GroupMethod, SourceMethod, PathMethod, PackageMethod, ConfigMethod, TestNameMethod, TestTypeMethod, StateMethod, ExposureMethod, MetricMethod, ResultMethod, SourceStatusMethod, WildcardMethod};

impl SearchMethod for FQNMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for TagMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for GroupMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for SourceMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for PathMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for FileMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for PackageMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for ConfigMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for TestNameMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for TestTypeMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for ResourceTypeMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        let maybe_resource_type = NodeType::from_string(&selector);
        todo!()
    }
}

impl SearchMethod for StateMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for ExposureMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for MetricMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for ResultMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for SourceStatusMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}

impl SearchMethod for WildcardMethod {
    fn search<'a>(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &'a ParsedNode>, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        todo!()
    }
}
use crate::graph::ParsedGraph;

use super::{SearchMethod, FileMethod, ResourceTypeMethod, FQNMethod, TagMethod, GroupMethod, SourceMethod, PathMethod, PackageMethod, ConfigMethod, TestNameMethod, TestTypeMethod, StateMethod, ExposureMethod, MetricMethod, ResultMethod, SourceStatusMethod, WildcardMethod, SearchError};

impl<Iter> SearchMethod<Iter> for FQNMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for TagMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for GroupMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for SourceMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for PathMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for FileMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for PackageMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for ConfigMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for TestNameMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for TestTypeMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for ResourceTypeMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        let iter = graph.node_map.iter();
        let iter = iter.filter(|(id, node)| node.resource_type.key() == selector);
        let iter = iter.map(|(id, node)| *id);
        iter.collect()
    }
}

impl<Iter> SearchMethod<Iter> for StateMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for ExposureMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) -> Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for MetricMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) ->Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for ResultMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) ->Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for SourceStatusMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) ->Iter {
        todo!()
    }
}

impl<Iter> SearchMethod<Iter> for WildcardMethod where Iter: Iterator<Item = String> + std::iter::FromIterator<std::string::String> {
    fn search(&self, graph: &ParsedGraph, selector: &str) ->Iter {
        todo!()
    }
}
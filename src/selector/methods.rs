use crate::graph::ParsedGraph;

use super::MethodName;

use MethodName::*;

impl MethodName {
    pub fn search(&self, graph: &ParsedGraph, selector: &str) -> Vec<String> {
        match self {
            FQN => {
                unimplemented!()
            }

            Tag => {
                unimplemented!()
            }

            Group => {
                unimplemented!()
            }

            Source => {
                unimplemented!()
            }

            Path => {
                unimplemented!()
            }

            File => {
                unimplemented!()
            }

            Package => {
                unimplemented!()
            }

            Config => {
                unimplemented!()
            }

            TestName => {
                unimplemented!()
            }

            TestType => {
                unimplemented!()
            }

            ResourceType => {
                let iter = graph.node_map.iter();
                let iter = iter.filter(|(_, node)| node.resource_type.key() == selector);
                let iter = iter.map(|(id, _)| id.clone());
                iter.collect()
            }

            State => {
                unimplemented!()
            }

            Exposure => {
                unimplemented!()
            }

            Metric => {
                unimplemented!()
            }

            Result => {
                unimplemented!()
            }

            SourceStatus => {
                unimplemented!()
            }

            Wildcard => {
                unimplemented!()
            }
        }
    }
}

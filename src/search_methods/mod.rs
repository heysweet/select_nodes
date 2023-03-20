/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/graph/selector_methods.py

use std::collections::{HashMap};

use crate::{types::{SourceDefinition, ManifestNode, Exposure, Metric}, node::Node, graph::{UniqueId, ParsedGraph}};

#[derive(Copy, Clone)]
pub union SelectorTarget { source_definition: SourceDefinition, manifest_node: ManifestNode, exposure: Exposure, metric: Metric }

pub trait SearchMethod {
    fn search(graph: ParsedGraph, included_nodes: HashMap<UniqueId, &Node>, selector: String) -> std::slice::Iter<UniqueId>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum AccessType {
    Protected,
    Private,
    Public
}

impl AccessType {
    pub fn key(&self) -> &str {
        match self {
            AccessType::Protected => "protected",
            AccessType::Private => "private",
            AccessType::Public => "public",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum MethodName {
    FQN,
    Tag,
    Group,
    Source,
    Path,
    File,
    Package,
    Config,
    TestName,
    TestType,
    ResourceType,
    State,
    Exposure,
    Metric,
    Result,
    SourceStatus,
    Wildcard
}

impl MethodName {
    pub fn key(&self) -> &str {
        match self {
            MethodName::FQN => "fqn",
            MethodName::Tag => "tag",
            MethodName::Group => "group",
            MethodName::Source => "source",
            MethodName::Path => "path",
            MethodName::File => "file",
            MethodName::Package => "package",
            MethodName::Config => "config",
            MethodName::TestName => "test_name",
            MethodName::TestType => "test_type",
            MethodName::ResourceType => "resource_type",
            MethodName::State => "state",
            MethodName::Exposure => "exposure",
            MethodName::Metric => "metric",
            MethodName::Result => "result",
            MethodName::SourceStatus => "source_status",
            MethodName::Wildcard => "wildcard",
        }
    }
}
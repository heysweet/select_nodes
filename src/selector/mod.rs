/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/graph/selector_methods.py

pub mod methods;
pub mod spec;

use crate::graph::{types::{SourceDefinition, ManifestNode, Exposure, Metric}, {UniqueId, ParsedGraph}};

#[derive(Copy, Clone)]
pub union SelectorTarget { source_definition: SourceDefinition, manifest_node: ManifestNode, exposure: Exposure, metric: Metric }

enum SearchError {
    NoMatchingResourceType(String),
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
pub enum MethodName {
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
    Wildcard,
}

use MethodName::*;

impl MethodName {
    pub fn key(&self) -> &str {
        match self {
            FQNMethod => "fqn",
            TagMethod => "tag",
            GroupMethod => "group",
            SourceMethod => "source",
            PathMethod => "path",
            FileMethod => "file",
            PackageMethod => "package",
            ConfigMethod => "config",
            TestNameMethod => "test_name",
            TestTypeMethod => "test_type",
            ResourceTypeMethod => "resource_type",
            StateMethod => "state",
            ExposureMethod => "exposure",
            MetricMethod => "metric",
            ResultMethod => "result",
            SourceStatusMethod => "source_status",
            WildcardMethod => "wildcard",
        }
    }

    pub fn from_string(input: impl Into<String>) -> Option<MethodName> {
        match input.into().as_str() {
            "fqn" => Some(FQN),
            "tag" => Some(Tag),
            "group" => Some(Group),
            "source" => Some(Source),
            "path" => Some(Path),
            "file" => Some(File),
            "package" => Some(Package),
            "config" => Some(Config),
            "test_name" => Some(TestName),
            "test_type" => Some(TestType),
            "resource_type" => Some(ResourceType),
            "state" => Some(State),
            "exposure" => Some(Exposure),
            "metric" => Some(Metric),
            "result" => Some(Result),
            "source_status" => Some(SourceStatus),
            "wildcard" => Some(Wildcard),
            _ => None,
        }
    }
}
/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/graph/selector_methods.py

pub mod methods;
pub mod spec;

use crate::graph::{types::{SourceDefinition, ManifestNode, Exposure, Metric}, {UniqueId, ParsedGraph}};

#[derive(Copy, Clone)]
pub union SelectorTarget { source_definition: SourceDefinition, manifest_node: ManifestNode, exposure: Exposure, metric: Metric }

pub trait SearchMethod {
    fn search<'a>(&self, graph: ParsedGraph, selector: &'a str) -> std::slice::Iter<'a, UniqueId>;
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
struct FQNMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct TagMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]

struct GroupMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SourceMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct PathMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct FileMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct PackageMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ConfigMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct TestNameMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct TestTypeMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ResourceTypeMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct StateMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ExposureMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct MetricMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ResultMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SourceStatusMethod {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct WildcardMethod {}


#[derive(Copy, Clone)]
pub union MethodName {
    FQN: FQNMethod,
    Tag: TagMethod,
    Group: GroupMethod,
    Source: SourceMethod,
    Path: PathMethod,
    File: FileMethod,
    Package: PackageMethod,
    Config: ConfigMethod,
    TestName: TestNameMethod,
    TestType: TestTypeMethod,
    ResourceType: ResourceTypeMethod,
    State: StateMethod,
    Exposure: ExposureMethod,
    Metric: MetricMethod,
    Result: ResultMethod,
    SourceStatus: SourceStatusMethod,
    Wildcard: WildcardMethod,
}

impl SearchMethod for MethodName {
    fn search<'a>(&self, graph: ParsedGraph, selector: &'a str) -> std::slice::Iter<'a, UniqueId> {
        panic!("All derived types must implement this")
    }
}

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

    pub fn from_string(input: &str) -> Option<MethodName> {
        match input {
            "fqn" => Some(MethodName{FQN: FQNMethod{}}),
            "tag" => Some(MethodName{Tag: TagMethod{}}),
            "group" => Some(MethodName{Group: GroupMethod{}}),
            "source" => Some(MethodName{Source: SourceMethod{}}),
            "path" => Some(MethodName{Path: PathMethod{}}),
            "file" => Some(MethodName{File: FileMethod{}}),
            "package" => Some(MethodName{Package: PackageMethod{}}),
            "config" => Some(MethodName{Config: ConfigMethod{}}),
            "test_name" => Some(MethodName{TestName: TestNameMethod{}}),
            "test_type" => Some(MethodName{TestType: TestTypeMethod{}}),
            "resource_type" => Some(MethodName{ResourceType: ResourceTypeMethod{}}),
            "state" => Some(MethodName{State: StateMethod{}}),
            "exposure" => Some(MethodName{Exposure: ExposureMethod{}}),
            "metric" => Some(MethodName{Metric: MetricMethod{}}),
            "result" => Some(MethodName{Result: ResultMethod{}}),
            "source_status" => Some(MethodName{SourceStatus: SourceStatusMethod{}}),
            "wildcard" => Some(MethodName{Wildcard: WildcardMethod{}}),
            _ => None,
        }
    }
}
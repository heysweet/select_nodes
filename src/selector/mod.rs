/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/graph/selector_methods.py
pub mod methods;
pub mod node_selector;
pub mod resource_type_filter;
pub mod spec;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum AccessType {
    Protected,
    Private,
    Public,
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
    /// Yield all nodes in the graph that match the selector.
    FQN,
    /// Yields nodes from included that have the specified tag.
    Tag,
    /// Yields nodes from included in the specified group.
    Group,
    /// Yields nodes from included are the specified source.
    Source,
    /// Yields nodes from included that match the given path.
    Path,
    /// Yields nodes from included that match the given file name.
    File,
    /// Yields nodes from included that have the specified package.
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

/// Decide if the value is probably a path. Windows has two path separators, so
/// we should check both '\\' and '/' there.
fn _probably_path(value: &str) -> bool {
    if value.contains('/') {
        true
    } else {
        value.contains(std::path::MAIN_SEPARATOR)
    }
}

use MethodName::*;

impl MethodName {
    pub fn key(&self) -> &str {
        match self {
            FQN => "fqn",
            Tag => "tag",
            Group => "group",
            Source => "source",
            Path => "path",
            File => "file",
            Package => "package",
            Config => "config",
            TestName => "test_name",
            TestType => "test_type",
            ResourceType => "resource_type",
            State => "state",
            Exposure => "exposure",
            Metric => "metric",
            Result => "result",
            SourceStatus => "source_status",
            Wildcard => "wildcard",
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

    pub fn default_method(value: impl Into<String>) -> MethodName {
        let value = value.into();
        let is_probably_path = _probably_path(&value);
        let lowercase_value = value.to_lowercase();
        let is_relevant_filetype = lowercase_value.ends_with(".sql")
            || lowercase_value.ends_with(".py")
            || lowercase_value.ends_with(".csv");
        match (is_probably_path, is_relevant_filetype) {
            (true, _) => MethodName::Path,
            (_, true) => MethodName::File,
            _ => MethodName::FQN,
        }
    }
}

/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/node_types.py

use crate::graph::UniqueId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NodeType {
    Model,
    Analysis,
    Test,
    Snapshot,
    Operation,
    Seed,
    // TODO: rm?
    RPCCall,
    SqlOperation,
    Documentation,
    Source,
    Macro,
    Exposure,
    Metric,
    Group
}

impl NodeType {
    pub fn key(&self) -> &str {
        match self {
            NodeType::Model => "model",
            NodeType::Analysis => "analysis",
            NodeType::Test => "test",
            NodeType::Snapshot => "snapshot",
            NodeType::Operation => "operation",
            NodeType::Seed => "seed",
            NodeType::RPCCall => "rpc",
            NodeType::SqlOperation => "sql operation",
            NodeType::Documentation => "doc",
            NodeType::Source => "source",
            NodeType::Macro => "macro",
            NodeType::Exposure => "exposure",
            NodeType::Metric => "metric",
            NodeType::Group => "group",
        }
    }

    pub fn from_string(resource_type: &str) -> Option<NodeType> {
        match resource_type {
            "model" => Some(NodeType::Model),
            "analysis" => Some(NodeType::Analysis),
            "test" => Some(NodeType::Test),
            "snapshot" => Some(NodeType::Snapshot),
            "operation" => Some(NodeType::Operation),
            "seed" => Some(NodeType::Seed),
            "rpc" => Some(NodeType::RPCCall),
            "sql operation" => Some(NodeType::SqlOperation),
            "doc" => Some(NodeType::Documentation),
            "source" => Some(NodeType::Source),
            "macro" => Some(NodeType::Macro),
            "exposure" => Some(NodeType::Exposure),
            "metric" => Some(NodeType::Metric),
            "group" => Some(NodeType::Group),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct UnparsedBaseNode {
    name: String,
    resource_type: String,
    package_name: String,
    path: String,
    original_file_path: String,
    unique_id: String,
}


#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct BaseNode {
    name: String,
    resource_type: NodeType,
    package_name: String,
    path: String,
    original_file_path: String,
    unique_id: UniqueId,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Node {
    unique_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParsedNode {
    unique_id: UniqueId,
}
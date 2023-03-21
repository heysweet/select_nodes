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

struct NoMatchingResourceType {}

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

    pub fn from_string(resource_type: &str) -> Result<NodeType, NoMatchingResourceType> {
        match resource_type {
            "model" => Ok(NodeType::Model),
            "analysis" => Ok(NodeType::Analysis),
            "test" => Ok(NodeType::Test),
            "snapshot" => Ok(NodeType::Snapshot),
            "operation" => Ok(NodeType::Operation),
            "seed" => Ok(NodeType::Seed),
            "rpc" => Ok(NodeType::RPCCall),
            "sql operation" => Ok(NodeType::SqlOperation),
            "doc" => Ok(NodeType::Documentation),
            "source" => Ok(NodeType::Source),
            "macro" => Ok(NodeType::Macro),
            "exposure" => Ok(NodeType::Exposure),
            "metric" => Ok(NodeType::Metric),
            "group" => Ok(NodeType::Group),
            _ => Err(NoMatchingResourceType{}),
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

impl Node {
    pub fn new(unique_id: &str) -> Node {
        Node{ unique_id: unique_id.to_string() }
    }

    pub fn parse(&self) -> ParsedNode {
        // TODO: we're not validating this is unique, and cannot from
        // a parse on Node itself
        ParsedNode{ unique_id: UniqueId(self.unique_id) }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParsedNode {
    unique_id: UniqueId,
}
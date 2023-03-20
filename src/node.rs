use crate::graph::UniqueId;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum NodeType {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Node {

}
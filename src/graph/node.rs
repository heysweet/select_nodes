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
    Group,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NoMatchingResourceType {}

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

    pub fn from_string(
        resource_type: impl Into<String>,
    ) -> Result<NodeType, NoMatchingResourceType> {
        match resource_type.into().as_str() {
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
            _ => Err(NoMatchingResourceType {}),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// All nodes or node-like objects in this file should have these properties
pub struct BaseNodeData {
    pub unique_id: UniqueId,
    pub resource_type: NodeType,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Nodes in the DAG
pub struct GraphNode {
    pub unique_id: UniqueId,
    pub resource_type: NodeType,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    /// Macro and Documentation don't have fqn
    pub fqn: Vec<String>,
}

pub trait GraphNodeData {}

use indexmap::IndexMap;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NodeCreateError {
    MissingFieldError { field_name: String },
    NoMatchingResourceType { resource_type: String },
}

use NodeCreateError::*;

impl GraphNode {
    pub fn from_indexmap(
        fqn: Vec<String>,
        index_map: &IndexMap<String, String>,
    ) -> Result<Self, NodeCreateError> {
        let resource_type = Self::get_required_key(index_map, "resource_type")?;
        Ok(Self {
            unique_id: Self::get_required_key(index_map, "unique_id")?,
            name: Self::get_required_key(index_map, "name")?,
            resource_type: NodeType::from_string(&resource_type)
                .or_else(|_| Err(NodeCreateError::NoMatchingResourceType { resource_type }))?,
            package_name: Self::get_required_key(index_map, "package_name")?,
            path: Self::get_required_key(index_map, "path")?,
            original_file_path: Self::get_required_key(index_map, "original_file_path")?,
            fqn,
        })
    }

    pub fn new(
        fqn: Vec<impl Into<String>>,
        unique_id: impl Into<String>,
        name: impl Into<String>,
        resource_type: impl Into<String>,
        package_name: impl Into<String>,
        path: impl Into<String>,
        original_file_path: impl Into<String>,
    ) -> Result<Self, NodeCreateError> {
        let resource_type: String = resource_type.into();
        let resource_type = NodeType::from_string(&resource_type)
            .or_else(|_| Err(NodeCreateError::NoMatchingResourceType { resource_type }))?;
        Ok(Self {
            fqn: fqn.into_iter().map(|s| s.into()).collect(),
            unique_id: unique_id.into(),
            name: name.into(),
            resource_type,
            package_name: package_name.into(),
            path: path.into(),
            original_file_path: original_file_path.into(),
        })
    }

    fn get_required_key(
        index_map: &IndexMap<String, String>,
        key: &str,
    ) -> Result<String, NodeCreateError> {
        Ok(index_map
            .get(key)
            .ok_or_else(|| MissingFieldError {
                field_name: key.to_string(),
            })?
            .to_string())
    }
}

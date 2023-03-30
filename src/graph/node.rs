use std::collections::HashMap;

/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/node_types.py
use crate::{graph::UniqueId, interface::{self, NodeCreateError}};

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

impl GraphNode {
    pub fn from(
        node: &interface::Node,
    ) -> Result<Self, NodeCreateError> {
        let resource_type = node.resource_type;
        
        Ok(Self {
            unique_id: node.unique_id,
            name: node.name,
            resource_type: NodeType::from_string(&resource_type)
                .or_else(|_| Err(NodeCreateError::NoMatchingResourceType(resource_type)))?,
            package_name: node.package_name,
            path: node.path,
            original_file_path: node.original_file_path,
            fqn: node.fqn,
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
            .or_else(|_| Err(NodeCreateError::NoMatchingResourceType(resource_type)))?;
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
            .ok_or_else(|| {
                NodeCreateError::MissingField(
                            key.to_string(),
                        )
            })?
            .to_string())
    }
}

pub fn generate_node_hash_map(nodes: Vec<GraphNode>) -> HashMap<UniqueId, GraphNode> {
    nodes
        .iter()
        .map(|node| (node.unique_id.clone(), node.clone()))
        .collect()
}
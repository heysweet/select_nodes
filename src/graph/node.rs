use std::collections::HashMap;
use std::fmt::Display;

/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/node_types.py
use crate::graph::UniqueId;

use crate::interface::{Node, NodeType};

use crate::SelectorCreateError;
use crate::SelectorCreateError::*;

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Display for SelectorCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoMatchingResourceType(resource_type) => {
                write!(f, "Invalid resource_type '{}'", resource_type)
            }

            MissingField(field) => {
                write!(f, "Missing required field '{}'", field)
            }
        }
    }
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
            NodeType::Rpc => "rpc",
            NodeType::SqlOperation => "sql operation",
            NodeType::Doc => "doc",
            NodeType::Source => "source",
            NodeType::Macro => "macro",
            NodeType::Exposure => "exposure",
            NodeType::Metric => "metric",
            NodeType::Group => "group",
        }
    }

    pub fn from_string(resource_type: impl Into<String>) -> Result<NodeType, SelectorCreateError> {
        let resource_type = resource_type.into();
        match resource_type.as_str() {
            "model" => Ok(NodeType::Model),
            "analysis" => Ok(NodeType::Analysis),
            "test" => Ok(NodeType::Test),
            "snapshot" => Ok(NodeType::Snapshot),
            "operation" => Ok(NodeType::Operation),
            "seed" => Ok(NodeType::Seed),
            "rpc" => Ok(NodeType::Rpc),
            "sql operation" => Ok(NodeType::SqlOperation),
            "doc" => Ok(NodeType::Doc),
            "source" => Ok(NodeType::Source),
            "macro" => Ok(NodeType::Macro),
            "exposure" => Ok(NodeType::Exposure),
            "metric" => Ok(NodeType::Metric),
            "group" => Ok(NodeType::Group),
            _ => Err(NoMatchingResourceType(resource_type)),
        }
    }
}

#[derive(Clone, Debug)]
/// All nodes or node-like objects in this file should have these properties
pub struct BaseNodeData {
    pub unique_id: UniqueId,
    pub resource_type: NodeType,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
}

#[derive(Clone, Debug)]
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
    pub fn from(node: &Node) -> Result<Self, SelectorCreateError> {
        let resource_type = &node.resource_type;

        Ok(Self {
            unique_id: node.unique_id.to_owned(),
            name: node.name.to_owned(),
            resource_type: NodeType::from_string(resource_type)
                .or_else(|_| Err(NoMatchingResourceType(resource_type.to_owned())))?,
            package_name: node.package_name.to_owned(),
            path: node.path.to_owned(),
            original_file_path: node.original_file_path.to_owned(),
            fqn: node.fqn.to_owned(),
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
    ) -> Result<Self, SelectorCreateError> {
        let resource_type: String = resource_type.into();
        let resource_type = NodeType::from_string(&resource_type)
            .or_else(|_| Err(NoMatchingResourceType(resource_type)))?;
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
    ) -> Result<String, SelectorCreateError> {
        Ok(index_map
            .get(key)
            .ok_or_else(|| MissingField(key.to_string()))?
            .to_string())
    }
}

pub fn generate_node_hash_map(nodes: Vec<GraphNode>) -> HashMap<UniqueId, GraphNode> {
    nodes
        .iter()
        .map(|node| (node.unique_id.clone(), node.clone()))
        .collect()
}

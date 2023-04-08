use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/node_types.py
use crate::graph::UniqueId;

use crate::dbt_node_selector::{Node, NodeType};

use crate::SelectorCreateError;
use crate::SelectorCreateError::*;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeKey {
    Model,
    Analysis,
    Test,
    Snapshot,
    Operation ,
    Seed,
    Rpc,
    SqlOperation,
    Doc,
    Source,
    Macro,
    Exposure,
    Metric,
    Group,
}

impl NodeTypeKey {
    pub fn key(&self) -> &str {
        match self {
            NodeTypeKey::Model => "model",
            NodeTypeKey::Analysis => "analysis",
            NodeTypeKey::Test => "test",
            NodeTypeKey::Snapshot => "snapshot",
            NodeTypeKey::Operation => "operation",
            NodeTypeKey::Seed => "seed",
            NodeTypeKey::Rpc => "rpc",
            NodeTypeKey::SqlOperation => "sql operation",
            NodeTypeKey::Doc => "doc",
            NodeTypeKey::Source => "source",
            NodeTypeKey::Macro => "macro",
            NodeTypeKey::Exposure => "exposure",
            NodeTypeKey::Metric => "metric",
            NodeTypeKey::Group => "group",
        }
    }

    pub fn from_key(key: impl Into<String>) -> Result<Self, SelectorCreateError> {
        let key_str = key.into();
        match key_str.as_str() {
            "model" => Ok(Self::Model),
            "analysis" => Ok(Self::Analysis),
            "test" => Ok(Self::Test),
            "snapshot" => Ok(Self::Snapshot),
            "operation" => Ok(Self::Operation),
            "seed" => Ok(Self::Seed),
            "rpc" => Ok(Self::Rpc),
            "sql operation" => Ok(Self::SqlOperation),
            "doc" => Ok(Self::Doc),
            "source" => Ok(Self::Source),
            "macro" => Ok(Self::Macro),
            "exposure" => Ok(Self::Exposure),
            "metric" => Ok(Self::Metric),
            "group" => Ok(Self::Group),
            _ => Err(NoMatchingResourceType(key_str)),
        }
    }

    pub fn from_node_type(node_type: &NodeType) -> Self {
        match node_type {
            NodeType::Model(_) => Self::Model,
            NodeType::Analysis(_) => Self::Analysis,
            NodeType::Test(_) => Self::Test,
            NodeType::Snapshot(_) => Self::Snapshot,
            NodeType::Operation(_) => Self::Operation,
            NodeType::Seed(_) => Self::Seed,
            NodeType::Rpc(_) => Self::Rpc,
            NodeType::SqlOperation(_) => Self::SqlOperation,
            NodeType::Doc(_) => Self::Doc,
            NodeType::Source(_) => Self::Source,
            NodeType::Macro(_) => Self::Macro,
            NodeType::Exposure(_) => Self::Exposure,
            NodeType::Metric(_) => Self::Metric,
            NodeType::Group(_) => Self::Group,                 
        }
    }
}

impl NodeType {
    pub fn key(&self) -> NodeTypeKey {
        NodeTypeKey::from_node_type(self)
    }

    pub fn key_matches(&self, other: &Self) -> bool {
        self.key() == other.key()
    }

    pub fn fqn(&self) -> Option<Vec<String>> {
        match self {
            NodeType::Doc(_) => None,
            NodeType::Macro(_) => None,
            NodeType::Model(data) => Some(data.fqn.clone()),
            NodeType::Analysis(data) => Some(data.fqn.clone()),
            NodeType::Test(data) => Some(data.fqn.clone()),
            NodeType::Snapshot(data) => Some(data.fqn.clone()),
            NodeType::Operation(data) => Some(data.fqn.clone()),
            NodeType::Seed(data) => Some(data.fqn.clone()),
            NodeType::Rpc(data) => Some(data.fqn.clone()),
            NodeType::SqlOperation(data) => Some(data.fqn.clone()),
            NodeType::Source(data) => Some(data.fqn.clone()),
            NodeType::Exposure(data) => Some(data.fqn.clone()),
            NodeType::Metric(data) => Some(data.fqn.clone()),
            NodeType::Group(data) => Some(data.fqn.clone()),
        }
    }
}

use crate::dbt_node_selector::*;

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
pub struct ParsedNode {
    pub unique_id: UniqueId,
    pub depends_on: HashSet<UniqueId>,
    pub resource_type: NodeType,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
}

#[derive(Clone, Debug)]
/// Nodes in the DAG
pub struct ParsedMacroNode {
    pub unique_id: UniqueId,
    pub depends_on: HashSet<UniqueId>,
    pub macro_node: MacroNode,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
}

impl ParsedMacroNode {
    pub fn from(parsed_node: &ParsedNode) -> Result<Self, SelectorCreateError> {
        let NodeType::Macro(macro_node) = parsed_node.resource_type.clone() else { return Err(SelectorCreateError::NoMatchingResourceType("macro".to_string()))};
        Ok(Self {
            macro_node,
            unique_id: parsed_node.unique_id.clone(),
            depends_on: parsed_node.depends_on.clone(),
            name: parsed_node.name.clone(),
            package_name: parsed_node.package_name.clone(),
            path: parsed_node.path.clone(),
            original_file_path: parsed_node.original_file_path.clone(),
        })
    }
}

#[derive(Clone, Debug)]
/// Nodes in the DAG
pub struct ParsedSourceNode {
    pub unique_id: UniqueId,
    pub depends_on: HashSet<UniqueId>,
    pub source_node: SourceNode,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
}

impl ParsedSourceNode {
    pub fn from(parsed_node: &ParsedNode) -> Result<Self, SelectorCreateError> {
        let NodeType::Source(source_node) = parsed_node.resource_type.clone() else { return Err(SelectorCreateError::NoMatchingResourceType("source".to_string()))};
        Ok(Self {
            source_node,
            unique_id: parsed_node.unique_id.clone(),
            depends_on: parsed_node.depends_on.clone(),
            name: parsed_node.name.clone(),
            package_name: parsed_node.package_name.clone(),
            path: parsed_node.path.clone(),
            original_file_path: parsed_node.original_file_path.clone(),
        })
    }
}

#[derive(Clone, Debug)]
/// Nodes in the DAG
pub struct ParsedExposureNode {
    pub unique_id: UniqueId,
    pub depends_on: HashSet<UniqueId>,
    pub exposure_node: ExposureNode,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
}


impl ParsedExposureNode {
    pub fn from(parsed_node: &ParsedNode) -> Result<Self, SelectorCreateError> {
        let NodeType::Exposure(exposure_node) = parsed_node.resource_type.clone() else { return Err(SelectorCreateError::NoMatchingResourceType("exposure".to_string()))};
        Ok(Self {
            exposure_node,
            unique_id: parsed_node.unique_id.clone(),
            depends_on: parsed_node.depends_on.clone(),
            name: parsed_node.name.clone(),
            package_name: parsed_node.package_name.clone(),
            path: parsed_node.path.clone(),
            original_file_path: parsed_node.original_file_path.clone(),
        })
    }
}

#[derive(Clone, Debug)]
/// Nodes in the DAG
pub struct ParsedMetricNode {
    pub unique_id: UniqueId,
    pub depends_on: HashSet<UniqueId>,
    pub metric_node: MetricNode,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
}


impl ParsedMetricNode {
    pub fn from(parsed_node: &ParsedNode) -> Result<Self, SelectorCreateError> {
        let NodeType::Metric(metric_node) = parsed_node.resource_type.clone() else { return Err(SelectorCreateError::NoMatchingResourceType("metric".to_string()))};
        Ok(Self {
            metric_node,
            unique_id: parsed_node.unique_id.clone(),
            depends_on: parsed_node.depends_on.clone(),
            name: parsed_node.name.clone(),
            package_name: parsed_node.package_name.clone(),
            path: parsed_node.path.clone(),
            original_file_path: parsed_node.original_file_path.clone(),
        })
    }
}

use indexmap::IndexMap;

impl ParsedNode {
    pub fn fqn(&self) -> Option<Vec<String>> {
        self.resource_type.fqn()
    }

    pub fn from(node: &Node) -> Result<Self, SelectorCreateError> {

        Ok(Self {
            unique_id: node.unique_id.to_owned(),
            depends_on: HashSet::from_iter(node.depends_on.iter().map(|s| s.to_string())),
            name: node.name.to_owned(),
            resource_type: node.node_type.clone(),
            package_name: node.package_name.to_owned(),
            path: node.path.to_owned(),
            original_file_path: node.original_file_path.to_owned(),
        })
    }

    pub fn new(
        unique_id: impl Into<String>,
        depends_on: Vec<String>,
        name: impl Into<String>,
        package_name: impl Into<String>,
        path: impl Into<String>,
        original_file_path: impl Into<String>,
        resource_type: NodeType,
    ) -> Result<Self, SelectorCreateError> {
        Ok(Self {
            unique_id: unique_id.into(),
            depends_on: depends_on.iter().map(|s| s.into()).collect(),
            name: name.into(),
            package_name: package_name.into(),
            path: path.into(),
            original_file_path: original_file_path.into(),
            resource_type,
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

pub fn generate_node_hash_map(nodes: Vec<ParsedNode>) -> HashMap<UniqueId, ParsedNode> {
    nodes
        .iter()
        .map(|node| (node.unique_id.clone(), node.clone()))
        .collect()
}

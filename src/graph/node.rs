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
    Operation,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommonNodeData {
    pub unique_id: UniqueId,
    pub depends_on: HashSet<UniqueId>,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub config: HashMap<String, String>,
    pub tags: HashSet<String>,
}

#[derive(Clone, Debug)]
/// A generic node type which holds all node-specific logic inside `resource_type`
pub struct WrapperNode {
    common: CommonNodeData,
    resource_type: NodeType,
}

pub trait WrapperNodeExt {
    fn unique_id(&self) -> &UniqueId;
    fn depends_on(&self) -> &HashSet<UniqueId>;
    fn name(&self) -> &String;
    fn package_name(&self) -> &String;
    fn path(&self) -> &String;
    fn original_file_path(&self) -> &String;
    fn config(&self) -> &HashMap<String, String>;
    fn tags(&self) -> &HashSet<String>;
    fn has_tag(&self, tag: &String) -> bool;
    fn resource_type(&self) -> &NodeType;

    fn fqn(&self) -> Option<Vec<String>>;
    fn same_content(&self, other: &Self) -> bool;
}

impl WrapperNodeExt for WrapperNode {
    fn unique_id(&self) -> &UniqueId {
        &self.common.unique_id
    }
    fn depends_on(&self) -> &HashSet<UniqueId> {
        &self.common.depends_on
    }
    fn name(&self) -> &String {
        &self.common.name
    }
    fn package_name(&self) -> &String {
        &self.common.package_name
    }
    fn path(&self) -> &String {
        &self.common.path
    }
    fn original_file_path(&self) -> &String {
        &self.common.original_file_path
    }
    fn config(&self) -> &HashMap<String, String> {
        &self.common.config
    }
    fn tags(&self) -> &HashSet<UniqueId> {
        &self.common.tags
    }
    fn has_tag(&self, tag: &String) -> bool {
        self.common.tags.contains(tag)
    }
    fn resource_type(&self) -> &NodeType {
        &self.resource_type
    }

    fn fqn(&self) -> Option<Vec<UniqueId>> {
        self.resource_type.fqn()
    }

    fn same_content(&self, other: &Self) -> bool {
        self.resource_type.same_content(&other.resource_type)
    }
}

use indexmap::IndexMap;

use super::parsed_graph::ParsedGraph;

impl WrapperNode {
    pub fn fqn(&self) -> Option<Vec<String>> {
        self.resource_type.fqn()
    }

    pub fn from(node: &Node) -> Result<Self, SelectorCreateError> {
        Ok(Self {
            common: CommonNodeData {
                unique_id: node.unique_id.to_owned(),
                depends_on: HashSet::from_iter(node.depends_on.iter().map(|s| s.to_string())),
                name: node.name.to_owned(),
                package_name: node.package_name.to_owned(),
                path: node.path.to_owned(),
                original_file_path: node.original_file_path.to_owned(),
                config: node.config.into_iter().collect(),
                tags: node.tags.into_iter().map(|tag| tag.to_lowercase()).collect(),
            },
            resource_type: node.node_type.to_owned(),
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
        tags: Vec<String>,
        config: Vec<(String, String)>,
    ) -> Result<Self, SelectorCreateError> {
        Ok(Self {
            common: CommonNodeData {
                unique_id: unique_id.into(),
                depends_on: depends_on.iter().map(|s| s.into()).collect(),
                name: name.into(),
                package_name: package_name.into(),
                path: path.into(),
                original_file_path: original_file_path.into(),
                config: config.into_iter().collect(),
                tags: tags.into_iter().map(|tag| tag.to_lowercase()).collect(),
            },
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

    pub fn depends_on_macros(&self, graph: &ParsedGraph) -> HashSet<UniqueId> {
        self.common
            .depends_on
            .intersection(&graph.macros)
            .map(|s| s.to_string())
            .collect()
    }
}

pub fn generate_node_hash_map(nodes: Vec<WrapperNode>) -> HashMap<UniqueId, WrapperNode> {
    nodes
        .iter()
        .map(|node| (node.unique_id().clone(), node.to_owned()))
        .collect()
}

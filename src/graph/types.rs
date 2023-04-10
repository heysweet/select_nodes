use crate::dbt_node_selector::ModelNode;

/// https://github.com/dbt-labs/dbt-core/blob/a203fe866ad3e969e7de9cc24ddbbef1934aa7d0/core/dbt/contracts/graph/nodes.py

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceDefinition {}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ManifestSqlNode {}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ManifestNode {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Exposure {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Metric {}

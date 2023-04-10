use std::collections::HashMap;

use crate::dbt_node_selector::*;

use super::node::WrapperNode;

trait FqnCompareExt {
    fn same_fqn(&self, other: &Self) -> bool;
}

pub trait ComparableContents {
    fn same_content(&self, other: &Self) -> bool;
}

pub trait GraphNodeExt {
    fn fqn(&self) -> Vec<String>;
}

pub trait ParsedNodeExt: GraphNodeExt {
    fn same_content(&self, other: &Self) -> bool;
    fn fqn(&self) -> Vec<String>;
    fn depends_on(&self) -> Vec<String>;
    fn node_config(&self) -> HashMap<String, String>;

    fn same_body(&self, other: &Self) -> bool;
    fn same_config(&self, other: &Self) -> bool;
    fn same_persisted_description(&self, other: &Self) -> bool;
    fn same_database_representation(&self, other: &Self) -> bool;
    fn same_contract(&self, other: &Self) -> bool;
}

pub trait CompiledNodeExt: ParsedNodeExt {
    fn fqn(&self) -> Vec<String>;
    fn depends_on(&self) -> Vec<String>;
    fn node_config(&self) -> HashMap<String, String>;
}

macro_rules! impl_GraphNodeExt { 
    ($T:ident) => {
        impl GraphNodeExt for $T {
            fn fqn(&self) -> Vec<String> { self.fqn }
        }
    }
}

macro_rules! impl_ParsedNodeExt { 
    ($T:ident) => {
        impl GraphNodeExt for $T {
            fn fqn(&self) -> Vec<String> { self.fqn }
        }
    }
}

macro_rules! impl_CompiledNodeExt { 
    ($T:ident) => {
        impl GraphNodeExt for $T {
            fn fqn(&self) -> Vec<String> { self.fqn }
        }

        impl ParsedNodeExt for $T {

        }

        impl CompiledNodeExt for $T {
            fn depends_on(&self) -> Vec<String> { self.depends_on }
            fn node_config(&self) -> HashMap<String, String> { self.node_config };
        }
    }
}

impl_GraphNodeExt!(TestNode);
impl_GraphNodeExt!(SnapshotNode);
impl_GraphNodeExt!(OperationNode);
impl_GraphNodeExt!(GroupNode);
impl_GraphNodeExt!(SourceNode);
impl_GraphNodeExt!(ExposureNode);
impl_GraphNodeExt!(MetricNode);

impl_ParsedNodeExt!(SeedNode);

impl_CompiledNodeExt!(AnalysisNode);
impl_CompiledNodeExt!(ModelNode);
impl_CompiledNodeExt!(RpcNode);
impl_CompiledNodeExt!(SqlOperationNode);

/// Or HookNode
impl ComparableContents for OperationNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<T> FqnCompareExt for T
where
    T: GraphNodeExt,
{
    fn same_fqn(&self, other: &Self) -> bool {
        let fqn_a = self.fqn();
        let fqn_b = other.fqn();
        &fqn_a.len() == &fqn_b.len() && fqn_a.iter().zip(&fqn_b).all(|(&a, &b)| a == b)
    }
}

impl WrapperNode {
    pub fn same_contents(&self, other: Option<&WrapperNode>) -> bool {
        match other {
            None => false,
            Some(other) => self.resource_type.same_content(&other.resource_type),
        }
    }
}

impl NodeType {
    pub fn same_content(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeType::Model(this), NodeType::Model(other)) => this.same_content(other),
            (NodeType::Analysis(this), NodeType::Analysis(other)) => this.same_content(other),
            (NodeType::Test(this), NodeType::Test(other)) => this.same_content(other),
            (NodeType::Snapshot(this), NodeType::Snapshot(other)) => this.same_content(other),
            (NodeType::Operation(this), NodeType::Operation(other)) => this.same_content(other),
            (NodeType::Seed(this), NodeType::Seed(other)) => this.same_content(other),
            (NodeType::Rpc(this), NodeType::Rpc(other)) => this.same_content(other),
            (NodeType::SqlOperation(this), NodeType::SqlOperation(other)) => {
                this.same_content(other)
            }
            (NodeType::Source(this), NodeType::Source(other)) => this.same_content(other),
            (NodeType::Exposure(this), NodeType::Exposure(other)) => this.same_content(other),
            (NodeType::Metric(this), NodeType::Metric(other)) => this.same_content(other),
            (NodeType::Group(this), NodeType::Group(other)) => this.same_content(other),
            (NodeType::Doc(this), NodeType::Doc(other)) => this.same_content(other),
            (NodeType::Macro(this), NodeType::Macro(other)) => this.same_content(other),
            (_, _) => false,
        }
    }
}

impl ComparableContents for WrapperNode {
    fn same_content(&self, other: &Self) -> bool {
        self.resource_type.same_content(&other.resource_type)
    }
}

impl<T> ComparableContents for T
where
    T: ParsedNodeExt,
{
    fn same_content(&self, other: &Self) -> bool {
        self.same_body(other)
            && self.same_config(other)
            && self.same_persisted_description(other)
            && self.same_fqn(other)
            && self.same_database_representation(other)
            && self.same_contract(other)
    }
}

impl ComparableContents for SourceNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl ComparableContents for ExposureNode {
    fn same_content(&self, other: &Self) -> bool {
        self.same_fqn(other) && todo!()
        //   && self.same_exposure_type(other)
        //   && self.same_owner(other)
        //   && self.same_maturity(other)
        //   && self.same_url(other)
        //   && self.same_description(other)
        //   && self.same_label(other)
        //   && self.same_depends_on(other)
        //   && self.same_config(other)
    }
}

impl ComparableContents for MetricNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl ComparableContents for DocNode {
    /// The only thing that makes one doc different from another with the
    /// same name/package is its content
    fn same_content(&self, other: &Self) -> bool {
        self.block_contents == other.block_contents
    }
}

impl ComparableContents for MacroNode {
    /// The only thing that makes one macro different from another with the
    /// same name/package is its content
    fn same_content(&self, other: &Self) -> bool {
        self.macro_sql == other.macro_sql
    }
}

use crate::dbt_node_selector::{
    CompiledNode, DocNode, ExposureNode, GraphNode, MacroNode, MetricNode, ModelNode, NodeType,
    ParsedNode, SourceNode,
};

use super::node::BaseNode;

trait HasFqn {
    fn fqn(&self) -> Vec<String>;
}
impl HasFqn for GraphNode {
    fn fqn(&self) -> Vec<String> {
        self.fqn
    }
}
impl HasFqn for ParsedNode {
    fn fqn(&self) -> Vec<String> {
        self.fqn
    }
}
impl HasFqn for CompiledNode {
    fn fqn(&self) -> Vec<String> {
        self.fqn
    }
}
impl HasFqn for SourceNode {
    fn fqn(&self) -> Vec<String> {
        self.fqn
    }
}
impl HasFqn for ExposureNode {
    fn fqn(&self) -> Vec<String> {
        self.fqn
    }
}
impl HasFqn for MetricNode {
    fn fqn(&self) -> Vec<String> {
        self.fqn
    }
}
impl HasFqn for ModelNode {
    fn fqn(&self) -> Vec<String> {
        self.fqn
    }
}

trait FqnCompareExt {
    fn same_fqn(&self, other: &Self) -> bool;
}

impl<T> FqnCompareExt for T
where
    T: HasFqn,
{
    fn same_fqn(&self, other: &Self) -> bool {
        self.fqn() == other.fqn()
    }
}

impl BaseNode {
    pub fn same_contents(&self, other: Option<&BaseNode>) -> bool {
        match other {
            None => false,
            Some(other) => self.resource_type.same_contents(&other.resource_type),
        }
    }
}

impl NodeType {
    pub fn same_contents(&self, other: &Self) -> bool {
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

pub trait CompareNode {
    fn same_content(&self, other: &Self) -> bool;
}

impl CompareNode for ModelNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl CompareNode for CompiledNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl CompareNode for GraphNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl ParsedNode {
    pub fn same_body(&self, other: &Self) -> bool {
        unimplemented!()
    }

    pub fn same_config(&self, other: &Self) -> bool {
        unimplemented!()
    }

    pub fn same_persisted_description(&self, other: &Self) -> bool {
        unimplemented!()
    }

    pub fn same_fqn(&self, other: &Self) -> bool {
        unimplemented!()
    }

    pub fn same_database_representation(&self, other: &Self) -> bool {
        unimplemented!()
    }

    pub fn same_contract(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl CompareNode for ParsedNode {
    fn same_content(&self, other: &Self) -> bool {
        self.same_body(other)
            && self.same_config(other)
            && self.same_persisted_description(other)
            && self.same_fqn(other)
            && self.same_database_representation(other)
            && self.same_contract(other)
    }
}

impl CompareNode for SourceNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl CompareNode for ExposureNode {
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

impl CompareNode for MetricNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl CompareNode for DocNode {
    /// The only thing that makes one doc different from another with the
    /// same name/package is its content
    fn same_content(&self, other: &Self) -> bool {
        self.block_contents == other.block_contents
    }
}

impl CompareNode for MacroNode {
    /// The only thing that makes one macro different from another with the
    /// same name/package is its content
    fn same_content(&self, other: &Self) -> bool {
        self.macro_sql == other.macro_sql
    }
}

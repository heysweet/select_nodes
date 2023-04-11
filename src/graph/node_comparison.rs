use crate::dbt_node_selector::*;

use super::node::{WrapperNode, WrapperNodeExt};

trait FqnCompareExt {
    fn same_fqn(&self, other: &Self) -> bool;
}

pub trait ComparableContents {
    fn same_content(&self, other: &Self) -> bool;
}

pub trait GraphNodeExt: ComparableContents {
    fn fqn(&self) -> Vec<String>;
}

pub trait ParsedNodeExt: GraphNodeExt {
    fn same_body(&self, other: &Self) -> bool;
    fn same_persisted_description(&self, other: &Self) -> bool;
    fn same_database_representation(&self, other: &Self) -> bool;
    fn same_contract(&self, other: &Self) -> bool;
}

pub trait CompiledNodeExt: ParsedNodeExt {}

macro_rules! impl_GraphNodeExt {
    ($T:ident) => {
        impl GraphNodeExt for $T {
            fn fqn(&self) -> Vec<String> {
                self.fqn.clone()
            }
        }
    };
}

macro_rules! impl_ParsedNodeExt {
    ($T:ident) => {
        impl GraphNodeExt for $T {
            fn fqn(&self) -> Vec<String> {
                self.fqn.clone()
            }
        }
    };
}

macro_rules! impl_CompiledNodeExt {
    ($T:ident) => {
        impl GraphNodeExt for $T {
            fn fqn(&self) -> Vec<String> {
                self.fqn.clone()
            }
        }

        impl ParsedNodeExt for $T {
            fn same_body(&self, other: &Self) -> bool {
                self.raw_code == other.raw_code
            }

            fn same_persisted_description(&self, other: &Self) -> bool {
                todo!()
            }

            fn same_database_representation(&self, other: &Self) -> bool {
                todo!()
            }

            fn same_contract(&self, other: &Self) -> bool {
                todo!()
            }
        }

        // impl CompiledNodeExt for $T {}
    };
}

impl SeedNode {
    fn same_seed(&self, other: &Self) -> bool {
        todo!()
    }
}

impl ParsedNodeExt for SeedNode {
    fn same_body(&self, other: &Self) -> bool {
        self.same_seed(other)
    }

    fn same_persisted_description(&self, other: &Self) -> bool {
        todo!()
    }

    fn same_database_representation(&self, other: &Self) -> bool {
        todo!()
    }

    fn same_contract(&self, other: &Self) -> bool {
        todo!()
    }
}

impl ParsedNodeExt for ModelNode {
    fn same_body(&self, other: &Self) -> bool {
        todo!()
    }

    fn same_persisted_description(&self, other: &Self) -> bool {
        todo!()
    }

    fn same_database_representation(&self, other: &Self) -> bool {
        todo!()
    }

    fn same_contract(&self, other: &Self) -> bool {
        todo!()
    }
}

// BaseNode (GroupNode);
// BaseNode (MacroNode);
// BaseNode (DocNode);

impl_GraphNodeExt!(TestNode);
impl_GraphNodeExt!(SnapshotNode);
impl_GraphNodeExt!(OperationNode);
impl_GraphNodeExt!(SourceNode);
impl_GraphNodeExt!(ExposureNode);
impl_GraphNodeExt!(MetricNode);

impl_ParsedNodeExt!(SeedNode);

// Compiled with custom impl
impl_ParsedNodeExt!(ModelNode);

impl_CompiledNodeExt!(AnalysisNode);
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
        &fqn_a.len() == &fqn_b.len() && fqn_a.iter().zip(&fqn_b).all(|(a, b)| a.eq(b))
    }
}

impl WrapperNode {
    pub fn same_contents(&self, other: Option<&WrapperNode>) -> bool {
        match other {
            None => false,
            Some(other) => self.resource_type().same_content(&other.resource_type()),
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
        self.resource_type().same_content(&other.resource_type())
    }
}

impl<T> ComparableContents for T
where
    T: ParsedNodeExt,
{
    fn same_content(&self, other: &Self) -> bool {
        self.same_body(other)
            && self.same_persisted_description(other)
            && self.same_fqn(other)
            && self.same_database_representation(other)
            && self.same_contract(other)
    }
}

impl ComparableContents for TestNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl ComparableContents for SnapshotNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
    }
}

impl ComparableContents for GroupNode {
    fn same_content(&self, other: &Self) -> bool {
        todo!()
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

impl PartialEq for AccessType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialEq for ModelNode {
    fn eq(&self, other: &Self) -> bool {
        self.fqn == other.fqn && self.depends_on == other.depends_on && self.access == other.access
    }
}

// TODO: Consider just comparing hashes?
impl PartialEq for AnalysisNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for TestNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
impl PartialEq for SnapshotNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for OperationNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for SeedNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for RpcNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for SqlOperationNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for SourceNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for ExposureNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for MetricNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for GroupNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for DocNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for MacroNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Model(l0), Self::Model(r0)) => l0 == r0,
            (Self::Analysis(l0), Self::Analysis(r0)) => l0 == r0,
            (Self::Test(l0), Self::Test(r0)) => l0 == r0,
            (Self::Snapshot(l0), Self::Snapshot(r0)) => l0 == r0,
            (Self::Operation(l0), Self::Operation(r0)) => l0 == r0,
            (Self::Seed(l0), Self::Seed(r0)) => l0 == r0,
            (Self::Rpc(l0), Self::Rpc(r0)) => l0 == r0,
            (Self::SqlOperation(l0), Self::SqlOperation(r0)) => l0 == r0,
            (Self::Source(l0), Self::Source(r0)) => l0 == r0,
            (Self::Exposure(l0), Self::Exposure(r0)) => l0 == r0,
            (Self::Metric(l0), Self::Metric(r0)) => l0 == r0,
            (Self::Group(l0), Self::Group(r0)) => l0 == r0,
            (Self::Doc(l0), Self::Doc(r0)) => l0 == r0,
            (Self::Macro(l0), Self::Macro(r0)) => l0 == r0,
            _ => false,
        }
    }
}

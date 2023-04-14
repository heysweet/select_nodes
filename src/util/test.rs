/// Test utilities

use std::collections::HashSet;

use crate::{dbt_node_selector::*, graph::node::NodeTypeKey};

/// Often times, we have a Vec and we really don't care about the order
pub fn vec_to_set(vec: Vec<impl Into<String>>) -> HashSet<String> {
    vec.into_iter().map(|s| s.into()).collect()
}

#[macro_export]
macro_rules! assert_ok {
    ($expression:expr) => {
        match $expression {
            Ok(result) => result,
            _ => panic!("expected `Ok` but got `Err`"),
        }
    };
}

#[macro_export]
macro_rules! assert_err {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
        }
    }
}

/// Asserts option is Some and returns HashSet<String>
pub fn assert_vec_to_set(vec: Option<Vec<impl Into<String>>>) -> HashSet<String> {
    assert!(vec.is_some());
    vec.unwrap().into_iter().map(|s| s.into()).collect()
}

/// Asserts option is Some and returns HashSet<String>
pub fn assert_hashset(vec: Option<HashSet<String>>) -> HashSet<String> {
    assert!(vec.is_some());
    vec.unwrap()
}

pub fn get_model_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Model(ModelNode {
        fqn: vec!["MODEL".to_owned(), "FQN".to_owned(), unique_id.into()],
        depends_on: vec!["DEP_A".to_string()],
        raw_code: "RAW_MODEL".to_string(),
        access: AccessType::Private,
    })
}

pub fn get_analysis_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Analysis(AnalysisNode {
        fqn: vec!["ANALYSIS".to_owned(), "FQN".to_owned(), unique_id.into()],
        depends_on: vec!["DEP_A".to_string()],
        raw_code: "RAW_ANALYSIS".to_owned(),
    })
}

pub fn get_test_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Test(TestNode {
        fqn: vec!["TEST".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_TEST".to_owned(),
    })
}

pub fn get_snapshot_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Snapshot(SnapshotNode {
        fqn: vec!["SNAPSHOT".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_SNAPSHOT".to_owned(),
    })
}

pub fn get_operation_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Operation(OperationNode {
        fqn: vec!["OPERATION".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_OPERATION".to_owned(),
    })
}

pub fn get_seed_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Seed(SeedNode {
        depends_on: vec!["DEP_A".to_owned()],
        fqn: vec!["SEED".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_SEED".to_owned(),
    })
}

pub fn get_rpc_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Rpc(RpcNode {
        depends_on: vec!["DEP_A".to_owned()],
        fqn: vec!["RPC".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_RPC".to_owned(),
    })
}

pub fn get_sql_operation_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::SqlOperation(SqlOperationNode {
        depends_on: vec!["DEP_A".to_owned()],
        fqn: vec!["SQL_OPERATION".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_SQL_OPERATION".to_owned(),
    })
}

pub fn get_doc_node(_unique_id: impl Into<String>) -> NodeType {
    NodeType::Doc(DocNode {
        block_contents: "RAW_DOC".to_owned(),
    })
}

pub fn get_source_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Source(SourceNode {
        fqn: vec!["SOURCE".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_SOURCE".to_owned(),
    })
}

pub fn get_macro_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Macro(MacroNode {
        macro_sql: "RAW_MACRO".to_owned(),
        depends_on: vec!["DEP_A".to_owned()],
    })
}

pub fn get_exposure_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Exposure(ExposureNode {
        fqn: vec!["EXPOSURE".to_owned(), "FQN".to_owned(), unique_id.into()],
        raw_code: "RAW_EXPOSURE".to_owned(),
    })
}

pub fn get_metric_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Metric(MetricNode {
        fqn: vec!["METRIC".to_owned(), "FQN".to_owned(), unique_id.into()],
    })
}

pub fn get_group_node(unique_id: impl Into<String>) -> NodeType {
    NodeType::Group(GroupNode {
        fqn: vec!["GROUP".to_owned(), "FQN".to_owned(), unique_id.into()],
    })
}

pub fn get_resource_type(unique_id: impl Into<String>) -> NodeType {
    let unique_id = unique_id.into();
    let node_type_key = unique_id.split("_").next().unwrap_or_else(|| "model");

    let node_type_key = NodeTypeKey::from_key(node_type_key).unwrap_or_else(|_| NodeTypeKey::Model);

    match node_type_key {
        NodeTypeKey::Model => get_model_node(unique_id),
        NodeTypeKey::Analysis => get_analysis_node(unique_id),
        NodeTypeKey::Test => get_test_node(unique_id),
        NodeTypeKey::Snapshot => get_snapshot_node(unique_id),
        NodeTypeKey::Operation => get_operation_node(unique_id),
        NodeTypeKey::Seed => get_seed_node(unique_id),
        NodeTypeKey::Rpc => get_rpc_node(unique_id),
        NodeTypeKey::SqlOperation => get_sql_operation_node(unique_id),
        NodeTypeKey::Doc => get_doc_node(unique_id),
        NodeTypeKey::Source => get_source_node(unique_id),
        NodeTypeKey::Macro => get_macro_node(unique_id),
        NodeTypeKey::Exposure => get_exposure_node(unique_id),
        NodeTypeKey::Metric => get_metric_node(unique_id),
        NodeTypeKey::Group => get_group_node(unique_id),
    }
}

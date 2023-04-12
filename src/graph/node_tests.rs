#[cfg(test)]
mod node_tests {
    use super::super::*;
    use crate::util::test::*;

    #[test]
    fn selector_create_error_display_no_matching_resource_type() {
        let output =
            SelectorCreateError::NoMatchingResourceType("TEST_VALUE".to_string()).to_string();

        assert!(output == "Invalid resource_type 'TEST_VALUE'".to_string())
    }

    #[test]
    fn selector_create_error_display_missing_field() {
        let output = SelectorCreateError::MissingField("TEST_VALUE".to_string()).to_string();

        assert!(output == "Missing required field 'TEST_VALUE'".to_string())
    }

    #[test]
    fn node_type_key_to_str() {
        assert!(NodeTypeKey::Model.key() == "model");
        assert!(NodeTypeKey::Analysis.key() == "analysis");
        assert!(NodeTypeKey::Test.key() == "test");
        assert!(NodeTypeKey::Snapshot.key() == "snapshot");
        assert!(NodeTypeKey::Operation.key() == "operation");
        assert!(NodeTypeKey::Seed.key() == "seed");
        assert!(NodeTypeKey::Rpc.key() == "rpc");
        assert!(NodeTypeKey::SqlOperation.key() == "sql operation");
        assert!(NodeTypeKey::Doc.key() == "doc");
        assert!(NodeTypeKey::Source.key() == "source");
        assert!(NodeTypeKey::Macro.key() == "macro");
        assert!(NodeTypeKey::Exposure.key() == "exposure");
        assert!(NodeTypeKey::Metric.key() == "metric");
        assert!(NodeTypeKey::Group.key() == "group");
    }

    #[test]
    fn node_type_str_to_type() {
        assert!(NodeTypeKey::from_key("model").unwrap() == NodeTypeKey::Model);
        assert!(NodeTypeKey::from_key("analysis").unwrap() == NodeTypeKey::Analysis);
        assert!(NodeTypeKey::from_key("test").unwrap() == NodeTypeKey::Test);
        assert!(NodeTypeKey::from_key("snapshot").unwrap() == NodeTypeKey::Snapshot);
        assert!(NodeTypeKey::from_key("operation").unwrap() == NodeTypeKey::Operation);
        assert!(NodeTypeKey::from_key("seed").unwrap() == NodeTypeKey::Seed);
        assert!(NodeTypeKey::from_key("rpc").unwrap() == NodeTypeKey::Rpc);
        assert!(NodeTypeKey::from_key("sql operation").unwrap() == NodeTypeKey::SqlOperation);
        assert!(NodeTypeKey::from_key("doc").unwrap() == NodeTypeKey::Doc);
        assert!(NodeTypeKey::from_key("source").unwrap() == NodeTypeKey::Source);
        assert!(NodeTypeKey::from_key("macro").unwrap() == NodeTypeKey::Macro);
        assert!(NodeTypeKey::from_key("exposure").unwrap() == NodeTypeKey::Exposure);
        assert!(NodeTypeKey::from_key("metric").unwrap() == NodeTypeKey::Metric);
        assert!(NodeTypeKey::from_key("group").unwrap() == NodeTypeKey::Group);
    }

    #[test]
    fn node_type_str_to_type_error() {
        assert!(NodeTypeKey::from_key("").is_err());
        assert!(NodeTypeKey::from_key(" ").is_err());
        assert!(NodeTypeKey::from_key("MODEL").is_err());
        assert!(NodeTypeKey::from_key("Model").is_err());
        assert!(NodeTypeKey::from_key("models").is_err());
        assert!(NodeTypeKey::from_key("test2").is_err());
        assert!(NodeTypeKey::from_key(" model").is_err());
        assert!(NodeTypeKey::from_key("model ").is_err());
        assert!(NodeTypeKey::from_key("mode").is_err());
        assert!(NodeTypeKey::from_key("model_node").is_err());
        assert!(NodeTypeKey::from_key("odel").is_err());
        assert!(NodeTypeKey::from_key("foo").is_err());
        assert!(NodeTypeKey::from_key("sqloperation").is_err());
        assert!(NodeTypeKey::from_key("sql_operation").is_err());
        assert!(NodeTypeKey::from_key("resource").is_err());
        assert!(NodeTypeKey::from_key("documentation").is_err());
    }

    #[test]
    fn node_type_key_from_node_type() {
        assert!(NodeTypeKey::from_node_type(&get_model_node()) == NodeTypeKey::Model);
        assert!(NodeTypeKey::from_node_type(&get_analysis_node()) == NodeTypeKey::Analysis);
        assert!(NodeTypeKey::from_node_type(&get_test_node()) == NodeTypeKey::Test);
        assert!(NodeTypeKey::from_node_type(&get_snapshot_node()) == NodeTypeKey::Snapshot);
        assert!(NodeTypeKey::from_node_type(&get_operation_node()) == NodeTypeKey::Operation);
        assert!(NodeTypeKey::from_node_type(&get_seed_node()) == NodeTypeKey::Seed);
        assert!(NodeTypeKey::from_node_type(&get_rpc_node()) == NodeTypeKey::Rpc);
        assert!(
            NodeTypeKey::from_node_type(&get_sql_operation_node()) == NodeTypeKey::SqlOperation
        );
        assert!(NodeTypeKey::from_node_type(&get_doc_node()) == NodeTypeKey::Doc);
        assert!(NodeTypeKey::from_node_type(&get_source_node()) == NodeTypeKey::Source);
        assert!(NodeTypeKey::from_node_type(&get_macro_node()) == NodeTypeKey::Macro);
        assert!(NodeTypeKey::from_node_type(&get_exposure_node()) == NodeTypeKey::Exposure);
        assert!(NodeTypeKey::from_node_type(&get_metric_node()) == NodeTypeKey::Metric);
        assert!(NodeTypeKey::from_node_type(&get_group_node()) == NodeTypeKey::Group);
    }

    fn get_wrapper_model(resource_type: NodeType) -> WrapperNode {
        WrapperNode::new(
            "UNIQUE_ID",
            vec!["DEP_A".to_owned()],
            "NAME",
            "PACKAGE_NAME",
            "PATH",
            "ORIGINAL_FILE_PATH",
            resource_type,
            vec!["TAG_A".to_owned(), "TAG_B".to_owned()],
            vec![("TEST_KEY".to_owned(), "VALUE".to_owned())],
        )
        .unwrap()
    }

    #[test]
    fn wrapper_node_fqn() {
        assert_eq!(
            get_wrapper_model(get_model_node()).fqn(),
            Some(vec!["MODEL".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_analysis_node()).fqn(),
            Some(vec!["ANALYSIS".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_test_node()).fqn(),
            Some(vec!["TEST".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_snapshot_node()).fqn(),
            Some(vec!["SNAPSHOT".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_operation_node()).fqn(),
            Some(vec!["OPERATION".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_rpc_node()).fqn(),
            Some(vec!["RPC".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_sql_operation_node()).fqn(),
            Some(vec!["SQL_OPERATION".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_source_node()).fqn(),
            Some(vec!["SOURCE".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_exposure_node()).fqn(),
            Some(vec!["EXPOSURE".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_metric_node()).fqn(),
            Some(vec!["METRIC".to_owned(), "FQN".to_owned()])
        );
        assert_eq!(
            get_wrapper_model(get_group_node()).fqn(),
            Some(vec!["GROUP".to_owned(), "FQN".to_owned()])
        );
    }

    #[test]
    fn wrapper_node_no_fqn() {
        assert_eq!(get_wrapper_model(get_doc_node()).fqn(), None);
        assert_eq!(get_wrapper_model(get_macro_node()).fqn(), None);
    }
}

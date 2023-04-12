/// test/unit/test_graph_selector_spec.py

#[cfg(test)]
mod select_nodes_tests {
    use std::path::Path;

    use super::super::*;

    use MethodName::*;

    #[test]
    fn raw_parse_simple() {
        let raw = "asdf";
        let result = SelectionCriteria::from_single_raw_spec(raw);
        let result = result.unwrap();

        let expected_method_arguments: Vec<String> = vec![];

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), FQN.key());
        assert_eq!(result.method_arguments, expected_method_arguments);
        assert_eq!(result.value, raw);
        assert_eq!(result.childrens_parents, false);
        assert_eq!(result.children, false);
        assert_eq!(result.parents, false);
        assert!(result.parents_depth.is_none());
        assert!(result.children_depth.is_none());
    }

    #[test]
    fn raw_parse_simple_infer_path() {
        let raw = format!("{:?}", Path::new("asdf").join("*"));
        let result = SelectionCriteria::from_single_raw_spec(&raw);
        let result = result.unwrap();

        let expected_method_arguments: Vec<String> = vec![];

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), Path.key());
        assert_eq!(result.method_arguments, expected_method_arguments);
        assert_eq!(result.value, raw);
        assert_eq!(result.childrens_parents, false);
        assert_eq!(result.children, false);
        assert_eq!(result.parents, false);
        assert!(result.parents_depth.is_none());
        assert!(result.children_depth.is_none());
    }

    #[test]
    fn raw_parse_simple_infer_path_modified() {
        let binding = Path::new("asdf").join("*");
        let expected_value = binding.to_str().unwrap();
        let raw = format!("@{}", expected_value);
        let result = SelectionCriteria::from_single_raw_spec(&raw);
        let result = result.unwrap();

        let expected_method_arguments: Vec<String> = vec![];

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), Path.key());
        assert_eq!(result.method_arguments, expected_method_arguments);
        assert_eq!(result.value, expected_value);
        assert_eq!(result.childrens_parents, true);
        assert_eq!(result.children, false);
        assert_eq!(result.parents, false);
        assert!(result.parents_depth.is_none());
        assert!(result.children_depth.is_none());
    }

    #[test]
    fn raw_parse_simple_infer_fqn_parents() {
        let raw = "+asdf";
        let result = SelectionCriteria::from_single_raw_spec(raw);
        let result = result.unwrap();

        let expected_method_arguments: Vec<String> = vec![];

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), FQN.key());
        assert_eq!(result.method_arguments, expected_method_arguments);
        assert_eq!(result.value, "asdf");
        assert_eq!(result.childrens_parents, false);
        assert_eq!(result.children, false);
        assert_eq!(result.parents, true);
        assert!(result.parents_depth.is_none());
        assert!(result.children_depth.is_none());
    }

    #[test]
    fn raw_parse_simple_infer_fqn_children() {
        let raw = "asdf+";
        let result = SelectionCriteria::from_single_raw_spec(raw);
        let result = result.unwrap();

        let expected_method_arguments: Vec<String> = vec![];

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), FQN.key());
        assert_eq!(result.method_arguments, expected_method_arguments);
        assert_eq!(result.value, "asdf");
        assert_eq!(result.childrens_parents, false);
        assert_eq!(result.children, true);
        assert_eq!(result.parents, false);
        assert!(result.parents_depth.is_none());
        assert!(result.children_depth.is_none());
    }

    #[test]
    fn raw_parse_complex() {
        let raw = "2+config.arg.secondarg:argument_value+4";
        let result = SelectionCriteria::from_single_raw_spec(raw);
        let result = result.unwrap();

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), Config.key());
        assert_eq!(result.method_arguments, vec!["arg", "secondarg"]);
        assert_eq!(result.value, "argument_value");
        assert_eq!(result.childrens_parents, false);
        assert_eq!(result.children, true);
        assert_eq!(result.parents, true);
        assert_eq!(result.parents_depth.unwrap(), 2);
        assert_eq!(result.children_depth.unwrap(), 4);
    }

    /// you can have an empty method name (defaults to FQN/path) and you can have
    /// an empty value, so you can also have this...
    #[test]
    fn raw_parse_weird() {
        let raw = "";
        let result = SelectionCriteria::from_single_raw_spec(raw);
        let result = result.unwrap();

        let expected_method_arguments: Vec<String> = vec![];

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), FQN.key());
        assert_eq!(result.method_arguments, expected_method_arguments);
        assert_eq!(result.value, "");
        assert_eq!(result.childrens_parents, false);
        assert_eq!(result.children, false);
        assert_eq!(result.parents, false);
        assert!(result.parents_depth.is_none());
        assert!(result.children_depth.is_none());
    }

    #[test]
    fn raw_raw_parse_invalid() {
        let invalid1 = SelectionCriteria::from_single_raw_spec("invalid_method:something");
        let invalid2 = SelectionCriteria::from_single_raw_spec("@foo+");
        let invalid3 = SelectionCriteria::from_single_raw_spec("foo\n");
        let invalid4 = SelectionCriteria::from_single_raw_spec("f\noo");
        let invalid5 = SelectionCriteria::from_single_raw_spec("\nfoo");
        assert!(invalid1.is_err());
        assert!(invalid2.is_err());
        assert!(invalid3.is_err());
        assert!(invalid4.is_err());
        assert!(invalid5.is_err());
    }

    #[test]
    fn intersection() {
        let fqn_a = SelectionCriteria::from_single_raw_spec("fqn:model_a").unwrap();
        let fqn_b = SelectionCriteria::from_single_raw_spec("fqn:model_b").unwrap();
        todo!()
    }

    #[test]
    fn difference() {
        let fqn_a = SelectionCriteria::from_single_raw_spec("fqn:model_a").unwrap();
        let fqn_b = SelectionCriteria::from_single_raw_spec("fqn:model_b").unwrap();
        todo!()
    }

    #[test]
    fn union() {
        let fqn_a = SelectionCriteria::from_single_raw_spec("fqn:model_a").unwrap();
        let fqn_b = SelectionCriteria::from_single_raw_spec("fqn:model_b").unwrap();
        let fqn_c = SelectionCriteria::from_single_raw_spec("fqn:model_c").unwrap();
        todo!()
    }
}

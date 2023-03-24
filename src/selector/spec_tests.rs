/// test/unit/test_graph_selector_spec.py

#[cfg(test)]
mod select_nodes_tests {
    use std::path::Path;

    use super::super::*;

    use MethodName::*;

    #[test]
    fn it_parses_base_raw_input() {
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
    fn it_parses_simple_infer_path() {
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
    fn it_parses_simple_infer_path_modified() {
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
    fn it_parses_simple_infer_fqn_parents() {
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
    fn it_parses_simple_infer_fqn_children() {
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
    fn it_parses_complex_input() {
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
}

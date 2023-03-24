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

        let expected_method = FQN;
        let expected_method_arguments: Vec<String> = vec![];

        assert_eq!(result.raw, raw);
        assert_eq!(result.method.key(), expected_method.key());
        assert_eq!(result.method_arguments, expected_method_arguments);
        assert_eq!(result.value, raw);
        assert_eq!(result.childrens_parents, false);
        assert_eq!(result.children, false);
        assert_eq!(result.parents, false);
        assert_eq!(result.parents_depth, 0);
        assert_eq!(result.children_depth, 0);
    }

    #[test]
    fn it_parses_simple_infer_path() {
        let raw = Path::new("asdf").join("*");
        assert_eq!("test", "TODO");
    }

    #[test]
    fn it_parses_simple_infer_path_modified() {
        let raw = format!("@{}", Path::new("asdf").join("*").to_str().unwrap());
        assert_eq!("test", "TODO");
    }

    #[test]
    fn it_parses_simple_infer_fqn_parents() {
        let raw = "+asdf";
        assert_eq!("test", "TODO");
    }
}

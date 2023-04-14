/// test/unit/test_graph_selector_spec.py

#[cfg(test)]
mod select_nodes_tests {
    use std::path::Path;

    use crate::util::test::vec_to_set;

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
    fn intersection_simple() {
        let components = vec![
            vec_to_set(vec!["model_a", "model_b", "model_c"]),
            vec_to_set(vec!["model_c", "model_d"])
        ];
        let operation = SetOperation::Intersection;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["model_c"]);
        assert_eq!(expected, combined)
    }

    #[test]
    fn intersection_empty() {
        let empty: Vec<String> = vec![];
        let components = vec![
            vec_to_set(empty.clone())
        ];
        let operation = SetOperation::Intersection;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(empty);
        assert_eq!(expected, combined)
    }

    #[test]
    fn intersection_empty_op() {
        let empty: Vec<String> = vec![];
        let components = vec![
            vec_to_set(vec!["model_a", "model_b", "model_c"]),
            vec_to_set(empty.clone())
        ];
        let operation = SetOperation::Intersection;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(empty);
        assert_eq!(expected, combined)
    }

    #[test]
    fn intersection_multi() {
        let components = vec![
            vec_to_set(vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"]),
            vec_to_set(vec!["b", "d", "f", "h", "j", "l"]),
            vec_to_set(vec!["a", "b", "c", "d", "e", "f"]),
            vec_to_set(vec!["f", "g", "h", "i", "j", "k"]),
            vec_to_set(vec!["f"]),
        ];
        let operation = SetOperation::Intersection;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["f"]);
        assert_eq!(expected, combined)
    }

    #[test]
    fn difference_simple() {
        let components = vec![
            vec_to_set(vec!["model_a", "model_b", "model_c"]),
            vec_to_set(vec!["model_c", "model_d"])
        ];
        let operation = SetOperation::Difference;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["model_a", "model_b"]);
        assert_eq!(expected, combined)
    }

    #[test]
    fn difference_empty() {
        let empty: Vec<String> = vec![];
        let components = vec![
            vec_to_set(empty.clone())
        ];
        let operation = SetOperation::Difference;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(empty);
        assert_eq!(expected, combined)
    }

    #[test]
    fn difference_empty_op() {
        let empty: Vec<String> = vec![];
        let components = vec![
            vec_to_set(vec!["model_a", "model_b", "model_c"]),
            vec_to_set(empty.clone())
        ];
        let operation = SetOperation::Difference;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["model_a", "model_b", "model_c"]);
        assert_eq!(expected, combined)
    }

    #[test]
    fn difference_multi() {
        let components = vec![
            vec_to_set(vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"]),
            vec_to_set(vec!["a"]),
            vec_to_set(vec!["b", "c"]),
            vec_to_set(vec!["d", "e"]),
            vec_to_set(vec!["a", "b", "d"]),
            vec_to_set(vec!["g", "h"]),
            vec_to_set(vec!["i"]),
            vec_to_set(vec!["a", "d", "k"]),
            vec_to_set(vec!["j"]),
        ];
        let operation = SetOperation::Difference;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["f"]);
        assert_eq!(expected, combined)
    }

    #[test]
    fn union_simple() {
        let components = vec![
            vec_to_set(vec!["model_a", "model_b", "model_c"]),
            vec_to_set(vec!["model_c", "model_d"])
        ];
        let operation = SetOperation::Union;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["model_a", "model_b", "model_c", "model_d"]);
        assert_eq!(expected, combined)
    }

    #[test]
    fn union_multi() {
        let components = vec![
            vec_to_set(vec!["a"]),
            vec_to_set(vec!["a", "b"]),
            vec_to_set(vec!["c"]),
            vec_to_set(vec!["b", "d", "e", "f"]),
            vec_to_set(vec!["g", "h"]),
            vec_to_set(vec!["i"]),
            vec_to_set(vec!["j"]),
            vec_to_set(vec!["k"]),
        ];
        let operation = SetOperation::Union;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"]);
        assert_eq!(expected, combined)
    }

    #[test]
    fn union_empty() {
        let empty: Vec<String> = vec![];
        let components = vec![
            vec_to_set(empty.clone()),
        ];
        let operation = SetOperation::Union;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(empty);
        assert_eq!(expected, combined)
    }

    #[test]
    fn union_empty_op() {
        let empty: Vec<String> = vec![];
        let components = vec![
            vec_to_set(vec!["a", "b"]),
            vec_to_set(empty.clone()),
        ];
        let operation = SetOperation::Union;

        let combined = operation.combine_selections(&components);

        let expected = vec_to_set(vec!["a", "b"]);
        assert_eq!(expected, combined)
    }
}

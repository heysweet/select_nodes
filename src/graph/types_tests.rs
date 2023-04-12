#[cfg(test)]
mod types_tests {
    use std::collections::HashSet;

    use super::super::*;

    // fn get_test_nodes() -> HashSet<UniqueId, WrapperNode> {
    //     let mut test_nodes = HashSet::new();
    // }

    #[test]
    fn macro_node_same_content_same_sql() {
        let a = MacroNode {
            macro_sql: "test".to_string(),
            depends_on: vec![],
        };
        let b = MacroNode {
            macro_sql: "test".to_string(),
            depends_on: vec![],
        };

        assert!(a.same_content(&b));
    }

    #[test]
    fn macro_node_same_content_substring_sql() {
        let a = MacroNode {
            macro_sql: "test".to_string(),
            depends_on: vec![],
        };
        let b = MacroNode {
            macro_sql: "tes".to_string(),
            depends_on: vec![],
        };

        assert!(!a.same_content(&b));
    }

    #[test]
    fn macro_node_same_content_superstring_sql() {
        let a = MacroNode {
            macro_sql: "test".to_string(),
            depends_on: vec![],
        };
        let b = MacroNode {
            macro_sql: "test2".to_string(),
            depends_on: vec![],
        };

        assert!(!a.same_content(&b));
    }

    #[test]
    fn macro_node_same_content_no_match_sql() {
        let a = MacroNode {
            macro_sql: "hello".to_string(),
            depends_on: vec![],
        };
        let b = MacroNode {
            macro_sql: "world".to_string(),
            depends_on: vec![],
        };

        assert!(!a.same_content(&b));
    }

    #[test]
    fn doc_node_same_content_matching_contents() {
        let a = DocNode {
            block_contents: "test".to_string(),
        };
        let b = DocNode {
            block_contents: "test".to_string(),
        };

        assert!(a.same_content(&b));
    }

    #[test]
    fn doc_node_same_content_substring() {
        let a = DocNode {
            block_contents: "tes".to_string(),
        };
        let b = DocNode {
            block_contents: "test".to_string(),
        };

        assert!(!a.same_content(&b));
    }

    #[test]
    fn doc_node_same_content_superstring() {
        let a = DocNode {
            block_contents: "test2".to_string(),
        };
        let b = DocNode {
            block_contents: "test".to_string(),
        };

        assert!(!a.same_content(&b));
    }

    #[test]
    fn doc_node_same_content_disjoint() {
        let a = DocNode {
            block_contents: "hello".to_string(),
        };
        let b = DocNode {
            block_contents: "world".to_string(),
        };

        assert!(!a.same_content(&b));
    }

    #[test]
    fn access_type_eq() {
        assert!(AccessType::Private.eq(&AccessType::Private));
        assert!(!AccessType::Private.eq(&AccessType::Protected));
        assert!(!AccessType::Private.eq(&AccessType::Public));

        assert!(!AccessType::Protected.eq(&AccessType::Private));
        assert!(AccessType::Protected.eq(&AccessType::Protected));
        assert!(!AccessType::Protected.eq(&AccessType::Public));

        assert!(!AccessType::Public.eq(&AccessType::Private));
        assert!(!AccessType::Public.eq(&AccessType::Protected));
        assert!(AccessType::Public.eq(&AccessType::Public));
    }

    #[test]
    fn model_node_eq_true() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };

        assert!(a.eq(&b));
    }

    #[test]
    fn model_node_eq_fqn_shorter() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };

        assert!(!a.eq(&b));
    }

    #[test]
    fn model_node_eq_fqn_longer() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };

        assert!(!a.eq(&b));
    }

    #[test]
    fn model_node_eq_fqn_different() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "d".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };

        assert!(!a.eq(&b));
    }

    #[test]
    fn model_node_eq_depends_on_different() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "another".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };

        assert!(!a.eq(&b));
    }

    #[test]
    fn model_node_eq_depends_on_shorter() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };

        assert!(!a.eq(&b));
    }

    #[test]
    fn model_node_eq_depends_on_longer() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string(), "2".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };

        assert!(!a.eq(&b));
    }

    #[test]
    fn model_node_eq_raw_code_different() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "different contents".to_string(),
            access: AccessType::Public,
        };

        assert!(!a.eq(&b));
    }

    #[test]
    fn model_node_eq_raw_access_different() {
        let a = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Public,
        };
        let b = ModelNode {
            fqn: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            depends_on: vec!["other".to_string(), "thing".to_string()],
            raw_code: "model contents".to_string(),
            access: AccessType::Protected,
        };

        assert!(!a.eq(&b));
    }

    // #[test]
    // fn node_type_different_types() {

    // }
}

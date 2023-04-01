#[cfg(test)]
mod select_nodes_tests {
    use crate::graph::node::GraphNode;
    use interface::NodeType::*;

    use super::super::*;

    /// Any node with an id: "PREFIX_" will have the node "PREFIX" as
    /// a direct parent.
    /// All nodes will have a parent where the number of chars in the
    /// ID is a parent id ("and" is 3 chars, so a parent is "3")
    /// All test nodes have all ancestors as direct parents
    fn get_test_edges() -> Vec<Edge> {
        vec![
            Edge {
                unique_id: "a".to_string(),
                parents: vec![],
            },
            Edge {
                unique_id: "b".to_string(),
                parents: vec![],
            },
            Edge {
                unique_id: "c".to_string(),
                parents: vec![],
            },
            Edge {
                unique_id: "d".to_string(),
                parents: vec![],
            },
            Edge {
                unique_id: "e".to_string(),
                parents: vec![],
            },
            Edge {
                unique_id: "f".to_string(),
                parents: vec![],
            },
            Edge {
                unique_id: "an".to_string(),
                parents: vec!["a".to_string()],
            },
            Edge {
                unique_id: "and".to_string(),
                parents: vec!["an".to_string()],
            },
            Edge {
                unique_id: "andr".to_string(),
                parents: vec!["and".to_string()],
            },
            Edge {
                unique_id: "andre".to_string(),
                parents: vec!["andr".to_string()],
            },
            Edge {
                unique_id: "andrew".to_string(),
                parents: vec!["andre".to_string()],
            },
            Edge {
                unique_id: "andrew_test".to_string(),
                /// All ancestors are parents
                parents: ["a", "an", "and", "andr", "andre", "andrew"]
                    .map(|s| s.to_string())
                    .into(),
            },
            Edge {
                unique_id: "ab".to_string(),
                parents: vec!["a".to_string()],
            },
            Edge {
                unique_id: "abb".to_string(),
                parents: vec!["ab".to_string()],
            },
            Edge {
                unique_id: "abs".to_string(),
                parents: vec!["ab".to_string()],
            },
            Edge {
                unique_id: "abby".to_string(),
                parents: vec!["abb".to_string()],
            },
            Edge {
                unique_id: "abby_test".to_string(),
                /// All ancestors are parents
                parents: ["a", "abb", "abby"].map(|s| s.to_string()).into(),
            },
            Edge {
                unique_id: "ba".to_string(),
                parents: vec!["b".to_string()],
            },
            Edge {
                unique_id: "bar".to_string(),
                parents: vec!["ba".to_string()],
            },
            Edge {
                unique_id: "bat".to_string(),
                parents: vec!["ba".to_string()],
            },
            Edge {
                unique_id: "ca".to_string(),
                parents: vec!["c".to_string()],
            },
            Edge {
                unique_id: "car".to_string(),
                parents: vec!["ca".to_string(), "bar".to_string()],
            },
        ]
    }

    fn make_node(id: impl Into<String>) -> Node {
        let id: String = id.into();
        let package_name = format!("pkg_{}", &id);
        let resource_type = if id.len() == 1 { "source" } else { "model" };
        Node {
            unique_id: id.clone(),
            name: format!("name_{}", &id),
            resource_type: resource_type.to_string(),
            package_name: package_name.clone(),
            path: format!("path_{}", &id),
            original_file_path: format!("opath_{}", &id),
            fqn: [package_name.to_string(), format!("{}", &id)].to_vec(),
        }
    }

    fn get_test_nodes() -> Vec<Node> {
        get_test_edges()
            .iter()
            .map(|edge| make_node(&edge.unique_id))
            .collect()
    }

    fn get_test_node_selector(nodes: Vec<Node>, edges: Vec<Edge>) -> NodeSelector {
        let node_selector = NodeSelector::from(nodes, edges);
        node_selector.unwrap()
    }

    fn get_expected(ids: Vec<&str>) -> Vec<String> {
        ids.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let node_selector = get_test_node_selector(vec![], get_test_edges());

        let result = node_selector.select_and_filter(
            None,
            &"my_model".to_string(),
            &ResourceTypeFilter::All,
        );

        let expected = get_expected(vec![]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_returns_the_matching_node() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result =
            node_selector.select_and_filter(None, &"andr".to_string(), &ResourceTypeFilter::All);

        let expected = get_expected(vec!["andr"]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result =
            node_selector.select_and_filter(None, &"andrew".to_string(), &ResourceTypeFilter::All);

        let expected = get_expected(vec!["andrew"]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result =
            node_selector.select_and_filter(None, &"spoon".to_string(), &ResourceTypeFilter::All);

        let expected = get_expected(vec![]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_should_select_singular_parent() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result =
            node_selector.select_and_filter(None, &"spoon".to_string(), &ResourceTypeFilter::All);

        let expected = get_expected(vec![]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_should_select_all_parents() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result =
            node_selector.select_and_filter(None, &"1+and".to_string(), &ResourceTypeFilter::All);

        let expected = get_expected(vec!["an", "and"]);
        assert_eq!(result.unwrap(), expected);
    }
}

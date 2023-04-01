#[cfg(test)]
mod select_nodes_tests {
    use crate::graph::node::GraphNode;
    use interface::NodeType::*;

    use super::super::*;

    /// Any node with an id: "PREFIX_" will have the node "PREFIX" as
    /// a direct parent.
    /// All nodes will have a parent where the number of chars in the
    /// ID is a parent id ("and" is 3 chars, so a parent is "3")
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
        Node {
            unique_id: id.clone(),
            name: format!("name_{}", &id),
            resource_type: "model".to_string(),
            package_name: package_name.clone(),
            path: format!("path_{}", &id),
            original_file_path: format!("opath_{}", &id),
            fqn: [package_name.to_string(), format!("{}", &id)].to_vec(),
        }
    }

    fn get_test_nodes() -> Vec<Node> {
        get_test_edges().iter().map(|edge| make_node(&edge.unique_id)).collect()
    }

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let nodes: Vec<Node> = vec![];
        let edges = get_test_edges();
        let node_selector = NodeSelector::from(nodes, edges);
        let node_selector = node_selector.unwrap();
        let result = node_selector.select_and_filter(
            None,
            &"my_model".to_string(),
            &ResourceTypeFilter::All,
        );

        let expected: std::slice::Iter<UniqueId> = vec![].iter();
        assert!(matches!(result, Ok(expected)));
    }

    #[test]
    fn it_returns_the_matching_node() {
        let nodes = get_test_nodes();
        let edges = get_test_edges();
        let node_selector = NodeSelector::from(nodes, edges);
        let node_selector = node_selector.unwrap();
        let result =
            node_selector.select_and_filter(None, &"andr".to_string(), &ResourceTypeFilter::All);

        let expected = vec!["andr".to_string()];

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.eq(&expected));
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let nodes = get_test_nodes();
        let edges = get_test_edges();
        let node_selector = NodeSelector::from(nodes, edges);
        let node_selector = node_selector.unwrap();
        let result = node_selector.select_and_filter(
            None,
            &"my_model".to_string(),
            &ResourceTypeFilter::All,
        );

        let expected: Vec<GraphNode> = vec![GraphNode {
            fqn: ["test".to_string()].to_vec(),
            unique_id: "test".to_string(),
            resource_type: Analysis,
            name: "name".to_string(),
            package_name: "pkg".to_string(),
            path: "path".to_string(),
            original_file_path: "opath".to_string(),
        }];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let nodes = get_test_nodes();
        let edges = get_test_edges();
        let node_selector = NodeSelector::from(nodes, edges);
        let node_selector = node_selector.unwrap();
        let result = node_selector.select_and_filter(
            None,
            &"other_model".to_string(),
            &ResourceTypeFilter::All,
        );

        let expected: Vec<GraphNode> = vec![];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }
}

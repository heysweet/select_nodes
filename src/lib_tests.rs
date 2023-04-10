#[cfg(test)]
mod select_nodes_tests {
    use crate::{dbt_node_selector::NodeType, graph::node::NodeTypeKey};

    use super::super::*;
    use crate::dbt_node_selector::*;

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

    fn make_resource_type(package_name: String, id: String, resource_type: &str) -> Result<NodeType, SelectorCreateError> {
        let resource_type = NodeTypeKey::from_key(resource_type)?;
        let fqn: Vec<String> = [package_name, format!("{}", id)].to_vec();
        match resource_type {
            NodeTypeKey::Model => {
                Ok(NodeType::Model(ModelNode{access: AccessType::Public, depends_on: vec![], fqn: vec![]}))
            },
            NodeTypeKey::Analysis => Ok(NodeType::Analysis(CompiledNode{depends_on: vec![], fqn: vec![]})),
            NodeTypeKey::Test => Ok(NodeType::Test(GraphNode{fqn: vec![]})),
            NodeTypeKey::Snapshot => Ok(NodeType::Snapshot(GraphNode{fqn: vec![]})),
            NodeTypeKey::Operation => Ok(NodeType::Operation(GraphNode{fqn: vec![]})),
            NodeTypeKey::Seed => Ok(NodeType::Seed(BaseNode{depends_on: vec![], fqn: vec![]})),
            NodeTypeKey::Rpc => Ok(NodeType::Rpc(CompiledNode{depends_on: vec![], fqn: vec![]})),
            NodeTypeKey::SqlOperation => Ok(NodeType::SqlOperation(CompiledNode{depends_on: vec![], fqn: vec![]})),
            NodeTypeKey::Doc => Ok(NodeType::Doc(DocNode{block_contents: "".to_string()})),
            NodeTypeKey::Source => Ok(NodeType::Source(SourceNode{fqn: vec![]})),
            NodeTypeKey::Macro => Ok(NodeType::Macro(MacroNode{macro_sql: "".to_string(), depends_on: vec![]})),
            NodeTypeKey::Exposure => Ok(NodeType::Exposure(ExposureNode{fqn: vec![]})),
            NodeTypeKey::Metric => Ok(NodeType::Metric(MetricNode{fqn: vec![]})),
            NodeTypeKey::Group => Ok(NodeType::Group(GraphNode{fqn: vec![]})),
        }
    }

    fn make_node(id: impl Into<String>) -> Result<Node, SelectorCreateError> {
        let id: String = id.into();
        let package_name = format!("pkg_{}", &id);
        let resource_type = if (&id).len() == 1 { "source" } else { "model" };
        let node_type = make_resource_type(package_name.clone(), id.clone(), resource_type)?;
        Ok(crate::dbt_node_selector::Node {
            unique_id: id.clone(),
            depends_on: vec!["test".to_string()],
            name: format!("name_{}", &id),
            package_name: package_name.clone(),
            path: format!("path_{}", &id),
            original_file_path: format!("opath_{}", &id),
            node_type,
        })
    }

    fn get_test_nodes() -> Vec<Node> {
        get_test_edges()
            .iter()
            .filter_map(|edge| make_node(&edge.unique_id).ok())
            .collect()
    }

    fn get_test_node_selector(nodes: Vec<Node>, edges: Vec<Edge>) -> crate::selector::node_selector::NodeSelector {
        let node_selector = crate::selector::node_selector::NodeSelector::from(&nodes, &edges, None);
        node_selector.unwrap()
    }

    fn get_expected(ids: Vec<&str>) -> Vec<String> {
        ids.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let node_selector = get_test_node_selector(vec![], get_test_edges());

        let result = node_selector._select("my_model".to_string());

        let expected = get_expected(vec![]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_returns_the_matching_node() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result = node_selector._select("andr".to_string());

        let expected = get_expected(vec!["andr"]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result = node_selector._select("andrew".to_string());

        let expected = get_expected(vec!["andrew"]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result = node_selector._select("spoon".to_string());

        let expected = get_expected(vec![]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn it_should_select_all_parents() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result = node_selector._select("1+and".to_string());

        let mut expected = get_expected(vec!["an", "and"]);
        let mut result = result.unwrap();
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_select_all_children() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result = node_selector._select("and+1".to_string());

        let mut expected = get_expected(vec!["and", "andr", "andrew_test"]);
        let mut result = result.unwrap();
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_select_all_ancestors() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result = node_selector._select("+and".to_string());

        let mut expected = get_expected(vec!["a", "an", "and"]);
        let mut result = result.unwrap();
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_select_all_descendants() {
        let node_selector = get_test_node_selector(get_test_nodes(), get_test_edges());

        let result = node_selector._select("and+".to_string());

        let mut expected = get_expected(vec!["and", "andr", "andre", "andrew", "andrew_test"]);
        let mut result = result.unwrap();
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}

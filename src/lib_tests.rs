#[cfg(test)]
mod select_nodes_tests {
    use std::collections::{HashMap, HashSet};

    use crate::graph::node::GraphNode;
    use interface::NodeType::*;

    use super::super::*;

    fn add_parents(
        mut parents_map: HashMap<String, HashSet<String>>,
        child_id: &str,
        parent_ids: Vec<&str>,
    ) -> HashMap<String, HashSet<String>> {
        let child_id: String = child_id.into();
        let parent_ids: Vec<UniqueId> = parent_ids
            .iter()
            .map(|id| {
                let str: String = id.to_string();
                str
            })
            .collect();
        match parents_map.get(&child_id) {
            Some(map) => {
                let mut map = map.clone();
                map.extend(parent_ids);
                parents_map.insert(child_id.to_string(), map);
                parents_map
            }
            None => {
                let mut map = HashSet::new();
                map.extend(parent_ids);
                parents_map.insert(child_id.to_string(), map);
                parents_map
            }
        }
    }

    fn get_test_edges() -> Vec<Edge> {
        vec![
            Edge {
                unique_id: "id_a".to_string(),
                parents: vec![],
            },
            Edge {
                unique_id: "id_b".to_string(),
                parents: vec!["id_a".to_string()],
            },
            Edge {
                unique_id: "id_c".to_string(),
                parents: vec!["id_a".to_string(), "id_b".to_string()],
            },
        ]
    }

    fn get_test_nodes() -> Vec<Node> {
        vec![
            Node {
                unique_id: "id_a".to_string(),
                name: "name_a".to_string(),
                resource_type: "model".to_string(),
                package_name: "pkg_a".to_string(),
                path: "path_a".to_string(),
                original_file_path: "opath_a".to_string(),
                fqn: ["id_a".to_string()].to_vec(),
            },
            Node {
                unique_id: "id_b".to_string(),
                name: "name_b".to_string(),
                resource_type: "analysis".to_string(),
                package_name: "pkg_b".to_string(),
                path: "path_b".to_string(),
                original_file_path: "opath_b".to_string(),
                fqn: ["id_b".to_string()].to_vec(),
            },
            Node {
                unique_id: "id_c".to_string(),
                name: "name_c".to_string(),
                resource_type: "test".to_string(),
                package_name: "pkg_c".to_string(),
                path: "path_c".to_string(),
                original_file_path: "opath_c".to_string(),
                fqn: ["id_c".to_string()].to_vec(),
            },
        ]
    }

    fn test_parents_map() -> HashMap<String, HashSet<String>> {
        let parents_map = HashMap::new();
        let parents_map = add_parents(parents_map, "A", vec!["B"]);
        add_parents(parents_map, "B", vec!["C"])
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
        let result = node_selector.select_and_filter(
            None,
            &"id_a".to_string(),
            &ResourceTypeFilter::All,
        );

        let expected = vec!["id_a".to_string()];

        assert!(result.is_ok());
        assert!(result.unwrap().eq(&expected));
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

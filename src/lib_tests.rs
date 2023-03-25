#[cfg(test)]
mod select_nodes_tests {
    use std::collections::HashSet;

    use crate::graph::node::{Node, NodeParseError};

    use super::super::*;

    use graph::node::NodeType::*;

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

    fn get_test_node_map() -> Result<HashMap<String, ParsedNode>, NodeParseError> {
        let nodes = vec![
            Node::new("A", "model"),
            Node::new("B", "analysis"),
            Node::new("C", "test"),
        ];
        let nodes: Result<Vec<ParsedNode>, NodeParseError> =
            nodes.iter().map(|node| node.parse()).collect();

        Ok(generate_node_hash_map(nodes?))
    }

    fn test_parents_map() -> HashMap<String, HashSet<String>> {
        let parents_map = HashMap::new();
        let parents_map = add_parents(parents_map, "A", vec!["B"]);
        add_parents(parents_map, "B", vec!["C"])
    }

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let graph = ParsedGraph::new(get_test_node_map().unwrap(), test_parents_map());

        let result = select_nodes(graph, "my_model");

        let expected: std::slice::Iter<UniqueId> = vec![].iter();
        assert!(matches!(result, Ok(expected)));
    }

    #[test]
    fn it_returns_the_matching_node() {
        let graph = ParsedGraph::new(get_test_node_map().unwrap(), test_parents_map());

        let result = select_nodes(graph, "my_model");

        // TODO:
        // assert!(does_my_thing_match(result, expected));

        assert!(result.is_ok());
        let result: Vec<String> = result.unwrap();

        let expected = vec!["test".to_string()];

        assert!(result.eq(&expected));
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let graph = ParsedGraph::new(get_test_node_map().unwrap(), test_parents_map());

        let result = select_nodes(graph, "my_model");

        let expected: Vec<ParsedNode> = vec![ParsedNode {
            unique_id: "test".to_string(),
            resource_type: Analysis,
        }];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let graph = ParsedGraph::new(get_test_node_map().unwrap(), test_parents_map());

        let result = select_nodes(graph, "other_model");

        let expected: Vec<ParsedNode> = vec![];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }
}

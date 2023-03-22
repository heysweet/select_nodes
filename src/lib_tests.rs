

#[cfg(test)]
mod select_nodes_tests {
    use std::collections::HashSet;

    use crate::graph::node::Node;

    use super::super::{*};

    fn add_parents(
        mut parents_map: HashMap<String, HashSet<String>>,
        child_id: impl Into<String>,
        parent_ids: Vec<impl Into<String>>
    ) -> HashMap<String, HashSet<String>> {
        let child_id: String = child_id.into();
        let parent_ids = parent_ids.iter().map(|id| {
            let str: String = (*id).into();
            str
        });
        match parents_map.get(&child_id) {
            Some(map) => {
                let mut map = map.clone();
                map.extend(parent_ids);
                parents_map.insert(child_id.to_string(), map);
                parents_map
            },
            None => {
                let mut map = HashSet::new();
                map.extend(parent_ids);
                parents_map.insert(child_id.to_string(), map);
                parents_map
            }
        }
    }

    fn test_node_map() -> HashMap<String, ParsedNode> {
        let nodes = vec![
            Node::new("A"),
            Node::new("B"),
            Node::new("C"),
        ];
        let nodes = nodes.iter().map(|node| node.parse());

        generate_node_hash_map(nodes)
    }

    fn get_string_iter(strings: Vec<&str>) -> std::vec::IntoIter<String> {
        let strings: Vec<String> = strings.iter().map(|str| str.to_string()).collect();
        strings.into_iter()
    }

    fn test_parents_map() -> HashMap<String, HashSet<String>> {
        let parents_map = HashMap::new();
        let parents_map = add_parents(parents_map, "A", vec!["B"]);
        add_parents(parents_map, "B", vec!["C"])
    }

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let graph = ParsedGraph::new(test_node_map(), test_parents_map());

        let result = select_nodes(graph, "my_model");
        
        let expected: std::slice::Iter<UniqueId> = vec![].iter();
        assert!(matches!(result, Ok(expected)));
    }

    #[test]
    fn it_returns_the_matching_node() {
        let graph = ParsedGraph::new(test_node_map(), test_parents_map());

        let result = select_nodes(graph, "my_model");
        
        let expected = vec!["test".to_string()];
        let expected = get_string_iter(["test"].to_vec());
        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result.eq(expected));
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let graph = ParsedGraph::new(test_node_map(), test_parents_map());
        
        let result = select_nodes(graph, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: "test".to_string() }];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let graph = ParsedGraph::new(test_node_map(), test_parents_map());
        
        let result = select_nodes(graph, "other_model");
        
        let expected: Vec<ParsedNode> = vec![];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }
}

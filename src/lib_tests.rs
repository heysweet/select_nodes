

#[cfg(test)]
mod select_nodes_tests {
    use std::collections::HashSet;

    use crate::graph::node::Node;

    use super::super::{*};

    fn add_parents(
        mut parents_map: &HashMap<String, HashSet<String>>,
        child_id: &str,
        parent_ids: Vec<&str>
    ) -> HashMap<String, HashSet<String>> {
        match parents_map.get(child_id) {
            Some(map) => {
                let to_add = parent_ids.iter().map(|id| (*id).to_string());
                map.extend(to_add);
                *parents_map
            },
            None => {
                let map = HashSet::new();
                let to_add = parent_ids.iter().map(|id| (*id).to_string());
                map.extend(to_add);
                parents_map.insert(child_id.to_string(), map);
                *parents_map
            }
        }
    }

    lazy_static! {
        static ref NODE_MAP: HashMap<String, ParsedNode> = {
            let node_map = HashMap::new();
            let add_node = |id: &str| node_map.insert(id.to_string(), Node::new(id).parse());

            add_node("A");
            add_node("B");
            add_node("C");

            node_map
        };

        static ref PARENTS_MAP: HashMap<String, HashSet<String>> = {
            let mut parents_map = HashMap::new();
            let mut parents_map = add_parents(&parents_map, "A", vec!["B"]);
            add_parents(&parents_map, "B", vec!["C"])
        };
    }

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let nodes: Vec<Node> = vec![];
        let graph = ParsedGraph::new(NODE_MAP, PARENTS_MAP);
        let nodes = generate_node_hash_map(vec![]);
        let result = select_nodes(graph, "my_model");
        
        let expected: Result<std::slice::Iter<UniqueId>, String> = Ok(vec![].iter());
        let does_match = matches!(result, expected);
        assert!(does_match);
    }

    #[test]
    fn it_returns_the_matching_node() {
        let nodes = vec![Node::new("test")];
        let graph = todo!();
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: "test".to_string() }];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let nodes = vec![Node::new("test")];
        let graph = todo!();
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: todo!() }];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let nodes = vec![Node::new("test")];
        let graph = todo!();
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, "other_model");
        
        let expected: Vec<ParsedNode> = vec![];
        let does_match = matches!(result, Ok(expected));
        assert!(does_match);
    }
}

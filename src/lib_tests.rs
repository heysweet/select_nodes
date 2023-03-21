

#[cfg(test)]
mod select_nodes_tests {
    use std::assert_matches::assert_matches;

    use crate::graph::node::Node;

    use super::super::{*};

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let nodes = vec![];
        let graph = todo!();
        let nodes = generate_node_hash_map(vec![]);
        let result = select_nodes(graph, "my_model");
        
        let expected: Result<std::slice::Iter<UniqueId>, String> = Ok(vec![].iter());
        assert_matches!(result, expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_the_matching_node() {
        let nodes = vec![Node::new("test")];
        let graph = todo!();
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: todo!() }];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let nodes = vec![Node::new("test")];
        let graph = todo!();
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: todo!() }];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let nodes = vec![Node::new("test")];
        let graph = todo!();
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, "other_model");
        
        let expected: Vec<ParsedNode> = vec![];
        assert_eq!(result, expected);
    }
}

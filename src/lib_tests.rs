

#[cfg(test)]
mod select_nodes_tests {
    use crate::graph::node::Node;

    use super::super::{*};

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let graph = todo!();
        let nodes: Vec<Node> = vec![];
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let nodes = generate_node_hash_map(vec![]);
        let result = select_nodes(graph, nodes, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: todo!() }];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_the_matching_node() {
        let graph = todo!();
        let nodes = vec![Node::new("test")];
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, node_map, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: todo!() }];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let graph = todo!();
        let nodes = vec![Node::new("test")];
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, node_map, "my_model");
        
        let expected: Vec<ParsedNode> = vec![ParsedNode{ unique_id: todo!() }];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let graph = todo!();
        let nodes = vec![Node::new("test")];
        let nodes: Vec<&ParsedNode> = nodes.iter().map(|n| &n.parse()).collect();
        let node_map = generate_node_hash_map(nodes);
        let result = select_nodes(graph, node_map, "other_model");
        
        let expected: Vec<ParsedNode> = vec![];
        assert_eq!(result, expected);
    }
}

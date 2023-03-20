

#[cfg(test)]
mod select_nodes_tests {
    use super::super::{*};

    #[test]
    fn it_handles_an_empty_collection_of_nodes() {
        let graph = todo!();
        let nodes = vec![];
        let result = select_nodes(graph, nodes, "my_model".to_string());
        
        let expected: Vec<&Node> = vec![&Node{}];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_the_matching_node() {
        let graph = todo!();
        let nodes = vec![&Node{}];
        let result = select_nodes(graph, nodes, "my_model".to_string());
        
        let expected: Vec<&Node> = vec![&Node{}];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_filters_to_the_matching_node() {
        let graph = todo!();
        let nodes = vec![&Node{}, &Node{}];
        let result = select_nodes(graph, nodes, "my_model".to_string());
        
        let expected: Vec<&Node> = vec![&Node{}];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_no_node_if_not_matching() {
        let graph = todo!();
        let nodes = vec![&Node{}];
        let result = select_nodes(graph, nodes, "other_model".to_string());
        
        let expected: Vec<&Node> = vec![];
        assert_eq!(result, expected);
    }
}

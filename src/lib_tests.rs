
#[cfg(test)]
mod select_nodes_tests {
    use super::super::{*};

    #[test]
    fn it_works() {
        let nodes = vec![Node{}];
        let result = select_nodes(nodes, "my_model".to_string());
        
        assert_eq!(result, vec![Node{}]);
    }
}

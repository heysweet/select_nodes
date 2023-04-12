#[cfg(test)]
mod parsed_graph_tests {
    use crate::util::test::*;

    use super::super::*;

    fn get_node(unique_id: &UniqueId) -> WrapperNode {
        WrapperNode::new(
            unique_id,
            vec![],
            format!("{} name", unique_id),
            "PKG",
            "PATH",
            "OPATH",
            get_resource_type(unique_id),
            vec![],
            vec![],
        )
        .unwrap()
    }

    fn new_node(
        node_map: &mut HashMap<UniqueId, WrapperNode>,
        parents_map: &mut HashMap<UniqueId, HashSet<UniqueId>>,
        unique_id: &str,
        parents: Vec<&str>,
    ) -> () {
        let unique_id = unique_id.to_string();
        node_map.insert(unique_id.clone(), get_node(&unique_id));
        parents_map.insert(
            unique_id,
            parents.into_iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Graphs are built from "hello", "howdy", "hero", and "test".
    ///
    /// Each prefix of these words (h, he, hel, hell, hello) is its own node.
    ///
    /// Each node also has a `seed_n` as a parent, where `n` is the length of the prefix
    ///
    /// All non-seed nodes also have `origin` as a parent.
    ///
    /// Node type is `model` by default, but if the `UniqueId` starts with another
    /// node type followed by an "_" (seed_, source_, exposure_, metric_, macro_),
    /// then we will use that NodeType.
    fn get_test_data() -> (
        HashMap<UniqueId, WrapperNode>,
        HashMap<UniqueId, HashSet<UniqueId>>,
    ) {
        let mut node_map: HashMap<UniqueId, WrapperNode> = HashMap::default();
        let mut parents_map: HashMap<UniqueId, HashSet<UniqueId>> = HashMap::default();

        new_node(&mut node_map, &mut parents_map, "origin", vec![]);
        new_node(&mut node_map, &mut parents_map, "seed_1", vec![]);
        new_node(&mut node_map, &mut parents_map, "seed_2", vec![]);
        new_node(&mut node_map, &mut parents_map, "seed_3", vec![]);
        new_node(&mut node_map, &mut parents_map, "seed_4", vec![]);
        new_node(&mut node_map, &mut parents_map, "seed_5", vec![]);
        new_node(
            &mut node_map,
            &mut parents_map,
            "h",
            vec!["seed_1", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "he",
            vec!["h", "seed_2", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hel",
            vec!["he", "seed_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hell",
            vec!["hel", "seed_4", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hello",
            vec!["hell", "seed_5", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "ho",
            vec!["h", "seed_2", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "how",
            vec!["ho", "seed_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "howd",
            vec!["how", "seed_4", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "howdy",
            vec!["howd", "seed_5", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "her",
            vec!["he", "seed_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hero",
            vec!["her", "seed_4", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "t",
            vec!["seed_1", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "te",
            vec!["t", "seed_2", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "tes",
            vec!["te", "seed_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "test",
            vec!["tes", "seed_4", "origin"],
        );

        (node_map, parents_map)
    }

    #[test]
    fn from_parents_get_children() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = graph.children_map.get("he").expect("Got no children");
        let parents = graph.parents_map.get("he").expect("Got no parents");

        assert_eq!(children, &vec_to_set(vec!["her", "hel"]));
        assert_eq!(parents, &vec_to_set(vec!["h", "seed_2", "origin"]));
    }

    #[test]
    fn from_children_get_parents() {
        // Since we're just passing in the parents map as a children map, we're inverting the graph here.
        // The test has the same assumptions as `from_parents_get_children` because of this inversion
        let (node_map, children_map) = get_test_data();
        let graph = ParsedGraph::from_children(node_map, children_map);

        let children = graph.children_map.get("he").expect("Got no children");
        let parents = graph.parents_map.get("he").expect("Got no parents");

        assert_eq!(parents, &vec_to_set(vec!["her", "hel"]));
        assert_eq!(children, &vec_to_set(vec!["h", "seed_2", "origin"]));
    }
}

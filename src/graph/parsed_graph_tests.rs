#[cfg(test)]
mod parsed_graph_tests {
    use crate::{assert_ok, util::test::*};

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

    #[test]
    fn select_children_zero() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_children(&vec_to_set(vec!["he"]), &Some(0)));
        let expected: Vec<String> = vec![];
        let expected = vec_to_set(expected);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_children_one() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_children(&vec_to_set(vec!["he"]), &Some(1)));
        let expected = vec_to_set(vec!["her", "hel"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_children_two() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_children(&vec_to_set(vec!["he"]), &Some(2)));
        let expected = vec_to_set(vec!["her", "hel", "hero", "hell"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_parents_zero() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_parents(&vec_to_set(vec!["he"]), &Some(0)));
        let expected: Vec<String> = vec![];
        let expected = vec_to_set(expected);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_parents_one() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_parents(&vec_to_set(vec!["he"]), &Some(1)));
        let expected = vec_to_set(vec!["h", "seed_2", "origin"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_parents_two() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_parents(&vec_to_set(vec!["he"]), &Some(2)));
        let expected = vec_to_set(vec!["h", "seed_2", "origin", "seed_1"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_children_none() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_children(&vec_to_set(vec!["h"]), &None));
        let expected = vec_to_set(vec![
            "he", "her", "hero", "ho", "how", "howd", "howdy", "hel", "hell", "hello",
        ]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_children_no_children() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_children(&vec_to_set(vec!["hello"]), &None));
        let expected: Vec<String> = vec![];
        let expected = vec_to_set(expected);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_parents_no_parents() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_parents(&vec_to_set(vec!["origin"]), &None));
        let expected: Vec<String> = vec![];
        let expected = vec_to_set(expected);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_parents_none() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_parents(&vec_to_set(vec!["hello"]), &None));
        let expected = vec_to_set(vec![
            "origin", "h", "he", "hel", "hell", "seed_1", "seed_2", "seed_3", "seed_4", "seed_5",
        ]);

        assert_eq!(expected, children);
    }

    #[test]
    fn and_select_parents_zero() {
        // Should select the included set and 0 parents
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.and_select_parents(&vec_to_set(vec!["howd"]), &Some(0)));
        let expected = vec_to_set(vec!["howd"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn and_select_parents_one() {
        // Should select the included set and 1 depth of parents
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.and_select_parents(&vec_to_set(vec!["howd"]), &Some(1)));
        let expected = vec_to_set(vec!["howd", "how", "seed_4", "origin"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn and_select_parents_none() {
        // Should select the included set and all parents
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.and_select_parents(&vec_to_set(vec!["how"]), &None));
        let expected = vec_to_set(vec![
            "how", "ho", "h", "origin", "seed_3", "seed_2", "seed_1",
        ]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_childrens_parents_simple() {
        // Should select the included set and all parents
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_childrens_parents(&vec_to_set(vec!["how"])));
        let expected = vec_to_set(vec![
            "origin", "h", "ho", "how", "howd", "howdy", "seed_1", "seed_2", "seed_3", "seed_4", "seed_5",
        ]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_childrens_parents_complex() {
        // Should select the included set and all parents
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_childrens_parents(&vec_to_set(vec!["h"])));
        let expected = vec_to_set(vec![
            "origin", "h", "he", "her", "hero", "hel", "hell", "hello", "ho", "how", "howd",
            "howdy", "seed_1", "seed_2", "seed_3", "seed_4", "seed_5",
        ]);

        assert_eq!(expected, children);
    }
}

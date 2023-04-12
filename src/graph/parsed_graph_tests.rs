#[cfg(test)]
mod parsed_graph_tests {
    use crate::{assert_ok, util::test::*, dbt_node_selector::NodeType};

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
    /// Each node also has a `source_n` as a parent, where `n` is the length of the prefix
    ///
    /// All non-source nodes also have `origin` as a parent.
    ///
    /// Node type is `model` by default, but if the `UniqueId` starts with another
    /// node type followed by an "_" (source_, exposure_, metric_, macro_),
    /// then we will use that NodeType.
    fn get_test_data() -> (
        HashMap<UniqueId, WrapperNode>,
        HashMap<UniqueId, HashSet<UniqueId>>,
    ) {
        let mut node_map: HashMap<UniqueId, WrapperNode> = HashMap::default();
        let mut parents_map: HashMap<UniqueId, HashSet<UniqueId>> = HashMap::default();

        new_node(&mut node_map, &mut parents_map, "origin", vec![]);
        new_node(&mut node_map, &mut parents_map, "source_1", vec![]);
        new_node(&mut node_map, &mut parents_map, "source_2", vec![]);
        new_node(&mut node_map, &mut parents_map, "source_3", vec![]);
        new_node(&mut node_map, &mut parents_map, "source_4", vec![]);
        new_node(&mut node_map, &mut parents_map, "source_5", vec![]);
        // Macro, exposure, metric are just chained by number
        new_node(&mut node_map, &mut parents_map, "macro_1", vec!["origin"]);
        new_node(&mut node_map, &mut parents_map, "macro_2", vec!["macro_1"]);
        new_node(&mut node_map, &mut parents_map, "exposure_1", vec!["origin"]);
        new_node(&mut node_map, &mut parents_map, "exposure_2", vec!["exposure_1"]);
        new_node(&mut node_map, &mut parents_map, "metric_1", vec!["origin"]);
        new_node(&mut node_map, &mut parents_map, "metric_2", vec!["metric_2"]);
        // We also tag on a macro, metric, and exposure to the end of each of the full words
        new_node(&mut node_map, &mut parents_map, "macro_howdy", vec!["howdy"]);
        new_node(&mut node_map, &mut parents_map, "metric_test", vec!["test"]);
        new_node(&mut node_map, &mut parents_map, "exposure_hello", vec!["hello"]);
        new_node(&mut node_map, &mut parents_map, "model_floating", vec![]);
        new_node(&mut node_map, &mut parents_map, "source_floating", vec![]);
        new_node(&mut node_map, &mut parents_map, "macro_floating", vec![]);
        new_node(&mut node_map, &mut parents_map, "metric_floating", vec![]);
        new_node(&mut node_map, &mut parents_map, "exposure_floating", vec![]);
        new_node(
            &mut node_map,
            &mut parents_map,
            "h",
            vec!["source_1", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "he",
            vec!["h", "source_2", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hel",
            vec!["he", "source_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hell",
            vec!["hel", "source_4", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hello",
            vec!["hell", "source_5", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "ho",
            vec!["h", "source_2", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "how",
            vec!["ho", "source_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "howd",
            vec!["how", "source_4", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "howdy",
            vec!["howd", "source_5", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "her",
            vec!["he", "source_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "hero",
            vec!["her", "source_4", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "t",
            vec!["source_1", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "te",
            vec!["t", "source_2", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "tes",
            vec!["te", "source_3", "origin"],
        );
        new_node(
            &mut node_map,
            &mut parents_map,
            "test",
            vec!["tes", "source_4", "origin"],
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
        assert_eq!(parents, &vec_to_set(vec!["h", "source_2", "origin"]));
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
        assert_eq!(children, &vec_to_set(vec!["h", "source_2", "origin"]));
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
    /// This test is meant to confirm that every edge is represent in both the children and parents map
    fn children_parents_exhaustive() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let parents_map = &graph.parents_map;
        assert!(graph.children_map.into_iter().all(|(parent_id, children)| {
            children.iter().all(|child_id| {
                let parents = parents_map.get(child_id);
                assert!(parents.is_some());
                let parents = parents.unwrap();
                parents.contains(&parent_id)
            })
        }));
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
        let expected = vec_to_set(vec!["h", "source_2", "origin"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_parents_two() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_parents(&vec_to_set(vec!["he"]), &Some(2)));
        let expected = vec_to_set(vec!["h", "source_2", "origin", "source_1"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_children_none() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_children(&vec_to_set(vec!["h"]), &None));
        let expected = vec_to_set(vec![
            "he", "her", "hero", "ho", "how", "howd", "howdy", "hel", "hell", "hello", "exposure_hello", "macro_howdy"
        ]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_children_no_children() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_children(&vec_to_set(vec!["exposure_hello"]), &None));
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
            "origin", "h", "he", "hel", "hell", "source_1", "source_2", "source_3", "source_4", "source_5",
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
        let expected = vec_to_set(vec!["howd", "how", "source_4", "origin"]);

        assert_eq!(expected, children);
    }

    #[test]
    fn and_select_parents_none() {
        // Should select the included set and all parents
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.and_select_parents(&vec_to_set(vec!["how"]), &None));
        let expected = vec_to_set(vec![
            "how", "ho", "h", "origin", "source_3", "source_2", "source_1",
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
            "origin", "h", "ho", "how", "howd", "howdy", "source_1", "source_2", "source_3", "source_4",
            "source_5", "macro_howdy",
        ]);

        assert_eq!(expected, children);
    }

    #[test]
    fn select_childrens_parents_complex() {
        // Should select the included set and all parents
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let children = assert_ok!(graph.select_childrens_parents(&vec_to_set(vec!["h"])));
        // While "t", "te", "tes", "test" are all connected to "origin", they are not ancestors of any children
        let expected = vec_to_set(vec![
            "origin", "h", "he", "her", "hero", "hel", "hell", "hello", "ho", "how", "howd",
            "howdy", "source_1", "source_2", "source_3", "source_4", "source_5", "macro_howdy", "exposure_hello"
        ]);

        assert_eq!(expected, children);
    }

    #[test]
    fn get_node_if_true() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let node = graph.get_node_if(&"hello".to_string(), &|_| true);

        assert!(node.is_some());
        let node = node.unwrap();
        
        assert_eq!(node.unique_id(), &"hello".to_string());
    }

    #[test]
    fn get_node_if_true_does_not_exist() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let node = graph.get_node_if(&"does_not_exist".to_string(), &|_| true);

        assert!(node.is_none());
    }

    #[test]
    fn get_node_if_false() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let node = graph.get_node_if(&"hello".to_string(), &|_| false);

        assert!(node.is_none());
    }

    #[test]
    fn is_node_true() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let is_node = graph.is_node(&"hello".to_string(), &|_| true);

        assert!(is_node);
    }

    #[test]
    fn is_node_true_does_not_exist() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let is_node = graph.is_node(&"does_not_exist".to_string(), &|_| true);

        assert!(!is_node);
    }

    #[test]
    fn is_node_false() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let is_node = graph.is_node(&"hello".to_string(), &|_| false);

        assert!(!is_node);
    }

    #[test]
    fn get_sources() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let sources = graph.get_sources();
        let num_sources = (&sources).len();

        // Confirm that all returned nodes are actually sources
        let sources: HashSet<UniqueId> = sources.into_iter().filter_map(|(unique_id, node)| {
            if node.resource_type.key() == NodeTypeKey::Source {
                Some(unique_id)
            } else {
                None
            }
        }).collect();
        assert_eq!(num_sources, (&sources).len());

        // Confirm we got all sources
        let expected = vec_to_set(vec!["source_1", "source_2", "source_3", "source_4", "source_5", "source_floating"]);
        assert_eq!(expected, sources);
    }

    #[test]
    fn get_exposures() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let exposures = graph.get_exposures();
        let num_exposures = (&exposures).len();

        // Confirm that all returned nodes are actually sources
        let exposures: HashSet<UniqueId> = exposures.into_iter().filter_map(|(unique_id, node)| {
            if node.resource_type.key() == NodeTypeKey::Exposure {
                Some(unique_id)
            } else {
                None
            }
        }).collect();
        assert_eq!(num_exposures, (&exposures).len());

        // Confirm we got all sources
        let expected = vec_to_set(vec!["exposure_1", "exposure_2", "exposure_hello", "exposure_floating"]);
        assert_eq!(expected, exposures);
    }

    #[test]
    fn get_metrics() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let metrics = graph.get_metrics();
        let num_metrics = (&metrics).len();

        // Confirm that all returned nodes are actually sources
        let metrics: HashSet<UniqueId> = metrics.into_iter().filter_map(|(unique_id, node)| {
            if node.resource_type.key() == NodeTypeKey::Metric {
                Some(unique_id)
            } else {
                None
            }
        }).collect();
        assert_eq!(num_metrics, (&metrics).len());

        // Confirm we got all sources
        let expected = vec_to_set(vec!["metric_1", "metric_2", "metric_test", "metric_floating"]);
        assert_eq!(expected, metrics);
    }

    #[test]
    fn get_macros() {
        let (node_map, parents_map) = get_test_data();
        let graph = ParsedGraph::from_parents(node_map, parents_map);

        let macros = graph.get_macros();
        let num_macros = (&macros).len();

        // Confirm that all returned nodes are actually sources
        let macros: HashSet<UniqueId> = macros.into_iter().filter_map(|(unique_id, node)| {
            if node.resource_type.key() == NodeTypeKey::Macro {
                Some(unique_id)
            } else {
                None
            }
        }).collect();
        assert_eq!(num_macros, (&macros).len());

        // Confirm we got all sources
        let expected = vec_to_set(vec!["macro_1", "macro_2", "macro_howdy", "macro_floating"]);
        assert_eq!(expected, macros);
    }
}

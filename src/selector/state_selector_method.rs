use std::{borrow::BorrowMut, collections::HashSet, rc::Rc};

use crate::{
    dbt_node_selector::{MacroNode, NodeType, SelectionError, UniqueId},
    graph::{node::WrapperNode, parsed_graph::ParsedGraph},
};

use super::{methods::SelectorTarget, node_selector::PreviousState};
use crate::SelectionError::*;

pub struct StateSelectorMethod {}

type DiffFn = dyn Fn(Option<SelectorTarget>, SelectorTarget) -> ();

impl StateSelectorMethod {
    pub fn generate_modified_macros(
        graph: &ParsedGraph,
        previous_graph: &ParsedGraph,
    ) -> Result<Vec<String>, SelectionError> {
        let mut modified_macros: Vec<String> = vec![];

        Ok(modified_macros)
    }

    pub fn prepare(
        graph: Rc<ParsedGraph>,
        previous_state: &Option<Rc<PreviousState>>,
    ) -> Result<PreviousState, SelectionError> {
        let Some(previous_state) = previous_state else { Err(RequiresPreviousState("No previous state found for state selector.".to_string()))? };
        let Some(previous_graph) = previous_state.graph.clone() else { Err(RequiresPreviousState("No previous graph found for state selector.".to_string()))? };
        let modified_macros = previous_state.get_modified_macros(&graph)?;

        match modified_macros {
            None => Ok(PreviousState::from_graph_and_macros(previous_graph, vec![])),
            Some(macros) => Ok(PreviousState::from_graph_and_macros(graph, macros.to_vec())),
        }
    }

    fn _macros_modified(
        previous_state: &Option<Rc<PreviousState>>,
        graph: Rc<ParsedGraph>,
    ) -> Result<Vec<String>, SelectionError> {
        let previous_state = previous_state.clone().ok_or(RequiresPreviousState(
            "No previous state available for comparison.".to_string(),
        ))?;
        let previous_graph = &previous_state.graph;
        let previous_graph = previous_graph.clone().ok_or(RequiresPreviousState(
            "No previous graph available for comparison.".to_string(),
        ))?;
        let old_macros = &previous_graph.get_macros();
        let new_macros = &graph.get_macros();

        let mut modified: Vec<String> = vec![];
        for (uid, new_macro) in new_macros {
            if !new_macro.same_contents(old_macros.get(uid)) {
                modified.push(uid.to_string())
            }
        }

        for (uid, _) in old_macros {
            if !new_macros.contains_key(uid) {
                modified.push(uid.to_string())
            }
        }

        Ok(modified)
    }

    fn recursively_check_macros_modified<'a>(
        graph: &ParsedGraph,
        modified_macros: &HashSet<String>,
        node: &WrapperNode,
        visited_macros: &'a mut HashSet<UniqueId>,
    ) -> bool {
        for uid in node.depends_on_macros(graph) {
            if visited_macros.contains(&uid) {
                continue;
            }

            if modified_macros.contains(&uid) {
                return true;
            }

            visited_macros.insert(uid.clone());

            let Some(next_macro_node) = graph.node_map.get(&uid) else { continue; };
            let upstream_macros_changed = Self::recursively_check_macros_modified(
                graph,
                modified_macros,
                &next_macro_node,
                visited_macros,
            );
            if upstream_macros_changed {
                return true;
            }
        }
        false
    }

    fn check_macros_modified(
        graph: &ParsedGraph,
        modified_macros: &HashSet<String>,
        base_node: &WrapperNode,
    ) -> bool {
        if modified_macros.len() == 0 {
            false
        } else {
            let mut visited_macros: HashSet<String> = HashSet::new();
            Self::recursively_check_macros_modified(
                graph,
                modified_macros,
                base_node,
                &mut visited_macros,
            )
        }
    }

    // fn make_check_modified_macros(
    //     graph: &ParsedGraph,
    //     modified_macros: &HashSet<String>,
    // ) -> DiffFn {
    //     |_, &target| Self::check_macros_modified(graph, modified_macros, target)
    // }

    pub fn search(
        previous_state: &Option<Rc<PreviousState>>,
        graph: Rc<ParsedGraph>,
        _included_nodes: &HashSet<UniqueId>,
        selector: &str,
    ) -> Result<Vec<String>, SelectionError> {
        let checker = match selector {
            "new" => |prev_option: &Option<WrapperNode>, _curr: &WrapperNode| prev_option.is_none(),
            "modified" => unimplemented!(),
            "modified.body" => unimplemented!(),
            "modified.configs" => unimplemented!(),
            "modified.persisted_descriptions" => unimplemented!(),
            "modified.relation" => unimplemented!(),
            "modified.macros" => unimplemented!(), //Self::make_check_modified_macros(graph.as_ref(), modified_macros),
            "modified.contract" => unimplemented!(),
            _ => Err(InvalidSelector(format!(
                "Got an invalid macro selector '{}'",
                selector
            )))?,
        };

        Ok(graph
            .node_map
            .iter()
            .filter_map(|(unique_id, node)| {
                let prev_option = previous_state.clone().and_then(|p| p.get_node(&unique_id));
                if checker(&prev_option, &node) {
                    Some(unique_id.clone())
                } else {
                    None
                }
            })
            .collect())
    }
}

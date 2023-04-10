use std::{collections::HashSet, rc::Rc};

use crate::{
    dbt_node_selector::{SelectionError, UniqueId},
    graph::{node_comparison::WrapperNodeExt, parsed_graph::ParsedGraph, node::WrapperNode},
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
            if let Some(old_macro) = old_macros.get(uid) {
                if new_macro.macro_node.macro_sql != old_macro.macro_node.macro_sql {
                    modified.push(uid.to_string());
                }
            } else {
                modified.push(uid.to_string());
            }
        }

        for (uid, _) in old_macros {
            if !new_macros.contains_key(uid) {
                modified.push(uid.to_string())
            }
        }

        Ok(modified)
    }

    fn recursively_check_macros_modified(
        graph: &ParsedGraph,
        modified_macros: &HashSet<String>,
        node: &WrapperNode,
        mut visited_macros: HashSet<UniqueId>,
    ) -> bool {
        for uid in node.depends_on_macros(graph) {
            if (visited_macros.contains(uid)) {
                continue;
            }

            if (modified_macros.contains(uid)) {
                return true;
            }

            visited_macros.insert(uid.clone());

            let Some(next_macro_node) = graph.node_map.get(uid) else { continue; };
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
                visited_macros,
            )
        }
    }

    fn make_check_modified_macros(
        graph: &ParsedGraph,
        modified_macros: &HashSet<String>,
    ) -> DiffFn {
        |_, &target| Self::check_macros_modified(graph, modified_macros, target)
    }

    pub fn search(
        previous_state: &Option<Rc<PreviousState>>,
        graph: Rc<ParsedGraph>,
        _included_nodes: &HashSet<UniqueId>,
        selector: &str,
    ) -> Result<Vec<String>, SelectionError> {
        let checker: &dyn Fn(Option<SelectorTarget>) -> () = match selector {
            "new" => unimplemented!(),
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
        unimplemented!();
    }
}

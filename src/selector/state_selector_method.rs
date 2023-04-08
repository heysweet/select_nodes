use std::{collections::HashSet, rc::Rc};

use crate::{
    dbt_node_selector::{SelectionError, UniqueId},
    graph::parsed_graph::ParsedGraph,
};

use super::{node_selector::{self, PreviousState}, MethodName};
use crate::SelectionError::*;

pub struct StateSelectorMethod {}

impl StateSelectorMethod {
    pub fn generate_modified_macros(
        graph: &ParsedGraph,
        previous_graph: &ParsedGraph,
    ) -> Result<Vec<String>, SelectionError> {
        let mut modified_macros: Vec<String> = vec![];

        Ok(modified_macros)
    }

    pub fn prepare(graph: Rc<ParsedGraph>, previous_state: &Option<Rc<PreviousState>>) -> Result<PreviousState, SelectionError>  {
        let Some(previous_state) = previous_state else { Err(RequiresPreviousState("No previous state found for state selector.".to_string()))? };
        let Some(previous_graph) = previous_state.graph.clone() else { Err(RequiresPreviousState("No previous graph found for state selector.".to_string()))? };
        let modified_macros = previous_state.get_modified_macros(&graph)?;
        
        match modified_macros {
            None => Ok(PreviousState::from_graph_and_macros(previous_graph, vec![])),
            Some(macros) => Ok(PreviousState::from_graph_and_macros(graph, macros.to_vec())),
        }
    }

    pub fn search(
        previous_state: &Option<Rc<PreviousState>>,
        graph: Rc<ParsedGraph>,
        included_nodes: &HashSet<UniqueId>,
        selector: &str,
    ) -> Result<Vec<String>, SelectionError> {
        let previous_state = previous_state
            .clone()
            .ok_or(RequiresPreviousState(
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
}

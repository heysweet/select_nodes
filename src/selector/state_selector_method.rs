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
                "No comparison manifest in _macros_modified".to_string(),
            ))?;

        unimplemented!()
    }
}

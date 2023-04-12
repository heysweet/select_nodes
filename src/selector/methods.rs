/// core/dbt/graph/selector_methods.py
use std::{collections::HashSet, path::Path, rc::Rc};

use crate::{
    dbt_node_selector::{SelectionError, UniqueId},
    file::fnmatch_normalized,
    graph::{
        node::{NodeTypeKey, WrapperNode, WrapperNodeExt},
        parsed_graph::ParsedGraph,
    },
};

use super::{node_selector::PreviousState, state_selector_method::StateSelectorMethod, MethodName};
use crate::dbt_node_selector::SelectionError::*;

use MethodName::*;

impl MethodName {
    /// Dots in model names act as namespace separators
    fn flatten_node_parts(fqn: &Vec<String>) -> Vec<String> {
        fqn.iter()
            .flat_map(|segment| segment.split("."))
            .map(|s| s.to_string())
            .collect()
    }

    fn is_selected_node(fqn: &Vec<String>, node_selector: &str) -> bool {
        let last = fqn.last();
        if last == Some(&node_selector.to_string()) {
            return true;
        }
        let flat_fqn = Self::flatten_node_parts(fqn);
        let selector_parts: Vec<&str> = node_selector.split(".").collect();
        if flat_fqn.len() < selector_parts.len() {
            return false;
        }
        let wildcard = vec!['*', '?', '[', ']'];
        for (i, part) in selector_parts.clone().into_iter().enumerate() {
            for char in &wildcard {
                if part.contains(*char) {
                    // If we have a wildcard, we need to make sure that the selector matches the
                    // rest of the fqn, this is 100% backwards compatible with the old behavior of
                    // encountering a wildcard but more expressive in naturally allowing you to
                    // match the rest of the fqn with more advanced patterns
                    let flat_fqn = flat_fqn[i..].join(".");
                    let regex = fnmatch_regex::glob_to_regex(flat_fqn.as_str());
                    let remainder = &(selector_parts)[i..];
                    let remainder = remainder.join(".");
                    match regex {
                        Err(_) => false,
                        Ok(regex) => regex.is_match(remainder.as_str()),
                    };
                }
            }
            if flat_fqn[i] != part {
                return false;
            }
        }
        return true;
    }

    fn is_node_match(&self, qualified_name: &str, fqn: &Option<Vec<String>>) -> bool {
        let Some(fqn) = fqn else { return false };
        if Self::is_selected_node(fqn, qualified_name) {
            true
        } else {
            let unscoped_fqn = &fqn[1..].to_vec();
            // Match nodes across different packages
            Self::is_selected_node(unscoped_fqn, qualified_name)
        }
    }

    /// Some methods (StateSelectorMethod) use prepare in order to update
    /// cached state.
    pub fn prepare(
        &self,
        graph: Rc<ParsedGraph>,
        previous_state: &Option<Rc<PreviousState>>,
    ) -> Option<Result<PreviousState, SelectionError>> {
        match self {
            State => Some(StateSelectorMethod::prepare(graph, previous_state)),
            _ => None,
        }
    }

    pub fn search(
        &self,
        previous_state: &Option<Rc<PreviousState>>,
        graph: Rc<ParsedGraph>,
        included_nodes: &HashSet<UniqueId>,
        selector: &str,
    ) -> Result<Vec<String>, SelectionError> {
        match self {
            FQN => Ok(graph
                .node_map
                .iter()
                .filter_map(|(id, node)| {
                    if self.is_node_match(selector, &node.fqn()) {
                        Some(id.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()),

            Tag => {
                // TODO: Confirm with core there's no need to fnmatch tags, and just a string
                // match (lowercased) should be sufficient
                let selector = &selector.to_lowercase();
                Ok(graph.node_map.iter().filter_map(|(unique_id, node)| {
                    node.has_tag(selector).then_some(unique_id.to_string())
                }).collect())
            }

            Group => {
                Ok(graph.node_map.iter().filter_map(|(unique_id, node)| {
                    let config = node.config();
                    let Some(group) = config.get("group") else { return None; };
                    (group == selector).then_some(unique_id.to_string())
                }).collect())
            }

            Source => {
                unimplemented!()
            }

            Self::Path => {
                let Ok(regex) = &fnmatch_regex::glob_to_regex(selector) else {
                    return Err(SelectionError::FailedRegexMatchError(selector.to_string()));
                };
                Ok(graph.node_map.iter().filter_map(|(unique_id, node)| {
                    if regex.is_match_at(node.original_file_path().as_str(), 0) {
                        Some(unique_id.to_string())
                    } else {
                        None
                    }
                }).collect::<Vec<String>>())
            }

            File => Ok(graph
                .node_map
                .iter()
                .filter_map(|(id, node)| {
                    let file_path = &node.original_file_path();
                    let Some(file_name) = Path::new(file_path).file_name() else { return None; };
                    let Some(file_name) = file_name.to_str() else { return None; };
                    let Ok(is_match) = fnmatch_normalized(file_name, selector) else { return None; };
                    is_match.then_some(id.to_string())
                })
                .collect::<Vec<String>>()),

            Package => Ok(graph
                .node_map
                .iter()
                .filter_map(|(id, node)| {
                    let Ok(is_match) = fnmatch_normalized(node.package_name(), selector) else { return None; };
                    is_match.then_some(id.to_string())
                })
                .collect::<Vec<String>>()),

            Config => {
                unimplemented!()
            }

            TestName => {
                unimplemented!()
            }

            TestType => {
                unimplemented!()
            }

            ResourceType => {
                let resource_key = NodeTypeKey::from_key(selector);
                match resource_key {
                    Err(_) => Err(NoMatchingResourceType(selector.to_string())),
                    Ok(resource_key) => {
                        let iter = graph.node_map.iter();
                        let iter =
                            iter.filter(|(_, node)| node.resource_type().key() == resource_key);
                        let iter = iter.map(|(id, _)| id.clone());
                        Ok(iter.collect())
                    }
                }
            }

            State => StateSelectorMethod::search(previous_state, graph, included_nodes, selector),

            Exposure => {
                unimplemented!()
            }

            Metric => {
                unimplemented!()
            }

            RunResult => {
                unimplemented!()
            }

            SourceStatus => {
                unimplemented!()
            }

            Wildcard => {
                unimplemented!()
            }
        }
    }
}

pub type SelectorTarget = WrapperNode;

/// core/dbt/graph/selector_methods.py
use std::{path::Path, rc::Rc};

use crate::{
    dbt_node_selector::{NodeType, SelectionError},
    graph::parsed_graph::ParsedGraph,
};

use super::MethodName;
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
                    let fnmatch = fnmatch_regex::glob_to_regex(flat_fqn.as_str());
                    let remainder = &(selector_parts)[i..];
                    let remainder = remainder.join(".");
                    match fnmatch {
                        Err(_) => false,
                        Ok(fnmatch) => fnmatch.is_match(remainder.as_str()),
                    };
                }
            }
            if flat_fqn[i] != part {
                return false;
            }
        }
        return true;
    }

    fn is_node_match(&self, qualified_name: &str, fqn: &Vec<String>) -> bool {
        if Self::is_selected_node(fqn, qualified_name) {
            true
        } else {
            let unscoped_fqn = &fqn[1..].to_vec();
            // Match nodes across different packages
            Self::is_selected_node(unscoped_fqn, qualified_name)
        }
    }

    fn fnmatch(path: &str, selector: &str) -> bool {
        let file_name = Path::new(path).file_name();
        if file_name.is_none() {
            return false;
        }
        let path = file_name.unwrap().to_str();
        if path.is_none() {
            return false;
        }

        let fnmatch = fnmatch_regex::glob_to_regex(path.unwrap());
        match fnmatch {
            Ok(fnmatch) => fnmatch.is_match(selector),
            Err(_) => false,
        }
    }

    pub fn search(
        &self,
        previous_state: &Option<Rc<ParsedGraph>>,
        subgraph: &ParsedGraph,
        selector: &str,
    ) -> core::result::Result<Vec<String>, SelectionError> {
        match self {
            FQN => Ok(subgraph
                .node_map
                .iter()
                .filter_map(|(id, node)| {
                    if self.is_node_match(selector, &node.fqn) {
                        Some(id.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()),

            Tag => {
                unimplemented!()
            }

            Group => {
                unimplemented!()
            }

            Source => {
                unimplemented!()
            }

            Self::Path => {
                unimplemented!()
            }

            File => Ok(subgraph
                .node_map
                .iter()
                .filter_map(|(id, node)| {
                    Self::fnmatch(&node.original_file_path, selector).then(|| id.to_string())
                })
                .collect::<Vec<String>>()),

            Package => Ok(subgraph
                .node_map
                .iter()
                .filter_map(|(id, node)| {
                    Self::fnmatch(&node.package_name, selector).then(|| id.to_string())
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
                let resource_type = NodeType::from_string(selector);
                match resource_type {
                    Err(_) => Err(NoMatchingResourceType(selector.to_string())),
                    Ok(resource_type) => {
                        let iter = subgraph.node_map.iter();
                        let iter = iter.filter(|(_, node)| node.resource_type == resource_type);
                        let iter = iter.map(|(id, _)| id.clone());
                        Ok(iter.collect())
                    }
                }
            }

            State => {
                let previous_state =
                    previous_state
                        .clone()
                        .ok_or(StateSelectorWithNoPreviousState(
                            "No comparison manifest in _macros_modified".to_string(),
                        ))?;
                unimplemented!()
            }

            Exposure => {
                unimplemented!()
            }

            Metric => {
                unimplemented!()
            }

            Result => {
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

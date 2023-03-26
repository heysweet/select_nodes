use std::fmt::Display;

use crate::graph::{node::NodeType, ParsedGraph};

use super::{spec::SelectionError, MethodName};

use MethodName::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchError {
    SelectionError { selection_error: SelectionError },
    // TODO(SourceStatusSelectorMethod)
    NoPreviousStateError {},
    // TODO(SourceStatusSelectorMethod)
    NoCurrentSourcesError {},
    NoMatchingResourceTypeError { selector: String },
}

use SearchError::*;

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoPreviousStateError {} => {
                write!(f, "No previous state comparison freshness results")
            }
            NoCurrentSourcesError {} => {
                write!(f, "No current state comparison freshness results")
            }
            NoMatchingResourceTypeError { selector } => {
                write!(f, "Invalid resource_type selector '{}'", selector)
            }
            SearchError::SelectionError { selection_error } => write!(f, "{}", selection_error),
        }
    }
}

impl MethodName {
    /// Dots in model names act as namespace separators
    fn flatten_node_parts(fqn: &Vec<String>) -> Vec<String> {
        fqn.iter()
            .flat_map(|segment| segment.split("."))
            .map(|s| s.to_string())
            .collect()
    }

    // TODO from selector_methods.py
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
        for (i, part) in selector_parts.into_iter().enumerate() {
            for char in &wildcard {
                if part.contains(*char) {
                    // PYTHON
                    // # If we have a wildcard, we need to make sure that the selector matches the
                    // # rest of the fqn, this is 100% backwards compatible with the old behavior of
                    // # encountering a wildcard but more expressive in naturally allowing you to
                    // # match the rest of the fqn with more advanced patterns
                    // return fnmatch(
                    //     ".".join(flat_fqn[slurp_from_ix:]),
                    //     ".".join(node_selector.split(".")[slurp_from_ix:]),
                    // )
                    todo!()
                }
            }
            if (flat_fqn[i] != part) {
                return false
            }
        }
        return true
    }

    fn is_node_match(&self, qualified_name: &str, fqn: &Vec<String>) -> bool {
        true
    }

    pub fn search(
        &self,
        graph: &ParsedGraph,
        selector: &str,
    ) -> core::result::Result<Vec<String>, SearchError> {
        match self {
            FQN => Ok(graph
                .node_map
                .iter()
                .filter_map(|(id, node)| {
                    if self.is_node_match(selector, &node.fqn) {
                        Some(id.clone())
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

            Path => {
                unimplemented!()
            }

            File => {
                unimplemented!()
            }

            Package => {
                unimplemented!()
            }

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
                    Err(_) => Err(NoMatchingResourceTypeError {
                        selector: selector.to_string(),
                    }),
                    Ok(resource_type) => {
                        let iter = graph.node_map.iter();
                        let iter = iter.filter(|(_, node)| node.resource_type.key() == selector);
                        let iter = iter.map(|(id, _)| id.clone());
                        Ok(iter.collect())
                    }
                }
            }

            State => {
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

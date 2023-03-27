use crate::ParsedGraph;
use crate::UniqueId;
use std::collections::HashSet;

use crate::selector::methods::SearchError;
use crate::SelectionCriteria;

use super::SelectionError;

pub struct NodeSelector {
    graph: ParsedGraph,
}

impl NodeSelector {
    pub fn select_included(
        &self,
        included_nodes: HashSet<UniqueId>,
        spec: SelectionCriteria,
    ) -> Result<HashSet<UniqueId>, SearchError> {
        let result = spec
            .method
            .search(&self.graph.filter(&included_nodes), &spec.value)?;
        Ok(HashSet::from_iter(result.iter().map(|s| s.to_owned())))
    }

    /// Given the set of models selected by the explicit part of the
    /// selector (like "tag:foo"), apply the modifiers on the spec ("+"/"@").
    /// Return the set of additional nodes that should be collected (which may
    /// overlap with the selected set).
    pub fn collect_specified_neighbors(
        &self,
        spec: SelectionCriteria,
        selected: HashSet<UniqueId>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut additional = HashSet::new();
        let graph = &self.graph;

        if spec.childrens_parents {
            additional.extend(graph.select_childrens_parents(&selected)?);
        } else {
            if spec.children {
                additional.extend(graph.select_children(&selected, None)?);
            }
            if spec.parents {
                additional.extend(graph.select_parents(&selected, None)?);
            }
        }
        Ok(additional)
    }
}

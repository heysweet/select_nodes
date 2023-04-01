use crate::GraphNode;
use crate::ParsedGraph;
use crate::UniqueId;
use std::collections::HashSet;

use crate::interface::SelectionError;
use crate::selector::spec::SelectionCriteria;

trait NodeMatch {
    fn node_is_match(&self, node: GraphNode) -> bool;
}

trait OtherSelectNodes {
    /// Given the set of models selected by the explicit part of the
    /// selector (like "tag:foo"), apply the modifiers on the spec ("+"/"@").
    /// Return the set of additional nodes that should be collected (which may
    /// overlap with the selected set).
    fn collect_specified_neighbors(
        &self,
        spec: SelectionCriteria,
        selected: HashSet<UniqueId>,
    ) -> Result<HashSet<UniqueId>, SelectionError>;

    // fn new(graph: ParsedGraph, previous_state: PreviousState) -> Self;
}

impl ParsedGraph {
    fn select_included(
        &self,
        included_nodes: HashSet<UniqueId>,
        spec: SelectionCriteria,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let result = spec
            .method
            .search(&None, &self.filter(&included_nodes), &spec.value)?;
        Ok(HashSet::from_iter(result.iter().map(|s| s.to_owned())))
    }

    /// Get all nodes specified by the single selection criteria.
    ///
    /// - collect the directly included nodes
    /// - find their specified relatives
    /// - perform any selector-specific expansion
    fn get_nodes_from_criteria(
        &self,
        included_nodes: HashSet<UniqueId>,
        spec: SelectionCriteria,
    ) -> Result<(HashSet<UniqueId>, HashSet<UniqueId>), SelectionError> {
        // TODO: SelectorReportInvalidSelector in py has better error
        let collected: HashSet<UniqueId> = self.select_included(included_nodes, spec)?;
        todo!()
        // match spec.indirect_selection {
        //     Empty => Ok((collected, HashSet::new())),
        //     indirect_selection => {
        //         todo!()
        //     }
        // }
    }

    // Return the subset of selected nodes that is a match for this selector.
    // fn filter_selection(&self, selected: HashSet<UniqueId>) -> HashSet<UniqueId> {

    // }
}

impl OtherSelectNodes for ParsedGraph {
    fn collect_specified_neighbors(
        &self,
        spec: SelectionCriteria,
        selected: HashSet<UniqueId>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut additional: HashSet<UniqueId> = HashSet::new();

        if spec.childrens_parents {
            additional.extend(self.select_childrens_parents(&selected)?);
        } else {
            if spec.children {
                additional.extend(self.select_children(&selected, None)?);
            }
            if spec.parents {
                additional.extend(self.select_parents(&selected, None)?);
            }
        }
        Ok(additional)
    }
}

use crate::selector::spec::IndirectSelection;
use crate::GraphNode;
use crate::ParsedGraph;
use crate::UniqueId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

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
        included_nodes: &HashSet<UniqueId>,
        spec: &SelectionCriteria,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let result = spec
            .method
            .search(&None, &self.filter(&included_nodes), &spec.value)?;
        Ok(HashSet::from_iter(result.iter().map(|s| s.to_owned())))
    }

    fn successors(&self, node_id: &UniqueId) -> Option<impl Iterator<Item = &UniqueId>> {
        self.children_map
            .get(node_id)
            .and_then(|children| Some(children.iter()))
    }

    pub fn select_successors(&self, selected: &HashSet<UniqueId>) -> HashSet<String> {
        let mut successors = HashSet::new();
        for node_id in selected.iter() {
            match self.successors(node_id) {
                Some(new_successors) => successors.extend(new_successors),
                None => {}
            }
        }
        successors.into_iter().map(|id| id.to_string()).collect()
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
        let collected: HashSet<UniqueId> = self.select_included(&included_nodes, &spec)?;

        match &spec.indirect_selection {
            crate::selector::spec::IndirectSelection::Empty => Ok((collected, HashSet::new())),
            indirect_selector => {
                let neighbors = self.collect_specified_neighbors(spec, collected)?;
                todo!()
                // expand_selection()
            }
        }
    }

    /// Test selection by default expands to include an implicitly/indirectly selected tests.
    /// `dbt test -m model_a` also includes tests that directly depend on `model_a`.
    /// Expansion has four modes, EAGER, CAUTIOUS and BUILDABLE, EMPTY.
    ///
    /// EAGER mode: If ANY parent is selected, select the test.
    ///
    /// CAUTIOUS mode:
    ///  - If ALL parents are selected, select the test.
    ///  - If ANY parent is missing, return it separately. We'll keep it around
    ///    for later and see if its other parents show up.
    ///
    /// BUILDABLE mode:
    ///  - If ALL parents are selected, or the parents of the test are themselves parents of the selected, select the test.
    ///  - If ANY parent is missing, return it separately. We'll keep it around
    ///    for later and see if its other parents show up.
    ///
    /// EMPTY mode: Only select the given node and ignore attached nodes (i.e. ignore tests attached to a model)
    ///
    /// Users can opt out of inclusive EAGER mode by passing --indirect-selection cautious
    /// CLI argument or by specifying `indirect_selection: true` in a yaml selector
    fn expand_selection(
        &self,
        selected: &HashSet<UniqueId>,
        indirect_selection: IndirectSelection,
    ) -> Result<(HashSet<UniqueId>, HashSet<UniqueId>), SelectionError> {
        let direct_nodes = selected.clone();
        let indirect_nodes: HashSet<UniqueId> = HashSet::new();
        let selected_and_parents: HashSet<UniqueId> = HashSet::new();

        if indirect_selection == IndirectSelection::Buildable {
            let selected_and_parents: HashSet<UniqueId> = self
                .select_parents(selected, None)?
                .union(&self.sources)
                .map(|s| s.into())
                .collect();
            let selected_and_parents: HashSet<UniqueId> = selected
                .union(&selected_and_parents)
                .map(|s| s.into())
                .collect();
        }
        todo!()
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

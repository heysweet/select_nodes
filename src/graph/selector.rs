use std::collections::HashSet;
use crate::ParsedGraph;
use crate::UniqueId;

use crate::SelectionCriteria;
use crate::selector::methods::SearchError;
use crate::selector::spec::SelectionError;

pub struct NodeSelector {
    graph: ParsedGraph,
}

impl NodeSelector {
    pub fn select_included(&self, included_nodes: HashSet<UniqueId>, spec: SelectionCriteria) -> Result<HashSet<UniqueId>, SearchError> {
        let result = spec.method.search(&self.graph.filter(&included_nodes), spec)?
    }

    /// Given the set of models selected by the explicit part of the
    /// selector (like "tag:foo"), apply the modifiers on the spec ("+"/"@").
    /// Return the set of additional nodes that should be collected (which may
    /// overlap with the selected set).
    pub fn collect_specified_neighbors(&self, spec: SelectionCriteria, selected: HashSet<UniqueId>) -> HashSet<UniqueId> {
        let additional = HashSet::new();
        todo!()
    } 
}
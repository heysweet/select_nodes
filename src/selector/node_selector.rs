use wai_bindgen_rust::Handle;

use crate::graph::{
    node::{WrapperNode, WrapperNodeExt},
    parsed_graph::ParsedGraph,
    UniqueId,
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::selector::spec::{IndirectSelection, SelectionCriteria, SelectionGroup, SelectionSpec};

use crate::dbt_node_selector::{
    Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError,
};

use crate::IndirectSelection::*;
use crate::SelectionError::*;

use super::{spec::SetOperation, state_selector_method::StateSelectorMethod};

pub struct PreviousState {
    pub graph: Option<Rc<ParsedGraph>>,
    /// modified_macros is a cache of computed macros, which allows for reuse
    /// of prior modified macros computations.
    pub modified_macros: RefCell<Option<HashSet<UniqueId>>>,
}

impl PreviousState {
    pub fn get_node(&self, unique_id: &UniqueId) -> Option<WrapperNode> {
        self.graph
            .as_ref()
            .and_then(|graph| graph.node_map.get(unique_id))
            .cloned()
    }

    pub fn default() -> Self {
        Self {
            graph: None,
            modified_macros: None.into(),
        }
    }

    pub fn from_graph(prev_graph: Rc<ParsedGraph>) -> Self {
        Self {
            graph: Some(prev_graph),
            modified_macros: None.into(),
        }
    }

    pub fn from_graph_and_macros(
        prev_graph: Rc<ParsedGraph>,
        modified_macros: HashSet<UniqueId>,
    ) -> Self {
        Self {
            graph: Some(prev_graph),
            modified_macros: Some(modified_macros.into_iter().collect()).into(),
        }
    }

    pub fn get_modified_macros(
        &self,
        current_graph: &ParsedGraph,
    ) -> Result<Option<HashSet<UniqueId>>, SelectionError> {
        let previous_graph = &self.graph;
        let modified_macros = self.modified_macros.borrow();
        let modified_macros = modified_macros.as_ref();
        match (previous_graph, modified_macros) {
            (_, Some(previous_macros)) => Ok(Some(previous_macros.clone())),
            (None, _) => Err(RequiresPreviousState(
                "No previous state to generate modified macros.".to_string(),
            )),
            (Some(previous_graph), _) => {
                let modified_macros = StateSelectorMethod::generate_modified_macros(
                    current_graph,
                    &(previous_graph.clone()),
                )?;
                Ok(Some(modified_macros))
            }
        }
    }
}

pub struct NodeSelector {
    pub graph: Rc<ParsedGraph>,
    pub previous_state: Option<Rc<PreviousState>>,
}

type DirectNodes = HashSet<UniqueId>;
type IndirectNodes = HashSet<UniqueId>;

trait NodeMatch {
    fn node_is_match(&self, node: WrapperNode) -> bool;
}

trait OtherSelectNodes {
    /// Given the set of models selected by the explicit part of the
    /// selector (like "tag:foo"), apply the modifiers on the spec ("+"/"@").
    /// Return the set of additional nodes that should be collected (which may
    /// overlap with the selected set).
    fn collect_specified_neighbors(
        &self,
        spec: &SelectionCriteria,
        selected: &HashSet<UniqueId>,
    ) -> Result<HashSet<UniqueId>, SelectionError>;

    // fn new(graph: ParsedGraph, previous_state: PreviousState) -> Self;
}

impl OtherSelectNodes for NodeSelector {
    fn collect_specified_neighbors(
        &self,
        spec: &SelectionCriteria,
        selected: &HashSet<UniqueId>,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let mut additional: HashSet<UniqueId> = HashSet::new();

        if spec.childrens_parents {
            additional.extend(self.graph.select_childrens_parents(&selected)?);
        } else {
            if spec.children {
                additional.extend(
                    self.graph
                        .select_children(&selected, &spec.children_depth)?,
                );
            }
            if spec.parents {
                additional.extend(self.graph.select_parents(&selected, &spec.parents_depth)?);
            }
        }
        Ok(additional)
    }
}

impl NodeSelector {
    pub fn from(
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        previous_state: Option<Rc<PreviousState>>,
    ) -> Result<Self, SelectorCreateError> {
        let mut node_map = HashMap::<UniqueId, WrapperNode>::new();
        for node in nodes.iter() {
            node_map.insert(node.unique_id.to_owned(), WrapperNode::from(node)?);
        }

        let mut parent_map = HashMap::<UniqueId, HashSet<UniqueId>>::new();
        for edge in edges.iter() {
            let mut parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents.to_owned());
            parent_map.insert(edge.unique_id.to_owned(), parents);
        }
        let graph = Rc::new(ParsedGraph::from_parents(node_map, parent_map));
        Ok(Self {
            graph,
            previous_state,
        })
    }

    fn select_included(
        &self,
        included_nodes: &HashSet<UniqueId>,
        spec: &SelectionCriteria,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let new_previous_state = spec
            .method
            .prepare(self.graph.clone(), &self.previous_state);

        match (self.previous_state.clone(), new_previous_state) {
            (Some(saved_previous_state), Some(Ok(new_previous_state))) => {
                // Write to the modified_macros cache whenever we get new_previous_state
                *saved_previous_state.modified_macros.borrow_mut() =
                    new_previous_state.modified_macros.into_inner();

                let result =
                    spec.method
                        .search(&None, self.graph.clone(), included_nodes, &spec.value)?;
                Ok(HashSet::from_iter(result.iter().map(|s| s.to_owned())))
            }
            (_, None) => {
                let result =
                    spec.method
                        .search(&None, self.graph.clone(), included_nodes, &spec.value)?;
                Ok(HashSet::from_iter(result.iter().map(|s| s.to_owned())))
            }
            (_, Some(Err(e))) => return Err(e),
            (_, _) => {
                return Err(RequiresPreviousState(
                    "Unknown issue with previous state".to_string(),
                ))
            }
        }
    }

    fn successors(&self, node_id: &UniqueId) -> Option<impl Iterator<Item = &UniqueId>> {
        self.graph
            .children_map
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
        spec: &SelectionCriteria,
    ) -> Result<(DirectNodes, IndirectNodes), SelectionError> {
        let nodes: HashSet<UniqueId> = self
            .graph
            .node_map
            .keys()
            .map(|id| id.to_string())
            .collect();
        // TODO: SelectorReportInvalidSelector in py has better error
        let collected: HashSet<UniqueId> = self.select_included(&nodes, &spec)?;

        match &spec.indirect_selection {
            Empty => Ok((collected, HashSet::new())),
            indirect_selector => {
                let neighbors = self.collect_specified_neighbors(&spec, &collected)?;
                let selected: HashSet<UniqueId> = collected
                    .union(&neighbors)
                    .map(|id| id.to_string())
                    .collect();
                let (direct_nodes, indirect_nodes) =
                    self.expand_selection(&selected, indirect_selector)?;
                Ok((direct_nodes, indirect_nodes))
            }
        }
    }

    /// Check tests previously selected indirectly to see if ALL their
    /// parents are now present.
    fn incorporate_indirect_nodes(
        &self,
        direct_nodes: &HashSet<UniqueId>,
        indirect_nodes: &HashSet<UniqueId>,
        indirect_selection: &IndirectSelection,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        if direct_nodes.eq(indirect_nodes) {
            return Ok(direct_nodes.clone());
        }
        let mut selected = direct_nodes.clone();
        match indirect_selection {
            Cautious => {
                for unique_id in indirect_nodes {
                    let Some(node) = self.graph.node_map.get(unique_id) else {
                        continue;
                    };
                    if selected.is_superset(&node.depends_on()) {
                        selected.insert(unique_id.to_string());
                    }
                }
                Ok(selected)
            }
            Buildable => {
                let selected_and_parents: HashSet<String> =
                    self.graph.and_select_parents(&selected, &None)?;
                for unique_id in indirect_nodes {
                    let Some(node) = self.graph.node_map.get(unique_id) else {
                        continue;
                    };
                    if selected_and_parents.is_superset(&node.depends_on()) {
                        selected.insert(unique_id.to_string());
                    }
                }
                Ok(selected)
            }
            _ => Ok(selected),
        }
    }

    pub fn get_selected_type(
        &self,
        selection_group: &SelectionGroup,
        resource_type_filter: &ResourceTypeFilter,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let (selected_nodes, _indirect_only) = self.select_nodes(selection_group)?;

        self.filter_selection(&selected_nodes, resource_type_filter)
    }

    /// get_selected runs through the node selection process:
    ///
    /// - node selection. Based on the include/exclude sets, the set
    ///     of matched unique IDs is returned
    ///     - includes direct + indirect selection (for tests)
    /// - filtering:
    ///     - selectors can filter the nodes after all of them have been
    ///         selected
    pub fn get_selected(
        &self,
        selection_group: &SelectionGroup,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        self.get_selected_type(selection_group, &ResourceTypeFilter::All)
    }

    fn _is_match(
        &self,
        unique_id: &UniqueId,
        resource_type_filter: &ResourceTypeFilter,
    ) -> Result<bool, SelectionError> {
        // TODO: it looks like manifest.nodes is not a superset of
        // sources, exposures, metrics
        //  core/dbt/contracts/graph/nodes.py
        match self.graph.node_map.get(unique_id) {
            None => Err(SelectionError::NodeNotInGraph(unique_id.to_string())),
            Some(node) => Ok(resource_type_filter.should_include(&node.resource_type())),
        }
    }

    /// Return the subset of selected nodes that is a match for this selector.
    fn filter_selection(
        &self,
        selected: &HashSet<UniqueId>,
        resource_type_filter: &ResourceTypeFilter,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let filtered =
            selected
                .iter()
                .filter_map(|id| match self._is_match(&id, resource_type_filter) {
                    Ok(false) => None,
                    Ok(true) => Some(Ok(id.to_string())),
                    Err(e) => Some(Err(e)),
                });
        let err = filtered.clone().find(|e| e.is_err());
        match err {
            Some(err) => Err(err.unwrap_err()),
            None => Ok(filtered.map(|id| id.unwrap()).collect()),
        }
    }

    /// Select the nodes in the graph according to the spec.
    ///
    /// This is the main point of entry for turning a spec into a set of nodes:
    /// - Recurse through spec, select by criteria, combine by set operation
    /// - Return final (unfiltered) selection set
    fn select_nodes(
        &self,
        selection_group: &SelectionGroup,
    ) -> Result<(DirectNodes, IndirectNodes), SelectionError> {
        let (direct_nodes, indirect_nodes) = self.select_nodes_recursively(selection_group)?;
        let indirect_only =
            HashSet::difference(&indirect_nodes, &direct_nodes).map(|s| s.to_string());
        Ok((direct_nodes.to_owned(), indirect_only.collect()))
    }

    /// If the spec is a composite spec (a union, difference, or intersection),
    /// recurse into its selections and combine them. If the spec is a concrete
    /// selection criteria, resolve that using the given graph.
    fn select_nodes_recursively(
        &self,
        selection_group: &SelectionGroup,
    ) -> Result<(DirectNodes, IndirectNodes), SelectionError> {
        match &selection_group.spec {
            SelectionSpec::SelectionCriteria(spec) => self.get_nodes_from_criteria(&spec),
            SelectionSpec::SetOperation(operation) => {
                let bundles = selection_group
                    .components
                    .iter()
                    .map(|component| self.select_nodes_recursively(component));

                let mut direct_sets: Vec<HashSet<UniqueId>> = vec![];
                let mut indirect_sets: Vec<HashSet<UniqueId>> = vec![];

                for result in bundles {
                    let (direct, indirect) = result?;
                    indirect_sets.push(direct.union(&indirect).map(|s| s.to_owned()).collect());
                    direct_sets.push(direct);
                }

                let initial_direct = operation.combine_selections(&direct_sets);
                let indirect_nodes = operation.combine_selections(&indirect_sets);

                let direct_nodes: HashSet<UniqueId> = self.incorporate_indirect_nodes(
                    &initial_direct,
                    &indirect_nodes,
                    &selection_group.indirect_selection,
                )?;

                match selection_group.expect_exists && direct_nodes.len() == 0 {
                    true => Err(SelectionError::NoNodesForSelectionCriteria(
                        selection_group.raw.clone(),
                    )),
                    false => Ok((direct_nodes, indirect_nodes)),
                }
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
        indirect_selection: &IndirectSelection,
    ) -> Result<(DirectNodes, IndirectNodes), SelectionError> {
        let mut direct_nodes = selected.clone();
        let mut indirect_nodes: HashSet<UniqueId> = HashSet::new();
        let selected_and_parents: HashSet<UniqueId> = HashSet::new();

        if *indirect_selection == Buildable {
            let selected_and_parents = self
                .graph
                .select_parents(selected, &None)?
                .union(&self.graph.sources)
                .map(|s| s.into())
                .collect();
            let selected_and_parents: HashSet<UniqueId> = selected
                .union(&selected_and_parents)
                .map(|s| s.into())
                .collect();
        }

        for unique_id in self.select_successors(selected) {
            match self
                .graph
                .node_map
                .get(&unique_id)
                .and_then(|node| IndirectSelection::can_select_indirectly(node).then_some(node))
            {
                None => {}
                Some(node) => {
                    match indirect_selection {
                        Eager /* TODO: | OR depends_on_nodes is subset of selected */ => {
                            direct_nodes.insert(unique_id);
                        },
                        Buildable /* TODO: | OR depends_on_nodes is subset of selected_and_parents */ => {
                            direct_nodes.insert(unique_id);
                        },
                        Cautious => {
                            indirect_nodes.insert(unique_id);
                        },
                        Empty => {},
                    }
                }
            }
        }

        Ok((direct_nodes, indirect_nodes))
    }
}

impl NodeSelector {
    pub fn _new(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Handle<Self>, SelectorCreateError> {
        NodeSelector::from(nodes, edges, None).and_then(|s| Ok(s.into()))
    }

    pub fn _update(
        &self,
        nodes: Vec<Node>,
        edges: Vec<Edge>,
    ) -> Result<Handle<Self>, SelectorCreateError> {
        let previous_state = PreviousState::from_graph(self.graph.clone());
        NodeSelector::from(nodes, edges, Some(Rc::new(previous_state))).and_then(|s| Ok(s.into()))
    }

    pub fn _select(&self, selector: String) -> Result<Vec<UniqueId>, SelectionError> {
        let selection_criteria = SelectionCriteria::from_single_raw_spec(selector)?;
        let selection_group = SelectionGroup::from_criteria(&selection_criteria);

        let selected_set: HashSet<String> = self.get_selected(&selection_group)?;

        Ok(selected_set.into_iter().collect())
    }

    pub fn _select_type(
        &self,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        let selection_criteria = SelectionCriteria::from_single_raw_spec(selector)?;
        let selection_group = SelectionGroup::from_criteria(&selection_criteria);

        let selected_set: HashSet<String> =
            self.get_selected_type(&selection_group, &resource_type_filter)?;

        Ok(selected_set.into_iter().collect())
    }

    pub fn _select_included(
        &self,
        included_nodes: Vec<UniqueId>,
        selector: String,
        resource_type_filter: ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        todo!()
        // let selected_set: HashSet<String> = self.select_type(selector, resource_type_filter)?;

        // Ok(selected_set.into_iter().collect())
    }
}

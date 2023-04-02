use crate::graph::UniqueId;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::graph::{node::GraphNode, parsed_graph::ParsedGraph};
use crate::selector::spec::{IndirectSelection, SelectionCriteria, SelectionGroup, SelectionSpec};

use crate::dbt_node_selector::{Edge, Node, ResourceTypeFilter, SelectionError, SelectorCreateError};

use crate::IndirectSelection::*;

pub struct NodeSelector {
    pub graph: Rc<ParsedGraph>,
    pub previous_state: Option<Rc<ParsedGraph>>,
}

type DirectNodes = HashSet<UniqueId>;
type IndirectNodes = HashSet<UniqueId>;

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
                additional.extend(self.graph.select_children(&selected, None)?);
            }
            if spec.parents {
                additional.extend(self.graph.select_parents(&selected, None)?);
            }
        }
        Ok(additional)
    }
}

impl NodeSelector {
    pub fn select_and_filter(
        &self,
        included_nodes: Option<HashSet<UniqueId>>,
        selector: &String,
        resource_type_filter: &ResourceTypeFilter,
    ) -> Result<Vec<UniqueId>, SelectionError> {
        let selection_criteria = SelectionCriteria::from_single_raw_spec(selector)?;
        let unfiltered_result = selection_criteria.method.search(
            &self.previous_state.clone(),
            &self.graph,
            &selection_criteria.value,
        )?;
        Ok(unfiltered_result
            .iter()
            .filter(|id| match &included_nodes {
                Some(included_nodes) => self.graph.is_node(id, &|n| {
                    included_nodes.contains(id.as_str())
                        && resource_type_filter.should_include(n.resource_type)
                }),
                None => self.graph.is_node(id, &|n| {
                    resource_type_filter.should_include(n.resource_type)
                }),
            })
            .map(|s| s.to_owned())
            .collect())
    }

    pub fn from(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Self, SelectorCreateError> {
        let mut node_map = HashMap::<UniqueId, GraphNode>::new();
        for node in nodes.iter() {
            node_map.insert(node.unique_id.to_owned(), GraphNode::from(node)?);
        }

        let mut parent_map = HashMap::<UniqueId, HashSet<UniqueId>>::new();
        for edge in edges.iter() {
            let mut parents = HashSet::<UniqueId>::new();
            parents.extend(edge.parents.to_owned());
            parent_map.insert(edge.unique_id.to_owned(), parents);
        }
        Ok(Self {
            graph: ParsedGraph::from_parents(node_map, parent_map).into(),
            previous_state: None.into(), //previous_state.and_then(|s| Some(s.graph.clone())),
        })
    }

    fn select_included(
        &self,
        included_nodes: &HashSet<UniqueId>,
        spec: &SelectionCriteria,
    ) -> Result<HashSet<UniqueId>, SelectionError> {
        let filtered_graph = &self.graph.filter(&included_nodes);
        let result = spec.method.search(&None, filtered_graph, &spec.value)?;
        Ok(HashSet::from_iter(result.iter().map(|s| s.to_owned())))
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
                    if selected.is_superset(&node.depends_on) {
                        selected.insert(unique_id.to_string());
                    }
                }
                Ok(selected)
            }
            Buildable => {
                let selected_and_parents: HashSet<String> =
                    self.graph.and_select_parents(&selected, None)?;
                for unique_id in indirect_nodes {
                    let Some(node) = self.graph.node_map.get(unique_id) else {
                        continue;
                    };
                    if selected_and_parents.is_superset(&node.depends_on) {
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
        match self.graph.node_map.get(unique_id) {
            None => Err(SelectionError::NodeNotInGraph(unique_id.to_string())),
            Some(node) => Ok(resource_type_filter.should_include(node.resource_type)),
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
        match &selection_group.selection_method {
            SelectionSpec::SelectionCriteria(spec) => self.get_nodes_from_criteria(&spec),
            _ => {
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

                let initial_direct = selection_group.combined(direct_sets);
                let indirect_nodes = selection_group.combined(indirect_sets);

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
            let selected_and_parents: HashSet<UniqueId> = self
                .graph
                .select_parents(selected, None)?
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

    // Return the subset of selected nodes that is a match for this selector.
    // fn filter_selection(&self, selected: HashSet<UniqueId>) -> HashSet<UniqueId> {

    // }
}

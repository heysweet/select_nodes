use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct InputArgs {
    /// A custom project-defined SelectionSpec
    pub selector: Option<String>,

    /// List of inclusion selectors
    pub select: Option<Vec<String>>,

    /// List of exclusion selectors
    pub exclude: Option<Vec<String>>,
}

pub struct ParsedArgs {
    /// A custom project-defined SelectionSpec
    pub selector: Option<SelectionSpec>,

    /// List of inclusion selectors
    pub select: Option<HashSet<UniqueId>>,

    /// List of exclusion selectors
    pub exclude: Option<HashSet<UniqueId>>,
}

pub use String as ArgName;

use crate::{
    dbt_node_selector::{SelectionError, UniqueId},
    selector::spec::*,
};

fn parse_union(
    components: &Vec<String>,
    expect_exists: bool,
    indirect_selection: IndirectSelection,
) -> Result<SelectionGroup, SelectionError> {
    let INTERSECTION_DELIMITER = ",";
    // Turn ['a b', 'c'] -> ['a', 'b', 'c']
    let raw_specs = components.iter().flat_map(|r| r.split(" "));

    let mut union_components: Vec<SelectionGroup> = vec![];

    // ['a', 'b', 'c,d'] -> union('a', 'b', intersection('c', 'd'))
    for raw_spec in raw_specs {
        let parts = raw_spec.split(INTERSECTION_DELIMITER);
        let a = parts.map(|part| {
            let selection_criteria =
                SelectionCriteria::from_single_spec(part.to_string(), &indirect_selection);
            selection_criteria.and_then(|selection_criteria| {
                Ok(SelectionGroup::from_criteria(&selection_criteria))
            })
        });
        let intersection_components: Result<Vec<SelectionGroup>, SelectionError> = a.collect();

        let intersection = SelectionGroup::intersection(
            raw_spec.to_owned(),
            intersection_components?,
            indirect_selection,
            expect_exists,
        );
        union_components.push(intersection);
    }
    let raw = components.join(" ");
    Ok(SelectionGroup::union(
        raw,
        union_components,
        indirect_selection,
        false,
    ))
}

fn parse_union_from_default(
    raw: &Option<Vec<String>>,
    default: &Vec<String>,
    indirect_selection: IndirectSelection,
) -> Result<SelectionGroup, SelectionError> {
    match raw {
        Some(raw) => parse_union(raw, true, indirect_selection),
        None => parse_union(default, false, indirect_selection),
    }
}

pub fn from_args(args: HashMap<ArgName, String>) -> InputArgs {
    let selector = args.get("selector");
    let selection_arg = args.get("select");
    let exclusion_arg = args.get("exclude");

    todo!()
}

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ParsedArgs {
    /// A custom project-defined SelectionSpec
    pub selector: Option<SelectionSpec>,

    /// List of inclusion selectors
    pub select: Option<Vec<String>>,

    /// List of exclusion selectors
    pub exclude: Option<Vec<String>>,
}

pub use String as ArgName;

use crate::selector::spec::*;

fn parse_union(components: &Vec<String>, expect_exists: bool, indirect_selection: IndirectSelection) -> SelectionSpec {
    todo!()
}

fn parse_union_from_default(raw: &Option<Vec<String>>, default: &Vec<String>, indirect_selection: IndirectSelection) -> SelectionSpec {
    match raw {
        Some(raw) => 
            parse_union(raw, true, indirect_selection),
        None => 
        parse_union(default, false, indirect_selection),
    }
}

pub fn from_args(args: HashMap<ArgName, String>) -> ParsedArgs {
    let selector = args.get("selector");
    let selection_arg = args.get("select");
    let exclusion_arg = args.get("exclude");

    todo!()
}

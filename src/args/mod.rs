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

use crate::selector::spec::SelectionSpec;

pub fn from_args(args: HashMap<ArgName, String>) -> ParsedArgs {
    let selector = args.get("selector");
    let selection_arg = args.get("select");
    let exclusion_arg = args.get("exclude");

    todo!()
}

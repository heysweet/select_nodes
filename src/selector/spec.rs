/// core/dbt/graph/selector_spec.py

use super::{MethodName, FQNMethod, FileMethod, PathMethod};
use regex::Regex;

lazy_static! {
    static ref RAW_SELECTOR_PATTERN: Regex = {
        // TODO: Is this a functional multiline regex?
        Regex::new(
            r"\A\
            (?P<childrens_parents>(\@))?\
            (?P<parents>((?P<parents_depth>(\d*))\+))?\
            ((?P<method>([\w.]+)):)?(?P<value>(.*?))\
            (?P<children>(\+(?P<children_depth>(\d*))))?\
            \Z"
        ).unwrap()
    };
}

pub enum IndirectSelection {
    Eager,
    Cautious,
    Buildable,
    Empty
}

/// Decide if the value is probably a path. Windows has two path separators, so
/// we should check both '\\' and '/' there.
fn _probably_path(value: &str) -> bool {
    if value.contains('/') {
        true
    } else { 
      value.contains(std::path::MAIN_SEPARATOR)
    }
}

struct SelectionCriteria {
    raw: String,
    method: MethodName,
    method_arguments: Vec<String>,
}

impl SelectionCriteria {
    fn split_on_method_separator(raw: &str) -> std::str::Split<&str> {
        raw.split(".")
    }

    fn default_method(value: &str) -> MethodName {
        let is_probably_path = _probably_path(value);
        let lowercase_value = value.to_lowercase();
        let is_relevant_filetype = lowercase_value.ends_with(".sql") || lowercase_value.ends_with(".py") || lowercase_value.ends_with(".csv");
        match (is_probably_path, is_relevant_filetype) {
            (true, _) => MethodName{Path: PathMethod{}},
            (_, true) => MethodName{File: FileMethod{}},
            _ => MethodName{FQN: FQNMethod{}},
        }
    }

    pub fn from_single_spec(&self, raw: &str, indirect_selection: &IndirectSelection) -> Result<SelectionCriteria, &str> {
        let result = RAW_SELECTOR_PATTERN.captures_iter(raw).peekable();
    
        if result.peek().is_some() {
            // TODO: IMPL!!
            todo!()
        }
        Err("Invalid selector spec")

    }
}


use std::collections::VecDeque;

/// core/dbt/graph/selector_spec.py

use super::{MethodName, FQNMethod, FileMethod, PathMethod};
use regex::{Regex, Captures};

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

// TODO: Use builder type to validate at compile time
struct SelectionCriteriaBuilder {
    raw: String,
    method: Option<MethodName>,
    value: Option<String>,
    childrens_parents: Option<bool>,
    parents: Option<bool>,
    parents_depth: Option<u64>,
    children: Option<bool>,
    children_depth: Option<u64>,
    // TODO: Default to Eager
    indirect_selection: Option<IndirectSelection>,
}

impl SelectionCriteriaBuilder {
    pub fn new(raw: &str) -> SelectionCriteriaBuilder {
        SelectionCriteriaBuilder{
            raw: raw.to_owned(),
            method: None,
            value: None,
            childrens_parents: None,
            parents: None,
            parents_depth: None,
            children: None,
            children_depth: None,
            indirect_selection: None,
        }
    }
}

struct SelectionCriteria {
    raw: String,
    method: MethodName,
    method_arguments: Vec<String>,
    value: String,
    childrens_parents: bool,
    parents: bool,
    parents_depth: u64,
    children: bool,
    children_depth: u64,
    // TODO: Default to Eager
    indirect_selection: IndirectSelection,
}

impl SelectionCriteria {
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

    fn from_strings(
        raw: &str, 
        method: Option<&str>,
        method_arguments: Vec<String>,
        value: &str,
        childrens_parents: &str,
        parents: &str,
        parents_depth: &str,
        children: &str,
        children_depth: &str,
        indirect_selection: &str
    ) -> SelectionCriteria {
        SelectionCriteria {
            raw: raw.to_owned(),
            method: method.to_owned(),
            method_arguments: method_arguments.to_owned(),
            value: value.to_owned(),
            childrens_parents: (),
            parents: bool::from(parents),
            parents_depth: (),
            children: (),
            children_depth: (),
            indirect_selection: ()
        }
    }

    fn from_captures(raw: &str, captures: Captures) -> SelectionCriteria {
        /// TODO: if Match, get value and pass it along
        /// if None, then get default value
        let childrens_parents = captures.name("childrens_parents");
        let parents = captures.name("parents");
        let parents_depth = captures.name("parents_depth");
        let method = captures.name("method");
        let value = captures.name("value");
        let children = captures.name("children");
        let children_depth = captures.name("children_depth");

        let mut builder = SelectionCriteriaBuilder::new(raw);

        SelectionCriteria {
            raw: raw.to_owned(),
            method: captures.name("method").un().as_str(),
            value: ,
            childrens_parents: captures.name("childrens_parents"),
            parents: captures.name("parents"),
            parents_depth: captures.name("parents_depth"),
            children: todo!(),
            children_depth: todo!(),
            indirect_selection: todo!()
        }
    }

    fn parse_method_args(method_args: &str) -> Vec<&str> {
        let mut result = method_args.split(".");
        let mut result: VecDeque<&str> = result.collect();
        result.pop_front();
        Vec::from(result)
    }

    pub fn from_single_spec(raw: &str, indirect_selection: &IndirectSelection) -> Result<SelectionCriteria, &str> {
        let result = RAW_SELECTOR_PATTERN.captures(raw);
    
        match result {
            Some(captures) => {
                Ok(SelectionCriteria::from_captures(raw, captures))
            },
            None => Err("Invalid selector spec")
        }
    }
}


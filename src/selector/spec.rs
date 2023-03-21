use std::collections::VecDeque;

/// core/dbt/graph/selector_spec.py

use super::{MethodName, FQNMethod, FileMethod, PathMethod};
use regex::{Regex, Captures, Match};

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

struct ParsedMethod {
    method_name: MethodName,
    method_arguments: Vec<String>,
}

impl ParsedMethod {
    pub fn from_captures(captures: Captures) -> ParsedMethod {
        let method_match = captures.name("method");
        let method = SelectionCriteria::get_str_from_match(method_match);
        let value_match = captures.name("value");
        let value = SelectionCriteria::get_str_from_match(value_match);

        match (method_match, value_match) {
            (None, _) => {
                ParsedMethod{
                    method_name: SelectionCriteria::default_method(v),
                    method_arguments: todo!()
                }
            },
            (_, _) => {
                ParsedMethod{
                    method_name: SelectionCriteria::default_method(v),
                    method_arguments: todo!()
                }
            }
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
    pub fn default_method(value: &str) -> MethodName {
        let is_probably_path = _probably_path(value);
        let lowercase_value = value.to_lowercase();
        let is_relevant_filetype = lowercase_value.ends_with(".sql") || lowercase_value.ends_with(".py") || lowercase_value.ends_with(".csv");
        match (is_probably_path, is_relevant_filetype) {
            (true, _) => MethodName{Path: PathMethod{}},
            (_, true) => MethodName{File: FileMethod{}},
            _ => MethodName{FQN: FQNMethod{}},
        }
    }

    pub fn get_str_from_match(regex_match: Option<Match>) -> &str {
        match regex_match {
            Some(r) => r.as_str(),
            None => "",
        }
    }

    fn from_captures(raw: &str, captures: Captures) -> SelectionCriteria {
        let parsed_method = ParsedMethod::from_captures(captures);
        
        let childrens_parents = captures.name("childrens_parents").is_some();
        let parents = captures.name("parents").is_some();
        let parents_depth = captures.name("parents_depth").get_or_insert("0");
        let method = Self::get_str_from_match(captures.name("method"));
        let value = Self::get_str_from_match(captures.name("value"));
        let children = captures.name("children").is_some();
        let children_depth = captures.name("children_depth");        

        SelectionCriteria {
            raw: raw.to_owned(),
            method: parsed_method.method_name,
            method_arguments: parsed_method.method_arguments,
            value: value.to_owned(),
            childrens_parents,
            parents,
            // TODO: Convert Some(Match) to num
            parents_depth,
            children,
            children_depth,
            indirect_selection: IndirectSelection::Eager,
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


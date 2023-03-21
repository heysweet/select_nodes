#[cfg(test)]
#[path = "spec_tests.rs"]
mod spec_tests;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Clone)]
struct ParsedMethod {
    method_name: MethodName,
    method_arguments: Vec<String>,
}

impl ParsedMethod {
    pub fn from_captures(captures: &Captures) -> Result<ParsedMethod, String> {
        let method_match = captures.name("method");
        let raw_method = SelectionCriteria::get_str_from_match(method_match);
        let value_match = captures.name("value");
        let value = SelectionCriteria::get_str_from_match(value_match);

        let mut method_parts = raw_method.split(".");
        let raw_method_name = method_parts.next();

        match (method_match, raw_method_name) {
            (None, _) => {
                Ok(ParsedMethod{
                    method_name: SelectionCriteria::default_method(value),
                    method_arguments: vec![],
                })
            },
            (_, Some(method_name)) => {
                let method_arguments: Vec<String> = method_parts.map(|s| s.to_string()).collect();
                let method_name = MethodName::from_string(method_name)
                    .ok_or(format!("'{}' is not a valid method name", method_name))?;
                Ok(ParsedMethod{method_name, method_arguments})
            },
            // Should not be possible
            (_, None) => Err("Matched empty method".to_string())
        }
    }
}

#[derive(Clone)]
pub struct SelectionCriteria {
    pub raw: String,
    pub method: MethodName,
    pub method_arguments: Vec<String>,
    pub value: String,
    pub childrens_parents: bool,
    pub parents: bool,
    pub parents_depth: u64,
    pub children: bool,
    pub children_depth: u64,
    // TODO: Default to Eager
    pub indirect_selection: IndirectSelection,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SelectionError {
    ParseIntError
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

    fn get_str_from_match(regex_match: Option<Match>) -> &str {
        match regex_match {
            Some(r) => r.as_str(),
            None => "",
        }
    }

    fn get_num_from_match(regex_match: Option<Match>) -> Result<u64, SelectionError> {
        match regex_match {
            Some(r) => {
                r.as_str().parse::<u64>().or_else(|_| Err(SelectionError::ParseIntError{ }))
            },
            None => Err(SelectionError::ParseIntError{ }),
        }
    }

    fn from_captures(raw: &str, captures: &Captures, indirect_selection: &IndirectSelection) -> Result<SelectionCriteria, String> {
        let parsed_method = ParsedMethod::from_captures(captures)?;
        
        let childrens_parents = captures.name("childrens_parents").is_some();
        let parents = captures.name("parents").is_some();
        let parents_depth = Self::get_num_from_match(captures.name("parents_depth")).unwrap_or_default();
        let method = Self::get_str_from_match(captures.name("method"));
        let value = Self::get_str_from_match(captures.name("value"));
        let children = captures.name("children").is_some();
        let children_depth = Self::get_num_from_match(captures.name("children_depth")).unwrap_or_default();        

        Ok(SelectionCriteria {
            raw: raw.to_owned(),
            method: parsed_method.method_name,
            method_arguments: parsed_method.method_arguments,
            value: value.to_owned(),
            childrens_parents,
            parents,
            parents_depth,
            children,
            children_depth,
            indirect_selection: indirect_selection.clone(),
        })
    }

    fn parse_method_args(method_args: &str) -> Vec<&str> {
        let mut result = method_args.split(".");
        let mut result: VecDeque<&str> = result.collect();
        result.pop_front();
        Vec::from(result)
    }

    pub fn from_single_raw_spec(raw: &str) -> Result<SelectionCriteria, String> {
        Self::from_single_spec(raw, &IndirectSelection::Eager)
    }

    pub fn from_single_spec(raw: &str, indirect_selection: &IndirectSelection) -> Result<SelectionCriteria, String> {
        let result = RAW_SELECTOR_PATTERN.captures(raw);
    
        match result {
            Some(captures) => {
                SelectionCriteria::from_captures(raw, &captures, indirect_selection)
            },
            None => Err("Invalid selector spec".to_string())
        }
    }
}


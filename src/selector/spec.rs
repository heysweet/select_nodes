#[cfg(test)]
#[path = "spec_tests.rs"]
mod spec_tests;

use indexmap::IndexMap;
use std::{collections::VecDeque, num::ParseIntError};

/// core/dbt/graph/selector_spec.py
use regex::{Captures, Match, Regex};

use super::MethodName;

lazy_static! {
    static ref RAW_SELECTOR_PATTERN: Regex = {
        // TODO: Is this a functional multiline regex?
        Regex::new(
"\\A\
(?P<childrens_parents>(@))?\
(?P<parents>((?P<parents_depth>(\\d*))\\+))?\
((?P<method>([\\w.]+)):)?(?P<value>(.*?))\
(?P<children>(\\+(?P<children_depth>(\\d*))))?\
\\z"
        ).unwrap()
    };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IndirectSelection {
    Eager,
    Cautious,
    Buildable,
    Empty,
}

impl IndirectSelection {
    pub fn from_string(raw: impl Into<String>) -> Result<Self, String> {
        let raw_string = raw.into();
        match raw_string.as_str() {
            "eager" => Ok(Self::Eager),
            "cautious" => Ok(Self::Cautious),
            "buildable" => Ok(Self::Buildable),
            "empty" => Ok(Self::Empty),
            _ => Err(format!("Invalid IndirectSelection '{}'", raw_string))
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Self::Eager => "eager",
            Self::Cautious => "cautious",
            Self::Buildable => "buildable",
            Self::Empty => "empty",
        }
    }
}

impl Default for IndirectSelection {
    fn default() -> Self {
        IndirectSelection::Eager
    }
}

#[derive(Clone)]
struct ParsedMethod {
    method_name: MethodName,
    method_arguments: Vec<String>,
}

impl ParsedMethod {
    fn default_method(value: impl Into<String>) -> Self {
        Self { method_name: MethodName::default_method(value), method_arguments: vec![] }
    }
}

impl ParsedMethod {
    fn parse_method_args(value: String, method: Option<String>) -> Result<ParsedMethod, String> {
        match method {
            None => Ok(Self::default_method(value)),
            Some(method) => {
                let result = method.split(".");
                let mut result: VecDeque<String> = result.map(|s| s.to_string()).collect();
                let raw_method_name = result.pop_front();
        
                match raw_method_name {
                    Some(raw_method_name) => {
                        let method_name = MethodName::from_string(&raw_method_name);
                        let method_name = method_name
                            .ok_or(format!("'{}' is not a valid method name", raw_method_name))?;
                        Ok(ParsedMethod{method_name, method_arguments: Vec::from(result)})
                    }
                    // Should not be possible
                    None => Err("Matched empty method".to_string()),
                }
            }
        }
        
    }

    pub fn from_captures(captures: &Captures) -> Result<ParsedMethod, String> {
        let method_match = captures.name("method");
        let raw_method = SelectionCriteria::get_str_from_match(method_match);
        let value_match = captures.name("value");
        let value = SelectionCriteria::get_str_from_match(value_match);

        let parsed_method = Self::parse_method_args(value.to_string(), Some(raw_method.to_string()));

        match (method_match, parsed_method) {
            (None, _) => Ok(Self::default_method(value)),
            (_, Err(e)) => Err(e),
            (_, Ok(parsed_method)) => Ok(parsed_method),
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
    pub parents_depth: Option<u64>,
    pub children: bool,
    pub children_depth: Option<u64>,
    // TODO: Default to Eager
    pub indirect_selection: IndirectSelection,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SelectionError {
    ParseIntError,
}

impl SelectionCriteria {
    fn get_str_from_match(regex_match: Option<Match>) -> &str {
        match regex_match {
            Some(r) => r.as_str(),
            None => "",
        }
    }

    fn get_num_from_match(regex_match: Option<Match>) -> Result<u64, SelectionError> {
        match regex_match {
            Some(r) => r
                .as_str()
                .parse::<u64>()
                .or_else(|_| Err(SelectionError::ParseIntError {})),
            None => Err(SelectionError::ParseIntError {}),
        }
    }

    fn from_captures(
        raw: &str,
        captures: &Captures,
        indirect_selection: &IndirectSelection,
    ) -> Result<Self, String> {
        let parsed_method = ParsedMethod::from_captures(captures)?;

        let childrens_parents = captures.name("childrens_parents").is_some();
        let parents = captures.name("parents").is_some();
        let parents_depth =
            Self::get_num_from_match(captures.name("parents_depth")).ok();
        let value = Self::get_str_from_match(captures.name("value"));
        let children = captures.name("children").is_some();
        let children_depth =
            Self::get_num_from_match(captures.name("children_depth")).ok();

        match children && childrens_parents {
            true => Err(format!("Invalid node spec '{}' - '@' prefix and '+' suffix are incompatible", raw)),
            false => Ok(Self {
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
    }

    pub fn from_single_raw_spec(raw: impl Into<String>) -> Result<Self, String> {
        Self::from_single_spec(raw, &IndirectSelection::default())
    }

    pub fn from_single_spec(
        raw: impl Into<String>,
        indirect_selection: &IndirectSelection,
    ) -> Result<Self, String> {
        let raw: String = raw.into();
        let result = RAW_SELECTOR_PATTERN.captures(&raw);

        match result {
            Some(captures) => Self::from_captures(&raw, &captures, indirect_selection),
            None => Err("Invalid selector spec".to_string()),
        }
    }

    fn _selection_criteria_with_value(raw: String, index_map: IndexMap<String, String>, indirect_selection: Option<impl Into<String>>, value: &String, parents_depth: Option<usize>, children_depth: Option<usize>) -> Result<Self, String> {
        // WARN! This is a dictionary in the python impl, we expect a string instead.
        let method_args = index_map.get("method_args");

        let default_indirect_selection = indirect_selection;
        let indirect_selection = index_map.get("indirect_selection");
        
        todo!()
        // match (method_args, parents_depth, children_depth, default_indirect_selection, indirect_selection) {
        //     (_, _, _, _, _) => 
        //     (Some(e), _, _, _, _) => Err(e),
        //     (_, _, _, _, _) => {
        //         ()
        //     }
        // }

        // Ok(Self{ raw: raw.into(), method: todo!(), method_arguments: todo!(), value: todo!(), childrens_parents: todo!(), parents: todo!(), parents_depth: todo!(), children: todo!(), children_depth: todo!(), indirect_selection: todo!() })
    }

    fn _match_to_int(raw_str: Option<&String>) -> Result<Option<usize>, ParseIntError> {
        match raw_str {
            None => Ok(None),
            Some(raw_str) => {
                let int: usize = raw_str.parse()?;
                Ok(Some(int))
            }
        }
    }

    pub fn selection_criteria_from_indexmap(raw: impl Into<String>, index_map: IndexMap<String, String>, indirect_selection: Option<impl Into<String>>) -> Result<Self, String> {
        let raw: String = raw.into();
        let value = index_map.get("value");

        let parents_depth = index_map.get("parents_depth");
        let parents_depth = Self::_match_to_int(parents_depth);
        let children_depth = index_map.get("children_depth");
        let children_depth = Self::_match_to_int(children_depth);

        match (value, parents_depth, children_depth) {
            (None, _, _) => Err(format!("Invalid node spec '{}'", raw)),
            (_, Err(_), _) => Err(format!("Invalid node spec '{}'", raw)),
            (_, _, Err(_)) => Err(format!("Invalid node spec '{}'", raw)),
            (Some(value), Ok(parents_depth), Ok(children_depth)) => Self::_selection_criteria_with_value(raw, index_map, indirect_selection, value, parents_depth, children_depth)
        }
    }
}

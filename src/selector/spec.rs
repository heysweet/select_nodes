#[cfg(test)]
#[path = "spec_tests.rs"]
mod spec_tests;

use std::collections::VecDeque;

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
    fn parse_method(method_args: impl Into<String>) -> Result<(MethodName, Vec<String>), String> {
        let method_args: String = method_args.into();
        let result = method_args.split(".");
        let mut result: VecDeque<String> = result.map(|s| s.to_string()).collect();
        let raw_method_name = result.pop_front();

        match raw_method_name {
            Some(raw_method_name) => {
                let method_name = MethodName::from_string(&raw_method_name);
                let method_name = method_name
                    .ok_or(format!("'{}' is not a valid method name", raw_method_name))?;
                Ok((method_name, Vec::from(result)))
            }
            // Should not be possible
            None => Err("Matched empty method".to_string()),
        }
    }

    pub fn from_captures(captures: &Captures) -> Result<ParsedMethod, String> {
        let method_match = captures.name("method");
        let raw_method = SelectionCriteria::get_str_from_match(method_match);
        let value_match = captures.name("value");
        let value = SelectionCriteria::get_str_from_match(value_match);

        let method = Self::parse_method(raw_method);

        match (method_match, method) {
            (None, _) => Ok(ParsedMethod {
                method_name: SelectionCriteria::default_method(value),
                method_arguments: vec![],
            }),
            (_, Err(e)) => Err(e),
            (_, Ok((method_name, method_arguments))) => Ok(ParsedMethod {
                method_name,
                method_arguments,
            }),
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
    pub fn default_method(value: impl Into<String>) -> MethodName {
        let value = value.into();
        let is_probably_path = _probably_path(&value);
        let lowercase_value = value.to_lowercase();
        let is_relevant_filetype = lowercase_value.ends_with(".sql")
            || lowercase_value.ends_with(".py")
            || lowercase_value.ends_with(".csv");
        match (is_probably_path, is_relevant_filetype) {
            (true, _) => MethodName::Path,
            (_, true) => MethodName::File,
            _ => MethodName::FQN,
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
    ) -> Result<SelectionCriteria, String> {
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
            false => Ok(SelectionCriteria {
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

    pub fn from_single_raw_spec(raw: impl Into<String>) -> Result<SelectionCriteria, String> {
        Self::from_single_spec(raw, &IndirectSelection::Eager)
    }

    pub fn from_single_spec(
        raw: impl Into<String>,
        indirect_selection: &IndirectSelection,
    ) -> Result<SelectionCriteria, String> {
        let raw: String = raw.into();
        let result = RAW_SELECTOR_PATTERN.captures(&raw);

        match result {
            Some(captures) => SelectionCriteria::from_captures(&raw, &captures, indirect_selection),
            None => Err("Invalid selector spec".to_string()),
        }
    }
}

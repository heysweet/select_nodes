#[cfg(test)]
#[path = "spec_tests.rs"]
mod spec_tests;

use indexmap::IndexMap;
use std::{collections::VecDeque, fmt::Display, num::ParseIntError, str::{FromStr, ParseBoolError}};

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
    pub fn from_string(raw: impl Into<String>) -> Result<Self, SelectionError> {
        let raw_string = raw.into();
        match raw_string.as_str() {
            "eager" => Ok(Self::Eager),
            "cautious" => Ok(Self::Cautious),
            "buildable" => Ok(Self::Buildable),
            "empty" => Ok(Self::Empty),
            _ => Err(InvalidIndirectSelectionError { input: raw_string }),
        }
    }

    /// Returns the Default value of IndirectSelection if `None`, or parses for a strict
    /// key match with any IndirectSelection.
    pub fn from_string_option(
        raw: Option<impl Into<String>>,
    ) -> Result<Option<Self>, SelectionError> {
        match raw {
            Some(raw) => Ok(Some(Self::from_string(raw)?)),
            None => Ok(None),
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
        Self {
            method_name: MethodName::default_method(value),
            method_arguments: vec![],
        }
    }
}

impl ParsedMethod {
    pub fn from_value_and_method(
        value: String,
        method: Option<String>,
    ) -> Result<ParsedMethod, SelectionError> {
        match method {
            None => Ok(Self::default_method(value)),
            Some(method) => {
                let result = method.split(".");
                let mut result: VecDeque<String> = result.map(|s| s.to_string()).collect();
                let raw_method_name = result.pop_front();

                match raw_method_name {
                    Some(raw_method_name) => {
                        let method_name = MethodName::from_string(&raw_method_name);
                        let method_name = method_name.ok_or(InvalidMethodError {
                            method_name: raw_method_name.to_string(),
                        })?;
                        Ok(ParsedMethod {
                            method_name,
                            method_arguments: Vec::from(result),
                        })
                    }
                    // Should not be possible
                    None => Err(MatchedEmptyMethodError {}),
                }
            }
        }
    }

    pub fn from_captures(captures: &Captures) -> Result<ParsedMethod, SelectionError> {
        let method_match = captures.name("method");
        let raw_method = SelectionCriteria::get_str_from_match(method_match);
        let value_match = captures.name("value");
        let value = SelectionCriteria::get_str_from_match(value_match);

        let parsed_method =
            Self::from_value_and_method(value.to_string(), Some(raw_method.to_string()));

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
    pub parents_depth: Option<usize>,
    pub children: bool,
    pub children_depth: Option<usize>,
    // TODO: Default to Eager
    pub indirect_selection: IndirectSelection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectionError {
    MissingValueError { input: String },
    ParentsDepthParseIntError { input: String, err: ParseIntError },
    ChildrensDepthParseIntError { input: String, err: ParseIntError },
    IncompatiblePrefixAndSuffixError { input: String },
    FailedRegexMatchError { input: String },
    InvalidMethodError { method_name: String },
    MatchedEmptyMethodError {},
    InvalidIndirectSelectionError { input: String },
    BoolInputError { key:String, input:String, err: ParseBoolError },
}

use SelectionError::*;

impl Display for SelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissingValueError { input } => {
                write!(f, "'{}' is not a valid method name", input)
            }
            ParentsDepthParseIntError { input, err } => {
                write!(f, "Failed to parse parents depth in '{}': {}", input, err)
            }
            ChildrensDepthParseIntError { input, err } => {
                write!(f, "Failed to parse childrens depth in '{}': {}", input, err)
            }
            InvalidMethodError { method_name } => {
                write!(f, "'{}' is not a valid method name", method_name)
            }
            IncompatiblePrefixAndSuffixError { input } => {
                write!(
                    f,
                    "Invalid node spec '{}' - '@' prefix and '+' suffix are incompatible",
                    input
                )
            }
            FailedRegexMatchError { input } => {
                write!(f, "Failed to match regex for '{}'", input)
            }
            MatchedEmptyMethodError {} => {
                write!(f, "Matched empty method name")
            }
            InvalidIndirectSelectionError { input } => {
                write!(f, "Invalid IndirectSelection input '{}'", input)
            }
            BoolInputError { input, err, key } => {
                write!(f, "'{}' field was provided and was not string literal `true` or `false`", key)
            },
        }
    }
}

impl SelectionCriteria {
    fn get_str_from_match(regex_match: Option<Match>) -> &str {
        match regex_match {
            Some(r) => r.as_str(),
            None => "",
        }
    }

    fn get_num_from_match(regex_match: Option<Match>) -> Result<Option<usize>, ParseIntError> {
        match regex_match {
            Some(r) => {
                let num = r.as_str().parse::<usize>()?;
                Ok(Some(num))
            }
            None => Ok(None),
        }
    }

    fn from_captures(
        raw: &str,
        captures: &Captures,
        indirect_selection: &IndirectSelection,
    ) -> Result<Self, SelectionError> {
        let parsed_method = ParsedMethod::from_captures(captures)?;

        let childrens_parents = captures.name("childrens_parents").is_some();
        let parents = captures.name("parents").is_some();
        let parents_depth = Self::get_num_from_match(captures.name("parents_depth"));
        let value = Self::get_str_from_match(captures.name("value"));
        let children = captures.name("children").is_some();
        let children_depth = Self::get_num_from_match(captures.name("children_depth"));

        match (children && childrens_parents, parents_depth, children_depth) {
            (true, _, _) => Err(IncompatiblePrefixAndSuffixError {
                input: raw.to_string(),
            }),
            (_, Err(err), _) => Err(ParentsDepthParseIntError {
                input: raw.to_string(),
                err,
            }),
            (_, _, Err(err)) => Err(ChildrensDepthParseIntError {
                input: raw.to_string(),
                err,
            }),
            (false, Ok(parents_depth), Ok(children_depth)) => Ok(Self {
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
            }),
        }
    }

    pub fn from_single_raw_spec(raw: impl Into<String>) -> Result<Self, SelectionError> {
        Self::from_single_spec(raw, &IndirectSelection::default())
    }

    pub fn from_single_spec(
        raw: impl Into<String>,
        indirect_selection: &IndirectSelection,
    ) -> Result<Self, SelectionError> {
        let raw: String = raw.into();
        let result = RAW_SELECTOR_PATTERN.captures(&raw);

        match result {
            Some(captures) => Self::from_captures(&raw, &captures, indirect_selection),
            None => Err(FailedRegexMatchError {
                input: raw.to_string(),
            }),
        }
    }

    fn _match_to_int(raw_str: Option<&String>) -> Result<Option<usize>, ParseIntError> {
        match raw_str {
            None => Ok(None),
            Some(raw_str) => {
                let int = raw_str.parse::<usize>()?;
                Ok(Some(int))
            }
        }
    }

    fn parse_parents_depth(raw_str: Option<&String>) -> Result<Option<usize>, SelectionError> {
        match (Self::_match_to_int(raw_str), raw_str) {
            (Ok(depth), _) => Ok(depth),
            (_, None) => Ok(None),
            (Err(err), Some(raw_str)) => Err(ParentsDepthParseIntError {
                input: raw_str.to_string(),
                err,
            }),
        }
    }

    fn parse_childrens_depth(raw_str: Option<&String>) -> Result<Option<usize>, SelectionError> {
        match (Self::_match_to_int(raw_str), raw_str) {
            (Ok(depth), _) => Ok(depth),
            (_, None) => Ok(None),
            (Err(err), Some(raw_str)) => Err(ChildrensDepthParseIntError {
                input: raw_str.to_string(),
                err,
            }),
        }
    }

    fn _get_optional_bool(key: &str, str: Option<&String>) -> Result<Option<bool>, SelectionError> {
        match str {
            None => Ok(None),
            Some(str) => {
                let bool_from_str = bool::from_str(str);
                match bool_from_str {
                    Ok(parsed_bool) => Ok(Some(parsed_bool)),
                    Err(err) => Err(BoolInputError{ key: key.to_string(), input: str.clone(), err }),
                }
            },
        }
    }

    /// `default_indirect_selection` is only used if 'indirect_selection' not found in index_map
    pub fn selection_criteria_from_indexmap(
        raw: impl Into<String>,
        index_map: &IndexMap<String, String>,
        default_indirect_selection: Option<IndirectSelection>,
    ) -> Result<Self, SelectionError> {
        let raw: String = raw.into();
        let value = index_map.get("value");

        let parents_depth = index_map.get("parents_depth");
        let parents_depth = Self::parse_parents_depth(parents_depth)?;
        let children_depth = index_map.get("children_depth");
        let children_depth = Self::parse_childrens_depth(children_depth)?;

        match value {
            None => Err(MissingValueError {
                input: raw.to_string(),
            }),
            Some(value) => {
                // WARN! This is a dictionary in the python impl, we expect a string instead.
                let method_args = index_map.get("method_args");
                let method =
                    ParsedMethod::from_value_and_method(value.to_string(), method_args.cloned())?;

                let default_indirect_selection = default_indirect_selection.unwrap_or_default();
                let indirect_selection = index_map.get("indirect_selection");
                let indirect_selection =
                    IndirectSelection::from_string_option(indirect_selection)?.unwrap_or(default_indirect_selection);

                let childrens_parents = Self::_get_optional_bool("childrens_parents", index_map.get("childrens_parents"))?.unwrap_or_default();
                let parents = Self::_get_optional_bool("parents", index_map.get("parents"))?.unwrap_or_default();
                let children = Self::_get_optional_bool("children", index_map.get("children"))?.unwrap_or_default();

                Ok(Self {
                    raw: raw.into(),
                    method: method.method_name,
                    method_arguments: method.method_arguments,
                    value: value.to_string(),
                    childrens_parents,
                    parents,
                    parents_depth,
                    children,
                    children_depth,
                    indirect_selection,
                })
            }
        }
    }
}

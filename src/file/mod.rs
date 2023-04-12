use std::{path::Path, io};

use normpath::PathExt;

pub enum MatchError {
    IO(io::Error),
    Regex(fnmatch_regex::error::Error)
}

pub fn fnmatch(name: &Path, pattern: &Path) -> Result<bool, MatchError> {
    let name = name.normalize();
    let pattern = pattern.normalize();
    match (name, pattern) {
        (Ok(name), Ok(pattern)) => {
            fnmatchcase(name.as_path(), pattern.as_path())
        },
        (Err(e), _) => Err(MatchError::IO(e)),
        (_, Err(e)) => Err(MatchError::IO(e))
    }
}

pub fn fnmatchcase(name: &Path, pattern: &Path) -> Result<bool, MatchError> {
    let Some(pattern) = pattern.to_str() else { return Ok(false) };
    let Some(name) = name.to_str() else { return Ok(false) };
    let regex = fnmatch_regex::glob_to_regex(pattern);
    match regex {
        Ok(regex) => Ok(regex.is_match(name)),
        Err(err) => Err(MatchError::Regex(err)),
    }
}
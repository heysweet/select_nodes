/// TODO: Somebody from core should help define the correct
/// behavior here.
///
///  NOTE: This implementation _always_ lowercases file paths
/// and assumes two files are the same if they would match in lowercase,
/// an assumption that dbt-core does not allow 2 project constructs
/// to have the same path but in a different case.
///
/// This assumption may be wrong and need to change, but this _is_ true
/// for UNIX, and I figured consistent behavior across OS's is more valuable
/// than allowing a dbt project to behave differently in different OS's.
///
/// The only normalization we perform is to lowercase.
pub fn fnmatch_normalized(
    name: impl Into<String>,
    pattern: impl Into<String>,
) -> Result<bool, fnmatch_regex::error::Error> {
    let name = name.into().to_lowercase();
    let pattern = pattern.into().to_lowercase();

    fnmatchcase(&name, &pattern)
}

pub fn fnmatchcase(name: &String, pattern: &String) -> Result<bool, fnmatch_regex::error::Error> {
    let regex = fnmatch_regex::glob_to_regex(pattern)?;
    Ok(regex.is_match(name))
}

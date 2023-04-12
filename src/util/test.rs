use std::collections::HashSet;

/// Often times, we have a Vec and we really don't care about the order
pub fn vec_to_set(vec: Vec<impl Into<String>>) -> HashSet<String> {
    vec.into_iter().map(|s| s.into()).collect()
}

/// Asserts option is Some and returns HashSet<String>
pub fn assert_vec_to_set(vec: Option<Vec<impl Into<String>>>) -> HashSet<String> {
    assert!(vec.is_some());
    vec.unwrap().into_iter().map(|s| s.into()).collect()
}

/// Asserts option is Some and returns HashSet<String>
pub fn assert_hashset(vec: Option<HashSet<String>>) -> HashSet<String> {
    assert!(vec.is_some());
    vec.unwrap()
}

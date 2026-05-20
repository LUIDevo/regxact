use std::collections::HashSet;
use crate::regex_tree::RegexTree;

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    pub pattern: String,
    pub tree: RegexTree,
    pub allows: HashSet<String>,
    pub contract: Option<String>,// Todo: turn into a contract type
}


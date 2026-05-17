use std::collections::HashSet;
use std::collections::regex_tree;

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    pub pattern: String,
    pub tree: RegexTree,
    pub allows: HashSet<String>,
    pub contract: Option<String>,
}


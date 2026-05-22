use std::collections::HashSet;
use crate::regex_tree::RegexTree;
use crate::error::RegxactError;
use crate::allow::Allow;

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    pub pattern: String,
    pub tree: RegexTree,
    pub allows: HashSet<Allow>,
    pub contract: Option<String>,// Todo: turn into a contract type
}

impl Pattern {
    pub fn allow(mut self, allow: &str)-> Result<Self, RegxactError>{
        let allow = match allow{
            "exponentional"=>Allow::Exponential,
            "multiline"=>Allow::MultiLine,
            "dotall"=>Allow::DotAll,
            "wildcard"=>Allow::Wildcard,
            _=>return Err(RegxactError::UnknownAllow(allow.to_string())),
        };
        self.allows.insert(allow);
        Ok(self)
    }
}

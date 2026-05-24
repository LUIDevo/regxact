use std::collections::HashSet;
use crate::regex_tree::RegexTree;
use crate::error::RegxactError;
use crate::allow::Allow;
use crate::rx;

#[derive(Debug, PartialEq, Eq)]
pub struct Rx {
    pub pattern: String,
    pub tree: RegexTree,
    pub allows: HashSet<Allow>,
    pub contract: Option<String>,// Todo: turn into a contract type
}

impl Rx {
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
    pub fn email() -> Result<Self, RegxactError> {
        rx!("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
    }
    pub fn ipv4() -> Result<Self, RegxactError> {
        rx!(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$")
    }
    pub fn ipv6() -> Result<Self, RegxactError> {
        rx!("^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$")
    }
}

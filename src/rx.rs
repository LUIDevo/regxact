use std::collections::HashSet;
use regex;
use crate::regex_tree::RegexTree;
use crate::regex_tree::AnchorKind;
use crate::error::RegxactError;
use crate::allow::Allow;
use crate::rx;

#[derive(Debug, PartialEq, Eq)]
pub struct Rx {
    pub(crate) pattern: String,
    pub(crate) tree: RegexTree,
    pub(crate) allows: HashSet<Allow>,
    pub(crate) contract: Option<String>,// Todo: turn into a contract type
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
    // pub fn test(){
    // } //TODO: HERE
    pub fn unanchored(mut self)->Result<Self, RegxactError>{
        self.pattern=strip_anchors_string(&mut self.pattern);
        strip_anchors_tree(&mut self.tree);
        Ok(self)
    }
    pub fn email() -> Rx{
        rx!("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$").unwrap()
    }
    pub fn ipv4() -> Result<Self, RegxactError> {
        rx!(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$") 
    }
    pub fn ipv6() -> Result<Self, RegxactError> {
        rx!("^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$")
    }
    pub fn slug() -> Result<Self, RegxactError> {
        rx!("^[a-z0-9]+(?:-[a-z0-9]+)*$")
    }
    pub fn uuid() -> Result<Self, RegxactError> {
        rx!("^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
    }
    pub fn jwt() -> Result<Self, RegxactError> {
        rx!(r"^[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+$")
    }
    pub fn hex() -> Result<Self, RegxactError> {
        rx!("^#(?:[0-9a-fA-F]{6}|[0-9a-fA-F]{3})$")
    }
    pub fn versioning() -> Result<Self, RegxactError> {
        rx!(r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$")
    }
    pub fn iso_8601_date() -> Result<Self, RegxactError> {
        rx!("^^[0-9]{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12][0-9]|3[01])$")
    }
    pub fn time() -> Result<Self, RegxactError> {
        rx!("^(?:[01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$")
    }
    pub fn test(&self, input: &str)->Result<bool, RegxactError>{
        check_anchor(&self.pattern)?;
        let re = regex::Regex::new(&self.pattern).unwrap();
        Ok(re.is_match(input))
    }
}

fn strip_anchors_string(pattern: &mut str)->String{
    pattern.trim_start_matches('^').trim_end_matches('$').to_string()
}

fn strip_anchors_tree(tree: &mut RegexTree){
    let nodes=tree.nodes_mut();
    if nodes.first() == Some(&RegexTree::Anchor(AnchorKind::Start)) {
        nodes.remove(0);
    }
    if nodes.last() == Some(&RegexTree::Anchor(AnchorKind::End)) {
        nodes.pop();
    }
}

fn check_anchor(pattern: &String)->Result<(), RegxactError>{
    let mut chars = pattern.chars();
    match chars.nth(0)==Some('^') && chars.last()==Some('$'){
        true=>Ok(()),
        false=>Err(RegxactError::Test(crate::error::TestError::UnAnchored)),
    }
}

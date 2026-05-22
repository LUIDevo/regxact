use crate::parser::parse;
use crate::pattern::Rx;
use crate::error::RegxactError;
use crate::analysis::performance::check_performance;
use crate::analysis::character_classes::check_character_classes;
use crate::allow::Allow;
use std::collections::HashSet;

pub struct RegxactBuilder {
    pattern: String,
    allows: HashSet<Allow>,
    contract: Option<String>,
}

impl RegxactBuilder {
    pub fn new(pattern: &str)->Self{
        RegxactBuilder{
            pattern:pattern.to_string(),
            allows: HashSet::new(),
            contract: None,
        }
    }
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
    pub fn contract(mut self, contract: &str)->Self{
        self.contract=Some(contract.to_string());
        self
    }
    pub fn build(self)->Result<Rx, RegxactError>{
        let tree=parse(&self.pattern);

        //checks and validations
        check_performance(&tree, &self.allows)?; //nested, overlapping quantifier
        check_character_classes(&tree, &self.allows)?;
        //contract check, any contradictions if contract exists
        
        Ok(Rx{pattern: self.pattern, tree, allows: self.allows, contract: self.contract})
    }
}

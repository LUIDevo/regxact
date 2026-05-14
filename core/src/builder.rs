use crate::parser::parse;
use crate::parser::RegexTree;
use crate::error::RegxactError;
use crate::analysis::performance::check_performance;
use std::collections::HashSet;

pub struct Pattern {
    pattern: String,
    tree: RegexTree,
    allows: HashSet<String>,
    contract: Option<String>,
}

pub struct RegxactBuilder {
    pattern: String,
    allows: HashSet<String>,
    contract: Option<String>,
}

//TODO: build

impl RegxactBuilder {
    pub fn new(pattern: &str)->Self{
        RegxactBuilder{
            pattern:pattern.to_string(),
            allows: HashSet::new(),
            contract: None,
        }
    }
    pub fn allow(mut self, flag: &str)->Self{
        self.allows.insert(flag.to_string());
        self
    }
    pub fn contract(mut self, contract: &str)->Self{
        self.contract=Some(contract.to_string());
        self
    }
    pub fn build(self)->Result<Pattern, RegxactError>{
        let tree=parse(&self.pattern);
        //checks and validations
        check_performance(&tree, &self.allows)?; //nested, overlapping quantifier
        // check_character_classes(&tree, &self.allows)?; //dot, \w, etc.
        // check_length(&tree, &self.allows, &self.contract)?;
        // check_flags(&tree, &self.allows)?;

        //contract check, any contradictions if contract exists

        Ok(Pattern{pattern: self.pattern, tree, allows: self.allows, contract: self.contract})
    }
}


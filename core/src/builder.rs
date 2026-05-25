use crate::parser::parse;
use crate::rx::Rx;
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
    pub fn allow(mut self, allow: &str) -> Self {
        let list: Vec<&str> = allow.split(',').map(|s| s.trim()).collect();
        println!("{:?}", list);
        for item in list{
            let accepted = match item {
                "exponential" => Allow::Exponential,
                "multiline" => Allow::MultiLine,
                "dotall" => Allow::DotAll,
                "wildcard" => Allow::Wildcard,
                _ => panic!("unknown allow flag: {}", item),
            };
            self.allows.insert(accepted);
        }
        self
    }
    pub fn contract(mut self, contract: &str)->Self{
        self.contract=Some(contract.to_string());
        self
    }
    pub fn build(self)->Result<Rx, RegxactError>{
        let tree=parse(&self.pattern);

        //checks and validations
        check_performance(&tree, &self.allows)?;
        check_character_classes(&tree, &self.allows)?;
        //contract check, any contradictions if contract exists
        
        Ok(Rx{pattern: self.pattern, tree, allows: self.allows, contract: self.contract})
    }
}

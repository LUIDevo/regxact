use crate::regex_tree::RegexTree;
use crate::error::*;
use std::collections::HashSet;
use crate::allow::Allow;

// a++        — repeat inside repeat with same node, redundant
// (a|a)      — duplicate branches in alternation
// (a|b|a)    — same
// (a|ab)    — same

pub fn check_node(node: &RegexTree, inside_repeat: bool, allows: &HashSet<Allow>)->Result<(), RegxactError>{
    match node {
        RegexTree::Repeat{ node: inner_node, min, max} => {
            if inside_repeat && !allows.contains(&Allow::Exponential){
                return Err(RegxactError::Performance(PerformanceError::NestedQuantifier));
            }
            check_node(inner_node, true, allows)
        }
        RegexTree::Group{ node: inner_node, ..}=>{
            check_node(inner_node, inside_repeat, allows)
        }
        RegexTree::Alternation(inner_nodes)=>{
            let mut duplication=HashSet::new();
            for inner_node in inner_nodes{
                if !duplication.insert(inner_node) && !allows.contains(&Allow::MultiLine){
                    return Err(RegxactError::Performance(PerformanceError::DuplicateAlternation));
                }
                check_node(inner_node, inside_repeat, allows)?;
            }
            Ok(())
        }
        RegexTree::Sequence(inner_nodes)=>{
            for inner_node in inner_nodes{
                check_node(inner_node, inside_repeat, allows)?;
            }
            Ok(())
        }
        _=>Ok(())
    }
}

pub fn check_performance(tree: &RegexTree, allows: &HashSet<Allow>)->Result<(), RegxactError>{
    for node in tree.nodes(){
        check_node(node, false, allows)?;
    }
    Ok(())
} //TODO: add it so allows does something

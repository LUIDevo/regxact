use crate::regex_tree::RegexTree;
use crate::error::*;
use std::collections::HashSet;

// a++        — repeat inside repeat with same node, redundant
// (a|a)      — duplicate branches in alternation
// (a|b|a)    — same
// (a|ab)    — same

pub fn check_node(node: &RegexTree, inside_repeat: bool)->Result<(), RegxactError>{
    match node {
        RegexTree::Repeat{ node: inner_node, min, max} => {
            if inside_repeat{
                return Err(RegxactError::Performance(PerformanceError::NestedQuantifier));
            }
            check_node(inner_node, true)
        }
        RegexTree::Group{ node: inner_node, ..}=>{
            check_node(inner_node, inside_repeat)
        }
        RegexTree::Alternation(inner_nodes)=>{
            let mut duplication=HashSet::new();
            for inner_node in inner_nodes{
                if !duplication.insert(inner_node){
                    return Err(RegxactError::Performance(PerformanceError::DuplicateAlternation));
                }
                check_node(inner_node, inside_repeat)?;
            }
            Ok(())
        }
        RegexTree::Sequence(inner_nodes)=>{
            for inner_node in inner_nodes{
                check_node(inner_node, inside_repeat)?;
            }
            Ok(())
        }
        _=>Ok(())
    }
}

pub fn check_performance(tree: &RegexTree, allows: &HashSet<String>)->Result<(), RegxactError>{
    for node in tree.nodes(){
        check_node(node, false)?;
    }
    Ok(())
} //TODO: add it so allows does something

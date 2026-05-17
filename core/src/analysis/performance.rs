use crate::regex_tree::RegexTree;
use crate::error::*;
use std::collections::HashSet;

// a++        — repeat inside repeat with same node, redundant
// (a|a)      — duplicate branches in alternation
// (a|b|a)    — same
// a{1}       — repeat that does nothing
// (a)        — group with no quantifier and single literal, group is pointless

pub fn check_performance(tree: &RegexTree, allows: &HashSet<String>)->Result<(), RegxactError>{
    for node in tree.nodes(){
        match node{
            // a++
            RegexTree::Repeat{node: inner, min, max}=>{
                if matches!(inner.as_ref(), RegexTree::Repeat{..})&&allows.get("exponential").is_none(){
                    return Err(RegxactError::Performance(PerformanceError::NestedQuantifier));
                } else if *min==1&&*max==Some(1)&&allows.get("exponential").is_none(){ 
                    return Err(RegxactError::Performance(PerformanceError::UnneededRepeat));
                } else{()}
            }
            RegexTree::Group{node: inner, index, capturing}=>{
                if matches!(inner.as_ref(), RegexTree::Alternation{..}){
                    let buf: HashSet<&RegexTree>=inner.nodes().iter().collect();
                    if inner.nodes().len() < buf.len(){
                        // TODO: Rewrite for better error logs
                        return Err(RegxactError::Performance(PerformanceError::DuplicateAlternation));
                    }
                }
                if matches!(inner.as_ref(), RegexTree::Literal(_)){
                    return Err(RegxactError::Performance(PerformanceError::RedundantGroup));
                }
            }
            _=>()
        };
    }
    Ok(())
}

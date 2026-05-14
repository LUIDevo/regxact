use crate::parser::RegexTree;
use crate::error::*;

// a++        — repeat inside repeat with same node, redundant
// (a|a)      — duplicate branches in alternation
// (a|b|a)    — same
// a{1}       — repeat that does nothing
// (a)        — group with no quantifier and single literal, group is pointless

pub fn check_performance(tree: &RegexTree, allows: Vec<String>)->Result<(), RegxactError>{
    for node in tree.nodes(){
        match node{
            // a++
            RegexTree::Repeat{node: inner, min: _, max: _}=>{
                if matches!(inner.as_ref(), RegexTree::Repeat{..})&&allows{
                    return Err(RegxactError::Performance(PerformanceError::NestedQuantifier));
                } else{
                    ()
                }
            }
            _=>()
        };
    }
    Ok(())
}

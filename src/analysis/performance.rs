use crate::regex_tree::RegexTree;
use crate::error::*;
use std::collections::HashSet;
use crate::allow::Allow;

// a++        — repeat inside repeat with same node, redundant
// (a|a)      — duplicate branches in alternation
// (a|b|a)    — same
// (a|ab)    — same

// Strip Group / single-element Sequence wrappers so we can see whether a
// quantifier directly wraps another quantifier (the catastrophic case).
fn unwrap_singleton(node: &RegexTree) -> &RegexTree {
    match node {
        RegexTree::Group { node, .. } => unwrap_singleton(node),
        RegexTree::Sequence(nodes) if nodes.len() == 1 => unwrap_singleton(&nodes[0]),
        _ => node,
    }
}

// Flatten a branch into its sequence of elements so we can compare branches
// for equality / prefix overlap (e.g. `a` is a prefix of `ab`).
fn branch_elems(node: &RegexTree) -> Vec<&RegexTree> {
    match node {
        RegexTree::Sequence(nodes) => nodes.iter().collect(),
        other => vec![other],
    }
}

fn overlapping(a: &RegexTree, b: &RegexTree) -> bool {
    let ea = branch_elems(a);
    let eb = branch_elems(b);
    let n = ea.len().min(eb.len());
    ea[..n] == eb[..n]
}

pub fn check_node(node: &RegexTree, allows: &HashSet<Allow>)->Result<(), RegxactError>{
    match node {
        RegexTree::Repeat{ node: inner_node, min: _, max} => {
            // Only an unbounded quantifier directly wrapping another unbounded
            // quantifier causes exponential backtracking. A bounded outer (e.g.
            // `{7}`) or a separating token inside the group is safe.
            if max.is_none() && !allows.contains(&Allow::Exponential){
                if let RegexTree::Repeat{ max: None, .. } = unwrap_singleton(inner_node) {
                    return Err(RegxactError::Performance(PerformanceError::NestedQuantifier));
                }
            }
            check_node(inner_node, allows)
        }
        RegexTree::Group{ node: inner_node, ..}=>{
            check_node(inner_node, allows)
        }
        RegexTree::Alternation(inner_nodes)=>{
            if !allows.contains(&Allow::Exponential){
                for i in 0..inner_nodes.len(){
                    for j in (i+1)..inner_nodes.len(){
                        if overlapping(&inner_nodes[i], &inner_nodes[j]){
                            return Err(RegxactError::Performance(PerformanceError::DuplicateAlternation));
                        }
                    }
                }
            }
            for inner_node in inner_nodes{
                check_node(inner_node, allows)?;
            }
            Ok(())
        }
        RegexTree::Sequence(inner_nodes)=>{
            for inner_node in inner_nodes{
                check_node(inner_node, allows)?;
            }
            Ok(())
        }
        _=>Ok(())
    }
}

pub fn check_performance(tree: &RegexTree, allows: &HashSet<Allow>)->Result<(), RegxactError>{
    check_node(tree, allows)
}

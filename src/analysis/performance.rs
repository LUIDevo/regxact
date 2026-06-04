use crate::regex_tree::RegexTree;
use crate::error::*;
use std::collections::HashSet;
use crate::allow::Allow;

// ReDoS only bites a backtracking engine when an *unbounded* quantifier wraps an
// ambiguous body (something matchable two ways), AND a later "wall" can fail and
// force the engine to explore all the ambiguous paths. Two ambiguous shapes:
//
//   (a+)+      — nested unbounded quantifiers
//   (a|a)*     — duplicate / overlapping alternation under a quantifier
//
// A trailing open-ended catch-all (`.*` / `.+`) defuses it: it absorbs the rest
// of the input so nothing downstream can fail and trigger the backtrack. Hence
// `(a|a)*.*` is safe while `(a|a)*` (worst-case wall at the end) is not.

// Strip Group / single-element Sequence wrappers so we can see what a quantifier
// directly wraps.
fn unwrap_singleton(node: &RegexTree) -> &RegexTree {
    match node {
        RegexTree::Group { node, .. } => unwrap_singleton(node),
        RegexTree::Sequence(nodes) if nodes.len() == 1 => unwrap_singleton(&nodes[0]),
        _ => node,
    }
}

// Flatten a branch into its sequence of elements so branches can be compared for
// equality / prefix overlap (e.g. `a` is a prefix of `ab`).
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

// `.*` / `.+` — an unbounded repeat of the wildcard absorbs the remaining input.
fn is_catch_all(node: &RegexTree) -> bool {
    matches!(node, RegexTree::Repeat { node: inner, max: None, .. } if matches!(**inner, RegexTree::Wildcard))
}

// Does this subtree contain an alternation with overlapping branches anywhere?
fn has_overlapping_alternation(node: &RegexTree) -> bool {
    match node {
        RegexTree::Alternation(branches) => {
            for i in 0..branches.len() {
                for j in (i + 1)..branches.len() {
                    if overlapping(&branches[i], &branches[j]) {
                        return true;
                    }
                }
            }
            branches.iter().any(has_overlapping_alternation)
        }
        RegexTree::Group { node, .. } | RegexTree::Repeat { node, .. } => has_overlapping_alternation(node),
        RegexTree::Sequence(nodes) => nodes.iter().any(has_overlapping_alternation),
        _ => false,
    }
}

// `defused` means a trailing catch-all guarantees the input after this node is
// always consumed, so an ambiguous quantifier here cannot be forced to backtrack.
pub fn check_node(node: &RegexTree, allows: &HashSet<Allow>, defused: bool)->Result<(), RegxactError>{
    if allows.contains(&Allow::Exponential) {
        return Ok(());
    }
    match node {
        RegexTree::Repeat{ node: inner_node, max, ..} => {
            if max.is_none() && !defused {
                // (a+)+ — an unbounded quantifier directly wrapping another.
                if let RegexTree::Repeat{ max: None, .. } = unwrap_singleton(inner_node) {
                    return Err(RegxactError::Performance(PerformanceError::NestedQuantifier));
                }
                // (a|a)* — overlapping alternation branches under the quantifier.
                if has_overlapping_alternation(inner_node) {
                    return Err(RegxactError::Performance(PerformanceError::DuplicateAlternation));
                }
            }
            check_node(inner_node, allows, false)
        }
        RegexTree::Group{ node: inner_node, ..}=>{
            check_node(inner_node, allows, defused)
        }
        RegexTree::Sequence(inner_nodes)=>{
            for (i, inner_node) in inner_nodes.iter().enumerate(){
                let absorbed = defused || inner_nodes.get(i + 1).map_or(false, is_catch_all);
                check_node(inner_node, allows, absorbed)?;
            }
            Ok(())
        }
        RegexTree::Alternation(inner_nodes)=>{
            for inner_node in inner_nodes{
                check_node(inner_node, allows, false)?;
            }
            Ok(())
        }
        _=>Ok(())
    }
}

pub fn check_performance(tree: &RegexTree, allows: &HashSet<Allow>)->Result<(), RegxactError>{
    check_node(tree, allows, false)
}

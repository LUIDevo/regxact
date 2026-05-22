use crate::regex_tree::RegexTree;
use crate::error::*;
use std::collections::HashSet;
use crate::allow::Allow;

// \w \d \s without charset declaration
// Unescaped dot outside character class
// `/m` / `/s` / `/i` flags without explicit allow

pub fn check_node(node: &RegexTree, allows: &HashSet<Allow>)->Result<(), RegxactError>{
    match node {
        RegexTree::Wildcard=>{
            if !allows.contains(&Allow::Wildcard){
                return Err(RegxactError::CharacterClass(CharacterClassError::UnescapedDot));
            }
            Ok(())
        },
        RegexTree::Shorthand(c)=>{
            if *c=='m'{
                if !allows.contains(&Allow::MultiLine){ 
                    return Err(RegxactError::CharacterClass(CharacterClassError::MultiLine));
                }
                return Ok(());
            }
            if *c=='s'{
                if !allows.contains(&Allow::DotAll){
                    return Err(RegxactError::CharacterClass(CharacterClassError::DotAll));
                }
                return Ok(());
            }
            Ok(())
        },
        RegexTree::Sequence(inner_nodes)|RegexTree::Alternation(inner_nodes)=>{
            for inner_node in inner_nodes{
                check_node(inner_node, allows)?;
            }
            Ok(())
        },
        _=>Ok(())
    }
}

pub fn check_character_classes(tree: &RegexTree, allows: &HashSet<Allow>)->Result<(), RegxactError>{
    for node in tree.nodes(){
        check_node(node, allows)?;
    }
    Ok(())
}

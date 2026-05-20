use crate::regex_tree::RegexTree;
use crate::error::*;
use std::collections::HashSet;

// \w \d \s without charset declaration
// Unescaped dot outside character class
// No upper bound on pattern length (unbounded)
// Upper bound exceeds contract maximum
// `/m` / `/s` / `/i` flags without explicit allow

pub fn check_node(node: &RegexTree, allows: &HashSet<String>)->Result<(), RegxactError>{
    match node {
        RegexTree::Wildcard=>{
            if !allows.contains("wilcard"){
                return Err(RegxactError::CharacterClass(CharacterClassError::UnescapedDot));
            }
            Ok(())
        }
        RegexTree::Shorthand(c)=>{
            if c=='w' ||c=='s'||c=='d'{
                if !allows.contains("unicode"){ //BUG: THIS IS NOT THE PROPER IMPLEMENTATION, CHANGE TO CHARSET INSTEAD OF ALLOWS
                }
                Ok(())
            }
            Ok(())
        }
    }
}

pub fn check_character_classes(tree: &RegexTree, allows: &HashSet<String>)->Result<(), RegxactError>{
    for node in tree.nodes(){
        check_node(node, allows)?;
    }
    Ok(())
}

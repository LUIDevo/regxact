use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RegxactError{ 
    Performance(PerformanceError),
    CharacterClass(CharacterClassError),
}

#[derive(Debug, PartialEq)]
pub enum PerformanceError{
    NestedQuantifier,
    UnneededRepeat,
    DuplicateAlternation,
}

#[derive(Debug, PartialEq)]
pub enum CharacterClassError{
    UnescapedDot,
}

impl fmt::Display for RegxactError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegxactError::Performance(e) => write!(f, "Performance issue: {}", e),
            RegxactError::CharacterClass(e) => write!(f, "Character class issue: {}", e),
        }
    }
}

impl fmt::Display for PerformanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PerformanceError::NestedQuantifier => write!(f, "nested quantifier, try removing one of them"),
            PerformanceError::UnneededRepeat => write!(f, "unneeded repeat, just remove the brackets"),
            PerformanceError::DuplicateAlternation => write!(f, "duplicate alternation branch"),
        }
    }
}

impl fmt::Display for CharacterClassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CharacterClassError::UnescapedDot => write!(f, "declare wildcard to avoid error"),
        }
    }
}
impl std::error::Error for RegxactError {}

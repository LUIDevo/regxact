use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RegxactError{ 
    Performance(PerformanceError),
    CharacterClass(CharacterClassError),
    UnknownAllow(String),
    Test(TestError)
}

#[derive(Debug, PartialEq)]
pub enum PerformanceError{
    NestedQuantifier,
    DuplicateAlternation,
}

#[derive(Debug, PartialEq)]
pub enum CharacterClassError{
    UnescapedDot,
    MultiLine,
    DotAll,
}

#[derive(Debug, PartialEq)]
pub enum TestError{
    UnAnchored,
}

impl fmt::Display for RegxactError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegxactError::Performance(e) => write!(f, "Performance issue: {}", e),
            RegxactError::CharacterClass(e) => write!(f, "Character class issue: {}", e),
            RegxactError::UnknownAllow(e) => write!(f, "allow not valid: {}", e),
            RegxactError::Test(e) => write!(f, "Error during test: {}", e),
        }
    }
}

impl fmt::Display for PerformanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PerformanceError::NestedQuantifier => write!(f, "nested quantifier, try removing one of them"),
            PerformanceError::DuplicateAlternation => write!(f, "duplicate alternation branch"),
        }
    }
}

impl fmt::Display for CharacterClassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CharacterClassError::UnescapedDot => write!(f, "declare wildcard to avoid error"),
            CharacterClassError::MultiLine => write!(f, "declare multiline"),
            CharacterClassError::DotAll => write!(f, "Declare dotall in allow"),
        }
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestError::UnAnchored => write!(f, "missing anchors, add ^ and $"),
        }
    }
}

impl std::error::Error for RegxactError {}

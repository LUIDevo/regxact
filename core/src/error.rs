use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RegxactError{ 
    Performance(PerformanceError),
}

#[derive(Debug, PartialEq)]
pub enum PerformanceError{
    NestedQuantifier,
    UnneededRepeat,
    DuplicateAlternation,
    RedundantGroup,
}

impl fmt::Display for RegxactError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegxactError::Performance(e) => write!(f, "Performance issue: {}", e),
        }
    }
}

impl fmt::Display for PerformanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PerformanceError::NestedQuantifier => write!(f, "nested quantifier, try removing one of them"),
            PerformanceError::UnneededRepeat => write!(f, "unneeded repeat, just remove the brackets"),
            PerformanceError::RedundantGroup => write!(f, "redundant group"),
            PerformanceError::DuplicateAlternation => write!(f, "duplicate alternation branch"),
        }
    }
}

impl std::error::Error for RegxactError {}

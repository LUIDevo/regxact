use std::fmt;

pub type RegxactResult<T> = Result<T, RegxactError>;

/// Every variant maps to a specific failure philosophy rule.
/// regxact doesn't have generic errors — each one tells you
/// exactly what category of mistake was made and how to fix it.
#[derive(Debug)]
pub enum RegxactError {
    /// The regex itself doesn't parse. Nothing else matters.
    InvalidPattern(String),

    /// Pattern has catastrophic backtracking risk (nested quantifiers).
    /// Must be acknowledged with .use("exponential").
    UnsafePattern { reason: String, examples: Vec<String> },

    /// Pattern accepts/rejects strings outside its declared domain.
    /// Carries concrete examples so the user sees exactly what leaked.
    ContractViolation { domain: String, examples: Vec<String> },

    /// No anchors and no .use("partial") — regxact can't tell if
    /// you meant full match or substring match, so it refuses to guess.
    MissingIntent { reason: String },
}

impl fmt::Display for RegxactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for RegxactError {}

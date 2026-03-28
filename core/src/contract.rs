use crate::analysis::RegexAnalysis;
use crate::error::RegxactResult;
use serde::Serialize;

/// A domain contract is a set of positive/negative examples that define
/// what a regex _should_ accept. This is how regxact moves from
/// "regex compiles" to "regex does what you think it does."
pub struct DomainContract {
    pub name: String,
    pub positive: Vec<&'static str>,
    pub negative: Vec<&'static str>,
}

/// What comes back from running a contract — the user needs to see
/// exactly which strings leaked through or got wrongly rejected.
#[derive(Debug, Serialize)]
pub struct ContractResult {
    pub passed: bool,
    pub false_positives: Vec<String>,
    pub false_negatives: Vec<String>,
}

impl DomainContract {
    /// Looks up a contract by name ("email", "url", "ipv4").
    /// Returns None for unknown names so the caller can produce a clear error.
    pub fn from_name(name: &str) -> Option<Self> {
        todo!()
    }

    /// Runs the regex against every positive and negative example.
    /// Returns concrete failing strings — regxact never says "wrong"
    /// without showing you what went wrong.
    pub fn validate(&self, analysis: &RegexAnalysis) -> RegxactResult<ContractResult> {
        todo!()
    }
}

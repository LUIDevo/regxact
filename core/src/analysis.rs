use crate::error::RegxactResult;
use serde::Serialize;

/// The result of statically analyzing a regex pattern.
/// This is regxact's core value — it extracts structural facts
/// (anchoring, quantifier nesting) that determine whether
/// the pattern is safe to use without explicit opt-ins.
#[derive(Debug, Serialize)]
pub struct RegexAnalysis {
    pub pattern: String,
    pub anchored_start: bool,
    pub anchored_end: bool,
    pub is_full_match: bool,
    pub has_quantifier_risk: bool,
}

impl RegexAnalysis {
    /// Parses and analyzes the pattern in one shot.
    /// Fails immediately if the pattern is syntactically invalid.
    /// Everything downstream depends on this — no RegexAnalysis, no regxact.
    pub fn new(pattern: &str) -> RegxactResult<Self> {
        todo!()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Detects nested quantifiers that cause catastrophic backtracking.
    /// e.g. (.+\s?)+ or (a+)+
    /// Without this, regxact can't enforce the "exponential requires opt-in" rule.
    fn detect_quantifier_risk(pattern: &str) -> bool {
        todo!()
    }
}

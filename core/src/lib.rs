mod analysis;
mod contract;
mod error;
mod macros;

use wasm_bindgen::prelude::*;

pub use analysis::RegexAnalysis;
pub use contract::DomainContract;
pub use error::{RegxactError, RegxactResult};
pub use macros::BuiltinMacro;

/// WASM entry point. Takes a raw pattern, returns JSON analysis.
/// This is the single gate everything passes through — if a regex
/// can't be statically proven safe, it fails here before any matching happens.
#[wasm_bindgen]
pub fn analyze(pattern: &str, _flags: &str) -> Result<String, JsError> {
    todo!()
}

/// Tests a pattern against a named domain contract (e.g. "email").
/// Exists because knowing a regex compiles isn't enough — we need to
/// prove it actually accepts/rejects the right strings for its domain.
#[wasm_bindgen]
pub fn validate_contract(pattern: &str, domain: &str) -> Result<String, JsError> {
    todo!()
}

/// Resolves a built-in macro name + args to a pre-verified pattern string.
/// Exists so users never hand-write common regexes (email, hex, uuid) —
/// the library owns those patterns and guarantees their correctness.
#[wasm_bindgen]
pub fn builtin_macro(name: &str, args: &str) -> Result<String, JsError> {
    todo!()
}

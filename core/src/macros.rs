/// A built-in macro is a pre-verified regex pattern for common domains.
/// Exists because most hand-written email/hex/uuid regexes are subtly wrong.
/// regxact owns these patterns, tests them against contracts, and
/// guarantees they do exactly what the name says.
pub struct BuiltinMacro {
    pattern: String,
}

impl BuiltinMacro {
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Maps a macro name + args to a known-good pattern.
    /// e.g. ("hex", ["6"]) → ^[a-fA-F0-9]{6}$
    /// Returns None for unknown names so the WASM layer can error clearly.
    pub fn resolve(name: &str, args: &[&str]) -> Option<Self> {
        todo!()
    }
}

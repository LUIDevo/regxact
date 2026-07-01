# Regxact

> **Archived** I stopped development 2026-06-27. 

Regxact started as a regex safety layer for Rust: macros for common patterns (email, UUID, IPv4/IPv6, JWT, etc.), a builder API, test safety checkers, and a static analyzer meant to catch catastrophic-backtracking (ReDoS) patterns before they ever ran.

## Why I built it

Regex is one of software's biggest footguns. Malformed regex patterns have taken down large systems, and most developers avoid writing regex by copying from Google/StackOverflow/AI without fully understanding what they pasted. The idea was to hand developers safe, pre-built pattern macros so they'd rarely touch raw regex, and to statically flag dangerous patterns for the cases where they did.

## Why I stopped

The core premise didn't make sense for Rust.

The ReDoS checker was one of the main points of the project. It detected nested quantifiers and duplicate alternations, but Rust's `regex` crate runs in **guaranteed linear time**. It is immune to ReDoS. So one of the largest aspects of the projects was redundant.

On top of that, the "safer regex through validation" niche already exists - e.g. [recheck](https://makenowjust-labs.github.io/recheck/) for JavaScript, where ReDoS *is* a real concern because JS engines backtrack.

The analyzer being pointless against the linear-time engine it wrapped, and other languages already having tooling — left no meaningful direction to take the project.

## What I built

- **`rx!` macro** — build a checked pattern, with optional `allow` overrides for flagged constructs.
- **Regex parser + tree** (`src/parser.rs`, `src/regex_tree.rs`) — My favorite part of the project, taking apart a regex string and breaking it down into language.
- **Builder API** (`src/builder.rs`) — construction, anchoring control (`unanchored()`) for testing.
- **Pattern macros** (`src/rx.rs`) — `Rx::email()`, `ipv4`, `ipv6`, `uuid`, `slug`, `hex_color`, `jwt`, `semver`, `date`, `time`.
- **Static analysis** (`src/analysis/`) — the (now redundant) backtracking heuristics.
- **Test Suite** (`src/test.rs/`) — Unit tests covering parser output, ReDoS detection, macro correctness and anchoring rules.

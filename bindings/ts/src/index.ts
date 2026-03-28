import * as wasm from "regxact-core";

export type UsageFlag = "partial" | "exponential";
export type DomainName = "email" | "url" | "ipv4";

interface AnalysisResult {
  pattern: string;
  anchored_start: boolean;
  anchored_end: boolean;
  is_full_match: boolean;
  has_quantifier_risk: boolean;
}

interface ContractResult {
  passed: boolean;
  false_positives: string[];
  false_negatives: string[];
}

// Custom macro registry — lives in-process, populated by .macro() calls
const macroRegistry = new Map<string, string>();

class RegxactInstance {
  private pattern: string;
  private analysis: AnalysisResult;
  private flags: Set<UsageFlag> = new Set();

  // Constructor calls analyze() and immediately enforces safety rules.
  // If you get a RegxactInstance back, it's safe. If not, you got an error
  // with examples of what went wrong. There is no "unchecked" path.
  constructor(regex: RegExp | string) {
    throw new Error("todo");
  }

  // Declares explicit intent for unsafe or domain-specific behavior.
  // This is the opt-in mechanism — regxact is locked down by default,
  // and .use() is the only way to unlock partial matching,
  // exponential patterns, or domain contract validation.
  use(flag: UsageFlag | DomainName): this {
    throw new Error("todo");
  }

  // Registers this pattern as a reusable named macro.
  // Exists so teams can define domain-specific patterns once
  // (e.g. "uk-postcode") and reuse them with regxact.macro("uk-postcode").
  macro(name: string): this {
    throw new Error("todo");
  }

  // Tests pattern against a domain contract's positive/negative examples.
  // Throws with concrete failing strings — never a vague "invalid" message.
  private validateContract(domain: DomainName): this {
    throw new Error("todo");
  }

  // Applies the two core safety rules:
  // 1. Full match required unless "partial" declared
  // 2. Quantifier risk requires "exponential" declared
  // Called after construction and after each .use() so invariants
  // are always current.
  private enforce(): void {
    throw new Error("todo");
  }
}

// --- Public API surface ---

// Main entry. Wraps a regex and validates safety in one call.
export function regxact(regex: RegExp | string): RegxactInstance {
  throw new Error("todo");
}

// Built-in macros — pre-verified patterns the library owns.
// Users call these instead of writing common regexes by hand.

regxact.email = (): RegxactInstance => { throw new Error("todo"); };
regxact.alphanumeric = (min?: number, max?: number): RegxactInstance => { throw new Error("todo"); };
regxact.hex = (length?: number): RegxactInstance => { throw new Error("todo"); };
regxact.uuid = (): RegxactInstance => { throw new Error("todo"); };

// Retrieves a previously registered custom macro by name.
regxact.macro = (name: string): RegxactInstance => { throw new Error("todo"); };

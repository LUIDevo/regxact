# Regxact
Regxact is a multi-language regex safety layer, with macros for common use cases that developers can easily build off of.

## What it solves:
This simple regex pattern took down CrowdStrike, cost over x billion dollars to fix, and was completely invisible until it was too late.
Regex is one of software's most common footguns. A malformed pattern can bypass authentication, crash a server, or allow an attacker into your system with nothing in your toolchain to warn you. If billion dollar companies with dedicated security teams don't catch these in testing, there's no reason to assume you will either. 

The traditional developer approach to regex is to avoid writing it entirely, by googling, asking AI, or copying from StackOverflow. But it doesn't adress the core problem: Regex is hard to read, hard to write, and extremely easy to mess up.

Regxact fixes this by providing developers with macros that cover common use cases of regex, so you don't have to write or even see raw regex in your codebase. For everything else, Regxact allows you to write Regex with an additional safety checker, that can also be used to extend off of the macros. 

Experienced developers can adopt Regxact quickly. It gets out of your way and makes regex safer without slowing you down.

TODO:
- [x] finish checks
- [x] convert allow to an enum
- [x] Get allows working
- [x] Redesign the allows approach. Think whether how to approach allows with rx! vs Pattern
- [ ] Contract
- [ ] Builder pattern
- [ ] Contract Checker
- [ ] Finish test checks
- [ ] WASM integration
- [ ] Better error messaging

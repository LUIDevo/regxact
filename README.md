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
- [x] Fix reference (i dont know what I meant when I wrote this, im just going to asssume I did it)
- [x] Contract (email contract)
- [x] Builder pattern
- [x] add allows functionality
- [ ] Add other contracts
    - [x] ipv4
    - [x] ipv6
    - [x] slug
    - [x] uuid
    - [x] hex color
    - [x] jwt
    - [x] semantic verisoning
    - [x] date
    - [x] time
    - [ ] phone number (complicated)
    - [ ] postal/zip code (complicated)
    - [ ] filename/extension
    - [ ] file path
- [x] add multi allows functionality
- [x] Switch to fully index based system
- [x] Implement test, search, normal regex stuff
- [ ] Finish parsing errors
- [ ] Implement all tests into all modes
- [ ] Add ALOT more test checks, for each allow
- [ ] Better error messaging
- [ ] Complete README with proper writing
- [ ] validate that macros are correct and secure
- [ ] Make sure every error has a flag
- [ ] Make sure concept is right, if missing any additional checks (like exponential, but if i need more or less)
- [ ] Polish everything for v1 release
- [ ] Publish to crate, npm 

explain advantages of rust regex engine for security and linear time

Better way to handle dates
ReDOs

Rationale behind the rules
Try another language

Add contracts

Development has stopped, due to conflictions with the base "regex" crate which prevents ReDos, which diminishes the usage of the app.
I might make some more changes though heehe

Pivot to python or javascript/node depending on what gap I have in my resume
I think I will do node because most applications use it

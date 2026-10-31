# 0035. Front-end libraries and scope

Date: 2026-10-31
Status: Accepted. Amends 0029.

## Context

The clean-room front end exists to exercise the specification. It is
not meant to be a complete compiler. Decision 0029 limits it to the Rust
standard library, so all 13,679 of its lines are hand-written,
including the lexer, the parser, and exact arithmetic. Phases 3 and 4 of
the roadmap add name lookup, overload resolution, and templates, and
the code would grow with each.

A library helps only if it brings no C++ knowledge with it. A front end
that gets its C++ knowledge from a library passes tests because of that
library, and stops finding the places where Tidal's text is not enough.
Such a library is also an input that AGENTS.md rule 5 does not allow.

## Decision

- The front end can use any Rust crate that does not encode knowledge
  of C++. Examples are parser generators, lexer generators, exact
  integer and IEEE floating-point arithmetic, diagnostic rendering,
  command-line parsing, test harnesses, and general data structures.
- The front end must not use a crate that encodes C++ knowledge. The ban
  includes C++ grammars (such as tree-sitter's C++ grammar), bindings
  to a compiler (such as libclang), and crates that model C++ types,
  lookup, or overloads.
- A crate's license must be compatible with Tidal's license (Q-0009).
  `frontend/Cargo.toml` lists every dependency.
- When the parser moves to a generator, the generator reads a grammar
  written from Tidal's Syntax segments. The consolidated grammar of
  roadmap item F is the intended input.
- The front end follows the specification pages that are new or
  changed. It does not aim to cover the whole test suite. Constructs
  that it does not implement stay "unsupported".

## Consequences

- The semantic checks (types, constant evaluation, classes, and later
  lookup and overload resolution) stay hand-written, because they are
  the front end's reading of the text.
- Conformance numbers measure the pages that the front end covers. A
  falling share of the whole suite is expected as pages are added.
- The front-end prompt in `docs/exercises.md` states the crate rule.

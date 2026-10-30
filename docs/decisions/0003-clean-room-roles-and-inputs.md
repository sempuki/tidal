# 0003. Clean-room roles, input classes, and a blind similarity check

Date: 2026-10-30
Status: Accepted

## Context

The writer needs rich information about C++ semantics. The richest sources
(compiler source, compiler tests, reference sites) are full of quoted ISO
text. A classic clean room separates people who read the original from people
who write the replacement, and passes only facts between them.

## Decision

Adopt the protocol in `docs/clean-room.md`:

- Three roles: writer, checker, adjudicator.
- Three input classes: clean, sanitized (comments stripped by a tool),
  forbidden.
- Clang's `test/CXX/` tree, libc++'s `test/std/` tree, cppreference, and all
  WG21 papers are forbidden because they quote or closely paraphrase ISO text.
- The similarity tool prints only Tidal text, so its report can go back to
  the writer.

## Consequences

- Writers lose the best-organized test suites, because those are organized by
  ISO section. Tidal has to build its own tests from scratch.
- Comment stripping discards real explanations of intent in compiler code.
  Writers must infer intent from code and behavior.
- Staffing for the proof of concept is described in `docs/clean-room.md`.

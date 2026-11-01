# 0015. Recommended practice is a rule class

Date: 2026-10-30
Status: Accepted. Extends 0005.

## Context

Some language features exist mainly to drive diagnostics. `[[nodiscard]]`
asks for a warning when a result is ignored; `[[deprecated]]` asks for one
when a name is used. A program that triggers the warning is still valid,
and an implementation that never warns still conforms. None of the six
rule classes in decision 0005 describes "should, but need not".

## Decision

Add a seventh class, `recommended`. A `recommended` rule describes behavior
an implementation should provide, usually a diagnostic, without making any
program invalid. Its tests turn the expected warning into an error with
`tidal-flags` (for example `-Werror`), and show both the warning case and
the case the attribute silences. Compilers that don't take those flags skip
the tests.

## Consequences

- `tools/check_specification.py` accepts the new class.
- Tests for `recommended` rules depend on compiler warning flags, so they
  run on Clang and GCC only.

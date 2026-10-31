# 0038. Rules cited by a failed constant

Date: 2026-10-31
Status: Accepted. Extends 0034.

## Context

When a program needs a constant and the evaluation fails, two reject
rules apply:

- the context rule, which needs a constant, such as CX-1, CINIT-1, or a
  `static_assert` rule,
- the cause rule, which the evaluation broke, such as CE-22 or an
  `undefined` rule (CE-9).

Tests cite them inconsistently. A checker that names one rule then
matches some tests and not others. The clean-room front end found this
inconsistency (finding G7).

## Decision

- A test of a failed constant cites the cause rule.
- The test can also cite the context rule. A test about the context
  rule must cite it.
- A checker that names the cause rule, or both rules, matches the test.

## Consequences

- `docs/spec-format.md` states the convention.
- A test that cites only the context rule gets its cause rule when it
  is next changed.

# 0034. Prohibitions are reject rules

Date: 2026-10-31
Status: Accepted.

## Context

420 tests that expect a rejection cite only `required` rules: 175 rules
state a prohibition ("must not", "is not valid") inside a rule whose
class says nothing about diagnosing it. The clean-room front end could
not tell from a rule's class what to reject (F15), and the editor's
roadmap ranks this second (`docs/roadmap.md`, item C).

## Decision

- A test that expects `reject` cites at least one rule of a class that
  allows rejection: `reject`, `undefined` (rejected in constant
  evaluation), `recommended` (with a warning flag), `impl-defined`,
  `conditionally-supported`, `unspecified`, or `no-diagnostic`.
- A `required` rule states what a program can do and what it means. Its
  prohibitions move to a new `reject` rule on the same page, in the
  Constraints segment, cited by the same principle. A rule that holds
  only prohibitions changes class to `reject` instead.
- The reject tests move to the new rule. Tests that expect acceptance
  stay with the `required` rule.
- `tools/check_spec.py` fails on each test that breaks the first point.

## Consequences

- 175 rules were split or reclassed, and the rule count grew from 823
  to 983. A few reject tests moved to an existing reject rule instead,
  where the rejection already followed from it.
- Diagnose batches and the front end can read what to reject from the
  class of the rules a test cites.

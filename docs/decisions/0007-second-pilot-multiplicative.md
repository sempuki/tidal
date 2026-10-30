# 0007. Second pilot is the multiplicative operators

Date: 2026-10-30
Status: Accepted

## Context

The charter calls for one statement pilot and one expression pilot. The
first plan for the expression pilot was the logical operators (`&&`, `||`,
`!`). The `if` pilot produced only `required` and `reject` rules, so the
pipeline's handling of `undefined`, `unspecified`, and `impl-defined`
behavior is still untested. The logical operators are mostly `required` as
well.

## Decision

The second pilot covers `*`, `/`, and `%` on built-in types. They involve
division by zero, signed overflow, result types that depend on the target,
and operands with no fixed evaluation order.

## Consequences

- The pilot needs a second target (`-m32`) to show implementation-defined
  variation. Tests that need it carry `tidal-flags: -m32` and include no
  headers, so they compile without 32-bit libraries.
- GCC's UBSan runtime is not installed on the author's machine, so
  undefined-behavior evidence comes from constant evaluation and compiler
  warnings, both of which work on both compilers.
- Overloaded operators are out of scope and only referenced.

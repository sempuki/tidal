# 0007. Second pilot is the multiplicative operators

Date: 2026-10-30
Status: Accepted

## Context

The charter calls for one statement pilot and one expression pilot. The
`if` section has only `required` and `reject` rules. The expression pilot
needs to exercise `undefined`, `unspecified`, and `impl-defined` behavior.
The logical operators (`&&`, `||`, `!`) are mostly `required` and would not.

## Decision

The second pilot covers `*`, `/`, and `%` on built-in types. They involve
division by zero, signed overflow, result types that depend on the target,
and operands with no fixed evaluation order.

## Consequences

- The pilot needs a second target (`-m32`) to show implementation-defined
  variation. Tests that need it carry `tidal-flags: -m32` and include no
  headers, so they compile without 32-bit libraries.
- Undefined-behavior evidence comes from constant evaluation and compiler
  warnings, which work on every compiler in the matrix. The host GCC has no
  UBSan runtime, so sanitizer evidence needs the container images.
- Overloaded operators are out of scope and only referenced.

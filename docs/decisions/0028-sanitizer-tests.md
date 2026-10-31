# 0028. Sanitizer tests for undefined behavior

Date: 2026-10-31
Status: Accepted. Extends 0005.

## Context

Most `undefined` rules had only constant-evaluation tests. Those show
that a compiler treats a construct as undefined inside a constant
expression. They say nothing about a running program, and some rules
(an out-of-range enumeration value read later, a call through
`[[noreturn]]` that returns) have no useful constant form.

GCC and Clang both ship an undefined-behavior sanitizer (UBSan) and an
address sanitizer (ASan). With recovery turned off, the program stops at
the first error and exits nonzero, and the report names the error. ASan
finds the lifetime and heap errors that UBSan does not: a use after
`delete`, a double `delete`, a mismatched `delete`, and a read after a
block ends.

## Decision

- A sanitizer test expects `run-fail` and names its flags in
  `tidal-flags`, usually `-fsanitize=undefined
  -fno-sanitize-recover=undefined`. A check outside that group names
  its own flag (`float-cast-overflow`, `float-divide-by-zero`, `vptr`)
  with `-fno-sanitize-recover=all`. An ASan test uses
  `-fsanitize=address`, which stops at the first error by default.
- `tidal-error` also applies to `run-fail`. The pattern must match the
  sanitizer's report, so a crash for another reason fails the test.
- The program takes its operands from `argc` through `volatile`
  variables, so the compiler can't fold the operation at compile time.
- `unsupported_flags` in `tools/compilers.json` takes glob patterns.
  EDG is a front end only, and skips every `-fsanitize=*` test.

## Consequences

- 25 tests cover 21 undefined rules at run time: 18 with UBSan and 7
  with ASan.
- The host needs `libubsan`, `libasan`, and `compiler-rt`
  (`docs/setup.md`).
- `OUTCOME_VERSION` is 5, because a run failure now keeps the whole
  report.

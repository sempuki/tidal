# Q-0212: Does EDG letting a `dynamic_cast` from a private base succeed during constant evaluation change VIRT-10?

**Status:** compiler defect
**Affects:** VIRT-10, `tests/classes/virtual/dynamic-cast-private-base-constant.cpp`

## Context

A class `D` has a private base `A` and a public base `B`. A member
function of `D` returns a pointer to its `A` subobject. A `dynamic_cast`
of that pointer to `D*`, or to `B*`, follows the check of VIRT-10.

| Compiler | Run time | Constant evaluation |
|---|---|---|
| Clang 18, 22, host | null pointer | null pointer |
| GCC 14, 16, host | null pointer | null pointer |
| EDG 7.0 | no code generation | the cast succeeds |

A probe with the bases reversed gives the same split. There `D` has a
public base `A` and a private base `B`, and the casts go from the `B`
subobject to `D*` and to `A*`. Clang 22 and GCC 16 give a null pointer
for both casts, and EDG 7.0 lets both succeed.

## Evidence

`tests/classes/virtual/dynamic-cast-private-base-constant.cpp`, with an
override for EDG. The run tests `dynamic-cast-private-base.cpp`,
`dynamic-cast-cross-from-private-base.cpp`, and
`dynamic-cast-cross-to-private-base.cpp` give a null pointer on every
Clang and GCC configuration.

## Options

1. The check needs the operand's subobject to be a public base of the
   target object or of the most derived object (VIRT-10). EDG's success
   is a defect.
2. Constant evaluation ignores access in the check.

## Provisional answer

No; Clang and GCC give a null pointer, likely EDG defect (option 1). Clang and GCC agree at run time and in constant evaluation,
and the result of a constant must match the result at run time.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 give a null pointer, and EDG 7.0 lets the cast succeed, so two families disagree. CE-P1 supports the provisional answer. Constant evaluation is the same program run at translation time, so the cast gives the null pointer that Clang and GCC give at run time. The front end's interpreter accepts the test, so it gives the null pointer too.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1. The run-time check for a cast down or across needs the operand to designate a public base subobject. A private base subobject fails both branches of the check, and a failed cast to a pointer gives a null pointer. Constant evaluation follows the same check.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

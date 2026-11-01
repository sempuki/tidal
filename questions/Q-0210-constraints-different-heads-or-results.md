# Q-0210: Do constraints decide between templates whose heads, or whose conversion results, differ?

**Status:** provisional
**Affects:** ORD-13, `tests/templates/ordering/constraints-different-heads.cpp`, `tests/templates/ordering/constraints-conversion-results-differ.cpp`

## Context

Two cases reach the constraint tie-breaker of partial ordering after
every other step ties:

- Two function templates both take `T`. One has the head
  `template <class T>`. The other has `template <class T, class U = int>`
  and a constraint.
- Two conversion function templates convert to `T*` and to `T* const`.
  The second has a constraint. The target is `int*`.

| Compiler | Result in both cases |
|---|---|
| Clang 18, 22, host | ambiguous |
| GCC 14, 16, host | ambiguous |
| EDG 7.0 | the constrained template |

When the heads and the results are the same, every compiler chooses the
constrained template (`constraints-partial.cpp`,
`constraints-conversion.cpp`).

## Evidence

The two tests above, with overrides for EDG. The sanitized Clang source
(clang-sema-deduction) compares constraints only when the template
parameter lists are equivalent, the function parameter types are the
same, and, for conversion functions, the result types are the same.
Items M2 to M4 of the deduction inventory describe these conditions.
Q-0146 is the case of different function parameters.

## Options

1. Constraints decide only between templates with equivalent heads, the
   same function parameter types, and for conversion functions the same
   result type (Clang, GCC).
2. Constraints decide whenever the other steps tie (EDG).

## Provisional answer

Option 1, which matches the answer of Q-0146.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Q-0310: Which point of instantiation does a constexpr function specialization use outside constant evaluation?

**Status:** provisional
**Affects:** INST-31, INST-32, LAM-15, `tests/templates/instantiation/function-point-constexpr-later.cpp`

## Context

`template <class T> constexpr int info(T item) { return probe(item); }`
is used in `int use()`, and the function `lib::probe` that argument-dependent
lookup finds is declared after `use`. The use is not a constant
evaluation. The specialization has a point right after `use` and a
point at the end of the translation unit (INST-31).

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: `probe` is not declared |
| GCC 14, 16, host | accepts, and the call finds `lib::probe` |
| EDG 7.0 | rejects: `probe` is undefined |

The call operator of a lambda is `constexpr` when it can be, so the
call operator of a generic lambda with an explicit return type gives
the same split. A function template that is not `constexpr` and has a
written return type finds `probe` on every compiler
(`function-point-end-of-unit.cpp`).

## Evidence

The test above, and probes with a generic lambda at namespace scope.

## Options

1. The two points give the specialization different meanings, so the
   program is not valid and no diagnostic is required (INST-32). Both
   results are allowed.
2. A `constexpr` function specialization uses the declarations before
   its first point, as a constant evaluation does (Clang, EDG).
3. A use outside constant evaluation can see the declarations up to the
   end of the unit (GCC).

## Provisional answer

Option 1. INST-32 already covers points that differ, and the
implementations differ only in which point they take.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

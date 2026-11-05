# Q-0310: Which point of instantiation does a constexpr function specialization use outside constant evaluation?

**Status:** settled
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
(`function-point-end-of-unit.cpp`). The split needs a first use before
the declaration of `probe`. When the first use follows the
declaration, every compiler accepts
(`templates/dependent-names/generic-lambda-argument-lookup.cpp`).

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

## Answer

Either point (INST-32); GCC takes the end of the unit, Clang and EDG the first point (option 1). INST-32 already covers points that differ, and the
implementations differ only in which point they take.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and INST-P4 gives the same answer: a function specialization has a point after the use and one at the end of the unit, and all points must give the same meaning. No newest family differs, because the answer is a no-diagnostic rule that every outcome fits.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A function template specialization has a point right after the use and another at the end of the unit, and two points that give different meanings make the program not valid with no diagnostic required. No special point exists for a `constexpr` function used outside constant evaluation, so each result is allowed.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

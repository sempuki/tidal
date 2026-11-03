# Q-0220: Does a placement deallocation function win over a template that also matches?

**Status:** provisional
**Affects:** NEW-23, NEW-24, DED-39, `tests/expressions/new-delete/throw-placement-function-and-template.cpp`, `tests/expressions/new-delete/throw-placement-function-before-template-instantiation.cpp`, `tests/expressions/new-delete/throw-placement-deleted-function-and-template.cpp`

## Context

A class has the placement allocation function
`operator new(size, Arena*)`. The class also has the deallocation
function `operator delete(void*, Arena*)` and the template
`operator delete(void*, T*)`. Both match the allocation function, the
template with `T = Arena`. The constructor throws.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | calls the function |
| EDG 7.0 | chooses the function |
| Clang 18, 22, host | calls no deallocation function, and warns that two match |

EDG has no code generation. Two probes show its choice. EDG rejects
the program when the function is deleted. EDG accepts the program when
the body of the template holds a failing `static_assert`.

## Evidence

`tests/expressions/new-delete/throw-placement-function-and-template.cpp`,
with an override for GCC,
`tests/expressions/new-delete/throw-placement-deleted-function-and-template.cpp`,
with an override for EDG, and
`tests/expressions/new-delete/throw-placement-function-before-template-instantiation.cpp`.
The gap review of question cluster 3
(`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads
the reference.

## Options

1. The function wins over the template, as in overload resolution
   (GCC, EDG).
2. Two matches give no deallocation function (Clang).

## Provisional answer

No; a function and a matching template are two matches, and no deallocation function is called (option 2). Clang calls none.

Changed on 2026-11-03. The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 2, with doubt. The review matches Clang 22 and NEW-P3, where no deallocation function is called when no single function matches. The match of a placement deallocation function compares parameter types and forms no conversion sequences, so it is not overload resolution, and OVL-27 has no place in it. Partial ordering (ORD-9) narrows only templates, so the answer of Q-0219 does not carry over. NEW-23 no longer prefers the function, and NEW-24 and DED-39 now give option 2. `throw-placement-function-and-template.cpp` expects no call, with an override for GCC. The new test `throw-placement-deleted-function-and-template.cpp` expects acceptance, with an override for EDG, which chooses the deleted function.

The question stays open under decision 0051 (2026-11-03). GCC 16 calls the function, EDG 7.0 chooses it, and Clang 22 calls no deallocation function, so two families disagree. The review's doubt is whether the narrowing among templates was meant to cover a plain function too.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 2, with doubt. The match compares declarations, and for templates it deduces and then uses partial ordering to narrow the templates to one. A deallocation function is called only when exactly one function matches. The match is not overload resolution, and it gives no preference to a function that is not a template over a matching specialization. A plain function and one specialization are two matches, so no deallocation function is called, as Clang does. The doubt is whether the narrowing among templates was meant to extend to plain functions. The reference gives no sign of it.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

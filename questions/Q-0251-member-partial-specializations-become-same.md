# Q-0251: Is a class instantiation not valid when two member partial specializations become the same?

**Status:** provisional
**Affects:** INST-6, `tests/templates/instantiation/member-partial-specializations-collide.cpp`
**Waits for:** newest families that agree

## Context

`Outer<T>` declares the member partial specializations `In<U, T>`
and `In<U, int>`. With `T = int`, both are `In<U, int>`.

| Compiler | `Outer<int> o;` |
|---|---|
| Clang 18, 22, host | rejects: the partial specialization cannot be redeclared |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

Every compiler accepts `Outer<char>`.

## Evidence

The test above. Item 24 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the two declarations count as a redeclaration, so the class
instantiation is not valid.

## Options

1. The instantiation is not valid (Clang).
2. The instantiation is valid (GCC, EDG).

## Provisional answer

Yes; Clang only, GCC and EDG accept (option 1). INST-6 requires each declaration that an instantiation makes to be valid, and a partial specialization declared twice in one class is not. The gap review agrees with Clang. GCC and EDG accept the class.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer, and the rule records one differing family. GCC 16 and EDG 7.0 accept the class.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. An instantiated declaration counts as a definition when the template's declaration is one. Two member partial specializations that both have a body and become one partial specialization are two definitions of one entity. GCC's and EDG's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

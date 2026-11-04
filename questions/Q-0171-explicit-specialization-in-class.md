# Q-0171: Can an explicit specialization of a member template be declared in the class?

**Status:** settled
**Affects:** TSP-5, TSP-19

## Context

A class declares `template <class T> static constexpr int of()` and,
in its body, `template <> constexpr int of<int>()`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | rejects: explicit specialization in a scope that is not a namespace |
| EDG 7.0 | accepts |

## Evidence

`tests/templates/specialization/explicit-class-scope.cpp`, with an
override for GCC.

## Options

1. The class that declares the member template is a valid place for the
   explicit specialization (Clang, EDG).
2. Only namespace scope is valid (GCC).

## Answer

Yes; Clang and EDG agree, GCC rejects (option 1). Clang and EDG agree, and the class is the scope where the
member template is declared, which matches TSP-5's rule for namespaces.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and TSP-P3 gives the same answer: a specialization of a member template is declared in the class of the member template. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. An explicit specialization can appear in any scope where its primary template could be defined, and a member template can be defined in its class. A member function template of a class is among the things that can be explicitly specialized. GCC's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

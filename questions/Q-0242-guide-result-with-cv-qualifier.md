# Q-0242: Can the result of a deduction guide have a cv-qualifier?

**Status:** settled
**Affects:** CTAD-12, `tests/templates/class-template-deduction/guide-result-qualified.cpp`

## Context

`template <class T> Box(T*) -> const Box<T>;`

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the result is not a specialization of the template |
| EDG 7.0 | rejects: the return type must name a specialization directly |
| GCC 14, 16, host | accepts |

## Evidence

The test above.

## Options

1. The result must be a template-id of the class template, with no
   cv-qualifier (Clang, EDG).
2. A cv-qualifier is allowed and has no effect (GCC).

## Answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the result of a guide
names the specialization that a declaration gets. A declaration adds
its own cv-qualifiers (CTAD-1).

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CTAD-P1 gives the same answer: a guide returns a specialization, and a declaration adds its own cv-qualifiers. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The grammar of a deduction guide allows only a template name with an argument list after the arrow. A cv-qualifier has no place there, so the form is a syntax error. GCC's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

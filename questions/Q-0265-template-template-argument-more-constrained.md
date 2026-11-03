# Q-0265: Can a template template argument be more constrained than its parameter?

**Status:** provisional
**Affects:** CNS-25, `tests/templates/constraints/template-template-more-constrained.cpp`

## Context

The parameter is `template <A> class`, and the argument is a class
template whose parameter has the type constraint `AB`, which subsumes
`A`.

| Compiler | Result |
|---|---|
| Clang 18 | accepts |
| Clang 22, host | rejects: template template argument is more constrained than template template parameter |
| GCC 14, 16, host | rejects: constraint mismatch |
| EDG 7.0 | accepts |

## Evidence

The test above. Every compiler accepts an argument that is less constrained than its parameter (`template-template-constrained.cpp`).

## Options

1. Not valid (Clang 22, GCC). The template could use the parameter
   with an argument that the argument template does not accept.
2. Valid (Clang 18, EDG).

## Provisional answer

No; Clang 22 and GCC agree, Clang 18 and EDG differ (option 1). The current Clang and GCC agree, and the rule keeps a use of the parameter inside the template valid.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the more constrained argument, and EDG 7.0 accepts it, so two families disagree. Clang 18 also accepts it. CNS-P4 supports the provisional answer. A template template parameter must be at least as constrained as its argument (CNS-25), because the template can use the parameter with an argument that a more constrained argument template does not accept. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. The matching rule compares the constraints of the parameter and of the argument when the parameter is constrained, and an example of the reference rejects an argument with a stronger constraint.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

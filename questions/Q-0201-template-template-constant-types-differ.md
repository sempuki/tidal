# Q-0201: Does a template template argument match when its constant parameter has another fixed type?

**Status:** provisional
**Affects:** TARG-17, TARG-18
**Waits for:** the maintainer (pending.md row 3)

## Context

`template <long V> struct Long` is the argument for
`template <template <int> class C> struct Use`.

| Compiler | Result |
|---|---|
| Clang 22, host | accepts |
| Clang 18 | rejects: different template parameters |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: not compatible with the template template parameter |

All compilers accept a placeholder parameter on either side: a
`template <auto>` argument for a `template <int>` head, and a
`template <int>` argument for a `template <auto>` head.

## Evidence

`tests/templates/arguments/template-template-constant-type.cpp`, with
overrides for EDG and Clang 18, and `template-template-auto.cpp`.

## Options

1. The argument matches when each argument list that the head accepts
   is valid for the argument template (Clang 22, GCC). This is the model
   of TARG-17.
2. The two constant parameters must have the same type (Clang 18, EDG).

## Provisional answer

Yes; Clang 22 and GCC accept, Clang 18 and EDG reject (option 1). The newest Clang and GCC agree, and the answer follows the
model that TARG-17 states for the other cases (Q-0169).

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept `Long` for a `template <int>` head, and EDG 7.0 rejects it, so two families disagree. Clang 18 also rejects it. TARG-P2 supports the provisional answer through TARG-17. The argument is valid when each argument list that the head accepts is valid for the argument template, and every `int` argument converts to `long`. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1, with doubt.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1, with doubt. The match of a template template argument rewrites the parameter and the argument as two function templates and asks whether the one for the parameter is at least as specialized. The invented class template takes the argument's head, so the types that the check compares are equal, and the match succeeds. The doubt is that the reference does not say plainly how deduction treats a dependent argument converted to the type of the parameter.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

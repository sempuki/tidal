# Q-0245: Can the keyword template come before a destructor name?

**Status:** settled
**Affects:** DEP-34, `tests/templates/dependent-names/template-keyword-destructor.cpp`

## Context

`p->template ~T()` in a function template, where `p` has the type
`T*`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the keyword is not permitted in a destructor name |
| GCC 14, 16, host | rejects, with the same message |
| EDG 7.0 | accepts |

## Evidence

The test above. Item 2 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the keyword must not come before a destructor name. EDG also
accepts `p->T::template ~T()`, and Clang and GCC reject it.

EDG also accepts `r.template ~T()` on a reference and `p->template ~Item()` with a class name, and Clang and GCC reject both.

## Options

1. The keyword is not valid before a destructor name (Clang, GCC).
2. The keyword is accepted and has no effect (EDG).

## Answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations and the gap review agree.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the keyword before the destructor name, and EDG 7.0 accepts it, so two families disagree. A probe of `p->T::template ~T()` gives the same split. DEP-P5 supports the provisional answer. The keyword `template` tells the parser that a template-id follows, and a destructor name is no template-id, so DEP-34 is an exception of DEP-P5. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and DEP-P5 gives the same answer: the keyword tells the parser that a template-id follows, and a destructor name is no template-id (DEP-34 is its exception). EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. The keyword `template` is not valid right before the tilde of a destructor name.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

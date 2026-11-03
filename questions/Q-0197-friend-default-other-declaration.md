# Q-0197: Must an implementation diagnose another declaration of a friend function template that gives a default?

**Status:** provisional
**Affects:** TPL-38, `tests/templates/declarations/default-friend-redeclared.cpp`, `tests/templates/declarations/default-friend-declared-before.cpp`

## Context

A class defines `template <class T = int> friend int pick(Host)`.
Another declaration of the same function template comes before or after
the class.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: a friend template declaration with a default must be the only declaration |

All compilers accept the friend definition when it is the only
declaration.

## Evidence

`tests/templates/declarations/default-friend-redeclared.cpp` and
`default-friend-declared-before.cpp`, each with overrides for Clang and
GCC, and `default-friend-definition.cpp`. The gap review of question
cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`)
reads the reference.

## Options

1. The other declaration makes the program invalid, and an
   implementation need not diagnose it. All three compilers conform.
2. An implementation must reject the program (EDG).
3. The program is valid (Clang, GCC).

## Provisional answer

Yes, an implementation must reject the program; EDG rejects, Clang and GCC accept (option 2).

Changed on 2026-11-03. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2. The rule has no exemption from diagnosis, and both declarations are in one translation unit, where an implementation sees both. Tidal's own rules give the same answer. TPL-P3 gives each parameter its default in one declaration only, and the rules that enforce that model on one translation unit are `reject` rules: TPL-16 for a default given twice, TPL-17 for a member defined outside the class, and TPL-37 for a friend declaration that is not a definition. The reading also matches EDG 7.0. TPL-38 is now a `reject` rule in the Constraints segment. `default-friend-redeclared.cpp` expects a rejection, with overrides for Clang and GCC. The new test `default-friend-declared-before.cpp` covers the declaration before the class.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the other declaration, before or after the class, and EDG 7.0 rejects it, so two families disagree. The front end accepts the two tests, so `tools/frontend_check.py` reports them as missed rejections. This goes to item FE.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2. A friend function template declaration that gives a default template argument must be a definition and the only declaration of that template, in either direction of reachability. The rule carries no exemption from diagnosis. In both test forms each declaration is reachable from the other, so an implementation can see the violation, and EDG's rejection conforms.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

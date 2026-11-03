# Q-0195: Must an implementation diagnose a function template parameter after a pack that deduction cannot find?

**Status:** provisional
**Affects:** TPL-35, TPL-30, TPL-36, `tests/templates/declarations/pack-then-not-deducible.cpp`, `tests/templates/declarations/pack-then-pack-not-deducible.cpp`, `tests/templates/declarations/pack-then-typed-pack-not-deducible.cpp`

## Context

A function template declares a type pack, then a type parameter with no
default. No function parameter uses the second parameter, so no call
can give it an argument.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts the declaration |
| GCC 14, 16, host | accepts the declaration |
| EDG 7.0 | rejects: the parameter follows a pack and cannot be deduced |

The same split holds for two type packs in a row, and for a constant
pack whose type is the type pack before it.

## Evidence

`tests/templates/declarations/pack-then-not-deducible.cpp`,
`pack-then-pack-not-deducible.cpp`, and
`pack-then-typed-pack-not-deducible.cpp`, each with overrides for Clang
and GCC. `tests/templates/declarations/pack-then-deducible.cpp` shows
the valid forms, which all compilers accept. The gap review of question
cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`)
reads the reference.

## Options

1. The declaration is not valid, and an implementation need not
   diagnose it. All three compilers conform.
2. The declaration is not valid, and an implementation must reject it
   (EDG).
3. The declaration is valid (Clang, GCC).

## Provisional answer

Yes, an implementation must reject the declaration; EDG rejects, Clang and GCC accept (option 2).

Changed on 2026-11-03. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2. The rule on a parameter after a pack has no exemption from diagnosis, and the general exemption for a template with no valid specialization does not override it. Tidal's own rules give the same answer. TPL-35 is a check on the template head and the function parameters, and no argument list takes part in it. TPL-30 makes the same kind of check on a class template a `reject` rule. DEP-32 covers a construct whose validity depends on the arguments, which a head check does not. The reading also matches EDG 7.0. TPL-35 is now a `reject` rule in the Constraints segment. `pack-then-not-deducible.cpp` expects a rejection, with overrides for Clang and GCC. The new tests `pack-then-pack-not-deducible.cpp` and `pack-then-typed-pack-not-deducible.cpp` cover two type packs and a constant pack typed by the type pack before it.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the declaration, and EDG 7.0 rejects it, so two families disagree. The front end accepts the three tests, so `tools/frontend_check.py` reports them as missed rejections. This goes to item FE.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2. The rule forbids a parameter after a pack of a function template that has no default and that the function parameters do not give, and the rule carries no exemption from diagnosis. The reference marks the two type packs and the type parameter after a pack as errors, and the constant pack typed by the earlier pack falls under the same rule. The general rule for a template with no valid specialization gives way to this specific rule, so EDG conforms, and Clang and GCC do not.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

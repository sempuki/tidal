# Q-0249: Does a qualified call in a template use a default argument that a later declaration adds?

**Status:** settled
**Affects:** DEP-31, `tests/templates/dependent-names/qualified-call-later-default.cpp`

## Context

`lib::probe(Item, int)` is declared before the template, and
`lib::probe(Item, int = 4)` after it. The template calls
`lib::probe(item)` with a dependent argument.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | accepts, and uses the default |
| Clang 18, 22, host | rejects: too few arguments |
| EDG 7.0 | rejects: too few arguments |

With an unqualified call, argument-dependent lookup at the point of
instantiation finds the later declaration, and every compiler uses the
default (`argument-lookup-later-default.cpp`).

## Evidence

The tests above. Item 19 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that a default argument added later, which the lookup misses, makes
the program not valid with no diagnostic required.

## Options

1. The program breaks DEP-31, and both results are allowed.
2. The call must use the default (GCC).
3. The call must not use the default, and the implementation rejects it (Clang, EDG).

## Answer

No diagnostic required; GCC uses it, Clang and EDG reject (option 1). DEP-31 needs no diagnostic, so each compiler's result lies within the rule. The gap review supports option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and DEP-P1 gives the same answer: an independent name is bound at the definition, and a later declaration does not change the binding. DEP-P6 needs no diagnostic for a construct whose meaning changes before the instantiation. No newest family differs, because the answer is a no-diagnostic rule that every outcome fits.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A program is not valid, with no diagnostic required, when its meaning would change if it counted a default argument that a later declaration gives, one that the instantiation can see but that lookup for the specialization does not find. A qualified call is not dependent, so the name is bound at the definition.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

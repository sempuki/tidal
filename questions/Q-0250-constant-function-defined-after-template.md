# Q-0250: Can a template's independent constant use a constexpr function that is defined after the template?

**Status:** settled
**Affects:** DEP-31, `tests/templates/dependent-names/constant-function-defined-later.cpp`

## Context

`limit` is declared `constexpr` and not defined before the template.
The template initializes `constexpr int n = limit();`, and `limit` is
defined before the template is used.

| Compiler | `constexpr` variable | array bound `int items[limit()]` |
|---|---|---|
| Clang 18, 22, host | rejects at the definition | rejects: variable length array |
| GCC 14, 16, host | accepts | rejects: variable length array |
| EDG 7.0 | accepts | accepts |

## Evidence

The test above. Item 20 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that such a constant breaks the rule that an independent construct has
one meaning, with no diagnostic required.

## Options

1. The program breaks DEP-31, and both results are allowed.
2. The constant is evaluated at the instantiation, and the program is valid (GCC and EDG for the variable).
3. The constant is evaluated at the definition, and the program is not valid (Clang).

## Answer

No diagnostic required; GCC and EDG accept, Clang rejects (option 1). DEP-31 needs no diagnostic, so each compiler's result lies within the rule. The gap review supports option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and DEP-P6 gives the same answer: an independent construct whose meaning changes before the instantiation need not be diagnosed. No newest family differs, because the answer is a no-diagnostic rule that every outcome fits.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. Constant evaluation inside an instantiation that relies on a `constexpr` function not yet defined at the template's definition is one of the ways a construct's meaning can differ between the definition and an instantiation, which needs no diagnostic. The same holds for the array bound form.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

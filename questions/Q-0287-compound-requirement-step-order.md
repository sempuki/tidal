# Q-0287: In a compound requirement, are the arguments of the type constraint substituted only after the expression and the `noexcept` test hold?

**Status:** provisional
**Affects:** CPT-33, `tests/templates/concepts/compound-requirement-order.cpp`, `tests/templates/concepts/compound-requirement-order-noexcept.cpp`, `tests/templates/concepts/compound-requirement-extra-argument-unused.cpp`

## Context

A compound requirement `{ E } noexcept -> C<A>` has an extra argument
`A` for its type constraint. Two cases test the steps:

- Order. `E` is not valid, or `E` can throw and `noexcept` is written.
  `A` names a member of a class whose static assertion fails.
- Unused argument. The concept `C` does not use its second parameter,
  and `A` is not valid.

| Compiler | Order | Unused argument |
|---|---|---|
| Clang 18, 22, host | rejects: the static assertion fails | requirement false |
| GCC 14, 16, host | requirement false | requirement true |
| EDG 7.0 | requirement false | requirement false |

## Evidence

The three tests above. When the concept uses the extra argument and the
argument is not valid, every compiler finds the requirement false
(`compound-requirement-extra-argument.cpp`). The gap review of concepts
and constraints (item 19) gives the order: the expression, then the
`noexcept` test, then the extra arguments, then the concept.

## Options

1. The steps run in that order, and a failed step stops the later
   ones. The extra arguments are substituted even if the concept does
   not use them. EDG follows this option in both cases, Clang for the
   unused argument, and GCC for the order.
2. The extra arguments are substituted before the expression is
   checked (Clang).
3. An extra argument that the concept does not use is never
   substituted (GCC).

## Provisional answer

Yes, and unused extra arguments are substituted; EDG agrees, Clang and GCC each differ in one case (option 1). EDG follows it in both cases, each of Clang and GCC follows
it in one case, and the review describes the same order.

Kept open on 2026-11-03 under decision 0054, because the review reads the answer with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1, with doubt. The steps of a compound requirement run in order: substitution of the expression, the check for `noexcept`, substitution of the return-type requirement, and satisfaction of the type constraint. A failed step makes the requirement false, and checking stops. The whole return-type requirement is substituted, including an unused extra argument, which gives false. An example equates a compound requirement with a simple requirement and a nested requirement, where an argument that the concept never uses would not be substituted, which gives GCC's true (option 3).

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

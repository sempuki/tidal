# Q-0273: How does a list with an expansion before its end take part in deduction outside a call?

**Status:** provisional
**Affects:** PACK-12, `tests/templates/packs/nondeduced-before-end-partial.cpp`, `nondeduced-before-end-function-type.cpp`
**Waits for:** a review (against)

## Context

A list with an expansion before its end is a non-deduced position
(PACK-12). Two places outside a call give different results.

A partial specialization whose parameter appears only in such a list:
`template <class... T, class U> struct S<L<T..., U>>`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects the declaration: a template parameter cannot be deduced |
| GCC 14, 16, host | accepts, and the specialization never matches |
| EDG 7.0 | accepts, and the specialization never matches |

A function template `template <class... T> int take(void (*)(T..., int))`,
with a fallback `int take(...)`:

| Call | Clang | GCC | EDG |
|---|---|---|---|
| `take(f)` with `void f(char, int)` | fallback | fallback | fallback |
| `take(h)` with `void h(int)` | template | template | fallback |
| `take<char>(f)` | template | no match | template |

## Evidence

The tests above cover the cases where every compiler agrees. Item 9 of
the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that the rule applies to every deduction from a
type, and that such a partial specialization cannot be used.

## Options

1. The partial specialization breaks TSP-11 and is rejected (Clang).
   For a function type, `T` is empty unless explicit arguments give it
   elements, as for a call (DED-20).
2. The partial specialization is valid and never matches (GCC, EDG).

## Provisional answer

Non-deduced; the partial specialization is rejected; Clang agrees, GCC and EDG differ (option 1). TSP-11 asks that each parameter of a partial specialization
appear where deduction can find it, and the gap review reads the rule
the same way. For the function type, Option 1 follows the result that
two of three implementations give in each row.

Kept on 2026-11-03. The review reads the partial specialization with doubt. TSP-P3 gives the answer: a partial specialization uses each of its parameters in a form that deduction can find. Clang 22 follows it. The question stays open under decision 0054, because the review reads against part of the answer with doubt.

Kept open on 2026-11-04 under decision 0054, because the gap review reads against part of the answer with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads against the partial specialization, with doubt, and agrees for the function type. The C++23 text has no rule that each parameter of a partial specialization appears where deduction can find it. The declaration is more specialized than the primary template, and when the class template is used, matching leaves the last parameter without a value, so the specialization never matches (option 2, GCC and EDG). The doubt is the general rule that a template with no valid specialization is not valid, with no diagnostic required, which would allow Clang's rejection. TSP-11's bullet about where each parameter appears has the same status. For the function type, the pack is empty unless explicit arguments fill it, which is Clang's result in all three rows.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

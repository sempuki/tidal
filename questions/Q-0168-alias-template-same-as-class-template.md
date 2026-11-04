# Q-0168: Is an alias template that passes its parameters through the same template argument as its class template?

**Status:** provisional
**Affects:** TARG-19, TARG-22

## Context

`template <class T> using ListAlias = List<T>;` passes its parameter
to `List` unchanged. Both templates are used as the argument of a
template template parameter, `Use<List>` and `Use<ListAlias>`.

| Question | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `Use<List>` and `Use<ListAlias>` are one type | no | yes | yes |

## Evidence

`tests/templates/arguments/same-specialization-alias-template.cpp`
asserts that the two types differ. GCC and EDG fail the assertion and
have overrides.

## Options

1. The alias template is a template of its own, and the two types
   differ (Clang).
2. The alias template stands for the class template, and the two types
   are one (GCC, EDG).
3. The result is unspecified, and a program must not rely on either.

## Provisional answer

No, the two types differ; Clang agrees, GCC and EDG differ (option 1). An alias template is a template of its own (TPL-P1), and two template arguments are the same only when they name the same template (TARG-P4). TARG-22 states the answer, and TARG-19 covers the other cases.

Changed on 2026-11-03 to option 1. The review reads option 1, Clang 22 follows it, and TARG-P4 gives the same answer: two template arguments are the same when they name the same template, and an alias template is a template of its own (TPL-P1). TARG-22 is now a `required` rule that states option 1. The two tests already expect that the types differ, with overrides for GCC and EDG.

Kept open on 2026-11-03 under decision 0054, because the review reads option 1 with doubt, and two newest families differ from the answer (GCC 16 and EDG 7.0). TARG-22 also changed on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads option 1, with doubt. Two specializations are the same when, among other conditions, their template template arguments refer to the same template. An alias template is a template of its own, and nothing makes it refer to the class template, even when it passes its parameters through. So `Use<List>` and `Use<ListAlias>` differ, as Clang finds, and the result is not unspecified. The doubt is an old open core issue about whether such an alias should count as the class template. GCC and EDG follow that idea, and no resolution of it is known to have been adopted.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

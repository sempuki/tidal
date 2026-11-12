# Q-0375: For a member of an inline namespace, which namespace must a qualified definition lie outside?

**Status:** provisional
**Affects:** NS-15, `tests/declarations/namespaces/qualified-definition-inline-member-inside-named.cpp`
**Waits for:** a review (doubt)

## Context

`namespace geo { inline namespace v2 { int area(); } }` declares `area`
in `geo::v2`. The definition `int geo::area() { ... }` names the
function through `geo`. NS-15 says a qualified definition lies in a
namespace that encloses `N` and is not `N` itself. `N` can be `geo`,
the namespace that the name gives, or `geo::v2`, the namespace that
declares the member.

| Place of `int geo::area()` | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| global namespace | accepts | accepts | accepts |
| inside `geo` | accepts | accepts | rejects ("qualified name is not allowed in namespace member declaration") |
| inside `geo::v2` | rejects | rejects | rejects |

GCC rejects `int geo::f()` inside `geo` for a function that `geo`
declares itself (Q-0371), and accepts the definition inside `geo` here.
Clang accepts both with its "extra qualification" warning.

## Evidence

The test in Affects,
`tests/declarations/namespaces/qualified-definition-inline-member-inside-inline.cpp`,
and a probe that also defines the function as `geo::v2::area` and as
`v2::area` inside `geo`, which every compiler accepts.

## Options

1. `N` is the namespace that declares the member, here `geo::v2`. The
   definition inside `geo` is valid (GCC; Clang accepts it too).
2. `N` is the namespace that the qualified name gives, here `geo`. The
   definition inside `geo` is not valid (EDG).

## Provisional answer

`N` is the namespace that declares the member (option 1). GCC and
Clang accept the definition, and option 1 gives the same result as the
definition `int geo::v2::area()` in the same place, which every
compiler accepts.

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 rewrote agrees with the result and not with the test that NS-15 states, and reads the definition inside the inline namespace as valid, with doubt. EDG 7.0 is the one newest family that differs from the result.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer for the definition inside the named namespace, and EDG 7.0 alone differs. The narrower doubt is NS-15's exclusion of the declaring namespace, which the reference does not state (DQ-22).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) agrees with the result of option 1 for the case in dispute, and not with the test that NS-15 states. In the reference, a qualified definition must be in a namespace that contains the member. The global namespace, the named namespace, and the inline namespace all contain it, so the definition through the named namespace is valid in all three places. The row inside the named namespace matches option 1. The row inside the inline namespace is valid in the reference, and every compiler rejects it. The compilers seem to keep an older rule, under which the definition had to be in a namespace that encloses the one the qualifier names. The review holds this reading of the third row with some doubt, because the reference does not define what it means for a namespace to contain an entity.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-22). A declaration with a qualified name must be a friend declaration or lie in a scope that contains the entity. The definition inside the named namespace is valid under any reading of that phrase, because the named namespace contains the inline namespace that declares the member. So the answer holds for that row. The doubt is now only NS-15's own condition, which excludes the declaring namespace itself. The reference has no such exclusion and does not define what it means for a scope to contain an entity. So a definition inside the inline namespace may be valid in the reference, although every compiler rejects that definition.

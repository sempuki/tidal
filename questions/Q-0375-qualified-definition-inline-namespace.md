# Q-0375: For a member of an inline namespace, which namespace must a qualified definition lie outside?

**Status:** provisional
**Affects:** NS-15, `tests/declarations/namespaces/qualified-definition-inline-member-inside-named.cpp`

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

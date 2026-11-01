# Q-0256: How does an explicit instantiation name a variable template declared with auto?

**Status:** provisional
**Affects:** INST-22, `tests/templates/instantiation/explicit-placeholder-variable.cpp`

## Context

`template <class T> auto zero = T(0);` is explicitly instantiated
with the deduced type or with `auto`.

| Compiler | `template int zero<int>;` | `template auto zero<int>;` | `template long zero<int>;` |
|---|---|---|---|
| Clang 18, 22, host | accepts | rejects | rejects |
| GCC 14, 16, host | accepts | rejects | accepts |
| EDG 7.0 | rejects | accepts | rejects |

## Evidence

The test above. Item 35 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the explicit instantiation writes the deduced type, which is the
opposite of the function case. The review gives no outcome for the
other forms.

## Options

1. The explicit instantiation writes the deduced type (Clang, GCC).
2. The explicit instantiation writes `auto` (EDG).

## Provisional answer

Option 1. Two of three implementations and the gap review agree. GCC also accepts a type other than the deduced one, and INST-22 does not allow it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

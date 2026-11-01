# Q-0274: What does a name mean that finds an expanded using-declarator when the pack is empty?

**Status:** provisional
**Affects:** PACK-30, `tests/templates/packs/using-declaration-empty-pack-outer-name.cpp`

## Context

`template <class... B> struct D : B... { using B::f...; int g() { return f(0); } };`
with a function `f(int)` in the namespace. At the definition, `f` in
`g` finds the using-declarator. `D<>` has an empty pack.

| Compiler | `D<>().g()` |
|---|---|
| Clang 18, 22, host | rejects: the using-declaration expands to an empty pack |
| GCC 14, 16, host | accepts, and calls the namespace `f` |
| EDG 7.0 | accepts, and calls the namespace `f` |

With no namespace `f`, every compiler rejects the call.

## Evidence

The test above, and a probe without the namespace `f`. Item 15 of the
gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that the program is not valid and that no
diagnostic is required.

## Options

1. The program is not valid, and no diagnostic is required. Both
   results are allowed (`no-diagnostic`).
2. The program is valid and calls the namespace `f` (GCC, EDG).
3. The implementation must reject the program (Clang).

## Provisional answer

Not valid, no diagnostic required; GCC and EDG accept, Clang rejects (option 1). The implementations differ, and the gap review reads the
rule as one that requires no diagnostic.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

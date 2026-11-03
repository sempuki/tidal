# Q-0345: Do the members that a base declares count in the common initial sequence of a struct?

**Status:** provisional
**Affects:** UNI-15, UNI-16, `tests/classes/unions/common-initial-sequence-inherited.cpp`, `tests/classes/members/grids/layout-compatible.json`

## Context

`struct Base { int id; int size; }; struct Named : Base {};` and
`struct Direct { int id; int size; };`. `Named` declares no members, and
its base declares them all, so `Named` is standard-layout (CLS-19).

UNI-15 takes the members of `Named` to be `id` and `size` of `Base`.
Then the common initial sequence of `Named` and `Direct` holds both
members, and the two structs are layout-compatible (UNI-16).

| Check | Result |
|---|---|
| Builtin query of Clang 22 and GCC (grid row "members from a base") | layout-compatible |
| Run test: a union holds a `Named`, and the program reads `id` and `size` through `Direct` | the values of the `Named` object, on every compiler |
| Clang 18, EDG 7.0 | no such query |

The source-reader gap review of item O
(`reports/review/2026-11-03-gap-review-item-o.md`) says the reference
does not state whether the members of a struct include those that its
base declares. A run test cannot show the answer, because the read
through `Direct` gives the expected value on every compiler either way.

## Evidence

The grid row and the run test in **Affects**.

## Options

1. The members that a base declares count, so `Named` and `Direct`
   have a common initial sequence of two members (Clang 22 and GCC
   queries).
2. Only the members that the struct itself declares count, so the
   common initial sequence of `Named` and `Direct` is empty.

## Provisional answer

Yes; the members that a base declares count (option 1). The builtin queries of Clang 22 and GCC agree, and the
class-properties inventory (item LC-inherited-members) describes the
same reading. A struct whose members all come from one base has the
layout of that base, so the reading changes no address.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Q-0342: Does a base that declares defaulted special members lend its tail padding to a derived class?

**Status:** provisional
**Affects:** CLS-34, `tests/classes/members/layout-base-tail-padding-defaulted.cpp`

## Context

`struct Defaulted { Defaulted() = default; Defaulted& operator=(const
Defaulted&) = default; int a; char b; };` has 3 bytes of padding at its
end. `struct Next : Defaulted { char c; };` adds one member.

| Compiler | `sizeof(Next)` |
|---|---|
| Clang 18, 22 | 12 |
| GCC 16 | 12 |
| GCC 14 | 8 |
| EDG 7.0 | 12 |

The results are the same on x86-64 and with `-m32`. A base with only
implicit special members keeps its padding on every compiler (size 12),
and a base with a user-provided special member lends it on every
compiler (size 8).

The use of the end padding of a base is unspecified (CLS-34), so each
answer is allowed. The question records which choice the Evidence of
CLS-34 lists for the matrix.

## Evidence

The test in **Affects**, with an override for GCC 14, and
`tests/classes/members/layout-base-tail-padding.cpp`.

## Options

1. The base keeps its padding, as with implicit special members (GCC 16,
   Clang, EDG).
2. The base lends its padding, as with user-provided special members
   (GCC 14).

## Provisional answer

The base keeps its padding (option 1). The newest version of each
compiler family gives this layout. GCC 14 made the other choice.

Kept open on 2026-11-03 under decision 0054, because the review reads both options as allowed, and item TP of decision 0054 sets the matrix layout.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) finds that the reference leaves the use of the end padding of a base open, so both options are allowed. CLS-34 now classes the choice as unspecified, and the question records the matrix choice.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

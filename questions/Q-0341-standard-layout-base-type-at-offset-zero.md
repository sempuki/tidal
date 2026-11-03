# Q-0341: Does a base whose type sits deep inside the first member, or in a `[[no_unique_address]]` member, make a class not standard-layout?

**Status:** provisional
**Affects:** CLS-19, CLS-32, `tests/classes/members/standard-layout-base-type-inside-member.cpp`, `tests/classes/members/grids/standard-layout.json`

## Context

An empty class `E` is a base of `X`. `E` is not the type of the first
member of `X`, but an object of type `E` sits at the start of that
member, or `X` has a member of type `E` with `[[no_unique_address]]`.

| `X` | Clang 18, 22 query | GCC 14, 16 query | EDG 7.0 query |
|---|---|---|---|
| `struct M { E e; int b; }; struct X : E { M m; };` | not standard-layout | standard-layout | standard-layout |
| `struct X : E { E arr[2]; };` | not standard-layout | standard-layout | not standard-layout |
| `union V { int i; E e; }; struct X : E { V v; };` | not standard-layout | standard-layout | standard-layout |
| `struct X : E { int a; [[no_unique_address]] E e; };` | not standard-layout | standard-layout | standard-layout |

All three compilers say not standard-layout when the first member
itself has type `E`.

The class-properties inventory
(`reports/review/2026-11-03-inventory-class-properties.md`, item
SL-first-member-type) describes all four classes as not
standard-layout. The set of types to avoid follows the first member
down through classes, arrays, and unions, and adds every empty member
with `[[no_unique_address]]`.

No core-language check shows the property. In the first class, the
base `E` and the member `m.e` are two objects of one type and need two
addresses (CLS-10). Every compiler gives them two addresses.

## Evidence

The grid `tests/classes/members/grids/standard-layout.json`, rows
"base type first inside first member", "first member array of base
type", "first member union holding base type", and "overlap member of
base type, not first". The test in **Affects** checks the two
addresses.

## Options

1. Not standard-layout in all four cases (Clang).
2. Not standard-layout only when the first member itself has the base
   type (GCC).

## Provisional answer

Not standard-layout in all four cases (option 1). The inventory
describes this rule, and CLS-30 could not keep its promise that the
object and each base share the first member's address. The queries
differ, and no test can show the property, so the question stays
provisional.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

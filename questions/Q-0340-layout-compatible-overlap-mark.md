# Q-0340: Are two structs layout-compatible when matching members both have `[[no_unique_address]]`?

**Status:** provisional
**Affects:** UNI-15, UNI-16, `tests/classes/members/grids/layout-compatible.json`

## Context

`struct A { int a; [[no_unique_address]] E e; };` and `struct B { int x;
[[no_unique_address]] E f; };`, with `E` an empty class. The members
match in type and order, and both last members have the attribute.

| Builtin query | Result |
|---|---|
| Clang 22 | not layout-compatible |
| GCC 14, 16 | layout-compatible |
| Clang 18, EDG 7.0 | no such query |

When only one of the two last members has the attribute, Clang 22 and
GCC both say not layout-compatible.

The class-properties inventory
(`reports/review/2026-11-03-inventory-class-properties.md`, item
LC-overlap-mark) describes the condition as both members marked or
neither. Under that description, `A` and `B` are layout-compatible.

No core-language check shows the property (inventory item
OBS-no-core-layout-compatible). A union read through the other struct
is undefined if the member is outside the common initial sequence, and
no sanitizer stops such a read.

## Evidence

The grid `tests/classes/members/grids/layout-compatible.json`, rows
"overlap mark on one" and "overlap mark on both".

## Options

1. Layout-compatible: the attribute must agree, and here it does (GCC).
2. Not layout-compatible: a member with the attribute ends the common
   initial sequence (Clang 22).

## Provisional answer

Layout-compatible (option 1). The inventory describes this condition,
and the two members have the same type and the same attribute. The
queries differ, and no test can show the property, so the question
stays provisional.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

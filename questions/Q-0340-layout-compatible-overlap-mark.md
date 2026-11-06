# Q-0340: Are two structs layout-compatible when matching members both have `[[no_unique_address]]`?

**Status:** settled
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

## Answer

Layout-compatible (option 1). The inventory describes this condition,
and the two members have the same type and the same attribute. The
queries differ, and no test can show the property, so the question
stays provisional.

Kept open on 2026-11-03 under decision 0054, because no principle gives the answer. UNI-P6 does not name the attribute, and only UNI-15 does.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). UNI-P6 failed step 3, because it did not name the attribute, and only UNI-15 did. UNI-P6 now lists the conditions on each pair of the common initial sequence, with both or neither member carrying `[[no_unique_address]]` among them. The grid `layout-compatible.json` checks the conditions through the builtin queries. The queries still differ, as the provisional answer says. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses UNI-P6.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies UNI-P6 to the program of the row "overlap mark on both" of `layout-compatible.json` without the rule text and reaches the provisional answer: each pair of members has the same type and both last members carry the attribute, so the sequence holds every member and the structs are layout-compatible.

Kept open on 2026-11-04 under decision 0055, because UNI-P6 changed in the same commit: the review found that it paired only the members, and an unnamed bit-field also takes part in the sequence. The question waits for the next use of UNI-P6.

Settled on 2026-11-05 under decision 0054. The gap review of the text phase 9 wrote used UNI-P6 on the question's program (decision 0056), the use supports the answer, and the review supports it with no doubt. No test of the question has a newest-family override.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) reads the reference as supporting option 1, the provisional answer. Matching members must both carry the attribute or both lack it, and a marked member does not end the sequence. Clang 22's query conflicts with that reading.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. The common initial sequence runs over the non-static data members and bit-fields, unnamed bit-fields included, and the reference's own example shows an unnamed bit-field of width 0 ending it. For this question the attribute condition is the one that matters.

### Gap review, 2026-11-05

The source-reader gap review of the text phase 9 wrote on classes, overloading, and exceptions (`reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md`) used UNI-P6 on the question's program, the row "overlap mark on both" of `tests/classes/members/grids/layout-compatible.json`, without the rule text. The model gives layout-compatible, the provisional answer, and the reference supports that answer with no doubt.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

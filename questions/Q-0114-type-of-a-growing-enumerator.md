# Q-0114: What type does an enumerator get when its value outgrows the type before it?

**Status:** provisional
**Affects:** ENUM-13, `tests/declarations/enumerations/enumerator-type-grows.cpp`

## Context

In an enumeration without a fixed type, an enumerator without an
initializer takes the value of the one before it plus one. If the first
enumerator is the largest `int`, the second does not fit in `int`. Inside
the enumerator list, Clang gives the second enumerator the type `long`,
GCC gives it `unsigned int`, and EDG gives it an unsigned type of 8
bytes.

## Options

1. The type is unspecified: any integer type that can hold the value.
2. The type is the first type, by rank from `int` upward, that can hold
   the value, which matches Clang.

## Provisional answer

Unspecified: any integer type that holds the value (option 1). The three compilers give three types, and each type holds the
value.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. ENUM-P3 lets the implementation choose the underlying type, and no principle speaks of the type of an enumerator inside the list. Each newest family gives a type that holds the value, which the answer allows.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ENUM-P1 failed step 3, because no principle spoke of the type of an enumerator inside the list. ENUM-P1 now says that inside the list each enumerator has the type of its own value, that an enumerator without an initializer keeps the type of the one before it, and that the implementation chooses an integer type that holds the value when that type cannot hold it. The review reads with no doubt, and each newest family gives a type that the answer allows. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ENUM-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied ENUM-P1 to the program of `enumerator-type-grows.cpp` without the rule text and reached the provisional answer: the type of the enumerator before cannot hold the value, so the implementation picks some integer type that can.

Kept open on 2026-11-04 under decisions 0054 and 0055, because ENUM-P1 changed in this commit: a first enumerator without an initializer has the type `int`. The question waits for the next use of the changed text.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. An enumerator with no initializer keeps the type of the one before it unless the value no longer fits. Then the enumerator gets an unspecified integral type large enough for the value, and the program is not valid if no such type exists.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The review found two cases that ENUM-P1 left out: an initializer of unscoped enumeration type, which the review reads as giving the enumeration's underlying type, and a first enumerator with no initializer, which gets a signed integer type.

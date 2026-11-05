# Q-0083: Can a default member initializer of a nested class be used before the enclosing class is complete?

**Status:** provisional
**Affects:** CTOR-18, `tests/classes/constructors/nested-default-member-init-before-complete.cpp`
**Waits for:** a review (against)

## Context

A class has a nested class with a default member initializer.
A default argument of a constructor of the enclosing class value-initializes
the nested class with empty braces. Clang and GCC reject the program. EDG
accepts it.

EDG also accepts the nested initializer in the initializer of a static
data member, in the operand of `noexcept`, and in the bound of a member
array, where Clang and GCC reject it. So every test of CTOR-18 needs an
override for EDG.

## Options

1. The program is not valid.
2. The program is valid.

## Provisional answer

No; the program is not valid (option 1). Clang and GCC agree.

Kept on 2026-11-03. The review reads option 2 with doubt, and CTOR-P3 gives the answer that Clang 22 and GCC 16 follow: a nested class's default member initializer is part of the enclosing class, so it is usable only once the enclosing class is complete. The question stays open under decision 0054, because the review reads against the answer with doubt.

Kept open on 2026-11-04 under decision 0054, because the gap review reads against the answer with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads option 2, with doubt. A default argument and a default member initializer are both places where the class counts as complete, and such a place inside a nested class is one for the enclosing class too. No rule orders the two or makes the use invalid, and the bans on a default member initializer that leads back to its own class do not fit this test. So the program reads as valid, as EDG finds. The doubt is that the order of these delayed places is a known unsettled area that the text does not describe in detail.

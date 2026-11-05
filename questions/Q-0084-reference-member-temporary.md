# Q-0084: Must an implementation reject a constructor that binds a reference member to a temporary?

**Status:** provisional
**Affects:** CTOR-17, `tests/classes/constructors/reference-member-binds-temporary.cpp`, `tests/classes/constructors/reference-default-member-binds-temporary.cpp`, `tests/classes/constructors/reference-member-binds-converted-parameter.cpp`, `tests/classes/constructors/initializer-list-member-from-braces.cpp`, `tests/classes/constructors/initializer-list-default-member-from-braces.cpp`, `tests/classes/constructors/reference-default-member-unused.cpp`, `tests/classes/constructors/reference-default-member-aggregate-value.cpp`
**Waits for:** newest families that agree

## Context

A constructor binds a reference member to a temporary, through a
member initializer or a default member initializer. The temporary ends with
the constructor, so the reference dangles. Clang rejects both forms. GCC
accepts with a warning. EDG accepts.

A default member initializer counts only through a constructor that
uses it (finding G136 of `reports/clean-room/rust-frontend/report-19.md`).
Clang rejects the initializer at such a constructor, including an
implicit default constructor that the program uses. Every compiler
accepts a class whose constructors all give the member a `mem_init`,
and an aggregate whose initialization gives the member a value.

Every compiler also accepts an aggregate initialization that leaves the
member to its default member initializer, and Clang and GCC run it with
the temporary still alive. The gap review of the text phase 8 wrote on
classes and overloading
(`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`)
finds that the reference does not name this case. In constant
evaluation, Clang and GCC accept a read of the member through such an
aggregate, and EDG rejects it. No test checks that read.

## Options

1. The program is not valid, and an implementation need not diagnose it
   (no-diagnostic).
2. Implementations must reject the program.
3. The program is valid, and a later use of the reference is undefined.

## Provisional answer

No; implementations must reject the program (option 2). Clang rejects both forms. GCC warns and accepts, and EDG accepts.

Changed on 2026-11-03 to option 2. The review reads option 2, Clang 22 rejects both forms, and CTOR-P6 gives the same answer: such a reference member is dead before the body runs, and the binding is visible where the constructor or the class is defined, so nothing is unknown at that point (the `no-diagnostic` class of `documentation/specification-format.md` is for a fault that depends on information the implementation lacks). CTOR-17 is now a `reject` rule, and its three tests expect a rejection, with overrides for GCC and EDG. The question stays open under decision 0054, because two newest families differ from the answer.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads option 2. A member initializer that binds a reference member to a temporary makes the program not valid, and so does a default member initializer that does so. Neither rule is exempt from diagnosis, so both forms must be rejected. Clang follows this reading. GCC's warning with acceptance and EDG's acceptance do not. Option 3 does not fit, because the binding itself is the fault.

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) agrees with CTOR-17 for both forms, and finds one more case. The reference treats a member of type `std::initializer_list` that a member initializer sets from a braced list as a reference member bound to the backing array, so the program is not valid. A probe splits as for a reference member: Clang 22 rejects both forms, and GCC 16 and EDG 7.0 accept them. CTOR-17 now covers the case, with two tests that record GCC and EDG as overrides.

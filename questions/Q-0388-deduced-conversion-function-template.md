# Q-0388: Can a conversion function template have `auto` as its name?

**Status:** settled
**Affects:** CVF-7, CVF-P1, `tests/classes/conversion-functions/template-deduced-auto.cpp`
**Source:** the CVF-P1 note of `reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`

## Context

A plain conversion function can be named `operator auto`, and its
`return` statements give the target type (CVF-1, Q-0059). The question
asks about a conversion function template with that name:

```cpp
struct S { template <class T> operator auto() const { return 1; } };
```

| Compiler | Result |
|---|---|
| Clang 22 | rejects: `auto` is not allowed in a conversion function template |
| GCC 16 | rejects under `-pedantic-errors`, with the message it also gives for a plain `operator auto` (Q-0059), and warns that the type can never be deduced |
| EDG 7.0 | accepts, and warns that `T` is not used |
| GCC 14 | accepts |

No compiler finds the template for a conversion to `int`.

## Evidence

The test in **Affects**. It expects a rejection and records the
acceptance of EDG and GCC 14.

## Options

1. The declaration is rejected (Clang, GCC).
2. The declaration is valid, and the template takes part in no
   conversion (EDG).

## Answer

No; the declaration is rejected (option 1). A conversion finds a
conversion function template by its target type before any body is
instantiated, so a template named `auto` could never be used. The gap
review of the text phase 8 wrote reads the reference this way. EDG is
the one newest family that differs.

Kept open on 2026-11-04 under decision 0054, because the question, CVF-7,
and CVF-P1 changed in the commit that opened it, and the question waits
for a use of CVF-P1.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied CVF-P1, without the rule text, to the program of `template-deduced-auto.cpp`. The answer names CVF-P1 and gives the outcome that the test expects, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and CVF-P1 gives the same answer by the test of the style guide (section 3). EDG 7.0 still differs, and the test keeps its override, with one more for GCC 14.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) finds that the reference allows a deduced return type for a plain conversion function and forbids one for a conversion function template. Neither CVF-P1, CVF-1, nor the deduction page stated the limit.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

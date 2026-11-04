# Q-0367: Which further forms does ODR-3 exempt from a use, and does a discarded statement exempt a use from the capture rules?

**Status:** settled
**Affects:** ODR-2, ODR-3, ODR-P1, `tests/foundations/odr/discarded-value-constant-in-lambda.cpp`, `tests/foundations/odr/discarded-value-class-constant-in-lambda.cpp`, `tests/foundations/odr/constant-array-element-in-lambda.cpp`, `tests/foundations/odr/discarded-branch-address-in-lambda.cpp`

## Context

A lambda with no captures can name a local variable of the enclosing
function only where the name is not a use (ODR-2, ODR-3). The
source-reader gap review of program structure
(`reports/review/2026-11-03-gap-review-program-structure.md`) reads
the reference as giving these answers for four forms:

| Form in the lambda | Use in the review's reading | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|---|
| `n;` for a local `const int n` | no | rejects | accepts | accepts |
| `origin;` for a local `constexpr` object of class type | no | rejects | accepts | accepts |
| `table[1]` for a local `constexpr int table[3]` | no | accepts | rejects | accepts |
| `&local` in a branch of `if constexpr (false)`, in a function that is not a template | yes | accepts | rejects | rejects |

In the first two forms the value of the expression is discarded. In
the third form the subscript reads one element. In the fourth form the
branch is a discarded statement, which needs no definition of what it
names.

## Evidence

The four tests in **Affects**. Each test expects the review's answer
and records the family that differs with an override.

## Options

1. Follow the review in each form. A discarded value of a constant and
   an element read from a constant array are no uses. A name in a
   discarded statement is a use for the capture rules, and needs no
   definition.
2. Follow Clang 22 in each form.
3. Treat every form as no use, as the earlier text of ODR-3 did for the
   discarded statement.

## Answer

Follow the review in each form (option 1). In each form two of the
three newest families agree with the review, and ODR-P1 gives the same
model: a use is where the program needs the entity itself, and a
capture rule asks only whether the lambda names the entity where the
program needs it.

Settled on 2026-11-04 under decision 0054. The gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) supports the answer in each form with no doubt, and ODR-P1 gives the same answer: a use is a point where the program needs the entity itself, and a capture rule asks whether the lambda names the entity there. In each form one newest family still differs, as the ODR-3 table of the page gives, and each test keeps its override.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) agrees with option 1 in each form. Forms 1 and 2 are expressions whose value is discarded, and the read of the value is not applied to an operand that is not `volatile`, so neither is a use. That exemption needs neither a variable usable in constant expressions nor a type without `mutable` members. In form 3, the subscript of an array counts the array among its possible results, so the array is not used, whether the element is read or discarded. In form 4, taking the address is a use. A discarded statement exempts a use only from the need for a definition, and capture is decided by the program text, also inside a discarded statement. So a lambda with no captures that takes the address there is not valid, as GCC and EDG find.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Q-0141: After a built-in candidate wins, does a class operand convert past its conversion function?

**Status:** settled
**Affects:** OVL-11, `tests/foundations/overload-resolution/builtin-chosen-other-operand.cpp`

## Context

The expression is `p + ToDouble{}`, where `p` is an `int*` and
`ToDouble` converts to `double`. The built-in candidate for `int*` and
`std::ptrdiff_t` wins, through the conversion function and then a
conversion from `double` to the integer type.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | rejects: the operands are `int*` and `double` |
| GCC 14, GCC 16 | rejects |
| EDG 7.0 | accepts |

When the class operand converts to the pointer instead, as in
`Holder{} + 1.5`, every compiler rejects.

## Evidence

`tests/foundations/overload-resolution/builtin-chosen-other-operand.cpp`
and `builtin-chosen-then-checked.cpp`. Item BI-16 of the overload
resolution inventory.

## Options

1. The class operand converts by its conversion function only, and the
   built-in operator then sees `int*` and `double` (Clang, GCC).
2. The class operand converts to the candidate's parameter type
   (EDG).

## Answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and the inventory
describes the conversion the same way.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and OVL-P4 gives the same answer: after a built-in candidate wins, the operator's own page decides what the expression does with the result of the conversion function. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. When a built-in candidate wins, a class operand converts by the user-defined part of its sequence, without the standard conversion that would follow. The built-in operator then sees a pointer and a floating type, which is not valid. EDG's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

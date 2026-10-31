# Q-0141: After a built-in candidate wins, does a class operand convert past its conversion function?

**Status:** provisional
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

## Provisional answer

Option 1. Two of three implementations agree, and the inventory
describes the conversion the same way.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Bitwise operators

The operators `&`, `^`, and `|` combine the bits of two integer values.
The operator `~` inverts the bits of one integer value. These rules apply
to the built-in operators. An overloaded operator is a function call
*(overload resolution referenced)*.

## Syntax

```{eval-rst}
.. productionlist:: bitwise
   or_expr: `xor_expr` {"|" `xor_expr`}
   xor_expr: `and_expr` {"^" `and_expr`}
   and_expr: `equality_expr` {"&" `equality_expr`}
   complement: "~" `unary_expr`
```

## Legality Rules

:::{rule} BIT-1
:kind: required
:tests: expressions/bitwise/precedence.cpp

`&` binds more tightly than `^`, and `^` binds more tightly than `|`.
All three bind less tightly than `==` and `!=`. Each groups from left to
right. `~` is a unary operator and binds more tightly than all of them.
:::

:::{rule} BIT-2
:kind: required
:tests: expressions/bitwise/operands-arithmetic.cpp expressions/bitwise/different-enum-types.cpp

The operands of `&`, `^`, `|`, and `~` can have integer, character,
`bool`, or unscoped enumeration type. The two operands of a binary
operator can be of different types, including two different enumerations.
:::

:::{rule} BIT-3
:kind: reject
:tests: expressions/bitwise/operands-floating.cpp expressions/bitwise/operands-pointer.cpp expressions/bitwise/operands-scoped-enum.cpp expressions/bitwise/complement-floating.cpp expressions/bitwise/complement-pointer.cpp

An operand of `&`, `^`, `|`, or `~` must not have floating-point
type, pointer type, or scoped enumeration type.
:::

:::{rule} BIT-4
:kind: required
:tests: expressions/bitwise/result-types.cpp expressions/bitwise/complement-promotes.cpp

The operands of `&`, `^`, and `|` go through the {term}`usual
arithmetic conversions`. The operand of `~` goes through {term}`integer
promotion`. The result has the converted type and is a prvalue.
:::

## Dynamic Semantics

:::{rule} BIT-5
:kind: required
:tests: expressions/bitwise/values.cpp expressions/bitwise/negative-values.cpp expressions/bitwise/complement-bool.cpp

The operators work on the two's complement bits of the converted values.
Each bit of the result is:

- 1 for `&` if both bits are 1,
- 1 for `|` if one or both bits are 1,
- 1 for `^` if exactly one bit is 1,
- the inverse of the operand bit for `~`.
:::

## Recommended Practice

:::{rule} BIT-6
:kind: recommended
:tests: expressions/bitwise/parentheses-warns.cpp expressions/bitwise/parentheses-silences.cpp

An implementation should warn if a comparison is an operand of `&`, `^`,
or `|` without parentheses.
:::

:::{rule} BIT-7
:kind: recommended
:tests: expressions/bitwise/complement-bool-warns.cpp expressions/bitwise/complement-int-silent.cpp

An implementation should warn if `~` applies to a `bool` operand.
:::

:::{rule} BIT-8
:kind: recommended
:tests: expressions/bitwise/mixed-enum-warns.cpp

An implementation should warn when the operands of `&`, `^`, or `|` have
two different enumeration types.
:::

## Examples

```cpp
6 & 3                // 2
6 | 3                // 7
6 ^ 3                // 5
~5                   // -6
-1 & 0xFF            // 255

unsigned char zero = 0;
~zero                // -1: zero is promoted to int first

flags & mask == 0    // flags & (mask == 0); compilers warn
~true                // -2; compilers warn

enum class Mode { Read = 1, Write = 2 };
Mode::Read | Mode::Write   // rejected without an operator|
```

## Evidence

All seven compilers agree on all 17 tests. The results are in
`tests/RESULTS.md`.

- BIT-6: Clang warns "& has lower precedence than ==" and GCC warns
  "suggest parentheses around comparison in operand of '&'". Both are in
  `-Wparentheses`.
- BIT-7: Clang and GCC warn under `-Wbool-operation`.
- EDG doesn't take these warning flags and skips the recommended-practice
  tests.

Inputs: clang-sema-expr, clang-diag-sema.

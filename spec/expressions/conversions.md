# Standard conversions

A standard conversion changes a value to another type where an operator,
an initialization, or a call needs that type. This page covers the
conversions between arithmetic types, to `bool`, and between pointer
types.

## Static Semantics

:::{rule} CONV-1
:kind: required
:tests: expressions/conversions/integral-promotion.cpp expressions/conversions/bitfield-promotes.cpp expressions/conversions/enum-promotion.cpp

These operands go through {term}`integer promotion` in arithmetic:

- `bool`, the character types, `short`, and `unsigned short`,
- a bit-field narrower than `int` *(referenced)*,
- an unscoped enumeration. With a fixed underlying type, the value
  converts to that type first.

A `float` operand does not change to `double` in arithmetic.
:::

:::{rule} CONV-8
:kind: reject
:tests: expressions/conversions/null-pointer-constants.cpp expressions/conversions/nonzero-to-pointer.cpp expressions/conversions/integer-variable-to-pointer.cpp expressions/conversions/false-not-null-pointer.cpp

`nullptr` and an integer literal with the value zero convert to a null
pointer of any pointer type. Other values must not convert to a pointer
type implicitly. This includes an integer variable with the value zero and
the literal `false`.
:::

:::{rule} CONV-9
:kind: reject
:tests: expressions/conversions/pointer-to-void-pointer.cpp expressions/conversions/void-pointer-to-pointer.cpp expressions/conversions/const-pointer-to-void-pointer.cpp expressions/conversions/function-pointer-to-void-pointer.cpp

A pointer to an object type converts implicitly to a pointer to `void`
with the same or more {term}`cv-qualifiers <cv-qualifier>`. These
conversions must not occur implicitly:

- a pointer to `void` to another pointer type,
- a conversion that removes a cv-qualifier,
- a pointer to a function to a pointer to `void`.
:::

:::{rule} CONV-10
:kind: reject
:tests: expressions/conversions/noexcept-function-pointer.cpp expressions/conversions/potentially-throwing-to-noexcept.cpp

A pointer to a `noexcept` function converts to a pointer to a function
without `noexcept`. The opposite conversion must not occur implicitly.
:::

:::{rule} CONV-12
:kind: reject
:tests: expressions/conversions/nullptr-to-bool-copy.cpp expressions/conversions/nullptr-to-bool-direct.cpp

`nullptr` converts to `bool` only in direct initialization, with
parentheses or braces. It gives `false`.
:::

:::{rule} CONV-13
:kind: required
:tests: expressions/conversions/array-function-to-pointer.cpp

An array lvalue can be used as a pointer to element zero of the array.
A function name can be used as a pointer that calls that function.
:::

:::{rule} CONV-14
:kind: required
:tests: expressions/conversions/ambiguous-base.cpp expressions/conversions/derived-to-base.cpp

A pointer or reference to a derived class converts implicitly to a pointer
or reference to a base class *(classes referenced)*. The base must be
unambiguous and accessible *(referenced)*.
:::

:::{rule} CONV-15
:kind: required
:tests: expressions/conversions/member-pointer-base-to-derived.cpp expressions/conversions/member-pointer-derived-to-base.cpp

A pointer to a member of a base class converts implicitly to a pointer to
the same member of a derived class. The opposite direction is not
implicit.
:::

:::{rule} CONV-16
:kind: required
:tests: expressions/conversions/array-pointer-unknown-bound.cpp

A pointer to an array with a known bound converts implicitly to a pointer
to an array of the same element type with no bound.
:::

## Dynamic Semantics

:::{rule} CONV-2
:kind: required
:tests: expressions/conversions/to-unsigned-modulo.cpp expressions/conversions/to-signed-modulo.cpp

An integer value that converts to another integer type keeps its low N
bits, where N is the width of the target type. The target can be signed
or unsigned. Signed types use two's complement.
:::

:::{rule} CONV-3
:kind: required
:tests: expressions/conversions/to-bool.cpp expressions/conversions/member-pointer-to-bool.cpp expressions/conversions/nan-to-bool.cpp

A conversion to `bool` gives `false` for zero, negative zero, a null
pointer, and a null pointer to member. It gives `true` for other values,
including a NaN.
:::

:::{rule} CONV-4
:kind: required
:tests: expressions/conversions/floating-to-integral-truncates.cpp

A floating-point value converts to an integer type by discarding the
fractional part. The result rounds toward zero.
:::

:::{rule} CONV-6
:kind: required
:tests: expressions/conversions/integral-to-floating-exact.cpp expressions/conversions/floating-narrowing-in-range.cpp

When a value converts to a floating-point type that can represent it
exactly, the result is that value.
:::

## Undefined Behavior

:::{rule} CONV-5
:kind: undefined
:tests: expressions/conversions/floating-to-integral-out-of-range.cpp expressions/conversions/floating-to-unsigned-negative.cpp

If a floating-point value, without its fractional part, is outside the
range of the target integer type, the conversion is undefined. This
includes a negative value that converts to an unsigned type.
:::

## Implementation Requirements

:::{rule} CONV-7
:kind: impl-defined
:tests: expressions/conversions/integral-to-floating-rounding.cpp expressions/conversions/integral-to-floating-nearest.cpp

If a value is between two adjacent values of the target floating-point
type, the result is one of the two. The implementation defines which one.
:::

:::{rule} CONV-11
:kind: impl-defined
:tests: expressions/conversions/floating-narrowing-out-of-range.cpp

If the target floating-point type has infinities, a value above its
largest finite value converts to the largest finite value or to infinity.
The implementation defines which one ({question}`Q-0028`).
:::

## Examples

```cpp
unsigned u = -1;           // ~0u
unsigned char uc = 300;    // 44: the low 8 bits
signed char sc = 200;      // -56
int i = 3.99;              // 3
int big = 1e10;            // undefined
float f = 16777217;        // 16777216.0f or 16777218.0f
int* p = 0;                // a null pointer
int zero = 0;
int* q = zero;             // rejected
bool b = nullptr;          // rejected
bool c(nullptr);           // false
```

## Evidence

The results are in `tests/RESULTS.md`.

- CONV-5: all compilers reject the conversion during constant evaluation.
- CONV-7: every compiler in the matrix rounds to the nearest value, and
  to the even value at a tie.
- CONV-11: Clang and GCC give infinity. EDG rejects the conversion during
  constant evaluation ({question}`Q-0028`).

Inputs: clang-sema-expr, clang-diag-sema.

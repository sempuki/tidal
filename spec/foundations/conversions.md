# Standard conversions

A standard conversion changes a value to another type where an operator,
an initialization, or a call needs that type. This page covers the
conversions between arithmetic types, to `bool`, and between pointer
types.

## Principles

:::{principle} CONV-P1
:rules: CONV-1 CONV-13

An operand takes the shape that the operation needs before the operation
sees it. An integer narrower than `int` becomes an `int`, and for a
bit-field the width decides, because the width is what the value can
hold. An array used as a value becomes a pointer to its first element,
and a function name becomes a pointer to the function, because the
operations on values have no use for an array or a function itself. A
`float` keeps its type, because arithmetic on `float` is an operation of
its own.
:::

:::{principle} CONV-P2
:rules: CONV-2 CONV-3
:exceptions: CONV-12

A conversion between integer types keeps the value when the target can
hold it. When it cannot, the target keeps the low bits, so the result is
the value modulo the target's range, for signed and unsigned targets
alike. A conversion to `bool` asks one question, zero or not zero, and a
null pointer counts as zero.
:::

:::{principle} CONV-P4
:rules: CONV-4 CONV-5 CONV-6 CONV-7 CONV-11

A conversion to or from a floating-point type keeps the value when the
target can hold it exactly. A floating-point target that cannot takes
one of the two nearest values, and a value beyond its range becomes the
largest finite value or infinity, by the implementation's choice. An
integer target of a floating-point value takes the value with its
fraction dropped, and if that value does not fit, there is no answer, so
the behavior is undefined.
:::

:::{principle} CONV-P3
:rules: CONV-8 CONV-9 CONV-10 CONV-14 CONV-15 CONV-16 CONV-17 CONV-18

A pointer converts implicitly only in the direction that loses no
safety: toward `void`, toward a base class, toward more cv-qualifiers at
every level, toward a function type without `noexcept`, toward a pointer
to member of a derived class, and from an array of known bound to one of
unknown bound. Each reverse direction would claim something the pointer
cannot promise, so none of them is implicit. A null pointer comes only
from `nullptr` or a literal zero, because any other value with the value
zero is a number, and a number is not a pointer. For the same reason a
pointer is not a number, so no pointer converts implicitly to an
integer. Through a base that is virtual, ambiguous, or not accessible,
there is no single path, so no implicit conversion.
:::

## Constraints

:::{rule} CONV-1
:kind: required
:tests: expressions/conversions/integral-promotion.cpp expressions/conversions/bitfield-promotes.cpp expressions/conversions/enum-promotion.cpp expressions/conversions/bitfield-promotion-by-width.cpp expressions/conversions/enum-promotion-no-fixed-type.cpp

These operands go through {term}`integer promotion` in arithmetic:

- `bool`, the character types, `short`, and `unsigned short`,
- a bit-field, by its width and not by its declared type (BF-6). A
  bit-field of width 3 declared `long long` promotes to `int`; one of
  width 40 keeps `long long`,
- an unscoped enumeration. With a fixed underlying type, the value
  converts to that type first and then promotes as that type does.
  Without one, the candidates are tried by rank from `int` upward, the
  signed type of each rank before the unsigned one, and the first that
  can hold all the values of the enumeration (ENUM-6) is the result.

A `float` operand does not change to `double` in arithmetic.
:::

:::{rule} CONV-8
:kind: reject
:tests: expressions/conversions/null-pointer-constants.cpp expressions/conversions/nonzero-to-pointer.cpp expressions/conversions/integer-variable-to-pointer.cpp expressions/conversions/false-not-null-pointer.cpp expressions/conversions/null-member-pointer-from-zero.cpp expressions/conversions/parenthesized-zero-null.cpp expressions/conversions/char-zero-not-null.cpp

`nullptr` and an integer literal with the value zero convert to a null
pointer of any pointer type and to a null pointer to member of any
member type. A zero literal in parentheses counts. A character literal
with the value zero is not an integer literal, so it does not count
({question}`Q-0100`). Other values must not convert to a pointer
type implicitly. This includes an integer variable with the value zero and
the literal `false`.
:::

:::{rule} CONV-9
:kind: reject
:tests: expressions/conversions/pointer-to-void-pointer.cpp expressions/conversions/void-pointer-to-pointer.cpp expressions/conversions/const-pointer-to-void-pointer.cpp expressions/conversions/function-pointer-to-void-pointer.cpp expressions/conversions/deep-const-every-level.cpp expressions/conversions/deep-const-not-at-every-level.cpp

A pointer to an object type converts implicitly to a pointer to `void`
with the same or more {term}`cv-qualifiers <cv-qualifier>`. These
conversions must not occur implicitly:

- a pointer to `void` to another pointer type,
- a conversion that removes a cv-qualifier,
- a pointer to a function to a pointer to `void`,
- across several levels of pointers, a conversion that adds `const` at
  one level without adding it at every level above that level.
:::

:::{rule} CONV-10
:kind: reject
:tests: expressions/conversions/noexcept-function-pointer.cpp expressions/conversions/potentially-throwing-to-noexcept.cpp expressions/conversions/member-function-noexcept-drop.cpp expressions/conversions/member-function-noexcept-add.cpp

A pointer to a `noexcept` function converts to a pointer to a function
without `noexcept`. The opposite conversion must not occur implicitly. The same applies to
pointers to member functions.
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
:tests: expressions/conversions/derived-to-base.cpp expressions/conversions/null-derived-to-second-base.cpp

A pointer or reference to a derived class converts implicitly to a pointer
or reference to a base class (DER-5). A null pointer converts to a
null pointer, also when the base subobject is at a nonzero offset.
CONV-17 lists the bases that do not allow it.
:::

:::{rule} CONV-15
:kind: required
:tests: expressions/conversions/member-pointer-base-to-derived.cpp

A member pointer that names a member of a base class can be used as a
member pointer of a derived class without a cast. It still names the
member that the base declares.
:::

:::{rule} CONV-17
:kind: reject
:tests: expressions/conversions/member-pointer-virtual-base.cpp expressions/conversions/member-pointer-ambiguous-base.cpp expressions/conversions/ambiguous-base.cpp expressions/conversions/member-pointer-derived-to-base.cpp

These conversions must not occur implicitly:

- the conversion of CONV-14 to a base that is ambiguous (DER-10) or
  not accessible *(referenced)*,
- the conversion of CONV-15 through a virtual base or an ambiguous
  base, or to a base that is not accessible (MPTR-11),
- a pointer to a member of a derived class to a pointer to a member of
  its base.
:::

:::{rule} CONV-18
:kind: reject
:tests: expressions/conversions/pointer-to-integer.cpp expressions/conversions/nullptr-to-integer.cpp expressions/conversions/member-pointer-to-integer.cpp foundations/translation/ill-formed-diagnosed.cpp

A pointer, a pointer to member, and `nullptr` must not convert
implicitly to an integer type. An array, such as a string literal,
becomes a pointer first, so it does not convert either. The conversion
to `bool` is the exception (CONV-3, CONV-12), and `reinterpret_cast` can
convert a pointer explicitly (CAST-6).
:::

:::{rule} CONV-16
:kind: required
:tests: expressions/conversions/array-pointer-unknown-bound.cpp

A pointer to an array with a known bound converts implicitly to a pointer
to an array of the same element type with no bound.
:::

## Runtime Behavior

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
:tests: expressions/conversions/floating-to-integral-out-of-range.cpp expressions/conversions/floating-to-unsigned-negative.cpp expressions/conversions/negative-fraction-to-unsigned.cpp expressions/conversions/floating-to-integral-sanitizer.cpp

If a floating-point value, without its fractional part, is outside the
range of the target integer type, the conversion is undefined. The
test is on the value after the fractional part is dropped, so a value
between -1 and 0 converts to zero, and -1 to an unsigned type is
undefined.
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
The implementation defines which one ({question}`Q-0028`). The
conversion is not undefined, so it can occur in constant evaluation.
:::

## Examples

```cpp
unsigned u = -1;           // ~0u
unsigned char uc = 300;    // 44: the low 8 bits
signed char sc = 200;      // -56
int i = 3.99;              // 3
int big = 1e10;            // undefined
float f = 16777217;        // 16777216.0f or 16777218.0f
float g = 16777219;        // 16777220.0f on every matrix target
long n = &i;               // rejected: a pointer is not a number
int* p = 0;                // a null pointer
int zero = 0;
int* q = zero;             // rejected
bool b = nullptr;          // rejected
bool c(nullptr);           // false
```

## Evidence

The results are in `tests/RESULTS.md`.

- CONV-5: all compilers reject the conversion during constant evaluation.
  At run time, the sanitizer (`-fsanitize=float-cast-overflow`) stops
  the program with a nonzero status on Clang and GCC.
- CONV-8: EDG accepts a character literal with the value zero as a null
  pointer constant. Clang and GCC reject it ({question}`Q-0100`).
- CONV-7: every compiler in the matrix rounds to the nearest value, and
  to the even value at a tie.
- CONV-11: Clang and GCC give infinity. EDG rejects the conversion during
  constant evaluation ({question}`Q-0028`).

Inputs: clang-sema-expr, clang-diag-sema.

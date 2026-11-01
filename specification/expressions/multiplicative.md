# Multiplicative operators

The operators `*` (multiply), `/` (divide), and `%` (remainder) apply to
operands of built-in types. An overloaded form of these operators is a
function call ({rule}`OPR-7`).

## Principles

:::{principle} MUL-P1
:rules: MUL-1 MUL-2 MUL-3 MUL-4 MUL-5 MUL-15 MUL-19

The multiplicative operators compute on numbers, so their operands are
the arithmetic types and the unscoped enumerations, which convert to
integers. Nothing else is a number here, so an implementation rejects
these operands:

- a pointer,
- a scoped enumeration,
- `nullptr`,
- a class, unless it supplies its own operator.

Before the operation, the two operands become one common type through
the {term}`usual arithmetic conversions`, and the result is a prvalue
of that type ({principle}`VC-P1`). The common type can depend on the
sizes of the integer types, which each implementation chooses. `%`
works on whole numbers only, because a remainder of a floating-point
division has no single meaning.
:::

:::{principle} MUL-P2
:rules: MUL-6 MUL-7 MUL-8 MUL-18
:exceptions: MUL-17

The result is the mathematical one, fitted to the type.

| Operands | How the result is fitted |
|---|---|
| Floating-point | Rounding |
| Integer, with `/` and `%` | The quotient drops the fraction, toward zero. The remainder is then whatever makes quotient and remainder add back up to the dividend. |
| Unsigned | The type is modular, so the result wraps around instead of overflowing. |

An implementation can keep extra precision in floating-point
intermediates. The x86-64 targets in the matrix do not. GCC on 32-bit
x86 does.
:::

:::{principle} MUL-P3
:rules: MUL-9 MUL-10 MUL-12

A signed integer type is not modular, so a result that the type cannot
hold has no value. The behavior is undefined in these cases:

- a signed integer result that its type cannot hold, such as the most
  negative value divided by `-1`,
- an integer division or remainder by zero, which has no mathematical
  result,
- a floating-point result beyond the range of its type.

Constant evaluation rejects each of these cases ({principle}`TR-P4`).
:::

:::{principle} MUL-P6
:rules: MUL-11 MUL-20

An implementation chooses whether each floating-point type follows
IEEE 754. A type that follows IEEE 754 has infinities and NaN values,
so a floating-point division by zero has a result in that type.

| Operation | Type follows IEEE 754 | Type does not follow IEEE 754 |
|---|---|---|
| `x / 0.0`, `x` nonzero and not a NaN | an infinity | undefined |
| `0.0 / 0.0` | a NaN | undefined |

Constant evaluation rejects a floating-point division by zero in every
type ({rule}`CE-28`), because the result depends on the machine.
:::

:::{principle} MUL-P4
:rules: MUL-13 MUL-14

The two operands are not ordered with respect to each other
({rule}`SEQ-8`). So a program must not write a scalar in one operand and
touch it in the other, unless a function call separates the two
accesses.
:::

:::{principle} MUL-P5
:rules: MUL-16

Mixing an enumeration with a floating-point value, or two different
enumerations, is deprecated, and an implementation should warn about
the mix.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   mul_expr: `pm_expr` {("*" | "/" | "%") `pm_expr`}
```

## Constraints

:::{rule} MUL-1
:kind: required
:tests: expressions/multiplicative/grouping.cpp expressions/multiplicative/grouping-mixed.cpp

`*`, `/`, and `%` have the same precedence and group from left to right.
They bind more tightly than `+` and `-`, and less tightly than unary
operators.
:::

:::{rule} MUL-2
:kind: required
:tests: expressions/multiplicative/operands-arithmetic.cpp expressions/multiplicative/operands-enum-float.cpp

The built-in `*` and `/` accept two operands of arithmetic type or
unscoped enumeration type, in any combination. Arithmetic types are
these:

- the integer types,
- the character types,
- `bool`,
- the floating-point types.
:::

:::{rule} MUL-3
:kind: reject
:tests: expressions/multiplicative/operands-pointer.cpp expressions/multiplicative/operands-scoped-enum.cpp expressions/multiplicative/operands-class.cpp expressions/multiplicative/nullptr-operand.cpp expressions/multiplicative/member-pointer-operand.cpp

An operand of `*`, `/`, or `%` must not have one of these types:

- a pointer type,
- a scoped enumeration type,
- a class type,
- a pointer-to-member type,
- the type of `nullptr`.

The class type rule does not apply if overload resolution
({rule}`OPR-6`) finds a user-declared operator or a usable conversion.
:::

:::{rule} MUL-4
:kind: reject
:tests: expressions/multiplicative/remainder-floating.cpp expressions/multiplicative/remainder-integral.cpp

The operands of `%` must have one of these types:

- an integer type,
- a character type,
- `bool`,
- an unscoped enumeration type.

An operand of `%` must not have floating-point type.
:::

:::{rule} MUL-5
:kind: required
:tests: expressions/multiplicative/result-types.cpp expressions/multiplicative/result-types-promoted.cpp expressions/multiplicative/most-negative-short-promoted.cpp

Both operands go through the {term}`usual arithmetic conversions`. The
result has the common type and is a prvalue (see
{term}`value category`).
:::

:::{rule} MUL-19
:kind: reject
:tests: expressions/multiplicative/result-prvalue-address.cpp expressions/multiplicative/result-prvalue-assign.cpp

A program cannot assign to the result of MUL-5 or take its address.
:::

## Runtime Behavior

:::{rule} MUL-6
:kind: required
:tests: expressions/multiplicative/division-truncates.cpp expressions/multiplicative/division-most-negative.cpp expressions/multiplicative/remainder-run.cpp

For integer operands, `a / b` is the mathematical quotient without its
fractional part. The quotient rounds toward zero.
:::

:::{rule} MUL-7
:kind: required
:tests: expressions/multiplicative/remainder-sign.cpp expressions/multiplicative/division-most-negative.cpp expressions/multiplicative/remainder-run.cpp

For integer operands, `a % b` is the value that makes
`(a / b) * b + a % b` equal to `a`. A nonzero remainder has the sign of
`a`.
:::

:::{rule} MUL-8
:kind: required
:tests: expressions/multiplicative/unsigned-wraps.cpp expressions/multiplicative/unsigned-wraps-wide.cpp

If the common type is an unsigned integer type, the result wraps around.
The result differs from the mathematical result by a multiple of one
more than the largest value of the type.
:::

:::{rule} MUL-18
:kind: required
:tests: expressions/multiplicative/product-value.cpp expressions/multiplicative/product-value-run.cpp

For arithmetic operands, `*` gives the mathematical product and `/` on
floating-point operands gives the mathematical quotient. For a
floating-point type, the result is rounded to the type
({rule}`TYP-14`).
:::

## Undefined Behavior

:::{rule} MUL-9
:kind: undefined
:tests: expressions/multiplicative/divide-by-zero.cpp expressions/multiplicative/remainder-by-zero.cpp expressions/multiplicative/divide-by-zero-sanitizer.cpp

Integer division or remainder by zero is undefined.
:::

:::{rule} MUL-10
:kind: undefined
:tests: expressions/multiplicative/signed-overflow-multiply.cpp expressions/multiplicative/signed-overflow-divide.cpp expressions/multiplicative/signed-overflow-remainder.cpp expressions/multiplicative/signed-overflow-after-promotion.cpp expressions/multiplicative/signed-overflow-divide-sanitizer.cpp expressions/multiplicative/signed-overflow-multiply-sanitizer.cpp expressions/multiplicative/signed-overflow-remainder-sanitizer.cpp expressions/multiplicative/signed-overflow-divide-long-long.cpp expressions/multiplicative/most-negative-short-promoted.cpp

If the common type is a signed integer type, these conditions make the
behavior undefined:

- the mathematical result of `*` or `/` does not fit in the type,
- `/` on the same operands has undefined behavior, for `%`.

The most negative value of the type, with a right operand of `-1`, is
such a case for `/` and for `%`.
{term}`Integer promotion` can give a signed type for unsigned operands.
:::

:::{rule} MUL-12
:kind: undefined
:tests: expressions/multiplicative/floating-overflow-constant.cpp expressions/multiplicative/floating-overflow-divide-constant.cpp

If the mathematical result of floating-point `*` or `/` is finite and
larger in magnitude than the largest finite value of the type, the
behavior is undefined. This rule is provisional ({question}`Q-0001`).
:::

:::{rule} MUL-14
:kind: undefined
:tests: expressions/multiplicative/unsequenced-modification.cpp expressions/multiplicative/unsequenced-modification-remainder.cpp

If one operand modifies a scalar object and the other operand reads or
modifies the same object, the behavior is undefined. This rule does not
apply if a function call separates the two accesses.
:::

## Implementation Requirements

:::{rule} MUL-13
:kind: unspecified
:tests: expressions/multiplicative/operand-order.cpp expressions/multiplicative/operand-order-overloaded.cpp

The order of evaluation of the two operands is unspecified.
:::

:::{rule} MUL-15
:kind: impl-defined
:tests: expressions/multiplicative/mixed-long-unsigned-lp64.cpp expressions/multiplicative/mixed-long-unsigned-ilp32.cpp

The common type, and so the result type, can depend on the sizes of the
integer types. The sizes of the integer types are
implementation-defined.
:::

:::{rule} MUL-20
:kind: impl-defined
:tests: expressions/multiplicative/ieee-types.cpp expressions/multiplicative/floating-divide-by-zero-infinity.cpp expressions/multiplicative/floating-zero-by-zero-nan.cpp

Whether a floating-point type follows IEEE 754 is implementation-defined.
A type that follows IEEE 754 has a positive and a negative infinity and
NaN values. On the x86-64 targets in the matrix, `float`, `double`, and
`long double` follow IEEE 754.
:::

:::{rule} MUL-11
:kind: impl-defined
:tests: expressions/multiplicative/floating-divide-by-zero-infinity.cpp expressions/multiplicative/floating-zero-by-zero-nan.cpp expressions/multiplicative/floating-divide-by-zero-sanitizer.cpp

If the right operand of floating-point `/` is zero, the result depends
on the type ({rule}`MUL-20`):

- If the type does not follow IEEE 754, the behavior is undefined.
- If the left operand is zero or a NaN, the result is a NaN.
- Otherwise the result is an infinity. The infinity is negative if
  exactly one operand is negative or is `-0.0`.
:::

:::{rule} MUL-17
:kind: unspecified
:tests: expressions/multiplicative/excess-precision-either.cpp expressions/multiplicative/cast-rounds-to-type.cpp expressions/multiplicative/cast-rounds-to-type-ilp32.cpp

An implementation can evaluate floating-point operations with more range
and precision than the type of the expression. The extra range and
precision apply to the additive operators too. These rules apply:

- The type of the expression does not change.
- A cast or an assignment rounds the value to its type.
- On the x86-64 targets in the matrix, each operation rounds to its
  type.
- On the 32-bit x86 target, GCC keeps extra precision, and Clang and
  EDG round each operation.
:::

## Recommended Practice

:::{rule} MUL-16
:kind: recommended
:tests: expressions/multiplicative/enum-float-warns.cpp expressions/multiplicative/enum-enum-warns.cpp

An implementation should warn about `*`, `/`, or `%` between an
enumeration and a floating-point value, or between two different
enumeration types.
:::

## Examples

```cpp
12 / 3 * 2           // (12 / 3) * 2, which is 8
2 + 3 * 4            // 2 + (3 * 4), which is 14

char c = 1;
c * c                // type int

7 / 2                // 3
-7 / 2               // -3
7 / -2               // -3
-7 % 2               // -1
7 % -2               // 1

~0u * 2u             // ~0u - 1u

double zero = 0.0;
1.0 / zero           // positive infinity if double follows IEEE 754
-1.0 / zero          // negative infinity if double follows IEEE 754
zero / zero          // a NaN if double follows IEEE 754

unsigned short s = 65535;
s * s                // int; undefined if int has 32 bits

int i = 1;
i++ * i              // undefined

int q = left() / right();   // left() or right() can execute first

-1L / 2u             // 0 if long has 64 bits and unsigned has 32 bits
                     // ULONG_MAX / 2 if both have 32 bits

int lowest = -2147483647 - 1;   // int has 32 bits
lowest / 2           // -1073741824
lowest % 3           // -2
lowest / -1          // undefined
lowest % -1          // undefined, although the remainder would be 0

short low = -32768;
low / short(-1)      // 32768: both operands become int
```

## Evidence

The results for each compiler in the matrix are in `tests/RESULTS.md`.
All eight compilers agree on 47 of 52 tests.

- **MUL-2, MUL-16.** Clang and GCC warn that an enumeration with a
  floating-point operand is deprecated
  (`-Wdeprecated-enum-float-conversion`). Both compilers warn about two
  different enumerations under `-Wdeprecated-enum-enum-conversion`. Each
  warning is on by default, and `-pedantic-errors` does not make it an
  error, so both compilers accept the operands in strict mode. The
  MUL-16 tests turn the warning into an error with `-Werror`.
- **MUL-9, MUL-10.** Clang and GCC (all tested versions) reject
  each case during constant evaluation. Their notes name the cause:
  division by zero, a result too large for `int`, or overflow. Each case
  is otherwise valid. EDG agrees on integer division by zero and on
  multiply overflow. EDG differs on the cases below.
- **MUL-10, the most negative value.** At run time, `INT_MIN / -1` and
  `INT_MIN % -1` stop the program with a hardware exception on every
  Clang and GCC configuration, `-O2` included. MUL-10 allows this
  result. `-fsanitize=undefined` reports both operators with the same
  words. Integer promotion makes the most negative `short` divided by
  `-1` an `int` division, which every compiler evaluates to `32768`.
- **MUL-10, EDG.** Clang and GCC reject `INT_MIN / -1` and
  `INT_MIN % -1` during constant evaluation, in `int` and in
  `long long`. EDG 7.0 accepts both, and its quotient lies outside the
  range of `int`. EDG rejects `-INT_MIN` and `INT_MIN * -1`. The
  provisional answer keeps MUL-10, and EDG's behavior is a possible
  defect ({question}`Q-0003`).
- **MUL-11, MUL-20.** At run time on x86-64, every Clang and GCC
  configuration, `-O2` included, gives an infinity for `1.0 / 0.0` and a
  NaN for `0.0 / 0.0` in `float`, `double`, and `long double`.
  `-fsanitize=undefined` does not stop the division. The separate check
  `-fsanitize=float-divide-by-zero` stops the division. Clang and GCC
  report `__STDC_IEC_559__` and IEEE 754 types in their libraries on the
  64-bit and 32-bit x86 targets. MUL-11 is `impl-defined` because the
  result rests on a choice of the implementation, which these
  implementations make and report ({question}`Q-0004`, settled). Clang
  and GCC reject the division during constant evaluation (CE-28).
- **CE-28, EDG.** Clang and GCC reject `1.0 / 0.0` and `0.0 / 0.0`
  during constant evaluation. EDG 7.0 accepts both and gives an
  infinity and a NaN. EDG rejects floating overflow. The provisional
  answer keeps the rejection, and EDG's behavior is a possible defect
  ({question}`Q-0002`).
- **MUL-12.** These are the results for an overflow of `*` or `/` in a
  `constexpr` initializer, in `float` or `double`:

  - Clang 22.1.8 and 18.1.3 accept it and give an infinity.
  - GCC 16.2.1, 16.2.0, and 14.2.0 reject it ("not a constant
    expression").
  - EDG 7.0 rejects it ("floating-point error").

  Clang rejects a result that is a NaN. At run time every Clang and GCC
  configuration gives an infinity, and `-fsanitize=undefined` lets the
  overflow run. The provisional answer is that the behavior is
  undefined, and that Clang's acceptance is an extension
  ({question}`Q-0001`).
- **MUL-13.** Clang and GCC evaluate the left operand of the built-in
  operators first at `-O0` and at `-O2`, in every form probed. For an
  overloaded `*` or `/`, GCC evaluates the right operand first. This
  result shows that the order is not fixed. The tests accept either
  order ({question}`Q-0005`).
- **MUL-14.** Clang warns "unsequenced modification and access" under
  `-Wunsequenced`, which is on by default. GCC warns "operation ... may
  be undefined" under `-Wsequence-point`, which `-Wall` turns on. GCC
  gives no warning without `-Wall` or the named flag. The tests change
  these warnings into errors. EDG does not accept these warning flags
  and skips the tests.
- **MUL-15.** The result type on x86-64 (LP64) differs from the result
  type with `-m32` (ILP32). The tests depend on `sizeof(long)`.
- **MUL-17.** `0.1 * 3.0 / 3.0` gives `0.10000000000000002` when each
  operation rounds to `double`, and `0.1` with extra precision. The
  test accepts both values and stays clear of overflow, which MUL-12
  makes undefined ({question}`Q-0001`). With `-m32`, GCC keeps `float`
  and `double` intermediates in the x87 format during constant
  evaluation, so `1.0 + 0x1p-53 != 1.0` holds there. Clang and EDG give
  `1.0` for the sum. A cast, an initializer, and a return value round
  the sum to `1.0` on every compiler. MUL-17 allows both results.
- MUL-12: no sanitizer of GCC or Clang stops a floating-point `*` whose
  result is outside the range of its type.
- MUL-14: no sanitizer of GCC or Clang stops an unsequenced
  modification. The compilers warn at translation instead
  (`unsequenced-modification.cpp`).

Inputs: clang-sema-expr.

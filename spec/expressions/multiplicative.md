# Multiplicative operators

The operators `*` (multiply), `/` (divide), and `%` (remainder) apply to
operands of built-in types. An overloaded form of these operators is a
function call ({rule}`OPR-7`).

## Principles

:::{principle} MUL-P1
:rules: MUL-1 MUL-2 MUL-3 MUL-4 MUL-5 MUL-15 MUL-19

The multiplicative operators compute on numbers, so their operands are
the arithmetic types and the unscoped enumerations, which convert to
integers. Nothing else is a number here.
These are rejected:

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
intermediates, and the matrix targets do not.
:::

:::{principle} MUL-P3
:rules: MUL-9 MUL-10 MUL-11 MUL-12

A signed integer type is not modular, so a result it cannot hold has no
value. The behavior is undefined in these cases:

- a signed integer result that its type cannot hold,
- an operation with no mathematical result, such as division by zero,
- a floating-point result beyond the range of its type.

Constant evaluation rejects each of these ({principle}`TR-P4`).
:::

:::{principle} MUL-P4
:rules: MUL-13 MUL-14

The two operands are not ordered with respect to each other
({rule}`SEQ-8`).
So a program must not write a scalar in one operand and touch it in the
other, unless a function call separates the two accesses.
:::

:::{principle} MUL-P5
:rules: MUL-16

Mixing an enumeration with a floating-point value, or two different
enumerations, is deprecated, and an implementation should say so.
:::

## Syntax

```{eval-rst}
.. productionlist:: multiplicative
   mul_expr: `operand` (("*" | "/" | "%") `operand`)*
   operand: `unary_expression`
```

## Constraints

:::{rule} MUL-1
:kind: required
:tests: expressions/multiplicative/grouping.cpp

`*`, `/`, and `%` have the same precedence. They group from left to right.
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
:tests: expressions/multiplicative/result-types.cpp

Both operands go through the {term}`usual arithmetic conversions`. The
result has the common type and is a prvalue (see {term}`value category`).
:::

:::{rule} MUL-19
:kind: reject
:tests: expressions/multiplicative/result-prvalue-address.cpp expressions/multiplicative/result-prvalue-assign.cpp

A program cannot assign to the result of MUL-5 or take its address.
:::

## Runtime Behavior

:::{rule} MUL-6
:kind: required
:tests: expressions/multiplicative/division-truncates.cpp

For integer operands, `a / b` is the mathematical quotient without its
fractional part. The quotient rounds toward zero.
:::

:::{rule} MUL-7
:kind: required
:tests: expressions/multiplicative/remainder-sign.cpp

For integer operands, `a % b` is the value that makes
`(a / b) * b + a % b` equal to `a`. A nonzero remainder has the sign of `a`.
:::

:::{rule} MUL-8
:kind: required
:tests: expressions/multiplicative/unsigned-wraps.cpp

If the common type is an unsigned integer type, the result wraps around.
The result differs from the mathematical result by a multiple of one more
than the largest value of the type.
:::

:::{rule} MUL-18
:kind: required
:tests: expressions/multiplicative/product-value.cpp

For arithmetic operands, `*` gives the mathematical product and `/` on
floating-point operands gives the mathematical quotient. For a
floating-point type, the result is rounded to the type.
:::

## Undefined Behavior

:::{rule} MUL-9
:kind: undefined
:tests: expressions/multiplicative/divide-by-zero.cpp expressions/multiplicative/remainder-by-zero.cpp expressions/multiplicative/divide-by-zero-sanitizer.cpp

Integer division or remainder by zero is undefined.
:::

:::{rule} MUL-10
:kind: undefined
:tests: expressions/multiplicative/signed-overflow-multiply.cpp expressions/multiplicative/signed-overflow-divide.cpp expressions/multiplicative/signed-overflow-remainder.cpp expressions/multiplicative/signed-overflow-after-promotion.cpp expressions/multiplicative/signed-overflow-divide-sanitizer.cpp expressions/multiplicative/signed-overflow-multiply-sanitizer.cpp

If the common type is a signed integer type, these conditions make the
behavior undefined:

- the mathematical result of `*` or `/` does not fit in the type,
- `/` on the same operands has undefined behavior, for `%`.

{term}`Integer promotion` can give a signed type for unsigned operands.
:::

:::{rule} MUL-11
:kind: undefined
:tests: expressions/multiplicative/floating-divide-by-zero.cpp expressions/multiplicative/floating-zero-by-zero.cpp expressions/multiplicative/floating-divide-by-zero-sanitizer.cpp

If the right operand of floating-point `/` is zero, the behavior is
undefined. This includes `0.0 / 0.0`.
:::

:::{rule} MUL-12
:kind: undefined
:tests: expressions/multiplicative/floating-overflow-constant.cpp

If the mathematical result of floating-point `*` or `/` is finite and
larger in magnitude than the largest finite value of the type, the
behavior is undefined. This rule is provisional ({question}`Q-0001`).
:::

:::{rule} MUL-14
:kind: undefined
:tests: expressions/multiplicative/unsequenced-modification.cpp

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
integer types. The sizes of the integer types are implementation-defined.
:::

:::{rule} MUL-17
:kind: unspecified
:tests: expressions/multiplicative/excess-precision-either.cpp expressions/multiplicative/cast-rounds-to-type.cpp

An implementation can evaluate floating-point operations with more range
and precision than the type of the expression. The type does not change.
A cast or an assignment rounds the value to its type. This applies to
the additive operators too. On the x86-64 targets in the matrix, each
operation rounds to its type.
:::

## Recommended Practice

:::{rule} MUL-16
:kind: recommended
:tests: expressions/multiplicative/enum-float-warns.cpp

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

unsigned short s = 65535;
s * s                // int; undefined if int has 32 bits

int i = 1;
i++ * i              // undefined

int q = left() / right();   // left() or right() can execute first

-1L / 2u             // 0 if long has 64 bits and unsigned has 32 bits
                     // ULONG_MAX / 2 if both have 32 bits
```

## Evidence

The results for each compiler in the matrix are in `tests/RESULTS.md`.

- **MUL-2.** Clang and GCC warn that an enumeration with a floating-point
  operand is deprecated. They accept it.
- **MUL-9, MUL-10, MUL-11.** Clang and GCC (all tested versions) reject
  each case during constant evaluation. Their notes name the cause:
  division by zero, a result too large for `int`, or overflow. Each case is
  otherwise valid. EDG agrees on integer division by zero and on multiply
  overflow. EDG differs on the cases below.
- **MUL-11.** At run time on x86-64, Clang and GCC give IEEE 754 infinity or
  NaN. Tidal does not require that result ({question}`Q-0004`).
- **MUL-12.** These are the results for `1e308 * 10.0` in a `constexpr`
  initializer:

  - Clang 22.1.8 and 18.1.3 accept it and give positive infinity.
  - GCC 16.2.1 and 14.2.0 reject it ("not a constant expression").
  - EDG 7.0 rejects it ("floating-point error").

  The provisional answer is that the behavior is undefined, and that
  Clang's acceptance is an extension ({question}`Q-0001`).
- **MUL-11, EDG.** Clang and GCC reject `1.0 / 0.0` and `0.0 / 0.0` during
  constant evaluation. EDG 7.0 accepts both. EDG rejects floating overflow.
  The provisional answer keeps MUL-11, and EDG's behavior is a possible
  defect ({question}`Q-0002`).
- **MUL-10, EDG.** Clang and GCC reject `INT_MIN / -1` and `INT_MIN % -1`
  during constant evaluation. EDG 7.0 accepts both. EDG rejects
  `INT_MAX * 2`. The provisional answer keeps MUL-10, and EDG's behavior is
  a possible defect ({question}`Q-0003`).
- **MUL-13.** Clang and GCC evaluate the left operand first at `-O0` and at
  `-O2`. For an overloaded `*`, GCC evaluates the right operand first.
  This shows that the order for `*` is not fixed. The test
  accepts either order ({question}`Q-0005`).
- **MUL-14.** Clang warns "unsequenced modification and access". GCC warns
  "operation ... may be undefined". The test changes these warnings into
  errors. EDG does not accept these warning flags and skips the test.
- **MUL-15.** The result type on x86-64 (LP64) differs from the result type
  with `-m32` (ILP32). The tests depend on `sizeof(long)`.

Inputs: clang-sema-expr.

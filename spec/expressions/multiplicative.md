# Multiplicative operators

The operators `*` (multiply), `/` (divide), and `%` (remainder) applied to
operands of built-in types. Overloaded forms of these operators are calls to
functions and are covered by overload resolution *(referenced)*.

Depends on: [usual arithmetic conversions](../glossary.md#usual-arithmetic-conversions),
[value categories](../glossary.md#value-category),
[evaluation order](../glossary.md#unsequenced), floating-point types *(referenced)*,
overload resolution *(referenced)*.

Inputs: clang-sema-expr, gcc-cp-typeck (see `inputs/MANIFEST.md`).

## Syntax

    mul-expr  = operand { ( "*" | "/" | "%" ) operand }
    operand   = unary-expression *(referenced)*, or a cast written as one

## Rules

### MUL-1 (required)

`*`, `/`, and `%` have equal precedence and group from left to right. They
bind more tightly than `+` and `-` and less tightly than unary operators.

    12 / 3 * 2    // (12 / 3) * 2 == 8
    2 + 3 * 4     // 2 + (3 * 4) == 14

Tests: expressions/multiplicative/grouping.cpp

### MUL-2 (required)

The built-in `*` and `/` accept two operands of arithmetic type (integer,
character, `bool`, or floating-point) or unscoped enumeration type, in any
combination. Mixing an enumeration with a floating-point operand is accepted,
though both compilers warn that it is deprecated.

Tests: expressions/multiplicative/operands-arithmetic.cpp, expressions/multiplicative/operands-enum-float.cpp

### MUL-3 (reject)

A program is rejected if an operand of `*`, `/`, or `%` has pointer type,
scoped enumeration type, or class type, unless overload resolution
*(referenced)* finds a user-declared operator or a conversion that makes the
built-in operator usable.

Tests: expressions/multiplicative/operands-pointer.cpp, expressions/multiplicative/operands-scoped-enum.cpp, expressions/multiplicative/operands-class.cpp

### MUL-4 (reject)

`%` accepts only integer, character, `bool`, and unscoped enumeration
operands. A program is rejected if either operand of `%` has floating-point
type.

Tests: expressions/multiplicative/remainder-floating.cpp, expressions/multiplicative/remainder-integral.cpp

### MUL-5 (required)

Both operands go through the
[usual arithmetic conversions](../glossary.md#usual-arithmetic-conversions),
which bring them to one common type. The result has that type and is a
[prvalue](../glossary.md#value-category), so it can't be assigned to or have
its address taken. Operands narrower than `int` are promoted first, so
`char * char` has type `int`.

Tests: expressions/multiplicative/result-types.cpp, expressions/multiplicative/result-prvalue-address.cpp, expressions/multiplicative/result-prvalue-assign.cpp

### MUL-6 (required)

For integer operands, `a / b` is the mathematical quotient with any
fractional part dropped, which rounds toward zero.

    7 / 2 == 3      -7 / 2 == -3      7 / -2 == -3

Tests: expressions/multiplicative/division-truncates.cpp

### MUL-7 (required)

For integer operands, `a % b` is the value that makes `(a / b) * b + a % b`
equal `a`. So a nonzero remainder has the same sign as `a`.

    -7 % 2 == -1      7 % -2 == 1

Tests: expressions/multiplicative/remainder-sign.cpp

### MUL-8 (required)

When the common type is an unsigned integer type, results wrap around. If
the mathematical result doesn't fit, the value kept is the one that differs
from it by a multiple of one more than the type's largest value. Unsigned
multiplication never overflows.

    ~0u * 2u == ~0u - 1u

Tests: expressions/multiplicative/unsigned-wraps.cpp

### MUL-9 (undefined)

Integer `/` or `%` with a right operand of zero has undefined behavior.

Tests: expressions/multiplicative/divide-by-zero.cpp, expressions/multiplicative/remainder-by-zero.cpp

### MUL-10 (undefined)

When the common type is a signed integer type and the mathematical result of
`*` or `/` does not fit in it, the behavior is undefined. `%` has undefined
behavior whenever `/` on the same operands would, so `INT_MIN % -1` is
undefined even though its mathematical value, zero, would fit.

Promotion can produce a signed type from unsigned operands. Two
`unsigned short` values of 65535 are promoted to `int`, and on a target
where `int` has 32 bits their product does not fit.

Tests: expressions/multiplicative/signed-overflow-multiply.cpp, expressions/multiplicative/signed-overflow-divide.cpp, expressions/multiplicative/signed-overflow-remainder.cpp, expressions/multiplicative/signed-overflow-after-promotion.cpp

### MUL-11 (undefined)

Floating-point `/` with a right operand of zero has undefined behavior,
including `0.0 / 0.0`.

Tests: expressions/multiplicative/floating-divide-by-zero.cpp, expressions/multiplicative/floating-zero-by-zero.cpp

### MUL-12 (undefined)

*Provisional, see question Q-0001.* A floating-point `*` or `/` whose
mathematical result is finite but larger in magnitude than the largest finite
value of the result type has undefined behavior.

Tests: expressions/multiplicative/floating-overflow-constant.cpp

### MUL-13 (unspecified)

The two operands may be evaluated in either order.

    int q = left() / right();   // left() or right() may be called first

Tests: expressions/multiplicative/operand-order.cpp

### MUL-14 (undefined)

If evaluating one operand modifies a scalar object and evaluating the other
operand reads or modifies the same object, and the two accesses are not
separated by a function call, the behavior is undefined.

    i++ * i     // undefined

Tests: expressions/multiplicative/unsequenced-modification.cpp

### MUL-15 (impl-defined)

The common type of the operands, and so the result type, can depend on the
sizes of the integer types, which each implementation chooses. For example,
`long` and `unsigned` operands produce `long` when `long` is wider than
`unsigned`, and `unsigned long` when they are the same width.

    -1L / 2u    // 0 where long is 64 bits and unsigned is 32 bits,
                // ULONG_MAX / 2 where both are 32 bits

Tests: expressions/multiplicative/mixed-long-unsigned-lp64.cpp, expressions/multiplicative/mixed-long-unsigned-ilp32.cpp

## Evidence

Results for every compiler in the matrix are in `tests/RESULTS.md`.

- **MUL-9, MUL-10, MUL-11.** Clang and GCC (all tested versions) reject each
  case during constant evaluation. Their notes name the cause: division by
  zero, a result too large for `int`, or overflow. Compilers reject undefined operations
  during constant evaluation, and every case here is otherwise well formed.
  EDG agrees on integer division by zero and multiply overflow, and differs
  on the cases listed under MUL-D2 and MUL-D3.
- **MUL-11.** At run time Clang and GCC produce IEEE 754 infinity or NaN on
  x86-64. Tidal does not require that result (question Q-0004).
- **MUL-13.** Clang and GCC evaluate the left operand first at `-O0` and
  `-O2`, so no variation is observed. GCC's option that forces left-to-right
  evaluation applies to subscripts and shifts only (sanitized source,
  `flag_strong_eval_order` in `typeck.cc`), which indicates that the order of
  `*`, `/`, and `%` operands is not fixed. The test accepts either order
  (question Q-0005).
- **MUL-14.** Clang warns "unsequenced modification and access" and GCC warns
  "operation ... may be undefined". The test turns those warnings into
  errors. EDG doesn't take these warning flags and skips the test.
- **MUL-15.** The result type differs between x86-64 (LP64) and `-m32`
  (ILP32). The tests are written to depend on `sizeof(long)`.

## Divergences

### MUL-D1: floating-point overflow during constant evaluation

**Test:** `expressions/multiplicative/floating-overflow-constant.cpp`
evaluates `1e308 * 10.0` in a `constexpr` initializer.

| Compiler | Result |
|---|---|
| Clang 22.1.8, 18.1.3 | accepts; the value is positive infinity |
| GCC 16.2.1, 14.2.0 | rejects: "`(1.0e+308 * 1.0e+1)` is not a constant expression" |
| EDG 7.0 | rejects: "floating-point error" |

**Provisional answer:** `undefined`, as MUL-12 states. Clang's acceptance is
an extension. Options and opinions are in question Q-0001.

### MUL-D2: EDG accepts floating-point division by zero during constant evaluation

**Tests:** `expressions/multiplicative/floating-divide-by-zero.cpp`,
`expressions/multiplicative/floating-zero-by-zero.cpp`.

Clang and GCC reject `1.0 / 0.0` and `0.0 / 0.0` during constant evaluation.
EDG 7.0 accepts both. EDG does reject floating overflow (MUL-D1), so it
treats some floating-point results as errors and not others.

**Provisional answer:** MUL-11 stays `undefined`, and EDG's behavior is a
possible EDG defect. See question Q-0002.

### MUL-D3: EDG accepts the most negative int divided by -1 during constant evaluation

**Tests:** `expressions/multiplicative/signed-overflow-divide.cpp`,
`expressions/multiplicative/signed-overflow-remainder.cpp`.

Clang and GCC reject `INT_MIN / -1` and `INT_MIN % -1` during constant
evaluation. EDG 7.0 accepts both. EDG rejects `INT_MAX * 2`, so it detects
signed overflow in multiplication but not in this division case.

**Provisional answer:** MUL-10 stays `undefined`, and EDG's behavior is a
possible EDG defect. See question Q-0003.

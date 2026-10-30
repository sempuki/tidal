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

When the common type is an unsigned integer type, the result is the
mathematical result reduced modulo 2^N, where N is the number of bits in that
type. Unsigned multiplication never overflows.

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

*Proposed, pending adjudication of MUL-D1.* A floating-point `*` or `/` whose
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

- **MUL-9, MUL-10, MUL-11.** Both compilers reject each case during constant
  evaluation. Their notes name the cause ("division by zero", "value ... is
  outside the range of representable values", "overflow in constant
  expression"). Compilers reject undefined operations during constant
  evaluation, and every case here is otherwise well formed.
- **MUL-11.** At run time both compilers produce IEEE 754 infinity or NaN on
  the tested target. Tidal does not require that result. A later revision
  could make it `impl-defined` for implementations that document IEEE 754
  arithmetic.
- **MUL-13.** Both compilers evaluated the left operand first at `-O0` and
  `-O2`, so no variation was seen. GCC has an option that forces left-to-right
  evaluation for some operators. Its sanitized source applies that option to
  subscripts and shifts only, which indicates that the order of `*`, `/`, and
  `%` operands is not fixed. The test accepts either order.
- **MUL-14.** Clang warns "unsequenced modification and access" and GCC warns
  "operation ... may be undefined". The test turns those warnings into errors.
- **MUL-15.** The same assertions hold on x86-64 (LP64) and `-m32` (ILP32)
  only when written to depend on `sizeof(long)`. The result type differs
  between the two targets.

## Divergences

### MUL-D1: floating-point overflow during constant evaluation

**Test:** `expressions/multiplicative/floating-overflow-constant.cpp`
evaluates `1e308 * 10.0` in a `constexpr` initializer.

**Clang 22.1.8:** accepts. The value is positive infinity.
**GCC 16.2.1:** rejects. "`(1.0e+308 * 1.0e+1)` is not a constant expression".

Both compilers reject division by zero during constant evaluation (MUL-11),
so they agree that some floating-point results are not allowed there. They
disagree only on overflow.

**Options:**

1. Classify floating overflow as `undefined` (MUL-12 as drafted). GCC's
   behavior matches. Clang's acceptance is then an extension.
2. Classify it as `impl-defined`. Both compilers conform, and a program can't
   rely on either outcome without checking the implementation's
   documentation.

**Writer's recommendation:** option 1. It is the conservative choice. A
program that depends on overflow being accepted in constant evaluation is
not portable today.

**Adjudicator's decision:** pending.

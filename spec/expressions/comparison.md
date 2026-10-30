# Comparison operators

The relational operators `<`, `>`, `<=`, and `>=` and the equality
operators `==` and `!=` compare two values and give a `bool`. The
three-way comparison operator `<=>` gives an ordering value.

## Syntax

```{eval-rst}
.. productionlist:: cmp
   equality: `relational` {("==" | "!=") `relational`}
   relational: `three_way` {("<" | ">" | "<=" | ">=") `three_way`}
   three_way: `shift_expr` {"<=>" `shift_expr`}
```

## Legality Rules

:::{rule} CMP-1
:kind: required
:tests: expressions/comparison/grouping.cpp expressions/comparison/chained-comparison.cpp

`<=>` binds more tightly than the relational operators. The relational
operators bind more tightly than `==` and `!=`. Each group associates from
left to right. In `a < b < c`, the value of `a < b` is compared with `c`.
:::

:::{rule} CMP-2
:kind: required
:tests: expressions/comparison/result-type.cpp expressions/comparison/arithmetic-conversions.cpp

Operands of arithmetic or unscoped enumeration type go through the
{term}`usual arithmetic conversions`. The result of a relational or
equality operator is a prvalue of type `bool`.
:::

:::{rule} CMP-6
:kind: required
:tests: expressions/comparison/pointer-void.cpp expressions/comparison/pointer-qualification.cpp

Two pointers can be compared if one converts to the type of the other. The
conversion can add {term}`cv-qualifiers <cv-qualifier>` or convert to
`void*`. A pointer can be compared with `nullptr` for equality.
:::

:::{rule} CMP-9
:kind: reject
:tests: expressions/comparison/pointer-integer.cpp expressions/comparison/pointer-relational-zero.cpp expressions/comparison/pointer-different-types.cpp expressions/comparison/function-pointer-void.cpp expressions/comparison/pointer-relational-nullptr.cpp

A comparison must not have these operands:

- a pointer and an integer other than a literal `0` for equality,
- a pointer and the literal `0` or `nullptr` for a relational operator,
- pointers to unrelated types,
- a pointer to a function and `void*`.
:::

:::{rule} CMP-10
:kind: required
:tests: expressions/comparison/scoped-enum-same.cpp expressions/comparison/scoped-enum-different.cpp

Two values of the same scoped enumeration can be compared. Values of two
different scoped enumerations must not be compared.
:::

:::{rule} CMP-11
:kind: required
:tests: expressions/comparison/array-comparison.cpp expressions/comparison/three-way-arrays.cpp

If both operands of `==`, `!=`, `<`, `>`, `<=`, or `>=` are arrays, each
converts to a pointer to its first element. The comparison then compares
the addresses. `<=>` must not have two array operands.
:::

:::{rule} CMP-12
:kind: required
:tests: expressions/comparison/three-way-type.cpp expressions/comparison/three-way-no-library.cpp expressions/comparison/three-way-precedence.cpp expressions/comparison/three-way-function-pointers.cpp

`<=>` on integer, enumeration, or object pointer operands gives
`std::strong_ordering`. On floating-point operands it gives
`std::partial_ordering`. These types come from the library *(referenced)*,
and must be declared before `<=>` is used. `<=>` must not compare two
function pointers.
:::

:::{rule} CMP-13
:kind: reject
:tests: expressions/comparison/three-way-narrowing.cpp expressions/comparison/three-way-bool-int.cpp expressions/comparison/three-way-pointer-zero.cpp

The operands of `<=>` must not need a narrowing conversion. A `bool` must
not be compared with another type. A pointer must not be compared with the
literal `0`.
:::

:::{rule} CMP-18
:kind: reject
:tests: expressions/comparison/three-way-different-enums.cpp expressions/comparison/three-way-enum-float.cpp expressions/comparison/three-way-enum-int.cpp

`<=>` must not compare two different unscoped enumeration types, or an
unscoped enumeration and a floating-point value. An unscoped enumeration
and an integer are valid ({question}`Q-0050` for the floating-point case).
:::

:::{rule} CMP-19
:kind: required
:tests: expressions/comparison/derived-base-pointer-equality.cpp expressions/comparison/multilevel-qualification.cpp expressions/comparison/null-pointer-equality.cpp

Equality comparison is valid for these pointer operands:

- pointers whose types differ in cv-qualifiers at any level. The
  comparison uses a type that adds `const` where it is needed.
- a pointer to a derived class and a pointer to its base class, after
  CONV-14,
- two values of the type of `nullptr`, which compare equal,
- two pointers to members, where two null values compare equal.
:::

## Dynamic Semantics

:::{rule} CMP-3
:kind: required
:tests: expressions/comparison/signed-unsigned.cpp

The comparison uses the values after conversion. If an `int` is
converted to `unsigned`, `-1` becomes the largest `unsigned` value.
:::

:::{rule} CMP-5
:kind: required
:tests: expressions/comparison/nan.cpp

If an operand is a NaN, `<`, `>`, `<=`, `>=`, and `==` give `false`.
`!=` gives `true`.
:::

:::{rule} CMP-7
:kind: required
:tests: expressions/comparison/pointer-array-order.cpp expressions/comparison/pointer-member-order.cpp

Pointers to elements of the same array compare in the order of their
indexes. A pointer one past the last element is greater than all element
pointers. Pointers to data members of one object compare in the order of
the member declarations (CLS-10).
:::

:::{rule} CMP-15
:kind: required
:tests: expressions/comparison/pointer-equality.cpp expressions/comparison/pointer-equality-constant.cpp

Two pointers are equal if they point to the same object or function, or
if both are null. Pointers to two distinct objects are not equal.
:::

:::{rule} CMP-17
:kind: required
:tests: expressions/comparison/three-way-integers.cpp expressions/comparison/three-way-floating.cpp

`a <=> b` gives `less`, `equal`, or `greater` as `a` is less than, equal
to, or greater than `b`. If a floating-point operand is a NaN, the result
is `std::partial_ordering::unordered`.
:::

## Implementation Requirements

:::{rule} CMP-8
:kind: unspecified
:tests: expressions/comparison/pointer-unrelated-run.cpp expressions/comparison/pointer-unrelated-constant.cpp expressions/comparison/function-pointer-order.cpp

The result of a relational operator on pointers to unrelated objects or
functions is unspecified. One of `a < b` and `a > b` is true. During
{term}`constant evaluation`, this comparison is not a constant expression.
:::

:::{rule} CMP-20
:kind: unspecified
:tests: expressions/comparison/one-past-end-adjacent-constant.cpp expressions/comparison/one-past-end-adjacent.cpp

A pointer one past the end of one object can compare equal or not equal
to a pointer to another object that follows it in memory. The result is
unspecified. During constant evaluation, the comparison is rejected
({question}`Q-0051`).
:::

## Recommended Practice

:::{rule} CMP-4
:kind: recommended
:tests: expressions/comparison/sign-compare-warns.cpp expressions/comparison/sign-compare-cast.cpp

An implementation should warn when a comparison converts a signed
operand to an unsigned type. An explicit cast should silence the warning.
:::

:::{rule} CMP-14
:kind: recommended
:tests: expressions/comparison/chained-comparison-warns.cpp

An implementation should warn about a chained comparison such as
`a < b < c`.
:::

:::{rule} CMP-16
:kind: recommended
:tests: expressions/comparison/array-comparison-warns.cpp

An implementation should warn when a comparison has two array operands.
:::

:::{rule} CMP-21
:kind: recommended
:tests: expressions/comparison/different-enums-warns.cpp

An implementation should warn about a comparison between two different
enumeration types.
:::

## Examples

```cpp
1 < 2 == 3 > 4         // (1 < 2) == (3 > 4), false
3 > 2 > 1              // (3 > 2) > 1, so 1 > 1, false
-1 < 0u                // false: -1 becomes the largest unsigned value

int arr[4];
&arr[0] < &arr[3]      // true

int a, b;
&a < &b                // unspecified

(1 <=> 2) == std::strong_ordering::less     // true
(1.0 <=> nan) == std::partial_ordering::unordered
```

## Evidence

The results are in `tests/RESULTS.md`. The `<=>` tests declare a minimal
`std::strong_ordering`, `std::weak_ordering`, and `std::partial_ordering`,
because tests include no headers. EDG needs `std::weak_ordering` declared
even for integer operands.

- CMP-1: Clang 22 rejects a chained comparison by default as a
  `-Wparentheses` error. It accepts the comparison when that warning is
  off. Clang 18, GCC, and EDG accept it ({question}`Q-0029`).
- CMP-8: all compilers reject the comparison of unrelated pointers during
  constant evaluation. A function name followed by `<` starts a template
  argument list, so the function-pointer test puts each operand in
  parentheses.
- CMP-9 and CMP-13: EDG accepts an ordered comparison or `<=>` between a
  pointer and the literal `0`. Clang and GCC reject both
  ({question}`Q-0030`).
- CMP-11 and CMP-16: Clang and GCC warn that array comparison is
  deprecated. In strict C++23 mode they accept it.
- CMP-4, CMP-14, CMP-16: Clang and GCC give the warnings under
  `-Wsign-compare`, `-Wparentheses`, and `-Wall -Wextra`. Clang 18 gives no
  warning for a chained comparison of variables ({question}`Q-0029`).
- CMP-9: EDG accepts a relational comparison of a pointer with `nullptr`,
  as with `0` ({question}`Q-0030`).
- CMP-18: EDG accepts `<=>` between an unscoped enumeration and a
  floating-point value. Clang and GCC reject it ({question}`Q-0050`).
- CMP-20: Clang and GCC reject the comparison during constant evaluation.
  EDG accepts it ({question}`Q-0051`).

Inputs: clang-sema-expr, clang-diag-sema.

# Additive operators

The operators `+` and `-` add and subtract arithmetic values. They also
move a pointer within an array, and give the distance between two pointers
into the same array.

## Syntax

```{eval-rst}
.. productionlist:: expr_additive
   add_expr: `mul_expr` { ("+" | "-") `mul_expr` }
```

## Static Semantics

:::{rule} ADD-1
:kind: required
:tests: expressions/additive/grouping.cpp

`+` and `-` have the same precedence. They group from left to right.
They bind more tightly than shifts and less tightly than `*`, `/`, and
`%`.
:::

:::{rule} ADD-2
:kind: required
:tests: expressions/additive/operands-arithmetic.cpp

Both operators accept two operands of arithmetic type or unscoped
enumeration type, in any combination.
:::

:::{rule} ADD-3
:kind: reject
:tests: expressions/additive/operands-scoped-enum.cpp expressions/additive/operands-class.cpp expressions/additive/pointer-plus-pointer.cpp expressions/additive/integer-minus-pointer.cpp expressions/additive/void-pointer-arithmetic.cpp expressions/additive/function-pointer-arithmetic.cpp expressions/additive/pointer-to-incomplete.cpp expressions/additive/pointer-minus-different-types.cpp expressions/additive/void-pointer-difference.cpp

The operands must have one of these forms:

- two arithmetic or unscoped enumeration operands,
- for `+`, a pointer and an integer, in either order,
- for `-`, a pointer and then an integer,
- for `-`, two pointers to the same type, ignoring cv-qualifiers.

A pointer operand must point to a complete object type. A pointer to
`void` or to a function is not valid.
:::

:::{rule} ADD-4
:kind: required
:tests: expressions/additive/result-types.cpp expressions/additive/result-prvalue.cpp

Arithmetic operands go through the {term}`usual arithmetic conversions`.
The result has the common type. A pointer and an integer give the type of
the pointer. The result is a prvalue.
:::

## Dynamic Semantics

:::{rule} ADD-5
:kind: required
:tests: expressions/additive/unsigned-wraps.cpp

If the common type is an unsigned integer type, the result wraps around.
It is the mathematical result reduced modulo one more than the largest
value of the type.
:::

:::{rule} ADD-6
:kind: required
:tests: expressions/additive/pointer-within-array.cpp expressions/additive/pointer-one-past-end.cpp expressions/additive/pointer-object-as-array.cpp expressions/additive/pointer-arith-runtime.cpp

Adding `n` to a pointer to an array element moves it `n` elements
forward. Subtracting `n` moves it back. The position after the last
element is valid. A single object counts as an array with one element.
:::

:::{rule} ADD-7
:kind: required
:tests: expressions/additive/pointer-difference.cpp

The difference of two pointers into one array is the number of elements
from the second pointer to the first. It is negative if the first pointer
comes earlier.
:::

:::{rule} ADD-8
:kind: required
:tests: expressions/additive/null-plus-zero.cpp

Adding or subtracting zero to a null pointer gives a null pointer. The
difference of two null pointers is zero.
:::

## Undefined Behavior

:::{rule} ADD-9
:kind: undefined
:tests: expressions/additive/signed-overflow-add.cpp expressions/additive/signed-overflow-subtract.cpp

If the common type is a signed integer type and the mathematical result
does not fit in it, the behavior is undefined.
:::

:::{rule} ADD-10
:kind: undefined
:tests: expressions/additive/pointer-beyond-array.cpp expressions/additive/pointer-before-array.cpp expressions/additive/null-plus-nonzero.cpp expressions/additive/deref-one-past-end.cpp expressions/additive/past-end-of-row.cpp

The behavior is undefined if pointer arithmetic gives a position that is:

- before the first element of the array,
- after the position one past the last element,
- not zero distance from a null pointer.

Each row of a multidimensional array is a separate array for this rule.
Reading through a pointer one past the last element is also undefined.
:::

:::{rule} ADD-11
:kind: undefined
:tests: expressions/additive/pointer-difference-different-arrays.cpp

If two pointers do not point into the same array, `p - q` is undefined.
:::

:::{rule} ADD-14
:kind: undefined
:tests: expressions/additive/base-pointer-derived-array.cpp

Pointer arithmetic is undefined if the pointer's element type differs
from the type of the array elements, apart from cv-qualifiers. An example
is a base class pointer into an array of a derived class (question
pending).
:::

## Implementation Requirements

:::{rule} ADD-12
:kind: impl-defined
:tests: expressions/additive/pointer-difference-type-lp64.cpp expressions/additive/pointer-difference-type-ilp32.cpp

The type of `p - q` is a signed integer type that the implementation
chooses. The library names it `std::ptrdiff_t` *(referenced)*.
:::

:::{rule} ADD-13
:kind: unspecified
:tests: expressions/additive/operand-order.cpp

The order in which the two operands are evaluated is unspecified.
:::

## Recommended Practice

:::{rule} ADD-16
:kind: recommended
:tests: expressions/additive/mixed-enum-warns.cpp

An implementation should warn about `+` or `-` between two different
enumeration types, or between an enumeration and a floating-point value.
:::

## Examples

```cpp
10 - 3 - 2;               // (10 - 3) - 2 == 5
0u - 1u;                  // the largest unsigned value

int a[4] = {10, 20, 30, 40};
int* p = a + 3;           // points to a[3]
int* end = a + 4;         // fine: one past the last element
int* bad = a + 5;         // undefined
end - a;                  // 4

int b[2];
&b[0] - &a[0];            // undefined: different arrays
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 28 of
30 tests.

- ADD-3: GCC rejects arithmetic on `void*` and on function pointers only
  as a pedantic diagnostic (`-Wpointer-arith`). Clang and EDG reject it in
  all modes.
- ADD-9, ADD-11: all compilers reject these operations during
  {term}`constant evaluation`.
- ADD-10: Clang and EDG reject a pointer before or beyond the array during
  constant evaluation. GCC accepts the pointer and rejects only a read
  through it ({question}`Q-0031`). All compilers reject a nonzero offset from
  a null pointer.
- ADD-12: `p - q` has type `long` on x86-64 (LP64) and `int` with
  `-m32` (ILP32).
- ADD-13: Clang and GCC evaluate the left operand first. No variation is
  observed (compare {question}`Q-0005`).
- ADD-14: Clang and EDG reject arithmetic on a base class pointer into an
  array of a derived class during constant evaluation. GCC accepts it
  ({question}`Q-0049`).
- ADD-16, MUL-16: Clang and GCC warn about these operations as deprecated.

Inputs: clang-sema-expr, clang-diag-sema.

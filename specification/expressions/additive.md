# Additive operators

The operators `+` and `-` add and subtract arithmetic values. The
operators also move a pointer within an array, and give the distance
between two pointers into the same array.

## Principles

:::{principle} ADD-P1
:rules: ADD-1 ADD-2 ADD-3 ADD-4

The operand forms select what `+` and `-` do:

| Operands | Job | Result |
|---|---|---|
| Two numbers | Add or subtract | A value of the common type of the {term}`usual arithmetic conversions` |
| A pointer and an integer | Move the pointer | A pointer of the pointer operand's type |
| Two pointers, with `-` | Measure the distance | The distance |

Any other combination of operands is rejected. The result is a prvalue
in every case ({principle}`VC-P1`).
:::

:::{principle} ADD-P2
:rules: ADD-5 ADD-9 ADD-17

For numbers the result is the mathematical one, fitted to the type.
The types differ when the result is outside their range:

| Type | Modular | Result outside the range |
|---|---|---|
| Unsigned integer | Yes | Wraps |
| Signed integer | No | No value. The behavior is undefined. |
| Floating-point | No | No value. The behavior is undefined. |

Constant evaluation rejects the undefined cases ({principle}`TR-P4`).
:::

:::{principle} ADD-P3
:rules: ADD-6 ADD-7 ADD-8 ADD-12 ADD-18

A pointer into an array is a position in that array, counted in
elements of the array's own type. Adding moves the position, and
subtracting two positions in one array gives the count between them, in
a signed type that the implementation chooses. A count that does not
fit that type has no value. A null pointer is no position at all, so
only moving it by zero means anything.
:::

:::{principle} ADD-P6
:rules: ADD-10 ADD-11 ADD-14

A single object is an array of one, and every array has one extra
valid position just past its last element, so a loop can run to the
end:

```text
   +------+------+------+------+
   | a[0] | a[1] | a[2] | a[3] |
   +------+------+------+------+
   ^                           ^
   a                           a + 4: one past the last element

   valid positions: a + 0 to a + 4
```

Outside those positions there is nothing to point at, and the
behavior is undefined. Two pointers into different arrays have no
distance. The arithmetic counts in the size of the pointer's type. A
pointer of another type, such as a base class pointer into an array of
a derived class, sees only its own object, as an array of one. A byte
pointer sees the bytes of the object, as an array of `sizeof` bytes
({rule}`STG-13`).
:::

:::{principle} ADD-P4
:rules: ADD-13

The two operands are not ordered with respect to each other
({rule}`SEQ-8`).
:::

:::{principle} ADD-P5
:rules: ADD-16

Mixing an enumeration with a floating-point value, or two different
enumerations, is deprecated, and an implementation should say so.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   add_expr: `mul_expr` {("+" | "-") `mul_expr`}
```

## Constraints

:::{rule} ADD-1
:kind: required
:tests: expressions/additive/grouping.cpp expressions/additive/grouping-shift.cpp

`+` and `-` have the same precedence and group from left to right.
Both operators bind more tightly than shifts and less tightly than `*`,
`/`, and `%`.
:::

:::{rule} ADD-2
:kind: required
:tests: expressions/additive/operands-arithmetic.cpp expressions/additive/operands-mixed.cpp

Both operators accept two operands of arithmetic type or unscoped
enumeration type, in any combination.
:::

:::{rule} ADD-3
:kind: reject
:tests: expressions/additive/operands-scoped-enum.cpp expressions/additive/operands-class.cpp expressions/additive/pointer-plus-pointer.cpp expressions/additive/integer-minus-pointer.cpp expressions/additive/void-pointer-arithmetic.cpp expressions/additive/function-pointer-arithmetic.cpp expressions/additive/pointer-to-incomplete.cpp expressions/additive/pointer-minus-different-types.cpp expressions/additive/void-pointer-difference.cpp expressions/additive/pointer-plus-unscoped-enum.cpp expressions/additive/pointer-plus-scoped-enum.cpp expressions/additive/nullptr-plus-int.cpp

The operands must have one of these forms:

- two arithmetic or unscoped enumeration operands,
- for `+`, a pointer and an integer, in either order,
- for `-`, a pointer and then an integer,
- for `-`, two pointers to the same type, ignoring cv-qualifiers.

In this rule, an integer includes:

- `bool`,
- the character types,
- an unscoped enumeration.

A scoped enumeration and the type of `nullptr` are not valid. A pointer
operand must point to a complete object type. A pointer to `void` or to
a function is not valid.
:::

:::{rule} ADD-4
:kind: required
:tests: expressions/additive/result-types.cpp expressions/additive/result-prvalue.cpp expressions/additive/operands-mixed.cpp

Arithmetic operands go through the {term}`usual arithmetic conversions`.
The result has the common type. A pointer and an integer give the type
of the pointer. The result is a prvalue.
:::

## Runtime Behavior

:::{rule} ADD-5
:kind: required
:tests: expressions/additive/unsigned-wraps.cpp expressions/additive/unsigned-wraps-subtract.cpp

If the common type is an unsigned integer type, the result wraps around.
The wrapped result is the mathematical result reduced modulo one more
than the largest value of the type.
:::

:::{rule} ADD-6
:kind: required
:tests: expressions/additive/pointer-within-array.cpp expressions/additive/pointer-one-past-end.cpp expressions/additive/pointer-object-as-array.cpp expressions/additive/pointer-arith-runtime.cpp expressions/additive/pointer-difference-run.cpp expressions/additive/base-pointer-one-past-subobject.cpp foundations/storage/byte-pointer-walks-object.cpp

Adding `n` to a pointer to an array element moves the pointer `n`
elements forward. Subtracting `n` moves the pointer back. The position
after the last element is valid. A single object counts as an array with
one element. A pointer to `std::byte`, `unsigned char`, or `char` can
step through the bytes of any object as an array of `sizeof` elements
({rule}`STG-13`, {question}`Q-0304`).
:::

:::{rule} ADD-7
:kind: required
:tests: expressions/additive/pointer-difference.cpp expressions/additive/pointer-difference-run.cpp expressions/additive/pointer-difference-largest-ilp32.cpp

The difference of two pointers into one array is the number of elements
from the second pointer to the first. The difference is negative if the
first pointer comes earlier.
:::

:::{rule} ADD-8
:kind: required
:tests: expressions/additive/null-plus-zero.cpp expressions/additive/null-minus-null-run.cpp

Adding or subtracting zero to a null pointer gives a null pointer. The
difference of two null pointers is zero.
:::

## Undefined Behavior

:::{rule} ADD-9
:kind: undefined
:tests: expressions/additive/signed-overflow-add.cpp expressions/additive/signed-overflow-subtract.cpp expressions/additive/signed-overflow-add-sanitizer.cpp foundations/types/int-max-overflow.cpp

If the common type is a signed integer type and the mathematical result
does not fit in it, the behavior is undefined.
:::

:::{rule} ADD-10
:kind: undefined
:tests: expressions/additive/pointer-beyond-array.cpp expressions/additive/pointer-before-array.cpp expressions/additive/null-plus-nonzero.cpp expressions/additive/deref-one-past-end.cpp expressions/additive/past-end-of-row.cpp expressions/additive/null-plus-nonzero-sanitizer.cpp expressions/additive/base-pointer-beyond-subobject.cpp

The behavior is undefined if pointer arithmetic gives a position that
is:

- before the first element of the array,
- after the position one past the last element,
- not zero distance from a null pointer.

Each row of a multidimensional array is a separate array for this rule.
For a byte pointer that steps through an object, the array is the bytes
of the object ({rule}`STG-13`, {question}`Q-0304`). Reading through a
pointer one past the last element is also undefined.
:::

:::{rule} ADD-11
:kind: undefined
:tests: expressions/additive/pointer-difference-different-arrays.cpp expressions/additive/pointer-difference-separate-objects.cpp

If two pointers do not point into the same array, `p - q` is undefined.
:::

:::{rule} ADD-14
:kind: undefined
:tests: expressions/additive/base-pointer-derived-array.cpp expressions/additive/subtract-base-pointers-derived-array.cpp expressions/additive/base-pointer-one-past-subobject.cpp expressions/additive/base-pointer-beyond-subobject.cpp

Arithmetic on a pointer whose type differs from the array's element
type counts the object as an array of one. Cv-qualifiers do not count
as a difference. A pointer to `std::byte`, `unsigned char`, or `char`
counts the bytes instead (ADD-6). These operations on any other such
pointer are undefined ({question}`Q-0049`):

- a move to a position other than the object or one past it,
- a subtraction of two pointers into different array elements.

The rule covers a pointer to the signed or unsigned integer type that
matches the element type. A read through such a pointer is valid
({rule}`STG-14`). A read of another element through it is undefined.
:::

:::{rule} ADD-17
:kind: undefined
:tests: expressions/additive/float-overflow-constant.cpp expressions/additive/float-overflow-subtract-constant.cpp

If the exact result of a floating-point `+` or `-` is finite but outside
the range of its type, the behavior is undefined, as for MUL-12
({question}`Q-0001`).
:::

:::{rule} ADD-18
:kind: undefined
:tests: expressions/additive/pointer-difference-overflow-ilp32.cpp expressions/additive/pointer-difference-largest-ilp32.cpp

If the difference of two pointers does not fit in the type of `p - q`
({rule}`ADD-12`), the behavior is undefined ({question}`Q-0298`).
:::

## Implementation Requirements

:::{rule} ADD-12
:kind: impl-defined
:tests: expressions/additive/pointer-difference-type-lp64.cpp expressions/additive/pointer-difference-type-ilp32.cpp

The type of `p - q` is a signed integer type that the implementation
chooses. The library names it `std::ptrdiff_t` ({rule}`LSUP-4`).
:::

:::{rule} ADD-13
:kind: unspecified
:tests: expressions/additive/operand-order.cpp expressions/additive/operand-order-overloaded.cpp

The order in which the two operands are evaluated is unspecified.
:::

## Recommended Practice

:::{rule} ADD-16
:kind: recommended
:tests: expressions/additive/mixed-enum-warns.cpp expressions/additive/enum-float-warns.cpp

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

struct Base { int id; };
struct Item : Base { int extra; };
Item items[3];
Base* first = items;
first + 1;                // fine: one past the Base subobject
first + 2;                // undefined
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 47
of 55 tests.

- ADD-3: GCC rejects arithmetic on `void*` and on function pointers only
  as a pedantic diagnostic (`-Wpointer-arith`). Clang and EDG reject
  such arithmetic in all modes.
- ADD-9, ADD-11: all compilers reject these operations during
  {term}`constant evaluation`. For ADD-11 this includes two local
  variables that are not arrays.
- ADD-10: Clang and EDG reject a pointer before or beyond the array
  during constant evaluation. GCC 14 and 16 accept the pointer, even
  far outside the array, and reject only a read through it
  ({question}`Q-0031`). All compilers reject a nonzero offset from a
  null pointer.
- ADD-12: `p - q` has type `long` on x86-64 (LP64) and `int` with
  `-m32` (ILP32).
- ADD-13: Clang and GCC evaluate the left operand of the built-in `+`
  first. For an overloaded `+`, GCC evaluates the right operand first.
  The tests accept either order ({question}`Q-0005`).
- ADD-14: every compiler accepts a base class pointer moved one past
  its subobject during constant evaluation, with a difference of `1`.
  Clang and EDG reject a move of two, a read through the pointer one
  past, and a difference of base pointers into two elements. GCC
  accepts all three, and computes the difference from the addresses
  ({question}`Q-0049`).
- ADD-6, ADD-10, ADD-14: the address and undefined sanitizers of Clang
  and GCC report no step through the bytes of an object. They also
  report no read of the next element through the unsigned type that
  matches `int`. Constant evaluation
  cannot form either pointer ({rule}`CAST-8`). The byte rule is
  provisional ({question}`Q-0304`).
- ADD-16, MUL-16: Clang and GCC warn about these operations as
  deprecated.
- ADD-17: Clang gives an infinity during constant evaluation. GCC and
  EDG reject the expression ({question}`Q-0001`).
- ADD-18: with `-m32`, Clang rejects a difference of `2147483648`
  during constant evaluation and accepts `INT_MIN`. EDG rejects every
  position past `INT_MAX`. GCC rejects an array of more than `INT_MAX`
  bytes, so no difference into one array overflows there
  ({question}`Q-0298`).

Inputs: clang-sema-expr, clang-diag-sema.

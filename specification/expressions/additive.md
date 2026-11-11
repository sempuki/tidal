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

Both operators accept two operands of {term}`arithmetic type` or unscoped
enumeration type, in any combination.
:::

:::{rule} ADD-3
:kind: reject
:tests: expressions/additive/operands-scoped-enum.cpp expressions/additive/operands-class.cpp expressions/additive/pointer-plus-pointer.cpp expressions/additive/integer-minus-pointer.cpp expressions/additive/void-pointer-arithmetic.cpp expressions/additive/function-pointer-arithmetic.cpp expressions/additive/pointer-to-incomplete.cpp expressions/additive/pointer-minus-different-types.cpp expressions/additive/void-pointer-difference.cpp expressions/additive/pointer-plus-unscoped-enum.cpp expressions/additive/pointer-plus-scoped-enum.cpp expressions/additive/nullptr-plus-int.cpp

The operands must have one of these forms:

- two arithmetic or unscoped enumeration operands,
- for `+`, a pointer and an integer, in either order,
- for `-`, a pointer and then an integer,
- for `-`, two pointers to the same type, ignoring {term}`cv-qualifiers <cv-qualifier>`.

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
The result has the common type. If the common type is unsigned, a
negative signed operand converts first ({rule}`CONV-2`). A pointer and an integer give the type
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
:tests: expressions/additive/pointer-within-array.cpp expressions/additive/pointer-one-past-end.cpp expressions/additive/pointer-object-as-array.cpp expressions/additive/pointer-arith-runtime.cpp expressions/additive/pointer-difference-run.cpp expressions/additive/base-pointer-one-past-subobject.cpp foundations/storage/byte-pointer-walks-object.cpp expressions/additive/pointer-loop-inside-string-literal.cpp statements/range-for/view-over-string-literal.cpp

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
:tests: expressions/additive/signed-overflow-add.cpp expressions/additive/signed-overflow-subtract.cpp expressions/additive/signed-overflow-add-sanitizer.cpp foundations/types/int-max-overflow.cpp foundations/translation/undefined-on-taken-path-sanitizer.cpp

If the common type is a signed integer type and the mathematical result
does not fit in it, the behavior is undefined.
:::

:::{rule} ADD-10
:kind: undefined
:tests: expressions/additive/pointer-beyond-array.cpp expressions/additive/intermediate-step-beyond-array.cpp expressions/additive/pointer-before-array.cpp expressions/additive/null-plus-nonzero.cpp expressions/additive/deref-one-past-end.cpp expressions/additive/past-end-of-row.cpp expressions/additive/null-plus-nonzero-sanitizer.cpp expressions/additive/base-pointer-beyond-subobject.cpp expressions/additive/base-pointer-beyond-subobject-run.cpp expressions/additive/pointer-loop-inside-string-literal.cpp foundations/storage/byte-rows-pointer-does-not-reach-next-row.cpp

The behavior is undefined if pointer arithmetic gives a position that
is:

- before the first element of the array,
- after the position one past the last element,
- not zero distance from a null pointer.

Each row of a multidimensional array is a separate array for this rule.
For a byte pointer that steps through an object, the array is the bytes
of the object ({rule}`STG-13`, {question}`Q-0304`). Reading through a
pointer one past the last element is also undefined. {rule}`ADD-14` is
the specific case of ADD-10 for a pointer whose type differs from the
element type of the array (decision 0052).
:::

:::{rule} ADD-11
:kind: undefined
:tests: expressions/additive/pointer-difference-different-arrays.cpp expressions/additive/pointer-difference-separate-objects.cpp expressions/additive/pointer-difference-different-arrays-run.cpp

If two pointers do not point into the same array, `p - q` is undefined.
:::

:::{rule} ADD-14
:kind: undefined
:tests: expressions/additive/base-pointer-derived-array.cpp expressions/additive/subtract-base-pointers-derived-array.cpp expressions/additive/base-pointer-one-past-subobject.cpp expressions/additive/base-pointer-beyond-subobject.cpp expressions/additive/base-pointer-beyond-subobject-run.cpp expressions/additive/typed-pointer-to-byte-two-past.cpp foundations/storage/allocated-storage-typed-step-after-byte-offset.cpp foundations/storage/byte-array-typed-step-distance.cpp foundations/storage/byte-array-typed-step-placement.cpp

Arithmetic on a pointer whose type differs from the array's element
type counts the object as an array of one. Cv-qualifiers do not count
as a difference. A pointer to `std::byte`, `unsigned char`, or `char`
counts the bytes instead (ADD-6). A `T*` into an array of `T` that an
allocation function created points to an element of that array
({rule}`STG-28`), so this rule does not apply to it. These operations on any other such
pointer are undefined ({question}`Q-0049`):

- a move to a position other than the object or one past it,
- a subtraction of two pointers into different array elements.

The rule covers a pointer to the signed or unsigned integer type that
matches the element type. A read through such a pointer is valid
({rule}`STG-14`). A read of another element through it is undefined.

A move of such a pointer counts in the size of the pointer's type. So
if a `T*` named `p` points to a byte, `p + 1` holds the address
`sizeof(T)` bytes after `p` ({question}`Q-0430`).
:::

:::{rule} ADD-17
:kind: undefined
:tests: expressions/additive/float-overflow-constant.cpp expressions/additive/float-overflow-subtract-constant.cpp expressions/additive/float-overflow-run.cpp

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
chooses. The library names it `std::ptrdiff_t` ({rule}`LSUP-4`). Every
compiler in the matrix chooses `long` on x86-64 and `int` with `-m32`.
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

% tidal-example: fragment
```cpp
10 - 3 - 2;               // (10 - 3) - 2 == 5
0u - 1u;                  // the largest unsigned value
-1 + 0u;                  // ~0u: -1 converts to unsigned first

int a[4] = {10, 20, 30, 40};
int* p = a + 3;           // points to a[3]
int* end = a + 4;         // fine: one past the last element
int* bad = a + 5;         // undefined
end - a;                  // 4
a + 'a' - 'a';            // undefined: the first step, a + 97, leaves the array
a + ('a' - 'a');          // fine: a + 0

int b[2];
&b[0] - &a[0];            // undefined: different arrays

struct Base { int id; };
struct Item : Base { int extra; };
Item items[3];
Base* first = items;
first + 1;                // fine: one past the Base subobject
first + 2;                // undefined

// Test before adding, so that no step overflows (ADD-9).
constexpr int imax = static_cast<int>(~0u >> 1);
constexpr int imin = -imax - 1;
bool add_fits(int a, int b) {
  return b > 0 ? a <= imax - b : a >= imin - b;
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 58
of 67 tests.

### Divergences

- {question}`Q-0031` (ADD-10; settled on ADD-P6): whether forming a
  pointer before or beyond an array is undefined without a read through
  it. It is, and constant evaluation rejects the pointer, as Clang and
  EDG do. GCC accepts the pointer, also after an intermediate step that
  leaves the array, and rejects only a read through it.
- {question}`Q-0049` (ADD-14; settled on ADD-P6): whether arithmetic on
  a base class pointer into an array of a derived class is undefined
  past the subobject. The subobject acts as an array of one element, so
  a move of two, a read through the pointer one past, and a difference
  of pointers into two elements are undefined, as Clang and EDG say.
  GCC accepts all three.
- {question}`Q-0430` (ADD-14, STG-29; open): where `p + 1` is when
  `p` is a `T*` that points to a byte, and whether placement `new` can
  use that position. The position is `sizeof(T)` bytes after `p`, and
  placement `new` can use it. No family differs.
- {question}`Q-0304` (ADD-6, ADD-10, ADD-14; open): whether a pointer
  to a character type can step through the bytes of an object that is
  not an array of bytes. It can. No family differs, and constant
  evaluation cannot form such a pointer ({rule}`CAST-8`).
- {question}`Q-0005` (ADD-13; settled on ADD-P4): whether the order of
  the operands can be unspecified when no compiler varies it for the
  built-in `+`. It can. No family differs, and the tests accept either
  order.
- {question}`Q-0001` (ADD-17; open): whether a floating overflow during
  constant evaluation is undefined or implementation-defined. It is
  undefined, as GCC and EDG say. Clang gives an infinity.
- {question}`Q-0298` (ADD-18; settled on ADD-P3): how implementations
  treat a pointer difference that does not fit its type. The
  difference is undefined. With `-m32`, Clang rejects only a difference
  that does not fit, and GCC rejects an array of more than `INT_MAX`
  bytes. EDG 7.0 rejects every position past `INT_MAX`.

### Split tests

EDG skips the tests that use `-fsanitize`.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `base-pointer-beyond-subobject.cpp` | ADD-14, ADD-10 | reject | reject | accept | reject | {question}`Q-0049` |
| `base-pointer-derived-array.cpp` | ADD-14 | reject | reject | accept | reject | {question}`Q-0049` |
| `float-overflow-constant.cpp` | ADD-17 | reject | accept | reject | reject | {question}`Q-0001` |
| `float-overflow-subtract-constant.cpp` | ADD-17 | reject | accept | reject | reject | {question}`Q-0001` |
| `intermediate-step-beyond-array.cpp` | ADD-10 | reject | reject | accept | reject | {question}`Q-0031` |
| `null-plus-nonzero-sanitizer.cpp` | ADD-10 | run-fail | run-fail | run | skips | |
| `pointer-before-array.cpp` | ADD-10 | reject | reject | accept | reject | {question}`Q-0031` |
| `pointer-beyond-array.cpp` | ADD-10 | reject | reject | accept | reject | {question}`Q-0031` |
| `subtract-base-pointers-derived-array.cpp` | ADD-14 | reject | reject | accept | reject | {question}`Q-0049` |

### Notes

- ADD-3: GCC rejects arithmetic on `void*` and on function pointers only
  as a pedantic diagnostic (`-Wpointer-arith`). Clang and EDG reject
  such arithmetic in all modes.
- ADD-9, ADD-11: all compilers reject these operations during
  {term}`constant evaluation`. For ADD-11 this includes two local
  variables that are not arrays.
- ADD-10: all compilers reject a nonzero offset from a null pointer
  during constant evaluation.
- ADD-10: during constant evaluation, every compiler rejects a read of
  the next row through a pointer into the current row of a
  two-dimensional array (`past-end-of-row.cpp`). GCC names the bound of
  the row in its message.
- ADD-10: at run time, `-fsanitize=undefined` in Clang stops a nonzero
  offset from a null pointer. The GCC sanitizer does not check the
  offset and lets the program run (`null-plus-nonzero-sanitizer.cpp`).
- ADD-10: a namespace-scope variable whose initializer is not a
  constant expression is {term}`dynamically initialized <dynamic initialization>` ({rule}`SINIT-2`). If
  that initializer forms a pointer outside the array, the behavior at
  run time is undefined.
- ADD-12: `p - q` has type `long` on x86-64 (LP64) and `int` with
  `-m32` (ILP32).
- ADD-14: every compiler accepts a base class pointer moved one past
  its subobject during constant evaluation, with a difference of `1`.
- ADD-6, ADD-10, ADD-14: the address and undefined sanitizers of Clang
  and GCC report no step through the bytes of an object. They also
  report no read of the next element through the unsigned type that
  matches `int`.
- ADD-16, MUL-16: Clang and GCC warn about these operations as
  deprecated. The flag is `-Wdeprecated-enum-enum-conversion` for two
  different enumerations and `-Wdeprecated-enum-float-conversion` for an
  enumeration and a floating-point value. Each warning is on by
  default, and `-pedantic-errors` does not make it an error. So both
  compilers accept the operands in strict mode. The ADD-16 tests add
  `-Werror` and the named flag. EDG does not accept `-Werror` and skips
  the tests.
- ADD-18: the boundary is the same on every compiler. An array of
  `0x7fffffff` bytes, with the differences `0x7fffffff` and
  `-0x7fffffff` inside it, is accepted
  (`pointer-difference-largest-ilp32.cpp`). An array of `0x80000000`
  bytes with a difference across the whole array is rejected
  (`pointer-difference-overflow-ilp32.cpp`).
- ADD-9: `-fsanitize=undefined` in Clang and GCC stops a signed `+`
  whose result does not fit, at run time
  (`signed-overflow-add-sanitizer.cpp`).
- ADD-11: the address sanitizer of GCC and Clang stops a difference of
  pointers into two arrays only under `-fsanitize=pointer-subtract`
  and a run-time option that a test cannot set. No other configuration
  stops it. The front end's interpreter stops the difference
  (`pointer-difference-different-arrays-run.cpp`, decision 0053).
- ADD-14: no sanitizer of GCC or Clang stops a step of a base pointer
  through an array of derived objects. The front end's interpreter stops
  the step and names ADD-10 (`base-pointer-beyond-subobject-run.cpp`).
- ADD-17: no sanitizer of GCC or Clang stops a floating-point `+` whose
  result is outside the range of its type. The front end's interpreter
  stops the sum (`float-overflow-run.cpp`).
- ADD-18: a 64-bit object is too small for the difference to overflow.
  With `-m32`, GCC rejects an array of more than `INT_MAX` bytes. In the
  writer's probes in the Clang 18 and Clang 22 containers with `-m32`,
  `new` throws `std::bad_alloc` for such an array, so the program never
  reaches the difference. The front end's interpreter cannot hold an
  array of that size. No tool stops ADD-18 at run time, and no run test
  covers the rule.

### Sources

Inputs: clang-sema-expr, clang-diag-sema.

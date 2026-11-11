# Comparison operators

The relational operators `<`, `>`, `<=`, and `>=` and the equality
operators `==` and `!=` compare two values and give a `bool`. The
three-way comparison operator `<=>` gives an ordering value.

## Principles

:::{principle} CMP-P1
:rules: CMP-1 CMP-2 CMP-3

A comparison compares two values and gives a `bool` prvalue
({principle}`VC-P1`). Arithmetic operands are first brought to one
common type by the {term}`usual arithmetic conversions`, and the
comparison sees the converted values, so a negative `int` compared with
an `unsigned` by `<` has become a large unsigned value. The operator
`<=>` rejects that conversion, because the conversion narrows
({rule}`CMP-13`). The operators bind in
three groups, and each group associates to the left:

```text
   binds more tightly
         <=>
         <   >   <=   >=
         ==  !=
   binds less tightly
```

So `a < b < c` compares a `bool` with `c`. The two operands are not
ordered with respect to each other ({rule}`SEQ-8`).
:::

:::{principle} CMP-P2
:rules: CMP-6 CMP-9 CMP-10 CMP-11 CMP-19 CMP-23 CMP-24

Two operands can be compared when they are the same kind of thing, or
one converts to the other's kind. Two pointers compare when one converts
to the other's type. Such a conversion includes:

- adding cv-qualifiers,
- converting to `void*`,
- converting from a derived class to its base.

A pointer compares with `nullptr` and with a literal `0` for equality
only, because a null pointer has no order. Two values of one scoped
enumeration compare, and values of two different scoped enumerations do
not, because a scoped enumeration is its own kind. An array operand
becomes a pointer to its first element, so comparing two arrays
compares their addresses.
:::

:::{principle} CMP-P3
:rules: CMP-7 CMP-15 CMP-22 CMP-26 CMP-8 CMP-20

Pointers have an order only inside one thing:

| Pointers to | Order |
|---|---|
| Elements of one array | Index order, with the position past the end last |
| Members of one object | Declaration order |

Outside one thing there is no order, so a relational comparison of
pointers to unrelated objects gives an unspecified result, and the two
directions need not even agree. Two pointers are equal when they hold
one address. Pointers to different objects can hold one address too, for
example an object and its first member. A pointer past the end of one
object can hold the address of the next object in memory, and whether
the two pointers compare equal is unspecified. The rows of an array of
arrays have no gap between them, so the position past the end of one
row equals the first element of the next row. Two pointers to members
are equal when they select the same member of the same {term}`subobject`.

A constant evaluation must not depend on where the implementation
places objects ({principle}`CE-P3`). So during constant evaluation
these comparisons are rejected:

- a relational comparison of pointers to unrelated objects,
- a relational comparison of pointers to two base class subobjects of
  one object, or to a base class subobject and a member
  ({question}`Q-0387`),
- an equality comparison of a pointer past the end of one complete
  object with a pointer to another complete object.

Inside one complete object the layout is fixed, so the other
comparisons are accepted there. A row of an array of arrays is not a
complete object. So for `int grid[2][3]`, `&grid[0][3] == &grid[1][0]`
is accepted during constant evaluation and gives `true`, while
`a + 1 == b` for two arrays `a` and `b` is rejected. The rows of an
array of arrays, the members of one class, the members of a union, and a member that
occupies no storage are examples. For the last two, the order is
unspecified, and constant evaluation still gives a result.
:::

:::{principle} CMP-P4
:rules: CMP-5

A NaN is not equal to anything, not even itself, and is neither less nor
greater than anything. So every comparison with a NaN is `false`, except
`!=`, which is `true`. A program gets a NaN from `0.0 / 0.0` at run time
in a type that follows IEEE 754 ({rule}`MUL-11`). Constant evaluation
rejects that division ({rule}`CE-28`), so a NaN check goes in code that
runs.
:::

:::{principle} CMP-P5
:rules: CMP-12 CMP-13 CMP-17 CMP-27 CMP-18 CMP-25

`<=>` compares once and gives an ordering value.

| Result | Used where | Examples |
|---|---|---|
| `strong_ordering` | Equal values are the same | Integers, object pointers |
| `partial_ordering` | Some pairs of values have no order, or equal values can differ | Floating-point types, where NaN has no order with any value; an integer against a floating-point value, such as `int` against `double` |

Because `<=>` stands in for all six comparisons, its operands must be
comparable without loss. Whether a conversion loses a value can depend
on the sizes of the integer types. These operand forms are rejected:

- a narrowing conversion, apart from an integer to a floating-point
  type. `int <=> double` and `long long <=> float` are valid and give
  `partial_ordering`,
- a `bool` against another type,
- a pointer against `0`,
- two different enumerations,
- an unscoped enumeration against a floating-point value.

An unscoped enumeration takes part as an integer only against an integer
type or its own type. Against any other type the enumeration has no
lossless common type with the other operand.

Function pointers and pointers to members have no order, so `<=>` does
not apply to them.
:::

:::{principle} CMP-P6
:rules: CMP-4 CMP-14 CMP-16 CMP-21

An implementation should warn about these comparisons, which are valid
and almost surely wrong:

- a signed operand converted to unsigned,
- a chained comparison,
- two array operands,
- the deprecated mixes of enumerations.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   three_way_expr: `shift_expr` {"<=>" `shift_expr`}
   relational_expr: `three_way_expr` {("<" | ">" | "<=" | ">=") `three_way_expr`}
   equality_expr: `relational_expr` {("==" | "!=") `relational_expr`}
```

## Constraints

:::{rule} CMP-1
:kind: required
:tests: expressions/comparison/grouping.cpp expressions/comparison/chained-comparison.cpp expressions/comparison/chained-requires-not-dependent.cpp expressions/comparison/chained-in-concept-no-warning.cpp expressions/comparison/chained-equality-gcc-warns.cpp expressions/comparison/chained-relational-equality-gcc-warns.cpp

`<=>` binds more tightly than the relational operators. The relational
operators bind more tightly than `==` and `!=`. Each group associates
from left to right. In `a < b < c`, the value of `a < b` is compared
with `c`.
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
:tests: expressions/comparison/pointer-void.cpp expressions/comparison/pointer-qualification.cpp expressions/comparison/single-array-operand.cpp expressions/comparison/three-way-array-and-pointer.cpp

Two pointers can be compared if one converts to the type of the other.
The conversion can add {term}`cv-qualifiers <cv-qualifier>` or convert
to `void*`. A pointer can be compared with `nullptr` for equality. An
array operand of any comparison operator, `<=>` included, converts to a
pointer to its first element.
:::

:::{rule} CMP-9
:kind: reject
:tests: expressions/comparison/pointer-integer.cpp expressions/comparison/pointer-relational-zero.cpp expressions/comparison/pointer-different-types.cpp expressions/comparison/function-pointer-void.cpp expressions/comparison/pointer-relational-nullptr.cpp expressions/comparison/relational-member-pointers.cpp expressions/comparison/relational-nullptr-values.cpp

A comparison must not have these operands:

- a pointer and an integer other than a literal `0` for equality,
- a pointer and the literal `0` or `nullptr` for a relational operator,
- pointers to unrelated types,
- a pointer to a function and `void*`,
- two pointers to members, or two values of the type of `nullptr`, for a
  relational operator ({question}`Q-0103` for `nullptr`).
:::

:::{rule} CMP-10
:kind: required
:tests: expressions/comparison/scoped-enum-same.cpp expressions/comparison/scoped-enum-order.cpp

Two values of the same scoped enumeration can be compared.
:::

:::{rule} CMP-23
:kind: reject
:tests: expressions/comparison/scoped-enum-different.cpp expressions/comparison/scoped-enum-integer.cpp

A value of a scoped enumeration must not be compared with a value of
another type, such as an integer or another scoped enumeration (CMP-10).
:::

:::{rule} CMP-11
:kind: required
:tests: expressions/comparison/array-comparison.cpp expressions/comparison/array-comparison-constant.cpp

If both operands of `==`, `!=`, `<`, `>`, `<=`, or `>=` are arrays, each
converts to a pointer to its first element. The comparison then compares
the addresses.
:::

:::{rule} CMP-24
:kind: reject
:tests: expressions/comparison/three-way-arrays.cpp expressions/comparison/three-way-same-array.cpp expressions/comparison/three-way-array-and-pointer.cpp

`<=>` must not have two array operands (CMP-11).
:::

:::{rule} CMP-12
:kind: required
:tests: expressions/comparison/three-way-type.cpp expressions/comparison/three-way-precedence.cpp expressions/comparison/three-way-array-and-pointer.cpp

`<=>` on integer, enumeration, or object pointer operands gives
`std::strong_ordering`. On floating-point operands it gives
`std::partial_ordering`. These types come from the library
({rule}`LSUP-2`).
:::

:::{rule} CMP-25
:kind: reject
:tests: expressions/comparison/three-way-no-library.cpp expressions/comparison/three-way-function-pointers.cpp expressions/comparison/three-way-member-pointers.cpp expressions/comparison/three-way-nullptr-values.cpp expressions/comparison/three-way-function-pointer-nullptr.cpp

The types that CMP-12 gives must be declared before `<=>` is used. `<=>`
must not compare:

- two function pointers,
- a function pointer and `nullptr`,
- two pointers to members,
- two values of the type of `nullptr`.
:::

:::{rule} CMP-13
:kind: reject
:tests: expressions/comparison/three-way-narrowing.cpp expressions/comparison/three-way-bool-int.cpp expressions/comparison/three-way-pointer-zero.cpp expressions/comparison/three-way-int-to-double.cpp expressions/comparison/three-way-long-unsigned-lp64.cpp expressions/comparison/three-way-long-unsigned-ilp32.cpp expressions/comparison/three-way-mixed-types.cpp

These operands of `<=>` are not valid:

- operands that need a {term}`narrowing conversion`. The conversion of an
  integer to a floating-point type is not counted as narrowing here.
- a `bool` and an operand of another type,
- a pointer and the literal `0`.

A constant whose value fits the converted type does not narrow. Whether
a conversion narrows can depend on the sizes of the integer types
({rule}`MUL-15`).
:::

:::{rule} CMP-18
:kind: reject
:tests: expressions/comparison/three-way-different-enums.cpp expressions/comparison/three-way-enum-float.cpp expressions/comparison/three-way-enum-int.cpp

`<=>` must not compare two different unscoped enumeration types, or an
unscoped enumeration and a floating-point value. An unscoped enumeration
and an integer are valid ({question}`Q-0050` for the floating-point
case).
:::

:::{rule} CMP-19
:kind: required
:tests: expressions/comparison/derived-base-pointer-equality.cpp expressions/comparison/multilevel-qualification.cpp expressions/comparison/null-pointer-equality.cpp expressions/comparison/noexcept-function-pointer-equality.cpp expressions/comparison/nullptr-equals-zero.cpp expressions/comparison/member-pointer-equality.cpp

Equality comparison is valid for these pointer operands:

- pointers whose types differ in cv-qualifiers at any level. The
  comparison uses a type that adds `const` where `const` is needed.
- a pointer to a derived class and a pointer to its base class, after
  CONV-14,
- two values of the type of `nullptr`, which compare equal,
- two pointers to members, where two null values compare equal,
- a pointer to a `noexcept` function and a pointer to a function of the
  same type without `noexcept`,
- `nullptr` and the literal `0`, which compare equal.
:::

## Runtime Behavior

:::{rule} CMP-3
:kind: required
:tests: expressions/comparison/signed-unsigned.cpp expressions/comparison/signed-unsigned-promoted.cpp

The comparison uses the values after conversion. If an `int` is
converted to `unsigned`, `-1` becomes the largest `unsigned` value.
:::

:::{rule} CMP-5
:kind: required
:tests: expressions/comparison/nan.cpp expressions/comparison/nan-float.cpp

If an operand is a NaN, `<`, `>`, `<=`, `>=`, and `==` give `false`.
`!=` gives `true`.
:::

:::{rule} CMP-7
:kind: required
:tests: expressions/comparison/pointer-array-order.cpp expressions/comparison/pointer-member-order.cpp expressions/comparison/subobject-order.cpp expressions/comparison/subobject-order-constant.cpp expressions/comparison/pointer-array-member-order.cpp expressions/comparison/row-end-equals-next-row.cpp expressions/comparison/same-base-members-constant.cpp

Pointers to elements of the same array compare in the order of their
indexes. A pointer one past the last element is greater than all element
pointers. Pointers to data members of one object compare in the order of
the member declarations (CLS-10). The order also applies to pointers
into these subobjects, for example into two elements or two members.
Constant evaluation can do these comparisons.
:::

:::{rule} CMP-26
:kind: required
:tests: expressions/comparison/row-end-equals-next-row.cpp expressions/comparison/row-end-equals-next-row-run.cpp

In an array of arrays, a pointer one past the end of one row and a
pointer to the next row's first element are equal. Constant evaluation
can do this comparison.
:::

:::{rule} CMP-15
:kind: required
:tests: expressions/comparison/pointer-equality.cpp expressions/comparison/pointer-equality-constant.cpp expressions/comparison/struct-and-first-member-equal.cpp expressions/comparison/union-members-equal.cpp expressions/comparison/struct-and-first-member-equal-run.cpp expressions/primary/string-literal-pointer-copy-equal.cpp

Two pointers are equal if they hold the same address, or if both are
null. Pointers to different objects can be equal. Examples are a pointer
to a class object and a pointer to its first member, after conversion to
`void*`, and pointers to two members of a union ({question}`Q-0104`).
:::

:::{rule} CMP-17
:kind: required
:tests: expressions/comparison/three-way-integers.cpp expressions/comparison/three-way-floating.cpp expressions/comparison/three-way-negative-zero.cpp expressions/comparison/three-way-mixed-types.cpp expressions/comparison/three-way-result-copies-member.cpp

For integer and pointer operands, `a <=> b` gives `less`, `equal`, or
`greater` as `a` is less than, equal to, or greater than `b`. For
floating-point operands, equal values give `equivalent`, and `-0.0` and
`0.0` are equal. If a floating-point operand is a NaN, the result is
`std::partial_ordering::unordered`.
:::

:::{rule} CMP-27
:kind: required
:tests: expressions/comparison/three-way-result-copies-member.cpp expressions/comparison/three-way-floating-and-pointer-copies-member.cpp

The result of a built-in `<=>` is a prvalue. The prvalue is initialized
from the static data member of the result type that {rule}`CMP-17`
names ({rule}`LSUP-2`).
:::

:::{rule} CMP-22
:kind: required
:tests: expressions/comparison/member-pointer-two-paths.cpp expressions/comparison/member-pointer-equality.cpp

Two pointers to members that are not null are equal if they select the
same member of the same subobject. If a class has one base on two paths,
the same member of the base through each path gives two different
pointers to members.
:::

## Implementation Requirements

:::{rule} CMP-8
:kind: unspecified
:tests: expressions/comparison/pointer-unrelated-run.cpp expressions/comparison/pointer-unrelated-constant.cpp expressions/comparison/function-pointer-order.cpp expressions/comparison/unrelated-pointers-either.cpp expressions/comparison/union-member-pointers-relational-either.cpp expressions/comparison/three-way-unrelated-pointers-constant.cpp expressions/comparison/zero-size-member-pointers-either.cpp expressions/comparison/zero-size-member-pointers-constant.cpp expressions/comparison/union-member-pointers-constant.cpp expressions/comparison/base-subobject-pointers-constant.cpp expressions/comparison/base-and-member-pointers-constant.cpp expressions/comparison/base-subobject-pointers-either.cpp expressions/comparison/base-member-and-member-pointers-constant.cpp expressions/comparison/same-base-members-constant.cpp

If no rule orders two pointers, each relational operator and `<=>` gives
an unspecified result. The results of `a < b` and `a > b` need not
agree. This rule applies to pointers to:

- unrelated objects or functions,
- members of a union,
- members that occupy no storage,
- two base class subobjects of one object, or a base class subobject
  and a member of the object.

Outside {term}`constant evaluation`, each such comparison is valid and
gives `true` or `false`. During constant evaluation, a comparison of
pointers to unrelated objects or functions is not a constant
expression, and neither is a comparison that involves a base class
subobject ({question}`Q-0387`). That covers two different base class
subobjects, and a base class subobject or a member of it against a
member of the derived object. Two members of one base class subobject
are members of one object, so their order is fixed.
:::

:::{rule} CMP-20
:kind: unspecified
:tests: expressions/comparison/one-past-end-adjacent-constant.cpp expressions/comparison/one-past-end-adjacent.cpp expressions/comparison/one-past-member-adjacent-constant.cpp

A pointer one past the end of one object can compare equal or not equal
to a pointer to another object that follows the first object in memory.
The result is unspecified. During constant evaluation, the equality
comparison is rejected if the first object is a complete object
({question}`Q-0051`). Inside one complete object, such as an array
member and the member after it, the comparison is accepted.
:::

## Recommended Practice

:::{rule} CMP-4
:kind: recommended
:tests: expressions/comparison/sign-compare-warns.cpp expressions/comparison/sign-compare-cast.cpp

An implementation should warn when a comparison converts a signed
operand to an unsigned type. An explicit cast should silence the
warning.
:::

:::{rule} CMP-14
:kind: recommended
:tests: expressions/comparison/chained-comparison-warns.cpp expressions/comparison/chained-less-equal-warns.cpp expressions/comparison/chained-mixed-relational-warns.cpp expressions/comparison/chained-parenthesized-no-warning.cpp expressions/comparison/chained-class-bool-warns.cpp expressions/comparison/chained-class-result-no-warning.cpp expressions/comparison/chained-fold-not-chain.cpp

An implementation should warn about a chained comparison. In a
*chained comparison*, the left operand of a relational operator is a
relational comparison of type `bool` without parentheses, as in
`a < b >= c`. These rules apply:

- The inner comparison can be built-in or call an operator function.
- A fold expression is not a chained comparison, because each nested
  part acts as if it were in parentheses ({rule}`PRIM-27`).
- Parentheses around the inner comparison should silence the warning.
:::

:::{rule} CMP-16
:kind: recommended
:tests: expressions/comparison/array-comparison-warns.cpp expressions/comparison/array-relational-warns.cpp

An implementation should warn when a comparison has two array operands.
:::

:::{rule} CMP-21
:kind: recommended
:tests: expressions/comparison/different-enums-warns.cpp expressions/comparison/enum-float-comparison-warns.cpp

An implementation should warn about a comparison between two different
enumeration types, and between an unscoped enumeration and a
floating-point value. Both comparisons are deprecated.
:::

## Examples

% tidal-example: reject
```cpp
1 < 2 == 3 > 4         // (1 < 2) == (3 > 4), false
3 > 2 > 1              // (3 > 2) > 1, so 1 > 1, false
-1 < 0u                // false: -1 becomes the largest unsigned value

int arr[4];
&arr[0] < &arr[3]      // true

int a, b;
&a < &b                // unspecified

int grid[2][3];
grid[0] + 3 == grid[1] // true
static_assert(&grid[0][3] == &grid[1][0]);  // true: one complete object

struct Record { int values[2]; int tail; };
Record r;
r.values + 2 <= &r.tail   // true, also during constant evaluation
r.values + 2 == &r.tail   // unspecified; accepted during constant evaluation
int x, y;
&x + 1 == &y              // unspecified; rejected during constant evaluation

int list[2];
list < &list[1]           // true: list becomes &list[0]

(1 <=> 2) == std::strong_ordering::less     // true
double zero = 0.0;
double nan = zero / zero;  // a NaN, at run time only
(1.0 <=> nan) == std::partial_ordering::unordered  // true

long balance = -1;
unsigned count = 1;
balance <=> count      // valid if long is wider than unsigned
                       // rejected if they have the same size
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 84
of 103 tests. The `<=>` tests declare a minimal `std::strong_ordering`,
`std::weak_ordering`, and `std::partial_ordering`, because tests include
no headers. EDG needs `std::weak_ordering` declared even for integer
operands.

### Divergences

- {question}`Q-0029` (CMP-1, CMP-14; settled on CMP-P6): whether a
  chained comparison such as `3 > 2 > 1` is valid. It is, and an
  implementation should warn about it. Clang 22 rejects a relational
  chain by default, through a `-Wparentheses` warning whose default
  severity is an error, and also rejects a fold and a chain in a
  requires expression that depends on no template parameter. GCC warns
  about more forms under `-Wparentheses`.
- {question}`Q-0387` (CMP-8; settled on CMP-P3): whether constant
  evaluation rejects a relational comparison of two base class
  subobjects of one object, or of a base class subobject and a member.
  It does, as Clang 22 and EDG say. GCC 16 gives a result.
- {question}`Q-0030` (CMP-9, CMP-13; settled on CMP-P2): whether a
  pointer may be compared with `<` or `<=>` to a literal `0`. It may
  not. EDG accepts `0`, `0L`, and `nullptr` there.
- {question}`Q-0103` (CMP-9; settled on CMP-P2): whether two values of
  the type of `nullptr` may be compared with a relational operator.
  They may not. EDG accepts `<`, `<=`, and `>=`, and rejects `<=>`.
- {question}`Q-0104` (CMP-15; settled on CMP-P3): whether a class
  object and its first member compare equal after conversion to
  `void*`. They do, as Clang and GCC say. EDG gives `false`.
- {question}`Q-0050` (CMP-18; settled on CMP-P5): whether `<=>` may
  compare an unscoped enumeration with a floating-point value. It may
  not. EDG accepts the comparison in either order and gives
  `std::partial_ordering`.
- {question}`Q-0051` (CMP-20; settled on CMP-P3): whether constant
  evaluation rejects the comparison of a pointer one past the end of a
  complete object with a pointer to the next object. It does, as Clang
  and GCC say. EDG accepts the comparison.

### Split tests

EDG does not take the warning flags, so it skips the tests that use
them.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `base-and-member-pointers-constant.cpp` | CMP-8 | reject | reject | accept | reject | {question}`Q-0387` |
| `base-member-and-member-pointers-constant.cpp` | CMP-8 | reject | reject | accept | reject | {question}`Q-0387` |
| `base-subobject-pointers-constant.cpp` | CMP-8 | reject | reject | accept | reject | {question}`Q-0387` |
| `chained-class-bool-warns.cpp` | CMP-14 | reject | accept | reject | skips | {question}`Q-0029` |
| `chained-comparison-warns.cpp` | CMP-14 | reject | Clang 22: reject; Clang 18: accept | reject | skips | {question}`Q-0029` |
| `chained-comparison.cpp` | CMP-1 | accept | Clang 22: reject; Clang 18: accept | accept | accept | {question}`Q-0029` |
| `chained-equality-gcc-warns.cpp` | CMP-1 | accept | accept | reject | skips | {question}`Q-0029` |
| `chained-fold-not-chain.cpp` | CMP-14, PRIM-27 | accept | Clang 22: reject; Clang 18: accept | accept | skips | {question}`Q-0029` |
| `chained-less-equal-warns.cpp` | CMP-14 | reject | Clang 22: reject; Clang 18: accept | reject | skips | {question}`Q-0029` |
| `chained-mixed-relational-warns.cpp` | CMP-14 | reject | Clang 22: reject; Clang 18: accept | reject | skips | {question}`Q-0029` |
| `chained-relational-equality-gcc-warns.cpp` | CMP-1 | accept | accept | reject | skips | {question}`Q-0029` |
| `chained-requires-not-dependent.cpp` | CMP-1 | accept | Clang 22: reject; Clang 18: accept | accept | accept | {question}`Q-0029` |
| `one-past-end-adjacent-constant.cpp` | CMP-20 | reject | reject | reject | accept | {question}`Q-0051` |
| `pointer-relational-nullptr.cpp` | CMP-9 | reject | reject | reject | accept | {question}`Q-0030` |
| `pointer-relational-zero.cpp` | CMP-9 | reject | reject | reject | accept | {question}`Q-0030` |
| `relational-nullptr-values.cpp` | CMP-9 | reject | reject | reject | accept | {question}`Q-0103` |
| `struct-and-first-member-equal.cpp` | CMP-15 | accept | accept | accept | reject | {question}`Q-0104` |
| `three-way-enum-float.cpp` | CMP-18 | reject | reject | reject | accept | {question}`Q-0050` |
| `three-way-pointer-zero.cpp` | CMP-13 | reject | reject | reject | accept | {question}`Q-0030` |

### Notes

- CMP-14 is Tidal's own advice. The gap review of the rules that phase
  7 rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`)
  finds no recommendation about chained comparisons in the reference.
  CMP-14 follows the warnings that Clang and GCC give.
- CMP-4, CMP-14, CMP-16: Clang and GCC give the warnings under
  `-Wsign-compare`, `-Wparentheses`, and `-Wall -Wextra`. GCC turns on
  `-Wparentheses` under `-Wall`. CMP-14 covers the four relational
  operators only. A chain in a dependent requirement, or in a template
  that is never instantiated, gets no warning on any compiler
  (`chained-in-concept-no-warning.cpp`), and neither does a chain whose
  inner `<` returns a class (`chained-class-result-no-warning.cpp`).
- CMP-7, CMP-26: every compiler orders pointers into two rows of one
  array of arrays, and into an array member and the member after it,
  during constant evaluation. Every compiler gives `true` for a pointer
  past the end of one row compared with the first element of the next
  row, during constant evaluation and at run time.
- CMP-8: for pointers to two members of a union, or to an empty member
  and its neighbor, all three implementations give a result during
  constant evaluation. The gap review of the text phase 8 wrote on
  expressions
  (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`)
  reads both comparisons as having no fixed result, so constant
  evaluation would reject them. CMP-8 keeps the compilers' answer
  (`documentation/decisions/pending.md`, row 4).
- CMP-8: at run time, every Clang and GCC configuration orders two
  unrelated global variables by address, so `&a < &b` and `&a > &b`
  disagree (`pointer-unrelated-run.cpp`). CMP-8 does not promise this
  result.
- CMP-8: all compilers reject the comparison of unrelated pointers
  during constant evaluation. A function name followed by `<` starts a
  template argument list, so the function-pointer test puts each operand
  in parentheses.
- CMP-11 and CMP-16: Clang and GCC warn that array comparison is
  deprecated. In strict C++23 mode both accept array comparison, also
  during constant evaluation.
- CMP-13: the usual arithmetic conversions of `long` and `unsigned` give
  `long` on x86-64 and `unsigned long` with `-m32`. Every compiler
  accepts `<=>` on the two types on x86-64 and rejects it with `-m32` as
  narrowing. Every compiler accepts a constant that fits the converted
  type, such as `1 <=> 2u`.
- CMP-15, sanitizer builds: with `-fsanitize=undefined` or
  `-fsanitize=null`, GCC 14 and 16 reject `!=` on the addresses of two
  variables in a `static_assert` for these variables:
  - two specializations of a variable template,
  - two static members of a class template,
  - two `inline constexpr` variables.

  GCC accepts two plain `constexpr` variables at namespace scope. Clang
  accepts every form with both flags. Without the flags, every compiler
  accepts every form. The writer probes
  `reports/clean-room/roadmap-10/writer-probes/w20-4-tag-address-*.json`
  show the results. The matrix builds no constant test with a sanitizer
  flag, so no test shows the split.
- CMP-20: inside one array of arrays, every compiler accepts the
  comparison during constant evaluation, as CMP-26 states. Inside one
  class object, every compiler accepts `==` and `<` between the pointer
  one past an array member and the next member, and gives `true` for
  both (`one-past-member-adjacent-constant.cpp`).
- CMP-21: Clang warns under `-Wdeprecated-enum-compare` for two
  different enumerations, and GCC warns under `-Wenum-compare`. For an
  enumeration and a floating-point value, both compilers warn under
  `-Wdeprecated-enum-float-conversion`. Each warning is on by default,
  and `-pedantic-errors` does not make it an error. A test turns the
  warning into an error with `-Werror`.

### Sources

Inputs: clang-sema-expr, clang-diag-sema.

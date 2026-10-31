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
an `unsigned` has become a large unsigned value. The operators bind in
three groups, and each group associates to the left:

```text
   binds more tightly
         <=>
         <   >   <=   >=
         ==  !=
   binds less tightly
```

So `a < b < c` compares a `bool` with `c`. The two operands are not ordered
with respect to each other ({rule}`SEQ-8`).
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
:rules: CMP-7 CMP-15 CMP-22
:exceptions: CMP-8 CMP-20

Pointers have an order only inside one thing:

| Pointers to | Order |
|---|---|
| Elements of one array | Index order, with the position past the end last |
| Members of one object | Declaration order |

Outside one thing there is no order, so a relational comparison of
pointers to unrelated objects gives an unspecified result, and the two
directions need not even agree. Two pointers are equal when they hold
one address, which happens for different objects too, for example an
object and its first member. A pointer past the end of one object can
hold the address of the next object in memory, and whether that compares
equal is unspecified. Two pointers to members are equal when they select
the same member of the same subobject.
:::

:::{principle} CMP-P4
:rules: CMP-5

A NaN is not equal to anything, not even itself, and is neither less nor
greater than anything. So every comparison with a NaN is `false`, except
`!=`, which is `true`.
:::

:::{principle} CMP-P5
:rules: CMP-12 CMP-13 CMP-17 CMP-18 CMP-25

`<=>` compares once and gives an ordering value.

| Result | Used where | Examples |
|---|---|---|
| `strong_ordering` | Equal values are the same | Integers, object pointers |
| `partial_ordering` | Some values are unordered, or equal values can differ | Floating-point types, with NaN as the unordered case |

Because `<=>` stands in for all six comparisons, its operands must be
comparable without loss. These are rejected:

- a narrowing conversion,
- a `bool` against another type,
- a pointer against `0`,
- two different enumerations.

Function pointers and pointers to members have no order, so `<=>` does
not apply to them.
:::

:::{principle} CMP-P6
:rules: CMP-4 CMP-14 CMP-16 CMP-21

An implementation should warn where a comparison is valid and almost
surely wrong. These cases are:

- a signed operand converted to unsigned,
- a chained comparison,
- two array operands,
- the deprecated mixes of enumerations.
:::

## Syntax

```{eval-rst}
.. productionlist:: cmp
   equality: `relational` {("==" | "!=") `relational`}
   relational: `three_way` {("<" | ">" | "<=" | ">=") `three_way`}
   three_way: `shift_expr` {"<=>" `shift_expr`}
```

## Constraints

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
:tests: expressions/comparison/scoped-enum-same.cpp

Two values of the same scoped enumeration can be compared.
:::

:::{rule} CMP-23
:kind: reject
:tests: expressions/comparison/scoped-enum-different.cpp

Values of two different scoped enumerations must not be compared
(CMP-10).
:::

:::{rule} CMP-11
:kind: required
:tests: expressions/comparison/array-comparison.cpp

If both operands of `==`, `!=`, `<`, `>`, `<=`, or `>=` are arrays, each
converts to a pointer to its first element. The comparison then compares
the addresses.
:::

:::{rule} CMP-24
:kind: reject
:tests: expressions/comparison/three-way-arrays.cpp

`<=>` must not have two array operands (CMP-11).
:::

:::{rule} CMP-12
:kind: required
:tests: expressions/comparison/three-way-type.cpp expressions/comparison/three-way-precedence.cpp

`<=>` on integer, enumeration, or object pointer operands gives
`std::strong_ordering`. On floating-point operands it gives
`std::partial_ordering`. These types come from the library
*(library referenced)*.
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
:tests: expressions/comparison/three-way-narrowing.cpp expressions/comparison/three-way-bool-int.cpp expressions/comparison/three-way-pointer-zero.cpp expressions/comparison/three-way-int-to-double.cpp

These operands of `<=>` are not valid:

- operands that need a narrowing conversion. The conversion of an
  integer to a floating-point type is not counted as narrowing here.
- a `bool` and an operand of another type,
- a pointer and the literal `0`.
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
:tests: expressions/comparison/derived-base-pointer-equality.cpp expressions/comparison/multilevel-qualification.cpp expressions/comparison/null-pointer-equality.cpp expressions/comparison/noexcept-function-pointer-equality.cpp expressions/comparison/nullptr-equals-zero.cpp

Equality comparison is valid for these pointer operands:

- pointers whose types differ in cv-qualifiers at any level. The
  comparison uses a type that adds `const` where it is needed.
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
:tests: expressions/comparison/pointer-array-order.cpp expressions/comparison/pointer-member-order.cpp expressions/comparison/subobject-order.cpp expressions/comparison/subobject-order-constant.cpp

Pointers to elements of the same array compare in the order of their
indexes. A pointer one past the last element is greater than all element
pointers. Pointers to data members of one object compare in the order of
the member declarations (CLS-10). The order also applies to pointers into
these subobjects, for example into two elements or two members. Constant
evaluation can do these comparisons.
:::

:::{rule} CMP-15
:kind: required
:tests: expressions/comparison/pointer-equality.cpp expressions/comparison/pointer-equality-constant.cpp expressions/comparison/struct-and-first-member-equal.cpp expressions/comparison/union-members-equal.cpp

Two pointers are equal if they hold the same address, or if both are
null. Pointers to different objects can be equal. Examples are a pointer
to a class object and a pointer to its first member, after conversion to
`void*`, and pointers to two members of a union ({question}`Q-0104`).
:::

:::{rule} CMP-17
:kind: required
:tests: expressions/comparison/three-way-integers.cpp expressions/comparison/three-way-floating.cpp expressions/comparison/three-way-negative-zero.cpp

For integer and pointer operands, `a <=> b` gives `less`, `equal`, or
`greater` as `a` is less than, equal to, or greater than `b`. For
floating-point operands, equal values give `equivalent`, and `-0.0` and
`0.0` are equal. If a floating-point operand is a NaN, the result is
`std::partial_ordering::unordered`.
:::

:::{rule} CMP-22
:kind: required
:tests: expressions/comparison/member-pointer-two-paths.cpp

Two pointers to members that are not null are equal if they select the
same member of the same subobject. If a class has one base on two paths,
the same member of the base through each path gives two different
pointers to members.
:::

## Implementation Requirements

:::{rule} CMP-8
:kind: unspecified
:tests: expressions/comparison/pointer-unrelated-run.cpp expressions/comparison/pointer-unrelated-constant.cpp expressions/comparison/function-pointer-order.cpp expressions/comparison/unrelated-pointers-either.cpp expressions/comparison/union-member-pointers-relational-either.cpp expressions/comparison/three-way-unrelated-pointers-constant.cpp expressions/comparison/zero-size-member-pointers-either.cpp expressions/comparison/zero-size-member-pointers-constant.cpp expressions/comparison/union-member-pointers-constant.cpp

If no rule orders two pointers, each relational operator and `<=>` gives
an unspecified result. The results of `a < b` and `a > b` need not
agree. This applies to pointers to these:

- unrelated objects or functions,
- members of a union,
- members that occupy no storage.

During {term}`constant evaluation`, a comparison of pointers to
unrelated objects or functions is not a constant expression.
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
:tests: expressions/comparison/different-enums-warns.cpp expressions/comparison/enum-float-comparison-warns.cpp

An implementation should warn about a comparison between two different
enumeration types, and between an unscoped enumeration and a
floating-point value. Both are deprecated.
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

- CMP-8: for pointers to two members of a union, or to an empty member
  and its neighbor, all three implementations give a result during
  constant evaluation. Only the unrelated-object case is rejected there.
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
- CMP-15: EDG evaluates the comparison of a class object and its first
  member, after conversion to `void*`, to `false`. Clang and GCC give
  `true` ({question}`Q-0104`).
- CMP-9: EDG accepts a relational comparison of two values of the type of
  `nullptr`. Clang and GCC reject it ({question}`Q-0103`).

Inputs: clang-sema-expr, clang-diag-sema.

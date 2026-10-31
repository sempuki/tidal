# Defaulted comparisons

A class can default its equality and three-way comparison operators. A
defaulted comparison compares the bases and members of two objects in
order. An expression can also use a comparison operator that the
implementation rewrites in terms of `==` or `<=>`. This page covers:

- defaulted comparison functions,
- their return types,
- when they are deleted,
- rewritten and reversed comparisons.

Depends on: comparison operators, operator overloading, derived classes.

## Principles

:::{principle} DCMP-P1
:rules: DCMP-1 DCMP-2 DCMP-3 DCMP-4 DCMP-5 DCMP-8 DCMP-9 DCMP-10 DCMP-11

A defaulted comparison compares part by part, bases first, in
declaration order, and stops at the first difference. The implementation
writes the function only when asked and only in a shape that the
implementation can fill in. When a part cannot be compared that way, the
implementation deletes the function, so only a use is an error
({principle}`CPY-P3`). The return type of a defaulted `<=>` is the
weakest ordering among the parts, and a defaulted `<=>` brings a
defaulted `==` with it.
:::

:::{principle} DCMP-P2
:rules: DCMP-6 DCMP-7 DCMP-12

The other comparison operators are spelled from `==` and `<=>`. An
expression with `!=` or a relational operator also tries `==` or `<=>`,
with the operands in either order, so a class need write only `==` and
`<=>`.
A declared operator wins over a rewritten one, and an implementation
should warn when the rewrites make a comparison ambiguous.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   defaulted_comparison: `comparison_decl` "=" "default" ";"
   comparison_decl: `type_id` "operator" `comparison_op` "(" `parameters` ")" ["const"] [`ref_qualifier`]
                  : | "friend" `type_id` "operator" `comparison_op` "(" `parameters` ")"
   comparison_op: "==" | "!=" | "<" | ">" | "<=" | ">=" | "<=>"
```

The *comparison category types* are:

- `std::strong_ordering`,
- `std::weak_ordering`,
- `std::partial_ordering`.

A program that uses a defaulted `<=>` must declare these types, usually
through the standard library ({rule}`LSUP-2`).

## Constraints

:::{rule} DCMP-3
:kind: reject
:tests: declarations/volatile/defaulted-comparison-volatile.cpp classes/comparisons/declared-outside-then-defaulted.cpp classes/comparisons/decltype-auto-return.cpp classes/comparisons/eq-auto-return.cpp classes/comparisons/eq-by-value-param.cpp classes/comparisons/eq-mixed-params.cpp classes/comparisons/eq-nonconst-member.cpp classes/comparisons/eq-nonmember-nonfriend.cpp classes/comparisons/eq-other-class-param.cpp classes/comparisons/eq-ref-qualified.cpp classes/comparisons/eq-rvalue-ref-qualified.cpp classes/comparisons/eq-wrong-param.cpp classes/comparisons/eq-wrong-return.cpp classes/comparisons/not-equal-defaulted.cpp classes/comparisons/template-defaulted.cpp

A defaulted comparison of class `C` must be one of these:

- a `const` member function with one parameter of type `const C&`,
- a friend of `C` with two parameters, both `const C&` or both `C`.

A defaulted comparison must not be:

- a template,
- a non-member that is not a friend,
- a friend first declared outside `C` ({question}`Q-0061`).

A defaulted `==`
or secondary operator (`!=`, `<`, `>`, `<=`, `>=`) must return `bool`.
The return type of a defaulted `<=>` must be written as `auto` if it is
deduced. A ref-qualifier `&` is valid. A ref-qualifier `&&` and `volatile`
are not.
:::

:::{rule} DCMP-1
:kind: required
:tests: classes/comparisons/eq-array-member.cpp classes/comparisons/eq-bases-first.cpp classes/comparisons/eq-constexpr.cpp classes/comparisons/eq-friend-by-value.cpp classes/comparisons/eq-friend.cpp classes/comparisons/eq-memberwise.cpp classes/comparisons/eq-out-of-class.cpp classes/comparisons/eq-private-members.cpp classes/comparisons/eq-stops-at-first-false.cpp

A defaulted `==` compares the direct bases in the order of the base list,
then the non-static data members in the order of their declarations. An
array member is compared element by element. The first comparison that
gives `false` ends the comparison. The defaulted `==` can be used in
constant evaluation and has access to private members.
:::

:::{rule} DCMP-2
:kind: required
:tests: classes/comparisons/eq-member-not-comparable-unused.cpp classes/comparisons/member-eq-returns-int.cpp

A defaulted `==` is deleted if one of these is true:

- a base or member has no usable `==`, or the result of that `==` does
  not convert to `bool` as a condition does,
- the class has a reference member,
- the class is a union, or has an anonymous union member.

A deleted defaulted `==` makes only a use of the operator invalid.
:::

:::{rule} DCMP-9
:kind: reject
:tests: classes/comparisons/eq-anonymous-union-member.cpp classes/comparisons/eq-member-eq-private.cpp classes/comparisons/eq-member-not-comparable.cpp classes/comparisons/eq-reference-member.cpp classes/comparisons/eq-union.cpp

A program must not use a defaulted `==` that DCMP-2 deletes.
:::

:::{rule} DCMP-4
:kind: required
:tests: classes/comparisons/empty-class-compares-equal.cpp classes/comparisons/secondary-defaulted.cpp classes/comparisons/spaceship-array.cpp classes/comparisons/spaceship-auto-partial.cpp classes/comparisons/spaceship-auto-strong.cpp classes/comparisons/spaceship-auto-weak-member.cpp classes/comparisons/spaceship-bases-first.cpp classes/comparisons/spaceship-bool-member-ok.cpp classes/comparisons/spaceship-constexpr.cpp classes/comparisons/spaceship-declares-eq.cpp classes/comparisons/spaceship-memberwise.cpp classes/comparisons/spaceship-relational-rewrite.cpp classes/comparisons/spaceship-specified-type-synthesized.cpp

A defaulted `<=>` compares the bases and members in the same order as
`==`. The result is the first result that is not equal, or equal if all
are equal. A class with no bases and no members compares equal. These
rules apply:

- With the return type `auto`, the type is the weakest category among
  the members. `partial_ordering` is weaker than `weak_ordering`, which
  is weaker than `strong_ordering`.
- With a category type as return type, a member without `<=>` is
  compared with `==` and `<`.
- A defaulted `<=>` declares a defaulted `==` too, if the class declares
  no `==`. A `<=>` that is not defaulted does not.
- A defaulted secondary operator calls `<=>`.
:::

:::{rule} DCMP-10
:kind: reject
:tests: classes/comparisons/spaceship-no-eq-if-not-defaulted.cpp classes/comparisons/spaceship-no-header-types.cpp

If a class has a `<=>` that is not defaulted and declares no `==`, a
program must not compare objects of the class with `==`. A program that
uses a defaulted `<=>` must declare the comparison category types.
:::

:::{rule} DCMP-5
:kind: required
:tests: classes/comparisons/spaceship-wrong-return.cpp

A defaulted `<=>` is deleted in these cases:

- The return type is not `auto` or a comparison category type.
- The return type is `auto`, and a member has no `<=>`, or the member's
  `<=>` does not return a comparison category type.

A defaulted secondary operator is deleted if the class has no usable
`<=>`.
:::

:::{rule} DCMP-11
:kind: reject
:tests: classes/comparisons/secondary-without-spaceship.cpp classes/comparisons/spaceship-auto-no-member-spaceship.cpp classes/comparisons/spaceship-member-returns-non-category.cpp classes/comparisons/spaceship-wrong-return-used.cpp

A program must not use a defaulted `<=>` or secondary operator that
DCMP-5 deletes.
:::

:::{rule} DCMP-6
:kind: required
:tests: classes/comparisons/ne-declared-direct-ok.cpp classes/comparisons/ne-declared-not-rewritten.cpp classes/comparisons/non-member-reversed.cpp classes/comparisons/reversed-eq.cpp classes/comparisons/rewritten-ne.cpp classes/comparisons/rewritten-relational-reversed.cpp

An expression with `==`, `!=`, `<`, `>`, `<=`, `>=` also considers these
candidates:

- for `x != y`, the candidate `!(x == y)`,
- for `x < y` and the other relational operators, `(x <=> y) < 0` and
  similar,
- each of these candidates with the operands reversed, such as `y == x`
  for `x == y`.

`<=>` is never used to rewrite `==` or `!=`. A declared `!=` is used
before a rewritten one. If the scope of an `==` declares a matching
`!=`, that `==` is not used for reversed or rewritten candidates.
:::

:::{rule} DCMP-12
:kind: reject
:tests: classes/comparisons/ne-declared-blocks-reversed.cpp classes/comparisons/ne-needs-bool-eq.cpp classes/comparisons/not-rewritten-for-eq-from-spaceship.cpp

An `==` used to rewrite `!=` must return `bool`. A comparison that needs
a candidate that DCMP-6 excludes is not valid.
:::

## Implementation Requirements

:::{rule} DCMP-8
:kind: no-diagnostic
:tests: classes/comparisons/out-of-class-would-delete.cpp

A comparison defaulted after the class definition must not be one that
would be deleted. An implementation need not diagnose such a comparison
({question}`Q-0062`).
:::

## Recommended Practice

:::{rule} DCMP-7
:kind: recommended
:tests: classes/comparisons/rewritten-ambiguous-const-ok.cpp classes/comparisons/rewritten-ambiguous-warns.cpp

An implementation should warn if a reversed candidate makes a comparison
ambiguous, for example an `==` that is not `const`.
:::

## Examples

```cpp
struct Version {
  int major, minor, patch;
  auto operator<=>(const Version&) const = default;  // also declares ==
};
static_assert(Version{1, 2, 0} < Version{1, 10, 0});
static_assert(Version{2, 0, 0} == Version{2, 0, 0});

struct Reading {
  double value;
  auto operator<=>(const Reading&) const = default;  // partial_ordering
};

struct Id {
  int n;
  bool operator==(int other) const { return n == other; }
};
bool a = 7 == Id{7};             // reversed: Id{7} == 7
bool b = Id{7} != 8;             // rewritten: !(Id{7} == 8)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 61
of 63 tests. The tests declare the comparison category types themselves.

- DCMP-3: GCC accepts defaulting a friend that was declared outside the
  class. Clang and EDG reject it ({question}`Q-0061`).
- DCMP-8: Clang rejects the definition. GCC and EDG accept it
  ({question}`Q-0062`).
- DCMP-7: Clang and GCC warn under `-Wall`.

Inputs: clang-sema-declcxx, clang-sema-expr, clang-diag-sema,
edg-class-decl, edg-overload.

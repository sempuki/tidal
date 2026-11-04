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
implementation deletes the function, so the implementation rejects only
a use ({principle}`CPY-P3`). A comparison defaulted after the class is a
definition that the program asks for, so the implementation rejects one
that would be deleted, as for the copy and move functions
({rule}`CPY-17`). The return type of a defaulted `<=>` is the
weakest ordering among the parts, and a defaulted `<=>` brings a
defaulted `==` with it.
:::

:::{principle} DCMP-P2
:rules: DCMP-6 DCMP-7 DCMP-12 DCMP-13

The other comparison operators are spelled from `==` and `<=>`. An
expression with `!=` or a relational operator also tries `==` or `<=>`,
with the operands in either order, so a class need write only `==` and
`<=>`. An expression with `<=>` also tries `<=>` with the operands
reversed.
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
:tests: declarations/volatile/defaulted-comparison-volatile.cpp classes/comparisons/declared-outside-then-defaulted.cpp classes/comparisons/decltype-auto-return.cpp classes/comparisons/eq-auto-return.cpp classes/comparisons/eq-by-value-param.cpp classes/comparisons/eq-mixed-params.cpp classes/comparisons/eq-nonconst-member.cpp classes/comparisons/eq-nonmember-nonfriend.cpp classes/comparisons/eq-other-class-param.cpp classes/comparisons/eq-ref-qualified.cpp classes/comparisons/eq-rvalue-ref-qualified.cpp classes/comparisons/eq-wrong-param.cpp classes/comparisons/eq-wrong-return.cpp classes/comparisons/not-equal-defaulted.cpp classes/comparisons/template-defaulted.cpp classes/comparisons/class-template-defaulted.cpp

A defaulted comparison of class `C` must be one of these:

- a `const` member function with one parameter of type `const C&`,
- a friend of `C` with two parameters, both `const C&` or both `C`.

A defaulted comparison must not be:

- a {term}`function template` (a comparison declared in a {term}`class template` is
  valid, and each {term}`specialization` gets its own),
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
{term}`constant evaluation` and has access to private members.
:::

:::{rule} DCMP-2
:kind: required
:tests: classes/comparisons/eq-member-not-comparable-unused.cpp classes/comparisons/member-eq-returns-int.cpp

A defaulted `==` is deleted if one of these is true:

- a base or member has no usable `==`, or the result of that `==` does
  not convert to `bool` as a condition does,
- the class has a reference member,
- the class is a union, or has an anonymous union member.

A deleted defaulted `==` makes only a use of the operator not valid.
:::

:::{rule} DCMP-9
:kind: reject
:tests: classes/comparisons/eq-anonymous-union-member.cpp classes/comparisons/eq-member-eq-private.cpp classes/comparisons/eq-member-not-comparable.cpp classes/comparisons/eq-reference-member.cpp classes/comparisons/eq-union.cpp

A program must not use a defaulted `==` that DCMP-2 deletes.
:::

:::{rule} DCMP-4
:kind: required
:tests: classes/comparisons/empty-class-compares-equal.cpp classes/comparisons/secondary-defaulted.cpp classes/comparisons/spaceship-array.cpp classes/comparisons/spaceship-auto-partial.cpp classes/comparisons/spaceship-auto-strong.cpp classes/comparisons/spaceship-auto-weak-member.cpp classes/comparisons/spaceship-bases-first.cpp classes/comparisons/spaceship-bool-member-ok.cpp classes/comparisons/spaceship-constexpr.cpp classes/comparisons/spaceship-declares-eq.cpp classes/comparisons/spaceship-memberwise.cpp classes/comparisons/spaceship-relational-rewrite.cpp classes/comparisons/spaceship-specified-type-synthesized.cpp classes/comparisons/not-equal-defaulted.cpp classes/comparisons/not-equal-defaulted-calls-eq.cpp

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
- A defaulted `!=` calls `==` and negates the result. A defaulted `<`,
  `>`, `<=`, or `>=` calls `<=>`.
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
:tests: classes/comparisons/spaceship-wrong-return.cpp classes/comparisons/not-equal-defaulted-no-eq-unused.cpp

A defaulted `<=>` is deleted in these cases:

- The return type is not `auto` or a comparison category type.
- The return type is `auto`, and a member has no `<=>`, or the member's
  `<=>` does not return a comparison category type.

A defaulted `!=` is deleted if the class has no usable `==`. A defaulted
`<`, `>`, `<=`, or `>=` is deleted if the class has no usable `<=>`. A
deleted function is valid to declare, and only a use is rejected
(DCMP-11).
:::

:::{rule} DCMP-11
:kind: reject
:tests: classes/comparisons/secondary-without-spaceship.cpp classes/comparisons/spaceship-auto-no-member-spaceship.cpp classes/comparisons/spaceship-member-returns-non-category.cpp classes/comparisons/spaceship-wrong-return-used.cpp classes/comparisons/not-equal-defaulted-no-eq.cpp

A program must not use a defaulted `<=>` or secondary operator that
DCMP-5 deletes.
:::

:::{rule} DCMP-6
:kind: required
:tests: classes/comparisons/ne-declared-direct-ok.cpp classes/comparisons/ne-declared-not-rewritten.cpp classes/comparisons/non-member-reversed.cpp classes/comparisons/reversed-eq.cpp classes/comparisons/rewritten-ne.cpp classes/comparisons/rewritten-relational-reversed.cpp foundations/overload-resolution/rewritten-three-way-reversed.cpp

An expression with `==`, `!=`, `<`, `>`, `<=`, `>=`, or `<=>` also
considers these candidates:

- for `x != y`, the candidate `!(x == y)`,
- for `x < y` and the other relational operators, `(x <=> y) < 0` and
  similar,
- each of these candidates with the operands reversed, such as `y == x`
  for `x == y`,
- for `x <=> y`, the candidate `y <=> x` with the operands reversed.

`<=>` is never used to rewrite `==` or `!=`. A declared `!=` is used
before a rewritten one. If an `==` has a matching `!=` (DCMP-13), that
`==` is not used for reversed or {term}`rewritten candidates <rewritten candidate>`.
{rule}`OVL-13` gives what each rewritten candidate evaluates, and
{rule}`OVL-32` and {rule}`OVL-33` rank the candidates.
:::

:::{rule} DCMP-12
:kind: reject
:tests: classes/comparisons/ne-declared-blocks-reversed.cpp classes/comparisons/ne-needs-bool-eq.cpp classes/comparisons/not-rewritten-for-eq-from-spaceship.cpp classes/comparisons/reversed-eq-needs-bool.cpp classes/comparisons/eq-cv-bool-result.cpp classes/comparisons/eq-reference-to-bool-result.cpp classes/comparisons/ne-rewrite-reference-to-bool-result.cpp classes/comparisons/ne-matching-other-result.cpp classes/comparisons/ne-matching-base-member.cpp classes/comparisons/ne-matching-using-declaration.cpp classes/comparisons/ne-matching-ref-qualifier.cpp

An `==` used to rewrite `!=`, and a reversed `==` that wins for `==`,
must return `bool`. The `bool` can be cv-qualified, and a reference to
`bool` does not count ({question}`Q-0158`). A comparison that needs a
candidate that DCMP-6 excludes is not valid.
:::

:::{rule} DCMP-13
:kind: required
:tests: classes/comparisons/ne-declared-blocks-reversed.cpp classes/comparisons/ne-matching-other-result.cpp classes/comparisons/ne-matching-base-member.cpp classes/comparisons/ne-matching-using-declaration.cpp classes/comparisons/ne-matching-ref-qualifier.cpp classes/comparisons/ne-not-matching.cpp classes/comparisons/ne-not-matching-friend.cpp

An `operator!=` matches an `operator==` for DCMP-6 if these conditions
hold:

- A search finds the `!=`. For a member `==`, the search is member
  lookup in the class of the object operand, which also finds a `!=`
  that a base declares. For a non-member `==`, the
  search looks in the namespace of the `==`, and a `friend` declared only
  in a class is not found.
- Both are members with an {term}`implicit object parameter`, or both are
  non-members, and neither is a template.
- Both have the same parameter types ({rule}`DCL-8`).

The return types, the {term}`cv-qualifiers <cv-qualifier>` and ref-qualifiers of members,
access, and deletion take no part ({question}`Q-0190`).
:::

:::{rule} DCMP-8
:kind: reject
:tests: classes/comparisons/out-of-class-would-delete.cpp classes/comparisons/out-of-class-reference-member-would-delete.cpp

A comparison defaulted after the class definition must not be one that
would be deleted. The program is not valid also when nothing uses the
comparison ({question}`Q-0062`).
:::

## Recommended Practice

:::{rule} DCMP-7
:kind: recommended
:tests: classes/comparisons/rewritten-ambiguous-const-ok.cpp classes/comparisons/rewritten-ambiguous-warns.cpp

An implementation should warn if a reversed candidate makes a comparison
ambiguous, for example an `==` that is not `const`. In strict mode,
{rule}`OPR-15` rejects such a comparison.
:::

## Examples

% tidal-example: fragment
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

struct Code {
  int n;
  bool operator==(const Code&) const = default;
  bool operator!=(const Code&) const = default;  // calls ==, no <=>
};
```

An `==` that is not `const` makes `a == b` ambiguous for two non-`const`
objects (DCMP-7). The normal candidate binds `a` to `Loose&` and `b` to
`const Loose&`. The reversed candidate binds them the other way, so
each candidate is better for one argument. With a `const` member, both
candidates bind `const` references, and the normal one wins
({rule}`OVL-32`):

% tidal-example: reject
```cpp
struct Loose { int v; bool operator==(const Loose& o) { return v == o.v; } };
struct Tight { int v; bool operator==(const Tight& o) const { return v == o.v; } };
bool f(Loose a, Loose b) { return a == b; }  // warning; rejected in strict mode
bool g(Tight a, Tight b) { return a == b; }  // valid, no warning
```

A class template can compare its specializations with each other. For
`a == b`, the friend from the class of `a` is a normal candidate, and
the friend from the class of `b` is a reversed one. The normal
candidate wins ({rule}`OVL-32`), so the call is not ambiguous:

```cpp
template <int N> struct Name {
  char text[N];
  template <int M>
  friend bool operator==(const Name& a, const Name<M>& b);
};
Name<2> s;
Name<8> l;
bool x = s == l;   // the friend from Name<2>, with M = 8
bool y = l == s;   // the friend from Name<8>, with M = 2
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 73
of 79 tests. The tests declare the comparison category types themselves.

- DCMP-3: GCC accepts defaulting a friend that was declared outside the
  class. Clang and EDG reject it ({question}`Q-0061`).
- DCMP-8: Clang rejects the definition. GCC and EDG accept it
  ({question}`Q-0062`). DCMP-8 is a `reject` rule, as {rule}`CPY-17` is
  for the copy and move functions, and the tests record GCC and EDG as
  overrides.
- DCMP-7: Clang and GCC give the warning by default, and
  `-pedantic-errors` makes the warning an error. EDG reports the
  comparison as ambiguous.
- DCMP-12: a reversed or rewritten `==` that returns `bool&` or
  `bool&&` is rejected by Clang and EDG. GCC accepts it
  ({question}`Q-0158`). Every compiler accepts a cv-qualified `bool`.
- DCMP-12: an `==` that rewrites `!=` follows the same result rules as
  a reversed `==`. Every compiler accepts a cv-qualified `bool`. Clang
  and EDG reject `const bool&`, and GCC accepts it ({question}`Q-0158`).
- DCMP-13: the compilers agree on what makes a `!=` match an `==`,
  except for the ref-qualifiers `&` and `&&`. Clang and EDG ignore the
  ref-qualifiers, and GCC treats `const&` and `const&&` as different
  ({question}`Q-0190`). The rule comes from probes of each condition.
- DCMP-6, DCMP-12: the reversed `<=>` candidate and the `bool` result of
  a reversed `==` come from items M-1 and M-3 of the gap review
  `reports/review/2026-10-31-gap-review-overload-resolution.md`, and from
  probes.

Inputs: clang-sema-declcxx, clang-sema-expr, clang-diag-sema,
edg-class-decl, edg-overload.

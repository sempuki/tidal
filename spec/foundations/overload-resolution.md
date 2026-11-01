# Overload resolution

Overload resolution chooses one function when a name, an operator, or
an initialization can call more than one. This page covers:

- the candidate functions that each context provides,
- which candidates are viable,
- how two viable functions compare, and each tie-breaker,
- the checks on the chosen function, and the cases with no result.

Depends on: name lookup, implicit conversion sequences, constructors,
conversion functions, operator functions.

## Principles

:::{principle} OVL-P1
:rules: OVL-3 OVL-22 OVL-23 OVL-24 OVL-25 OVL-34 OVL-35 OVL-36 OVL-37 OVL-38

Resolution runs in four steps. Each step takes the set that the
previous step gives.

```text
a call, an operator, an initialization, or &f
   |
   v
1. candidates   the functions that the context provides     OVL-P2
   |
   v
2. viable       the parameter count fits, the constraints   OVL-22 to OVL-24
   |            hold, and every argument has a sequence     none: OVL-35
   v
3. best         the one viable function that is better      OVL-25, OVL-34
   |            than each other one                         none: OVL-36
   v
4. checks       deleted, access, faults in the sequences,   OVL-37, OVL-3,
                an object for a member, default arguments   OVL-38
```

Steps 2 and 3 look only at types and value categories. Access, deletion,
and the faults that {rule}`ICS-8` lists come into play only in step 4, so
a function can win and then make the program not valid.
:::

:::{principle} OVL-P2
:rules: OVL-1 OVL-2 OVL-4 OVL-5 OVL-6 OVL-7 OVL-8 OVL-9 OVL-10 OVL-11 OVL-12 OVL-13 OVL-14 OVL-15 OVL-16 OVL-17 OVL-18 OVL-19 OVL-20 OVL-21

The context decides the candidates and the arguments. A member function
gets one more argument, the {term}`implied object argument`, which meets
its object parameter ({rule}`ICS-13`):

| Context | Candidates | Implied object argument | Rules |
|---|---|---|---|
| `f(a)`, `N::f(a)` | the functions that lookup finds | the current object, if lookup finds members | OVL-1 to OVL-3 |
| `x.f(a)`, `p->f(a)` | the members that member lookup finds | `x`, or `*p` | OVL-4 |
| `c(a)`, for a class object `c` | call operators and {term}`surrogate candidates <surrogate candidate>` | `c` | OVL-5, OVL-6 |
| `x @ y`, `@x` | members, non-members, {term}`built-in candidates <built-in candidate>`, {term}`rewritten candidates <rewritten candidate>` | the left operand, for a member | OVL-7 to OVL-13 |
| initialization of a class | constructors, and conversion functions of the initializer | the initializer, for a conversion function | OVL-14, OVL-16, OVL-17, {rule}`ILC-3` |
| initialization of a reference or a non-class object from a class | conversion functions of the initializer | the initializer | OVL-15, {rule}`ICS-4` |
| `&f` and `f` with a target type | the functions that `f` names | none | OVL-18 to OVL-20 |
| `(&f)(a)` | the functions that `f` names | none | OVL-21 |

Each function template that lookup finds adds at most one
specialization, by deduction *(templates referenced)*.
:::

:::{principle} OVL-P3
:rules: OVL-25 OVL-26 OVL-27 OVL-28 OVL-29 OVL-30 OVL-31 OVL-32 OVL-33 OVL-34

One viable function `F1` is better than another, `F2`, by these steps.
The first step that tells them apart decides:

```text
viable functions F1 and F2
   |
   v
1. arguments     no sequence of F1 is worse (ICS-P5),             OVL-25
   |             and some sequence of F1 is better
   |  every sequence ties
   v
2. tie-breakers, the first one that applies:
   a. result      the result of a conversion function converts     OVL-26
                  better to the target
   b. template    a plain function beats a template                OVL-27
                  specialization
   c. template    the more specialized template wins               OVL-28
   d. reference   the conversion function returns the kind of      OVL-29
                  function reference that the target is
   e. concepts    the more constrained plain function wins         OVL-30
   f. inherited   a constructor of the derived class beats an      OVL-31
                  inherited constructor
   g. rewritten   a written candidate beats a rewritten one        OVL-32
   h. swapped     a rewritten candidate in the written order       OVL-33
                  beats a swapped one
   |
   v
3. no step decides: neither function is better
```

The steps look first at what the call does to the values, then at how
the functions are declared, and last at how the expression was written.
The best function must be better than each other viable function
(OVL-34).

Tests observe these orders: 2a before 2b and 2e, 2b before 2d, 2f,
and 2g, and 2d before 2e. The order of 2b and 2d is provisional
({question}`Q-0148`). These pairs never decide the same case, so their
order is not observable:

- 2b, 2c, and 2e compare a plain function with a template, two
  templates, and two plain functions.
- 2a and 2d apply to conversion functions, 2f to constructors, and 2g
  and 2h to operators.
- 2a looks at results whose sequences differ, and 2d at two results
  that both bind with the identity. 2a also never separates two
  templates, whose results match exactly ({rule}`ICS-5`).
- 2g compares a written candidate with a rewritten one, and 2h compares
  two rewritten candidates.

No test decides the order of 2c and 2d, or of 2c and 2e against 2f to
2h.
:::

:::{principle} OVL-P4
:rules: OVL-9 OVL-10 OVL-11 OVL-12

The built-in operators take part through candidates with fixed
parameter types. A class operand reaches them through a conversion
function, so a class that converts to `int` can use the built-in `+`. In
the table, these letters stand for sets of types:

- `A`: a {term}`promoted arithmetic type`, which is a floating-point
  type or an integer type that promotion leaves unchanged
  ({rule}`CONV-1`),
- `I`: a promoted integer type,
- `X`: an arithmetic type, and `J`: an integer type,
- `T`: an object type,
- `E`: an enumeration type,
- `P`: a pointer type,
- `V`: `volatile` or nothing.

| Operators | Candidates | Result | Page |
|---|---|---|---|
| `*`, `/`, `+`, `-` | `(A, A)` for every pair | the common type of the two | {doc}`/expressions/multiplicative`, {doc}`/expressions/additive` |
| `%`, `&`, `^`, `\|`, `<<`, `>>` | `(I, I)` for every pair | the common type; for a shift, the left type | {doc}`/expressions/multiplicative`, {doc}`/expressions/bitwise`, {doc}`/expressions/shift` |
| `+`, `-` with pointers | `(T*, std::ptrdiff_t)`, `(std::ptrdiff_t, T*)` for `+`, `(T*, T*)` for `-` | `T*`, or `std::ptrdiff_t` | {doc}`/expressions/additive` |
| `[]` | `(T*, std::ptrdiff_t)` and `(std::ptrdiff_t, T*)` | `T&` | {doc}`/expressions/postfix` |
| unary `+`, `-` | `(A)`; also `(P)` for `+` | the same type | {doc}`/expressions/unary` |
| `~` | `(I)` | `I` | {doc}`/expressions/unary` |
| unary `*` | `(T*)`, and `(F*)` for a function type `F` without cv-qualifiers | `T&`, `F&` | {doc}`/expressions/unary` |
| `!`, `&&`, `\|\|` | `(bool)`, `(bool, bool)` | `bool` | {doc}`/expressions/logical` |
| `==`, `!=`, `<`, `>`, `<=`, `>=` | `(A, A)` for every pair; `(P, P)` and `(E, E)` for one type | `bool` | {doc}`/expressions/comparison` |
| `==`, `!=` only | `(M, M)` for one member pointer type `M`; `(std::nullptr_t, std::nullptr_t)` | `bool` | {doc}`/expressions/comparison` |
| `<=>` | `(J, J)` for one integer type; `(A, A)` for every pair of floating-point types; `(P, P)` and `(E, E)` for one type | an ordering type | {doc}`/expressions/comparison` |
| `->*` | `(C*, M C2::*)` where `C2` is `C` or a base of `C` | a reference to `M` | {doc}`/expressions/member-pointers` |
| `++`, `--` | `(VX&)` for every `X` except `bool`; `(T* V&)` | prefix: the operand; postfix: its old value | {doc}`/expressions/unary`, {doc}`/expressions/postfix` |
| `=` | `(VX&, A)` for every `X`; `(P V&, P)`, `(E V&, E)`, `(M V&, M)` | the left operand | {doc}`/expressions/assignment` |
| `*=`, `/=`, `+=`, `-=` | `(VX&, A)` for every `X`; `(T* V&, std::ptrdiff_t)` for `+=`, `-=` | the left operand | {doc}`/expressions/assignment` |
| `%=`, `&=`, `^=`, `\|=`, `<<=`, `>>=` | `(VJ&, I)` for every `J` | the left operand | {doc}`/expressions/assignment` |
| `?:` | {rule}`COND-11` | | {doc}`/expressions/conditional` |
| `,`, unary `&`, `->` | none (OVL-12) | | |

The left operand of a built-in assignment gets no user-defined
conversion ({rule}`ICS-10`). After a built-in candidate wins, the
operator's own page decides what the expression does (OVL-11).
:::

## Constraints

### Calls

:::{rule} OVL-1
:kind: required
:tests: foundations/overload-resolution/call-by-name-candidates.cpp

The {term}`candidate functions <candidate function>` of a call by name
are the functions that name lookup finds ({rule}`NAM-5`). Each function
template found adds the specialization that deduction gives
*(templates referenced)*. A template whose deduction fails adds no
candidate.
:::

:::{rule} OVL-2
:kind: required
:tests: foundations/overload-resolution/current-object-argument.cpp foundations/overload-resolution/current-object-explicit-parameter.cpp foundations/overload-resolution/current-object-explicit-extra-argument.cpp

If a call by name finds members and appears in a non-static member
function, `*this` is the implied object argument. `*this` meets the
implicit object parameter ({rule}`ICS-13`), or the explicit object
parameter ({rule}`ICS-41`).
:::

:::{rule} OVL-3
:kind: reject
:tests: foundations/overload-resolution/no-object-nonstatic-chosen.cpp foundations/overload-resolution/no-object-static-chosen.cpp foundations/overload-resolution/no-object-static-default-ambiguous.cpp foundations/overload-resolution/call-through-address-nonstatic.cpp

A call with no object can still have members as candidates, for example
a call by name in a static member function. If such a call chooses a
member with an implicit object parameter, the program is not valid. A
static member can be chosen.
:::

:::{rule} OVL-4
:kind: required
:tests: foundations/overload-resolution/member-call-object.cpp

The candidates of `x.f(a)` are the members `f` that lookup finds in the
class of `x`. `x` is the implied object argument, with its
cv-qualifiers and value category. `p->f(a)` is `(*p).f(a)`.
:::

:::{rule} OVL-5
:kind: required
:tests: foundations/overload-resolution/object-call-candidates.cpp

The candidates of `c(a)`, for an object `c` of class type, are these:

- the function call operators of the class, with `c` as the implied
  object argument,
- the surrogate candidates of OVL-6.
:::

:::{rule} OVL-6
:kind: required
:tests: foundations/overload-resolution/surrogate-forms.cpp foundations/overload-resolution/surrogate-from-base.cpp foundations/overload-resolution/surrogate-qualifiers.cpp foundations/overload-resolution/surrogate-explicit-not-candidate.cpp foundations/overload-resolution/surrogate-same-parameters.cpp

A conversion function gives a {term}`surrogate candidate` for `c(a)` if
these conditions hold:

- The function converts to a pointer to function, a reference to such a
  pointer, or a reference to function.
- The function is not `explicit`.
- The function's cv-qualifiers include those of `c`.
- A declaration in the class of `c` does not hide the function.

The surrogate's first parameter is the conversion's result type, and
`c` converts to that type by the conversion function. The other parameters are
those of the function type.
:::

### Operators

:::{rule} OVL-7
:kind: required
:tests: foundations/overload-resolution/operator-sets.cpp foundations/overload-resolution/operator-lookup-skips-members.cpp foundations/overload-resolution/operator-call-by-name-hidden.cpp

For an operator with an operand of class or enumeration type, the
candidates are these ({rule}`OPR-6`):

- the member operator functions of the left operand's class,
- the non-member operator functions that lookup finds, with members
  skipped, and argument-dependent lookup,
- the built-in candidates of OVL-9,
- the rewritten candidates of {rule}`DCMP-6`.

A call by the operator function's name uses ordinary lookup instead, so
a member there can hide a non-member.
:::

:::{rule} OVL-8
:kind: required
:tests: foundations/overload-resolution/enum-operand-parameter.cpp foundations/overload-resolution/enum-operand-no-parameter.cpp

If no operand has class type, a non-member operator function is a
candidate only if one of its parameters has the type of the matching
enumeration operand. A reference to that type also counts.
:::

:::{rule} OVL-9
:kind: required
:tests: foundations/overload-resolution/builtin-arithmetic.cpp foundations/overload-resolution/builtin-pointer.cpp foundations/overload-resolution/builtin-assignment.cpp foundations/overload-resolution/builtin-enumeration.cpp foundations/overload-resolution/builtin-enumeration-no-arithmetic.cpp foundations/overload-resolution/builtin-three-way.cpp foundations/overload-resolution/builtin-integer-only.cpp foundations/overload-resolution/builtin-bool-increment.cpp foundations/overload-resolution/builtin-member-pointer-equality.cpp foundations/overload-resolution/builtin-member-pointer-order.cpp

An operator has the {term}`built-in candidates <built-in candidate>` that
the table of OVL-P4 lists. Each candidate is a function with the
parameter types of its row.
:::

:::{rule} OVL-10
:kind: required
:tests: foundations/overload-resolution/builtin-same-shape-removed.cpp foundations/overload-resolution/builtin-other-shape-kept.cpp foundations/overload-resolution/builtin-removed-by-rewritten.cpp

A built-in candidate is removed if a non-member candidate that is not a
template has the same parameter types. A rewritten non-member candidate
also removes the built-in candidate ({question}`Q-0147`).
:::

:::{rule} OVL-11
:kind: reject
:tests: foundations/overload-resolution/builtin-chosen-then-checked.cpp foundations/overload-resolution/builtin-chosen-other-operand.cpp

If a built-in candidate wins, each class operand converts by the
conversion function of its sequence. The standard conversions after the
conversion function do not apply. The built-in operator then applies to
the operands, and the program is not valid if the operator's own rules
reject them.
:::

:::{rule} OVL-12
:kind: required
:tests: foundations/overload-resolution/builtin-none-for-address-comma.cpp foundations/overload-resolution/address-of-deleted-chosen.cpp

The comma operator, unary `&`, and `->` have no built-in candidates. If
none of their candidates is viable, the operator has its built-in
meaning.
:::

:::{rule} OVL-13
:kind: required
:tests: foundations/overload-resolution/rewritten-three-way-result.cpp foundations/overload-resolution/rewritten-not-equal-result.cpp foundations/overload-resolution/rewritten-swapped-member-object.cpp foundations/overload-resolution/rewritten-swapped-nonmember-converts.cpp

A rewritten candidate of {rule}`DCMP-6` evaluates as this table gives:

| Expression | Candidate | Evaluates |
|---|---|---|
| `x < y` and the other relational operators | `x <=> y` | `(x <=> y) < 0` |
| | swapped `y <=> x` | `0 < (y <=> x)` |
| `x != y` | `x == y`, or swapped `y == x` | `!(x == y)`, `!(y == x)` |
| `x == y` | swapped `y == x` | `y == x` |

A swapped candidate from a member function takes the right operand as
its object, which no user-defined conversion can reach.
:::

### Initialization

:::{rule} OVL-14
:kind: required
:tests: foundations/overload-resolution/init-constructor-against-conversion.cpp foundations/overload-resolution/init-constructor-conversion-tie.cpp foundations/overload-resolution/init-conversion-to-derived.cpp foundations/overload-resolution/init-qualified-result.cpp foundations/overload-resolution/init-direct-explicit-conversion.cpp foundations/overload-resolution/init-copy-explicit-conversion.cpp foundations/overload-resolution/init-copy-list-explicit-conversion.cpp

The initialization of an object of class `T` from one initializer `v`
has these candidates:

| Form | Candidates |
|---|---|
| direct, default, and value initialization | every constructor of `T` |
| copy initialization, `v` of class `T` or derived from `T` | the converting constructors of `T` ({rule}`CTOR-9`) |
| copy initialization, `v` of another type | the converting constructors of `T`, and the conversion functions of `v` to `T` or a class derived from `T` that are not `explicit` |
| list initialization | {rule}`ILC-3` |

The argument is `v`. A conversion function that returns a cv-qualified
`T`, or a reference to `T`, counts as a conversion to `T`. In direct
initialization, an `explicit` conversion function can make the temporary
for the parameter of a copy or move constructor. In
copy-list-initialization, the `explicit` conversion function cannot
make that temporary ({question}`Q-0145`).
:::

:::{rule} OVL-15
:kind: required
:tests: foundations/overload-resolution/reference-conversion-kind.cpp

When a reference binds directly to the result of a conversion function
({rule}`ICS-4`), the reference decides the candidates:

- An lvalue reference takes the conversion functions that return an
  lvalue reference.
- An rvalue reference takes the conversion functions that return a
  value or an rvalue reference.
- A reference to function takes both kinds.
:::

:::{rule} OVL-16
:kind: required
:tests: foundations/overload-resolution/explicit-condition-after-deduction.cpp

If the `explicit` condition of a constructor or conversion function
template depends on a template parameter, deduction comes first
*(templates referenced)*. A context that takes only functions that
are not `explicit` then removes an `explicit` specialization.
:::

:::{rule} OVL-17
:kind: required
:tests: foundations/overload-resolution/inherited-copy-shape-excluded.cpp

Some inherited constructors are removed when an object of the derived
class `D` is initialized from one argument of type `D`. A constructor
inherited from a base `B` is removed if its first parameter is a
reference to `B`, `D`, or a class between them.
:::

### The address of an overload set

:::{rule} OVL-18
:kind: required
:tests: foundations/overload-resolution/address-targets.cpp foundations/overload-resolution/address-exact-type.cpp foundations/overload-resolution/address-no-conversion.cpp foundations/overload-resolution/address-noexcept-target.cpp foundations/overload-resolution/address-member-kinds.cpp

The name of an overload set, with or without `&`, takes its function
from a target type. These contexts give a target type:

- an initialized object or reference,
- the left operand of an assignment,
- a parameter of a function or of an operator function,
- a `return` statement,
- an explicit cast,
- a template parameter that is not a type.

A function matches if its type is the target's function type. The
function can be `noexcept` when the target is not. Members with an
implicit object parameter match only pointers to member
({rule}`MPTR-2`).
:::

:::{rule} OVL-19
:kind: required
:tests: foundations/overload-resolution/address-plain-over-template.cpp foundations/overload-resolution/address-more-specialized.cpp foundations/overload-resolution/address-more-constrained.cpp

The matching functions of OVL-18 are reduced in these steps:

1. Each function template adds the specialization that deduction from
   the target gives *(templates referenced)*.
2. Functions whose constraints are not satisfied are removed
   *(concepts referenced)*.
3. If a plain function remains, the specializations are removed.
4. A plain function is removed if another plain function is more
   constrained.
5. A specialization is removed if another comes from a more specialized
   template.
:::

:::{rule} OVL-20
:kind: reject
:tests: foundations/overload-resolution/address-ambiguous.cpp foundations/overload-resolution/address-no-target-template.cpp foundations/overload-resolution/address-no-target-single.cpp foundations/overload-resolution/address-no-conversion.cpp foundations/overload-resolution/address-noexcept-target.cpp

After OVL-19, exactly one function must remain. Without a target type,
for example with `auto` or `decltype`, the overload set must have
exactly one function, which is not a template.
:::

:::{rule} OVL-21
:kind: required
:tests: foundations/overload-resolution/call-through-address.cpp foundations/overload-resolution/call-through-address-static-member.cpp foundations/overload-resolution/call-through-address-nonstatic.cpp foundations/overload-resolution/overload-set-argument.cpp foundations/overload-resolution/overload-set-argument-ambiguous.cpp

A call `(&f)(a)` or `(f)(a)` resolves `f` as a call by name, with no
implied object argument. If an argument is itself an overload set, each
of its functions is tried with each candidate, and one pairing must win.
:::

### Viable functions

:::{rule} OVL-22
:kind: required
:tests: foundations/overload-resolution/viable-count.cpp

A candidate is a {term}`viable function` only if its parameters fit the
arguments ({rule}`PFX-5`):

- one parameter for each argument,
- fewer parameters, and an ellipsis for the other arguments,
- more parameters, each of the others with a default argument,
- for a template with a parameter pack, the parameters that deduction
  gives *(templates referenced)*.
:::

:::{rule} OVL-23
:kind: required
:tests: foundations/overload-resolution/viable-sequence.cpp

A viable function has an implicit conversion sequence ({rule}`ICS-1`)
from each argument to its parameter. The implied object argument
counts, with the object parameter as its parameter. A reference that
cannot bind has no sequence ({rule}`ICS-12`).
:::

:::{rule} OVL-24
:kind: required
:tests: foundations/overload-resolution/viable-constraints.cpp

A candidate whose constraints are not satisfied is not viable
*(concepts referenced)*.
:::

### The best viable function

:::{rule} OVL-25
:kind: required
:tests: foundations/overload-resolution/better-one-argument.cpp foundations/overload-resolution/better-crossing-arguments.cpp foundations/overload-resolution/defaults-take-no-part.cpp foundations/overload-resolution/static-slot-no-part.cpp foundations/overload-resolution/template-exact-over-plain.cpp

A viable function `F1` is better than `F2` if no sequence of `F1` is
worse than the sequence of `F2` for the same argument
({rule}`ICS-21`), and one of these holds:

- Some sequence of `F1` is better.
- Every sequence ties, and a tie-breaker of OVL-26 to OVL-33 prefers
  `F1`.

Parameters that take default arguments, and the object slot of a static
member ({rule}`ICS-14`), take no part.
:::

:::{rule} OVL-26
:kind: required
:tests: foundations/overload-resolution/tie-conversion-result.cpp foundations/overload-resolution/tie-conversion-result-before-template.cpp foundations/overload-resolution/tie-conversion-result-before-constraints.cpp

In an initialization by a conversion function, the function whose result
has the better sequence to the target wins ({rule}`ICS-37`, with
{rule}`ICS-42` to {rule}`ICS-44` for results from different sources). No other
tie-breaker comes before this one.
:::

:::{rule} OVL-27
:kind: required
:tests: foundations/overload-resolution/tie-plain-over-template.cpp foundations/overload-resolution/template-exact-over-plain.cpp foundations/overload-resolution/tie-conversion-result-before-template.cpp foundations/overload-resolution/tie-template-before-inherited.cpp foundations/overload-resolution/tie-template-before-rewritten.cpp foundations/overload-resolution/tie-template-before-swapped.cpp foundations/overload-resolution/tie-template-before-reference-kind.cpp

A function that is not a template specialization wins over a
specialization *(templates referenced)*. The rule comes after OVL-26.
:::

:::{rule} OVL-28
:kind: required
:tests: foundations/overload-resolution/tie-more-specialized.cpp foundations/overload-resolution/tie-more-specialized-constraints.cpp foundations/overload-resolution/tie-constraints-different-parameters.cpp

Of two specializations, the one from the more specialized template wins
*(templates referenced)*. Of two templates with the same function
parameters, the more constrained one is more specialized
*(concepts referenced)*. OVL-27 and this rule never decide the same
case.
:::

:::{rule} OVL-29
:kind: required
:tests: foundations/overload-resolution/tie-function-reference-kind.cpp foundations/overload-resolution/tie-template-before-reference-kind.cpp foundations/overload-resolution/tie-reference-kind-before-constraints.cpp

If a reference to function binds directly to the result of either of
two conversion functions, the one that returns the same kind of
reference wins. An lvalue reference prefers an lvalue reference result.
The rule comes after OVL-27 ({question}`Q-0148`).
:::

:::{rule} OVL-30
:kind: required
:tests: foundations/overload-resolution/tie-more-constrained.cpp foundations/overload-resolution/tie-conversion-result-before-constraints.cpp foundations/overload-resolution/tie-reference-kind-before-constraints.cpp

Of two functions that are not specializations and have the same
parameter types, the more constrained one wins *(concepts referenced)*.
The rule comes after OVL-26 and OVL-29.
:::

:::{rule} OVL-31
:kind: required
:tests: foundations/overload-resolution/tie-derived-over-inherited.cpp foundations/overload-resolution/tie-template-before-inherited.cpp

A constructor of a derived class wins over a constructor inherited from
a base, if both have the same types for the parameters that the call
uses. The rule comes after OVL-27.
:::

:::{rule} OVL-32
:kind: required
:tests: foundations/overload-resolution/tie-written-over-rewritten.cpp foundations/overload-resolution/tie-template-before-rewritten.cpp foundations/overload-resolution/tie-template-before-swapped.cpp

A candidate that is not rewritten wins over a rewritten candidate
({rule}`DCMP-6`). A swapped candidate counts as rewritten. The rule
comes after OVL-27.
:::

:::{rule} OVL-33
:kind: required
:tests: foundations/overload-resolution/tie-written-order.cpp foundations/overload-resolution/tie-swapped-same-function.cpp

Of two rewritten candidates, the one that keeps the written order of
the operands wins over a swapped one. No test observes the place of
this rule before OVL-32 or after OVL-27.
:::

:::{rule} OVL-34
:kind: required
:tests: foundations/overload-resolution/best-beats-each.cpp foundations/overload-resolution/best-no-single-winner.cpp

The {term}`best viable function` is the viable function that is better
than each other viable function. The result does not depend on the
order of the declarations.
:::

### No result, and checks on the result

:::{rule} OVL-35
:kind: reject
:tests: foundations/overload-resolution/no-viable-function.cpp foundations/overload-resolution/current-object-explicit-extra-argument.cpp foundations/overload-resolution/operator-call-by-name-hidden.cpp foundations/overload-resolution/init-copy-explicit-conversion.cpp foundations/overload-resolution/init-copy-list-explicit-conversion.cpp foundations/overload-resolution/rewritten-swapped-member-object.cpp

If a use has no viable function, the program is not valid. These rules
name specific cases (decision 0042): {rule}`PFX-5`, {rule}`OPR-15`, and
{rule}`CTOR-25`.
:::

:::{rule} OVL-36
:kind: reject
:tests: foundations/overload-resolution/best-no-single-winner.cpp foundations/overload-resolution/better-crossing-arguments.cpp foundations/overload-resolution/defaults-take-no-part.cpp foundations/overload-resolution/no-object-static-default-ambiguous.cpp foundations/overload-resolution/surrogate-same-parameters.cpp foundations/overload-resolution/builtin-other-shape-kept.cpp foundations/overload-resolution/init-constructor-conversion-tie.cpp foundations/overload-resolution/overload-set-argument-ambiguous.cpp foundations/overload-resolution/tie-swapped-same-function.cpp foundations/overload-resolution/tie-constraints-different-parameters.cpp

If no viable function is better than each of the others, the program
is not valid. Such a use is ambiguous. Two rules name specific cases
(decision 0042): {rule}`OPR-15` for operators, and {rule}`CTOR-25` for
constructors.
:::

:::{rule} OVL-37
:kind: reject
:tests: foundations/overload-resolution/chosen-deleted.cpp foundations/overload-resolution/chosen-not-accessible.cpp foundations/overload-resolution/address-of-deleted-chosen.cpp

The best viable function must pass these checks. A worse viable function
never takes its place. Each check has its own rule (decision 0042):

- The function is not deleted ({rule}`DEL-1`).
- The function is accessible ({rule}`ACC-3`).
- No sequence has a fault of {rule}`ICS-8`.
- A member with an implicit object parameter has an object (OVL-3).
- The call does not use default arguments from two scopes (OVL-38).
:::

:::{rule} OVL-38
:kind: reject
:tests: foundations/overload-resolution/default-from-two-scopes.cpp foundations/overload-resolution/default-from-two-scopes-passed.cpp foundations/overload-resolution/default-from-two-scopes-same.cpp

The best viable function can have declarations in two scopes that give
different default arguments for one parameter. A call that uses the
default argument is not valid ({question}`Q-0140`). A call that passes
the argument is valid.
:::

## Examples

```cpp
void show(int);                  // 1
void show(long);                 // 2
template <class T> void show(T); // 3
show(1);                         // 1: plain over template
show(1L);                        // 2
show('c');                       // 3: exact match beats a promotion

struct Meter {
  operator double() const;
  bool operator<(const Meter&) const;
};
Meter a, b;
a < b;                           // the member
a * 2;                           // built-in double * int

void put(int) = delete;
void put(long);
put(1);                          // rejected: the deleted put(int) wins

void tick(int, long);
void tick(long, int);
tick(1, 1);                      // rejected: each is better on one argument
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 89
of 100 tests.

- OVL-27, OVL-29: when a plain conversion function and a conversion
  function template both bind an rvalue function reference, Clang and
  EDG choose the plain function. GCC chooses the template, which returns
  the target's kind of reference ({question}`Q-0148`).
- OVL-29: GCC and EDG choose the conversion that returns the target's
  kind of function reference. Clang finds the binding of an rvalue
  reference ambiguous ({question}`Q-0138`).
- OVL-6, OVL-36: Clang and EDG find two surrogates that differ only in
  the qualifiers of their conversion functions ambiguous. GCC accepts
  the call ({question}`Q-0139`).
- OVL-6: GCC gives a `const` object a surrogate from a conversion
  function that is not `const`. Clang and EDG do not
  ({question}`Q-0144`).
- OVL-10: Clang and EDG remove a built-in candidate when a rewritten
  candidate has its parameter list. GCC keeps the built-in candidate
  ({question}`Q-0147`).
- OVL-11: EDG converts a class operand to the parameter type of the
  winning built-in candidate. Clang and GCC stop after the conversion
  function ({question}`Q-0141`).
- OVL-14: Clang uses an `explicit` conversion function for the copy
  constructor's parameter in copy-list-initialization. GCC and EDG do
  not ({question}`Q-0145`).
- OVL-19: Clang 22 and GCC 16 keep the more constrained function when a
  target type selects from a set. EDG, Clang 18, and GCC 14 find the set
  ambiguous ({question}`Q-0142`).
- OVL-21: Clang 22 calls a static member through the address of a set of
  members. Clang 18 and GCC reject the call, and EDG stops with an
  internal error ({question}`Q-0143`).
- OVL-28: EDG lets a constraint decide between templates with different
  function parameters. Clang and GCC find the call ambiguous
  ({question}`Q-0146`).
- OVL-38: GCC rejects a call that uses different default arguments from
  two scopes. Clang and EDG accept it ({question}`Q-0140`).
- OVL-9: for `ToDouble{} % 2`, Clang reports that the integer candidates
  are ambiguous, and GCC and EDG report no match. All reject.
- OVL-20: gap G-11 of the inventory expects that a set of one plain
  function and one template keeps the plain function when no target
  type exists. Every compiler rejects such a set with `auto`,
  `decltype`, a cast to `void`, and an expression statement. The rule
  follows the compilers.
- OVL-36: a written `==` that is not `const` ties with its own swapped
  form (inventory RK-16). Every compiler rejects the comparison in
  strict mode, and {rule}`DCMP-7` recommends a warning.
- Class template argument deduction and its deduction-guide steps
  (inventory RK-11) are left out. They belong to the templates phase of
  the roadmap.
- Inventory AF-8, a usable result of resolution, has no rule. No rule on
  this page uses the notion, and {rule}`CPY-7` states its own
  conditions.
- Other pages state these inventory items: DE-2 ({rule}`CPY-7`), CO-1
  ({rule}`OPR-5`), CO-8 ({rule}`OPR-4`), BI-3 ({rule}`COND-11`), BI-9
  ({rule}`ICS-10`), CI-5 ({rule}`CVF-5`, {rule}`ICS-5`), CI-6
  ({rule}`ILC-3`, {rule}`ILC-8`), OP-1 to OP-7 ({rule}`ICS-10`,
  {rule}`ICS-13`, {rule}`ICS-14`), and AO-8 ({rule}`MPTR-2`).

The probe grids are in `tests/foundations/overload-resolution/grids/`,
and `tools/probe_grid.py` prints these tables from them. A cell shows
`accept` or `reject` when every compiler agrees, and lists the compilers
on each side when they split. In the labels, lref stands for `&` and
rref for `&&`.

Viability (OVL-22). Each column declares one `f`, and each row calls it.
An `accept` means that the function is viable for the call:

| | none | one | one default | two | one then default | ellipsis | one then ellipsis | pack | one then pack |
|---|---|---|---|---|---|---|---|---|---|
| no argument | accept | reject | accept | reject | reject | accept | reject | accept | reject |
| one argument | reject | accept | accept | reject | accept | accept | accept | accept | accept |
| two arguments | reject | reject | reject | accept | accept | accept | accept | accept | accept |
| three arguments | reject | reject | reject | reject | reject | accept | accept | accept | accept |

Call contexts (OVL-1 to OVL-5). Each column declares one kind of
candidate, and each row calls it from one context. An `accept` means
that the context takes that kind of candidate:

| | free function | member | static member | explicit object member | call operator | surrogate |
|---|---|---|---|---|---|---|
| name at namespace scope | accept | reject | reject | reject | reject | reject |
| name in a member function | accept | accept | accept | accept | reject | reject |
| name in a static member function | accept | reject | accept | reject | reject | reject |
| qualified name | reject | reject | accept | reject | reject | reject |
| dot call | reject | accept | accept | accept | reject | reject |
| arrow call | reject | accept | accept | accept | reject | reject |
| call on the object | reject | reject | reject | reject | accept | accept |

Calls on a class object (OVL-5, OVL-6, OVL-36). Each column declares two
members of `S`. The first returns `One` and the second `Two`. The row
"X → first" checks that the call `X(1)` chooses the first member, and
"X → second" checks the second. A `reject` in both rows means that the
call is ambiguous or that no candidate is viable:

| | lref call / const lref call | lref call / rref call | lref call / long call | lref call / surrogate | lref call / const surrogate | lref call / reference surrogate | const lref call / rref call | const lref call / long call | const lref call / surrogate | const lref call / const surrogate | const lref call / reference surrogate | rref call / long call | rref call / surrogate | rref call / const surrogate | rref call / reference surrogate | long call / surrogate | long call / const surrogate | long call / reference surrogate | surrogate / const surrogate | surrogate / reference surrogate | const surrogate / reference surrogate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| S lvalue → first | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| S lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| const S lvalue → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | reject | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject | reject | reject | reject | reject |
| const S lvalue → second | accept | reject | accept | reject | accept | accept | reject | reject | reject | reject | reject | accept | reject | accept | accept | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject |
| S prvalue → first | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| S prvalue → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |

Operators (OVL-7, OVL-9, OVL-32, OVL-33). Each column gives `X` two kinds
of candidate for `x < 1`. Each kind returns its own type, so each row
checks that one kind wins. "rewritten" is a member `operator<=>`, and
"swapped" is a non-member `operator<=>(int, const X&)`:

| | member / non-member | member / built-in | member / rewritten | member / swapped | non-member / built-in | non-member / rewritten | non-member / swapped | built-in / rewritten | built-in / swapped | rewritten / swapped |
|---|---|---|---|---|---|---|---|---|---|---|
| x < 1 → member | reject | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| x < 1 → non-member | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| x < 1 → built-in | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| x < 1 → rewritten | reject | reject | reject | reject | reject | reject | reject | accept | reject | accept |
| x < 1 → swapped | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject |

Initialization (OVL-14). Each column declares how `Src` converts to
`T`, and each row initializes a `T` from the lvalue `s` in one form:

| | converting constructor | explicit constructor | conversion function | explicit conversion function | conversion to derived | constructor and conversion function | explicit constructor and conversion function |
|---|---|---|---|---|---|---|---|
| copy init | accept | reject | accept | reject | accept | reject | accept |
| direct init | accept | accept | accept | accept | accept | accept | accept |
| direct list init | accept | accept | accept | accept | accept | accept | accept |
| copy list init | accept | reject | accept | accept: clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-14, gcc-16 | accept | accept | reject |
| functional cast | accept | accept | accept | accept | accept | accept | accept |
| static cast | accept | accept | accept | accept | accept | accept | accept |
| argument | accept | reject | accept | reject | accept | reject | accept |
| return | accept | reject | accept | reject | accept | reject | accept |

The address of an overload set (OVL-18 to OVL-20). Each column declares
an overload set `f`, and each row takes its address for one target:

| | one plain | two plain | noexcept plain | plain and template | two templates |
|---|---|---|---|---|---|
| pointer to One(int) | accept | accept | accept | accept | accept |
| noexcept pointer to One(int) | reject | reject | accept | reject | reject |
| reference to One(int) | accept | accept | accept | accept | accept |
| pointer to Two(long) | reject | accept | accept | accept | reject |
| pointer to Two(int pointer) | reject | reject | reject | accept | accept |
| pointer to Two(double) | reject | reject | reject | accept | reject |
| cast to pointer to One(int) | accept | accept | accept | accept | accept |
| no target | accept | reject | reject | reject | reject |

Plain functions and templates (OVL-25, OVL-27, OVL-28). Each column
declares two functions `f`, in two namespaces that a using-declaration
merges. The first returns `One` and the second `Two`:

| | plain int / plain const int lref | plain int / plain long | plain int / template value | plain int / template const lref | plain int / template pointer | plain int / template forwarding | plain int / constrained template | plain int / ellipsis | plain const int lref / plain long | plain const int lref / template value | plain const int lref / template const lref | plain const int lref / template pointer | plain const int lref / template forwarding | plain const int lref / constrained template | plain const int lref / ellipsis | plain long / template value | plain long / template const lref | plain long / template pointer | plain long / template forwarding | plain long / constrained template | plain long / ellipsis | template value / template const lref | template value / template pointer | template value / template forwarding | template value / constrained template | template value / ellipsis | template const lref / template pointer | template const lref / template forwarding | template const lref / constrained template | template const lref / ellipsis | template pointer / template forwarding | template pointer / constrained template | template pointer / ellipsis | template forwarding / constrained template | template forwarding / ellipsis | constrained template / ellipsis |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int lvalue → first | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | accept | accept |
| int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept | reject | accept | accept | reject | reject | reject | reject | accept | reject | reject | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | reject |
| int prvalue → first | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | accept | accept |
| int prvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept | reject | accept | accept | reject | reject | reject | reject | accept | reject | reject | accept | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, clang-host, gcc-16, clang-18, clang-22 | reject | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-14, gcc-host, gcc-16, clang-host, clang-18, clang-22 | reject | reject |
| long → first | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | accept | accept |
| long → second | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, clang-host, gcc-16, clang-22, clang-18 | reject | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-16, clang-host, gcc-14, clang-18, clang-22 | reject | reject |
| int pointer → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | accept | accept | accept | accept | reject | accept | accept |
| int pointer → second | reject | reject | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | reject | accept | reject | accept | accept | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-14, gcc-host, gcc-16, clang-22, clang-18, clang-host | reject | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-14, gcc-host, gcc-16, clang-22, clang-18, clang-host | reject | reject |

The source of each rule:

| Rule | Source |
|---|---|
| OVL-1 | probes; inventory CC-6, CC-11 |
| OVL-2, OVL-3 | grid contexts; diagnostics for a call without an object; inventory CC-3 to CC-5, gap G-1 |
| OVL-4 | grid contexts; inventory CC-1, CC-7 |
| OVL-5, OVL-6 | grids contexts, object-call; edg-overload (the surrogate candidates); inventory CC-2, CC-8 to CC-10, AF-7, gap G-10 |
| OVL-7 | grid operators; inventory CO-1, CO-4 to CO-7, CO-14 |
| OVL-8 | probes; inventory CO-2, CO-3 |
| OVL-9 | probes; inventory BI-1 to BI-4, BI-6 to BI-8, BI-10 to BI-12, BI-14, BI-15, BI-17, BI-18 |
| OVL-10 | probes; edg-overload (the removal of a built-in candidate with the signature of a function); inventory BI-5, gap G-9 |
| OVL-11 | diagnostics; inventory BI-16 |
| OVL-12 | probes; inventory BI-13, gap G-13 |
| OVL-13 | probes; inventory CO-9, CO-10, CO-12, CO-13 |
| OVL-14 | grid initialization; inventory CI-1, CI-2, CI-4, CI-8, CI-9 |
| OVL-15 | probes; inventory CI-7, CI-10 |
| OVL-16 | probe; inventory CI-3, gap G-12 |
| OVL-17 | probe; inventory DE-3, gap G-7 |
| OVL-18 to OVL-20 | grid address; diagnostics; inventory AO-2 to AO-6, AO-9 to AO-12, gap G-11 |
| OVL-21 | probes; inventory AO-1, AO-7, gap G-2 |
| OVL-22 to OVL-24 | grid viability; inventory VI-1 to VI-9 |
| OVL-25 | grid templates; edg-overload (the argument loop of the comparison); inventory RK-1, RK-2, RK-12 to RK-14, RK-17, AF-3 |
| OVL-26 to OVL-33 | grids templates, operators; edg-overload (the tie-breakers after the argument comparison); inventory RK-3 to RK-10, RK-16, gaps G-4 to G-6, G-15 |
| OVL-34 | probes; edg-overload (the best-match set); inventory RK-15, AF-2 |
| OVL-35, OVL-36 | diagnostics; inventory AF-2, AF-5, AF-7 |
| OVL-37 | diagnostics; inventory AF-1, AF-6, DE-1, VI-6 |
| OVL-38 | diagnostics; inventory AF-4, gap G-8 |

Inputs: edg-overload, clang-diag-sema.

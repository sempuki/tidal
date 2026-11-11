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
:rules: OVL-1 OVL-2 OVL-4 OVL-5 OVL-6 OVL-7 OVL-8 OVL-9 OVL-10 OVL-11 OVL-12 OVL-13 OVL-14 OVL-15 OVL-16 OVL-17 OVL-18 OVL-19 OVL-20 OVL-21 OVL-39 OVL-40 OVL-42 OVL-43 OVL-44

The context decides the candidates and the arguments. A member function
gets one more argument, the {term}`implied object argument`, which meets
its object parameter ({rule}`ICS-13`):

| Context | Candidates | Implied object argument | Rules |
|---|---|---|---|
| `f(a)`, `N::f(a)`, `(f)(a)` | the functions that lookup finds | `*this`, if lookup finds members of its class or of a base | OVL-1 to OVL-3, OVL-21 |
| `x.f(a)`, `p->f(a)` | the members that member lookup finds | `x`, or `*p` | OVL-4 |
| `c(a)`, for a class object `c` | call operators and {term}`surrogate candidates <surrogate candidate>` | `c` | OVL-5, OVL-6 |
| `x @ y`, `@x` | members, non-members, {term}`built-in candidates <built-in candidate>`, {term}`rewritten candidates <rewritten candidate>` | the left operand, for a member | OVL-7 to OVL-13, OVL-43 |
| initialization of a class | constructors, and conversion functions of the initializer | the initializer, for a conversion function | OVL-14, OVL-16, OVL-17, OVL-40, {rule}`ILC-3` |
| initialization of a reference or a non-class object from a class | conversion functions of the initializer | the initializer | OVL-15, {rule}`ICS-4` |
| `&f` and `f` with a target type | the functions that `f` names | none | OVL-18 to OVL-20 |
| `(&f)(a)` | the functions that `f` names | none | OVL-21, OVL-44 |
| an argument that names an {term}`overload set` | the candidates of the call, each of which selects one function of the set | | OVL-39, {rule}`ICS-47` |

Each {term}`function template` that lookup finds adds at most one
{term}`specialization`, by {term}`deduction` ({rule}`DED-1`). Two kinds of function are
never candidates:

- an inherited constructor that has the shape of a copy or move
  constructor, in a call with one argument (OVL-17),
- a defaulted move constructor or move assignment operator that is
  deleted (OVL-42).

An `explicit` conversion function serves only direct initialization
({principle}`CVF-P2`). In a direct initialization of a class, the
function can make the temporary for the parameter of a copy or move
constructor (OVL-40). Copy-list-initialization is a form of copy
initialization, so the function cannot make that temporary there.
:::

:::{principle} OVL-P3
:rules: OVL-25 OVL-26 OVL-27 OVL-28 OVL-29 OVL-30 OVL-31 OVL-32 OVL-33 OVL-34 OVL-41

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
   a. result      the result of a conversion function or a         OVL-26
                  constructor converts better to the target
   b. reference   the conversion function returns the kind of      OVL-29
                  function reference that the target is
   c. template    a plain function or a built-in candidate         OVL-27
                  beats a template specialization
   d. template    the more specialized template wins               OVL-28
   e. template    of two templates that meet the conditions of     OVL-41
                  ORD-13, the more constrained one wins
   f. concepts    of two plain functions that meet the conditions  OVL-30
                  of ORD-13, the more constrained one wins
   g. inherited   a constructor of the derived class beats an      OVL-31
                  inherited constructor
   h. rewritten   a written candidate beats a rewritten one        OVL-32
   i. swapped     a rewritten candidate in the written order       OVL-33
                  beats a swapped one
   |
   v
3. no step decides: neither function is better
```

The steps look first at what the call does to the values, then at how
the functions are declared, and last at how the expression was written.
The best function must be better than each other viable function
(OVL-34).

Tests observe these orders:

| Earlier | Later | Test |
|---|---|---|
| 2a | 2c | `tie-conversion-result-before-template.cpp` |
| 2a | 2f | `tie-conversion-result-before-constraints.cpp` |
| 2b | 2c, 2d | `tie-reference-kind-before-template.cpp`, `tie-reference-kind-before-more-specialized.cpp` ({question}`Q-0148`) |
| 2b | 2f | `tie-reference-kind-before-constraints.cpp` |
| 2c | 2g, 2h, 2i | `tie-template-before-inherited.cpp`, `tie-template-before-rewritten.cpp`, `tie-template-before-swapped.cpp` |
| 2d | 2e | 2e applies only when 2d does not decide |
| 2d | 2h | `templates/ordering/reversed-candidate.cpp` ({question}`Q-0211`) |

These pairs never decide the same case, so their order is not
observable:

- 2c, 2d, 2e, and 2f compare a plain function with a template, two
  templates, two templates, and two plain functions. 2d and 2e
  together order two templates.
- 2a applies when one of the two functions is a conversion function,
  2b to two conversion functions, 2g to two constructors, and 2h and 2i
  to operators.
- 2a looks at results whose sequences differ, and 2b at two results
  that both bind with the identity.
- 2h compares a written candidate with a rewritten one, and 2i compares
  two rewritten candidates.

2a can separate two templates whose results differ by a function
pointer conversion (`tie-conversion-result-templates.cpp`). No test
decides the order of these pairs:

- 2a against 2d and 2e,
- 2d against 2g and 2i,
- 2e and 2f against 2g to 2i.
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
- `X`: an {term}`arithmetic type`,
- `J`: an {term}`integral type`, which is an integer type, `bool`, or a
  character type,
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
| unary `*` | `(T*)`, and `(F*)` for a function type `F` without {term}`cv-qualifiers <cv-qualifier>` | `T&`, `F&` | {doc}`/expressions/unary` |
| `!`, `&&`, `\|\|` | `(bool)`, `(bool, bool)` | `bool` | {doc}`/expressions/logical` |
| `==`, `!=`, `<`, `>`, `<=`, `>=` | `(A, A)` for every pair; `(P, P)` and `(E, E)` for one type | `bool` | {doc}`/expressions/comparison` |
| `==`, `!=` only | `(M, M)` for one member pointer type `M`; `(std::nullptr_t, std::nullptr_t)` | `bool` | {doc}`/expressions/comparison` |
| `<=>` | `(A, A)` for every pair; `(P, P)` and `(E, E)` for one type | an ordering type | {doc}`/expressions/comparison` |
| `->*` | `(C*, M C2::*)` where `C2` is `C` or a base of `C` | a reference to `M` | {doc}`/expressions/member-pointers` |
| `++`, `--` | `(VX&)` for every `X` except `bool`; `(T* V&)` | prefix: the operand; postfix: its old value | {doc}`/expressions/unary`, {doc}`/expressions/postfix` |
| `=` | `(VX&, A)` for every `X`; `(P V&, P)`, `(E V&, E)`, `(M V&, M)` | the left operand | {doc}`/expressions/assignment` |
| `*=`, `/=`, `+=`, `-=` | `(VX&, A)` for every `X`; `(T* V&, std::ptrdiff_t)` for `+=`, `-=` | the left operand | {doc}`/expressions/assignment` |
| `%=`, `&=`, `^=`, `\|=`, `<<=`, `>>=` | `(VJ&, I)` for every `J` | the left operand | {doc}`/expressions/assignment` |
| `?:` | {rule}`COND-11` | | {doc}`/expressions/conditional` |
| `,`, unary `&`, `->` | none (OVL-12) | | |

A non-member function that the program declares with the parameter
types of a built-in candidate replaces that candidate, also when the
function takes part only as a rewritten candidate (OVL-10). A reference
parameter differs from its referred type, so `operator==(const E&, const E&)`
for an enumeration `E` leaves the built-in `(E, E)` in place.

The left operand of a built-in `=` gets no user-defined conversion
({rule}`ICS-10`). The operand of a built-in `++` or `--`, and the left
operand of a built-in compound assignment such as `+=`, can come from a
conversion function that returns a reference. So for a class object `c`
that converts to `int&`, `++c` and `c += 1` use the built-in operator,
and `c = 1` has no built-in candidate. The operands of `!`, `&&`, and `||` can
use an `explicit` conversion function to `bool` (OVL-9). After a built-in candidate wins, the
operator's own page decides what the expression does (OVL-11). For
example, `<=>` between a class that converts to `char` and a class that
converts to `bool` chooses `(int, int)`. The built-in `<=>` then rejects
the `char` and the `bool`.
:::

## Constraints

### Calls

:::{rule} OVL-1
:kind: required
:tests: overloading/overload-resolution/call-by-name-candidates.cpp overloading/overload-resolution/call-same-function-found-twice.cpp overloading/overload-resolution/default-from-two-scopes-passed.cpp declarations/functions/definitions/different-parameter-types-overload.cpp

The {term}`candidate functions <candidate function>` of a call by name
are the functions that name lookup finds ({rule}`NAM-5`):

- A function that lookup finds through several declarations is one
  candidate. Examples are a C function declared in two namespaces
  ({rule}`LSP-4`) and a function that two using-declarations name.
- A member that a using-declaration brings from a base is a candidate
  unless a member of the derived class hides it ({rule}`DER-3`).
- Each function template found adds the specialization that deduction
  gives ({rule}`DED-1`). A template with a {term}`deduction failure` adds
  no candidate ({rule}`DED-35`).
:::

:::{rule} OVL-2
:kind: required
:tests: overloading/overload-resolution/current-object-argument.cpp overloading/overload-resolution/current-object-explicit-parameter.cpp overloading/overload-resolution/current-object-explicit-extra-argument.cpp overloading/overload-resolution/current-object-default-member-init.cpp overloading/overload-resolution/current-object-other-class.cpp overloading/overload-resolution/call-parenthesized-name.cpp

A call by name that finds members of a class `C` has `*this` as the
implied object argument if these conditions hold:

- `this` can be used at the call, as in a non-static member function or
  a default member initializer.
- The class of `*this` is `C` or a class derived from `C`
  ({question}`Q-0159`).

`*this` meets the {term}`implicit object parameter` ({rule}`ICS-13`), or the
explicit object parameter ({rule}`ICS-41`). Otherwise the call has no
object (OVL-3).
:::

:::{rule} OVL-3
:kind: reject
:tests: overloading/overload-resolution/no-object-nonstatic-chosen.cpp overloading/overload-resolution/no-object-static-chosen.cpp overloading/overload-resolution/no-object-static-default-ambiguous.cpp overloading/overload-resolution/no-object-ref-qualified-member.cpp overloading/overload-resolution/call-through-address-nonstatic.cpp overloading/overload-resolution/current-object-other-class.cpp

A call with no object can still have members as candidates, for example
a call by name in a static member function. If such a call chooses a
member with an implicit object parameter, the program is not valid. A
static member can be chosen.
:::

:::{rule} OVL-4
:kind: required
:tests: overloading/overload-resolution/member-call-object.cpp overloading/overload-resolution/call-parenthesized-member-access.cpp

The candidates of `x.f(a)` are the members `f` that member lookup
({rule}`NAM-37`) finds in the class of `x`. `x` is the implied object
argument, with its cv-qualifiers and {term}`value category`. `p->f(a)` is
`(*p).f(a)`.
:::

:::{rule} OVL-5
:kind: required
:tests: overloading/overload-resolution/object-call-candidates.cpp overloading/overload-resolution/object-call-surrogate-reference.cpp

The candidates of `c(a)`, for an object `c` of class type, are these:

- the function call operators of the class, with `c` as the implied
  object argument,
- the surrogate candidates of OVL-6.
:::

:::{rule} OVL-6
:kind: required
:tests: overloading/overload-resolution/surrogate-forms.cpp overloading/overload-resolution/surrogate-from-base.cpp overloading/overload-resolution/surrogate-qualifiers.cpp overloading/overload-resolution/surrogate-explicit-not-candidate.cpp overloading/overload-resolution/surrogate-same-parameters.cpp overloading/overload-resolution/surrogate-template-not-candidate.cpp overloading/overload-resolution/surrogate-ref-qualifier.cpp overloading/overload-resolution/surrogate-ref-qualifier-lvalue.cpp overloading/overload-resolution/static-call-operator-over-surrogate.cpp overloading/overload-resolution/static-call-operator-surrogate-ambiguous.cpp

A conversion function gives a {term}`surrogate candidate` for `c(a)` if
these conditions hold:

- The function converts to a pointer to function, a reference to such a
  pointer, or a reference to function.
- The function is not `explicit`, and the function is not a template.
- The function's cv-qualifiers include those of `c`.
- The function's ref-qualifier, if any, takes the value category of
  `c`. `&` takes an lvalue, `&&` an rvalue, and `const&` both.
- A declaration in the class of `c` does not hide the function.

The surrogate's first parameter is the conversion's result type, and
`c` converts to that type by the conversion function. The other parameters are
those of the function type.
:::

### Operators

:::{rule} OVL-7
:kind: required
:tests: overloading/overload-resolution/operator-sets.cpp overloading/overload-resolution/operator-lookup-skips-members.cpp overloading/overload-resolution/operator-call-by-name-hidden.cpp foundations/names/operator-lookup-block-hides.cpp foundations/names/operator-lookup-namespace-hides.cpp foundations/names/operator-lookup-skips-class.cpp overloading/overload-resolution/operator-base-class-namespace.cpp

For an operator with an operand of class or enumeration type, the
candidates are these ({rule}`OPR-6`):

- the member {term}`operator functions <operator function>` of the left operand's class,
- the non-member operator functions that lookup finds, with class
  scopes passed over ({rule}`NAM-38`), and {term}`argument-dependent lookup`,
- the built-in candidates of OVL-9,
- the rewritten candidates of {rule}`DCMP-6`.

A call by the operator function's name uses ordinary lookup instead, so
a member there can hide a non-member. If ordinary lookup finds a class
member, argument-dependent lookup does not run ({rule}`NAM-32`).
:::

:::{rule} OVL-8
:kind: required
:tests: overloading/overload-resolution/enum-operand-parameter.cpp overloading/overload-resolution/enum-operand-no-parameter.cpp overloading/overload-resolution/enum-operand-swapped.cpp

If no operand has class type, a non-member operator function is a
candidate only if one of its parameters has the type of the matching
enumeration operand. A reference to that type also counts. For a
swapped candidate of {rule}`DCMP-6`, the first parameter matches the
right operand, and the second parameter matches the left operand.
:::

:::{rule} OVL-9
:kind: required
:tests: overloading/overload-resolution/builtin-arithmetic.cpp overloading/overload-resolution/builtin-pointer.cpp overloading/overload-resolution/builtin-assignment.cpp overloading/overload-resolution/builtin-enumeration.cpp overloading/overload-resolution/builtin-enumeration-no-arithmetic.cpp overloading/overload-resolution/builtin-three-way.cpp overloading/overload-resolution/builtin-integer-only.cpp overloading/overload-resolution/builtin-bool-increment.cpp overloading/overload-resolution/builtin-member-pointer-equality.cpp overloading/overload-resolution/builtin-member-pointer-order.cpp overloading/overload-resolution/builtin-integral-compound.cpp overloading/overload-resolution/builtin-three-way-mixed.cpp overloading/overload-resolution/builtin-three-way-bool-operand.cpp overloading/overload-resolution/builtin-not-callable-by-name.cpp overloading/overload-resolution/builtin-over-template.cpp overloading/overload-resolution/builtin-logical-explicit-bool.cpp overloading/overload-resolution/builtin-logical-explicit-competes.cpp overloading/overload-resolution/builtin-logical-explicit-competes-and.cpp overloading/overload-resolution/builtin-logical-explicit-int.cpp overloading/overload-resolution/builtin-increment-from-conversion.cpp overloading/overload-resolution/builtin-increment-from-value.cpp

An operator has the {term}`built-in candidates <built-in candidate>` that
the table of OVL-P4 lists, each a function with the parameter types of
its row. A call by the operator function's name, such as
`operator+(1, 2)`, has no built-in candidates. An operand of a built-in
`!`, `&&`, or `||` converts to `bool` as in direct initialization, so
{rule}`ICS-45` adds an `explicit` conversion function to `bool`.
:::

:::{rule} OVL-10
:kind: required
:tests: overloading/overload-resolution/builtin-same-shape-removed.cpp overloading/overload-resolution/builtin-other-shape-kept.cpp overloading/overload-resolution/builtin-removed-by-rewritten.cpp

A built-in candidate is removed if a non-member candidate that is not a
template has the same parameter types. A reference parameter differs
from its referred type, so `operator==(const E&, const E&)` keeps the
built-in `==` for an enumeration `E`. The kept built-in candidate and
that function then tie ({rule}`ICS-11`), so `a == b` for two
enumerators of `E` is ambiguous (OVL-36). A rewritten non-member candidate
also removes the built-in candidate ({question}`Q-0147`).
:::

:::{rule} OVL-11
:kind: reject
:tests: overloading/overload-resolution/builtin-chosen-then-checked.cpp overloading/overload-resolution/builtin-chosen-other-operand.cpp overloading/overload-resolution/builtin-three-way-bool-operand.cpp expressions/conditional/two-classes-int-references-prvalue.cpp expressions/conditional/two-classes-int-references-value.cpp

If a built-in candidate wins, each class operand converts by the
conversion function of its sequence. The standard conversions after the
conversion function do not apply, and an operand that is not of class
type keeps its type. The built-in operator then applies to
the operands, and the program is not valid if the operator's own rules
reject them. For `?:`, the result is the prvalue that {rule}`COND-11`
gives, also when the conversion functions return references.
:::

:::{rule} OVL-12
:kind: required
:tests: overloading/overload-resolution/builtin-none-for-address-comma.cpp overloading/overload-resolution/address-of-deleted-chosen.cpp

The comma operator, unary `&`, and `->` have no built-in candidates. If
none of their candidates is viable, the operator has its built-in
meaning.
:::

:::{rule} OVL-13
:kind: required
:tests: overloading/overload-resolution/rewritten-three-way-result.cpp overloading/overload-resolution/rewritten-not-equal-result.cpp overloading/overload-resolution/rewritten-swapped-member-object.cpp overloading/overload-resolution/rewritten-swapped-nonmember-converts.cpp overloading/overload-resolution/rewritten-three-way-reversed.cpp classes/operators/relational-chain-class.cpp

A rewritten candidate of {rule}`DCMP-6` evaluates as this table gives:

| Expression | Candidate | Evaluates |
|---|---|---|
| `x < y` and the other relational operators | `x <=> y` | `(x <=> y) < 0` |
| | swapped `y <=> x` | `0 < (y <=> x)` |
| `x <=> y` | swapped `y <=> x` | `0 <=> (y <=> x)` |
| `x != y` | `x == y`, or swapped `y == x` | `!(x == y)`, `!(y == x)` |
| `x == y` | swapped `y == x` | `y == x` |

A swapped candidate from a member function takes the right operand as
its object, which no user-defined conversion can reach.
:::

:::{rule} OVL-43
:kind: reject
:tests: overloading/overload-resolution/rewritten-inner-no-rewrite.cpp overloading/overload-resolution/rewritten-inner-reversed-only.cpp

In a rewritten candidate, the comparison of the `<=>` result with `0`
has no rewritten candidates. If that comparison has no viable function,
the program is not valid.
:::

### Initialization

:::{rule} OVL-14
:kind: required
:tests: overloading/overload-resolution/init-constructor-against-conversion.cpp overloading/overload-resolution/init-constructor-conversion-tie.cpp overloading/overload-resolution/init-conversion-to-derived.cpp overloading/overload-resolution/init-qualified-result.cpp overloading/overload-resolution/init-copy-explicit-conversion.cpp overloading/overload-resolution/init-copy-constructor-standard-only.cpp classes/constructors/return-owner-converts-to-base-owner.cpp overloading/overload-resolution/direct-init-through-conversion-function.cpp classes/copy-move/assign-from-int-through-converting-constructor.cpp classes/copy-move/assign-from-lambda-through-constructor-template.cpp

The initialization of an object of class `T` from one initializer `v`
has these candidates:

| Form | Candidates |
|---|---|
| direct, default, and value initialization | every constructor of `T` |
| copy initialization, `v` of class `T` or derived from `T` | the {term}`converting constructors <converting constructor>` of `T` ({rule}`CTOR-9`) |
| copy initialization, `v` of another type | the converting constructors of `T`, and the conversion functions of `v` to `T` or a class derived from `T` that are not `explicit` |
| list initialization | {rule}`ILC-3` |

The argument is `v`. A conversion function that returns a cv-qualified
`T`, or a reference to `T`, counts as a conversion to `T`. In copy
initialization from another type, a constructor of `T` gets `v` by a
standard sequence only ({rule}`ICS-10`). So no constructor of `T`, also
the copy or move constructor, takes `v` through a conversion function.
:::

:::{rule} OVL-15
:kind: required
:tests: overloading/overload-resolution/reference-conversion-kind.cpp overloading/overload-resolution/reference-function-both-kinds.cpp overloading/overload-resolution/reference-conversion-not-compatible.cpp overloading/overload-resolution/reference-explicit-direct.cpp overloading/overload-resolution/reference-explicit-copy.cpp expressions/conditional/class-converts-to-int-reference-lvalue.cpp

When a reference binds directly to the result of a conversion function
({rule}`ICS-4`), the reference decides the candidates:

- An lvalue reference takes the conversion functions that return an
  lvalue reference.
- An rvalue reference takes the conversion functions that return a
  value or an rvalue reference.
- A reference to function takes both kinds ({question}`Q-0138`).
- The result type must be the referenced type or a class derived from
  it, apart from cv-qualifiers that the reference adds.
- Direct initialization also takes the `explicit` conversion functions
  that {rule}`ICS-45` allows.

Without such a candidate, the reference binds to a temporary.
:::

:::{rule} OVL-16
:kind: required
:tests: overloading/overload-resolution/explicit-condition-after-deduction.cpp overloading/overload-resolution/explicit-condition-conversion-function.cpp

If the `explicit` condition of a constructor or conversion function
template depends on a template parameter, deduction comes first
({rule}`DED-24`). A context that takes only functions that are not
`explicit` then removes an `explicit` specialization.
:::

:::{rule} OVL-40
:kind: required
:tests: overloading/overload-resolution/init-direct-explicit-conversion.cpp overloading/overload-resolution/init-copy-list-explicit-conversion.cpp

In direct initialization of a class `T`, an `explicit` conversion
function can make the temporary for the parameter of a copy or move
constructor of `T`. In copy-list-initialization, the `explicit`
conversion function cannot make that temporary ({question}`Q-0145`).
:::

:::{rule} OVL-17
:kind: required
:tests: overloading/overload-resolution/inherited-copy-shape-excluded.cpp overloading/overload-resolution/inherited-copy-shape-any-argument.cpp overloading/overload-resolution/inherited-copy-shape-default.cpp overloading/overload-resolution/inherited-copy-shape-two-arguments.cpp

A constructor that the derived class `D` inherits from a base `B` is
removed from the candidates of a call with exactly one argument, of any
type. The rule applies if the constructor's first parameter is a
reference to `B`, to `D`, or to a class between them.
:::

:::{rule} OVL-42
:kind: required
:tests: overloading/overload-resolution/deleted-defaulted-move-not-candidate.cpp overloading/overload-resolution/user-deleted-move-candidate.cpp

A defaulted move constructor or move assignment operator that is
deleted ({rule}`CPY-7`) is not a candidate. A move function that is
declared deleted is a candidate, and OVL-37 applies when it wins.
:::

### The address of an overload set

:::{rule} OVL-18
:kind: required
:tests: overloading/overload-resolution/address-targets.cpp overloading/overload-resolution/address-exact-type.cpp overloading/overload-resolution/address-no-conversion.cpp overloading/overload-resolution/address-noexcept-target.cpp overloading/overload-resolution/address-member-kinds.cpp overloading/overload-resolution/address-base-member.cpp overloading/overload-resolution/address-static-member-template.cpp

The name of an overload set, with or without `&`, takes its function
from a target type. These contexts give a target type:

- an initialized object or reference,
- the left operand of an assignment,
- a parameter of a function or of an operator function,
- a `return` statement,
- an explicit cast,
- a template parameter that is not a type.

A parameter gives a target type only when its type is known. A function
parameter whose type depends on a template parameter gets its type from
deduction on the other arguments ({rule}`DED-19`). A constant template
parameter declared with `auto` gives no target type. A function matches if its type is the target's function type. The
function can be `noexcept` when the target is not. Members with an
implicit object parameter match only pointers to member ({rule}`MPTR-2`),
and the class of the pointer to member takes no part ({rule}`MPTR-4`).
:::

:::{rule} OVL-19
:kind: required
:tests: overloading/overload-resolution/address-plain-over-template.cpp overloading/overload-resolution/address-more-specialized.cpp overloading/overload-resolution/address-more-constrained.cpp

The matching functions of OVL-18 are reduced in these steps:

1. Each function template adds the specialization that deduction from
   the target gives ({rule}`DED-30`).
2. Functions whose {term}`constraints <constraint>` are not satisfied ({rule}`CNS-7`) are
   removed.
3. If a plain function remains, the specializations are removed.
4. A plain function is removed if another plain function is more
   constrained ({rule}`CNS-23`) and the two meet the conditions of
   {rule}`ORD-13` ({question}`Q-0142`).
5. A specialization is removed if another comes from a {term}`more specialized`
   template ({rule}`ORD-9`).
:::

:::{rule} OVL-20
:kind: reject
:tests: overloading/overload-resolution/address-ambiguous.cpp overloading/overload-resolution/address-no-target-return.cpp overloading/overload-resolution/address-no-target-init-capture.cpp overloading/overload-resolution/address-no-target-auto-template-parameter.cpp overloading/overload-resolution/address-no-target-cast-void.cpp overloading/overload-resolution/address-no-target-expression-statement.cpp overloading/overload-resolution/address-no-target-single-positions.cpp overloading/overload-resolution/address-no-target-template.cpp overloading/overload-resolution/address-no-target-single.cpp overloading/overload-resolution/address-no-conversion.cpp overloading/overload-resolution/address-noexcept-target.cpp overloading/overload-resolution/address-no-target-explicit-arguments.cpp overloading/overload-resolution/address-no-target-explicit-partial.cpp overloading/overload-resolution/address-no-target-constraints.cpp overloading/overload-resolution/address-no-target-more-constrained.cpp overloading/overload-resolution/address-no-target-constraints-different-parameters.cpp declarations/auto/overload-set-template.cpp templates/ordering/address-explicit-arguments-no-target.cpp

After OVL-19, exactly one function must remain. Without a target type,
these steps apply:

1. A function template adds the specialization that its explicit
   {term}`template arguments <template argument>` fix, if they fix every template parameter. Any
   other template makes the program not valid.
2. Steps 2 and 4 of OVL-19 remove functions ({question}`Q-0142`).
3. Exactly one function must remain.

These contexts give no target type:

- a variable or a return type declared with `auto` or `decltype(auto)`
  ({rule}`AUTO-8`, {rule}`AUTO-11`),
- an init capture,
- the operand of `decltype` ({rule}`DTY-4`) or of `sizeof`,
- an argument for `...` ({rule}`ICS-47`),
- a constant template parameter declared with `auto`,
- a cast to `void`, and an expression statement.

{rule}`DED-35` names the case of a target type that no specialization
of a function template matches (decision 0042).
:::

:::{rule} OVL-21
:kind: required
:tests: overloading/overload-resolution/call-through-address.cpp overloading/overload-resolution/call-through-address-static-member.cpp overloading/overload-resolution/call-through-address-nonstatic.cpp overloading/overload-resolution/call-parenthesized-name.cpp overloading/overload-resolution/call-through-address-parenthesized-static.cpp overloading/overload-resolution/call-through-address-parenthesized-qualified.cpp overloading/overload-resolution/call-through-address-unqualified.cpp overloading/overload-resolution/call-through-address-unqualified-argument.cpp overloading/overload-resolution/call-parenthesized-member-access.cpp

A call `(&f)(a)` resolves `f` as a call by name, with no implied object
argument. A call `(f)(a)` is the same call as `f(a)`, and keeps the
implied object argument of OVL-2. A call `(x.f)(a)` or `(p->f)(a)` is
the call `x.f(a)` or `p->f(a)` of {rule}`OVL-4`.
:::

:::{rule} OVL-44
:kind: reject
:tests: overloading/overload-resolution/call-through-address-parenthesized-static.cpp overloading/overload-resolution/call-through-address-parenthesized-qualified.cpp

In a call `(&f)(a)`, if the set `f` holds a member with an implicit
object parameter, the operand of `&` must not be a qualified name in
parentheses ({rule}`MPTR-2`). The rule holds even when a static member
of the set would win. An unqualified name without parentheses follows
{rule}`OVL-21` ({question}`Q-0161`).
:::

:::{rule} OVL-39
:kind: required
:tests: overloading/overload-resolution/overload-set-argument.cpp overloading/overload-resolution/overload-set-argument-ambiguous.cpp overloading/overload-resolution/overload-set-argument-with-other.cpp

If an argument of a call names an overload set, each candidate selects
its own function from the set, with its parameter type as the target
type ({rule}`ICS-47`). The candidates then compare as for any other
argument, and one candidate must win (OVL-36).
:::

### Viable functions

:::{rule} OVL-22
:kind: required
:tests: overloading/overload-resolution/viable-count.cpp overloading/overload-resolution/viable-count-defaults-and-ellipsis.cpp

A candidate is a {term}`viable function` only if its parameters fit the
arguments ({rule}`PFX-5`):

- one parameter for each argument,
- fewer parameters, and an ellipsis for the other arguments,
- more parameters, each of the others with a default argument,
- for a template with a parameter pack, the parameters that deduction
  gives ({rule}`DED-20`).
:::

:::{rule} OVL-23
:kind: required
:tests: overloading/overload-resolution/viable-sequence.cpp overloading/overload-resolution/no-object-ref-qualified-member.cpp overloading/overload-resolution/const-ref-parameter-through-converting-constructor.cpp

A viable function has an {term}`implicit conversion sequence` ({rule}`ICS-1`)
from each argument to its parameter, and a reference that cannot bind
has no sequence ({rule}`ICS-12`). The implied object argument counts,
with the object parameter as its parameter. In a call with no object
(OVL-3), the object parameter of a member takes no part, whatever its
cv-qualifiers and ref-qualifier ({question}`Q-0187`).
:::

:::{rule} OVL-24
:kind: required
:tests: overloading/overload-resolution/viable-constraints.cpp overloading/overload-resolution/viable-constraints-function-template.cpp

A candidate whose constraints are not satisfied ({rule}`CNS-7`) is not
viable. {rule}`CNS-11` says when a function template's constraints are
checked.
:::

### The best viable function

:::{rule} OVL-25
:kind: required
:tests: overloading/overload-resolution/better-one-argument.cpp overloading/overload-resolution/better-crossing-arguments.cpp overloading/overload-resolution/defaults-take-no-part.cpp overloading/overload-resolution/static-slot-no-part.cpp overloading/overload-resolution/template-exact-over-plain.cpp overloading/overload-resolution/static-call-operator-over-surrogate.cpp overloading/overload-resolution/static-call-operator-surrogate-ambiguous.cpp expressions/lambdas/static-call-operator.cpp

A viable function `F1` is better than `F2` if no sequence of `F1` is
worse than the sequence of `F2` for the same argument
({rule}`ICS-21`), and one of these holds:

- Some sequence of `F1` is better.
- Every sequence ties, and a tie-breaker of OVL-26 to OVL-33 or OVL-41
  prefers `F1`.

Parameters that take default arguments and an object parameter in a
call with no object (OVL-23) take no part. The object slot of a static
member ({rule}`ICS-14`) takes no part against another function's
standard sequence. Against the first sequence of a surrogate candidate
({rule}`OVL-6`), which is user-defined, the slot's sequence is better.
:::

:::{rule} OVL-26
:kind: required
:tests: overloading/overload-resolution/tie-conversion-result.cpp overloading/overload-resolution/tie-conversion-result-before-template.cpp overloading/overload-resolution/tie-conversion-result-before-constraints.cpp overloading/overload-resolution/tie-conversion-result-templates.cpp overloading/overload-resolution/tie-conversion-result-noexcept.cpp overloading/overload-resolution/tie-conversion-result-constructor.cpp overloading/overload-resolution/init-constructor-conversion-tie.cpp

In an initialization by a user-defined conversion, the function whose
result has the better sequence to the target wins. The results compare
in these cases:

- two conversion functions ({rule}`ICS-37`, with {rule}`ICS-42` to
  {rule}`ICS-44` for results from different sources),
- a constructor of the target class, whose result has the identity, and
  a conversion function ({question}`Q-0189`).

The identity is better than a function pointer conversion
({question}`Q-0163`), and no other tie-breaker comes before this one.
:::

:::{rule} OVL-27
:kind: required
:tests: overloading/overload-resolution/tie-plain-over-template.cpp overloading/overload-resolution/template-exact-over-plain.cpp overloading/overload-resolution/tie-conversion-result-before-template.cpp overloading/overload-resolution/tie-template-before-inherited.cpp overloading/overload-resolution/tie-template-before-rewritten.cpp overloading/overload-resolution/tie-template-before-swapped.cpp overloading/overload-resolution/tie-reference-kind-before-template.cpp overloading/overload-resolution/tie-reference-kind-before-template-lvalue.cpp overloading/overload-resolution/builtin-over-template.cpp expressions/lambdas/closure-as-base-overload-set.cpp templates/deduction/losing-candidate-body-not-instantiated.cpp

A function that is not a template specialization wins over a
specialization. A built-in candidate counts as such a function. The
rule comes after OVL-26 and OVL-29.
:::

:::{rule} OVL-28
:kind: required
:tests: overloading/overload-resolution/tie-more-specialized.cpp overloading/overload-resolution/tie-reference-kind-before-more-specialized.cpp

Of two specializations, the one from the more specialized template wins
({rule}`ORD-1`, {rule}`ORD-5`).
:::

:::{rule} OVL-41
:kind: required
:tests: overloading/overload-resolution/tie-more-specialized-constraints.cpp overloading/overload-resolution/tie-constraints-different-parameters.cpp

The rule applies only if each of two templates is at least as
specialized as the other ({rule}`ORD-1`). Then the specialization from
the {term}`more constrained` template ({rule}`CNS-23`) wins if the two
templates meet the conditions of {rule}`ORD-13`. Otherwise the
constraints do not decide ({question}`Q-0146`).
:::

:::{rule} OVL-29
:kind: required
:tests: overloading/overload-resolution/tie-function-reference-kind.cpp overloading/overload-resolution/tie-reference-kind-before-template.cpp overloading/overload-resolution/tie-reference-kind-before-template-lvalue.cpp overloading/overload-resolution/tie-reference-kind-before-more-specialized.cpp overloading/overload-resolution/tie-reference-kind-before-constraints.cpp

If a reference to function binds directly to the result of either of
two conversion functions, the one that returns the same kind of
reference wins. An lvalue reference prefers an lvalue reference result.
The rule comes after OVL-26 and before OVL-27 and OVL-28
({question}`Q-0148`).
:::

:::{rule} OVL-30
:kind: required
:tests: overloading/overload-resolution/tie-more-constrained.cpp overloading/overload-resolution/tie-conversion-result-before-constraints.cpp overloading/overload-resolution/tie-reference-kind-before-constraints.cpp overloading/overload-resolution/tie-constraints-default-parameter.cpp overloading/overload-resolution/tie-constraints-ellipsis.cpp

Of two functions that are not specializations, the more constrained one
({rule}`CNS-23`) wins if the two functions meet the conditions of
{rule}`ORD-13` ({question}`Q-0146`). The rule comes after OVL-26 and
OVL-29.
:::

:::{rule} OVL-31
:kind: required
:tests: overloading/overload-resolution/tie-derived-over-inherited.cpp overloading/overload-resolution/tie-template-before-inherited.cpp

A constructor of a derived class wins over a constructor inherited from
a base, if both have the same types for the parameters that the call
uses. The rule comes after OVL-27.
:::

:::{rule} OVL-32
:kind: required
:tests: overloading/overload-resolution/tie-written-over-rewritten.cpp overloading/overload-resolution/tie-template-before-rewritten.cpp overloading/overload-resolution/tie-template-before-swapped.cpp classes/comparisons/eq-across-specializations.cpp

A candidate that is not rewritten wins over a rewritten candidate
({rule}`DCMP-6`). A swapped candidate counts as rewritten. The rule
comes after OVL-27.
:::

:::{rule} OVL-33
:kind: required
:tests: overloading/overload-resolution/tie-written-order.cpp overloading/overload-resolution/tie-swapped-same-function.cpp

Of two rewritten candidates, the one that keeps the written order of
the operands wins over a swapped one. No test observes the place of
this rule before OVL-32 or after OVL-27.
:::

:::{rule} OVL-34
:kind: required
:tests: overloading/overload-resolution/best-beats-each.cpp overloading/overload-resolution/best-no-single-winner.cpp

The {term}`best viable function` is the viable function that is better
than each other viable function. The result does not depend on the
order of the declarations.
:::

### No result, and checks on the result

:::{rule} OVL-35
:kind: reject
:tests: overloading/overload-resolution/no-viable-function.cpp overloading/overload-resolution/current-object-explicit-extra-argument.cpp overloading/overload-resolution/operator-call-by-name-hidden.cpp overloading/overload-resolution/init-copy-explicit-conversion.cpp overloading/overload-resolution/init-copy-list-explicit-conversion.cpp overloading/overload-resolution/rewritten-swapped-member-object.cpp overloading/overload-resolution/rewritten-inner-no-rewrite.cpp overloading/overload-resolution/surrogate-template-not-candidate.cpp overloading/overload-resolution/surrogate-ref-qualifier-lvalue.cpp overloading/overload-resolution/builtin-not-callable-by-name.cpp overloading/overload-resolution/reference-explicit-copy.cpp overloading/overload-resolution/inherited-copy-shape-any-argument.cpp overloading/overload-resolution/inherited-copy-shape-default.cpp overloading/overload-resolution/builtin-logical-explicit-int.cpp overloading/conversion-sequences/ellipsis-braced-list.cpp overloading/overload-resolution/builtin-increment-from-value.cpp overloading/overload-resolution/init-copy-constructor-standard-only.cpp overloading/conversion-sequences/assignment-left-no-builtin.cpp expressions/new-delete/only-aligned-class-form-ordinary-type.cpp overloading/conversion-sequences/explicit-conversion-to-derived-direct-init.cpp foundations/storage/class-allocation-function-hides-placement.cpp templates/declarations/default-argument-constrained-converting-constructor.cpp

If a use has no viable function, the program is not valid. These rules
name specific cases (decision 0042): {rule}`PFX-5`, {rule}`OPR-15`,
{rule}`CTOR-25`, OVL-43, {rule}`DED-35`, and {rule}`CTAD-24`.
:::

:::{rule} OVL-36
:kind: reject
:tests: overloading/overload-resolution/best-no-single-winner.cpp overloading/overload-resolution/better-crossing-arguments.cpp overloading/overload-resolution/defaults-take-no-part.cpp overloading/overload-resolution/no-object-static-default-ambiguous.cpp overloading/overload-resolution/surrogate-same-parameters.cpp overloading/overload-resolution/builtin-other-shape-kept.cpp overloading/overload-resolution/init-constructor-conversion-tie.cpp overloading/overload-resolution/overload-set-argument-ambiguous.cpp overloading/overload-resolution/tie-swapped-same-function.cpp overloading/overload-resolution/tie-constraints-different-parameters.cpp overloading/overload-resolution/tie-constraints-default-parameter.cpp overloading/overload-resolution/tie-constraints-ellipsis.cpp overloading/overload-resolution/builtin-logical-explicit-competes.cpp overloading/overload-resolution/builtin-logical-explicit-competes-and.cpp overloading/overload-resolution/static-call-operator-surrogate-ambiguous.cpp overloading/overload-resolution/call-through-address-unqualified.cpp overloading/conversion-sequences/overload-set-reference-or-pointer.cpp overloading/conversion-sequences/overload-set-to-ellipsis.cpp templates/concepts/type-constraint-pack-same-concept-two-declarations.cpp

If no viable function is better than each of the others, the program
is not valid. Such a use is ambiguous. These rules name specific cases
(decision 0042):

- {rule}`OPR-15` for operators,
- {rule}`CTOR-25` for constructors,
- {rule}`ORD-15` for templates that {term}`partial ordering` does not order,
- {rule}`CTAD-24` for the guides of a {term}`class template`,
- {rule}`CNS-24` for declarations that their constraints do not order.
:::

:::{rule} OVL-37
:kind: reject
:tests: overloading/overload-resolution/chosen-deleted.cpp overloading/overload-resolution/chosen-not-accessible.cpp overloading/overload-resolution/address-of-deleted-chosen.cpp overloading/overload-resolution/user-deleted-move-candidate.cpp templates/deduction/two-equal-bases-with-fallback.cpp templates/deduction/private-base-with-fallback.cpp templates/deduction/protected-base-with-fallback.cpp

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
:tests: overloading/overload-resolution/default-from-two-scopes.cpp overloading/overload-resolution/default-from-two-scopes-passed.cpp overloading/overload-resolution/default-from-two-scopes-same.cpp overloading/overload-resolution/default-from-two-scopes-same-value.cpp overloading/overload-resolution/default-from-two-scopes-same-literal.cpp overloading/overload-resolution/default-from-two-scopes-same-name-other-variable.cpp overloading/overload-resolution/default-from-two-scopes-same-variable.cpp

The best viable function can have declarations in two scopes that give
different default arguments for one parameter. A call that uses the
default argument is not valid ({question}`Q-0140`). A call that passes
the argument is valid.

Two default arguments are the same if they are the same expression.
Two expressions are the same when they apply the same operations to the
same entities and literals. The spelling does not decide:

- Parentheses around the whole argument do not count.
- Two literals with the same value are the same.
- Two names of one entity, such as `shared` and `::shared`, are the same.
- One name that each scope declares for a different entity, such as a
  variable `n` in each of two namespaces, gives different expressions.
- Two expressions with one value, such as `5` and `2 + 3`, are
  different.
:::

## Examples

% tidal-example: reject
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

struct Self {
  void take(this Self&, int);
  void use() {
    take(1);                     // *this fills the explicit object parameter
    take(*this, 1);              // rejected: one argument too many
  }
};
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 171
of 201 tests.

### Divergences

- {question}`Q-0138` (OVL-15, OVL-29; settled on OVL-P3, step 2b):
  whether the kind of a function reference breaks a tie between
  conversion functions to `F&` and `F&&`. It does, and an lvalue
  reference to a function takes either kind. Clang finds the binding
  of `F&&` ambiguous.
- {question}`Q-0139` (OVL-6, OVL-36; settled on OVL-P3): whether two
  surrogates that differ only in the qualifiers of their conversion
  functions tie. They tie, and the call is ambiguous. GCC accepts the
  call.
- {question}`Q-0140` (OVL-38; open): whether a call can use a default
  argument that two scopes give. It cannot when the two arguments
  differ, as GCC says. Clang and EDG accept the call.
- {question}`Q-0141` (OVL-11; settled on OVL-P4): whether a class
  operand converts past its conversion function to the parameter type
  of the winning built-in candidate. It does not. EDG converts it.
- {question}`Q-0142` (OVL-19, OVL-20; open): whether constraints reduce
  an overload set whose address is taken. They do, when the functions
  meet the conditions of ORD-13, as Clang 22 and GCC 16 do. EDG, and
  the older versions of Clang and GCC, differ in some forms.
- {question}`Q-0143` (OVL-21; open): whether a call through `&C::f` can
  choose a static member of a set that holds a non-static member. It
  can, as Clang 22 does. GCC rejects the call, and EDG stops with an
  internal error.
- {question}`Q-0144` (OVL-6; settled on OVL-P2): whether a `const`
  object gets a surrogate from a conversion function that is not
  `const`. It does not. GCC gives it one.
- {question}`Q-0145` (OVL-40; settled on {principle}`OVL-P2`): whether copy-list-initialization
  can use an `explicit` conversion function for the copy constructor's
  parameter. It cannot. Clang uses the function.
- {question}`Q-0146` (OVL-30, OVL-41; settled on {principle}`ORD-P2`
  and {principle}`CNS-P4`): which conditions two functions meet before
  their constraints decide between them. They are the conditions of
  ORD-13. EDG lets a constraint decide in every case, and Clang 18 does
  for an ellipsis.
- {question}`Q-0147` (OVL-10; settled on {principle}`OVL-P4`): whether a rewritten candidate
  removes the built-in candidate with its parameter list. It does. GCC
  keeps the built-in candidate.
- {question}`Q-0148` (OVL-27 to OVL-29; open): whether the kind of a
  function reference comes before the template tie-breakers. It comes
  first, as GCC says. EDG applies the template tie-breakers first, and
  Clang does not apply the kind.
- {question}`Q-0159` (OVL-2, OVL-3; open): whether `*this` is the
  object of a call by name that finds members of an enclosing class.
  It is not, and the call has no object, as GCC says. Clang and EDG
  take the current object and call a static member.
- {question}`Q-0161` (OVL-21, OVL-36, OVL-44; open): whether a call
  through `&(C::f)` or `&f` can use a set that holds a non-static
  member. `&(C::f)` cannot, and `&f` resolves the set as `&C::f` does,
  so `(&f)()` ties. GCC rejects every form, Clang 22 accepts `(&f)()`,
  and EDG accepts both forms with no argument.
- {question}`Q-0163` (OVL-26; settled on OVL-P3, step 2a): whether the
  result tie-breaker prefers the identity to a function pointer
  conversion. It does. EDG finds the results ambiguous.
- {question}`Q-0187` (OVL-3, OVL-23; open): whether a member's object
  parameter takes part in a call with no object. It takes no part. GCC
  gives the call an lvalue object.
- {question}`Q-0189` (OVL-26; open): whether the result tie-breaker
  compares a constructor with a conversion function. It does, and the
  constructor's result is the identity. Clang finds the two ambiguous.
- {question}`Q-0411` (CPT-19, CNS-4; open): whether the folds that one
  concept makes on a type parameter pack in two declarations are one
  atom or two. Two atoms, so neither declaration is more constrained,
  as Clang and GCC say. EDG treats them as one atom.

### Split tests

The table lists each test on which the compilers differ. The Expected
column gives the outcome that the rules predict.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `address-more-constrained.cpp` | OVL-19 | accept | Clang 22: accept; Clang 18: reject | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0142` |
| `address-no-target-constraints-different-parameters.cpp` | OVL-20, ORD-13 | reject | Clang 22: reject; Clang 18: accept | reject | reject | {question}`Q-0142` |
| `address-no-target-constraints.cpp` | OVL-20 | accept | accept | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0142` |
| `address-no-target-more-constrained.cpp` | OVL-20 | accept | Clang 22: accept; Clang 18: reject | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0142` |
| `builtin-chosen-other-operand.cpp` | OVL-11 | reject | reject | reject | accept | {question}`Q-0141` |
| `builtin-removed-by-rewritten.cpp` | OVL-10 | accept | accept | reject | accept | {question}`Q-0147` |
| `call-through-address-parenthesized-qualified.cpp` | OVL-44, OVL-21 | reject | reject | reject | accept | {question}`Q-0161` |
| `call-through-address-static-member.cpp` | OVL-21 | accept | Clang 22: accept; Clang 18: reject | reject | reject | {question}`Q-0143` |
| `call-through-address-unqualified-argument.cpp` | OVL-21 | accept | accept | reject | reject | {question}`Q-0161` |
| `call-through-address-unqualified.cpp` | OVL-21, OVL-36 | reject | Clang 22: accept; Clang 18: reject | reject | accept | {question}`Q-0161` |
| `current-object-other-class.cpp` | OVL-2, OVL-3 | reject | accept | reject | accept | {question}`Q-0159` |
| `default-from-two-scopes-same-name-other-variable.cpp` | OVL-38 | reject | accept | reject | accept | {question}`Q-0140` |
| `default-from-two-scopes-same-value.cpp` | OVL-38 | reject | accept | reject | accept | {question}`Q-0140` |
| `default-from-two-scopes.cpp` | OVL-38 | reject | accept | reject | accept | {question}`Q-0140` |
| `init-copy-list-explicit-conversion.cpp` | OVL-40, OVL-35 | reject | accept | reject | reject | {question}`Q-0145` |
| `no-object-ref-qualified-member.cpp` | OVL-3, OVL-23 | reject | reject | accept | reject | {question}`Q-0187` |
| `reference-function-both-kinds.cpp` | OVL-15 | accept | reject | accept | accept | {question}`Q-0138` |
| `surrogate-qualifiers.cpp` | OVL-6 | accept | accept | reject | accept | {question}`Q-0144` |
| `surrogate-same-parameters.cpp` | OVL-36, OVL-6 | reject | reject | accept | reject | {question}`Q-0139` |
| `templates/concepts/type-constraint-pack-same-concept-two-declarations.cpp` | CPT-19, CNS-4, OVL-36 | reject | reject | reject | accept | {question}`Q-0411` |
| `tie-constraints-default-parameter.cpp` | OVL-30, OVL-36 | reject | reject | reject | accept | {question}`Q-0146` |
| `tie-constraints-different-parameters.cpp` | OVL-36, OVL-41 | reject | reject | reject | accept | {question}`Q-0146` |
| `tie-constraints-ellipsis.cpp` | OVL-30, OVL-36 | reject | Clang 22: reject; Clang 18: accept | reject | accept | {question}`Q-0146` |
| `tie-conversion-result-constructor.cpp` | OVL-26 | accept | reject | accept | accept | {question}`Q-0189` |
| `tie-conversion-result-noexcept.cpp` | OVL-26 | accept | accept | accept | reject | {question}`Q-0163` |
| `tie-conversion-result-templates.cpp` | OVL-26 | accept | accept | accept | reject | {question}`Q-0163` |
| `tie-function-reference-kind.cpp` | OVL-29 | accept | reject | accept | accept | {question}`Q-0138` |
| `tie-reference-kind-before-more-specialized.cpp` | OVL-29, OVL-28 | accept | reject | accept | reject | {question}`Q-0138`, {question}`Q-0148` |
| `tie-reference-kind-before-template-lvalue.cpp` | OVL-29, OVL-27 | accept | accept | accept | reject | {question}`Q-0138`, {question}`Q-0148` |
| `tie-reference-kind-before-template.cpp` | OVL-29, OVL-27 | accept | reject | accept | reject | {question}`Q-0148` |

### Notes

- OVL-25, OVL-6: every compiler chooses a static call operator over a
  surrogate candidate with the same parameter types. Every compiler
  finds the call ambiguous when the surrogate's parameter fits the
  argument better. So the static member's object slot beats the
  surrogate's user-defined conversion of the object.
- OVL-9: every compiler gives the built-in `!`, `&&`, and `||` an
  `explicit` conversion function to `bool`. The built-in candidate then
  ties with an operator function that needs a converting constructor,
  so the expression is ambiguous, and every compiler rejects it (OVL-36).
  An `explicit` conversion function to `int` gives no sequence.
- OVL-8: every compiler takes a swapped `==` whose parameters meet the
  enumeration operands only in the swapped order.
- OVL-9: for `ToDouble{} % 2`, Clang reports that the integer candidates
  are ambiguous, and GCC and EDG report no match. All reject.
- OVL-9, OVL-11: in the three-way grid, `unsigned` against `long` is
  rejected on the ILP32 target only. There `long` cannot hold every
  `unsigned` value, and the built-in `<=>` rejects the narrowing.
- OVL-20: gap G-11 of the inventory expects that a set of one plain
  function and one template keeps the plain function when no target
  type exists. Every compiler rejects such a set with `auto`,
  `decltype`, a cast to `void`, and an expression statement. The rule
  follows the compilers.
- OVL-36: a written `==` that is not `const` ties with its own swapped
  form (inventory RK-16). Every compiler rejects the comparison in
  strict mode, and {rule}`DCMP-7` recommends a warning.
- OVL-38: two gap reviews read the reference as rejecting a call that
  uses default arguments from two scopes even when the two are equal.
  The test of when two default arguments are the same is Tidal's own,
  and it follows the compilers, which all accept equal arguments
  (`documentation/decisions/pending.md`, row 4).
- OVL-38: GCC compares the entities that two default arguments name.
  It rejects a call whose two default arguments `n` name the variables
  `n` of two namespaces, and it accepts `shared` and `::shared`, which
  name one variable. OVL-38 follows GCC. Clang and EDG accept both
  calls.

### Probe grids

The 12 probe grids of this page are in
`tests/overloading/overload-resolution/grids/`. The tables that
`tools/probe_grid.py` prints from them are in
[`grids/tables.md`](https://github.com/sempuki/tidal/blob/main/tests/overloading/overload-resolution/grids/tables.md),
with a caption for each grid that says what its rows and columns
declare. A cell shows `accept` or `reject` when every compiler agrees,
and lists the compilers on each side when they split.

### Sources

This table gives the source of each rule. A gap review item has the
number that `reports/review/2026-10-31-gap-review-overload-resolution.md`
gives it:

| Rule | Source |
|---|---|
| OVL-1 | probes; inventory CC-6, CC-11 |
| OVL-2, OVL-3 | grid contexts; diagnostics for a call without an object; inventory CC-3 to CC-5, gap G-1; gap review M-4, E-2 |
| OVL-4 | grid contexts; inventory CC-1, CC-7 |
| OVL-5, OVL-6 | grids contexts, object-call, surrogates; edg-overload (the surrogate candidates); inventory CC-2, CC-8 to CC-10, AF-7, gap G-10; gap review M-5, M-6 |
| OVL-7 | grid operators; inventory CO-1, CO-4 to CO-7, CO-14 |
| OVL-8 | probes; inventory CO-2, CO-3 |
| OVL-9 | probes; grids compound-integral, three-way; inventory BI-1 to BI-4, BI-6 to BI-8, BI-10 to BI-12, BI-14, BI-15, BI-17, BI-18; gap review M-8, E-8 |
| OVL-10 | probes; edg-overload (the removal of a built-in candidate with the signature of a function); inventory BI-5, gap G-9 |
| OVL-11 | diagnostics; inventory BI-16 |
| OVL-12 | probes; inventory BI-13, gap G-13 |
| OVL-13 | probes; inventory CO-9, CO-10, CO-12, CO-13; gap review M-1 |
| OVL-43 | diagnostics; gap review M-2 |
| OVL-14, OVL-40 | grid initialization; inventory CI-1, CI-2, CI-4, CI-8, CI-9; gap review S-2 |
| OVL-15 | probes; grid reference-kind; inventory CI-7, CI-10; gap review M-10 |
| OVL-16 | probe; inventory CI-3, gap G-12 |
| OVL-17 | probes; inventory DE-3, gap G-7; gap review E-3 |
| OVL-42 | probes; {rule}`CPY-7`; gap review S-4 |
| OVL-18 to OVL-20 | grids address, address-explicit; diagnostics; inventory AO-2 to AO-6, AO-9 to AO-12, gap G-11; gap review M-7, E-5, E-6 |
| OVL-21, OVL-44 | probes; diagnostics; inventory AO-1, AO-7, gap G-2; gap review E-1, E-7 |
| OVL-39 | probes; gap review S-1 |
| OVL-22 to OVL-24 | grid viability; inventory VI-1 to VI-9 |
| OVL-25 | grid templates; edg-overload (the argument loop of the comparison); inventory RK-1, RK-2, RK-12 to RK-14, RK-17, AF-3 |
| OVL-26 to OVL-33, OVL-41 | grids templates, operators, reference-kind; edg-overload (the tie-breakers after the argument comparison); inventory RK-3 to RK-10, RK-16, gaps G-4 to G-6, G-15; gap review M-9, O-1 to O-3, S-3 |
| OVL-34 | probes; edg-overload (the best-match set); inventory RK-15, AF-2 |
| OVL-35, OVL-36 | diagnostics; inventory AF-2, AF-5, AF-7 |
| OVL-37 | diagnostics; inventory AF-1, AF-6, DE-1, VI-6 |
| OVL-38 | diagnostics; inventory AF-4, gap G-8; gap review E-4 |

- Class template argument deduction and its deduction-guide steps
  (inventory RK-11) are on {doc}`/templates/class-template-deduction`.
- Inventory AF-8, a usable result of resolution, has no rule. No rule on
  this page uses the notion, and {rule}`CPY-7` states its own
  conditions.
- Other pages state these inventory items: DE-2 ({rule}`CPY-7`), CO-1
  ({rule}`OPR-5`), CO-8 ({rule}`OPR-4`), BI-3 ({rule}`COND-11`), BI-9
  ({rule}`ICS-10`), CI-5 ({rule}`CVF-5`, {rule}`ICS-5`), CI-6
  ({rule}`ILC-3`, {rule}`ILC-8`), OP-1 to OP-7 ({rule}`ICS-10`,
  {rule}`ICS-13`, {rule}`ICS-14`), and AO-8 ({rule}`MPTR-2`).

Inputs: edg-overload, clang-diag-sema.

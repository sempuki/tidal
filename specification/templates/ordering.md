# Partial ordering of templates

{term}`Partial ordering <partial ordering>` decides which of two templates is more
specialized, that is, which one accepts fewer argument lists. Overload
resolution, the address of an overload set, a declaration of a
specialization, and the selection of a partial specialization all use
it. This page covers:

- the types that take part in each context,
- the steps that compare two function templates,
- each tie-breaker after those steps,
- partial specializations of class templates and variable templates.

Deduction is on {doc}`deduction`. Constraints are not specified yet
*(concepts referenced)*. This page states only how they break a tie in
partial ordering.

Depends on: template declarations, explicit and partial
specialization, template argument deduction, overload resolution.

## Principles

:::{principle} ORD-P1
:rules: ORD-5 ORD-6 ORD-7 ORD-8 ORD-9 ORD-14

A template is more specialized than another if each argument list that
it accepts, the other accepts too, and not the other way round. The
context decides which types of the two templates take part:

| Context | Types that take part | Rules |
|---|---|---|
| a call, after the sequences tie ({rule}`OVL-28`) | the parameters for which the call has arguments | ORD-5 |
| a call with a member template | as for a call, and an object parameter for each member | ORD-6 |
| a call with a swapped rewritten candidate | as for a call, with the swapped parameters reversed | ORD-7 |
| an initialization by conversion function templates | the result types | ORD-8 |
| the address of an overload set ({rule}`OVL-19`), a declaration of a specialization ({rule}`DED-32`) | the function types | ORD-9 |
| partial specializations ({rule}`TSP-15`, {rule}`TSP-11`) | one parameter for each, of the class with its pattern | ORD-14 |
:::

:::{principle} ORD-P2
:rules: ORD-1 ORD-2 ORD-3 ORD-4 ORD-10 ORD-11 ORD-12 ORD-13 ORD-15

The comparison of two templates `F1` and `F2` runs in these steps:

```text
F1, F2, and the types that take part (ORD-P1)
   |
   v
1. invent      each template parameter of F1 becomes a new type,      ORD-1
   |           value, or template that matches nothing else
   v
2. strip       each type of both loses its reference, then its        ORD-2
   |           top-level const and volatile
   v
3. deduce      F2's template parameters are deduced from F1's        ORD-3, ORD-4
   |           invented types, pair by pair; every pair must succeed
   v
4. repeat      steps 1 to 3 with F1 and F2 swapped                    ORD-1
   |
   v
5. verdict     one direction succeeds: that template's source is      ORD-1
   |           more specialized
   |           neither: no order (ORD-15 if the use needs one)
   |           both: the tie-breakers, the first one that applies:
   v
   a. per pair    an lvalue reference beats an rvalue reference       ORD-10
   b. per pair    more const and volatile on a reference wins         ORD-11
   c. pack        no trailing pack beats a trailing pack              ORD-12
   d. constraints the more constrained wins, if the heads and the     ORD-13
                  types are the same
```

An invented type stands for any argument that `F1` accepts. If `F2`
can be deduced from the invented types, `F2` accepts everything that
`F1` accepts. Step 2 strips references and qualifiers because one
argument can bind to either kind of reference. Steps 5a and 5b then use
them as tie-breakers. Steps 5a and 5b decide one pair each. If one pair
prefers `F1` and another pair `F2`, neither template is more specialized
(`reference-pairs-disagree.cpp`).

A test observes 5a before 5c (`reference-before-variadic.cpp`). The
other tie-breakers never decide the same case, so their order is not
observable:

- 5a needs two references of different kinds, and 5b two references of
  one kind.
- 5a and 5b need types that differ, and 5d needs the same types.
- 5c needs a trailing pack in only one template, and 5d needs
  equivalent template heads.
- 5b against 5c has no test, and the page gives 5b first with 5a.
:::

## Constraints

### The comparison

:::{rule} ORD-1
:kind: required
:tests: templates/ordering/more-specialized.cpp templates/ordering/repeated-parameter.cpp templates/ordering/neither.cpp

Each template parameter of `F1` is replaced by a new unique type,
value, or template. `F1` is at least as specialized as `F2` if
deduction of `F2` from these types succeeds for every pair. `F1` is
{term}`more specialized` than `F2` if `F2` is not also at least as
specialized as `F1`.
:::

:::{rule} ORD-2
:kind: required
:tests: templates/ordering/reference-and-cv-removed.cpp

Before deduction, each type of both templates loses its reference, and
then its top-level `const` and `volatile`. The kind of reference and
the qualifiers are kept for ORD-10 and ORD-11.
:::

:::{rule} ORD-3
:kind: required
:tests: templates/ordering/unused-parameter.cpp

Deduction in partial ordering matches the types as they are after
ORD-2. The adjustments of {rule}`DED-6` and the differences of
{rule}`DED-14` and {rule}`DED-15` do not apply. A template parameter
that appears in none of the compared types needs no value.
:::

:::{rule} ORD-4
:kind: required
:tests: templates/ordering/pack-against-single.cpp

An invented pack cannot give a value to a parameter that is not a pack,
so such a pair fails. A pack with no parameter against the pack takes
no part. The number of arguments for a pack in the call takes no part.
:::

### The types that take part

:::{rule} ORD-5
:kind: required
:tests: templates/ordering/call-arguments-only.cpp templates/ordering/call-defaults-tie.cpp

In a call, only the function parameters for which the call has
arguments take part. Parameters that use a default argument, and the
ellipsis, take no part.
:::

:::{rule} ORD-6
:kind: required
:tests: templates/ordering/member-object.cpp

One template can be a member with an implicit object parameter while
the other is not. The member then gets a first parameter of type
reference to its class, with the member's qualifiers. The reference is
an rvalue reference for the ref-qualifier `&&`, and an lvalue reference
otherwise.
:::

:::{rule} ORD-7
:kind: required
:tests: templates/ordering/reversed-candidate.cpp

For a swapped rewritten candidate ({rule}`OVL-13`), the order of its
two parameters is reversed before the comparison ({question}`Q-0211`).
:::

:::{rule} ORD-8
:kind: required
:tests: templates/ordering/conversion-result.cpp

For two conversion function templates, only the result types take
part.
:::

:::{rule} ORD-9
:kind: required
:tests: templates/ordering/address-function-type.cpp templates/deduction/declaration-specialization.cpp

For the address of an overload set, the whole function types take
part, with the result types. A declaration of a specialization uses the
whole function types too.
:::

### Tie-breakers

:::{rule} ORD-10
:kind: required
:tests: templates/ordering/lvalue-over-rvalue-reference.cpp templates/ordering/reference-before-variadic.cpp templates/ordering/reference-pairs-disagree.cpp

Consider a pair whose types were both references before ORD-2, where
deduction succeeds both ways. For that pair, an lvalue reference is
more specialized than an rvalue reference.
:::

:::{rule} ORD-11
:kind: required
:tests: templates/ordering/more-qualified-reference.cpp

Consider a pair whose types were references of one kind before ORD-2,
where deduction succeeds both ways. For that pair, the type with more
`const` and `volatile` is more specialized.
:::

:::{rule} ORD-12
:kind: required
:tests: templates/ordering/non-variadic.cpp templates/ordering/reference-before-variadic.cpp

Each template can be at least as specialized as the other. If only one
of them ends in a function parameter pack with no parameter against
the pack, the other one is more specialized. ORD-10 and ORD-11 come
first.
:::

:::{rule} ORD-13
:kind: required
:tests: templates/ordering/constraints-partial.cpp templates/ordering/constraints-conversion.cpp templates/ordering/constraints-different-heads.cpp templates/ordering/constraints-conversion-results-differ.cpp

If no earlier step decides, the more constrained template is more
specialized *(concepts referenced)*. The rule applies only if these
conditions hold ({question}`Q-0210`):

- The template heads are equivalent ({rule}`TPL-20`).
- The function parameter types are the same, position by position.
- For conversion functions, the result types are the same.
:::

### Partial specializations

:::{rule} ORD-14
:kind: required
:tests: templates/ordering/partial-specializations.cpp templates/ordering/partial-specializations-placeholder.cpp

Two partial specializations of one template compare as two function
templates. Each has the template head and constraints of its partial
specialization, and one parameter. The parameter's type is the class
template specialization with the partial specialization's argument list.
The primary template compares in the same way, with its own parameters
as its argument list.
:::

### No order

:::{rule} ORD-15
:kind: reject
:tests: templates/ordering/neither.cpp templates/ordering/reference-and-cv-removed.cpp templates/ordering/call-defaults-tie.cpp templates/ordering/reference-pairs-disagree.cpp templates/ordering/constraints-different-heads.cpp templates/ordering/constraints-conversion-results-differ.cpp

A use can need partial ordering to choose one template. If no template
is more specialized than each other one, the use is not valid. The rule
is the case of {rule}`OVL-36` for templates (decision 0042).
{rule}`DED-33` and {rule}`TSP-16` give the cases of a declaration and
of a partial specialization.
:::

## Examples

```cpp
template <class T> int pick(T);          // 1
template <class T> int pick(T*);         // 2
template <class T> int pick(const T*);   // 3
int* p = nullptr;
const int* q = nullptr;
pick(1);                                 // 1
pick(p);                                 // 2: T* is more specialized than T
pick(q);                                 // 3: const T* is more specialized than T*

template <class T> int take(T);
template <class T> int take(const T&);
int n = 0;
take(n);                                 // rejected: neither is more specialized

template <class T> int bind(T&);
template <class T> int bind(T&&);
bind(n);                                 // bind(T&): an lvalue reference wins
```

Partial specializations compare by their patterns:

```cpp
template <class T, class U> struct Pair { };          // the primary template
template <class T> struct Pair<T, T> { };             // A
template <class T, class U> struct Pair<T*, U> { };   // B
template <class T> struct Pair<T*, T*> { };           // C
Pair<int, int> a;                                     // A
Pair<int*, char> b;                                   // B
Pair<int*, int*> c;                                   // C, more specialized than A and B
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 21
of 24 tests.

The clean-room writer probed each step and tie-breaker with Clang 22,
GCC 16, and EDG 7.0, and checked the tests and grids on the eight
baseline compilers. The compilers agree on every
cell of the grids below.

- ORD-7: Clang and EDG reverse the parameters of a swapped candidate
  for partial ordering, and choose the swapped candidate as more
  specialized. GCC does not
  reverse them and chooses the written candidate ({question}`Q-0211`).
  The test also shows that partial ordering ({rule}`OVL-28`) comes
  before the preference for a written candidate ({rule}`OVL-32`).
- ORD-13: EDG lets a constraint decide between templates whose heads
  differ in length, and between conversion function templates whose
  results differ. Clang and GCC find both calls ambiguous
  ({question}`Q-0210`). {question}`Q-0146` is the case of different
  function parameters.
- ORD-5: two templates that differ only in a parameter with a default
  argument are ambiguous for a call that uses the default, on every
  compiler.
- ORD-6: the inventory describes a special case for the kind of the
  invented object reference. The tests cover the ref-qualifiers `&&`
  and none, and every compiler agrees on them.
- ORD-14: the constant parameters `auto*` and `auto**` order as the
  matching function templates on every compiler.

Function templates (ORD-1, ORD-2, ORD-4, ORD-10, ORD-11). Each column declares
two templates `f` in two namespaces that a using-declaration merges.
The first returns `One` and the second `Two`. The row "X → first" checks
that the call `f(X)` chooses the first template. A `reject` in both rows
means that the call is ambiguous or has no viable function:

| | T / T pointer | T / const T pointer | T / T lref | T / const T lref | T / T rref | T / Box of T | T / pack | T pointer / const T pointer | T pointer / T lref | T pointer / const T lref | T pointer / T rref | T pointer / Box of T | T pointer / pack | const T pointer / T lref | const T pointer / const T lref | const T pointer / T rref | const T pointer / Box of T | const T pointer / pack | T lref / const T lref | T lref / T rref | T lref / Box of T | T lref / pack | const T lref / T rref | const T lref / Box of T | const T lref / pack | T rref / Box of T | T rref / pack | Box of T / pack |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int lvalue → first | accept | accept | reject | reject | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | reject |
| int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | reject | accept | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept |
| const int lvalue → first | accept | accept | reject | reject | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject |
| const int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| int prvalue → first | accept | accept | accept | reject | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject |
| int prvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | accept | reject | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject | accept |
| int pointer → first | reject | accept | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | accept | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | reject |
| int pointer → second | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | accept | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept |
| const int pointer → first | reject | reject | reject | reject | reject | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | reject |
| const int pointer → second | accept | accept | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept |
| Box lvalue → first | accept | accept | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | accept | reject | reject | accept | reject | accept | accept |
| Box lvalue → second | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | accept | reject | accept | accept | reject | accept | reject | reject |

Many cells are decided before partial ordering, by viability or by the
sequences ({rule}`ICS-28`, {rule}`ICS-30`). The columns `T / T lref`,
`T / const T lref`, and `T / T rref` reject in every row: the sequences
tie, and after ORD-2 both types are `T`.

Partial specializations (ORD-14, {rule}`TSP-15`, {rule}`TSP-16`). Each
column declares two partial specializations of `Pair`. The first
gives `value` 1 and the second 2. The row "X → first" checks that
`Pair<X>` selects the first. A `reject` in both rows means that the
selection is ambiguous or that the primary template is selected:

| | T T / T pointer U | T T / T U pointer | T T / T pointer T pointer | T T / T int | T T / const T U | T pointer U / T U pointer | T pointer U / T pointer T pointer | T pointer U / T int | T pointer U / const T U | T U pointer / T pointer T pointer | T U pointer / T int | T U pointer / const T U | T pointer T pointer / T int | T pointer T pointer / const T U | T int / const T U |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int int → first | accept | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| int int → second | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject |
| int pointer int pointer → first | reject | reject | reject | accept | accept | reject | reject | accept | accept | reject | accept | accept | accept | accept | reject |
| int pointer int pointer → second | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject |
| int pointer char → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| int pointer char → second | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int int pointer → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| int int pointer → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int int → second | reject | reject | reject | accept | accept | reject | reject | accept | accept | reject | accept | accept | accept | accept | reject |
| char int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| char int → second | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject |

The probe grids are in `tests/templates/ordering/grids/`. This table
gives the source of each rule. Inventory items have the numbers of
`reports/review/2026-10-31-inventory-deduction.md`:

| Rule | Source |
|---|---|
| ORD-1 | grid function-templates; clang-sema-deduction (the two directions of the comparison); inventory L3, M1, M11 |
| ORD-5 | probes; diagnostics; inventory L1 |
| ORD-6 | probes; clang-sema-deduction (the object parameter added to a member); inventory L4 |
| ORD-8, ORD-9 | probes; clang-sema-deduction (the types of each context); inventory L2, L5 |
| ORD-2 | grid function-templates; diagnostics; clang-sema-deduction (the references and qualifiers removed in ordering); inventory L6 |
| ORD-3 | probes; clang-sema-deduction (the template parameters that the compared types use); inventory L8 |
| ORD-4 | grid function-templates; probes; inventory M8, M9 |
| ORD-10, ORD-11 | grid function-templates; probes; inventory M5, M6 |
| ORD-12 | probes; clang-sema-deduction (the trailing pack check); inventory M7 |
| ORD-13 | probes; diagnostics; clang-sema-deduction (the conditions for comparing constraints); inventory M2 to M4 |
| ORD-7 | probe; inventory L7 |
| ORD-14 | grid partial-specializations; probes; inventory N1 to N4 |

Inventory item M10, constraints between a swapped candidate and another
template, is left out. It belongs to constraints *(concepts
referenced)*.

Inputs: clang-sema-deduction.

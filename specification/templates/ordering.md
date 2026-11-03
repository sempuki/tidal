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

Deduction is on {doc}`deduction`. Constraints are on {doc}`constraints`.
This page states only how they break a tie in partial ordering.

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
| a call with a {term}`member template` | as for a call, and an object parameter for each member | ORD-6 |
| a call with a swapped {term}`rewritten candidate` | as for a call, with the swapped parameters reversed | ORD-7 |
| an initialization by conversion {term}`function templates <function template>` | the result types | ORD-8 |
| the address of an {term}`overload set` ({rule}`OVL-19`) | the function types | ORD-9 |
| an {term}`explicit specialization`, {term}`explicit instantiation`, or friend declaration ({rule}`DED-32`) | the function types | ORD-9 |
| a placement deallocation function template ({rule}`DED-39`) | the function types | ORD-9 |
| {term}`partial specializations <partial specialization>` ({rule}`TSP-15`, {rule}`TSP-11`) | one parameter for each, of the class with its pattern | ORD-14 |
:::

:::{principle} ORD-P2
:rules: ORD-1 ORD-2 ORD-3 ORD-4 ORD-15 ORD-16

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
3. deduce      F2's template parameters are deduced from F1's        ORD-3, ORD-4,
   |           invented types, pair by pair                           ORD-16
   v
4. repeat      steps 1 to 3 with F1 and F2 swapped                    ORD-1
   |
   v
5. pairs       a pair keeps a direction if deduction succeeds in      ORD-1
   |           that direction; a pair of two references that
   |           succeeds both ways can lose one direction or both:
   |           a. an rvalue reference against an lvalue reference     ORD-10
   |              loses
   |           b. a reference with fewer const and volatile loses     ORD-11
   v
6. verdict     F1 is at least as specialized as F2 if every pair      ORD-1
   |           keeps the direction from F1 to F2
   |           one direction only: that template is more
   |           specialized
   |           neither: no order (ORD-15 if the use needs one)
   |           both: the tie-breakers, the first one that applies:
   v
   c. pack        no trailing pack beats a trailing pack              ORD-12
   d. constraints the more constrained wins, if the heads and the     ORD-13
                  parameter lists are the same (the conditions of
                  ORD-13)
```

An invented type stands for any argument that `F1` accepts. If `F2`
can be deduced from the invented types, `F2` accepts everything that
`F1` accepts.
:::

:::{principle} ORD-P3
:rules: ORD-10 ORD-11

Step 2 of {principle}`ORD-P2` strips references and qualifiers because
one argument can bind to either kind of reference. Step 5 then uses
them inside each pair, before the verdict. If one pair prefers `F1` and
another pair `F2`, neither template is more specialized
(`reference-pairs-disagree.cpp`). The same holds when {term}`deduction`
succeeds in only one direction, and a reference pair removes that
direction (`reference-pairs-one-way.cpp`). Steps 5a and 5b apply on
their own, and both can remove a direction from one pair
(`reference-kind-and-qualifiers.cpp`).
:::

:::{principle} ORD-P4
:rules: ORD-12 ORD-13

When each template is at least as specialized as the other, the
tie-breakers of step 6 apply. Steps 5a and 5b act inside a pair, so
they come before 6c and 6d. A test observes 5a before 6c
(`reference-before-variadic.cpp`). Steps 6c and 6d never decide the
same case, because 6c needs a trailing pack in only one template, and
6d needs parameter lists of the same length.
:::

## Constraints

### The comparison

:::{rule} ORD-1
:kind: required
:tests: templates/ordering/more-specialized.cpp templates/ordering/repeated-parameter.cpp templates/ordering/neither.cpp

Each template parameter of `F1` is replaced by a new unique type,
value, or template. `F1` is at least as specialized as `F2` if, for
every pair, deduction of `F2` from these types succeeds and ORD-10 and
ORD-11 keep that direction. `F1` is {term}`more specialized` than `F2`
if `F2` is not also at least as specialized as `F1`.
:::

:::{rule} ORD-2
:kind: required
:tests: templates/ordering/reference-and-cv-removed.cpp templates/ordering/reference-removed-forwarding.cpp

Before deduction, each type of both templates loses its reference, and
then its top-level `const` and `volatile`. The kind of reference and
the qualifiers are kept for ORD-10 and ORD-11.
:::

:::{rule} ORD-3
:kind: required
:tests: templates/ordering/unused-parameter.cpp templates/ordering/unused-parameter-explicit.cpp

Deduction in partial ordering matches the types as they are after
ORD-2. The adjustments of {rule}`DED-6` and the differences of
{rule}`DED-14` and {rule}`DED-15` do not apply. A template parameter
that appears in none of the compared types needs no value.
:::

:::{rule} ORD-4
:kind: required
:tests: templates/ordering/pack-against-single.cpp templates/ordering/pack-class-arguments.cpp

An invented pack cannot give a value to a parameter that is not a pack,
so such a pair fails. A pack with nothing against it takes no part, and
neither does the number of call arguments for a pack. Both statements
hold for function parameters and for the {term}`template arguments <template argument>` of a {term}`class template`
{term}`specialization`.
:::

:::{rule} ORD-16
:kind: required
:tests: templates/ordering/nondeduced-position.cpp templates/ordering/nondeduced-position-pointer.cpp

A {term}`non-deduced position` ({rule}`DED-13`) in `F2`'s types gives
no value, and its type is not checked against the invented type. In
`F1`'s invented types, a qualified name through an invented type stays
unresolved. Such a name matches only itself, so `typename Id<U>::type`
does not match `U`.
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
:tests: templates/ordering/member-object.cpp templates/ordering/member-object-rvalue.cpp templates/ordering/reversed-member-object.cpp

One template can be a member with an {term}`implicit object parameter` while
the other is not. The member then gets a first parameter of type
reference to its class, with the member's qualifiers. The reference is
an lvalue reference, except in these cases, where it is an rvalue
reference ({question}`Q-0214`):

- the member has the ref-qualifier `&&`,
- the member has no ref-qualifier, and the other template's first
  parameter is an rvalue reference.
:::

:::{rule} ORD-7
:kind: required
:tests: templates/ordering/reversed-candidate.cpp templates/ordering/reversed-member-object.cpp

For a swapped rewritten candidate ({rule}`OVL-13`), the order of its
two parameters is reversed before the comparison ({question}`Q-0211`).
For a member, the object parameter of ORD-6 is added first, and then
the two parameters are reversed ({question}`Q-0218`).
:::

:::{rule} ORD-8
:kind: required
:tests: templates/ordering/conversion-result.cpp templates/ordering/reference-kinds-conversion.cpp templates/ordering/reference-kind-and-qualifiers.cpp

For two conversion function templates, only the result types take
part.
:::

:::{rule} ORD-9
:kind: required
:tests: templates/ordering/address-function-type.cpp templates/deduction/declaration-specialization.cpp templates/deduction/placement-deallocation-ordered.cpp templates/deduction/placement-deallocation-ordered-instantiation.cpp

For the address of an overload set, the whole function types take
part, with the result types. A declaration of a specialization
({rule}`DED-32`) and the match of a placement deallocation function
({rule}`DED-39`) use the whole function types too.
:::

### Tie-breakers

:::{rule} ORD-10
:kind: required
:tests: templates/ordering/lvalue-over-rvalue-reference.cpp templates/ordering/reference-before-variadic.cpp templates/ordering/reference-pairs-disagree.cpp templates/ordering/reference-pairs-one-way.cpp templates/ordering/reference-kinds-conversion.cpp

Consider a pair whose types were both references before ORD-2, where
deduction succeeds both ways. If one type was an lvalue reference and
the other an rvalue reference, the pair loses the direction in which
the rvalue reference's template is at least as specialized. The rule
acts inside the pair, before ORD-1 combines the pairs.
:::

:::{rule} ORD-11
:kind: required
:tests: templates/ordering/more-qualified-reference.cpp templates/ordering/reference-kinds-conversion.cpp templates/ordering/reference-kind-and-qualifiers.cpp

Consider a pair whose types were both references before ORD-2, where
deduction succeeds both ways. If one referred type has more `const` and
`volatile` than the other, the pair loses the direction in which the
less qualified type's template is at least as specialized. The rule
applies whatever the kinds of the two references, together with ORD-10
({question}`Q-0217`).
:::

:::{rule} ORD-12
:kind: required
:tests: templates/ordering/non-variadic.cpp templates/ordering/reference-before-variadic.cpp templates/ordering/pack-default-parameter.cpp

Each template can be at least as specialized as the other after ORD-10
and ORD-11. If only one of them ends in a {term}`function parameter pack`, and
the other has no parameter at the pack's position, the other one is
more specialized. A parameter with a default argument counts as a
parameter at that position ({question}`Q-0216`).
:::

:::{rule} ORD-13
:kind: required
:tests: templates/ordering/constraints-partial.cpp templates/ordering/constraints-conversion.cpp templates/ordering/constraints-different-heads.cpp templates/ordering/constraints-conversion-results-differ.cpp templates/ordering/constraints-default-parameter.cpp templates/ordering/constraints-placeholder-parameter.cpp templates/ordering/constraints-type-constraint.cpp templates/ordering/constraints-ellipsis.cpp foundations/overload-resolution/address-no-target-constraints-different-parameters.cpp

If no earlier step decides, the {term}`more constrained` template
({rule}`CNS-23`) is more specialized. {term}`Constraints <constraint>` decide between two
functions, templates or not, only if all these conditions hold
({question}`Q-0146`):

- The function parameter lists have the same length, including the
  parameters with default arguments.
- The function parameter types are the same, position by position.
- For conversion functions, the result types are the same.
- For two templates, the {term}`template heads <template head>` are equivalent by
  {rule}`TPL-20`, which leaves out constraints. The invented type
  parameters of {rule}`DED-34` are part of the head.
- For two functions that are not templates, both parameter lists end
  in an ellipsis or neither does ({question}`Q-0293`).

{rule}`OVL-19`, {rule}`OVL-30`, and {rule}`OVL-41` use these conditions.
:::

### Partial specializations

:::{rule} ORD-14
:kind: required
:tests: templates/ordering/partial-specializations.cpp templates/ordering/partial-specializations-placeholder.cpp

Two partial specializations of one template compare as two function
templates. Each has the template head and constraints of its partial
specialization, and one parameter. The parameter's type is the class
template specialization with the partial specialization's argument list.
The {term}`primary template` compares in the same way, with its own parameters
as its argument list. Constraints compare by {rule}`CNS-23`.
:::

### No order

:::{rule} ORD-15
:kind: reject
:tests: templates/ordering/neither.cpp templates/ordering/reference-and-cv-removed.cpp templates/ordering/call-defaults-tie.cpp templates/ordering/reference-pairs-disagree.cpp templates/ordering/constraints-different-heads.cpp templates/ordering/constraints-conversion-results-differ.cpp templates/ordering/member-object-rvalue.cpp templates/ordering/reference-pairs-one-way.cpp templates/ordering/reference-kind-and-qualifiers.cpp templates/ordering/pack-default-parameter.cpp templates/ordering/constraints-default-parameter.cpp templates/ordering/reference-removed-forwarding.cpp

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

In ORD-10 and ORD-11, the template with the lvalue reference, or with
the more qualified reference, is more specialized. For two conversion
function templates, only the result types take part (ORD-8). Each row
gives a pair of results, and the template that the initialization
`int x = Source{};` calls:

| First result | Second result | Chosen | Rule |
|---|---|---|---|
| `T&` | `T&&` | `T&` | ORD-10 |
| `T&` | `const T&` | `const T&` | ORD-11 |
| `const T&` | `T&&` | `const T&` | ORD-10, ORD-11 |
| `T&&` | `const T&&` | `const T&&` | ORD-11 |
| `T&` | `const T&&` | Clang: neither; GCC and EDG: `T&` | ORD-10 against ORD-11 ({question}`Q-0217`) |

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

The results are in `tests/RESULTS.md`. All eight compilers agree on 31
of 42 tests.

Two questions on this page settled on 2026-11-03 under decision 0054.
For each, a source-reader gap review supports the answer with no doubt,
and a principle gives the same answer:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0146` | ORD-P2, {principle}`CNS-P4` | EDG 7.0 |
| {question}`Q-0211` | ORD-P1 | GCC 16 |

Each test keeps its override for the family that differs. Reporting the
EDG difference of Q-0146 to the EDG project waits on the maintainer
(`documentation/decisions/pending.md`).

The other questions of the partial-ordering cluster stay open:
{question}`Q-0214`, {question}`Q-0216`, {question}`Q-0217`,
{question}`Q-0218`, and {question}`Q-0293`. On 2026-11-03, Clang 22,
GCC 16, and EDG 7.0 still split on each of them. The provisional
answers of Q-0216, Q-0217, and Q-0218 follow Clang against GCC and EDG,
and the gap review of question cluster 3 supports each of them, Q-0218
with a small doubt.

The clean-room writer probed each step and tie-breaker with Clang 22,
GCC 16, and EDG 7.0, and checked the tests and grids on the eight
{ref}`baseline compilers <baseline-compilers>`. The compilers agree on every
cell of the grids below.

- ORD-7: Clang and EDG reverse the parameters of a swapped candidate
  for partial ordering, and choose the swapped candidate as more
  specialized. GCC does not
  reverse them and chooses the written candidate ({question}`Q-0211`).
  The test also shows that partial ordering ({rule}`OVL-28`) comes
  before the preference for a written candidate ({rule}`OVL-32`).
- ORD-13: EDG lets a constraint decide in three cases where Clang and
  GCC find the call ambiguous ({question}`Q-0146`). The cases are
  templates whose heads differ in length
  (`constraints-different-heads.cpp`), conversion function templates
  whose results differ (`constraints-conversion-results-differ.cpp`),
  and parameter lists that differ in length by a parameter with a
  default argument (`constraints-default-parameter.cpp`).
- ORD-13: two templates that differ only in an ellipsis tie in ordering
  by types on every compiler. Clang and EDG then let the constraint
  decide, and GCC finds the call ambiguous ({question}`Q-0293`). For
  two functions that are not templates, Clang 22 and GCC count the
  ellipsis ({question}`Q-0146`).
- ORD-5: two templates that differ only in a parameter with a default
  argument are ambiguous for a call that uses the default, on every
  compiler.
- ORD-6: a member with no ref-qualifier against an rvalue reference
  first parameter is ambiguous on Clang 22 and GCC, which give the
  member an rvalue reference. Clang 18 and EDG give it an lvalue
  reference and choose the member ({question}`Q-0214`). Every compiler
  agrees on the ref-qualifier `&&`, and on no ref-qualifier against an
  lvalue reference.
- ORD-7: for a member used as a swapped candidate, Clang adds the
  object parameter before the reversal and chooses the member. GCC and
  EDG choose the non-member ({question}`Q-0218`).
- ORD-10, ORD-11: every compiler finds the call of
  `reference-pairs-one-way.cpp` ambiguous. The reference tie-breakers
  therefore act inside each pair, as the gap review describes (item
  S1), and the diagram of {principle}`ORD-P2` follows that model.
- ORD-11: for the results `T&` and `const T&&`, Clang finds no order.
  GCC and EDG apply ORD-11 only to references of one kind and choose
  `T&` ({question}`Q-0217`). The other combinations of kind and
  `const` agree on every compiler (`reference-kinds-conversion.cpp`).
- ORD-12: a parameter with a default argument at the position of the
  pack makes the call ambiguous on Clang. GCC and EDG choose the
  template with no pack ({question}`Q-0216`).
- ORD-16: the gap review (item O9) expected the compilers to differ on
  a non-deduced position in partial ordering. Every compiler gives the
  same result in both calls of `nondeduced-position.cpp`.
- ORD-4: the gap review (item O10) predicted the outcome of each call
  in `pack-class-arguments.cpp`, and every compiler gives it.
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
`reports/review/2026-10-31-inventory-deduction.md`, and gap review
items those of
`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`:

| Rule | Source |
|---|---|
| ORD-1 | grid function-templates; clang-sema-deduction (the two directions of the comparison); inventory L3, M1, M11; gap review S1 |
| ORD-5 | probes; diagnostics; inventory L1 |
| ORD-6 | probes; clang-sema-deduction (the object parameter added to a member); inventory L4; gap review E6 |
| ORD-8, ORD-9 | probes; clang-sema-deduction (the types of each context); inventory L2, L5; gap review M7, M10 |
| ORD-2 | grid function-templates; diagnostics; clang-sema-deduction (the references and qualifiers removed in ordering); inventory L6 |
| ORD-3 | probes; clang-sema-deduction (the template parameters that the compared types use); inventory L8 |
| ORD-4 | grid function-templates; probes; inventory M8, M9; gap review O10 |
| ORD-10, ORD-11 | grid function-templates; probes; inventory M5, M6; gap review S1, O5 |
| ORD-12 | probes; clang-sema-deduction (the trailing pack check); inventory M7; gap review O6 |
| ORD-13 | probes; diagnostics; clang-sema-deduction (the conditions for comparing constraints); inventory M2 to M4; gap review O7, O8, S2 |
| ORD-7 | probe; inventory L7; gap review O11 |
| ORD-14 | grid partial-specializations; probes; inventory N1 to N4 |
| ORD-16 | probes; gap review O9 |

Inventory item M10, constraints between a swapped candidate and another
template, is left out. It belongs to {doc}`constraints`.

Inputs: clang-sema-deduction.

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
| the address of an {term}`overload set` with a target type ({rule}`OVL-19`) | the function types, as one pair | ORD-9 |
| the address of an overload set with no target type, such as the initializer of a variable declared with `auto` ({rule}`OVL-20`) | none; constraints remove a function only under the conditions of ORD-13. Two specializations that explicit template arguments give are not ordered, and the address is ambiguous ({question}`Q-0405`) | ORD-13 |
| an {term}`explicit specialization`, {term}`explicit instantiation`, or friend declaration ({rule}`DED-32`) | the function types, as one pair | ORD-9 |
| a placement deallocation function template ({rule}`DED-39`) | the function types, as one pair | ORD-9 |
| {term}`partial specializations <partial specialization>` ({rule}`TSP-15`, {rule}`TSP-11`) | one parameter for each, of the class with its pattern | ORD-14 |

The object parameter of a member stands for the way the member binds
its object in a call ({principle}`ICS-P3`). So the parameter is a
reference to the class, with the member's `const` and `volatile`, of
this kind:

| Member | Object parameter |
|---|---|
| with the ref-qualifier `&&` | an rvalue reference |
| with no ref-qualifier, against an rvalue reference as the other template's first parameter or object parameter | an rvalue reference |
| every other member | an lvalue reference |

A member with no ref-qualifier binds an lvalue and an rvalue alike. Its
object parameter takes the kind of the other template's parameter, so
the kind of reference does not decide the order for it. The same holds
when both templates are members.
:::

:::{principle} ORD-P2
:rules: ORD-1 ORD-17 ORD-2 ORD-3 ORD-4 ORD-15 ORD-16

The comparison of two templates `F1` and `F2` runs in these steps:

```text
F1, F2, and the types that take part (ORD-P1)
   |
   v
1. invent      each template parameter of F1 becomes a new type,      ORD-1,
   |           value, or template that matches nothing else; a value  ORD-17
   |           has the declared type of its parameter
   v
2. strip       each type of both loses its reference, then its        ORD-2
   |           top-level const and volatile
   v
3. deduce      F2's template parameters are deduced from F1's        ORD-3, ORD-4,
   |           invented types, pair by pair; the pairs form by        ORD-16
   |           position, and a pack of F2 pairs with each type
   |           of F1 that remains at its position and after it
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
6. verdict     F1 is at least as specialized as F2 if every pair      ORD-1,
   |           keeps the direction from F1 to F2, the values that     DED-21
   |           the pairs give agree, and each template parameter
   |           of F2 that the pairs name gets a value
   |           one direction only: that template is more
   |           specialized
   |           neither: no order (ORD-15 if the use needs one)
   |           both: the tie-breakers, the first one that applies:
   v
   c. pack        no trailing pack beats a trailing pack; an          ORD-12
                  ellipsis is not a pack
   d. constraints the more constrained wins, if the heads and the     ORD-13
                  parameter lists are the same as declared, before
                  step 2 (the conditions of ORD-13); for two
                  templates, an ellipsis does not count
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

An ellipsis takes no part in either step for two templates. `f(T)` and
`f(T, ...)` tie in 6c, because an ellipsis is not a pack, and their
parameter lists count as the same in 6d, so a constraint on either one
decides ({question}`Q-0293`). Step 6d compares the types as the
templates declare them. Step 2 does not apply there, so `const T&` and
`T` differ, and so do the results `T*` and `T* const`.
:::

## Constraints

### The comparison

:::{rule} ORD-1
:kind: required
:tests: templates/ordering/more-specialized.cpp templates/ordering/repeated-parameter.cpp templates/ordering/neither.cpp

Each template parameter of `F1` is replaced by a new unique type,
value, or template. `F1` is at least as specialized as `F2` if
deduction of `F2` from these types meets all these conditions:

- Each pair matches on its own, and ORD-10 and ORD-11 keep the
  direction from `F1` to `F2` in each pair.
- All values that the pairs give to one template parameter of `F2` are
  the same, as in {rule}`DED-21`.
- Each template parameter of `F2` that the compared types name gets a
  value (ORD-3).

`F1` is {term}`more specialized` than `F2` if `F2` is not also at least
as specialized as `F1`.
:::

:::{rule} ORD-17
:kind: required
:tests: templates/ordering/partial-specializations-placeholder.cpp templates/ordering/partial-specializations-int-against-auto.cpp templates/ordering/partial-specializations-int-against-constrained-auto.cpp

An invented value has the declared type of its template parameter.
Each placeholder in that type, such as `auto` in `auto**`, becomes a
new invented type ({question}`Q-0397`).
:::

:::{rule} ORD-2
:kind: required
:tests: templates/ordering/reference-and-cv-removed.cpp templates/ordering/reference-removed-forwarding.cpp templates/ordering/conversion-results-top-level-const.cpp

Before deduction, each type of both templates loses its reference, and
then its top-level `const` and `volatile`. The kind of reference and
the qualifiers are kept for ORD-10 and ORD-11.
:::

:::{rule} ORD-3
:kind: required
:tests: templates/ordering/unused-parameter.cpp templates/ordering/unused-parameter-explicit.cpp templates/ordering/nondeduced-only-parameter.cpp templates/ordering/reference-removed-forwarding.cpp

Deduction in partial ordering matches the types as they are after
ORD-2, without the adjustments of {rule}`DED-6`, the {term}`forwarding reference` of
{rule}`DED-8`, or the differences of {rule}`DED-14` and {rule}`DED-15`.
A template parameter that appears in none of the compared types needs
no value. A template parameter that appears in them only in
{term}`non-deduced positions <non-deduced position>` gets no value, so the direction fails.
:::

:::{rule} ORD-4
:kind: required
:tests: templates/ordering/pack-against-single.cpp templates/ordering/pack-class-arguments.cpp templates/ordering/pack-class-arguments-reference-pattern.cpp

The pairs form by position in the two lists. Packs pair in these ways,
in function parameter lists and in the {term}`template arguments <template argument>` of a
{term}`class template` {term}`specialization`:

- A pack expansion of `F2` pairs with each type of `F1` that remains at
  its position and after it.
- A function parameter pack counts once, whatever the number of its
  call arguments. A pack with no call arguments takes no part (ORD-5).
- In the template arguments of a class template specialization, an
  invented pack expansion with no argument of the other list at its
  position takes no part.
- An invented pack cannot give a value to a parameter that is not a
  pack, so such a pair fails.
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
:tests: templates/ordering/member-object.cpp templates/ordering/member-object-rvalue.cpp templates/ordering/reversed-member-object.cpp templates/ordering/swapped-member-candidates-ordered.cpp templates/ordering/member-object-lvalue-tie.cpp templates/ordering/member-objects-rvalue-and-none.cpp templates/ordering/member-objects-lvalue-and-none.cpp templates/ordering/member-object-const.cpp templates/ordering/constraints-member-object.cpp templates/ordering/member-using-base-against-derived.cpp

In a call, each template that is a member with an {term}`implicit object parameter`
gets a first parameter of type reference to its class. The reference
has the member's `const` and `volatile` ({question}`Q-0398`).
The reference is an lvalue reference, except in these cases, where it
is an rvalue reference ({question}`Q-0214`):

- the member has the ref-qualifier `&&`,
- the member has no ref-qualifier, and the other template's first
  parameter or object parameter is an rvalue reference.

A member template that a using declaration brings into a derived class
counts as a member of the derived class here. Its reference names the
derived class, as the other template's does.
:::

:::{rule} ORD-7
:kind: required
:tests: templates/ordering/reversed-candidate.cpp templates/ordering/reversed-member-object.cpp templates/ordering/swapped-candidates-ordered.cpp templates/ordering/swapped-member-candidates-ordered.cpp

For a swapped rewritten candidate ({rule}`OVL-13`), the order of its
two parameters is reversed before the comparison ({question}`Q-0211`).
For a member, the object parameter of ORD-6 is added first, and then
the two parameters are reversed ({question}`Q-0218`).
:::

:::{rule} ORD-8
:kind: required
:tests: templates/ordering/conversion-result.cpp templates/ordering/reference-kinds-conversion.cpp templates/ordering/reference-kind-and-qualifiers.cpp templates/ordering/conversion-results-top-level-const.cpp

For two conversion function templates, only the result types take
part.
:::

:::{rule} ORD-9
:kind: required
:tests: templates/ordering/address-function-type.cpp templates/deduction/declaration-specialization.cpp templates/deduction/placement-deallocation-ordered.cpp templates/deduction/placement-deallocation-ordered-instantiation.cpp templates/ordering/address-reference-kinds.cpp templates/ordering/address-reference-against-value.cpp templates/ordering/address-value-against-forwarding.cpp

In these contexts, the whole function types take part, with the result
types and every parameter:

- the address of an overload set,
- a declaration of a specialization ({rule}`DED-32`),
- the match of a placement deallocation function ({rule}`DED-39`).

The two function types form one pair, so ORD-2, ORD-10, and ORD-11 do
not act on the types inside them ({question}`Q-0396`).
Inside the pair, a parameter `T&&` of `F2`, where `T` is a template
parameter, is deduced from an lvalue reference `A&` of `F1` with
`T = A&`, because `A& &&` is `A&` ({rule}`DCL-4`). `T&&` is not
deduced from a type that is not a reference.
A member gets no object parameter in these contexts.
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
:tests: templates/ordering/more-qualified-reference.cpp templates/ordering/reference-kinds-conversion.cpp templates/ordering/reference-kind-and-qualifiers.cpp templates/ordering/qualifiers-incomparable.cpp

Consider a pair whose types were both references before ORD-2, where
deduction succeeds both ways. If the `const` and `volatile` of one
referred type are a proper superset of the other's, the pair loses the
direction in which the less qualified type's template is at least as
specialized. The rule
applies whatever the kinds of the two references, together with ORD-10
({question}`Q-0217`).
:::

:::{rule} ORD-12
:kind: required
:tests: templates/ordering/non-variadic.cpp templates/ordering/reference-before-variadic.cpp templates/ordering/pack-default-parameter.cpp templates/ordering/ellipsis-not-trailing-pack.cpp

Each template can be at least as specialized as the other after ORD-10
and ORD-11. If only one of them ends in a {term}`function parameter pack`, and
the other has no parameter at the pack's position, the other one is
more specialized. Positions count in the declared parameter lists, with
each parameter that has a default argument ({question}`Q-0216`) and
without the object parameter of ORD-6. An ellipsis is not a function
parameter pack, so `f(T)` and `f(T, ...)` stay tied.
:::

:::{rule} ORD-13
:kind: required
:tests: templates/ordering/constraints-partial.cpp templates/ordering/constraints-conversion.cpp templates/ordering/constraints-different-heads.cpp templates/ordering/constraints-conversion-results-differ.cpp templates/ordering/constraints-default-parameter.cpp templates/ordering/constraints-placeholder-parameter.cpp templates/ordering/constraints-type-constraint.cpp templates/ordering/constraints-ellipsis.cpp overloading/overload-resolution/address-no-target-constraints-different-parameters.cpp templates/ordering/constraints-member-object.cpp templates/ordering/constraints-ellipsis-on-second.cpp templates/ordering/constraints-reference-against-value.cpp

If no earlier step decides, the {term}`more constrained` template
({rule}`CNS-23`) is more specialized. {term}`Constraints <constraint>` decide between two
functions, templates or not, only if all these conditions hold
({question}`Q-0146`):

- The function parameter lists have the same length, including the
  parameters with default arguments and the object parameter of
  {rule}`ORD-6`.
- The function parameter types are the same, position by position.
- For conversion functions, the result types are the same.
- The types compare as the functions declare them. ORD-2 does not
  apply, so `const T&` and `T` differ, and so do the results `T*` and
  `T* const`.
- For two templates, the {term}`template heads <template head>` are equivalent by
  {rule}`TPL-20`, which leaves out constraints. The invented type
  parameters of {rule}`DED-34` are part of the head.
- For two functions that are not templates, both parameter lists end
  in an ellipsis or neither does. For two templates, the ellipsis does
  not count, so `f(T)` and `f(T, ...)` meet the first condition
  ({question}`Q-0293`).
- If exactly one of two templates is a swapped rewritten candidate
  ({rule}`ORD-7`), this condition takes the place of the conditions on
  the parameter types and on the heads above: every template parameter of
  both is deducible from the function parameters, and exactly one
  reordering of the template parameters makes the two heads equivalent
  and the parameter types the same.

{rule}`OVL-19`, {rule}`OVL-30`, and {rule}`OVL-41` use these conditions.
:::

### Partial specializations

:::{rule} ORD-14
:kind: required
:tests: templates/ordering/partial-specializations.cpp templates/ordering/partial-specializations-placeholder.cpp templates/ordering/constraints-partial.cpp templates/ordering/constraints-partial-different-patterns.cpp templates/ordering/partial-specializations-int-against-auto.cpp templates/ordering/partial-specializations-int-against-constrained-auto.cpp templates/ordering/address-explicit-arguments-no-target.cpp

Two partial specializations of one template compare as two function
templates. Each has the template head and constraints of its partial
specialization, and one parameter. The parameter's type is the class
template specialization with the partial specialization's argument list.
The {term}`primary template` compares in the same way, with its own parameters
as its argument list. Constraints decide by {rule}`CNS-23` only if the
heads are equivalent ({rule}`TPL-20`) and the argument lists are the
same.
:::

### No order

:::{rule} ORD-15
:kind: reject
:tests: templates/ordering/neither.cpp templates/ordering/reference-and-cv-removed.cpp templates/ordering/call-defaults-tie.cpp templates/ordering/reference-pairs-disagree.cpp templates/ordering/constraints-different-heads.cpp templates/ordering/constraints-conversion-results-differ.cpp templates/ordering/member-object-rvalue.cpp templates/ordering/reference-pairs-one-way.cpp templates/ordering/reference-kind-and-qualifiers.cpp templates/ordering/pack-default-parameter.cpp templates/ordering/constraints-default-parameter.cpp templates/ordering/reference-removed-forwarding.cpp templates/ordering/member-object-lvalue-tie.cpp templates/ordering/conversion-results-top-level-const.cpp templates/ordering/nondeduced-only-parameter.cpp templates/ordering/constraints-partial-different-patterns.cpp templates/ordering/qualifiers-incomparable.cpp templates/ordering/ellipsis-not-trailing-pack.cpp templates/ordering/constraints-reference-against-value.cpp

A use can need partial ordering to choose one template. If no template
is more specialized than each other one, the use is not valid. The rule
is the case of {rule}`OVL-36` for templates (decision 0042).
{rule}`DED-33` and {rule}`TSP-16` give the cases of a declaration and
of a partial specialization.
:::

## Examples

% tidal-example: reject
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

The examples below show the steps of {principle}`ORD-P2`. Each table
gives the types of one template after steps 1 and 2, with invented
types `A`, `B`, and so on, and the pairs that step 3 deduces the other
template from.

A pack against a parameter that is not a pack (ORD-4):

```cpp
template <class T, class... Rest> int first(T, Rest...);   // 1
template <class T, class U> int first(T, U);               // 2
int a = first(1, 2);                                       // 2
```

| Invented from | Types | Deduced | Pairs | Result |
|---|---|---|---|---|
| 2 | `A`, `B` | 1 | `T` from `A`; `Rest...` from `B` | succeeds: `T = A`, `Rest = {B}` |
| 1 | `A`, `Pack...` | 2 | `T` from `A`; `U` from `Pack...` | fails: an invented pack gives `U` no value |

So 2 is at least as specialized as 1, and 1 is not as specialized as 2.
2 is more specialized. With the call `first(1)`, only 1 is viable.

A member's object parameter (ORD-6):

```cpp
struct Meter {
  template <class T> int operator+(T*);                    // 1
};
template <class T, class U> int operator+(T&, U);          // 2
Meter m;
int* p = nullptr;
int sum = m + p;                                           // 1
```

| Invented from | Types | Deduced | Pairs | Result |
|---|---|---|---|---|
| 1 | `Meter`, `A*` | 2 | `T` from `Meter`; `U` from `A*` | succeeds |
| 2 | `A`, `B` | 1 | `Meter` against `A`; `T*` from `B` | fails: `A` is not `Meter`, and `B` is not a pointer |

Template 1 gets the object parameter `Meter&`, which step 2 makes
`Meter`. 1 is more specialized.

A swapped candidate (ORD-7):

```cpp
struct Key {};
template <class T> bool operator==(const Key&, const T&);  // 1
template <class T> bool operator==(T* const&, const Key&); // 2
Key k;
int* p = nullptr;
bool equal = k == p;                                       // 2, swapped
```

For `k == p`, 2 is a swapped candidate, so its parameters are reversed
to `const Key&` and `T* const&` before the comparison.

| Invented from | Types | Deduced | Pairs | Result |
|---|---|---|---|---|
| 2 | `Key`, `A*` | 1 | `Key` against `Key`; `T` from `A*` | succeeds |
| 1 | `Key`, `A` | 2 | `Key` against `Key`; `T*` from `A` | fails |

2 is more specialized. GCC does not reverse the parameters and chooses 1
({question}`Q-0211`). Without the reversal, the two templates have no
order, and the written candidate wins ({rule}`OVL-32`).

For a swapped member template, the object parameter of ORD-6 is added
first, and then the two parameters are reversed:

```cpp
struct Key {
  template <class T> bool operator==(T* const&) const;     // 1
};
template <class T> bool operator==(const T&, const Key&);  // 2
const Key k;
int* p = nullptr;
bool equal = p == k;                                       // 1, swapped
```

Template 1 is a swapped candidate for `p == k`. Its parameters
`(const Key&, T* const&)` become `(T* const&, const Key&)`, which is
more specialized than `(const T&, const Key&)`. GCC does not reverse
the parameters here either, so neither template is more specialized,
and GCC chooses the written candidate 2. EDG also chooses 2
({question}`Q-0218`).

A {term}`non-deduced position` (ORD-16):

```cpp
template <class T> struct Id { using type = T; };
template <class T> int f(T, typename Id<T>::type);         // 1
template <class T> int f(T, T);                            // 2
int r = f(1, 1);                                           // 2
```

| Invented from | Types | Deduced | Pairs | Result |
|---|---|---|---|---|
| 1 | `A`, `Id<A>::type` | 2 | `T` from `A`; `T` from `Id<A>::type` | fails: `Id<A>::type` stays unresolved and matches only itself |
| 2 | `A`, `A` | 1 | `T` from `A`; `Id<T>::type` gives no value and is not checked | succeeds |

2 is more specialized.

The whole function types (ORD-9). The target is `int(int&)`. `read(T)`
gives it with `T = int&`, and `read(T&)` with `T = int`:

% tidal-example: diverges
```cpp
template <class T> int read(T);                            // 1
template <class T> int read(T&);                           // 2
int (*pick)(int&) = read;                                  // 2
```

| Invented from | Types | Deduced | Pairs | Result |
|---|---|---|---|---|
| 2 | `int(A&)` | 1 | `int(T)` from `int(A&)` | succeeds: `T = A&` |
| 1 | `int(A)` | 2 | `int(T&)` from `int(A)` | fails |

The function types form one pair, and step 2 removes nothing inside
them. 2 is more specialized. Clang finds the address ambiguous
({question}`Q-0396`).

A parameter `T&&` inside the pair (ORD-9). The target is again
`int(int&)`. `read(T&)` gives it with `T = int`, and `read(T&&)` with
`T = int&`:

% tidal-example: diverges
```cpp
template <class T> int read(T&);                           // 1
template <class T> int read(T&&);                          // 2
int (*pick)(int&) = read;                                  // 1
```

| Invented from | Types | Deduced | Pairs | Result |
|---|---|---|---|---|
| 1 | `int(A&)` | 2 | `int(T&&)` from `int(A&)` | succeeds: `T = A&` |
| 2 | `int(A&&)` | 1 | `int(T&)` from `int(A&&)` | fails |

1 is more specialized. GCC 14 finds the address ambiguous. With
`read(T)` in place of `read(T&)`, the result is the other way round:
`int(T&&)` is not deduced from `int(A)`, so `read(T&&)` is more
specialized (`address-value-against-forwarding.cpp`), and Clang finds
that address ambiguous ({question}`Q-0396`).

The conditions of ORD-13 compare the declared types, and for two
templates they leave out the ellipsis:

% tidal-example: diverges
```cpp
struct Source {
  template <class T> operator T*() const;
  template <class T> requires (sizeof(T) > 1) operator T* const() const;
};
int* p = Source{};                       // rejected: T* and T* const differ

template <class T> int pick(T) requires (sizeof(T) > 1);
template <class T> int pick(T, ...);
int r = pick(1);                         // pick(T): the constraint decides
```

Ordering ties in both cases. In the first, the results `T*` and
`T* const` are not the same as declared, so the constraint does not
decide. EDG lets it decide ({question}`Q-0146`). In the second, the
parameter lists are `(T)` and `(T)` once the ellipsis is left out, so
the constraint decides. GCC finds the call ambiguous
({question}`Q-0293`).

An invented value (ORD-17). The value for `int V` has type `int`, and
the value for `auto V` has an invented type `A`:

```cpp
template <auto V> struct Kind { };                         // the primary template
template <int V> struct Kind<V> { };                       // 1
Kind<1> k;                                                 // 1
```

| Invented from | Types | Deduced | Pairs | Result |
|---|---|---|---|---|
| 1 | `Kind<v>`, `v` of type `int` | the primary template | `V` from `v` | succeeds |
| the primary template | `Kind<w>`, `w` of type `A` | 1 | `int V` from `w` | fails: `w` is not an `int` |

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 45
of 64 tests.

### Divergences

Each settled question below keeps its test's override for the family
that still differs. Reporting the EDG difference of Q-0146 to the EDG
project waits on the maintainer (`documentation/decisions/pending.md`).

- {question}`Q-0142` (OVL-19, OVL-20; open): whether constraints reduce
  an overload set whose address is taken. They do, when the functions
  meet the conditions of ORD-13, as Clang 22 and GCC 16 do. EDG, and
  the older versions of Clang and GCC, differ in some forms.
- {question}`Q-0214` (ORD-6; settled on ORD-P1): whether the object parameter of a
  member with no ref-qualifier is an rvalue reference in ordering
  against an rvalue reference. It is, so the call is ambiguous, as
  Clang 22 and GCC say. EDG and Clang 18 choose the member.
- {question}`Q-0211` (ORD-7; settled on ORD-P1): whether the
  parameters of a swapped candidate are reversed for partial ordering.
  They are. GCC differs.
- {question}`Q-0219` (DED-39; settled on {principle}`ORD-P1`): whether
  partial ordering chooses among matching placement deallocation
  templates. It does, as GCC and EDG say. Clang warns that two
  functions match and calls neither.
- {question}`Q-0398` (ORD-6; settled): whether
  the object parameter of a `const` member keeps the `const`. It does,
  as Clang and GCC say. EDG finds the call ambiguous.
- {question}`Q-0218` (ORD-7; open): whether the object parameter of a
  swapped member candidate is added before the reversal. It is, as
  Clang does. GCC and EDG choose the non-member.
- {question}`Q-0396` (ORD-9; open):
  whether the function types of ORD-9 form one pair, with no reference
  removed inside them. They do, as GCC and EDG say. Clang removes the
  reference and finds the address ambiguous, against `read(T&)` and
  against `read(T&&)` (`address-value-against-forwarding.cpp`).
- {question}`Q-0217` (ORD-11; open): whether the qualifier tie-breaker
  applies to an lvalue reference against an rvalue reference. It does,
  and Clang finds no order. GCC and EDG choose `T&`.
- {question}`Q-0216` (ORD-12; open): whether a parameter with a default
  argument counts against a trailing pack. It does, and Clang finds the
  call ambiguous. GCC and EDG choose the template with no pack.
- {question}`Q-0397` (ORD-17; settled):
  whether a partial specialization with `int V` is more specialized than
  one with a constrained `auto V`. It is, as Clang and GCC say. EDG
  chooses the constrained partial specialization.
- {question}`Q-0146` (ORD-13; settled on ORD-P2 and {principle}`CNS-P4`):
  which conditions two functions meet before their constraints decide.
  The heads, the parameter lists, and the conversion results are the
  same. EDG lets a constraint decide in more cases.
- {question}`Q-0293` (ORD-13; open): whether constraints decide between
  two templates that differ only in an ellipsis. They do, as Clang and
  EDG say, whichever template has the constraint
  (`constraints-ellipsis.cpp`, `constraints-ellipsis-on-second.cpp`).
  GCC finds the call ambiguous.

### Split tests

Each row is a test that carries an override for a baseline compiler. The
Expected column gives the outcome that the rules predict, and each
family column gives the outcome that the test expects on that family.
EDG runs no programs, so its cell for a run test gives the result of the
compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `address-reference-against-value.cpp` | ORD-9 | accept | reject | accept | accept | {question}`Q-0396` |
| `address-reference-kinds.cpp` | ORD-9 | accept | accept | GCC 16: accept; GCC 14: reject | accept | |
| `address-value-against-forwarding.cpp` | ORD-9 | accept | reject | accept | accept | {question}`Q-0396` |
| `constraints-conversion-results-differ.cpp` | ORD-13, ORD-15 | reject | reject | reject | accept | {question}`Q-0146` |
| `constraints-default-parameter.cpp` | ORD-13, ORD-15 | reject | reject | reject | accept | {question}`Q-0146` |
| `constraints-different-heads.cpp` | ORD-13, ORD-15 | reject | reject | reject | accept | {question}`Q-0146` |
| `constraints-ellipsis-on-second.cpp` | ORD-13 | accept | accept | reject | accept | {question}`Q-0293` |
| `constraints-ellipsis.cpp` | ORD-13 | accept | accept | reject | accept | {question}`Q-0293` |
| `constraints-member-object.cpp` | ORD-13, ORD-6 | accept | Clang 22: accept; Clang 18: reject | accept | accept | |
| `constraints-reference-against-value.cpp` | ORD-13, ORD-15 | reject | reject | reject | accept | {question}`Q-0146` |
| `member-object-const.cpp` | ORD-6 | accept | accept | accept | reject | {question}`Q-0398` |
| `member-object-rvalue.cpp` | ORD-6, ORD-15 | reject | Clang 22: reject; Clang 18: accept | reject | accept | {question}`Q-0214` |
| `overloading/overload-resolution/address-no-target-constraints-different-parameters.cpp` | OVL-20, ORD-13 | reject | Clang 22: reject; Clang 18: accept | reject | reject | {question}`Q-0142` |
| `pack-default-parameter.cpp` | ORD-12, ORD-15 | reject | reject | accept | accept | {question}`Q-0216` |
| `partial-specializations-int-against-constrained-auto.cpp` | ORD-14, ORD-17 | accept | accept | accept | reject | {question}`Q-0397` |
| `reference-kind-and-qualifiers.cpp` | ORD-11, ORD-15, ORD-8 | reject | reject | accept | accept | {question}`Q-0217` |
| `reversed-candidate.cpp` | ORD-7 | accept | accept | reject | accept | {question}`Q-0211` |
| `reversed-member-object.cpp` | ORD-7, ORD-6 | accept | accept | reject | reject | {question}`Q-0218` |
| `templates/deduction/placement-deallocation-ordered.cpp` | DED-39, ORD-9 | run | run-fail | run | accept | {question}`Q-0219` |

### Notes

- ORD-P1, ORD-6: the gap review of the text phase 8 wrote on expressions, foundations,
  and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) adds two details to
  ORD-P1: the object parameter carries the cv-qualifiers of the member,
  and a member with no ref-qualifier against another such member gets
  an lvalue reference. ORD-6 states the first detail, and the last row
  of ORD-P1's table gives the second.
- ORD-5: two templates that differ only in a parameter with a default
  argument are ambiguous for a call that uses the default, on every
  compiler.
- ORD-10, ORD-11: every compiler finds the call of
  `reference-pairs-one-way.cpp` ambiguous. So the reference
  tie-breakers act inside each pair, as the gap review describes (item
  S1), and the diagram of {principle}`ORD-P2` follows that model.
- ORD-16: the gap review (item O9) expected the compilers to differ on
  a non-deduced position in partial ordering. Every compiler gives the
  same result in both calls of `nondeduced-position.cpp`.
- ORD-4: the gap review (item O10) predicted the outcome of each call
  in `pack-class-arguments.cpp`, and every compiler gives it.
- ORD-14: the constant parameters `auto*` and `auto**` order as the
  matching function templates on every compiler.
- ORD-1, ORD-3: every compiler combines the values of all pairs
  (`repeated-parameter.cpp`). A template parameter that appears only in
  a non-deduced position gets no value, and the call is ambiguous on
  every compiler (`nondeduced-only-parameter.cpp`). With the same
  templates and no such parameter, the call chooses `T*`
  (`unused-parameter-explicit.cpp`).
- ORD-6: two members get an object parameter each. A member with no
  ref-qualifier against a member with `&&` gets an rvalue reference, and
  against a member with `&` an lvalue reference. Every compiler chooses
  the template that the other pair decides
  (`member-objects-rvalue-and-none.cpp`,
  `member-objects-lvalue-and-none.cpp`).
- ORD-9: no program separates whether a member gets an object
  parameter in the contexts of ORD-9. The function type of a member
  holds its qualifiers, and a target type matches only members with the
  same ones.
- ORD-11: every compiler finds the call of `qualifiers-incomparable.cpp`
  ambiguous, so `const` against `volatile` removes no direction.
- ORD-12: no program separates the declared parameter lists from the
  lists of ORD-6 and ORD-7. A pack without call arguments needs a
  unary operator or a call with fewer arguments, and the other template
  then has no parameter at that position in either list.
- ORD-13: the object parameter of ORD-6 counts in the parameter lists.
  A constraint decides between a member and a template that is not a
  member on Clang 22, GCC 16, and EDG 7.0. Clang 18 finds both calls of
  `constraints-member-object.cpp` ambiguous.
- ORD-14: every compiler rejects `constraints-partial-different-patterns.cpp`,
  so a constraint does not decide between partial specializations with
  different patterns.
- ORD-9: GCC 14 finds the address in `address-reference-kinds.cpp`
  ambiguous. GCC 16 and every other compiler choose `read(T&)`. So
  `T&&` inside the pair is deduced from `A&` with `T = A&`. Outside
  partial ordering, every compiler fails to deduce a parameter of type
  `void (*)(T&&)` from a function of type `void(int&)`.
- ORD-12: every compiler finds `pick(1)` ambiguous between `pick(T)` and
  `pick(T, ...)` (`ellipsis-not-trailing-pack.cpp`), so an ellipsis is
  not a trailing pack.
- ORD-13: Clang and GCC find a call ambiguous between a constrained
  `f(const T&)` and `f(T)` (`constraints-reference-against-value.cpp`),
  and between the results `T*` and a constrained `T* const`
  (`constraints-conversion-results-differ.cpp`). So the conditions
  compare the declared types. EDG lets the constraint decide in both
  ({question}`Q-0146`).
- {rule}`OVL-41`: constraints decide only after a tie of ORD-1. When
  the conditions of ORD-13 hold, the parameter types are the same, so
  partial ordering always ties, and no program separates the two
  readings.

### Probe grids

The probe grids are in `tests/templates/ordering/grids/`, and
[their tables](https://github.com/sempuki/tidal/blob/main/tests/templates/ordering/grids/tables.md)
sit beside them. The compilers agree on every cell. The
function-templates grid (ORD-1, ORD-2, ORD-4, ORD-10, ORD-11) declares
two templates in each column, and a row checks which one a call with
an argument of each kind chooses. The partial-specializations grid
(ORD-14, {rule}`TSP-15`, {rule}`TSP-16`) declares two partial
specializations in each column, and a row checks which one a
specialization selects.

### Sources

The clean-room writer probed each step and tie-breaker with Clang 22,
GCC 16, and EDG 7.0, and checked the tests and grids on the eight
{ref}`baseline compilers <baseline-compilers>`. This table gives the
source of each rule. Inventory items have the numbers of
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
| ORD-17 | probes; Implement report I2 (finding I2-7) |

Inventory item M10, constraints between a swapped candidate and another
template, is left out. It belongs to {doc}`constraints`.

Inputs: clang-sema-deduction.

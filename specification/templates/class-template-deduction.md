# Class template argument deduction

{term}`Class template argument deduction` finds the template arguments
of a class template from an initializer. The program writes the name of
the class template with no argument list, and the initializer decides
the specialization. This page covers:

- the places where a class template name can stand without arguments,
- the guides: from constructors, the copy candidate, the aggregate
  candidate, and {term}`deduction guides <deduction guide>` that the
  program writes,
- guides through alias templates and inherited constructors,
- the choice of a guide, and its tie-breakers,
- the initialization that follows the choice.

Deduction for function templates is on {doc}`deduction`. Overload
resolution is on {doc}`/foundations/overload-resolution`.

Depends on: deduction, partial ordering, overload resolution, the
forms of initialization, aggregates, inherited constructors.

## Principles

:::{principle} CTAD-P1
:rules: CTAD-1 CTAD-4 CTAD-5 CTAD-6 CTAD-7 CTAD-9 CTAD-11 CTAD-14 CTAD-16 CTAD-17 CTAD-25
:exceptions: CTAD-2 CTAD-3 CTAD-15 CTAD-23 CTAD-24 CTAD-26

The program writes `Box b(1);`, and the implementation finds `Box<int>`.
The implementation does not need a new algorithm for the purpose. The
class template gets a set of {term}`deduction guides <deduction guide>`.
Each guide is a function template that takes the arguments of an
initializer and returns a specialization. Overload resolution chooses
one guide, as for a call, and the result type of that guide is the
type. The procedure runs in these steps:

```text
a class template name with no argument list, and an initializer
   |
   v
1. place       the name stands where a type with an                 CTAD-1 to CTAD-4,
   |           initializer is declared                               CTAD-15
   v
2. guides      the guides of the class template:                    CTAD-5 to CTAD-14,
   |           one per constructor, or one with no parameters;       CTAD-16
   |           the copy candidate; the aggregate candidate;
   |           the user guides. An alias template or inherited
   |           constructors adapt the guides of another template.
   v
3. choose      overload resolution among the guides, as for the     CTAD-17 to CTAD-22
   |           constructors of a class, with the initializer;
   |           four tie-breakers after the usual ones
   v
4. check       an explicit guide must not win in copy-list          CTAD-23, CTAD-24
   |           initialization; one guide must be best
   v
5. initialize  the result type of the guide replaces the name;      CTAD-25, CTAD-26
               the initialization runs as for that type
```

Step 5 is a separate initialization. Deduction can succeed and the
initialization can still fail, for example on access or a deleted
constructor (CTAD-26).
:::

:::{principle} CTAD-P2
:rules: CTAD-5 CTAD-6 CTAD-7 CTAD-8 CTAD-9 CTAD-10 CTAD-11 CTAD-14 CTAD-16
:exceptions: CTAD-12 CTAD-13

A guide has the template parameters, the function parameters, and the
result below. For the class `template <class T> struct Box`:

| Source | Template parameters | Parameters | Result | Rule |
|---|---|---|---|---|
| constructor `Box(T)` | `T` | `T` | `Box<T>` | CTAD-5 |
| constructor template `template <class U> Box(T, U)` | `T`, `U` | `T`, `U` | `Box<T>` | CTAD-5 |
| no constructor | `T` | none | `Box<T>` | CTAD-6 |
| copy candidate | `T` | `Box<T>` | `Box<T>` | CTAD-7 |
| aggregate `T v;`, with the list `{1}` | `T` | `T` | `Box<T>` | CTAD-9 |
| user guide `Box(int) -> Box<long>;` | as written | as written | as written | CTAD-11 |

A guide from a constructor keeps the `explicit` of the constructor. Its
`T&&` is an rvalue reference to the class parameter, and no forwarding
reference (CTAD-8). The class's own parameters are deduced as a function
template's parameters are.

An alias template that stands for a specialization of a class template
gets the guides of that class template, matched to the alias (CTAD-14).
A class template that inherits constructors gets the guides of its base
in the same way (CTAD-16).
:::

:::{principle} CTAD-P3
:rules: CTAD-17 CTAD-18 CTAD-19 CTAD-20 CTAD-21 CTAD-22

The guides compete as the constructors of a class would, so the usual
rules of overload resolution apply first ({rule}`OVL-25` to
{rule}`OVL-28`). Every guide is a function template, so partial
ordering often decides. If two guides still tie, these tie-breakers
decide, in this order:

| Order | Wins | Over | Rule |
|---|---|---|---|
| 1 | a guide from a constructor of the class | a guide that the class inherits | CTAD-19 |
| 2 | a user guide | a guide that is not a user guide | CTAD-20 |
| 3 | the copy candidate | any other guide | CTAD-21 |
| 4 | a guide from a constructor that is not a template | a guide from a constructor template | CTAD-22 |

A user guide is a statement of intent by the program, so it wins over
what the implementation makes. The copy candidate wins over the guide
from a copy constructor, so a copy is not ambiguous. The fourth
tie-breaker follows {rule}`OVL-27` for the guides from constructors.
The first follows {rule}`OVL-31`. The tests
`tie-own-constructor-template.cpp` and
`tie-own-constructor-user-guide.cpp` place the first tie-breaker before
the second and the fourth.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   deduction_guide: [`explicit_spec`] `template_name` "(" `parameters` ")"
                  :    "->" `template_id` ";"
```

A `deduction_guide` is a `declaration`, and in a class it is a
`member_decl`. A template head before it makes a `template_decl`.

## Constraints

### Places

:::{rule} CTAD-1
:kind: required
:tests: templates/class-template-deduction/sites.cpp templates/class-template-deduction/dependent-qualifier.cpp

The name of a class template, or of an alias template (CTAD-14), can
stand with no argument list in these places:

- the type of a variable or static data member with an initializer,
- the type in a `new` expression with an initializer,
- the type of a functional cast, with parentheses or braces,
- the type of a constant template parameter ({rule}`TPL-11`).

The template arguments are then deduced from the initializer. The name
can have a qualifier, also a dependent one ({question}`Q-0243`).
:::

:::{rule} CTAD-2
:kind: reject
:tests: templates/class-template-deduction/site-pointer.cpp templates/class-template-deduction/site-reference.cpp templates/class-template-deduction/site-array.cpp templates/class-template-deduction/site-parameter.cpp templates/class-template-deduction/site-return-type.cpp templates/class-template-deduction/site-member.cpp

A class template name with no argument list must not be used in other
places. In particular, it must not be:

- part of a pointer, reference, or array declarator,
- the type of a function parameter or a return type,
- the declared type of a class member that is not `static`.

The name with `const` or `volatile` is valid. {rule}`TARG-10` is the
general rule.
:::

:::{rule} CTAD-3
:kind: reject
:tests: templates/class-template-deduction/several-declarators.cpp

If one declaration declares several variables with a deduced class
template type, each declarator must deduce the same type.
:::

:::{rule} CTAD-4
:kind: required
:tests: templates/class-template-deduction/empty-argument-list.cpp

A name with an argument list, even `<>`, is a template-id. No class
template argument deduction takes place, and the missing arguments take
their defaults ({rule}`TARG-9`).
:::

### Guides from the class

:::{rule} CTAD-5
:kind: required
:tests: templates/class-template-deduction/constructor-guides.cpp

Each constructor of the class template gives a guide:

- Its template parameters are those of the class, then those of the
  constructor, with their defaults.
- Its parameters are the parameters of the constructor.
- Its result is the class template with its own parameters as
  arguments.

A guide from an `explicit` constructor is explicit.
:::

:::{rule} CTAD-6
:kind: required
:tests: templates/class-template-deduction/no-constructor.cpp templates/class-template-deduction/no-initializer.cpp templates/class-template-deduction/aggregate-empty-list.cpp

If the class template declares no constructor, the class template has
one guide with no parameters. Its template
parameters are those of the class, and its result is as in CTAD-5.
:::

:::{rule} CTAD-7
:kind: required
:tests: templates/class-template-deduction/copy-candidate.cpp

Every class template has the {term}`copy candidate`. The copy candidate
is a guide with the template parameters of the class and one parameter.
The type of the parameter and the result are the class template with its
own parameters as arguments.
:::

:::{rule} CTAD-8
:kind: required
:tests: templates/class-template-deduction/constructor-rvalue-reference.cpp templates/class-template-deduction/constructor-rvalue-reference-lvalue.cpp templates/class-template-deduction/constructor-nondeduced.cpp templates/class-template-deduction/constructor-member-alias.cpp

A guide from a constructor deduces as a function template
({rule}`DED-1`). A parameter `T&&`, with `T` a parameter of the class,
is no {term}`forwarding reference`. A class parameter in a
{term}`non-deduced position` gets no value there. A member alias of the
class itself stands for its type.
:::

### The aggregate candidate

:::{rule} CTAD-9
:kind: required
:tests: templates/class-template-deduction/aggregate-candidate.cpp templates/class-template-deduction/aggregate-candidate-arrays.cpp templates/class-template-deduction/aggregate-candidate-pack.cpp templates/class-template-deduction/aggregate-conflict.cpp templates/class-template-deduction/aggregate-new-parentheses.cpp

An aggregate class template can have an {term}`aggregate candidate`.
The candidate has one parameter for each element of the initializer
list, in braces or parentheses, with or without designators. The type
of each parameter is one of these:

- for a member, its declared type,
- for an array member given a braced list, an rvalue reference to the
  array type,
- for an array member given a string literal, an lvalue reference to the
  `const` array type,
- for a pack of bases at the end, a function parameter pack of the
  base types.

The elements match the members as in aggregate initialization
({rule}`LIST-3`, {rule}`LIST-4`, {rule}`LIST-5`).
:::

:::{rule} CTAD-10
:kind: required
:tests: templates/class-template-deduction/aggregate-user-guide.cpp templates/class-template-deduction/aggregate-too-many.cpp templates/class-template-deduction/aggregate-no-elision.cpp templates/class-template-deduction/aggregate-elision.cpp templates/class-template-deduction/aggregate-empty-list.cpp

The aggregate candidate exists only if all of these hold:

- The class template is defined and is an aggregate.
- The class template has no user guide.
- The initializer is a list with at least one element.
- Each element of the list has a member to initialize.

The braces of a member whose type is dependent, and not an array, are
not elided ({rule}`LIST-4`).
:::

### User guides

:::{rule} CTAD-11
:kind: required
:tests: templates/class-template-deduction/user-guides.cpp templates/class-template-deduction/guide-after-use.cpp templates/class-template-deduction/member-guide.cpp

A `deduction_guide` declares a {term}`user guide <deduction guide>` for
the class template that it names. A user guide can be a template or
not. Its parameters can have default arguments. Its `T&&`, with `T` its
own type parameter, is a {term}`forwarding reference`. A use of the
class template takes the user guides declared before the use.
:::

:::{rule} CTAD-12
:kind: reject
:tests: templates/class-template-deduction/guide-result-not-specialization.cpp templates/class-template-deduction/guide-result-other-template.cpp templates/class-template-deduction/guide-result-qualified.cpp templates/class-template-deduction/guide-definition.cpp templates/class-template-deduction/guide-specifier.cpp templates/class-template-deduction/guide-redeclared.cpp templates/class-template-deduction/guide-wrong-scope.cpp templates/class-template-deduction/guide-access.cpp

A `deduction_guide` must obey these rules:

- The result is a template-id of the class template that the guide
  names, with no cv-qualifier ({question}`Q-0242`).
- The guide has no body and no specifier other than `explicit`.
- The guide is declared in the scope of the class template.
- A guide for a member template has the access of that member.
- One guide is not declared twice.
:::

### Alias templates and inherited constructors

:::{rule} CTAD-14
:kind: required
:tests: templates/class-template-deduction/alias-template.cpp templates/class-template-deduction/alias-template-fixed-argument.cpp templates/class-template-deduction/alias-template-guides.cpp templates/class-template-deduction/alias-template-unreachable.cpp templates/class-template-deduction/alias-template-conflict.cpp

An alias template can stand for a specialization of a class template,
directly or through another alias. Each guide of that class template
then gives a guide of the alias ({question}`Q-0240`):

- The result of the guide is matched with the type of the alias.
- The new guide has the alias parameters that the match uses, then the
  guide's parameters that the match leaves.
- A guide whose result does not match the type of the alias gives no
  guide.

The new guide keeps the explicit, copy, and user status of its source.
:::

:::{rule} CTAD-15
:kind: reject
:tests: templates/class-template-deduction/alias-template-not-class.cpp

Only the name of a class template, or of an alias template of CTAD-14,
can stand without an argument list for deduction. An alias template for
another type, such as a pointer, must have its arguments.
:::

:::{rule} CTAD-16
:kind: required
:tests: templates/class-template-deduction/inherited-constructors.cpp templates/class-template-deduction/inherited-user-guide.cpp templates/class-template-deduction/inherited-without-using.cpp templates/class-template-deduction/inherited-not-deducible.cpp

A class template that inherits the constructors of a base
({rule}`DER-8`) gets the guides of that base, user guides included. The
base type acts as an alias of the derived parameters (CTAD-14), and the
result is mapped back to the derived class template
({question}`Q-0239`). A class template that inherits no constructors
gets no guides from its bases.
:::

### Choosing a guide

:::{rule} CTAD-17
:kind: required
:tests: templates/class-template-deduction/selection-forms.cpp templates/class-template-deduction/selection-explicit-copy.cpp templates/class-template-deduction/explicit-user-guide-copy.cpp templates/class-template-deduction/selection-private-constructor.cpp

The guides act as the constructors of a class, and the initializer
chooses one as {rule}`OVL-14` gives. Copy initialization with `=` and no
braces leaves out the explicit guides. All guides are accessible,
whatever the access of their constructor.
:::

:::{rule} CTAD-18
:kind: required
:tests: templates/class-template-deduction/list-single-specialization.cpp

A braced list can have one element whose type is a specialization of
the class template, or a class derived from one. The choice then skips
the phase of {rule}`ILC-3` that tries only the initializer-list guides.
:::

:::{rule} CTAD-19
:kind: required
:tests: templates/class-template-deduction/tie-own-constructor.cpp templates/class-template-deduction/tie-own-constructor-template.cpp templates/class-template-deduction/tie-own-constructor-user-guide.cpp

If overload resolution and partial ordering leave two guides equal, a
guide from a constructor of the class wins over a guide that the class
inherits. This tie-breaker comes first.
:::

:::{rule} CTAD-20
:kind: required
:tests: templates/class-template-deduction/tie-user-guide.cpp templates/class-template-deduction/tie-user-guide-copy.cpp

If two guides are still equal after CTAD-19, a user guide wins over a
guide that is not a user guide.
:::

:::{rule} CTAD-21
:kind: required
:tests: templates/class-template-deduction/tie-copy-candidate.cpp

If two guides are still equal after CTAD-20, the copy candidate wins
over the other guide.
:::

:::{rule} CTAD-22
:kind: required
:tests: templates/class-template-deduction/tie-non-template.cpp templates/class-template-deduction/tie-non-template-explicit.cpp

If two guides are still equal after CTAD-21, a guide from a constructor
that is not a template wins over a guide from a constructor template.
:::

:::{rule} CTAD-23
:kind: reject
:tests: templates/class-template-deduction/explicit-copy-list.cpp templates/class-template-deduction/explicit-user-guide-copy-list.cpp templates/class-template-deduction/tie-non-template-explicit.cpp templates/class-template-deduction/tie-own-constructor-template.cpp templates/class-template-deduction/tie-own-constructor-user-guide.cpp

In copy-list initialization, the chosen guide must not be explicit. The
explicit guides take part in the choice ({rule}`ILC-8`).
:::

:::{rule} CTAD-24
:kind: reject
:tests: templates/class-template-deduction/no-initializer.cpp templates/class-template-deduction/ambiguous-guides.cpp templates/class-template-deduction/constructor-rvalue-reference-lvalue.cpp templates/class-template-deduction/constructor-nondeduced.cpp templates/class-template-deduction/aggregate-user-guide.cpp templates/class-template-deduction/aggregate-too-many.cpp templates/class-template-deduction/aggregate-conflict.cpp templates/class-template-deduction/aggregate-no-elision.cpp templates/class-template-deduction/alias-template-conflict.cpp templates/class-template-deduction/inherited-without-using.cpp templates/class-template-deduction/inherited-not-deducible.cpp templates/class-template-deduction/selection-explicit-copy.cpp

The guides must give one best guide. If no guide is viable, or no guide
is better than all others, the program is not valid. The rule is the
case of {rule}`OVL-35` and {rule}`OVL-36` for guides (decision 0042).
:::

### The initialization

:::{rule} CTAD-25
:kind: required
:tests: templates/class-template-deduction/result-type.cpp templates/class-template-deduction/result-then-initialize.cpp templates/class-template-deduction/result-then-initialize-deleted.cpp templates/class-template-deduction/selection-private-constructor.cpp

The result type of the chosen guide replaces the class template name.
The initialization then runs as for a declaration with that type, and
its own rules apply.
:::

:::{rule} CTAD-26
:kind: reject
:tests: templates/class-template-deduction/result-then-initialize.cpp templates/class-template-deduction/result-then-initialize-deleted.cpp templates/class-template-deduction/selection-private-constructor.cpp

The initialization with the deduced type must be valid. Its own rules
apply, such as {rule}`CTOR-25`, {rule}`DEL-1`, and {rule}`ACC-3`.
:::

## Implementation Requirements

:::{rule} CTAD-13
:kind: no-diagnostic
:tests: templates/class-template-deduction/guide-not-deducible.cpp

Each template parameter of a user guide must have a default or get a
value from the guide's parameters. An implementation need not diagnose
a guide that breaks this rule ({question}`Q-0241`).
:::

## Examples

```cpp
template <class T> struct Box { Box(T); };
Box a(1);                         // Box<int>
Box b = a;                        // Box<int>: the copy candidate
Box(int) -> Box<long>;
Box c(1);                         // Box<long>: the user guide

template <class T, class U> struct Pair { T first; U second; };
Pair p{1, 'c'};                   // Pair<int, char>: the aggregate candidate

template <class T> using Same = Pair<T, T>;
Same s{1, 2};                     // Pair<int, int>
Same t{1, 'c'};                   // rejected: T is int and char

Box* q = &a;                      // rejected: deduction needs the name alone
Box<> d(1);                       // rejected: an argument list, so no deduction
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 60
of 69 tests.

The clean-room writer probed each step with Clang 22, GCC 16, and EDG
7.0, and checked the tests and the grid on the eight baseline
compilers.

- CTAD-14: Clang 18 does not deduce through an alias template. EDG 7.0
  deduces through an alias that passes its parameters on, and not
  through an alias that fixes an argument or names another alias. Clang
  22 and GCC deduce in each case ({question}`Q-0240`).
- CTAD-16 and CTAD-19: Clang makes no guides from inherited
  constructors. GCC and EDG do ({question}`Q-0239`). On Clang, the
  tie-breaker tests of CTAD-19 give the same results with no inherited
  guide.
- CTAD-12: GCC accepts a guide whose result has a cv-qualifier. Clang
  and EDG reject it ({question}`Q-0242`).
- CTAD-13: Clang rejects a guide with a template parameter that no
  parameter of the guide gives a value. GCC and EDG accept the guide.
  Such a guide can never be chosen, so the rule needs no diagnostic
  ({question}`Q-0241`).
- CTAD-9: GCC forms no aggregate candidate for a parenthesized list in
  a `new` expression. Clang and EDG form it ({question}`Q-0244`).
- CTAD-1: the inventory (item G7) predicted that a dependent qualifier
  before the name is not valid. Every compiler deduces through
  `typename Outer<T>::In` after instantiation ({question}`Q-0243`).
- CTAD-24 is the reject rule of the tests in which no guide is chosen.
  Clang names deduction in its message. GCC says that deduction failed,
  and EDG reports that no constructor matches.
- CTAD-26: in each test of the rule, the message names the
  constructor of the deduced specialization, so deduction succeeded
  first.
- CTAD-21: without the tie-breaker, the copy candidate and the guide
  from `Copy(const Copy&)` would make `tie-copy-candidate.cpp`
  ambiguous. Every compiler accepts the test.

Initializer forms and guide sources (CTAD-1, CTAD-5 to CTAD-17,
CTAD-23). Each column declares a class template `C` with one kind of
guide, and `Want`, the type that deduction should give. Each row
initializes with the argument `1`, or with a `Want` object in the last
three rows, and checks the type:

| | constructor | constructor template | explicit constructor | initializer-list constructor | aggregate | user guide | explicit user guide | alias template | inherited constructor |
|---|---|---|---|---|---|---|---|---|---|
| parentheses | accept | accept | accept | reject | accept | accept | accept | accept: gcc-14, clang-host, edg-7.0-ilp32, edg-7.0, gcc-16, gcc-host, clang-22; reject: clang-18 | accept: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-16, gcc-14; reject: clang-host, clang-18, clang-22 |
| braces | accept | accept | accept | accept | accept | accept | accept | accept: edg-7.0-ilp32, edg-7.0, gcc-14, gcc-host, gcc-16, clang-host, clang-22; reject: clang-18 | accept: edg-7.0, gcc-host, edg-7.0-ilp32, gcc-14, gcc-16; reject: clang-host, clang-18, clang-22 |
| copy | accept | accept | reject | reject | reject | accept | reject | accept: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-14, gcc-16, clang-host, clang-22; reject: clang-18 | accept: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-14, gcc-16; reject: clang-host, clang-18, clang-22 |
| copy-list | accept | accept | reject | accept | accept | accept | reject | accept: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-14, clang-host, gcc-16, clang-22; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16; reject: clang-host, clang-18, clang-22 |
| functional cast | accept | accept | accept | reject | accept | accept | accept | accept: edg-7.0-ilp32, edg-7.0, gcc-14, gcc-host, gcc-16, clang-host, clang-22; reject: clang-18 | accept: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-14, gcc-16; reject: clang-host, clang-18, clang-22 |
| braced cast | accept | accept | accept | accept | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16, clang-host, clang-22; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-host, gcc-16; reject: clang-host, clang-18, clang-22 |
| new | accept | accept | accept | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept | accept | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16, clang-host, clang-22; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16; reject: clang-host, clang-18, clang-22 |
| copy, parentheses | accept | accept | accept | accept | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-host, gcc-16, clang-host, clang-22; reject: clang-18 | accept |
| copy, braces | accept | accept | accept | accept | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16, clang-host, clang-22; reject: clang-18 | accept |
| copy, equals | accept | accept | accept | accept | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-16, gcc-14, clang-host, clang-22; reject: clang-18 | accept |

In the last three rows of the last column, the copy candidate of `C`
decides on every compiler.

The grid is in `tests/templates/class-template-deduction/grids/`. This
table gives the source of each rule. Inventory items have the numbers
of `reports/review/2026-10-31-inventory-packs-forwarding-ctad.md`:

| Rule | Source |
|---|---|
| CTAD-1 | grid forms; probes; inventory G1, G3, G5, G7 |
| CTAD-2 | diagnostics; inventory G2, G6 |
| CTAD-3 | diagnostics; inventory G4 |
| CTAD-4 | probes; inventory G8 |
| CTAD-5 | grid forms; probes; clang-diag-sema (the note on an implicit guide); inventory D1, D4 |
| CTAD-6 | probes; inventory D3 |
| CTAD-7 | grid forms; probes; inventory D2 |
| CTAD-8 | probes; inventory D5, D6, P8 |
| CTAD-9, CTAD-10 | grid forms; probes; inventory C1 to C12 |
| CTAD-11 | grid forms; probes; inventory H2, H3, H5 to H7 |
| CTAD-12 | diagnostics; clang-diag-sema; inventory H1, H3, H8, H9 |
| CTAD-13 | diagnostics; inventory H4 |
| CTAD-14, CTAD-15 | grid forms; probes; diagnostics; inventory B1 to B8 |
| CTAD-16 | grid forms; probes; inventory E1 to E4 |
| CTAD-17 | grid forms; probes; inventory F1, F3 |
| CTAD-18 | probes; inventory F4 |
| CTAD-19 to CTAD-22 | probes; inventory F7 to F10 |
| CTAD-23 | grid forms; diagnostics; inventory F2, H2 |
| CTAD-24 | diagnostics in the `tidal-error` lines of its tests |
| CTAD-25, CTAD-26 | probes; diagnostics; inventory F5, F6 |

Inventory items left out of this page:

- B1, constraints on an alias template's parameters, belongs to
  {doc}`constraints`, which does not cover deduction guides made from
  an alias template yet.
- C8 and C10, a pack member before the end and the length of a trailing
  pack, have no test. A class member cannot be a pack, and a pack of
  bases is the only pack that an aggregate has.
- D3 for a class template that is declared and not defined has no
  test. Such a class gives an incomplete type after deduction.
- D1 with defaults: CTAD-5 copies the defaults of a constructor template,
  and `constructor-guides.cpp` tests one. The inventory's doubt about a
  default that names a class parameter has no probe.

Inputs: clang-diag-sema.

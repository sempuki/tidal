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
resolution is on {doc}`/overloading/overload-resolution`.

Depends on: deduction, partial ordering, overload resolution, the
forms of initialization, aggregates, inherited constructors.

## Principles

:::{principle} CTAD-P1
:rules: CTAD-1 CTAD-4 CTAD-5 CTAD-6 CTAD-7 CTAD-9 CTAD-11 CTAD-14 CTAD-16 CTAD-17 CTAD-25 CTAD-27 CTAD-28
:exceptions: CTAD-2 CTAD-3 CTAD-15 CTAD-23 CTAD-24 CTAD-26

The program writes `Box b(1);`, and the implementation finds `Box<int>`.
The implementation does not need a new algorithm for the purpose. The
{term}`class template` gets a set of {term}`deduction guides <deduction guide>`.
Each guide is a {term}`function template` that takes the arguments of an
initializer and returns a {term}`specialization`. Overload resolution chooses
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
   |           one per constructor, or one with no parameters;       CTAD-16, CTAD-27,
   |                                                                 CTAD-28
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

Step 5 is a separate initialization. {term}`Deduction <deduction>` can succeed and the
initialization can still fail, for example on access or a deleted
constructor (CTAD-26).
:::

:::{principle} CTAD-P2
:rules: CTAD-5 CTAD-6 CTAD-7 CTAD-8 CTAD-9 CTAD-10 CTAD-11 CTAD-14 CTAD-16 CTAD-27 CTAD-28 CTAD-29
:exceptions: CTAD-12 CTAD-13

A guide has the template parameters, the function parameters, and the
result below. For the class `template <class T> struct Box`:

| Source | Template parameters | Parameters | Result | Rule |
|---|---|---|---|---|
| constructor `Box(T)` | `T` | `T` | `Box<T>` | CTAD-5 |
| constructor template `template <class U> Box(T, U)` | `T`, `U` | `T`, `U` | `Box<T>` | CTAD-5 |
| no constructor, or the class template is declared and not defined | `T` | none | `Box<T>` | CTAD-6 |
| copy candidate | `T` | `Box<T>` | `Box<T>` | CTAD-7 |
| aggregate `T v;`, with the list `{1}` | `T` | `T` | `Box<T>` | CTAD-9 |
| user guide `Box(int) -> Box<long>;` | as written | as written | as written | CTAD-11 |

Only the {term}`primary template` gives guides from constructors. A guide from
a constructor keeps the `explicit` of the constructor. Its `T&&` is an
rvalue reference to the class parameter, and no forwarding reference.
The `U&&` of a constructor template's own parameter `U` is a forwarding
reference (CTAD-8). The class's own parameters are deduced as a function
template's parameters are.

An {term}`alias template` that stands for a specialization of a class template
gets the guides of that class template, matched to the alias (CTAD-14).
A guide of the alias must give a specialization of the alias that meets
the {term}`constraints <constraint>` of the alias (CTAD-28).
A class template that inherits constructors gets the guides of its base
in the same way (CTAD-16).
:::

:::{principle} CTAD-P3
:rules: CTAD-17 CTAD-18 CTAD-19 CTAD-20 CTAD-21 CTAD-22

The guides compete as the constructors of a class would, so the usual
rules of overload resolution apply first ({rule}`OVL-25` to
{rule}`OVL-28`). Every guide is a function template, so {term}`partial ordering`
often decides. If two guides still tie, these tie-breakers
decide, in this order:

| Order | Wins | Over | Rule |
|---|---|---|---|
| 1 | a guide from a constructor of the class | a guide that the class inherits | CTAD-19 |
| 2 | a user guide | a guide that is not a user guide | CTAD-20 |
| 3 | the copy candidate | any other guide | CTAD-21 |
| 4 | a guide from a constructor that is not a template | a guide from a constructor template | CTAD-22 |

A user guide is a statement of intent by the program, so it wins a tie
over what the implementation makes. The tie-breakers act only after the
usual rules. For a `short` argument, the guide from `Box(T)` needs no
conversion, so it wins over the user guide `Box(int) -> Box<long>`
(CTAD-20). The copy candidate wins over the guide
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
`member_decl`. A {term}`template head` before it makes a `template_decl`.

## Constraints

### Places

:::{rule} CTAD-1
:kind: required
:tests: templates/class-template-deduction/sites.cpp templates/class-template-deduction/dependent-qualifier.cpp templates/class-template-deduction/sites-for-and-condition.cpp templates/class-template-deduction/dependent-qualifier-template-keyword.cpp templates/class-template-deduction/template-template-parameter.cpp templates/class-template-deduction/template-template-parameter-alias.cpp templates/class-template-deduction/constant-parameter-guide.cpp templates/declarations/constant-parameter-class-template.cpp templates/class-template-deduction/no-constructor.cpp

The name of a class template, or of an alias template (CTAD-14), can
stand with no argument list in these places:

- the type of a variable, with or without an initializer,
- the type of a static data member with an initializer,
- the type of the variable of a range-based `for` loop,
- the type of a variable declared in a condition,
- the type in a `new` expression with an initializer,
- the type of a functional cast, with parentheses or braces,
- the type of a constant template parameter ({rule}`TPL-11`).

The {term}`template arguments <template argument>` are then deduced from the initializer. With no
initializer, the choice of a guide has no arguments. The name can have
a qualifier, also a dependent one ({question}`Q-0243`). A
{term}`template template parameter` can stand there, and deduction uses the
class template that it names after substitution ({question}`Q-0275`).
:::

:::{rule} CTAD-2
:kind: reject
:tests: templates/class-template-deduction/site-pointer.cpp templates/class-template-deduction/site-reference.cpp templates/class-template-deduction/site-array.cpp templates/class-template-deduction/site-parameter.cpp templates/class-template-deduction/site-return-type.cpp templates/class-template-deduction/site-member.cpp

A class template name with no argument list must not be used in other
places. In particular, it must not be:

- part of a pointer, reference, or array {term}`declarator`,
- the type of a function parameter or a return type,
- the declared type of a class member that is not `static`.

The name with `const` or `volatile` is valid. {rule}`TARG-10` is the
general rule.
:::

:::{rule} CTAD-3
:kind: reject
:tests: templates/class-template-deduction/several-declarators.cpp templates/class-template-deduction/several-declarators-copy-init.cpp

If one declaration declares several variables with a deduced class
template type, each declarator must deduce the same type.
:::

:::{rule} CTAD-4
:kind: required
:tests: templates/class-template-deduction/empty-argument-list.cpp templates/class-template-deduction/partial-argument-list.cpp

A name with an argument list, even `<>`, is a {term}`template-id`. No class
template argument deduction takes place, and the missing arguments take
their defaults ({rule}`TARG-9`).
:::

### Guides from the class

:::{rule} CTAD-5
:kind: required
:tests: templates/class-template-deduction/constructor-guides.cpp templates/class-template-deduction/specialization-constructors.cpp templates/class-template-deduction/conditional-explicit.cpp templates/class-template-deduction/conditional-explicit-copy-list.cpp

Each constructor of the primary class template gives a guide. The
constructors of partial and {term}`explicit specializations <explicit specialization>` give no guide.
A guide from a constructor has these parts:

- Its template parameters are those of the class, then those of the
  constructor, with their defaults.
- Its parameters are the parameters of the constructor.
- Its result is the class template with its own parameters as
  arguments.

A guide from an `explicit` constructor is explicit. With a conditional
`explicit`, the guide is explicit if the condition is true for the
deduced arguments.
:::

:::{rule} CTAD-6
:kind: required
:tests: templates/class-template-deduction/no-constructor.cpp templates/class-template-deduction/no-initializer.cpp templates/class-template-deduction/aggregate-empty-list.cpp templates/class-template-deduction/declared-not-defined.cpp templates/class-template-deduction/declared-not-defined-no-guide.cpp

If the class template declares no constructor, or is declared and not
defined, the class template has one guide with no parameters. Its
template parameters are those of the class, and its result is as in
CTAD-5. The user guides apply also to a class template that is not
defined (CTAD-11).
:::

:::{rule} CTAD-7
:kind: required
:tests: templates/class-template-deduction/copy-candidate.cpp templates/class-template-deduction/copy-candidate-only.cpp

Every class template has the {term}`copy candidate`. The copy candidate
is a guide with the template parameters of the class and one parameter.
The type of the parameter and the result are the class template with its
own parameters as arguments.
:::

:::{rule} CTAD-8
:kind: required
:tests: templates/class-template-deduction/constructor-rvalue-reference.cpp templates/class-template-deduction/constructor-rvalue-reference-lvalue.cpp templates/class-template-deduction/constructor-nondeduced.cpp templates/class-template-deduction/constructor-member-alias.cpp templates/class-template-deduction/constructor-template-forwarding.cpp templates/class-template-deduction/alias-template-rvalue-reference.cpp

A guide from a constructor deduces as a function template
({rule}`DED-1`), with these points:

- A parameter `T&&`, with `T` a parameter of the class, is no
  {term}`forwarding reference`.
- A parameter `U&&`, with `U` a parameter of a constructor template, is
  a forwarding reference.
- A class parameter in a {term}`non-deduced position` gets no value
  there.
- A member alias of the class itself stands for its type.
:::

:::{rule} CTAD-29
:kind: required
:tests: templates/class-template-deduction/guide-constraints.cpp templates/class-template-deduction/guide-constraints-none-satisfied.cpp templates/class-template-deduction/guide-constraints-absent-ambiguous.cpp

A guide from a constructor keeps the constraints of the constructor and
of the class template ({doc}`constraints`). A guide whose constraints
are not satisfied is not viable ({rule}`CNS-13`).
:::

### The aggregate candidate

:::{rule} CTAD-9
:kind: required
:tests: templates/class-template-deduction/aggregate-base.cpp templates/class-template-deduction/aggregate-candidate.cpp templates/class-template-deduction/aggregate-candidate-arrays.cpp templates/class-template-deduction/aggregate-candidate-pack.cpp templates/class-template-deduction/aggregate-conflict.cpp templates/class-template-deduction/aggregate-new-parentheses.cpp templates/class-template-deduction/aggregate-string-element-type.cpp templates/class-template-deduction/aggregate-string-fixed-array.cpp templates/class-template-deduction/aggregate-pack-not-last.cpp templates/class-template-deduction/aggregate-pack-not-last-element.cpp templates/class-template-deduction/aggregate-pack-length-fixed.cpp templates/class-template-deduction/aggregate-pack-length-conflict.cpp

An aggregate class template can have an {term}`aggregate candidate`.
The candidate has one parameter for each element of the initializer
list, in braces or parentheses, with or without designators. The type
of each parameter is one of these:

- for a member, its declared type,
- for a base that is not a pack, the type of the base,
- for an array member given a braced list, an rvalue reference to the
  array type,
- for an array member given a string literal, an lvalue reference to the
  `const` array type,
- for a pack of bases, the parameters that CTAD-27 gives.

The elements match the members as in aggregate initialization
({rule}`LIST-3`, {rule}`LIST-4`, {rule}`LIST-5`).
:::

:::{rule} CTAD-27
:kind: required
:tests: templates/class-template-deduction/aggregate-candidate-pack.cpp templates/class-template-deduction/aggregate-pack-not-last.cpp templates/class-template-deduction/aggregate-pack-not-last-element.cpp templates/class-template-deduction/aggregate-pack-length-fixed.cpp templates/class-template-deduction/aggregate-pack-length-conflict.cpp templates/class-template-deduction/aggregate-pack-length-fewer.cpp

A pack of bases gives the aggregate candidate a {term}`function parameter pack`
of the base types:

- A pack at the end takes the remaining elements of the list.
- A pack before other elements takes no element of the list.

If an earlier element fixes the length of the pack, the remaining
elements do not set that length ({question}`Q-0276`). A base of the pack
with no element in the list gets an empty list, as in aggregate
initialization ({principle}`LIST-P2`).
:::

:::{rule} CTAD-10
:kind: required
:tests: templates/class-template-deduction/aggregate-base.cpp templates/class-template-deduction/aggregate-base-no-elision.cpp templates/class-template-deduction/aggregate-user-guide.cpp templates/class-template-deduction/aggregate-too-many.cpp templates/class-template-deduction/aggregate-no-elision.cpp templates/class-template-deduction/aggregate-elision.cpp templates/class-template-deduction/aggregate-empty-list.cpp templates/class-template-deduction/aggregate-dependent-bound.cpp templates/class-template-deduction/aggregate-dependent-element.cpp

The aggregate candidate exists only if all of these hold:

- The class template is defined and is an aggregate.
- The class template has no user guide.
- The initializer is a list with at least one element.
- Each element of the list has a member to initialize.

The braces of a member or a base are not elided ({rule}`LIST-4`) in
these cases:

- The type of the member or the base is dependent and is not an array.
- The member is an array whose bound depends on a template parameter.

An array with a dependent element type and a fixed bound has its braces
elided, also for a string literal ({question}`Q-0277`).
:::

### User guides

:::{rule} CTAD-11
:kind: required
:tests: templates/class-template-deduction/user-guides.cpp templates/class-template-deduction/guide-after-use.cpp templates/class-template-deduction/member-guide.cpp templates/class-template-deduction/declared-not-defined.cpp templates/class-template-deduction/guide-before-instantiation.cpp

A `deduction_guide` declares a {term}`user guide <deduction guide>` for
the class template that it names. A user guide can be a template or
not, and its parameters can have default arguments. Its `T&&`, with `T`
its own {term}`type parameter`, is a {term}`forwarding reference`.

A use of the class template takes the user guides declared before the
use. A use in a template takes the user guides declared before the
{term}`point of instantiation`, also if the use depends on no template
parameter. A user guide between two points of {term}`instantiation` of one
specialization ({rule}`INST-31`) gives the points different meanings,
and {rule}`INST-32` applies.
:::

:::{rule} CTAD-12
:kind: reject
:tests: templates/class-template-deduction/guide-result-not-specialization.cpp templates/class-template-deduction/guide-result-other-template.cpp templates/class-template-deduction/guide-result-qualified.cpp templates/class-template-deduction/guide-definition.cpp templates/class-template-deduction/guide-specifier.cpp templates/class-template-deduction/guide-redeclared.cpp templates/class-template-deduction/guide-wrong-scope.cpp templates/class-template-deduction/guide-access.cpp templates/class-template-deduction/guide-same-parameters.cpp templates/class-template-deduction/guide-template-same-parameters.cpp

A `deduction_guide` must obey these rules:

- The result is a template-id of the class template that the guide
  names, with no {term}`cv-qualifier` ({question}`Q-0242`).
- The guide has no body and no specifier other than `explicit`.
- The guide is declared in the scope of the class template.
- A guide for a {term}`member template` has the access of that member.
- One guide is not declared twice.
- Two guides that are not templates do not have the same parameter
  types, even with different results ({question}`Q-0280`).
:::

:::{rule} CTAD-13
:kind: reject
:tests: templates/class-template-deduction/guide-not-deducible.cpp templates/class-template-deduction/guide-not-deducible-constant.cpp

Each template parameter of a user guide must have a default or get a
value from the guide's parameters ({question}`Q-0241`).
:::

### Alias templates and inherited constructors

:::{rule} CTAD-14
:kind: required
:tests: templates/class-template-deduction/alias-template.cpp templates/class-template-deduction/alias-template-fixed-argument.cpp templates/class-template-deduction/alias-template-guides.cpp templates/class-template-deduction/alias-template-unreachable.cpp templates/class-template-deduction/alias-template-conflict.cpp templates/class-template-deduction/alias-template-fixed-argument-exact.cpp

An alias template can stand for a specialization of a class template,
directly or through another alias. Each guide of that class template
then gives a guide of the alias ({question}`Q-0240`):

- The result of the guide is matched with the type of the alias.
- The new guide has the alias parameters that the match uses, then the
  guide's parameters that the match leaves.
- A guide parameter that the match fixes to a type is replaced by that
  type in the function parameters of the new guide.
- A guide whose result cannot match the type of the alias gives no
  guide.

The new guide keeps the explicit, copy, and user status of its source.
CTAD-28 gives when the new guide is viable.
:::

:::{rule} CTAD-28
:kind: required
:tests: templates/class-template-deduction/alias-template-result-check.cpp templates/class-template-deduction/alias-template-constraints.cpp templates/class-template-deduction/alias-template-rvalue-reference.cpp

A guide of an alias template is viable only if both of these hold after
its deduction:

- Its result is a specialization of the alias.
- The arguments satisfy the constraints of the alias template
  ({doc}`constraints`).

A parameter that stands for a class parameter stays one, so its `T&&`
is no forwarding reference (CTAD-8, {question}`Q-0278`).
:::

:::{rule} CTAD-15
:kind: reject
:tests: templates/class-template-deduction/alias-template-not-class.cpp templates/class-template-deduction/alias-template-dependent-qualifier.cpp

Only the name of a class template, or of an alias template of CTAD-14,
can stand without an argument list for deduction. An alias template for
another type, such as a pointer, must have its arguments. So must an
alias template whose type names the class template after a dependent
qualifier.
:::

:::{rule} CTAD-16
:kind: required
:tests: templates/class-template-deduction/inherited-constructors.cpp templates/class-template-deduction/inherited-user-guide.cpp templates/class-template-deduction/inherited-without-using.cpp templates/class-template-deduction/inherited-not-deducible.cpp templates/class-template-deduction/inherited-copy-candidate.cpp templates/class-template-deduction/inherited-copy-candidate-user-guide.cpp

A class template that inherits the constructors of a base
({rule}`DER-8`) gets the guides of that base, with the user guides and
the copy candidate. The
base type acts as an alias of the derived parameters (CTAD-14), and the
result is mapped back to the derived class template
({question}`Q-0239`). A class template that inherits no constructors
gets no guides from its bases.
:::

### Choosing a guide

:::{rule} CTAD-17
:kind: required
:tests: templates/class-template-deduction/selection-forms.cpp templates/class-template-deduction/selection-explicit-copy.cpp templates/class-template-deduction/explicit-user-guide-copy.cpp templates/class-template-deduction/selection-private-constructor.cpp templates/class-template-deduction/conditional-explicit.cpp

The guides act as the constructors of a class, and the initializer
chooses one as {rule}`OVL-14` gives. Copy initialization with `=` and no
braces leaves out the explicit guides. All guides are accessible,
whatever the access of their constructor.
:::

:::{rule} CTAD-18
:kind: required
:tests: templates/class-template-deduction/list-single-specialization.cpp templates/class-template-deduction/list-single-specialization-alias.cpp

A braced list can have one element whose type is a specialization of
the class template, or a class derived from one. The choice then skips
the phase of {rule}`ILC-3` that tries only the initializer-list guides.
The same holds for a name of an alias template of the class template
({question}`Q-0279`).
:::

:::{rule} CTAD-19
:kind: required
:tests: templates/class-template-deduction/tie-own-constructor.cpp templates/class-template-deduction/tie-own-constructor-template.cpp templates/class-template-deduction/tie-own-constructor-user-guide.cpp templates/class-template-deduction/tie-own-constructor-types-differ.cpp

If overload resolution and partial ordering leave two guides equal, a
guide from a constructor of the class wins over a guide that the class
inherits. The tie-breaker applies only if the two guides have the same
parameter type for each argument. This tie-breaker comes first.
:::

:::{rule} CTAD-20
:kind: required
:tests: templates/class-template-deduction/tie-user-guide.cpp templates/class-template-deduction/tie-user-guide-copy.cpp templates/class-template-deduction/tie-user-guide-conversion.cpp

If two guides are still equal after CTAD-19, a user guide wins over a
guide that is not a user guide.
:::

:::{rule} CTAD-21
:kind: required
:tests: templates/class-template-deduction/tie-copy-candidate.cpp templates/class-template-deduction/tie-copy-candidate-forms.cpp

If two guides are still equal after CTAD-20, the copy candidate wins
over the other guide.
:::

:::{rule} CTAD-22
:kind: required
:tests: templates/class-template-deduction/tie-non-template.cpp templates/class-template-deduction/tie-non-template-explicit.cpp

If two guides are still equal after CTAD-21, a guide from a constructor
that is not a template wins over a guide from a constructor template.
In a call, a parameter that uses its default argument takes no part in
the comparison of the two guides ({rule}`ORD-5`).
:::

:::{rule} CTAD-23
:kind: reject
:tests: templates/class-template-deduction/explicit-copy-list.cpp templates/class-template-deduction/explicit-user-guide-copy-list.cpp templates/class-template-deduction/tie-non-template-explicit.cpp templates/class-template-deduction/tie-own-constructor-template.cpp templates/class-template-deduction/tie-own-constructor-user-guide.cpp templates/class-template-deduction/conditional-explicit-copy-list.cpp

In copy-list initialization, the chosen guide must not be explicit. The
explicit guides take part in the choice ({rule}`ILC-8`).
:::

:::{rule} CTAD-24
:kind: reject
:tests: templates/class-template-deduction/no-initializer.cpp templates/class-template-deduction/ambiguous-guides.cpp templates/class-template-deduction/constructor-rvalue-reference-lvalue.cpp templates/class-template-deduction/constructor-nondeduced.cpp templates/class-template-deduction/aggregate-user-guide.cpp templates/class-template-deduction/aggregate-too-many.cpp templates/class-template-deduction/aggregate-conflict.cpp templates/class-template-deduction/aggregate-no-elision.cpp templates/class-template-deduction/alias-template-conflict.cpp templates/class-template-deduction/inherited-without-using.cpp templates/class-template-deduction/inherited-not-deducible.cpp templates/class-template-deduction/selection-explicit-copy.cpp templates/class-template-deduction/specialization-constructors.cpp templates/class-template-deduction/declared-not-defined-no-guide.cpp templates/class-template-deduction/aggregate-dependent-bound.cpp templates/class-template-deduction/aggregate-pack-not-last-element.cpp templates/class-template-deduction/aggregate-pack-length-conflict.cpp templates/class-template-deduction/aggregate-string-element-type.cpp templates/class-template-deduction/alias-template-result-check.cpp templates/class-template-deduction/alias-template-constraints.cpp templates/class-template-deduction/alias-template-rvalue-reference.cpp templates/class-template-deduction/tie-own-constructor-types-differ.cpp templates/class-template-deduction/guide-template-same-parameters.cpp templates/class-template-deduction/guide-constraints-none-satisfied.cpp templates/class-template-deduction/guide-constraints-absent-ambiguous.cpp templates/class-template-deduction/inherited-copy-candidate-user-guide.cpp templates/class-template-deduction/aggregate-base-no-elision.cpp

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
:tests: templates/class-template-deduction/result-then-initialize.cpp templates/class-template-deduction/result-then-initialize-deleted.cpp templates/class-template-deduction/selection-private-constructor.cpp templates/class-template-deduction/inherited-copy-candidate.cpp

The initialization with the deduced type must be valid. Its own rules
apply, such as {rule}`CTOR-25`, {rule}`DEL-1`, and {rule}`ACC-3`.
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

short n = 1;
Box e(n);                         // Box<short>: an exact match beats the user guide

template <class... B> struct Mix : B... { int x; };
Mix m{5};                         // Mix<>: the pack before x takes no element
```

```cpp
template <class T> struct Size {
  T value;
  constexpr Size(T v) : value(v) {}
};
auto* made = new Size(2.5);       // Size<double>*
auto copy = Size('c');            // Size<char>: a functional cast
template <Size S> struct Fixed {};
Fixed<4> four;                    // the parameter S has type Size<int>
Size x(1), y(2);                  // both Size<int>
Size u(1), v(2.0);                // rejected: Size<int> and Size<double>
```

```cpp
template <class T, class U = T> struct Pair { constexpr Pair(T, U) {} };
Pair<int> p(1, 2.5);              // Pair<int, int>: U takes its default

template <class T> struct Box { template <class U> constexpr Box(U) {} };
constexpr Box<int> one(5);
Box two(one);                     // Box<int>: only the copy candidate gives T
Box three(3);                     // rejected: no guide gives T

template <class T> struct Range { constexpr Range(T*, T*) {} };
int data[3];
Range r(data, data + 3);          // Range<int>: each pointer gives T = int

template <class T = int> struct Tag {};
Tag none;                         // Tag<int>: the guide with no parameters
```

An alias that fixes an argument gives a guide whose parameter has the
fixed type (CTAD-14). The argument for that parameter converts, and
deduction does not use it:

```cpp
template <class T, class U> struct Duo { Duo(T, U); };
template <class U> using IntDuo = Duo<int, U>;
IntDuo d(1L, 'c');                // Duo<int, char>: the guide is IntDuo(int, U)
```

The guides from `Mark(T)` and from the constructor template tie for
`Mark m(1);`. The parameter `U*` uses its default argument, so it takes
no part in partial ordering ({rule}`ORD-5`), and CTAD-22 decides. For a
string literal, the reference binding of `T&&` and the array-to-pointer
conversion to `const char*` both have exact-match rank
({rule}`ICS-2`). The guide that is not a template then wins
({rule}`OVL-27`).

```cpp
template <class T> struct Mark {
  Mark(T);
  template <class U = int> Mark(T, U* = nullptr);
};
Mark m(1);                        // Mark<int>: the guide from Mark(T)

template <class T> struct Text { template <class U> Text(U); };
template <class T> Text(T&&) -> Text<T>;
Text(const char*) -> Text<long>;
Text t("ab");                     // Text<long>

template <class T> struct Base { T v; };
template <class T> struct Wrap : Base<T> { T w; };
Wrap x{{1}, 2};                   // Wrap<int>: the braces of Base<int> stay
Wrap y{1, 2};                     // rejected: Base<T> from 1

template <class... T> struct List {};
struct X {}; struct Y {}; struct Z {};
template <class... T> struct Bases : List<T...>, T... {};
Bases b{List<X, Y, Z>{}, X{}};    // Bases<X, Y, Z>: Y and Z get empty lists
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 94
of 113 tests.

The questions of the cluster of EDG differences (cluster 5) on this page
stay open under decision 0051: {question}`Q-0240`, {question}`Q-0277`,
{question}`Q-0278`, {question}`Q-0279`, and {question}`Q-0280`. On
2026-11-03, Clang 22 and GCC 16 still agree with the provisional answer
of each, and EDG 7.0 still differs. Reporting these EDG differences to
the EDG project waits on the maintainer. The source-reader gap review
of the cluster (`reports/review/2026-11-03-gap-review-question-cluster-5.md`)
reads two answers against every compiler. In Q-0277 it reads the first
form as giving `Three<char>`, and in Q-0280 it reads the two guide
templates as not valid. These two parts wait on the maintainer.

The clean-room writer probed each step with Clang 22, GCC 16, and EDG
7.0, and checked the tests and the grid on the eight
{ref}`baseline compilers <baseline-compilers>`.

- CTAD-14: Clang 18 does not deduce through an alias template. Clang
  22 and GCC deduce through each alias in the table below. EDG 7.0
  deduces when each argument for a fixed position has exactly the
  fixed type, and rejects an argument that only converts to it
  ({question}`Q-0240`).
- CTAD-16 and CTAD-19: Clang makes no guides from inherited
  constructors, also no inherited copy candidate. GCC and EDG make them
  ({question}`Q-0239`). On Clang, the tie-breaker tests of CTAD-19 give
  the same results with no inherited guide.
- CTAD-12: GCC accepts a guide whose result has a cv-qualifier. Clang
  and EDG reject it ({question}`Q-0242`).
- CTAD-13: Clang rejects a guide with a template parameter that no
  parameter of the guide gives a value. GCC and EDG accept the guide.
  The gap review (item 38) reads the rule as one that requires a
  diagnostic, and Clang agrees. The rule is a `reject` rule
  ({question}`Q-0241`).
- CTAD-12: Clang and GCC reject two guides that are not templates with
  the same parameter types and different results. EDG accepts them.
  Every compiler accepts two guide templates with the same parameters
  and different results, and a use that both can take is ambiguous.
  The gap review (item 37) predicted a rejection of both pairs
  ({question}`Q-0280`).
- CTAD-1: every compiler deduces through a template template
  parameter, and through an alias of one, after substitution. The gap
  review (item 20) predicted a rejection ({question}`Q-0275`). With the
  keyword `template` after a dependent qualifier, Clang rejects the
  deduced name, and GCC and EDG accept it ({question}`Q-0243`).
- CTAD-10: the gap review (item 26) predicted that a string literal
  for an array with a dependent element type keeps its braces. Every
  compiler elides them, and `{"ab"}` for `T v[3]` gives
  `T = const char*`. For a fixed `char v[3]` before a dependent member,
  EDG forms no candidate from `{"ab", 1}`, and Clang and GCC deduce
  ({question}`Q-0277`).
- CTAD-27: if the first base fixes the length of the trailing pack,
  Clang and EDG reject a last element that only converts to its base.
  GCC accepts it ({question}`Q-0276`).
- CTAD-28: through an alias, EDG lets `T&&` of a class parameter take
  an lvalue. Clang and GCC reject it ({question}`Q-0278`). The gap
  review (item 30) predicted the check of the result after deduction.
  Without the alias, the same initializer deduces the type that the
  check removes.
- CTAD-18: through an alias, EDG uses the initializer-list guide for
  one element of the class type. Clang 22 and GCC skip that phase
  ({question}`Q-0279`).
- CTAD-19: GCC and EDG reject a tie between a guide from
  `Derived(const T&)` and an inherited guide from `Base(T)` as
  ambiguous. Clang has no inherited guide and accepts
  ({question}`Q-0239`).
- CTAD-16: every compiler deduces `Derived<int>` from an object of
  `Base<int>` and then rejects the initialization
  (`inherited-copy-candidate.cpp`). GCC and EDG take the type from the
  inherited copy candidate. Clang takes the type from the aggregate
  candidate, because Clang treats `Derived` as an aggregate. With a
  user guide, which removes the aggregate candidate (CTAD-10), Clang
  finds no guide, and GCC and EDG still deduce `Derived<int>`
  (`inherited-copy-candidate-user-guide.cpp`).
- CTAD-29: every compiler skips a guide whose constructor constraint
  fails. The same class without constraints makes the call ambiguous.
- CTAD-11: every compiler takes a guide declared after the definition
  of a function template and before its point of instantiation. The
  rule holds also for a deduction that depends on no template
  parameter. The gap review (item 39) asked for the record. In
  `guide-before-instantiation.cpp`, `later<int>` is used before the
  guide for `Cell` and again after it. Every compiler uses the guides
  before the first point of instantiation and deduces `Cell<int>` in
  both uses.
- CTAD-27: Clang 18 stops with an internal error when the list has
  only the element for `List<X, Y, Z>`. The test
  `aggregate-pack-length-fewer.cpp` gives one more element, and every
  compiler accepts it.
- CTAD-9: GCC forms no aggregate candidate for a parenthesized list in
  a `new` expression. Clang and EDG form it ({question}`Q-0244`).
- CTAD-1: the inventory (item G7) predicted that a dependent qualifier
  before the name is not valid. Every compiler deduces through
  `typename Outer<T>::In` after instantiation ({question}`Q-0243`).
- CTAD-24 is the reject rule of the tests in which no guide is chosen.
  Clang names deduction in its message. GCC reports a deduction failure,
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

The alias templates of CTAD-14 in the tests. Each row gives the type
of the alias, the initializer, and the result. "Converts" means that
an argument for a fixed position has another type than the fixed one:

| Alias | Initializer | Clang 22, GCC | EDG 7.0 | Test |
|---|---|---|---|---|
| `Pair<T, T>` | `(1, 2)` | `Pair<int, int>` | `Pair<int, int>` | `alias-template.cpp` |
| `Box<T>` | `(&n)`, user guide `Box(T*)` | `Box<int>` | `Box<int>` | `alias-template-guides.cpp` |
| `Box<T*>` | `(&n)` | `Box<int*>` | `Box<int*>` | `alias-template-unreachable.cpp` |
| `Agg<int, U>`, an aggregate | `{1, 'c'}` | `Agg<int, char>` | `Agg<int, char>` | `alias-template-guides.cpp` |
| `Pair<int, U>` | `(1, 'c')` | `Pair<int, char>` | `Pair<int, char>` | `alias-template-fixed-argument-exact.cpp` |
| `Pair<int, U>` | `(1L, 'c')`, converts | `Pair<int, char>` | rejects | `alias-template-fixed-argument.cpp` |
| `Both<long, U>`, an alias of an alias | `(1L, 'c')` | `Pair<long, char>` | `Pair<long, char>` | `alias-template-fixed-argument-exact.cpp` |
| `Both<long, U>`, an alias of an alias | `(1, 'c')`, converts | `Pair<long, char>` | rejects | `alias-template-fixed-argument.cpp` |

Clang 18 rejects every row.

The grid is in `tests/templates/class-template-deduction/grids/`. This
table gives the source of each rule. Inventory items have the numbers
of `reports/review/2026-10-31-inventory-packs-forwarding-ctad.md`, and
gap review items the numbers of
`reports/review/2026-10-31-gap-review-packs-and-ctad.md`:

| Rule | Source |
|---|---|
| CTAD-1 | grid forms; probes; inventory G1, G3, G5, G7; gap review 19 to 21 |
| CTAD-2 | diagnostics; inventory G2, G6 |
| CTAD-3 | diagnostics; inventory G4 |
| CTAD-4 | probes; inventory G8 |
| CTAD-5 | grid forms; probes; clang-diag-sema (the note on an implicit guide); inventory D1, D4; gap review 22, 24 |
| CTAD-6 | probes; inventory D3; gap review 23, 41 |
| CTAD-7 | grid forms; probes; inventory D2 |
| CTAD-8 | probes; inventory D5, D6, P8; gap review 25 |
| CTAD-9, CTAD-10 | grid forms; probes; inventory C1 to C12; gap review 26, 29 |
| CTAD-11 | grid forms; probes; inventory H2, H3, H5 to H7; gap review 39 |
| CTAD-12 | diagnostics; clang-diag-sema; inventory H1, H3, H8, H9; gap review 37 |
| CTAD-13 | diagnostics; inventory H4; gap review 38 |
| CTAD-14, CTAD-15 | grid forms; probes; diagnostics; inventory B1 to B8; gap review 20 |
| CTAD-16 | grid forms; probes; inventory E1 to E4; gap review 33 |
| CTAD-17 | grid forms; probes; inventory F1, F3 |
| CTAD-18 | probes; inventory F4; gap review 36 |
| CTAD-19 to CTAD-22 | probes; inventory F7 to F10; gap review 34, 35 |
| CTAD-27 | probes; inventory C8, C10; gap review 27, 28 |
| CTAD-28 | probes; diagnostics; inventory B1; gap review 30 to 32 |
| CTAD-29 | probes from the constraints repairs; diagnostics |
| CTAD-23 | grid forms; diagnostics; inventory F2, H2 |
| CTAD-24 | diagnostics in the `tidal-error` lines of its tests |
| CTAD-25, CTAD-26 | probes; diagnostics; inventory F5, F6 |

Inventory item D1 is in part left out. CTAD-5 copies the defaults of a
constructor template, and `constructor-guides.cpp` tests one. The
inventory's doubt about a default that names a class parameter has no
probe.

Inputs: clang-diag-sema.

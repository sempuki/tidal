# Explicit and partial specialization

A program can give its own definition for some specializations of a
template. An {term}`explicit specialization` defines one specialization,
for one fixed argument list. A {term}`partial specialization` defines a
pattern of argument lists for a class template or variable template.
This page covers:

- the declarations of both kinds,
- where and when the declarations can appear,
- how a specialization of a class template or variable template is
  selected from the template and its partial specializations.

{doc}`ordering` gives the order between two partial specializations
that both match ({rule}`ORD-14`).

Depends on: template declarations, template arguments.

## Principles

:::{principle} TSP-P1
:rules: TSP-1 TSP-2 TSP-3 TSP-9 TSP-10 TSP-18 TSP-22 TSP-25 TSP-28 TSP-30
:exceptions: TSP-7 TSP-20 TSP-23 TSP-27

Without a {term}`specialization` of its own, a specialization comes from the
definition of the template, the {term}`primary template`, with the
arguments in place of the parameters ({principle}`INST-P1`). A program
can replace that definition:

| Kind of template | Explicit specialization | Partial specialization |
|---|---|---|
| {term}`class template` | yes (TSP-1) | yes (TSP-9) |
| {term}`variable template` | yes (TSP-2) | yes (TSP-10) |
| {term}`function template` | yes (TSP-2) | no (TSP-7) |
| {term}`alias template` | no (TSP-7) | no (TSP-7) |
| member of a class template: a member function, `static` data member, member class, or member enumeration | yes (TSP-3) | no |
| {term}`member template` of a class template | yes (TSP-3) | as for its kind |

An explicit specialization or a partial specialization is a separate
definition. Its members need not match those of the primary template,
and it takes no `inline` or `constexpr` from the template (TSP-25). The
members of an explicitly specialized class are members of an ordinary
class (TSP-22).
:::

:::{principle} TSP-P2
:rules: TSP-13 TSP-14 TSP-15 TSP-21
:exceptions: TSP-16

A use of a specialization of a class template or a variable template
selects its definition in this order:

```text
argument list, with defaults added (TARG-9)
   │
   ├─ an explicit specialization for this list?  ──► use it
   │
   ├─ partial specializations that match (TSP-14):
   │     none ──► the primary template
   │     one  ──► that partial specialization
   │     more ──► the most specialized one (TSP-15),
   │              or no valid result (TSP-16)
```

A partial specialization matches when some values for its own
parameters make its argument list the same as the given list. The
values come from the given arguments by {term}`deduction`
({principle}`DED-P1`). The grid in Evidence shows which argument types
match which patterns.
:::

:::{principle} TSP-P3
:rules: TSP-4 TSP-5 TSP-6 TSP-8 TSP-11 TSP-12 TSP-19 TSP-24 TSP-26 TSP-31 TSP-32
:exceptions: TSP-17

A specialization changes the meaning of every use of its argument
lists, so the program must declare it in a place that all such uses can
see:

- in the namespace of the template, or in an enclosing namespace with
  a qualified name,
- in the class of a member template,
- before the first use that needs the definition.

A partial specialization must be able to match some argument list that
the primary template does not cover by itself. So its argument list
uses each of its parameters in a form that deduction can find, and the
list is not just the primary template's parameters again.

A member of a nested class template is specialized from the outside
in. The declaration fixes every enclosing class template, with one
`template <>` for each (TSP-26).
:::

:::{principle} TSP-P4
:rules: TSP-29

An explicit or partial specialization often names a private type of a
class, for example to give the definition for that type. So the
implementation does not check access to the names in the declaration of
the specialization. The body, the bases, and the initializers are
checked as usual.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   explicit_specialization: "template" "<" ">" `declaration`
```

The `declaration` after `template <>` declares a class, function, or
variable whose name is a `template_id`, or a member of such a class. A
partial specialization is a `template_decl` whose class or variable has
a `template_id` as its name. The parameters of the partial
specialization's head appear in that `template_id`.

## Constraints

### Explicit specialization

:::{rule} TSP-1
:kind: required
:tests: templates/specialization/class-explicit.cpp templates/specialization/class-explicit-other-members.cpp

An {term}`explicit specialization` of a class template defines the
class for one argument list. The class has the members of its own
definition only.
:::

:::{rule} TSP-20
:kind: reject
:tests: templates/specialization/class-explicit-declared-only.cpp templates/specialization/class-explicit-declared-only-member.cpp

An explicit specialization of a class template that is declared and not
defined is an incomplete class. A program must not create an object of
it or access its members. {rule}`TYN-7` is the general rule.
:::

:::{rule} TSP-2
:kind: required
:tests: templates/specialization/function-explicit.cpp templates/specialization/variable-explicit.cpp

A function template or a variable template can have explicit
specializations. In an explicit specialization of a function template,
arguments that deduction finds from the function's type can be left out
({rule}`DED-32`).
:::

:::{rule} TSP-4
:kind: reject
:tests: templates/specialization/explicit-needs-header.cpp templates/specialization/explicit-not-template.cpp templates/specialization/explicit-no-matching-template.cpp

An explicit specialization must start with `template <>`. The name must
be a {term}`template-id` of a template that is declared before it. For a
function template, one of the templates with that name must have a
specialization with the declared function type.
:::

:::{rule} TSP-32
:kind: reject
:tests: templates/specialization/explicit-union-key.cpp templates/specialization/partial-union-key.cpp templates/specialization/explicit-class-key.cpp

The class key of an explicit or partial specialization of a class
template must agree with the primary template's. `struct` and `class`
agree with each other, and `union` agrees only with `union`.
:::

:::{rule} TSP-24
:kind: reject
:tests: templates/specialization/explicit-storage-class.cpp templates/specialization/explicit-thread-local.cpp templates/specialization/explicit-thread-local-only.cpp

An explicit specialization must not have a storage class specifier
other than `thread_local` ({question}`Q-0203`).
:::

:::{rule} TSP-25
:kind: required
:tests: templates/specialization/explicit-not-inline.cpp templates/specialization/explicit-not-constexpr.cpp templates/specialization/explicit-thread-local-only.cpp

An explicit specialization is `inline` or `constexpr` only if its own
declaration says so. The template's `inline`, `constexpr`, and
attributes do not carry over to its explicit specializations.
:::

:::{rule} TSP-5
:kind: reject
:tests: templates/specialization/explicit-block-scope.cpp templates/specialization/explicit-other-namespace.cpp

An explicit specialization must not be declared at block scope. At
namespace scope, it must be in the namespace of the template or in a
namespace that encloses that namespace.
:::

:::{rule} TSP-19
:kind: required
:tests: templates/specialization/explicit-enclosing-namespace.cpp templates/specialization/explicit-class-scope.cpp

An explicit specialization can be declared in a namespace that encloses
the template's namespace, with a qualified name. An explicit
specialization of a member template can be declared in the class that
declares the member template ({question}`Q-0171`).
:::

:::{rule} TSP-6
:kind: reject
:tests: templates/specialization/explicit-after-use.cpp templates/specialization/explicit-after-use-function.cpp templates/specialization/explicit-enum-after-use.cpp

An explicit specialization must be declared before the first use of its
specialization that needs a definition, such as a variable of the class
type or a call of the function, in the same {term}`translation unit`. A use of
a class specialization creates the unscoped member enumerations that
the class template defines. So an explicit specialization of such an
enumeration after that use is not valid.
:::

:::{rule} TSP-8
:kind: reject
:tests: templates/specialization/explicit-default-argument.cpp templates/specialization/explicit-default-argument-member-template.cpp templates/specialization/explicit-default-argument-member.cpp templates/specialization/explicit-class-member-default.cpp

The program must not write a default argument ({rule}`DEFARG-1`) in
these declarations:

- a `template <>` declaration that fixes the arguments of a function
  template, also one that is a class member,
- a `template <>` declaration that gives one member function of `C<A>`,
  where `C<A>` itself comes from the primary template.

If the program gives the whole class `C<A>` explicitly, its member
functions are ordinary members and can have default arguments (TSP-22).
:::

:::{rule} TSP-7
:kind: reject
:tests: templates/specialization/alias-explicit.cpp templates/specialization/alias-partial.cpp templates/specialization/function-partial.cpp templates/specialization/function-partial-used.cpp

An alias template must not have an explicit specialization or a partial
specialization. A function template must not have a partial
specialization ({question}`Q-0172`).
:::

### Member specialization

:::{rule} TSP-3
:kind: required
:tests: templates/specialization/member-explicit.cpp templates/specialization/member-template-explicit.cpp templates/specialization/member-template-nested-explicit.cpp templates/specialization/explicit-member-class-member-outside.cpp templates/specialization/explicit-member-class-template-member.cpp templates/specialization/explicit-static-member-definition.cpp templates/specialization/explicit-enum-declared.cpp templates/specialization/explicit-enum-scoped-defined-after-use.cpp templates/specialization/explicit-enum-unscoped-declared-after-use.cpp

A member of a specialization of a class template can have an explicit
specialization while the class does not. These members can have one:

- a `static` data member,
- a member function,
- a member enumeration (TSP-30),
- a member class,
- a member template of any kind: class, function, or `static` data
  member template.

The other members of that specialization come from the class template.
TSP-26 gives the number of `template <>` prefixes.
:::

:::{rule} TSP-26
:kind: reject
:tests: templates/specialization/explicit-member-enclosing-not-specialized.cpp templates/specialization/explicit-member-prefix-count.cpp templates/specialization/member-template-nested-explicit.cpp

An explicit specialization of a member at namespace scope must fix the
arguments of every enclosing class template. It has one `template <>`
for each class template that it fixes, and one for the member if the
member is a template.
:::

:::{rule} TSP-22
:kind: required
:tests: templates/specialization/explicit-class-member-outside.cpp templates/specialization/explicit-member-class-member-outside.cpp templates/specialization/explicit-member-class-template-member.cpp templates/specialization/explicit-class-member-default.cpp

If the program gives a whole class specialization explicitly, that
class is an ordinary class. A definition of its member outside the
class has no `template <>`. The same holds for a member class of a
class specialization, given explicitly. If the program gives a member
class template explicitly as a class template, a definition of one of
its members starts with `template <>` and then that template's head.
:::

:::{rule} TSP-23
:kind: reject
:tests: templates/specialization/explicit-class-member-prefix.cpp templates/specialization/explicit-member-class-template-no-prefix.cpp

A definition of a member of an explicitly specialized class must not
start with `template <>` (TSP-22). A definition of a member of a member
class template that is given explicitly must start with `template <>`.
:::

:::{rule} TSP-27
:kind: reject
:tests: templates/specialization/explicit-implicit-member.cpp templates/specialization/explicit-implicit-destructor.cpp

A program must not explicitly specialize a {term}`special member function` that
the implementation declares implicitly, such as an implicit copy
constructor.
:::

:::{rule} TSP-28
:kind: required
:tests: templates/specialization/explicit-static-member-declaration.cpp templates/specialization/explicit-static-member-template-declaration.cpp templates/specialization/explicit-static-member-definition.cpp

An explicit specialization of a `static` member variable, or of a
`static` member variable template, with an initializer is a definition.
One without an initializer is only a declaration. Empty parentheses
after the name declare a function.
:::

:::{rule} TSP-30
:kind: required
:tests: templates/specialization/explicit-enum-declared.cpp templates/specialization/explicit-enum-after-use.cpp templates/specialization/explicit-enum-scoped-defined-after-use.cpp templates/specialization/explicit-enum-unscoped-declared-after-use.cpp

A member enumeration of a class template can have an explicit
specialization after a use of the class specialization in these cases
({question}`Q-0205`):

- The class template declares the enumeration without enumerators.
- The enumeration is scoped, and no use has needed its definition. A
  use of the class makes only the declaration of a scoped member
  enumeration ({rule}`INST-5`).

An unscoped member enumeration that the class template defines must not
have one after a use of the class specialization.
:::

### Partial specialization

:::{rule} TSP-9
:kind: required
:tests: templates/specialization/class-partial.cpp templates/specialization/class-partial-more-parameters.cpp templates/specialization/partial-redeclared.cpp templates/specialization/partial-member-template.cpp templates/specialization/partial-pack-before-parameter.cpp

A {term}`partial specialization` of a class template has its own
{term}`template head` and an argument list after the class name. The argument
list is a pattern in terms of the head's parameters. These rules also
apply:

- The number of parameters can differ from the primary template's.
- The head can have a pack before other parameters ({rule}`TPL-30`).
- A partial specialization can be declared again and defined later. Two
  declarations with equivalent heads and argument lists declare the
  same partial specialization.
- A partial specialization of a member template can be declared in the
  class, or outside it after the heads of the enclosing templates.
:::

:::{rule} TSP-31
:kind: reject
:tests: templates/specialization/partial-no-primary.cpp templates/specialization/partial-no-primary-variable.cpp

A partial specialization must come after a declaration of its primary
template.
:::

:::{rule} TSP-10
:kind: required
:tests: templates/specialization/variable-partial.cpp templates/specialization/variable-partial-two-parameters.cpp

A variable template can have partial specializations, with the same form
as for a class template.
:::

:::{rule} TSP-18
:kind: required
:tests: templates/specialization/partial-member-outside.cpp templates/specialization/partial-member-not-defined.cpp

A member of a partial specialization can be defined outside the class.
The definition starts with the head of the partial specialization and
names the class with the partial specialization's argument list. The
members of a partial specialization are separate from the primary
template's members. A member that the partial specialization declares
needs its own definition.
:::

:::{rule} TSP-11
:kind: reject
:tests: templates/specialization/partial-default.cpp templates/specialization/partial-same-as-primary.cpp templates/specialization/partial-unused-parameter.cpp templates/specialization/partial-value-expression.cpp templates/specialization/partial-reordered.cpp templates/specialization/partial-dependent-type-value.cpp templates/specialization/partial-pack-not-expanded.cpp templates/specialization/partial-pack-not-last.cpp templates/specialization/partial-placeholder-pattern-variable.cpp templates/specialization/partial-placeholder-pattern.cpp

The head and argument list of a partial specialization must follow
these conditions:

- No parameter has a {term}`default template argument`.
- The argument list is not the primary template's parameter list
  again, unless the partial specialization is {term}`more constrained` than the
  primary template ({rule}`CNS-36`). A {term}`constant parameter` whose type
  fits fewer arguments, such as `auto*` or `int` for `auto`, makes the
  list differ.
- Each parameter appears in the argument list where deduction can find
  it. A constant argument that is an expression of a parameter, such as
  `N + 1`, gives no value for `N`.
- The partial specialization is {term}`more specialized` than the primary
  template ({rule}`ORD-14`, {question}`Q-0174`).
- A constant argument whose parameter type in the primary template uses
  another parameter is not a fixed value, while that other parameter
  stays a parameter of the partial specialization.
- Every pack in an argument is expanded with `...`.
- A {term}`pack expansion` is the last argument.
:::

:::{rule} TSP-12
:kind: reject
:tests: templates/specialization/partial-friend.cpp templates/specialization/explicit-friend.cpp

A friend declaration must not declare a partial specialization or an
explicit specialization.
:::

### Access in specialization declarations

:::{rule} TSP-29
:kind: required
:tests: templates/specialization/explicit-private-argument.cpp templates/specialization/partial-private-argument.cpp templates/specialization/explicit-function-private-argument.cpp

The implementation does not check access to the names in the
declaration of an explicit or partial specialization ({question}`Q-0204`).
These parts are the exceptions, and are checked as usual
({rule}`ACC-2`):

- the body and the member list,
- the base list,
- default arguments and initializers,
- the enumerator list.
:::

### Selecting a specialization

:::{rule} TSP-13
:kind: required
:tests: templates/specialization/select.cpp templates/specialization/select-variable.cpp

The definition of a specialization of a class template or variable
template comes from the first of these that applies:

1. the explicit specialization for its argument list,
2. a partial specialization that matches the list (TSP-14 to TSP-16),
3. the primary template.
:::

:::{rule} TSP-14
:kind: required
:tests: templates/specialization/match-cv.cpp templates/specialization/match-defaults.cpp templates/specialization/match-repeated-parameter.cpp templates/specialization/match-values.cpp templates/specialization/match-template-template.cpp templates/specialization/match-alias.cpp templates/specialization/match-using-declaration.cpp templates/specialization/partial-constant-type-exact.cpp templates/specialization/partial-placeholder-pattern-const.cpp templates/specialization/partial-placeholder-pattern-variable.cpp templates/specialization/partial-placeholder-pattern.cpp

A partial specialization matches an argument list if there are
arguments for its parameters that make its own argument list the same
as the given list ({rule}`TARG-19`). The given list includes the
primary template's defaults. A parameter that appears twice gets the
same argument in both places. A constant argument matches a constant
parameter only if the argument's type fits the parameter's declared
type ({rule}`DED-12`). A use finds the partial specializations
through the primary template, so a use through a using-declaration
also finds partial specializations declared after it.
:::

:::{rule} TSP-15
:kind: required
:tests: templates/specialization/select-most-specialized.cpp templates/specialization/select-most-specialized-pair.cpp

If two or more partial specializations match, the selected one is more
specialized than each of the others ({rule}`ORD-14`).
:::

:::{rule} TSP-16
:kind: reject
:tests: templates/specialization/select-ambiguous.cpp templates/specialization/select-ambiguous-unused.cpp

If two or more partial specializations match an argument list, and none
is more specialized than all the others, a use of that specialization
is not valid. Partial specializations that would be ambiguous for some
argument list are valid until a use selects for such a list.
:::

:::{rule} TSP-21
:kind: required
:tests: templates/specialization/member-template-explicit-primary.cpp templates/specialization/member-template-explicit-partial.cpp

A member template of a class template `C` can have partial
specializations, and the program can give some of them explicitly for
one specialization `C<A>`:

- If the program gives the primary member template explicitly for
  `C<A>`, the member's partial specializations play no role in `C<A>`
  ({question}`Q-0206`).
- If the program gives one partial specialization explicitly for
  `C<A>`, the primary member template and the other partial
  specializations still apply in `C<A>` ({question}`Q-0207`).
:::

## Implementation Requirements

:::{rule} TSP-17
:kind: no-diagnostic
:tests: templates/specialization/partial-after-use.cpp templates/specialization/partial-after-use-variable.cpp

A partial specialization must be declared before the first use that
would select it. An implementation need not diagnose a partial
specialization that comes after such a use ({question}`Q-0173`).
:::

## Examples

```cpp
template <class T> struct Traits { static constexpr int kind = 0; };
template <class T> struct Traits<T*> { static constexpr int kind = 1; };
template <class T> struct Traits<const T*> { static constexpr int kind = 2; };
template <> struct Traits<void*> { static constexpr int kind = 3; };

static_assert(Traits<int>::kind == 0);          // the primary template
static_assert(Traits<int*>::kind == 1);         // T* with T = int
static_assert(Traits<const int*>::kind == 2);   // const T* is more specialized
static_assert(Traits<void*>::kind == 3);        // the explicit specialization
```

A variable template and a function template:

% tidal-example: diverges
```cpp
template <class T> constexpr bool is_array = false;
template <class T, int N> constexpr bool is_array<T[N]> = true;

template <class T> int size(T) { return sizeof(T); }
template <> int size(char) { return 1; }        // for T = char
template <class T> int size<T*>(T*);            // rejected: partial
```

An explicit specialization that is not `constexpr` gives no constant
(TSP-25). A requirement that needs such a constant does not hold
({rule}`CPT-26`):

```cpp
template <class T> constexpr int width(T) { return 0; }
template <> int width(char) { return 1; }        // not constexpr
template <int> struct Fixed { };
template <class T> concept Constant = requires { typename Fixed<width(T{})>; };
static_assert(Constant<int> && !Constant<char>);
```

Members of class specializations:

% tidal-example: reject
```cpp
template <class T> struct Store {
  int get();
  template <class U> struct Slot { int put(); };
};
template <> int Store<char>::get() { return 1; }        // one member only

template <> struct Store<long> { int get(); };          // the whole class
int Store<long>::get() { return 2; }                    // no template <>

template <> template <> int Store<int>::Slot<int>::put() { return 3; }
template <class T> template <> int Store<T>::Slot<int>::put();   // rejected
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 80
of 95 tests.

Four questions on this page settled on 2026-11-03 under decision 0054 after the source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`). For each, the review supports the answer with no doubt, and a principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0171` | TSP-P3 | GCC 16 |
| {question}`Q-0173` | TSP-P3 | none; the rule needs no diagnostic |
| {question}`Q-0204` | TSP-P4 | GCC 16 |
| {question}`Q-0207` | TSP-P2 | GCC 16 |

Two questions on this page settled on 2026-11-03 under decision 0054:
{question}`Q-0172`, where TSP-P1 gives the answer, and
{question}`Q-0292`, where TSP-P3 gives it. For each, the source-reader
gap review of question cluster 5 supports the answer with no doubt. EDG
7.0 still differs on both, and the tests keep their overrides for EDG.
{question}`Q-0203` stays open, because the same review reads it with
doubt. On 2026-11-03, Clang 22 and GCC 16 still agree with its
provisional answer, and EDG 7.0 still differs. Reporting these EDG
differences to the EDG project waits on the maintainer
(`documentation/decisions/pending.md`).

The clean-room writer probed explicit and partial specializations with
Clang 22, GCC 16, and EDG 7.0, and checked the tests on the eight
baseline compilers. The gap review of the chapter
(`reports/review/2026-10-31-gap-review-templates-n1.md`) gave the
probes for TSP-21 to TSP-32 and for the additions to the older rules.
The matching rule follows the sanitized Clang
deduction source: the deduced values are put into the partial
specialization's arguments, and the result must equal the given list.

- TSP-19: GCC rejects an explicit specialization in the class that
  declares the member template. Clang and EDG accept it
  ({question}`Q-0171`).
- TSP-7: EDG accepts a partial specialization of a function template
  until a call uses the template, and then rejects the call. Clang and
  GCC reject the declaration ({question}`Q-0172`).
- TSP-17: Clang accepts a partial specialization after a use that it
  would have matched, and keeps the primary template for that use. GCC
  and EDG reject the program. For a variable template, GCC rejects and
  Clang and EDG accept. The rule needs no diagnostic
  ({question}`Q-0173`).
  Every test of TSP-17 has an override, and each override records a
  result that the class allows.
- TSP-11: Clang rejects a partial specialization that only reorders the
  parameters, as no more specialized than the primary template. GCC and
  EDG accept it ({question}`Q-0174`).
- TSP-14: a partial specialization with a constant parameter `auto* P`
  matches only pointer arguments on every compiler. For a parameter
  `int V` and the arguments `1L` and `'c'`, and for `const auto* P` and
  an `int*`, GCC and EDG use the primary template. Clang converts the
  argument and matches, for a wider type such as `long` and a narrower
  type such as `char`. Clang 18 does not convert for `const auto*`, and it takes
  the variable template `kind<P>` for the primary's list
  ({question}`Q-0295`).
- TSP-6: all compilers reject an explicit specialization after a use in
  the same translation unit. {rule}`ODR-14` states the rule for TSP-6
  and TSP-17 across translation units.
- TSP-21: Clang 18 uses a member template's partial specialization for a
  class specialization whose primary member template is given
  explicitly. Clang 22, GCC, and EDG use the explicit specialization
  ({question}`Q-0206`). GCC ignores an explicit specialization of one
  member partial specialization. Clang and EDG select it
  ({question}`Q-0207`).
- TSP-24: EDG rejects a `thread_local` explicit specialization of a
  variable template that is not `thread_local`. Clang and GCC accept it
  ({question}`Q-0203`).
- TSP-29: GCC checks access in the function type of an explicit
  specialization of a function template. Clang and EDG do not
  ({question}`Q-0204`).
- TSP-30: GCC rejects every explicit specialization of a member
  enumeration under `-pedantic-errors`. Clang and EDG accept one for an
  enumeration that the class template only declares. All compilers
  reject one for an unscoped enumeration that the class template
  defines, Clang and EDG for that reason. Clang and EDG also accept one
  after a use for a scoped enumeration that the class template defines
  ({question}`Q-0205`).
- TSP-28: the declaration without an initializer defines nothing, so the
  link fails on every compiler that links.
- TSP-25: the template is `inline` and its explicit specialization is
  not, so two translation units that define the specialization fail to
  link on every compiler that links.

The probe grid is in `tests/templates/specialization/grids/`. Each
column declares one partial specialization of `Match`, and each row
asks whether it matches one argument. An `accept` means that the
partial specialization is selected. The compilers agree on every cell:

| | T* | const T | const T* | T& | T&& | T[N] | T[] | R(A) | R(*)(A) | T C::* | Box<T> | Box<T*> | C<T> | Pair<T, T> |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int* | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int* | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int* const | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int& | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int&& | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int[3] | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject |
| int[] | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject |
| int() | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int(char) | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject |
| int(*)(char) | accept | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| int Owner::* | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject |
| Box<int> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | accept | reject |
| Box<int*> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject |
| Pair<int, int> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| Pair<int, char> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |

The row `int()` matches no column because `R(A)` needs one parameter.
`C<T>` matches every specialization of a template with one type
parameter.

Inputs: clang-diag-sema, clang-diag-parse, clang-sema-deduction.

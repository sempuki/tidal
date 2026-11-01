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

The order between two partial specializations that both match belongs
with deduction and partial ordering *(templates referenced)*. The page
gives the selection up to that point.

Depends on: template declarations, template arguments.

## Principles

:::{principle} TSP-P1
:rules: TSP-1 TSP-2 TSP-3 TSP-9 TSP-10 TSP-18
:exceptions: TSP-7 TSP-20

Without a specialization of its own, a specialization comes from the
definition of the template, the {term}`primary template`, with the
arguments in place of the parameters *(templates referenced)*. A program
can replace that definition:

| Kind of template | Explicit specialization | Partial specialization |
|---|---|---|
| class template | yes (TSP-1) | yes (TSP-9) |
| variable template | yes (TSP-2) | yes (TSP-10) |
| function template | yes (TSP-2) | no (TSP-7) |
| alias template | no (TSP-7) | no (TSP-7) |
| member of a class template | yes (TSP-3) | as for its kind |

An explicit specialization or a partial specialization is a separate
definition. Its members need not match those of the primary template.
:::

:::{principle} TSP-P2
:rules: TSP-13 TSP-14 TSP-15
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
values come from the given arguments by deduction
*(templates referenced)*. The grid in Evidence shows which argument
types match which patterns.
:::

:::{principle} TSP-P3
:rules: TSP-4 TSP-5 TSP-6 TSP-8 TSP-11 TSP-12 TSP-19
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
:tests: templates/specialization/class-explicit.cpp

An {term}`explicit specialization` of a class template defines the
class for one argument list. The class has the members of its own
definition only.
:::

:::{rule} TSP-20
:kind: reject
:tests: templates/specialization/class-explicit-declared-only.cpp

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
*(templates referenced)*.
:::

:::{rule} TSP-3
:kind: required
:tests: templates/specialization/member-explicit.cpp templates/specialization/member-template-explicit.cpp

A member of a specialization of a class template can have an explicit
specialization while the class does not. These members can have one:

- a function member,
- a `static` data member,
- a member template, with one `template <>` for the class and one for
  the member.

The other members of that specialization come from the class template.
:::

:::{rule} TSP-4
:kind: reject
:tests: templates/specialization/explicit-needs-header.cpp templates/specialization/explicit-not-template.cpp templates/specialization/explicit-no-matching-template.cpp

An explicit specialization must start with `template <>`. The name must
be a template-id of a template that is declared before it. For a
function template, one of the templates with that name must have a
specialization with the declared function type.
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
:tests: templates/specialization/explicit-after-use.cpp templates/specialization/explicit-after-use-function.cpp

An explicit specialization must be declared before the first use of its
specialization that needs a definition, such as a variable of the class
type or a call of the function, in the same translation unit.
:::

:::{rule} TSP-8
:kind: reject
:tests: templates/specialization/explicit-default-argument.cpp

The parameters of an explicit specialization of a function template get
no default arguments ({rule}`DEFARG-1`). The program must not write
one.
:::

:::{rule} TSP-7
:kind: reject
:tests: templates/specialization/alias-explicit.cpp templates/specialization/alias-partial.cpp templates/specialization/function-partial.cpp templates/specialization/function-partial-used.cpp

An alias template must not have an explicit specialization or a partial
specialization. A function template must not have a partial
specialization ({question}`Q-0172`).
:::

### Partial specialization

:::{rule} TSP-9
:kind: required
:tests: templates/specialization/class-partial.cpp templates/specialization/class-partial-more-parameters.cpp

A {term}`partial specialization` of a class template has its own
template head and an argument list after the class name. The argument
list is a pattern in terms of the head's parameters. The number of
parameters can differ from the primary template's.
:::

:::{rule} TSP-10
:kind: required
:tests: templates/specialization/variable-partial.cpp

A variable template can have partial specializations, with the same form
as for a class template.
:::

:::{rule} TSP-18
:kind: required
:tests: templates/specialization/partial-member-outside.cpp

A member of a partial specialization can be defined outside the class.
The definition starts with the head of the partial specialization and
names the class with the partial specialization's argument list.
:::

:::{rule} TSP-11
:kind: reject
:tests: templates/specialization/partial-default.cpp templates/specialization/partial-same-as-primary.cpp templates/specialization/partial-unused-parameter.cpp templates/specialization/partial-value-expression.cpp templates/specialization/partial-reordered.cpp

The head and argument list of a partial specialization must follow
these conditions:

- No parameter has a default template argument.
- The argument list is not the primary template's parameter list again.
- Each parameter appears in the argument list where deduction can find
  it. A constant argument that is an expression of a parameter, such as
  `N + 1`, gives no value for `N`.
- The partial specialization is more specialized than the primary
  template *(templates referenced)* ({question}`Q-0174`).
:::

:::{rule} TSP-12
:kind: reject
:tests: templates/specialization/partial-friend.cpp

A friend declaration must not declare a partial specialization.
:::

### Selecting a specialization

:::{rule} TSP-13
:kind: required
:tests: templates/specialization/select.cpp

The definition of a specialization of a class template or variable
template comes from the first of these that applies:

1. the explicit specialization for its argument list,
2. a partial specialization that matches the list (TSP-14 to TSP-16),
3. the primary template.
:::

:::{rule} TSP-14
:kind: required
:tests: templates/specialization/match-cv.cpp templates/specialization/match-defaults.cpp templates/specialization/match-repeated-parameter.cpp templates/specialization/match-values.cpp templates/specialization/match-template-template.cpp templates/specialization/match-alias.cpp

A partial specialization matches an argument list if there are
arguments for its parameters that make its own argument list the same
as the given list ({rule}`TARG-19`). The given list includes the
primary template's defaults. A parameter that appears twice gets the
same argument in both places.
:::

:::{rule} TSP-15
:kind: required
:tests: templates/specialization/select-most-specialized.cpp

If two or more partial specializations match, the selected one is more
specialized than each of the others *(templates referenced)*.
:::

:::{rule} TSP-16
:kind: reject
:tests: templates/specialization/select-ambiguous.cpp templates/specialization/select-ambiguous-unused.cpp

If two or more partial specializations match an argument list, and none
is more specialized than all the others, a use of that specialization
is not valid. Partial specializations that would be ambiguous for some
argument list are valid until a use selects for such a list.
:::

## Implementation Requirements

:::{rule} TSP-17
:kind: no-diagnostic
:tests: templates/specialization/partial-after-use.cpp

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

```cpp
template <class T> constexpr bool is_array = false;
template <class T, int N> constexpr bool is_array<T[N]> = true;

template <class T> int size(T) { return sizeof(T); }
template <> int size(char) { return 1; }        // for T = char
template <class T> int size<T*>(T*);            // rejected: partial
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 37
of 41 tests.

The clean-room writer probed explicit and partial specializations with
Clang 22, GCC 16, and EDG 7.0, and checked the tests on the eight
baseline compilers. The matching rule follows the sanitized Clang
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
  and EDG reject the program. The rule needs no diagnostic
  ({question}`Q-0173`).
- TSP-11: Clang rejects a partial specialization that only reorders the
  parameters, as no more specialized than the primary template. GCC and
  EDG accept it ({question}`Q-0174`).
- TSP-6: all compilers reject an explicit specialization after a use in
  the same translation unit. The tests do not cover a use in another
  translation unit.

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

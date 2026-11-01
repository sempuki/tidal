# Packs

A {term}`pack` is a name that stands for a list of zero or more
elements. A {term}`pack expansion` writes a pattern once and gets one
copy of the pattern for each element. This page covers:

- the kinds of pack, and function parameter packs,
- how an expansion makes its list, and when an expansion is not valid,
- the places where an expansion can stand,
- the pack-size operator `sizeof...`,
- packs in lambda captures and in using-declarations,
- forwarding references, and the collapse of references.

Template parameter packs are declared on {doc}`declarations`
({rule}`TPL-30`, {rule}`TPL-36`). Deduction of a pack is on
{doc}`deduction` ({rule}`DED-20`, {rule}`DED-37`). Fold expressions are
on {doc}`/expressions/primary` ({rule}`PRIM-27` to {rule}`PRIM-29`).

Depends on: template declarations, template arguments, deduction,
lambda expressions, derived classes.

## Principles

:::{principle} PACK-P1
:rules: PACK-1 PACK-3 PACK-5 PACK-7 PACK-9 PACK-10 PACK-12 PACK-26 PACK-27
:exceptions: PACK-2 PACK-4 PACK-6 PACK-8 PACK-11 PACK-28

A pack is a list that a template or a lambda gets as a unit. Code
cannot name one element of a pack. Code writes a pattern with the pack
in it, and `...` after the pattern. The pattern is then copied once for
each element:

```text
template <class... T> void f(T*... p)       T is a pack of types, p a pack of parameters

f<int, char>(...)     T = {int, char}
   T*...              int*, char*          one copy of the pattern for each element
   g(p...)            g(p1, p2)            each copy names its own element
   g(p + 1 ...)       g(p1 + 1, p2 + 1)
```

The pack kinds are these:

| Kind | Declared as | An element is | Rule |
|---|---|---|---|
| type pack | `class... T` | a type | {rule}`TPL-30` |
| type pack that expands an outer pack | `Same<T>... U` | a type, with the constraint of one outer element | PACK-26 |
| constant pack | `int... N` | a constant value | {rule}`TPL-36` |
| template pack | `template <class> class... W` | a template | {rule}`TPL-12` |
| function parameter pack | `T... p`, `auto... p` | a function parameter | PACK-1 |
| init capture pack | `[...c = e]` | a member of the closure | PACK-18 |

An expansion keeps the meaning of the text around it. The list replaces
the expansion in place, and a list of no elements leaves the construct
in its form with nothing in that place (PACK-9). The lengths of the
packs in one pattern must agree, because each copy takes one element of
each pack (PACK-6). A name of a pack outside any pattern has no meaning,
so it is an error (PACK-8). In the substitution into the type of a
function template, such an error removes the template from the
candidates (PACK-27).
:::

:::{principle} PACK-P2
:rules: PACK-13 PACK-14 PACK-16 PACK-17 PACK-18 PACK-19 PACK-21
:exceptions: PACK-15 PACK-20 PACK-25 PACK-29 PACK-30

An expansion can stand only where the grammar has a list. Each place
puts `...` after one item of its list, and the copies become items of
that list:

| Place | Written as | Each copy is | Rule |
|---|---|---|---|
| call arguments | `f(p...)` | an argument | PACK-13 |
| braced list | `{p...}` | an element of the list | PACK-13 |
| template arguments | `List<T...>` | a template argument | PACK-10, {rule}`TSP-9` |
| function parameters | `void f(T... p)` | a parameter | PACK-1 |
| template parameters | `template <T... N>`, `template <Same<T>... U>` | a template parameter | {rule}`TPL-36`, PACK-26 |
| base list | `struct D : B...` | a base class | PACK-14 |
| member initializers | `D() : B(0)...` | a member initializer of a base | PACK-14 |
| `alignas` | `alignas(T...)` | an alignment | PACK-16 |
| lambda captures | `[p...]`, `[&p...]`, `[...c = p]` | a capture | PACK-17, PACK-18 |
| using-declaration | `using B::f...;` | a using-declarator | PACK-21 |
| fold expression | `(p + ...)` | an operand | {rule}`PRIM-27` |

`sizeof...(p)` counts the elements and needs no expansion (PACK-19).
`sizeof...` and a fold use up their packs, so an outer pattern cannot
expand the same pack through them (PACK-8).
:::

:::{principle} PACK-P3
:rules: PACK-22 PACK-23 PACK-24
:exceptions: PACK-31

A function template often passes its arguments on to another
function. A {term}`forwarding reference` `T&&` keeps the value category
of its argument in the deduced `T`. A reference added to a reference
type then collapses into one reference ({rule}`DCL-4`):

| Argument | `T` ({rule}`DED-8`) | Parameter `T&&` | `static_cast<T&&>(x)` |
|---|---|---|---|
| lvalue of type `A` | `A&` | `A&` | lvalue |
| xvalue or prvalue of type `A` | `A` | `A&&` | xvalue |

The cast follows `T`: an lvalue reference type `T` gives an lvalue, and
any other `T` gives an xvalue (PACK-23). Deduction sets `T` from the
argument, so the cast gives the argument back with its value category.
A pack `T&&... x` does the same for each element on its own (PACK-22).
`auto&&` follows the same table ({rule}`AUTO-4`), and the cast then
names `decltype(x)`.
:::

## Constraints

### Function parameter packs

:::{rule} PACK-1
:kind: required
:tests: templates/packs/function-pack-forms.cpp templates/packs/function-pack-array-and-unnamed.cpp

A function parameter whose declarator has `...` before its name, or at
the end with no name, is a {term}`function parameter pack`. The type of
the parameter is a pattern ({term}`pack expansion`). The pattern can be
any declarator, and it names a template parameter pack or holds `auto`.
:::

:::{rule} PACK-2
:kind: reject
:tests: templates/packs/declarator-not-parameter.cpp templates/packs/function-pack-names-no-pack.cpp templates/packs/unnamed-pack-in-parentheses.cpp

Only a function parameter or a template parameter can have `...` in its
declarator. The type of a function parameter pack must name a template
parameter pack or hold `auto`. A pack with no name must not have its
`...` inside parentheses, as in `R (*...)()`.
:::

:::{rule} PACK-3
:kind: required
:tests: templates/packs/ellipsis-after-parameter.cpp templates/packs/placeholder-named-before-ellipsis.cpp templates/packs/named-parameter-before-ellipsis.cpp

A parameter can have `...` after it with no comma before the `...`. The
parameter is then a function parameter pack if both of these hold:

- The parameter has no name.
- The type of the parameter names a pack that is not expanded, or holds
  `auto`.

Otherwise the `...` is the variable argument part of the parameter list
({rule}`DCL-9`), and a pack in the type stays unexpanded (PACK-8).
:::

:::{rule} PACK-4
:kind: reject
:tests: templates/packs/function-pack-default.cpp templates/packs/function-pack-default-placeholder.cpp

A function parameter pack must not have a default argument. Parameters
before the pack can have default arguments.
:::

### Packs of member templates

:::{rule} PACK-26
:kind: required
:tests: templates/packs/member-template-pack-expansion.cpp templates/packs/member-template-pack-length.cpp

A template parameter pack of a member template can expand a pack of an
enclosing template ({rule}`TPL-36`):

- a type parameter pack whose type constraint names the outer pack,
- a template template parameter pack whose own parameter list names the
  outer pack.

Element number i takes its constraint or its parameter list from element
number i of the outer pack. PACK-6 applies to the two lengths.
:::

### Expansion

:::{rule} PACK-5
:kind: required
:tests: templates/packs/expansion-elements.cpp templates/packs/lambda-in-pattern.cpp

A {term}`pack expansion` is a pattern followed by `...`. The expansion
stands for a list with one copy of the pattern for each element, in the
order of the pack. In copy number i, each pack in the pattern stands for
its element number i. A lambda expression in the pattern gives one
closure for each copy, and each closure uses its own element.
:::

:::{rule} PACK-6
:kind: reject
:tests: templates/packs/lengths-differ.cpp templates/packs/member-template-pack-length.cpp

All packs that one expansion expands must have the same number of
elements. PACK-27 gives the case of substitution into the type of a
function template.
:::

:::{rule} PACK-27
:kind: required
:tests: templates/packs/lengths-differ-substitution.cpp templates/packs/void-element-substitution.cpp

If substitution into the type of a function template breaks PACK-6 or
PACK-28, substitution fails ({rule}`DED-24`). The template then adds no
candidate, and overload resolution can choose another candidate.
:::

:::{rule} PACK-7
:kind: required
:tests: templates/packs/nested-expansion.cpp templates/packs/nested-expansion-two-packs.cpp

A pattern can hold another expansion. A pack belongs to the innermost
expansion whose pattern contains it. The outer expansion expands the
packs that no inner expansion takes, and one pack can be expanded in
both.
:::

:::{rule} PACK-8
:kind: reject
:tests: templates/packs/unexpanded-pack.cpp templates/packs/pattern-without-pack.cpp templates/packs/init-capture-unexpanded.cpp templates/packs/size-operator-in-outer-pattern.cpp templates/packs/fold-in-outer-pattern.cpp templates/packs/named-parameter-before-ellipsis.cpp

A use of a pack must be in the pattern of an expansion, or be the
operand of `sizeof...` (PACK-19). The pattern of an expansion must hold
a pack that no inner expansion expands. `sizeof...` and a fold
({rule}`PRIM-27`) count as inner expansions for this rule.
:::

:::{rule} PACK-9
:kind: required
:tests: templates/packs/empty-expansion.cpp templates/packs/empty-expansion-braces.cpp

An expansion of a pack with no elements gives an empty list. The
construct around the expansion keeps its kind. For example, `T(p...)`
with no elements is `T()`, which gives a value of zero to a scalar
({rule}`INIT-2`).
:::

:::{rule} PACK-28
:kind: reject
:tests: templates/packs/void-element-member.cpp templates/packs/void-element-substitution.cpp

An expansion in a function parameter list must not give a parameter of
type `void`. A list of one `void` is empty only in the written form
`(void)` ({rule}`DCL-9`).
:::

:::{rule} PACK-10
:kind: required
:tests: templates/packs/expansion-to-fixed-parameters.cpp templates/packs/expansion-to-fixed-constant-parameters.cpp

An expansion in the argument list of a class template can give
arguments to parameters that are not packs. The arguments are matched
with the parameters after substitution, and {rule}`TARG-6` then
applies.
:::

:::{rule} PACK-11
:kind: reject
:tests: templates/packs/expansion-to-alias-parameters.cpp templates/packs/expansion-to-concept-parameters.cpp

An expansion in the argument list of an alias template or a concept
must not give an argument to a parameter that is not a pack
({question}`Q-0271`, {question}`Q-0272`).
:::

### Expansions in deduction

:::{rule} PACK-12
:kind: required
:tests: templates/packs/nondeduced-before-end.cpp templates/packs/nondeduced-before-end-call.cpp templates/packs/nondeduced-before-end-partial.cpp templates/packs/nondeduced-before-end-function-type.cpp

In every deduction from a type, also in the match of a partial
specialization ({rule}`TSP-14`, {question}`Q-0273`), a list with an
expansion before its end is a {term}`non-deduced position`
({rule}`DED-13`). The list is one of these:

- a template argument list in a type,
- the parameter list of a function type.

A pack that appears only in such a list is empty unless explicit
arguments give it elements. An expansion at the end deduces as
{rule}`DED-37` gives.
:::

### Places of an expansion

:::{rule} PACK-13
:kind: required
:tests: templates/packs/call-and-braced-list.cpp templates/packs/braced-list-order-aggregate.cpp

In the arguments of a call, each copy of the pattern is one argument.
In a braced list, each copy is one element, and the elements are
evaluated from left to right ({rule}`LIST-7`).
:::

:::{rule} PACK-14
:kind: required
:tests: templates/packs/base-list-and-initializers.cpp templates/packs/base-list-order.cpp

An expansion in a base list gives one base for each element, in order.
In a member initializer list, an expansion of a `mem_init` that names a
pack of bases gives one member initializer for each base.
:::

:::{rule} PACK-29
:kind: reject
:tests: templates/packs/base-list-duplicate.cpp templates/packs/base-list-duplicate-fixed.cpp

An expansion in a base list must not give one class twice.
{rule}`DER-1` is the general rule.
:::

:::{rule} PACK-15
:kind: reject
:tests: templates/packs/member-initializer-data-member.cpp templates/packs/member-initializer-data-member-braced.cpp

A `mem_init` that names a data member must not be expanded.
:::

:::{rule} PACK-16
:kind: required
:tests: templates/packs/alignment-specifier.cpp templates/packs/alignment-specifier-class.cpp

`alignas` can hold an expansion of types or of constants. The result is
the strictest alignment of the elements ({rule}`ALN-1`). An expansion
with no elements adds no alignment requirement.
:::

### Lambda captures

:::{rule} PACK-17
:kind: required
:tests: templates/packs/capture-pack.cpp templates/packs/capture-pack-same-mode.cpp templates/packs/capture-default-pack.cpp templates/packs/capture-pack-same-mode-reference.cpp

A capture `p...` captures each element of the function parameter pack
`p` by copy, and `&p...` captures each element by reference. A capture
default captures each element of a pack that the body uses, in the
mode of the default.
:::

:::{rule} PACK-18
:kind: required
:tests: templates/packs/init-capture-pack.cpp templates/packs/init-capture-unexpanded.cpp templates/packs/init-capture-braced-expansion.cpp

An init capture `...c = e`, or `&...c = e`, declares one capture for
each element that the initializer `e` expands to. The name `c` is a
pack in the body, and with no elements the capture declares nothing. An
init capture with no `...` before its name, such as `c{p...}`, declares
one capture that is not a pack.
:::

### The pack-size operator

:::{rule} PACK-19
:kind: required
:tests: templates/packs/size-operator.cpp templates/packs/size-operator-constant.cpp

`sizeof...(p)` gives the number of elements of the pack `p`. The pack
can be of any kind. The result is a constant of the type of `sizeof`
({rule}`UN-12`), and the pack in the operand needs no `...`.
:::

:::{rule} PACK-20
:kind: reject
:tests: templates/packs/size-operator-pattern.cpp templates/packs/size-operator-not-pack.cpp templates/packs/size-operator-parentheses.cpp

The operand of `sizeof...` must be the name of a pack in parentheses. A
pattern, such as `T*`, is not valid there.
:::

### Using-declarations

:::{rule} PACK-21
:kind: required
:tests: templates/packs/using-declaration-pack.cpp templates/packs/using-declaration-empty-pack.cpp

A using-declarator in a class can be expanded over a pack of bases. The
declaration then names the member of each base. For a pack with no
elements, the declaration declares nothing ({rule}`DER-9`,
{rule}`DER-8`).
:::

### Forwarding

:::{rule} PACK-22
:kind: required
:tests: templates/packs/forwarding-pack.cpp templates/packs/forwarding-placeholder-pack.cpp

Each of these function parameter packs is a pack of forwarding
references:

- `T&&... p`, with `T` a type pack of the function template,
- `auto&&... p`.

Each element deduces on its own as {rule}`DED-8` gives. For
`auto&&... p`, `static_cast<decltype(p)>(p)...` forwards each element.
:::

:::{rule} PACK-31
:kind: reject
:tests: templates/packs/forwarding-bit-field.cpp templates/packs/forwarding-const-bit-field.cpp

An lvalue bit-field that is not `const` must not be the argument of a
forwarding reference. Deduction gives a non-`const` lvalue reference,
which must not bind to a bit-field ({rule}`REF-8`).
:::

:::{rule} PACK-23
:kind: required
:tests: templates/packs/forwarding-cast.cpp templates/packs/forwarding-cast-explicit-argument.cpp templates/packs/forwarding-placeholder-pack.cpp

Take a forwarding reference `x` of type `T&&`. The cast
`static_cast<T&&>(x)` gives an lvalue when `T` names an lvalue reference,
and an xvalue in the other cases. With a deduced `T`, the first case
arises for an lvalue argument only ({rule}`DED-8`).
:::

:::{rule} PACK-24
:kind: required
:tests: templates/packs/collapse-paths.cpp templates/packs/collapse-member-alias.cpp

A reference added to a reference type collapses as {rule}`DCL-4` gives.
The rule applies to a type from an alias, a template argument, or
`decltype`. A cv-qualifier on such a reference type has no effect.
:::

### Capture modes of packs

:::{rule} PACK-25
:kind: reject
:tests: templates/packs/capture-pack-same-mode.cpp templates/packs/capture-pack-same-mode-reference.cpp

A pack capture after a capture default must use the other mode, as a
capture of one name must ({rule}`LAM-6`). So `[=, p...]` and
`[&, &p...]` are not valid.
:::

## Implementation Requirements

:::{rule} PACK-30
:kind: no-diagnostic
:tests: templates/packs/using-declaration-empty-pack-outer-name.cpp templates/packs/using-declaration-empty-pack-only.cpp

A use of a name in a class template is not valid if both of these hold:

- At the definition, the name finds a using-declarator expanded over a
  pack (PACK-21).
- The pack of the instantiation has no elements.

An implementation need not diagnose the error ({question}`Q-0274`).
:::

## Examples

```cpp
template <class... T> struct List {};
template <class... T> using Pointers = List<T*...>;
Pointers<int, char>                        // List<int*, char*>

template <class... T> int count(T... x) { return sizeof...(x); }
count();                                   // 0
count(1, 'c');                             // 2

template <class... B> struct Mix : B... {  // one base per element
  using B::run...;                         // each base's run
};

template <class... T> void relay(T&&... x) {
  sink(static_cast<T&&>(x)...);            // each argument keeps its value category
}

template <class... T> auto keep(T... x) {
  return [...c = x] { return (c + ...); }; // one member per element
}

template <class... R> int calls(R (*... fp)());   // a named pack can have its ... in parentheses
template <class... T> void take(T x...);   // rejected: x has a name, so T is not expanded
void first(auto x...);                     // one parameter, then the variable argument part
template <class... T> void call(T...);
call<void>();                              // rejected: a parameter of type void
```

```cpp
template <class... T> struct List {};
template <class... T> void put(List<T..., int>);
put<char>(List<char, int>{});              // T = char from the explicit argument
put(List<char, int>{});                    // rejected: T is empty, and List<int> does not match

struct Flags { const int on : 1; };
template <class T> void pass(T&& x);
const Flags flags{0};
pass(flags.on);                            // T = const int&: the reference binds to a temporary

template <class... B> struct Merge : B... {
  using B::f...;
  int f(double);
};
Merge<>{}.f(1);                            // f(double): the using-declaration declares nothing
```

```cpp
template <class... T> struct Outer {
  template <class... U> struct Inner {
    using type = List<List<T, U...>...>;   // U... inside each copy, T in the outer one
  };
};
Outer<int, char>::Inner<long>::type        // List<List<int, long>, List<char, long>>

template <class... T> int total(T... x) {
  int sum = 0;
  int steps[] = {(sum += x)..., 0};        // evaluated from left to right
  return sum;
}
```

```cpp
template <class... A> int make(A... a) { return int(a...); }
make();                                    // 0: int() with no elements

struct Part {};
template <class... B> struct Mix : Part, B... {};
Mix<Part> m;                               // rejected: Part twice in the base list

template <class... T> void keep(T... x) {
  auto f = [&, &x...] {};                  // rejected: & repeats the capture default
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 64
of 68 tests.

The clean-room writer probed each rule with Clang 22, GCC 16, and EDG
7.0, and checked the tests and the grid on the eight
{ref}`baseline compilers <baseline-compilers>`.

- PACK-16: on a class declaration, GCC takes the alignment of the last
  element of `alignas(T...)`. Clang and EDG take the strictest. On a
  member, all compilers take the strictest ({question}`Q-0238`).
- PACK-3: every compiler reads `T...` after a parameter of a type that
  names no pack as the variable argument part. Every compiler also
  reads the `...` after a named parameter as the variable argument
  part, for a pack type and for `auto`.
- PACK-2: the gap review (item 3) predicted that an unnamed pack can
  have its `...` inside parentheses, as in `R (*...)()`. Every
  compiler rejects the form. Clang asks for a name, GCC reports a
  syntax error, and EDG says that an unnamed pack cannot be in
  parentheses. The rule follows the compilers.
- PACK-11: every compiler rejects an expansion for a parameter of an
  alias template that is not a pack. A class template accepts it
  (PACK-10). The gap review (item 7) found no explicit statement of
  this rule in its reference ({question}`Q-0271`). For a concept, Clang
  and GCC reject the expansion, and EDG accepts it ({question}`Q-0272`).
- PACK-12: for a partial specialization with a parameter that appears
  only in a list with an expansion before its end, Clang rejects the
  declaration. GCC and EDG accept it, and the specialization never
  matches. For the parameter `void (*)(T..., int)` and a fallback
  `take(...)`, the compilers differ in two calls ({question}`Q-0273`).
  For `void h(int)`, Clang and GCC deduce an empty `T` and call the
  template, and EDG calls the fallback. For `take<char>(f)` with
  `void f(char, int)`, Clang and EDG call the template, and GCC finds
  no match. Every compiler calls the fallback for `take(f)`, as
  `nondeduced-before-end-function-type.cpp` tests.
- PACK-27: in each test of the rule, every compiler removes the
  template and calls the other overload.
- PACK-30 is `no-diagnostic`. In the probe, `f` in a member function
  finds `using B::f...;` at the definition. For an empty pack, GCC and
  EDG call the `f` of the namespace ({question}`Q-0274`). Clang rejects
  the use of `f` in the class, and its message names the
  using-declaration that expands to nothing. Clang rejects only such a
  use. A call from outside the class that finds the class's own `f` is
  valid on every compiler (`using-declaration-empty-pack.cpp`). The
  gap review (item 15) gives the program no meaning and requires no
  diagnostic, and the class allows both results.
- PACK-18: the gap review (item 13) predicted a rejection for an init
  capture `c{p...}` with two elements. Every compiler rejects it, as a
  braced initializer of a placeholder type with two elements.
- PACK-21: a probe calls `f` of a class whose `using Bs::f...;`
  expands an empty pack, with no other `f`. Every compiler rejects the
  call. Clang names the using-declaration that expands to nothing, and
  GCC and EDG report that the class has no member `f`.
- Inventory item L2: no standard attribute accepts an expansion. Clang
  says the attribute cannot be a pack, and GCC and EDG say the pattern
  has no pack. The page does not specify attribute expansions.

Expansion places (PACK-P2). Each row is a function template `probe`
over a type pack `T` that expands `T` in one place and checks the
result. Each column gives `T` a number of elements:

| | no element | one element | three elements |
|---|---|---|---|
| call arguments | accept | accept | accept |
| braced list | accept | accept | accept |
| template arguments | accept | accept | accept |
| function parameters | accept | accept | accept |
| base list | accept | accept | accept |
| member initializers | accept | accept | accept |
| alignment specifier | accept | accept | accept |
| capture | accept | accept | accept |
| init capture | accept | accept | accept |
| using-declaration | accept | accept | accept |
| pack-size operator | accept | accept | accept |
| fold | accept | accept | accept |

The grid is in `tests/templates/packs/grids/`. This table gives the
source of each rule. Inventory items have the numbers of
`reports/review/2026-10-31-inventory-packs-forwarding-ctad.md`, and gap
review items the numbers of
`reports/review/2026-10-31-gap-review-packs-and-ctad.md`:

| Rule | Source |
|---|---|
| PACK-1, PACK-2 | probes; diagnostics; inventory I3 to I5, I9; gap review 3 |
| PACK-3 | probes; diagnostics; inventory I2; gap review 1, 2 |
| PACK-4 | diagnostics; inventory I1 |
| PACK-5 | probes; inventory K1, K11; gap review 12 |
| PACK-6 | diagnostics; inventory K5 |
| PACK-7 | probes; inventory K4, K7 |
| PACK-8 | diagnostics; inventory K6, K10, A6; gap review 1, 5 |
| PACK-9 | probes; grid sites; inventory K2, K3 |
| PACK-10, PACK-11 | probes; diagnostics; inventory K9; gap review 7, 8 |
| PACK-12 | probes; inventory J4; gap review 9 |
| PACK-13 | grid sites; probes; inventory L4, L5 |
| PACK-14, PACK-15 | grid sites; diagnostics; inventory L3, L7 |
| PACK-16 | grid sites; probes; inventory L1 |
| PACK-17, PACK-18, PACK-25 | grid sites; probes; diagnostics; inventory A1 to A7; gap review 13, 14 |
| PACK-19, PACK-20 | grid sites; diagnostics; inventory N1 to N4 |
| PACK-21 | grid sites; probes; inventory O1 to O3 |
| PACK-22, PACK-23 | probes; inventory P4, P7, P10; gap review 16, 17 |
| PACK-24 | probes; inventory P1 |
| PACK-26 | probes; diagnostics; gap review 4 |
| PACK-27 | probes; gap review 6, 10 |
| PACK-28 | diagnostics; gap review 10 |
| PACK-29 | diagnostics; gap review 11 |
| PACK-30 | probes; gap review 15 |
| PACK-31 | diagnostics; gap review 18 |

Inventory items left out of this page:

- L2, an expansion in an attribute list: no standard attribute accepts
  one.
- K8 restates how the other rules treat a pack and adds no behavior.
- Other pages state these items: I1 for template parameters
  ({rule}`TPL-13`, {rule}`TPL-34`), I6 to I8 ({rule}`TPL-30`,
  {rule}`TPL-35`, {rule}`TPL-36`), J1 to J3 and J5 to J10
  ({rule}`DED-2`, {rule}`DED-20`, {rule}`DED-23`, {rule}`DED-37`,
  {rule}`ORD-4`, {rule}`ORD-12`), M1 to M5 ({rule}`PRIM-27` to
  {rule}`PRIM-29`), P2, P3, P5 to P10 ({rule}`DED-8`, {rule}`DED-30`,
  {rule}`AUTO-4`, {rule}`ORD-10`), Q1 ({rule}`DEP-33`), Q3
  ({rule}`TSP-9`, {rule}`TSP-11`), Q4 ({rule}`TPL-20`), and Q5
  ({rule}`TARG-17`).
- Q2, a pack in the argument list of a template for itself, is in part
  {rule}`TPL-43`.

Inputs: clang-diag-sema, clang-diag-parse.

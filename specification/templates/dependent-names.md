# Dependent names

A template body holds names whose meaning depends on the template
arguments, and names whose meaning does not. The implementation reads
the body once at the definition and again for each specialization. This
page covers:

- which types and expressions depend on the template parameters,
- when a name is looked up, at the definition or at the instantiation,
- the {term}`current instantiation` and lookup into it,
- the keywords `typename` and `template` before a dependent name,
- what an implementation checks before any instantiation.

{doc}`instantiation` says when a specialization is made and from which
declarations. Requires expressions and constraints are on
{doc}`concepts` and {doc}`constraints`. Modules are outside the charter.

Depends on: template declarations, template arguments, name lookup,
argument-dependent lookup, class members.

## Principles

:::{principle} DEP-P1
:rules: DEP-5 DEP-6 DEP-7 DEP-8 DEP-9 DEP-10 DEP-28 DEP-38 DEP-40

The implementation reads a template in two phases:

```text
definition                              each instantiation
──────────                              ──────────────────
parse the body                          put the arguments in place of
look up each independent name   ──►     the parameters (DEP-7)
  and bind it (DEP-5, DEP-6)            look up each dependent name
check each independent construct        check each dependent construct
  (DEP-30)                                (DEP-28)
```

A name that does not depend on a parameter means the same thing in
every {term}`specialization`, so the implementation binds the name once, at the
definition. A later declaration does not change the binding. A
{term}`dependent name` can mean something different for each argument
list, so its lookup waits for the arguments.

Only an unqualified call with a dependent argument uses both phases
(DEP-8, DEP-9):

| Call | Ordinary lookup | Argument-dependent lookup | Finds a function declared after the template |
|---|---|---|---|
| `f(x)`, no argument is dependent | at the definition | at the definition | no (DEP-5) |
| `f(t)`, an argument is dependent | at the definition | at the definition and at a {term}`point of instantiation` | yes, in an {term}`associated namespace` (DEP-9) |
| `lib::f(t)`, qualified | at the definition, in `lib` | none | no (DEP-40) |
| `f(t)`, where `f` finds a class member | at the definition | none | no (DEP-40) |

A call whose arguments are all of fundamental type has no associated
namespace ({rule}`NAM-27`), so it finds only what the definition sees
(DEP-10). An operator expression with a type-dependent operand finds
its {term}`operator functions <operator function>` in the same two ways (DEP-9). Each {term}`translation unit`
sees only its own declarations. A better match in another unit
makes the program not valid, with no diagnostic required (DEP-38).
:::

:::{principle} DEP-P2
:rules: DEP-1 DEP-2 DEP-3 DEP-4

A construct depends on the template parameters in one of three ways:

| Kind | Meaning | Examples |
|---|---|---|
| {term}`dependent type` | the type differs between specializations | `T`, `T*`, `Box<T>`, `T[N]`, `typename T::type`, `decltype(t.size())` |
| {term}`type-dependent` expression | the type of the expression differs | `t`, `T(0)`, `t + 1`, `(t + ...)`, `this` in a {term}`class template` |
| {term}`value-dependent` expression | the type is fixed, the value differs | `N`, `sizeof(T)`, `sizeof...(T)`, `static_cast<int>(t)` |

Some forms have a fixed type whatever their operands: `sizeof`,
`sizeof...` ({rule}`PACK-19`), `alignof`, `noexcept`, a literal, and a
cast to an independent type. A call with such an argument is not dependent on that argument, and
the implementation chooses its function at the definition (DEP-3).
`sizeof`, `alignof`, and `noexcept` also have a value that only the
type of the operand decides, so a value-dependent operand does not make
them value-dependent (DEP-4).
:::

:::{principle} DEP-P3
:rules: DEP-11 DEP-12 DEP-14 DEP-36
:exceptions: DEP-13 DEP-35

A base class whose type is dependent is unknown at the definition. An
{term}`explicit specialization` can give the base any members, so the
implementation does not search such a base for an unqualified name. A
program reaches a member of a dependent base through `this->`, through
the name of the class, or through the name of the base:

| Written | Searches a dependent base | Rule |
|---|---|---|
| `count` | no | DEP-11, DEP-12 |
| `this->count` | yes, at the {term}`instantiation` | DEP-11 |
| `Derived::count`, `Base<T>::count` | yes, at the instantiation | DEP-11, DEP-18 |
| `operator T`, with `T` dependent | yes, at the instantiation | DEP-11 |

An unqualified name that finds a member at the definition keeps that
member (DEP-14). A qualified name into the current instantiation, and
a member access through `this`, are looked up again at the
instantiation. An ambiguity then makes the program not valid (DEP-13).
A different member makes the program not valid, with no diagnostic
required (DEP-35).

Unqualified lookup searches the class and its independent bases
before the template parameters, so a member can hide a parameter
(DEP-36).
:::

:::{principle} DEP-P4
:rules: DEP-15 DEP-16 DEP-17 DEP-18 DEP-39

Inside a class template, some names stand for the specialization that
the implementation is making. That specialization is the
{term}`current instantiation`. Its members are known at the definition,
so the implementation looks them up there:

| Inside | Names the current instantiation | Names another specialization |
|---|---|---|
| `template <class T> struct Box` | `Box`, `Box<T>`, `::Box<T>`, `Box<self>` after `using self = T;` | `Box<T*>`, `Box<const T>` |
| `template <int N> struct Box` | `Box`, `Box<N>` | `Box<M>` after `static const int M = N;`, and `Box<0>`, which is not dependent |
| a nested class `Box<T>::Inner` | `Inner`, `Box<T>::Inner` | `Box<T*>::Inner` |
| `template <class T> struct Box<T*>` | `Box`, `Box<T*>`, `Box<pointer>` after `using pointer = T*;` | `Box<T>` |

A member of the current instantiation needs no `typename` (DEP-16). If
the class has no dependent base, a name that the class does not declare
is not valid. The name is not dependent, so the implementation can
report the error at the definition (DEP-17). With a dependent base, the
lookup waits for the instantiation (DEP-18). A member access on an
object whose type depends on a parameter, and is not the current
instantiation, also waits for the instantiation (DEP-39).
:::

:::{principle} DEP-P5
:rules: DEP-19 DEP-20 DEP-21 DEP-22 DEP-23 DEP-24 DEP-25 DEP-26
:exceptions: DEP-27 DEP-34

The implementation parses a template before it knows the arguments. For
a qualified name such as `T::type`, the parser must decide whether the
name is a type, a template, or a value. The program states the choice:

| Written | Read as | Checked at the instantiation |
|---|---|---|
| `T::name` | a value, except in a type-only position (DEP-20) | the name is not a type (DEP-19) |
| `typename T::name` | a type | the name is a type (DEP-22) |
| `T::name<...>`, `x.name<...>` | `name`, then the less-than operator | |
| `x.name<...>`, where `x` has the type of the current instantiation and the class or a base that is not dependent declares the member template `name` | a {term}`template-id` (DEP-24, {question}`Q-0427`) | |
| `typename T::name<...>`, `T::name<...>` in a base list or an alias | not valid without `template` (DEP-24, {question}`Q-0225`) | |
| `T::template name<...>` | a {term}`template-id` | the name is a template (DEP-26) |
| `typename T::template name<...>` | a type | the name is a class template or {term}`alias template` |
| `typename T::template name` with no list, as in `typename T::template box b(1);` | a type whose template arguments are deduced | the name is a class template or alias template (DEP-26, {question}`Q-0226`) |

The middle column is the reading that the parser chooses at the
definition, and the choice reports no error. The right column is
checked at the instantiation, when the lookup of the name has the
arguments.

In a type-only position, only a type can appear, so the parser reads a
qualified name as a type without `typename` (DEP-20). Both keywords can
also appear where the name is not dependent (DEP-21, DEP-25). The
keyword `template` cannot come in the name that a declaration declares
(DEP-27) or before a destructor name (DEP-34).
:::

:::{principle} DEP-P6
:rules: DEP-29 DEP-30 DEP-31 DEP-32 DEP-33 DEP-37

A template that is never instantiated still has to be a template that
some arguments make valid. The implementation checks what it can at the
definition:

| Error in a template that is not instantiated | Requirement | Rule |
|---|---|---|
| a syntax error | rejects | DEP-29 |
| an independent name that lookup does not find | rejects | DEP-6 |
| an unqualified name that only a dependent base declares | rejects | DEP-12 |
| an independent construct that is not valid | need not diagnose | DEP-30 |
| an independent construct whose meaning changes before the instantiation | need not diagnose | DEP-31 |
| a construct that no argument list makes valid | need not diagnose | DEP-32 |
| a pack that every valid argument list leaves empty, such as a base list that expands one pack twice | need not diagnose | DEP-33 |
| an operator function that only an empty pack makes valid | rejects | DEP-37 |
| a template-id with independent arguments that fail its template's constraints | rejects | {rule}`CNS-13` |
| a dependent construct that some argument list makes valid | accepts | DEP-28 |

The condition of a `static_assert` in a template is checked only at the
instantiation, also when the condition is independent ({rule}`SA-5`).
A {term}`discarded statement` is not instantiated ({rule}`IF-13`).
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   typename_specifier: "typename" `nested_name` `identifier`
                     : | "typename" `nested_name` ["template"] `template_id`
```

A `typename_specifier` is a type specifier. The keyword `template` can
also come right after the `::` of a `nested_name`, and right after the
`.` or `->` of a member access, when a `template_id` follows. The
keyword `typename` can start the name of a using-declaration that
names a member of a base.

## Constraints

### Dependent types and expressions

:::{rule} DEP-1
:kind: required
:tests: templates/dependent-names/checked-at-instantiation.cpp templates/dependent-names/dependent-type-function-pack.cpp templates/dependent-names/dependent-type-noexcept.cpp templates/dependent-names/dependent-type-template-id.cpp templates/dependent-names/member-alias-independent.cpp templates/dependent-names/member-of-other-specialization-constant-argument.cpp templates/dependent-names/other-specialization-constant-argument-no-typename.cpp

A type is a {term}`dependent type` if the type is one of these:

- a template parameter,
- a pointer, reference, array, function, or member pointer type built
  from a dependent type, or a dependent type with `const` or
  `volatile`,
- a function type with a parameter pack, or with a `noexcept`
  condition that is {term}`value-dependent`,
- an array type whose bound is value-dependent,
- a template-id whose template is a template parameter, or with an
  argument that is a dependent type, a value-dependent expression, or
  a {term}`pack expansion`,
- `decltype` of a {term}`type-dependent` expression,
- a qualified name of a type whose qualifier is a dependent type other
  than the current instantiation,
- a nested class or enumeration of the {term}`current instantiation`.

A member alias of the current instantiation is a dependent type only if
its target type is dependent.
:::

:::{rule} DEP-2
:kind: required
:tests: templates/dependent-names/dependent-call-late.cpp templates/dependent-names/type-dependent-placeholder-parameter.cpp templates/dependent-names/type-dependent-placeholder-variable.cpp templates/dependent-names/type-dependent-deduced-return.cpp templates/dependent-names/type-dependent-fold.cpp templates/dependent-names/type-dependent-set-and-binding.cpp templates/dependent-names/member-template-through-current-instantiation-object.cpp templates/dependent-names/member-template-current-instantiation-pointer.cpp templates/dependent-names/this-member-independent-type-bound-early.cpp templates/dependent-names/this-in-local-class-not-type-dependent.cpp

An expression is {term}`type-dependent` if one of these holds, and
DEP-3 does not give a fixed type:

- the expression is a name declared with a dependent type,
- the expression names a structured binding whose hidden variable has
  a dependent type,
- the expression names an {term}`overload set` through a dependent qualifier,
  such as `T::make`, or with dependent explicit {term}`template arguments <template argument>`,
  such as `make<T>`,
- the expression names a constant template parameter whose type is a
  {term}`placeholder type`,
- the expression names a variable whose type is a placeholder type and
  whose initializer is type-dependent,
- the expression names a member function of the current instantiation
  whose return type is deduced,
- the expression is `this` in a member of a class template, which
  does not include a member of a local class of a function template,
- the expression is a cast or a `new` whose named type is dependent,
- the expression is a fold expression,
- a member access names a member whose type is dependent,
- an operand or subexpression is type-dependent.

A member access through `this`, or through an object of the current
instantiation, is an exception to the last item. It is type-dependent
only if the member's type is dependent, or if the name finds nothing and
the class has a dependent base ({rule}`DEP-18`,
{question}`Q-0434`).
:::

:::{rule} DEP-3
:kind: required
:tests: templates/dependent-names/independent-call-early.cpp templates/dependent-names/independent-call-pack-size.cpp

These expressions are never type-dependent:

- a literal,
- `sizeof`, `sizeof...` ({rule}`PACK-19`), `alignof`, and the
  `noexcept` operator,
- a `delete` expression and a `throw` expression,
- a cast or a `new` whose named type is not dependent.
:::

:::{rule} DEP-4
:kind: required
:tests: templates/dependent-names/value-dependent-bound.cpp templates/dependent-names/value-dependent-bound-fails.cpp templates/dependent-names/value-dependent-constant-variable.cpp templates/dependent-names/value-dependent-pack.cpp templates/dependent-names/sizeof-value-dependent-operand-call-bound-early.cpp templates/dependent-names/sizeof-value-dependent-operand-no-typename.cpp

An expression is {term}`value-dependent` if one of these holds:

- the expression names a constant template parameter,
- the expression names a `const` integer variable or a `constexpr`
  variable whose initializer is value-dependent,
- the expression is type-dependent,
- the expression is `sizeof`, `alignof`, or `noexcept` of a dependent
  type or a type-dependent operand,
- the expression is `sizeof...` ({rule}`PACK-19`) or a fold
  expression,
- an operand is value-dependent, other than the operand of `sizeof`,
  `alignof`, or `noexcept`
  ({question}`Q-0432`).

The value of such an expression is computed for each specialization.
Each specialization must then obey the rules for that value, such as
the array bound rules of {rule}`DCL-19`.
:::

### Binding of names

:::{rule} DEP-5
:kind: required
:tests: templates/dependent-names/independent-bound-early.cpp templates/dependent-names/independent-call-early.cpp templates/dependent-names/independent-call-pack-size.cpp templates/dependent-names/member-alias-independent.cpp templates/dependent-names/this-member-independent-type-bound-early.cpp templates/dependent-names/sizeof-value-dependent-operand-call-bound-early.cpp

A name that is not dependent is looked up at the definition of the
template. Every specialization uses the declarations found there, and a
later declaration does not change the result. An unqualified call with
no dependent argument uses ordinary lookup and argument-dependent
lookup ({rule}`NAM-27`), both at the definition.
:::

:::{rule} DEP-6
:kind: reject
:tests: templates/dependent-names/independent-not-declared.cpp templates/dependent-names/independent-type-not-declared.cpp templates/dependent-names/independent-error-unused-name.cpp

A name that is not dependent must be declared before the definition of
the template, also when no specialization is made. A declaration
between the definition and the use does not make an instantiation
valid.
:::

:::{rule} DEP-7
:kind: required
:tests: templates/dependent-names/per-specialization.cpp templates/dependent-names/per-specialization-type.cpp

A {term}`dependent name` is looked up again for each specialization,
with the template arguments in place of the parameters. Two
specializations can find different declarations.
:::

:::{rule} DEP-8
:kind: required
:tests: templates/dependent-names/dependent-call-late.cpp templates/dependent-names/dependent-call-pack.cpp templates/dependent-names/dependent-call-template-arguments.cpp templates/dependent-names/dependent-call-braced-list.cpp

The function name of an unqualified call is a dependent name in these
cases:

- an argument is type-dependent,
- an argument is a {term}`pack expansion` ({rule}`PACK-13`),
- an argument is a braced list with a type-dependent element,
- the name has explicit template arguments that are dependent. The
  `<` after the name opens the argument list even if no template of
  that name is visible ({rule}`TARG-23`).
:::

:::{rule} DEP-9
:kind: required
:tests: templates/dependent-names/argument-lookup-later.cpp templates/dependent-names/argument-lookup-better-later.cpp templates/dependent-names/dependent-call-pack.cpp templates/dependent-names/dependent-call-template-arguments.cpp templates/dependent-names/operator-lookup-later.cpp templates/dependent-names/argument-lookup-later-default.cpp templates/dependent-names/dependent-type-function-pack.cpp templates/dependent-names/dependent-type-noexcept.cpp templates/dependent-names/dependent-type-template-id.cpp templates/dependent-names/type-dependent-deduced-return.cpp templates/dependent-names/type-dependent-fold.cpp templates/dependent-names/type-dependent-placeholder-parameter.cpp templates/dependent-names/type-dependent-placeholder-variable.cpp templates/dependent-names/generic-lambda-argument-lookup.cpp templates/concepts/requires-expression-lookup-outside-constraint.cpp

For a dependent unqualified call, ordinary lookup finds the declarations
before the definition ({principle}`NAM-P1` gives the scopes it
searches). {term}`Argument-dependent lookup
<argument-dependent lookup>` finds the declarations before the
definition and those before a {term}`point of instantiation`
({rule}`INST-31`). Where the points give different results,
{rule}`INST-32` applies. Overload resolution chooses among all of them
({principle}`OVL-P1`). An
operator expression with a type-dependent operand finds its non-member
operator functions in the same way. A dependent call inside a requires
expression uses the same two lookups, also where the requires
expression is not a constraint, as in the condition of `if constexpr`
or the initializer of a variable template ({rule}`CPT-26`).
:::

:::{rule} DEP-10
:kind: reject
:tests: templates/dependent-names/ordinary-lookup-later-fundamental.cpp templates/dependent-names/ordinary-lookup-later-class.cpp templates/dependent-names/operator-ordinary-lookup-later.cpp

A dependent call or operator expression does not find a function that
is declared after the definition, unless the function is in an
associated namespace of an argument ({rule}`NAM-27`). Arguments of
fundamental type add no namespace.
:::

:::{rule} DEP-11
:kind: required
:tests: templates/dependent-names/dependent-base-skipped.cpp templates/dependent-names/dependent-base-qualified.cpp templates/dependent-names/independent-base-searched.cpp templates/dependent-names/current-instantiation-enclosing-base.cpp templates/dependent-names/dependent-base-conversion-function.cpp templates/dependent-names/template-keyword-this-dependent-base.cpp

Unqualified lookup in a class template does not search a base whose
type is dependent. It searches a base whose type is not dependent, and
a base that is the current instantiation of an enclosing class. A
member of a dependent base is found through `this->`, or through a
qualified name with the class or the base.

A conversion function name `operator T` whose type `T` is dependent is
a {term}`dependent name`. Its lookup waits for the instantiation, and
then also searches the dependent bases ({question}`Q-0248`).
:::

:::{rule} DEP-12
:kind: reject
:tests: templates/dependent-names/dependent-base-not-found.cpp templates/dependent-names/dependent-base-type-not-found.cpp templates/dependent-names/independent-error-unused-base-member.cpp templates/dependent-names/dependent-base-conversion-function-independent.cpp

An unqualified name that only a dependent base declares is not found
(DEP-11). The program must not use such a name as a value or as a type,
also when no specialization is made.
A conversion function name whose type is not dependent, such as
`operator int`, is not found either.
:::

:::{rule} DEP-13
:kind: reject
:tests: templates/dependent-names/qualified-member-found-again.cpp templates/dependent-names/this-member-ambiguous.cpp templates/dependent-names/this-member-dependent-base.cpp

A qualified name whose qualifier is the current instantiation is looked
up at the definition and again at the instantiation. A member access
through `this` is looked up in the same way. If the second lookup finds
an ambiguous set, the program is not valid ({question}`Q-0230`).
{rule}`DER-4` decides whether the set is ambiguous.
:::

:::{rule} DEP-14
:kind: required
:tests: templates/dependent-names/unqualified-member-kept.cpp templates/dependent-names/unqualified-member-function-kept.cpp templates/dependent-names/member-found-call-no-argument-lookup.cpp

An unqualified name that finds a member at the definition keeps that
member in every specialization. A member of the same name in a
dependent base does not make the name ambiguous.
:::

:::{rule} DEP-36
:kind: required
:tests: templates/dependent-names/base-member-hides-parameter.cpp templates/dependent-names/base-member-hides-constant-parameter.cpp

In a class template, unqualified lookup searches the class and its
independent bases before the template parameters. A member found there
hides a template parameter of the same name. The rule holds also in a
member defined outside the class.
:::

:::{rule} DEP-40
:kind: required
:tests: templates/dependent-names/qualified-call-dependent-argument-bound-early.cpp templates/dependent-names/member-found-call-no-argument-lookup.cpp

A qualified call looks up its name at the definition, in the scope that
the qualifier names, also when an argument is dependent. Such a call
and an unqualified call whose name finds a class member (DEP-14) take
no argument-dependent lookup. So neither call finds a function declared
after the definition.
:::

### The current instantiation

:::{rule} DEP-15
:kind: required
:tests: templates/dependent-names/current-instantiation-forms.cpp templates/dependent-names/current-instantiation-nested.cpp templates/dependent-names/current-instantiation-partial.cpp templates/dependent-names/current-instantiation-partial-alias.cpp templates/dependent-names/current-instantiation-alias.cpp templates/dependent-names/other-specialization-needs-typename.cpp templates/dependent-names/partial-primary-form-needs-typename.cpp templates/dependent-names/current-instantiation-parameter-alias.cpp templates/dependent-names/current-instantiation-constant-member.cpp classes/constructors/mem-init-other-specialization-own-template.cpp templates/dependent-names/member-of-other-specialization-constant-argument.cpp templates/dependent-names/other-specialization-constant-argument-no-typename.cpp templates/dependent-names/other-specialization-constant-argument-member-missing.cpp

In a class template, its members, and its nested classes, these names
name the {term}`current instantiation`:

- the {term}`injected class name` ({rule}`TARG-11`),
- the template name with its own parameters in order, also after `::`,
  where an alias for a {term}`type parameter` counts as that parameter,
- in a {term}`partial specialization`, the template name with an
  argument list that is the same as that partial specialization's list,
  argument by argument ({rule}`TARG-19`), where an alias counts as the
  type that it names,
- in a nested class, the nested class's name, also qualified with the
  enclosing current instantiation,
- an alias template specialization that gives one of the above
  ({question}`Q-0229`).

Any other argument list names another specialization. That
specialization is dependent only if {rule}`DEP-1` makes it so, as for
`Box<T*>`. An argument list with no dependent argument, such as
`Box<0>` inside `Box<N>`, names a specialization that is not dependent,
and the definition looks up its members. A `static` constant member
whose value is a {term}`constant parameter` does not count as that
parameter.
:::

:::{rule} DEP-16
:kind: required
:tests: templates/dependent-names/current-instantiation-forms.cpp templates/dependent-names/current-instantiation-nested.cpp templates/dependent-names/current-instantiation-partial.cpp templates/dependent-names/current-instantiation-partial-alias.cpp templates/dependent-names/current-instantiation-parameter-alias.cpp

A qualified name whose qualifier names the current instantiation is
looked up at the definition. A member type found so needs no
`typename`.
:::

:::{rule} DEP-17
:kind: reject
:tests: templates/dependent-names/current-member-missing-used.cpp templates/dependent-names/current-member-missing.cpp templates/dependent-names/this-member-missing.cpp templates/dependent-names/this-member-missing-specialized.cpp templates/dependent-names/this-member-missing-used.cpp templates/dependent-names/this-member-dependent-base.cpp

If the current instantiation has no dependent base, a qualified name
into the current instantiation must find a declaration. The same holds
for a member access through `this`, such as the call `this->f()` or
the read `this->n`. An explicit specialization of the class that
declares the member does not change the lookup in the primary
template. The name is not dependent, so DEP-30 applies when no
specialization is made ({question}`Q-0228`).
:::

:::{rule} DEP-39
:kind: required
:tests: templates/dependent-names/member-access-dependent-object-late.cpp templates/dependent-names/member-access-dependent-object-never-found.cpp

If the object expression of `.` or `->` is type-dependent and is not
the current instantiation, the member name is looked up at the
instantiation, in the class of the object ({rule}`NAM-37`). The
definition of the template checks nothing about the name.
:::

:::{rule} DEP-18
:kind: required
:tests: templates/dependent-names/current-member-dependent-base.cpp templates/dependent-names/current-member-dependent-base-function.cpp

If the current instantiation has a dependent base, a qualified name into
the current instantiation that finds nothing at the definition is
dependent. The name is looked up at the instantiation, also in the
bases.
:::

### The keywords typename and template

:::{rule} DEP-19
:kind: reject
:tests: templates/dependent-names/typename-needed-block.cpp templates/dependent-names/typename-needed-parameter.cpp templates/dependent-names/typename-needed-template-argument.cpp templates/dependent-names/typename-needed-expression.cpp templates/dependent-names/typename-needed-using.cpp templates/dependent-names/value-read-as-value.cpp templates/dependent-names/value-was-type.cpp templates/dependent-names/other-specialization-needs-typename.cpp templates/dependent-names/partial-primary-form-needs-typename.cpp templates/dependent-names/current-instantiation-constant-member.cpp templates/dependent-names/parameter-position-variable-template.cpp

Let `T` be dependent and not the current instantiation. Outside the
positions of DEP-20, the parser reads `T::name` as a value. A
specialization in which `T::name` is a type is then not valid
({question}`Q-0224`). The keyword `typename` before `T::name` makes the
name a type.
:::

:::{rule} DEP-20
:kind: required
:tests: templates/dependent-names/type-only-positions.cpp templates/dependent-names/type-only-positions-more.cpp

In these type-only positions, a qualified name with a dependent
qualifier names a type without `typename`:

- a base in a base list, and a name before `::`,
- the type of an alias declaration,
- the type of a variable and the return type of a function, declared
  at namespace scope, and a trailing return type,
- the type of a member, and the return type and parameter types of a
  member function or friend declared in a class,
- the parameter types of a function declared with a qualified name,
- a default type template argument, and the type of a constant
  template parameter,
- the type in a `new` expression and in a named cast,
- the parameter types of a lambda expression.
:::

:::{rule} DEP-21
:kind: required
:tests: templates/dependent-names/typename-not-needed.cpp templates/dependent-names/typename-needs-qualified-name.cpp

`typename` can come before a qualified name that is not dependent, also
outside a template. The name after `typename` is always a qualified
name.
:::

:::{rule} DEP-22
:kind: reject
:tests: templates/dependent-names/typename-finds-value.cpp templates/dependent-names/typename-finds-nothing.cpp templates/dependent-names/typename-finds-data-member.cpp

At the instantiation, the name after `typename` must name a type. The
keyword does not change the lookup. A specialization in which the name
finds a value or nothing is not valid, also if the class has a nested
type of the same name.
:::

:::{rule} DEP-23
:kind: required
:tests: templates/dependent-names/using-typename.cpp templates/dependent-names/using-typename-return.cpp

A using-declaration that names a member of a dependent base with
`typename` declares a type name in the class. Without `typename`, the
using-declaration names a value (DEP-19), and a use of the name as a
type is not valid.
:::

:::{rule} DEP-24
:kind: reject
:tests: templates/dependent-names/template-keyword-needed-qualified.cpp templates/dependent-names/template-keyword-needed-member.cpp templates/dependent-names/template-keyword-needed-template-argument.cpp templates/dependent-names/template-keyword-needed-after-typename.cpp templates/dependent-names/template-keyword-forms.cpp templates/dependent-names/template-keyword-needed-base.cpp templates/dependent-names/template-keyword-needed-alias-target.cpp expressions/lambdas/template-head-dependent-call.cpp templates/dependent-names/member-template-through-current-instantiation-object.cpp templates/dependent-names/member-template-current-instantiation-dependent-base-needs-template.cpp templates/dependent-names/member-template-current-instantiation-pointer.cpp templates/dependent-names/template-keyword-this-dependent-base.cpp templates/dependent-names/member-template-non-dependent-base.cpp templates/dependent-names/member-template-current-instantiation-qualifier-dependent-base.cpp

A {term}`member template` must have the keyword `template` before its name in
these places:

- after `::` with a dependent qualifier that is not the current
  instantiation, or with the current instantiation when lookup at the
  definition finds no member template, as for a member of a dependent
  base,
- after `.` or `->` with a type-dependent object, unless the object's
  type is the current instantiation and lookup at the definition finds
  the member template in the class or in a base that is not dependent
  ({question}`Q-0427`).

Without the keyword, a `<` after the name is the less-than operator
({rule}`TARG-2`), and the name is not a template template argument.
The rule holds also after `typename`, in a base list, and in the
target of an alias declaration ({question}`Q-0225`).
:::

:::{rule} DEP-25
:kind: required
:tests: templates/dependent-names/template-keyword-forms.cpp templates/dependent-names/template-keyword-template-template-default.cpp

The keyword `template` can come before a template name that lookup
already finds, also outside a template. Before a template template
argument, `template` needs no argument list after the name. The
default argument of a {term}`template template parameter` is such an argument.
:::

:::{rule} DEP-26
:kind: reject
:tests: templates/dependent-names/template-keyword-not-template.cpp templates/dependent-names/template-keyword-not-template-unused.cpp templates/dependent-names/template-keyword-no-list.cpp templates/dependent-names/template-keyword-no-list-class.cpp templates/dependent-names/template-keyword-no-list-static.cpp

At the instantiation, a name after the keyword `template` must name a
template. An argument list must follow the name, unless the name is a
template template argument, or a class template or alias template
whose arguments are deduced ({question}`Q-0226`).
:::

:::{rule} DEP-27
:kind: reject
:tests: templates/dependent-names/template-keyword-in-declarator.cpp templates/dependent-names/template-keyword-in-declarator-static.cpp templates/dependent-names/template-keyword-in-declarator-class.cpp

The keyword `template` must not come after the qualifier of the name
that a declaration declares ({question}`Q-0227`).
:::

:::{rule} DEP-34
:kind: reject
:tests: templates/dependent-names/template-keyword-destructor.cpp templates/dependent-names/template-keyword-destructor-dot.cpp

The keyword `template` must not come before the name of a destructor
({question}`Q-0245`).
:::

### Checking a template before instantiation

:::{rule} DEP-28
:kind: required
:tests: templates/dependent-names/checked-at-instantiation.cpp templates/dependent-names/dependent-error-unused.cpp templates/dependent-names/dependent-error-at-instantiation.cpp templates/dependent-names/template-keyword-not-template-unused.cpp templates/dependent-names/dependent-call-braced-list.cpp classes/virtual/override-dependent-base-missing.cpp classes/virtual/override-dependent-base-run.cpp

A construct that depends on the template parameters is checked for each
specialization. If some argument list makes the template valid, the
implementation accepts the definition. An error in a dependent
construct makes only the specializations with that error not valid.
:::

:::{rule} DEP-29
:kind: reject
:tests: templates/dependent-names/syntax-error-unused.cpp templates/dependent-names/typename-needs-qualified-name.cpp templates/dependent-names/typename-needed-parameter.cpp templates/dependent-names/parameter-position-variable-template.cpp

A template definition must follow the grammar, also when no
specialization is made. In `template <class T> int take(T::type)` at
namespace scope, `T::type` is a value (DEP-19), so the parentheses hold
the initializer of a variable template. A function body after them
breaks the grammar.
:::

:::{rule} DEP-37
:kind: reject
:tests: templates/dependent-names/operator-pack-never-fits.cpp templates/dependent-names/binary-operator-pack-never-fits.cpp

A template must not declare an operator function whose parameters
leave no room for an element of its parameter pack, which only an
empty pack makes valid. The implementation rejects such a template
also when no specialization is made. {rule}`OPR-2` gives the number of
parameters of each operator function, and DEP-33 gives the general case.
:::

## Implementation Requirements

:::{rule} DEP-30
:kind: no-diagnostic
:tests: templates/dependent-names/independent-error-unused-conversion.cpp templates/dependent-names/independent-error-unused-call.cpp templates/dependent-names/current-member-missing.cpp templates/dependent-names/this-member-missing-specialized.cpp templates/dependent-names/this-member-missing.cpp declarations/functions/default-arguments/template-default-independent-unused.cpp classes/virtual/override-independent-base-unused.cpp templates/dependent-names/other-specialization-constant-argument-member-missing.cpp

A specialization made right after the definition of a template must not
fail because of a construct in its {term}`templated context` that does
not depend on the parameters. An
implementation need not diagnose such a template if no specialization
is made ({question}`Q-0223`). These are such constructs:

- an expression, a conversion, or a name in the body that lookup
  finds, since DEP-6 and DEP-12 reject a name that it does not find,
- a default argument of a function template that does not depend on
  the parameters ({rule}`DEFARG-10`), although {rule}`INST-9`
  instantiates a default argument only for a call that uses it,
- `override` on a member that overrides nothing, in a class template
  whose bases do not depend on the parameters ({rule}`VIRT-3`).
:::

:::{rule} DEP-31
:kind: no-diagnostic
:tests: templates/dependent-names/incomplete-then-complete.cpp templates/dependent-names/qualified-call-later-default.cpp templates/dependent-names/constant-function-defined-later.cpp

A construct that does not depend on the parameters must have the same
meaning right after the definition and at each instantiation. These
changes break the rule:

- a class that is incomplete at the definition is complete later,
- an explicit or partial specialization is declared later,
- a default argument is added later to a function that lookup at the
  definition finds, also for a qualified call with a dependent argument
  ({question}`Q-0249`),
- a {term}`constant evaluation` needs a function, variable, or reference that
  is defined only after the definition ({question}`Q-0250`).

An implementation need not diagnose a break of this rule. A function
that only argument-dependent lookup at the instantiation finds takes the
default arguments declared before the point of instantiation (DEP-9).
:::

:::{rule} DEP-32
:kind: no-diagnostic
:tests: templates/dependent-names/no-valid-specialization.cpp templates/dependent-names/no-valid-discarded-statement.cpp

A template must have at least one argument list that makes a valid
specialization. The rule also applies to each discarded statement of
`if constexpr` in a {term}`templated context`. An implementation need not diagnose a
template that breaks the rule. A `static_assert` does not count
({rule}`SA-5`).
:::

:::{rule} DEP-33
:kind: no-diagnostic
:tests: templates/dependent-names/pack-always-empty.cpp templates/dependent-names/pack-always-empty-function-type.cpp templates/dependent-names/base-pack-expanded-twice.cpp

A variadic template must have a valid specialization in which the pack
has an element. An implementation need not diagnose a template whose
every valid specialization has an empty pack. A base list that expands
the same pack twice is such a case, because one element names a base
twice ({rule}`PACK-14`, {question}`Q-0247`). {rule}`PACK-9` gives the
result of an expansion of an empty pack. DEP-37 names the case that the
implementation rejects.
:::

:::{rule} DEP-35
:kind: no-diagnostic
:tests: templates/dependent-names/qualified-member-other-at-instantiation.cpp templates/dependent-names/qualified-member-function-other.cpp

If the second lookup of DEP-13 finds one member other than the member
found at the definition, the program is not valid. An implementation
need not diagnose a break of this rule ({question}`Q-0246`).
{rule}`PACK-30` gives the case of an empty expanded using-declarator.
:::

:::{rule} DEP-38
:kind: no-diagnostic
:tests: templates/dependent-names/argument-lookup-other-unit.cpp templates/dependent-names/argument-lookup-other-unit-ambiguous.cpp

Suppose argument-dependent lookup for a dependent call saw each
externally linked declaration of every translation unit. That lookup
must not find a better function than DEP-9 finds, or make the call not
valid. An implementation need not diagnose a break of this rule.
:::

## Examples

Two phases, and the lookup of a dependent call:

% tidal-example: reject
```cpp
int scale(double) { return 1; }
namespace geo { struct Point {}; }

template <class T> int measure(T value) {
  return scale(2)               // independent: scale(double), bound here
       + area(value);           // dependent: looked up again for each T
}

int scale(int) { return 2; }    // not seen by measure
namespace geo { int area(Point) { return 10; } }   // found by argument lookup

int x = measure(geo::Point{});  // 11
int y = measure(1);             // rejected: area(int) is not found
```

A member alias of the current instantiation whose target does not
depend on the parameters (DEP-1). The call is not dependent, so it
binds at the definition (DEP-5):

```cpp
namespace shop { struct Item {}; int pick(Item, long = 0) { return 1; } }
template <class T> struct Box {
  using item = shop::Item;            // not a dependent type
  int get() { item i; return pick(i); }   // bound here: pick(Item, long)
};
namespace shop { int pick(Item) { return 2; } }   // not seen by get
int one = Box<int>{}.get();           // 1
```

A dependent base:

% tidal-example: reject
```cpp
template <class T> struct Base { int size = 0; };
template <class T> struct List : Base<T> {
  int a() { return size; }             // rejected: Base<T> is not searched
  int b() { return this->size; }       // Base<T>::size
  int c() { return List::size; }       // Base<T>::size
};
```

The keywords:

% tidal-example: reject
```cpp
template <class C> int first(C& items) {
  typename C::value_type* p = items.data();       // a type
  C::value_type* q = nullptr;                     // rejected: a product with the undeclared q (DEP-6)
  C::value_type count;                            // rejected: a value, then a name (DEP-29)
  return items.template get<0>();                 // a member template
}

template <class C> struct View {
  using value = C::value_type;                    // type-only: no typename
  View::value top();                              // the current instantiation
};

template <class T> int read() {
  int n(T::count);                                // a variable: T::count is a value
  return n;
}

template <class T> struct Box { using type = long; };
template <class T> struct Box<T*> {
  using pointer = T*;
  using type = int;
  Box<pointer>::type get();                       // the current instantiation: no typename
  typename Box<T>::type other();                  // another specialization
};

template <class T> int take(T::type);             // a variable template
template <class T> int give(T::type) { return 0; } // rejected: a body after an initializer
template <class T> int keep(typename T::type) { return 0; }  // a function template
```

A second lookup through `this` or the current instantiation
(DEP-13, DEP-35). A member of a class hides the same member of its
{term}`virtual bases <virtual base>` ({rule}`DER-4`):

% tidal-example: reject
```cpp
struct Fixed { int m = 1; };
struct Extra : virtual Fixed { int m = 2; };
struct Other { int m = 3; };
template <class T> struct Both : virtual Fixed, T {
  int get() { return this->m; }
};
int a = Both<Extra>{}.get();     // Extra::m hides Fixed::m: no ambiguity (DEP-35 applies)
int b = Both<Other>{}.get();     // rejected: Fixed::m and Other::m are ambiguous
```

Templates with no valid specialization (DEP-32, DEP-33). The type of
`sizeof` is unsigned ({rule}`UN-12`), so `sizeof(T) - sizeof(T) - 1`
wraps to the largest value ({rule}`ADD-5`), which is too large for an
array bound ({rule}`DCL-19`). `void(T)` and `int(T)` in `sizeof` are
function types with one parameter of type `T`, and `sizeof` of a
function type is not valid ({rule}`UN-14`). With an empty pack, no such
`sizeof` is made:

```cpp
template <class T> void never() {
  int items[sizeof(T) - sizeof(T) - 1];          // no T makes a valid bound
}
template <class... T> int count() {
  return (0 + ... + int(sizeof(int(T))));       // valid only for an empty pack
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 117
of 144 tests.

### Divergences

Each settled question below keeps its test's override for the family
that still differs. Reporting the EDG differences to the EDG project
waits on the maintainer (`documentation/decisions/pending.md`).

- {question}`Q-0434` (DEP-2; open):
  whether `this->n` is type-dependent when the type of `n` does not
  depend on the parameters. It is not, also in a local class of a
  function template, as Clang 22, GCC, and EDG say. Clang 18 differs.
- {question}`Q-0432` (DEP-4; open):
  whether `sizeof` or `noexcept` of a value-dependent operand is
  value-dependent. It is not, as Clang and GCC say. EDG differs.
- {question}`Q-0354` (DEP-1, DEP-2; settled on DEP-P2): whether `decltype`
  of a generic lambda's parameter makes a call dependent. It does, and
  the program is valid. EDG rejects the call before the instantiation.
- {question}`Q-0248` (DEP-11, DEP-12; open): whether an unqualified
  conversion function name finds `operator T` of a dependent base. It
  does, as Clang does. GCC and EDG reject the call.
- {question}`Q-0230` (DEP-13; settled on DEP-P3): whether a qualified
  member name of the current instantiation is looked up again at the
  instantiation. It is, and an ambiguity makes the program not valid.
  GCC 16 keeps the member that the definition found.
- {question}`Q-0229` (DEP-15; settled on DEP-P4): whether an alias
  template specialization names the current instantiation. It does.
  EDG differs, and GCC 16 rejects a member of a dependent base through
  the alias (DEP-18).
- {question}`Q-0228` (DEP-17, DEP-30; settled on DEP-P4 and DEP-P6):
  whether a missing member of the current instantiation is diagnosed in
  an unused template. No diagnostic is required. No family differs from
  the answer. GCC 14 accepts the template, and Clang 18 accepts a
  member access through `this`.
- {question}`Q-0224` (DEP-19; settled on DEP-P5): whether a dependent
  name without `typename` that becomes a type at instantiation is an
  error outside a type-only position. It is. EDG uses the type.
- {question}`Q-0225` (DEP-24; open): whether a name after `typename T::`
  needs `template` before an argument list. It does, as Clang and EDG
  say. GCC accepts the name without the keyword.
- {question}`Q-0226` (DEP-26; open): whether `template` can come before
  a member with no argument list after it. Only before a template
  template argument or a class or alias template. GCC accepts a data
  member, and Clang rejects a class template with deduced arguments.
- {question}`Q-0227` (DEP-27; open): whether `template` can follow the
  qualifier of a name that a declaration declares. It cannot. GCC and
  EDG accept it for a member function, and EDG for a member class
  template.
- {question}`Q-0223` (DEP-30; settled on DEP-P6): whether an
  implementation diagnoses an independent error in an unused template.
  No diagnostic is required. No family differs from the answer, and GCC
  accepts the template.
- {question}`Q-0249` (DEP-31; settled on DEP-P1 and DEP-P6): whether a
  qualified call uses a default argument that a later declaration adds.
  No diagnostic is required. No family differs from the answer, and GCC
  uses the default.
- {question}`Q-0250` (DEP-31; settled on DEP-P6): whether an
  independent constant can use a `constexpr` function defined after the
  template. No diagnostic is required. No family differs from the
  answer, and Clang rejects the template.
- {question}`Q-0247` (DEP-33, DEP-37; settled): whether a base list that
  expands one pack twice is diagnosed before instantiation. No
  diagnostic is required. Every compiler's result fits, and GCC waits
  for the instantiation.
- {question}`Q-0245` (DEP-34; settled on DEP-P5): whether `template`
  can come before a destructor name. It cannot. EDG accepts it.
- {question}`Q-0246` (DEP-35; open): whether a name that finds another
  single member at the instantiation makes the program an error. It
  does, with no diagnostic required. Every compiler accepts the program
  and uses the other member.
- {question}`Q-0427` (DEP-24,
  DEP-39; open): whether a member template named after `.` needs
  `template` when the object's type is the current instantiation. It
  does not when lookup at the definition finds the member template in
  the class. No family differs.

### Split tests

Each row is a test that carries an override for a baseline compiler. The
Expected column gives the outcome that the rules predict, and each
family column gives the outcome that the test expects on that family.
EDG runs no programs, so its cell for a run test gives the result of the
compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `base-pack-expanded-twice.cpp` | DEP-33 | reject | reject | accept | reject | {question}`Q-0247` |
| `classes/virtual/override-independent-base-unused.cpp` | VIRT-3, DEP-30 | reject | reject | accept | accept | |
| `constant-function-defined-later.cpp` | DEP-31 | run | reject | run | accept | {question}`Q-0250` |
| `current-instantiation-alias.cpp` | DEP-15 | run | run | run | reject | {question}`Q-0229` |
| `current-member-missing.cpp` | DEP-30, DEP-17 | reject | reject | GCC 16: reject; GCC 14: accept | reject | {question}`Q-0228` |
| `declarations/functions/default-arguments/template-default-independent-unused.cpp` | DEFARG-10, DEP-30 | reject | reject | accept | reject | |
| `dependent-base-conversion-function.cpp` | DEP-11 | run | run | reject | reject | {question}`Q-0248` |
| `independent-error-unused-call.cpp` | DEP-30 | reject | reject | accept | reject | {question}`Q-0223` |
| `independent-error-unused-conversion.cpp` | DEP-30 | reject | reject | accept | reject | {question}`Q-0223` |
| `qualified-call-later-default.cpp` | DEP-31 | run | reject | run | reject | {question}`Q-0249` |
| `qualified-member-found-again.cpp` | DEP-13 | reject | reject | accept | reject | {question}`Q-0230` |
| `sizeof-value-dependent-operand-call-bound-early.cpp` | DEP-4, DEP-5 | accept | accept | accept | reject | {question}`Q-0432` |
| `sizeof-value-dependent-operand-no-typename.cpp` | DEP-4 | run | run | run | reject | {question}`Q-0432` |
| `template-keyword-destructor-dot.cpp` | DEP-34 | reject | reject | reject | accept | {question}`Q-0245` |
| `template-keyword-destructor.cpp` | DEP-34 | reject | reject | reject | accept | {question}`Q-0245` |
| `template-keyword-in-declarator-class.cpp` | DEP-27 | reject | reject | reject | accept | {question}`Q-0227` |
| `template-keyword-in-declarator-static.cpp` | DEP-27 | reject | reject | accept | accept | {question}`Q-0227` |
| `template-keyword-in-declarator.cpp` | DEP-27 | reject | reject | accept | accept | {question}`Q-0227` |
| `template-keyword-needed-after-typename.cpp` | DEP-24 | reject | reject | accept | reject | {question}`Q-0225` |
| `template-keyword-needed-base.cpp` | DEP-24 | reject | reject | accept | reject | {question}`Q-0225` |
| `template-keyword-no-list-class.cpp` | DEP-26 | accept | reject | accept | accept | {question}`Q-0226` |
| `template-keyword-no-list.cpp` | DEP-26 | reject | reject | accept | reject | {question}`Q-0226` |
| `this-in-local-class-not-type-dependent.cpp` | DEP-2 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0434` |
| `this-member-independent-type-bound-early.cpp` | DEP-2, DEP-5 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0434` |
| `this-member-missing-specialized.cpp` | DEP-30, DEP-17 | reject | Clang 22: reject; Clang 18: accept | GCC 16: reject; GCC 14: accept | reject | {question}`Q-0228` |
| `this-member-missing.cpp` | DEP-30, DEP-17 | reject | Clang 22: reject; Clang 18: accept | GCC 16: reject; GCC 14: accept | reject | {question}`Q-0228` |
| `typename-needed-expression.cpp` | DEP-19 | reject | reject | reject | accept | {question}`Q-0224` |

### Notes

The clean-room writer probed each item of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
with the same compilers. Where the compilers differ from the outcome
that the review predicts, the rules follow the compilers, and the notes
and the questions say so.

- DEP-15, DEP-1: every compiler treats `Box<0>` inside `Box<N>` as a
  type that is not dependent. `Box<0>::type` needs no `typename`, and a
  missing member of `Box<0>` is rejected in a member function that no
  program calls. An explicit specialization of `Box<0>` after the
  template is rejected, because the definition already made `Box<0>`
  (`reports/clean-room/roadmap-10/writer-probes/f3b-dep-15-constant-argument.json`).

- DEP-30: the gap review of the text phase 8 wrote on expressions, foundations,
  and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) reads the
  override item as fitting the rule. For a default argument that does
  not depend on the parameters, the review finds the reference unclear,
  because a default argument is instantiated only for a call that uses
  it. Clang 22 and EDG 7.0 diagnose such a default argument when no
  call uses it, and GCC 16 does not, which the class of DEP-30 allows
  (`template-default-independent-unused.cpp`).

- DEP-30: the compilers split on an unused template with an
  independent error. Clang and EDG reject an invalid conversion, an
  invalid call, and a default argument that cannot convert, and GCC
  accepts them. Every compiler rejects an undeclared name. Clang rejects
  `override` on a member that overrides nothing in an independent base,
  and GCC and EDG accept it
  (`override-independent-base-unused.cpp`). For a member that
  `this->` names and the class lacks, GCC 14 and Clang 18 accept, and
  GCC 16, Clang 22, and EDG reject, for a call and for a data member,
  also when an explicit specialization declares the member. DEP-30
  allows each result.
- DEP-29: every compiler rejects `template <class T> int take(T::type)`
  with a body at the definition, with no specialization. Clang reports
  a missing `typename`, GCC expects `;` before the body, and EDG reports
  that `T::type` is not a type name. Without a body, every compiler
  accepts the declaration as a variable template, and `take<C>` is a
  variable when `C::type` is a constant.
- DEP-24: every compiler rejects `T::box<int>` without `typename` as
  the target of an alias declaration. The review predicts that GCC
  accepts that alias.
- DEP-27, DEP-34: each form that the probes tried splits the
  compilers, so every test of DEP-27 and DEP-34 needs an override.
- DEP-15: inside the partial specialization `Box<T*>`, every compiler
  reads `Box<pointer>` after `using pointer = T*;`, `Box<self*>` after
  `using self = T;`, and `Box<Ptr<T>>` with an alias template `Ptr<U>`
  for `U*` as the current instantiation.
- DEP-15: every compiler reads `Box<M>`, where `M` is a `static`
  constant member equal to the parameter `N`, as another
  specialization. The result holds for `const` and `constexpr` members,
  with or without parentheses. The review predicts the current
  instantiation. The gap review of the text phase 13 wrote on
  templates (`reports/review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md`,
  RV6-B-2) reads a member of the parameter's type, initialized with the
  parameter's name alone, as standing for the parameter, and the form
  with parentheses as another specialization. DEP-15 and DEP-P4 follow
  the compilers, and `pending.md` row 4 holds the case.
- DEP-31: every compiler rejects `sizeof` of a class that is
  incomplete at the definition, even if the class is complete before
  the use. An explicit specialization declared after an instantiation
  is rejected under {rule}`TSP-6`.
- DEP-32, DEP-33: every compiler accepts the templates, and no compiler
  diagnoses them.
- DEP-37: every compiler rejects an operator template whose pack must
  be empty. An operator template whose pack can have one element, such
  as an assignment operator with only a pack, is valid, and every
  compiler accepts it.
- DEP-38: a better function that only another translation unit
  declares is not used, and no compiler or linker reports it.
- DEP-5, DEP-40: every compiler binds an independent call through
  argument-dependent lookup at the definition, a qualified call with a
  dependent argument, and a call whose name finds a member, and none of
  them sees a better function declared later (I6-1, I6-9). The writer
  probe is `reports/clean-room/roadmap-11/writer-probes/i6-call-binding.json`.
- DEP-9: for a specialization first used before a better function is
  declared, and checked again after it, GCC sees the later function,
  and Clang and EDG do not. The points of instantiation differ, and
  INST-32 allows each result (I6-10).
- DEP-6, DEP-12, CNS-13: every compiler rejects an undeclared name, a
  name that only a dependent base declares, and a template-id whose
  independent arguments fail the constraints, in a template that is
  never instantiated (I6-7, I6-8).
- DEP-4: in the grid `sizeof-operand.json`, Clang also treats
  `alignof(decltype(N))` as value-dependent, and GCC does not. No test
  holds that form.

### Probe grids

The probe grids are in `tests/templates/dependent-names/grids/`, and
[their tables](https://github.com/sempuki/tidal/blob/main/tests/templates/dependent-names/grids/tables.md)
sit beside them. The typename-positions grid (DEP-19, DEP-20) writes
`T::type` in each position, with and without `typename`. The
template-keyword grid (DEP-24 to DEP-26) names a member of a class in
each position, with and without `template`. The current-instantiation
grid (DEP-15 to DEP-18) names `Box<T>` inside `Box` in each way, and
reads a member type, a missing member, and a member of a dependent base
through it. The sizeof-operand grid (DEP-4) passes `sizeof`,
`noexcept`, and `alignof` of a constant parameter as a template
argument, and a column adds a better function after the template.

### Sources

The clean-room writer probed each rule with the eight baseline
compilers and the two `-O2` report compilers, and wrote the grids. The
sanitized Clang lookup source returns "not found in the current
instantiation" for a qualified lookup into a class with a dependent
base (DEP-18). Its scope source treats the injected class name and a
template-id with the class's own parameters as the current
instantiation (DEP-15).

This table gives the source of each rule. Inventory items have the
numbers of
`reports/review/2026-10-31-inventory-dependent-names-and-instantiation.md`:

| Rule | Source |
|---|---|
| DEP-1 | probes; inventory E1, E2, E4 to E9 |
| DEP-2, DEP-3 | probes; inventory D4, D9, D11, D14, D15 |
| DEP-4 | probes; diagnostics; inventory D7, D16 to D18, E1 |
| DEP-5, DEP-6 | probes; diagnostics; inventory A9, A10 |
| DEP-7 | probes; inventory A7 |
| DEP-8 to DEP-10 | probes; diagnostics; inventory A1, A5, A8, A13 |
| DEP-11, DEP-12 | probes; diagnostics; clang-sema-lookup; inventory A4 |
| DEP-13, DEP-14 | probes; inventory A14, A15 |
| DEP-15, DEP-16 | grid current-instantiation; clang-sema-cxxscopespec; inventory C1, C3 to C6, C9 to C12 |
| DEP-17, DEP-18 | grid current-instantiation; clang-sema-lookup; inventory C7, C8 |
| DEP-19, DEP-20 | grid typename-positions; diagnostics; inventory F1 to F4 |
| DEP-21 to DEP-23 | probes; clang-diag-sema; inventory F11, F14, F15 |
| DEP-24 to DEP-27 | grid template-keyword; clang-diag-sema; clang-diag-parse; inventory F5 to F10, F13 |
| DEP-28, DEP-29 | probes; inventory B2, B9, B10 |
| DEP-30 to DEP-33 | probes; inventory B1, B3, B6 to B8 |
| DEP-34 to DEP-38 | probes; gap review items 2, 6, 18, 21, 22 |
| DEP-40 | probes; I6 report, I6-9 |

Gap review items 1 to 23 and 40 also gave probes for DEP-1 to DEP-4,
DEP-8 to DEP-17, DEP-22, DEP-24 to DEP-26, and DEP-31.

Inventory items left out of this page:

- A2, A6, A11, A12, C2, C13, D1 to D3, D5, D6, D8, D10, D12, D13, E3,
  and F12 are details of the definitions in DEP-1 to DEP-4 and DEP-15.
  The page has no probe for each of them.
- B4 belongs to {doc}`constraints`.
- B5 is {rule}`INST-12`.
- I9 and the module parts of I4 are outside the charter.

Inputs: clang-diag-sema, clang-diag-parse, clang-sema-lookup, clang-sema-cxxscopespec.

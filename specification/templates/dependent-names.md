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
:rules: DEP-5 DEP-6 DEP-7 DEP-8 DEP-9 DEP-10 DEP-28 DEP-38

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
every specialization, so the implementation binds the name once, at the
definition. A later declaration does not change the binding. A
{term}`dependent name` can mean something different for each argument
list, so its lookup waits for the arguments.

An unqualified call with a dependent argument uses both phases
(DEP-8, DEP-9):

| Lookup | Context | Finds a function declared after the template |
|---|---|---|
| ordinary unqualified lookup | the definition | no |
| argument-dependent lookup | the definition and the {term}`point of instantiation` | yes, in an associated namespace |

A call whose arguments are all of fundamental type has no associated
namespace ({rule}`NAM-27`), so it finds only what the definition sees
(DEP-10). An operator expression with a type-dependent operand finds
its operator functions in the same two ways (DEP-9). Each translation
unit sees only its own declarations. A better match in another unit
makes the program not valid, with no diagnostic required (DEP-38).
:::

:::{principle} DEP-P2
:rules: DEP-1 DEP-2 DEP-3 DEP-4

A construct depends on the template parameters in one of three ways:

| Kind | Meaning | Examples |
|---|---|---|
| {term}`dependent type` | the type differs between specializations | `T`, `T*`, `Box<T>`, `T[N]`, `typename T::type`, `decltype(t.size())` |
| {term}`type-dependent` expression | the type of the expression differs | `t`, `T(0)`, `t + 1`, `(t + ...)`, `this` in a class template |
| {term}`value-dependent` expression | the type is fixed, the value differs | `N`, `sizeof(T)`, `sizeof...(T)`, `static_cast<int>(t)` |

Some forms have a fixed type whatever their operands: `sizeof`,
`sizeof...` ({rule}`PACK-19`), `alignof`, `noexcept`, a literal, and a
cast to an independent type. A call with such an argument is not dependent on that argument, and
the implementation chooses its function at the definition (DEP-3).
:::

:::{principle} DEP-P3
:rules: DEP-11 DEP-12 DEP-14 DEP-36
:exceptions: DEP-13 DEP-35

A base class whose type is dependent is unknown at the definition. An
explicit specialization can give the base any members, so the
implementation does not search such a base for an unqualified name. A
program reaches a member of a dependent base through `this->`, through
the name of the class, or through the name of the base:

| Written | Searches a dependent base | Rule |
|---|---|---|
| `count` | no | DEP-11, DEP-12 |
| `this->count` | yes, at the instantiation | DEP-11 |
| `Derived::count`, `Base<T>::count` | yes, at the instantiation | DEP-11, DEP-18 |

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
| `template <int N> struct Box` | `Box`, `Box<N>` | `Box<M>` after `static const int M = N;` |
| a nested class `Box<T>::Inner` | `Inner`, `Box<T>::Inner` | `Box<T*>::Inner` |
| `template <class T> struct Box<T*>` | `Box`, `Box<T*>` | `Box<T>` |

A member of the current instantiation needs no `typename` (DEP-16). If
the class has no dependent base, a name that the class does not declare
is an error. The name is not dependent, so the implementation can
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
| `typename T::name<...>`, `T::name<...>` in a base list or an alias | not valid without `template` (DEP-24, {question}`Q-0225`) | |
| `T::template name<...>` | a template-id | the name is a template (DEP-26) |
| `typename T::template name<...>` | a type | the name is a class template or alias template |

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
| an independent construct that is not valid | need not diagnose | DEP-30 |
| an independent construct whose meaning changes before the instantiation | need not diagnose | DEP-31 |
| a construct that no argument list makes valid | need not diagnose | DEP-32 |
| a pack that every valid argument list leaves empty | need not diagnose | DEP-33 |
| an operator or a base list that only an empty pack makes valid | rejects | DEP-37 |
| a dependent construct that some argument list makes valid | accepts | DEP-28 |

The condition of a `static_assert` in a template is checked only at the
instantiation, also when the condition is independent ({rule}`SA-5`).
A discarded statement is not instantiated ({rule}`IF-13`).
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
:tests: templates/dependent-names/checked-at-instantiation.cpp templates/dependent-names/dependent-type-function-pack.cpp templates/dependent-names/dependent-type-noexcept.cpp templates/dependent-names/dependent-type-template-id.cpp templates/dependent-names/member-alias-independent.cpp

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
:tests: templates/dependent-names/dependent-call-late.cpp templates/dependent-names/type-dependent-placeholder-parameter.cpp templates/dependent-names/type-dependent-placeholder-variable.cpp templates/dependent-names/type-dependent-deduced-return.cpp templates/dependent-names/type-dependent-fold.cpp templates/dependent-names/type-dependent-set-and-binding.cpp

An expression is {term}`type-dependent` if one of these holds, and
DEP-3 does not give a fixed type:

- the expression is a name declared with a dependent type,
- the expression names a structured binding whose hidden variable has
  a dependent type,
- the expression names an overload set through a dependent qualifier,
  such as `T::make`, or with dependent explicit template arguments,
  such as `make<T>`,
- the expression names a constant template parameter whose type is a
  {term}`placeholder type`,
- the expression names a variable whose type is a placeholder type and
  whose initializer is type-dependent,
- the expression names a member function of the current instantiation
  whose return type is deduced,
- the expression is `this` in a member of a class template,
- the expression is a cast or a `new` whose named type is dependent,
- the expression is a fold expression,
- a member access names a member whose type is dependent,
- an operand or subexpression is type-dependent.
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
:tests: templates/dependent-names/value-dependent-bound.cpp templates/dependent-names/value-dependent-bound-fails.cpp templates/dependent-names/value-dependent-constant-variable.cpp templates/dependent-names/value-dependent-pack.cpp

An expression is {term}`value-dependent` if one of these holds:

- the expression names a constant template parameter,
- the expression names a `const` integer variable or a `constexpr`
  variable whose initializer is value-dependent,
- the expression is type-dependent,
- the expression is `sizeof`, `alignof`, or `noexcept` of a dependent
  type or a type-dependent operand,
- the expression is `sizeof...` ({rule}`PACK-19`) or a fold
  expression,
- an operand is value-dependent.

The value of such an expression is computed for each specialization.
Each specialization must then obey the rules for that value, such as
the array bound rules of {rule}`DCL-19`.
:::

### Binding of names

:::{rule} DEP-5
:kind: required
:tests: templates/dependent-names/independent-bound-early.cpp templates/dependent-names/independent-call-early.cpp templates/dependent-names/independent-call-pack-size.cpp templates/dependent-names/member-alias-independent.cpp

A name that is not dependent is looked up at the definition of the
template. Every specialization uses the declarations found there. A
declaration after the definition does not change the result.
:::

:::{rule} DEP-6
:kind: reject
:tests: templates/dependent-names/independent-not-declared.cpp templates/dependent-names/independent-type-not-declared.cpp

A name that is not dependent must be declared before the definition of
the template. A declaration between the definition and the use does not
make an instantiation valid.
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
before the definition. {term}`Argument-dependent lookup
<argument-dependent lookup>` finds the declarations before the
definition and those before the {term}`point of instantiation`
({rule}`INST-31`). Overload resolution chooses among all of them. An
operator expression with a type-dependent operand finds its non-member
operator functions in the same way.
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
:tests: templates/dependent-names/dependent-base-skipped.cpp templates/dependent-names/dependent-base-qualified.cpp templates/dependent-names/independent-base-searched.cpp templates/dependent-names/current-instantiation-enclosing-base.cpp

Unqualified lookup in a class template does not search a base whose
type is dependent. It searches a base whose type is not dependent, and
a base that is the current instantiation of an enclosing class. A
member of a dependent base is found through `this->`, or through a
qualified name with the class or the base.
:::

:::{rule} DEP-12
:kind: reject
:tests: templates/dependent-names/dependent-base-not-found.cpp templates/dependent-names/dependent-base-type-not-found.cpp templates/dependent-names/independent-error-unused-base-member.cpp templates/dependent-names/dependent-base-conversion-function.cpp

An unqualified name that only a dependent base declares is not found
(DEP-11). The program must not use such a name as a value or as a type.
The rule covers a conversion function named as `operator T`
({question}`Q-0248`).
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
:tests: templates/dependent-names/unqualified-member-kept.cpp templates/dependent-names/unqualified-member-function-kept.cpp

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

### The current instantiation

:::{rule} DEP-15
:kind: required
:tests: templates/dependent-names/current-instantiation-forms.cpp templates/dependent-names/current-instantiation-nested.cpp templates/dependent-names/current-instantiation-partial.cpp templates/dependent-names/current-instantiation-alias.cpp templates/dependent-names/other-specialization-needs-typename.cpp templates/dependent-names/partial-primary-form-needs-typename.cpp templates/dependent-names/current-instantiation-parameter-alias.cpp templates/dependent-names/current-instantiation-constant-member.cpp

In a class template, its members, and its nested classes, these names
name the {term}`current instantiation`:

- the injected class name ({rule}`TARG-11`),
- the template name with its own parameters in order, also after `::`,
  where an alias for a type parameter counts as that parameter,
- in a partial specialization, the template name with that partial
  specialization's argument list,
- in a nested class, the nested class's name, also qualified with the
  enclosing current instantiation,
- an alias template specialization that gives one of the above
  ({question}`Q-0229`).

Any other argument list names another specialization, which is
dependent. A `static` constant member whose value is a constant
parameter does not count as that parameter.
:::

:::{rule} DEP-16
:kind: required
:tests: templates/dependent-names/current-instantiation-forms.cpp templates/dependent-names/current-instantiation-nested.cpp templates/dependent-names/current-instantiation-partial.cpp templates/dependent-names/current-instantiation-parameter-alias.cpp

A qualified name whose qualifier names the current instantiation is
looked up at the definition. A member type found so needs no
`typename`.
:::

:::{rule} DEP-17
:kind: reject
:tests: templates/dependent-names/current-member-missing-used.cpp templates/dependent-names/current-member-missing.cpp templates/dependent-names/this-member-missing.cpp templates/dependent-names/this-member-missing-specialized.cpp templates/dependent-names/this-member-missing-used.cpp templates/dependent-names/this-member-dependent-base.cpp

If the current instantiation has no dependent base, a qualified name
into the current instantiation must find a declaration. The same holds
for a member access through `this`, such as `this->f`. The name is not
dependent, so DEP-30 applies when no specialization is made
({question}`Q-0228`).
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
:tests: templates/dependent-names/typename-needed-block.cpp templates/dependent-names/typename-needed-parameter.cpp templates/dependent-names/typename-needed-template-argument.cpp templates/dependent-names/typename-needed-expression.cpp templates/dependent-names/typename-needed-using.cpp templates/dependent-names/value-read-as-value.cpp templates/dependent-names/value-was-type.cpp templates/dependent-names/other-specialization-needs-typename.cpp templates/dependent-names/partial-primary-form-needs-typename.cpp templates/dependent-names/current-instantiation-constant-member.cpp

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
- the type of a member, and the parameter types of a member function
  or friend declared in a class,
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
:tests: templates/dependent-names/template-keyword-needed-qualified.cpp templates/dependent-names/template-keyword-needed-member.cpp templates/dependent-names/template-keyword-needed-template-argument.cpp templates/dependent-names/template-keyword-needed-after-typename.cpp templates/dependent-names/template-keyword-forms.cpp templates/dependent-names/template-keyword-needed-base.cpp templates/dependent-names/template-keyword-needed-alias-target.cpp

A member template must have the keyword `template` before its name in
these places:

- after `::` with a dependent qualifier that is not the current
  instantiation,
- after `.` or `->` with a type-dependent object.

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
default argument of a template template parameter is such an argument.
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
:tests: templates/dependent-names/checked-at-instantiation.cpp templates/dependent-names/dependent-error-unused.cpp templates/dependent-names/dependent-error-at-instantiation.cpp templates/dependent-names/template-keyword-not-template-unused.cpp templates/dependent-names/dependent-call-braced-list.cpp

A construct that depends on the template parameters is checked for each
specialization. If some argument list makes the template valid, the
implementation accepts the definition. An error in a dependent
construct makes only the specializations with that error not valid.
:::

:::{rule} DEP-29
:kind: reject
:tests: templates/dependent-names/syntax-error-unused.cpp templates/dependent-names/typename-needs-qualified-name.cpp

A template definition must follow the grammar, also when no
specialization is made.
:::

:::{rule} DEP-37
:kind: reject
:tests: templates/dependent-names/operator-pack-never-fits.cpp templates/dependent-names/base-pack-expanded-twice.cpp

A template must not have these constructs, which only an empty pack
makes valid:

- an operator function whose parameters leave no room for an element
  of its parameter pack,
- a base list that expands the same pack twice ({rule}`PACK-14`,
  {question}`Q-0247`).

The implementation rejects such a template also when no specialization
is made. DEP-33 gives the general case.
:::

## Implementation Requirements

:::{rule} DEP-30
:kind: no-diagnostic
:tests: templates/dependent-names/independent-error-unused-name.cpp templates/dependent-names/independent-error-unused-base-member.cpp templates/dependent-names/independent-error-unused-conversion.cpp templates/dependent-names/independent-error-unused-call.cpp templates/dependent-names/current-member-missing.cpp templates/dependent-names/this-member-missing-specialized.cpp templates/dependent-names/this-member-missing.cpp

A specialization made right after the definition of a template must not
fail because of a construct in its {term}`templated context` that does
not depend on the parameters. An
implementation need not diagnose such a template if no specialization
is made ({question}`Q-0223`).
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
- a constant evaluation needs a function, variable, or reference that
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
:tests: templates/dependent-names/pack-always-empty.cpp templates/dependent-names/pack-always-empty-function-type.cpp

A variadic template must have a valid specialization in which the pack
has an element. An implementation need not diagnose a template whose
every valid specialization has an empty pack. {rule}`PACK-9` gives the
result of an expansion of an empty pack. DEP-37 names the cases that
the implementation rejects.
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

A dependent base:

```cpp
template <class T> struct Base { int size = 0; };
template <class T> struct List : Base<T> {
  int a() { return size; }             // rejected: Base<T> is not searched
  int b() { return this->size; }       // Base<T>::size
  int c() { return List::size; }       // Base<T>::size
};
```

The keywords:

```cpp
template <class C> int first(C& items) {
  typename C::value_type* p = items.data();       // a type
  C::value_type* q = nullptr;                     // rejected: read as a product
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
```

A second lookup through `this` or the current instantiation
(DEP-13, DEP-35). A member of a class hides the same member of its
virtual bases ({rule}`DER-4`):

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

The results are in `tests/RESULTS.md`. All eight compilers agree on 98
of 119 tests.

The questions of the cluster of EDG differences (cluster 5) on this page
stay open under decision 0051: {question}`Q-0224`, {question}`Q-0229`,
and {question}`Q-0245`. On 2026-11-03, Clang 22 and GCC 16 still agree
with the provisional answer of each, and EDG 7.0 still differs.
Reporting these EDG differences to the EDG project waits on the
maintainer.

The questions of the cluster of template syntax corners (cluster 6) on
this page stay open under decision 0051: {question}`Q-0225`,
{question}`Q-0226`, and {question}`Q-0227`. On 2026-11-03, Clang 22,
GCC 16, and EDG 7.0 still split on each of them, and the probes of that
day added no case where the three agree with a provisional answer.
{question}`Q-0224` and {question}`Q-0245` belong to the same cluster
and keep their cluster 5 notes.

The clean-room writer probed each rule with the eight baseline
compilers and the two `-O2` report compilers, and wrote the grids
below. The sanitized Clang lookup source returns "not found in the
current instantiation" for a qualified lookup into a class with a
dependent base (DEP-18). Its scope source treats the injected class
name and a template-id with the class's own parameters as the current
instantiation (DEP-15).

The clean-room writer probed each item of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
with the same compilers. Where the compilers differ from the outcome
that the review predicts, the rules follow the compilers, and the
bullets below say so.

- DEP-30: Clang and EDG reject an unused template whose body has an
  invalid conversion or call with no dependent operand. GCC accepts the
  template. Every compiler rejects an undeclared name and a name from
  a dependent base. The rule needs no diagnostic, so both results are
  allowed ({question}`Q-0223`).
- DEP-19: EDG accepts `sizeof(T::type)`, a functional cast, and a
  function declared in a block, when the instantiation makes `T::type`
  a type. Clang and GCC reject them ({question}`Q-0224`).
- DEP-24: GCC accepts `typename T::box<int>`, and `T::box<int>` as a
  base, without `template`. Clang and EDG reject both
  ({question}`Q-0225`). Every compiler rejects `T::box<int>` without
  `typename` as the target of an alias declaration. The review predicts
  that GCC accepts that alias.
- DEP-26: GCC accepts `object.template field` for a data member. Clang
  and EDG reject it. Clang rejects `typename T::template box b(1)`,
  where the arguments of `box` are deduced. GCC and EDG accept it
  ({question}`Q-0226`). The split is the same for a member alias
  template. Every compiler, GCC too, rejects `T::template field` for a
  static data member.
- DEP-34: Clang and GCC reject `p->template ~T()` and
  `r.template ~T()`. EDG accepts both ({question}`Q-0245`).
- DEP-27: GCC and EDG accept `template` after the qualifier of a
  declared member function template. Clang rejects it
  ({question}`Q-0227`). For a member class template, Clang and GCC
  reject the keyword, and EDG accepts it.
- DEP-24: GCC also accepts `typename T::box<int>` without `template`
  as an alias target and as a function parameter type. Clang and EDG
  reject both. Every compiler rejects a friend declaration that names
  `T::get<int>` without the keyword ({question}`Q-0225`).
- DEP-17: GCC 14 accepts an unused template whose qualified name into
  the current instantiation finds nothing. GCC 16 and the other
  compilers reject the template. GCC 16 puts the error in its
  `-Wtemplate-body` group, which also holds the independent errors of
  DEP-30. Every compiler rejects the name once the class is
  instantiated. DEP-30 allows both results for the unused template
  ({question}`Q-0228`).
  For a member access through `this` that finds nothing, GCC 14 and
  Clang 18 accept the unused template, and GCC 16, Clang 22, and EDG
  reject it.
- DEP-15: EDG does not treat an alias template specialization as the
  current instantiation. GCC treats the alias as the current
  instantiation but rejects a name that only a dependent base declares
  ({question}`Q-0229`).
- DEP-13: Clang and EDG reject a qualified name into the current
  instantiation that becomes ambiguous at the instantiation. GCC keeps
  the member that the definition found ({question}`Q-0230`). Every
  compiler rejects `this->m` in the same class.
- DEP-35: when the second lookup finds one other member, every
  compiler accepts the program and uses that member. The review
  predicts a rejection. DEP-35 needs no diagnostic, so the result is
  allowed ({question}`Q-0246`).
- DEP-15: every compiler reads `Box<M>`, where `M` is a `static`
  constant member equal to the parameter `N`, as another
  specialization. The result holds for `const` and `constexpr` members,
  with or without parentheses. The review predicts the current
  instantiation.
- DEP-12: Clang finds a conversion function `operator T` of a dependent
  base without `this->`. GCC and EDG do not ({question}`Q-0248`).
- DEP-31: every compiler rejects `sizeof` of a class that is
  incomplete at the definition, even if the class is complete before
  the use. An explicit specialization declared after an instantiation
  is rejected under {rule}`TSP-6`.
- DEP-31: a qualified call finds a function at the definition, and a
  later declaration adds a default argument. GCC uses the default.
  Clang and EDG reject the call ({question}`Q-0249`). For the
  unqualified call `probe(item)`, lookup at the definition finds no
  function, and every compiler uses the default that argument-dependent
  lookup finds at the instantiation (`argument-lookup-later-default.cpp`,
  DEP-9).
- DEP-31: a `constexpr` variable in the template is initialized by a
  call to a function that is defined after the template. Clang rejects
  the template, GCC and EDG accept it ({question}`Q-0250`). As an array
  bound, the same call makes Clang and GCC reject the array, and EDG
  accepts it.
- DEP-32 and DEP-33: every compiler accepts the templates, and no
  compiler diagnoses them.
- DEP-37: every compiler rejects an operator template whose pack must
  be empty. Clang and EDG reject a base list that expands a pack twice,
  and GCC accepts it until the class is instantiated
  ({question}`Q-0247`). An operator template whose pack can have one
  element, such as an assignment operator with only a pack, is valid,
  and every compiler accepts it.
- DEP-38: a better function that only another translation unit
  declares is not used, and no compiler or linker reports it.

Positions of a dependent qualified name (DEP-19, DEP-20). Each row
writes `T::type` in one position and instantiates the template with a
class whose `type` is `int`. The first column writes the name alone,
the second with `typename`:

| | T::type | typename T::type |
|---|---|---|
| alias target | accept | accept |
| data member | accept | accept |
| member function return, in class | accept | accept |
| member function parameter, in class | accept | accept |
| member function return, outside class | accept | accept |
| member function parameter, outside class | accept | accept |
| function template return | accept | accept |
| function template parameter | reject | accept |
| trailing return type | accept | accept |
| default type argument | accept | accept |
| constant parameter type | accept | accept |
| static_cast type | accept | accept |
| new type | accept | accept |
| variable in a block | reject | accept |
| sizeof operand | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, clang-host, gcc-14, gcc-16, clang-18, clang-22 | accept |
| functional cast | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, clang-host, gcc-14, gcc-16, clang-18, clang-22 | accept |
| template argument | reject | accept |
| lambda parameter | accept | accept |
| function declared in a block | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | accept |
| variable template type | accept | accept |
| friend function parameter | accept | accept |

The keyword `template` (DEP-24 to DEP-26). Each row names a member of
a class `Has` and instantiates the template where there is one:

| | no keyword | template |
|---|---|---|
| after T:: in a call | reject | accept |
| after a dot | reject | accept |
| after an arrow | reject | accept |
| after typename T:: | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept |
| alias target | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept |
| template template argument, no list | reject | accept |
| independent qualifier | accept | accept |
| independent object | accept | accept |
| member that is not a template | accept | reject |
| data member that is not a template | accept | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 |

The current instantiation (DEP-15 to DEP-18). Each row is a way to
name `Box<T>` inside `Box`. The first column reads a member type
without `typename`. The second reads a missing member in a class with
no dependent base, and the template is not used. The third reads a
member that only a dependent base declares:

| | member type without typename | missing member, no dependent base | missing member, dependent base |
|---|---|---|---|
| Box | accept | accept: gcc-14; reject: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-16, clang-host, clang-18, clang-22 | accept |
| Box<T> | accept | accept: gcc-14; reject: edg-7.0, edg-7.0-ilp32, gcc-16, gcc-host, clang-host, clang-18, clang-22 | accept |
| ::Box<T> | accept | accept: gcc-14; reject: edg-7.0, edg-7.0-ilp32, gcc-host, clang-host, gcc-16, clang-18, clang-22 | accept |
| Same<T> | accept: gcc-host, gcc-14, clang-host, gcc-16, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: edg-7.0, edg-7.0-ilp32, gcc-14; reject: gcc-host, clang-host, gcc-16, clang-18, clang-22 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, clang-host, clang-18, clang-22; reject: gcc-host, gcc-16 |
| Box<T*> | reject | accept | accept |
| Pointer<T> | reject | accept | accept |
| Box<const T> | reject | accept | accept |

`Same<U>` is an alias for `Box<U>`, and `Pointer<U>` is an alias for
`Box<U*>`.

The probe grids are in `tests/templates/dependent-names/grids/`. This
table gives the source of each rule. Inventory items have the numbers of
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

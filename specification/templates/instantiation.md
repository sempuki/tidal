# Instantiation

{term}`Instantiation <instantiation>` makes a specialization from its
template, with the arguments in place of the parameters. The
implementation instantiates a specialization when a use needs the
specialization's definition. A program can also ask for an
instantiation with an {term}`explicit instantiation`. This page covers:

- which uses instantiate a class, a function, or a variable,
- what the instantiation of a class makes, and what waits for a use,
- explicit instantiation definitions and declarations,
- points of instantiation, which decide the declarations that an
  instantiation sees.

{doc}`dependent-names` says which names in a template wait for the
arguments. {doc}`specialization` says which definition a specialization
uses. Constraints on members are on {doc}`constraints`
({rule}`CNS-15`). Modules are outside the charter.

Depends on: template declarations, explicit and partial specialization,
dependent names, the one-definition rule.

## Principles

:::{principle} INST-P1
:rules: INST-1 INST-2 INST-4 INST-7 INST-8 INST-9 INST-10 INST-12 INST-13 INST-36
:exceptions: INST-3 INST-11

The implementation instantiates a {term}`specialization` only when a use needs
its definition. A program can therefore name a specialization whose
definition would not be valid, as long as no use needs that
definition. A use needs the definition in these cases:

| Entity | Instantiated by | Not instantiated by | Rule |
|---|---|---|---|
| class | an object, `sizeof`, a member access, `delete`, a conversion to a base, a function definition or call with the class by value | a pointer or reference type, an alias, a {term}`template argument`, a function declaration | INST-1, INST-2 |
| function | a call, taking the address, `decltype` of a call when the return type is deduced, a braced initializer that names a `constexpr` function | `decltype`, `sizeof`, or `noexcept` of a call otherwise | INST-7 |
| `static` data member, {term}`variable template` | a read or other use of the value | `decltype` | INST-8 |
| default argument | a call that leaves out the argument | a call that gives the argument, an explicit instantiation | INST-9 |
| default member initializer | a constructor or an aggregate initialization that does not initialize the member | a constructor or an aggregate initialization that does | INST-10 |

A {term}`discarded statement` of `if constexpr` is not instantiated
({rule}`IF-13`). An exception specification is instantiated when a use
needs the specification ({rule}`EXS-9`).

A class or enumeration declared in the body of a function template is
part of that function. The instantiation of the function is the use
that needs the local class, so the instantiation makes the parts of
the local class at once (INST-12). The parts that wait for a use in a
member of a class template do not wait in a local class:

- member function bodies and exception specifications,
- default member initializers,
- default arguments.

The constraints of a member of the local class still wait. They are
checked when a use of the member needs them.
:::

:::{principle} INST-P2
:rules: INST-5 INST-6

The instantiation of a class makes the declarations of its members.
The definitions of most members wait for a use:

| Member | Made with the class |
|---|---|
| data member, its type | yes |
| member function, its type | the declaration only |
| member function body, default arguments, exception specification | no (INST-7, INST-9) |
| default member initializer | no (INST-10) |
| `static` data member | the declaration only (INST-8) |
| nested class | the declaration only |
| {term}`member template` | the declaration only |
| unscoped enumeration | the whole definition |
| scoped enumeration | the declaration only ({question}`Q-0232`) |

An error in a declaration made with the class makes the use of the
class not valid (INST-6). Such an error can come from the arguments, for
example a `static` data member whose {term}`dependent type` becomes a function
type. An error in a definition that waits makes
only the use that needs the definition not valid.
:::

:::{principle} INST-P3
:rules: INST-16 INST-17 INST-18 INST-20 INST-21 INST-22 INST-23 INST-24 INST-25 INST-26 INST-27 INST-28 INST-29 INST-34 INST-35
:exceptions: INST-19

An explicit instantiation lets a program make the definitions of chosen
specializations in one {term}`translation unit`, and use them from the
others:

| Form | Effect | Rule |
|---|---|---|
| `template int size(int);` | an {term}`explicit instantiation definition`, which makes the definition in this unit | INST-16, INST-17 |
| `extern template int size(int);` | an {term}`explicit instantiation declaration`, which says that some unit has the definition | INST-27, INST-28 |

```text
unit A                                   unit B
──────                                   ──────
template <class T> int size(T);          template <class T> int size(T);
extern template int size(int);           template int size(int);
size(0)  ── uses the definition ───────► made here
```

An explicit instantiation declaration stops only the definitions that
the unit can do without. The unit still instantiates an inline
function, a `constexpr` function, a function with a deduced return
type, a variable that a constant expression can use, and a class
(INST-27). The unit uses these definitions in constant evaluation and
can expand a call in place. A call can still go to the definition in
the unit with the explicit instantiation definition, so that definition
must exist (INST-28).

An explicit instantiation stands at namespace scope, in the namespace
of its template or in a namespace that encloses it. In an enclosing
namespace, the instantiation names the template with a qualified name,
so the name shows which namespace holds the template. The same placement
holds for a class, a function, and a variable template (INST-21).

The names in an explicit instantiation are not checked for access
(INST-34). A program has at most one explicit instantiation definition
of each specialization. Across translation units, the implementation
need not diagnose a second one (INST-35).
:::

:::{principle} INST-P4
:rules: INST-30 INST-31 INST-32 INST-33

A specialization sees the declarations before its
{term}`point of instantiation`. The point depends on the kind of
entity:

```text
template <class T> struct Info { ... };
template <class T> int info(T);
  ┌─────────────────────────────── point of Info<A>   (INST-30)
int use() { return Info<A>::value + info(A{}); }
  └─────────────────────────────── point of info<A>   (INST-31)
...
end of the translation unit ────── another point of info<A> (INST-31)
```

A class that a function specialization needs has its point right
before the point of that function specialization (INST-30):

```text
template <class T> struct Info { ... };
template <class T> int read() { return Info<T>::value; }
int main() { return read<A>(); }
  ── point of Info<A>   right before the point of read<A>   (INST-30)
  ── point of read<A>   right after main                     (INST-31)
```

A function or `static` data member has a point after the declaration
that uses it, and another point at the end of the translation unit. A
virtual member function has a point right after the point of its
class.
Each translation unit that uses a specialization adds points. All the
points must give the specialization the same meaning (INST-32).
:::

:::{principle} INST-P5
:rules: INST-15

An instantiation can need other instantiations, which can need others
in turn. Each implementation sets a limit on the depth of this
nesting ({rule}`TR-13`). A chain with no end exceeds every limit, so
every implementation rejects it.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   explicit_instantiation: ["extern"] "template" `declaration`
```

The `declaration` declares one specialization, without a body or an
initializer: a class with its class key and {term}`template-id`, a function,
a member function, a `static` data member, or a variable whose name is
a template-id. With `extern`, the form is an explicit instantiation
declaration. Without `extern`, the form is an explicit instantiation
definition.

## Constraints

### Implicit instantiation

:::{rule} INST-1
:kind: required
:tests: templates/instantiation/class-object.cpp templates/instantiation/class-sizeof.cpp templates/instantiation/class-member-access.cpp templates/instantiation/class-delete.cpp templates/instantiation/class-base-conversion.cpp templates/instantiation/class-function-definition-parameter.cpp templates/instantiation/class-function-definition-return.cpp templates/instantiation/class-call-return.cpp templates/instantiation/class-deduction-from-derived.cpp templates/instantiation/class-deduction-from-derived-error.cpp

A specialization of a {term}`class template` is instantiated when a use needs
a complete class, or when the definition can change the meaning of the
use. These uses instantiate the class:

- an object, `sizeof`, and pointer arithmetic,
- a member access, also through a pointer,
- `delete` through a pointer,
- a conversion of a pointer or reference to a base class,
- the definition of a function whose parameter or return type is the
  class,
- a call to a function that returns the class,
- a {term}`deduction` that searches the bases of the class ({rule}`DED-15`).
:::

:::{rule} INST-2
:kind: required
:tests: templates/instantiation/class-pointer-only.cpp templates/instantiation/class-defined-after-pointer.cpp classes/virtual/override-dependent-base-pointer.cpp

A pointer or reference type, an alias, a template argument, and a
function declaration can name a specialization of a class template
without its instantiation.
:::

:::{rule} INST-4
:kind: reject
:tests: templates/instantiation/class-declared-not-defined.cpp templates/instantiation/class-defined-after-pointer.cpp

If the class template is declared and not yet defined at a use that
needs a complete class, the specialization is incomplete there. The
use is not valid ({rule}`TSP-20` gives the same for an {term}`explicit specialization`).
:::

:::{rule} INST-5
:kind: required
:tests: templates/instantiation/class-member-definitions-unused.cpp templates/instantiation/class-inline-static-member-unused.cpp templates/instantiation/class-scoped-enumeration-unused.cpp templates/instantiation/class-deleted-member-specialized.cpp

The instantiation of a class makes the declarations of its members. It
does not make these definitions:

- member function bodies, also deleted ones, default arguments, and
  exception specifications,
- default member initializers,
- the definitions and initializers of `static` data members, also
  `inline` ones ({question}`Q-0231`),
- the definitions of nested classes, scoped enumerations
  ({question}`Q-0232`), and member templates.
:::

:::{rule} INST-6
:kind: reject
:tests: templates/dependent-names/dependent-error-at-instantiation.cpp templates/dependent-names/value-dependent-bound-fails.cpp templates/instantiation/class-base-conversion.cpp templates/instantiation/class-delete.cpp templates/instantiation/class-member-access.cpp templates/instantiation/class-member-declaration-fails.cpp templates/instantiation/class-object.cpp templates/instantiation/class-sizeof.cpp templates/instantiation/class-unscoped-enumeration.cpp templates/instantiation/default-argument-used.cpp templates/instantiation/default-member-initializer-used.cpp templates/instantiation/explicit-class-members.cpp templates/instantiation/explicit-declaration-inline-still-made.cpp templates/instantiation/explicit-member-class.cpp templates/instantiation/function-address.cpp templates/instantiation/function-called.cpp templates/instantiation/function-deduced-return.cpp templates/instantiation/local-class-member.cpp templates/instantiation/member-function-called.cpp templates/instantiation/static-member-used.cpp templates/instantiation/variable-template-used.cpp templates/instantiation/class-static-member-function-type.cpp templates/instantiation/member-partial-specializations-collide.cpp templates/instantiation/class-function-definition-parameter.cpp templates/instantiation/class-function-definition-return.cpp templates/instantiation/class-call-return.cpp templates/instantiation/function-unevaluated-braced.cpp templates/instantiation/function-unevaluated-braced-untaken.cpp templates/instantiation/default-argument-explicit-specialization.cpp templates/instantiation/default-member-initializer-aggregate-left-out.cpp templates/instantiation/local-class-default-argument.cpp templates/instantiation/local-class-member-initializer.cpp templates/instantiation/local-class-exception-specification.cpp templates/instantiation/local-enumeration.cpp templates/instantiation/class-deduction-from-derived-error.cpp templates/instantiation/member-template-pointer-to-member-non-class.cpp templates/instantiation/class-data-member-function-type.cpp

Each declaration and definition that an instantiation makes must be
valid for the arguments of the specialization. These include:

- the member declarations made with a class (INST-5),
- the definition of an unscoped member enumeration, which is made with
  the class,
- each definition that a use or an explicit instantiation needs
  (INST-1, INST-7 to INST-10, INST-12, INST-17, INST-27).

A member declared without a function {term}`declarator` must not get a
function type from the arguments. Two member {term}`partial specializations <partial specialization>`
must not become the same partial specialization ({question}`Q-0251`).
:::

:::{rule} INST-7
:kind: required
:tests: templates/instantiation/function-unevaluated.cpp templates/instantiation/function-called.cpp templates/instantiation/function-address.cpp templates/instantiation/function-deduced-return.cpp templates/instantiation/member-function-called.cpp templates/instantiation/function-unevaluated-braced.cpp templates/instantiation/function-unevaluated-braced-untaken.cpp templates/instantiation/function-unevaluated-parentheses.cpp templates/instantiation/function-unevaluated-sizeof-constant.cpp

A {term}`function template` specialization or a member function of a class
specialization is instantiated when a use needs its definition, such as
a call or taking its address. `decltype`, `sizeof`, and `noexcept` of a
call do not instantiate the function, except in these cases:

- the type of the call needs a deduced return type,
- the function is `constexpr`, and a braced initializer in the operand
  names the function ({question}`Q-0252`).
:::

:::{rule} INST-8
:kind: required
:tests: templates/instantiation/static-member-used.cpp templates/instantiation/static-member-not-initialized.cpp templates/instantiation/variable-template-unevaluated.cpp templates/instantiation/variable-template-used.cpp

A `static` data member of a class specialization, or a specialization
of a variable template, is instantiated when a use needs its
definition. Before such a use, its initializer is not instantiated and
its initialization does not run.
:::

:::{rule} INST-9
:kind: required
:tests: templates/instantiation/default-argument-not-used.cpp templates/instantiation/default-argument-used.cpp templates/instantiation/default-argument-explicit-instantiation.cpp templates/instantiation/default-argument-each-own.cpp templates/instantiation/default-argument-explicit-specialization.cpp

A default argument of a function template or of a member function is
instantiated when a call uses the default argument. Each default
argument is instantiated on its own. A call that gives the argument
and an explicit instantiation do not instantiate the default. A call to
an explicit specialization that leaves out the argument instantiates
the template's default.
:::

:::{rule} INST-10
:kind: required
:tests: templates/instantiation/default-member-initializer-not-used.cpp templates/instantiation/default-member-initializer-used.cpp templates/instantiation/default-member-initializer-aggregate.cpp templates/instantiation/default-member-initializer-aggregate-left-out.cpp

A default member initializer of a class specialization is instantiated
when a constructor or an aggregate initialization uses the initializer.
A constructor or an aggregate initialization that initializes the
member itself does not use the initializer.
:::

:::{rule} INST-12
:kind: required
:tests: templates/instantiation/local-class-member.cpp templates/instantiation/local-class-default-argument.cpp templates/instantiation/local-class-member-initializer.cpp templates/instantiation/local-class-exception-specification.cpp templates/instantiation/local-enumeration.cpp templates/instantiation/local-class-requires-clause-waits.cpp templates/instantiation/local-class-requires-clause-checked-at-use.cpp templates/instantiation/local-class-requires-clause-substitution-failure.cpp templates/instantiation/local-class-requires-clause-substitution-failure-call.cpp

A class or enumeration declared in the body of a function template is
part of the function. The instantiation of the function instantiates
the local class with these parts, also those that no use names:

- member function bodies and exception specifications,
- default member initializers,
- default arguments ({question}`Q-0253`).

A requires clause of a member function of the local class is not
instantiated with the class. It is checked when a use of the member
needs it ({question}`Q-0392`). A substitution failure in the clause
makes the constraint not satisfied at that use. So a requires
expression that names the call is false, and the call itself is not
valid.
:::

:::{rule} INST-36
:kind: required
:tests: templates/instantiation/local-entities-per-specialization.cpp templates/instantiation/local-entities-thread-local-and-nested-lambda.cpp

Each specialization of a function template has its own `static` and
`thread_local` local variables. Each lambda expression in its body has
a different closure type in each specialization. The call operator of a
generic lambda is a function template ({rule}`LAM-15`), so the rule
applies to it.
:::

:::{rule} INST-13
:kind: reject
:tests: templates/instantiation/friend-defined-twice.cpp templates/instantiation/friend-defined-per-specialization.cpp

A friend function defined in a class template is defined again by each
instantiation of the class. Two class specializations must not define
the same function. A friend whose trailing clause names the class's
template parameters is a different function in each specialization
({rule}`CNS-32`).
:::

### Explicit instantiation

:::{rule} INST-16
:kind: required
:tests: templates/instantiation/explicit-definition-used-elsewhere.cpp templates/instantiation/explicit-member-function-only.cpp templates/instantiation/explicit-definition-before-template-definition.cpp

An {term}`explicit instantiation definition` makes the definition of
the specialization that it names, in its own translation unit. It can
name these kinds of specialization:

- functions and member functions,
- `static` data members and variables of a variable template,
- classes and member classes.
:::

:::{rule} INST-17
:kind: required
:tests: templates/instantiation/explicit-class-members.cpp templates/instantiation/explicit-member-class.cpp templates/instantiation/explicit-class-skips.cpp

An explicit instantiation definition of a class instantiates each member
that is defined at that point. It does not instantiate these:

- member templates,
- members that have an explicit specialization,
- members that are declared and not defined,
- default arguments of member functions.
:::

:::{rule} INST-18
:kind: required
:tests: templates/instantiation/explicit-after-specialization.cpp templates/instantiation/explicit-after-member-specialization.cpp

An explicit instantiation of a specialization that has an explicit
specialization has no effect. The explicit specialization stays the
definition.
:::

:::{rule} INST-19
:kind: reject
:tests: templates/instantiation/specialization-after-explicit-definition.cpp templates/instantiation/specialization-after-explicit-declaration.cpp

An explicit specialization must not come after an explicit
instantiation of the same specialization ({rule}`TSP-6`,
{question}`Q-0237`).
:::

:::{rule} INST-20
:kind: reject
:tests: templates/instantiation/explicit-function-not-defined.cpp templates/instantiation/explicit-variable-not-defined.cpp templates/instantiation/explicit-class-not-defined.cpp templates/instantiation/explicit-declaration-class-not-defined.cpp templates/instantiation/explicit-definition-before-template-definition.cpp templates/instantiation/explicit-variable-before-definition.cpp

An explicit instantiation definition of a function or variable needs
the definition of its template in the same translation unit. The
definition can come after the explicit instantiation
({question}`Q-0254`). An explicit instantiation of a class, with or
without `extern`, needs the definition of the class template before it.
:::

:::{rule} INST-21
:kind: reject
:tests: templates/instantiation/explicit-in-class.cpp templates/instantiation/explicit-in-block.cpp templates/instantiation/explicit-other-namespace.cpp templates/instantiation/explicit-enclosing-namespace.cpp templates/instantiation/explicit-unqualified-outside-namespace.cpp templates/instantiation/explicit-unqualified-outside-namespace-function.cpp templates/instantiation/explicit-unqualified-outside-namespace-variable.cpp

An explicit instantiation must be at namespace scope, in the namespace
of the template or in a namespace that encloses it. Outside the
namespace of the template, the name must be qualified
({question}`Q-0235`).
:::

:::{rule} INST-22
:kind: reject
:tests: templates/instantiation/explicit-not-declared.cpp templates/instantiation/explicit-not-template.cpp templates/instantiation/explicit-wrong-type.cpp templates/instantiation/explicit-deduced-return-written.cpp templates/instantiation/explicit-deduced-arguments.cpp templates/instantiation/explicit-array-bound-left-out.cpp templates/instantiation/explicit-array-bound-differs.cpp templates/instantiation/explicit-variable-wrong-type.cpp templates/instantiation/explicit-exception-specification-matches.cpp templates/instantiation/explicit-exception-specification-differs.cpp templates/instantiation/explicit-placeholder-variable.cpp

An explicit instantiation must name a specialization of a template that
is declared before it. The declared type must be the type of the
specialization
({question}`Q-0346`), with
these allowances:

- trailing template arguments that deduction finds can be left out
  ({rule}`DED-32`),
- a function with a deduced return type is named with `auto`, not
  with the deduced type,
- a variable template with a {term}`placeholder type` is named with the
  deduced type ({question}`Q-0256`), and a `constexpr` variable
  template is named with the `const` type of the variable, because
  INST-25 forbids `constexpr` there,
- the bound of an array type can be left out ({question}`Q-0236`).

An exception specification on the explicit instantiation must match
the specialization's ({question}`Q-0255`). The explicit instantiation
can leave out the exception specification.
:::

:::{rule} INST-23
:kind: reject
:tests: templates/instantiation/explicit-two-declarators.cpp templates/instantiation/explicit-initializer.cpp templates/instantiation/explicit-class-no-key.cpp templates/instantiation/explicit-class-key-mismatch.cpp templates/instantiation/explicit-class-key-same-kind.cpp

An explicit instantiation of a function or variable has one declarator
and no initializer or body. An explicit instantiation of a class has a
class key. The key is `union` for a union template, and `class` or
`struct` for any other class template.
:::

:::{rule} INST-24
:kind: reject
:tests: templates/instantiation/explicit-implicit-member.cpp templates/instantiation/explicit-implicit-destructor.cpp

An explicit instantiation must not name a {term}`special member function` that
the implementation declares implicitly.
:::

:::{rule} INST-25
:kind: reject
:tests: templates/instantiation/explicit-inline.cpp templates/instantiation/explicit-constexpr.cpp templates/instantiation/explicit-attribute-before.cpp templates/instantiation/explicit-storage-class.cpp templates/instantiation/explicit-attribute-after.cpp templates/instantiation/explicit-thread-local.cpp

An explicit instantiation must not have these:

- `inline`, `constexpr`, or `consteval`,
- a storage class specifier other than `thread_local`,
- an attribute ({question}`Q-0233`).
:::

:::{rule} INST-26
:kind: reject
:tests: templates/instantiation/explicit-definition-twice.cpp templates/instantiation/explicit-declaration-after-definition.cpp templates/instantiation/explicit-declaration-then-definition.cpp

A translation unit must not have two explicit instantiation definitions
of one specialization. An explicit instantiation declaration must not
follow the explicit instantiation definition. A declaration can come
before the definition and can be repeated.
:::

:::{rule} INST-27
:kind: required
:tests: templates/instantiation/explicit-definition-used-elsewhere.cpp templates/instantiation/explicit-declaration-then-definition.cpp templates/instantiation/explicit-declaration-inline-still-made.cpp templates/instantiation/explicit-declaration-constant.cpp templates/instantiation/explicit-declaration-constant-variable.cpp

An {term}`explicit instantiation declaration` tells the implementation
that some translation unit has the explicit instantiation definition.
The unit then need not make the definition of a function that is not
inline, of a `static` data member, or of a variable. These are still
instantiated:

- inline functions, including member functions defined in the class,
- `constexpr` functions and functions with a deduced return type,
- variables that a constant expression can use, such as a `constexpr`
  variable or a `const` integer `static` data member,
- the class itself.

The unit can use these definitions in a constant expression and can
expand a call to such a function in place. The unit can also call the
definition in the unit that has the explicit instantiation definition.
So that definition must exist, also for an inline function
({rule}`INST-28`).
:::

:::{rule} INST-29
:kind: reject
:tests: templates/instantiation/explicit-declaration-internal.cpp templates/instantiation/explicit-definition-internal.cpp templates/instantiation/explicit-declaration-internal-class.cpp templates/instantiation/explicit-declaration-internal-variable.cpp

An explicit instantiation declaration must not name a specialization
with internal {term}`linkage`. An explicit instantiation definition can
({question}`Q-0234`).
:::

:::{rule} INST-34
:kind: required
:tests: templates/instantiation/explicit-private-argument.cpp templates/instantiation/explicit-private-member-argument.cpp

Access checking does not apply to the names in an explicit
instantiation. For example, a template argument can be a private member
of a class.
:::

### Points of instantiation

:::{rule} INST-30
:kind: required
:tests: templates/instantiation/class-point-sees-earlier.cpp templates/instantiation/class-point-not-later.cpp templates/instantiation/class-point-in-specialization.cpp

The {term}`point of instantiation` of a class specialization is right
before the namespace-scope declaration that first needs the class. If
that declaration is in another specialization, the point is right
before the point of that specialization. The class sees the
declarations before its point.
:::

:::{rule} INST-31
:kind: required
:tests: templates/instantiation/function-point-constant.cpp templates/instantiation/function-point-sees-earlier.cpp templates/instantiation/function-point-end-of-unit.cpp templates/instantiation/virtual-point-after-class.cpp templates/instantiation/function-point-constexpr-later.cpp templates/dependent-names/generic-lambda-argument-lookup.cpp

A specialization of a function or of a `static` data member has these
points of instantiation:

- right after the namespace-scope declaration that uses the
  specialization, or the point of the specialization that uses it,
- for a virtual member function, right after the point of its class,
- the end of the translation unit,
- an explicit instantiation definition.

A use inside a function counts at the namespace-scope declaration that
holds the function, also when the function is not a template. A use can
see the declarations up to the last point, and INST-32 applies where
the points differ. A {term}`constant evaluation` that needs a declaration after
the first point is such a case ({question}`Q-0257`).

The call operator of a generic lambda is a member function template
({rule}`LAM-15`). Each of its specializations has the points above,
set by the declarations that use the specialization. The place of the
lambda expression does not set a point.
:::

:::{rule} INST-33
:kind: reject
:tests: templates/instantiation/class-point-not-later.cpp templates/instantiation/class-point-not-later-type.cpp

An instantiation does not see a declaration after its point of
instantiation. A class specialization that needs such a declaration is
not valid. The initializer of a `static constexpr` data member in the
class body is instantiated with the class, so this rule covers it too.
INST-32 covers a function specialization ({question}`Q-0257`).
:::

:::{rule} INST-15
:kind: reject
:tests: templates/instantiation/recursion-endless.cpp templates/instantiation/recursion-endless-function.cpp

An instantiation must not need an endless chain of other
instantiations. Such a chain goes past the limit of {rule}`TR-13` on
every implementation.
:::

## Implementation Requirements

:::{rule} INST-3
:kind: unspecified
:tests: templates/instantiation/class-overload-void-pointer.cpp templates/instantiation/class-overload-two-candidates.cpp

If overload resolution can choose a function without the definition of
a class specialization, the class is or is not instantiated.
:::

:::{rule} INST-11
:kind: unspecified
:tests: templates/instantiation/virtual-object.cpp templates/instantiation/virtual-sizeof.cpp

When a class specialization is instantiated, a virtual member function
that no use names is or is not instantiated.
:::

:::{rule} INST-14
:kind: withdrawn

This rule merged into {rule}`TR-13`.
:::

:::{rule} INST-28
:kind: no-diagnostic
:tests: templates/instantiation/explicit-declaration-no-definition.cpp templates/instantiation/explicit-declaration-no-definition-inline.cpp templates/instantiation/explicit-declaration-no-definition-member.cpp templates/instantiation/explicit-declaration-no-definition-variable.cpp

If a program uses a specialization that an explicit instantiation
declaration names, some translation unit must have its explicit
instantiation definition. The rule holds also for an inline function.
An implementation need not diagnose a break of this rule.
:::

:::{rule} INST-32
:kind: no-diagnostic
:tests: templates/instantiation/function-point-end-of-unit.cpp templates/instantiation/function-point-better-later.cpp templates/instantiation/points-in-two-units.cpp templates/instantiation/virtual-point-after-class.cpp templates/instantiation/function-point-constexpr-later.cpp templates/instantiation/function-point-constant.cpp

All the points of instantiation of a specialization, in one translation
unit or in several, must give the specialization the same meaning. An
implementation need not diagnose a break of this rule.
:::

:::{rule} INST-35
:kind: no-diagnostic
:tests: templates/instantiation/explicit-definition-in-two-units.cpp templates/instantiation/explicit-specialization-in-other-unit.cpp

A program must have at most one explicit instantiation definition of a
specialization. An explicit instantiation must not name a
specialization that has an explicit specialization, unless the explicit
specialization comes before it in the same translation unit (INST-18).
Across translation units, an implementation need not diagnose a break
of this rule. INST-19 and INST-26 give the cases in one unit.
:::

## Examples

What a use instantiates:

% tidal-example: reject
```cpp
template <class T> struct Wrapper {
  T value;
  int size() { return value.size(); }   // not valid for int
};

Wrapper<int>* p = nullptr;               // no instantiation
Wrapper<int> w;                          // the class: declarations only
int n = w.size();                        // rejected: the body is instantiated
```

An explicit instantiation in one unit, used from another:

% tidal-example: fragment
```cpp
// unit A
template <class T> T twice(T x) { return x + x; }
extern template int twice(int);          // the definition is elsewhere
int a = twice(4);

// unit B
template <class T> T twice(T x) { return x + x; }
template int twice(int);                 // the definition
```

In the next example, a header holds a class template, and one unit
makes a specialization for the program. The header defines the members after
the class body. A member defined in the body is inline, and each unit
makes that member anyway (INST-27). A header can define the `static`
data member of a class template ({principle}`ODR-P2`):

```cpp
// fixed.h
#ifndef FIXED_H
#define FIXED_H
template <int Frac> struct Fixed {
  long long raw;
  static int made;                       // one object for each specialization
  static Fixed ratio(int n, int d);      // not inline: defined below
  long long floor() const;
};
template <int Frac> int Fixed<Frac>::made = 0;
template <int Frac> Fixed<Frac> Fixed<Frac>::ratio(int n, int d) {
  ++made;
  return Fixed{(static_cast<long long>(n) << Frac) / d};
}
template <int Frac> long long Fixed<Frac>::floor() const {
  return raw >> Frac;
}
extern template struct Fixed<16>;        // instances.cpp makes Fixed<16>
#endif
```

% tidal-example: fragment
```cpp
// instances.cpp
#include "fixed.h"
template struct Fixed<16>;               // makes ratio, floor, and made

// user.cpp
#include "fixed.h"
long long half() {                       // calls ratio in instances.cpp
  return Fixed<16>::ratio(1, 2).raw;
}
int count() { return Fixed<16>::made; }  // the one made of instances.cpp
```

Points of instantiation:

```cpp
namespace shop { struct Item {}; }
template <class T> struct Price { static constexpr int value = cost(T{}); };

namespace shop { constexpr int cost(Item) { return 3; } }
int p = Price<shop::Item>::value;        // 3: cost is declared before the point

auto price = [](auto item) { return cost(item); };
int q() { return price(shop::Item{}); }  // 3: the point follows q, not the lambda
```

The same class with the declaration after the point:

% tidal-example: reject
```cpp
namespace shop { struct Item {}; }
template <class T> struct Price { static constexpr int value = cost(T{}); };

int p = Price<shop::Item>::value;        // rejected: the point is before cost (INST-33)
namespace shop { constexpr int cost(Item) { return 3; } }
```

Explicit instantiations of a function with a deduced return type and of
a `constexpr` variable template (INST-22, INST-25):

% tidal-example: reject
```cpp
template <class T> auto get(T) { return 1; }
template auto get(int);                  // fine
template int get(long);                  // rejected: named with the deduced type

template <class T> constexpr int size_of = sizeof(T);
extern template const int size_of<int>;  // fine: the type is const int
extern template constexpr int size_of<long>;  // rejected: constexpr (INST-25)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 119
of 143 tests.

### Divergences

Each settled question below keeps its test's override for the family
that still differs. Reporting the EDG differences to the EDG project
waits on the maintainer (`documentation/decisions/pending.md`).

- {question}`Q-0231` (INST-5; settled on INST-P2): whether the
  initializer of an unused `inline static` data member is instantiated
  with the class. It waits for a use, as Clang and GCC say. EDG
  instantiates it.
- {question}`Q-0232` (INST-5; settled on INST-P2): whether the
  enumerators of a scoped member enumeration are instantiated with the
  class. They wait for a use, as Clang and EDG say. GCC 16 instantiates
  them.
- {question}`Q-0251` (INST-6; open): whether a class whose two member
  partial specializations become the same is valid. The instantiation
  is not valid, as Clang says. GCC and EDG accept the class.
- {question}`Q-0252` (INST-7; settled on INST-P1): when an unevaluated
  operand instantiates a `constexpr` function template. A braced
  initializer instantiates each one it names, also on a branch that is
  not taken, and other unevaluated operands do not. EDG differs on the
  braced conditional, and GCC 16 on `sizeof`.
- {question}`Q-0253` (INST-12; settled on INST-P1): whether the default
  argument of a member function of a local class is instantiated with
  the enclosing function. It is, as Clang and EDG say. GCC 16 waits for
  a call.
- {question}`Q-0392` (INST-12, INST-P1; settled): whether a member
  function of a local class in a function template can have a requires
  clause. It can, and the clause waits for a use of the member, as
  Clang 22 and GCC 16 say. EDG 7.0 rejects the requires clause.
- {question}`Q-0237` (INST-19; settled on INST-P3): whether an explicit
  specialization can follow an explicit instantiation declaration. It
  cannot, as Clang and GCC say. EDG accepts it.
- {question}`Q-0254` (INST-20; settled on INST-P3): whether a variable
  template's definition can follow its explicit instantiation
  definition. It can, as Clang and EDG say. GCC 16 rejects the order.
- {question}`Q-0235` (INST-21; open): whether an explicit instantiation
  outside the template's namespace can use an unqualified name after a
  using-directive. It cannot, for a class, a function, or a variable
  template. EDG accepts the class form, and Clang 22 and EDG accept the
  variable form. The gap reviews read the class and variable forms as
  valid, and the choice waits on the maintainer
  (`documentation/decisions/pending.md`, row 5).
- {question}`Q-0236` (INST-22; settled on INST-P3): whether an explicit
  instantiation of a variable can leave out the array bound. It can, as
  GCC and EDG say. Clang 22 rejects it.
- {question}`Q-0346` (INST-22; open): whether an explicit instantiation
  of a variable can state a type that differs from the
  specialization's. It cannot, as Clang says. GCC accepts every such
  type, and EDG accepts a bound that differs where the template has no
  bound.
- {question}`Q-0255` (INST-22; settled on INST-P3): whether the
  exception specification of an explicit instantiation must match the
  template. A stated one must match, as Clang and EDG say. GCC 16
  accepts one that differs.
- {question}`Q-0256` (INST-22; settled on INST-P3): how an explicit
  instantiation names a variable template declared with `auto`. It
  writes the deduced type, as Clang and GCC say. EDG rejects the
  deduced type and accepts `auto`, and GCC 16 also accepts another
  type.
- {question}`Q-0233` (INST-25; open): whether an explicit instantiation
  can have a storage class or an attribute after `template`. Only
  `thread_local` is valid, as Clang says. GCC accepts `static` and the
  attribute, and EDG rejects `thread_local`.
- {question}`Q-0234` (INST-29; open): whether an explicit instantiation
  declaration can name a specialization with internal linkage. It
  cannot, as Clang says. GCC accepts every form, and EDG accepts the
  class and variable forms.
- {question}`Q-0257` (INST-31, INST-32; settled on INST-P4): whether an
  implementation must reject a constant evaluation that needs a
  declaration after the first point of instantiation. The points
  differ, and INST-32 needs no diagnostic. No family differs, since
  every compiler rejects.
- {question}`Q-0310` (INST-31, INST-32; settled on INST-P4): which
  point of instantiation a `constexpr` function specialization uses
  outside constant evaluation. Either point, under INST-32. GCC takes
  the end of the unit and Clang and EDG the first point, which INST-32
  allows, so no family differs.

### Split tests

EDG runs no programs, so its cell for a run test gives the result of
the compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `class-inline-static-member-unused.cpp` | INST-5 | accept | accept | accept | reject | {question}`Q-0231` |
| `class-scoped-enumeration-unused.cpp` | INST-5 | accept | accept | reject | accept | {question}`Q-0232` |
| `explicit-array-bound-differs.cpp` | INST-22 | reject | reject | accept | accept | {question}`Q-0346` |
| `explicit-array-bound-left-out.cpp` | INST-22 | accept | reject | accept | accept | {question}`Q-0236` |
| `explicit-attribute-after.cpp` | INST-25 | reject | reject | accept | reject | {question}`Q-0233` |
| `explicit-declaration-internal-class.cpp` | INST-29 | reject | reject | accept | accept | {question}`Q-0234` |
| `explicit-declaration-internal-variable.cpp` | INST-29 | reject | reject | accept | accept | {question}`Q-0234` |
| `explicit-declaration-internal.cpp` | INST-29 | reject | reject | accept | reject | {question}`Q-0234` |
| `explicit-exception-specification-differs.cpp` | INST-22 | reject | reject | accept | reject | {question}`Q-0255` |
| `explicit-placeholder-variable.cpp` | INST-22 | accept | accept | accept | reject | {question}`Q-0256` |
| `explicit-storage-class.cpp` | INST-25 | reject | reject | accept | reject | {question}`Q-0233` |
| `explicit-thread-local.cpp` | INST-25 | accept | accept | accept | reject | {question}`Q-0233` |
| `explicit-unqualified-outside-namespace-variable.cpp` | INST-21 | reject | accept | reject | accept | {question}`Q-0235` |
| `explicit-unqualified-outside-namespace.cpp` | INST-21 | reject | reject | reject | accept | {question}`Q-0235` |
| `explicit-variable-before-definition.cpp` | INST-20 | accept | accept | reject | accept | {question}`Q-0254` |
| `explicit-variable-wrong-type.cpp` | INST-22 | reject | reject | accept | reject | {question}`Q-0346` |
| `function-point-constexpr-later.cpp` | INST-32, INST-31 | run | reject | run | reject | {question}`Q-0310` |
| `function-unevaluated-braced-untaken.cpp` | INST-7, INST-6 | reject | reject | reject | accept | {question}`Q-0252` |
| `function-unevaluated-sizeof-constant.cpp` | INST-7 | accept | accept | reject | accept | {question}`Q-0252` |
| `local-class-default-argument.cpp` | INST-12, INST-6 | reject | reject | accept | reject | {question}`Q-0253` |
| `local-class-requires-clause-substitution-failure.cpp` | INST-12, CNS-12 | run | run | run | reject | {question}`Q-0392` |
| `local-class-requires-clause-waits.cpp` | INST-12 | run | run | run | reject | {question}`Q-0392` |
| `member-partial-specializations-collide.cpp` | INST-6 | reject | reject | accept | accept | {question}`Q-0251` |
| `specialization-after-explicit-declaration.cpp` | INST-19 | reject | reject | reject | accept | {question}`Q-0237` |

### Notes

- INST-5: every compiler accepts an explicit specialization of a
  deleted member function after the class is instantiated. The review
  predicts that the class makes the deleted definition, so that the
  explicit specialization is a second definition.
- INST-9: a call to an explicit specialization that leaves out an
  argument instantiates the template's default argument on every
  compiler.
- INST-27: after an explicit instantiation declaration of an `auto`
  variable template, GCC and EDG reject a use and Clang fails to link.
  The page has no rule for that case.
- INST-31: every compiler accepts a virtual function whose call is
  resolved only by a declaration after the class's point. INST-32
  allows the result. The review predicts a rejection.
- INST-31, INST-32: every compiler rejects a constant evaluation that
  needs a declaration after the first point of instantiation. INST-32
  allows the rejection. The gap review (item 31) and the gap review of
  question cluster 1 read the case as one that needs no diagnostic, as
  INST-P4 gives for points that differ ({question}`Q-0257`).
- INST-32, INST-35: {rule}`ODR-10` and {rule}`ODR-14` state the
  rules for template definitions across translation units.
- INST-33: the gap review of the text phase 8 wrote on templates
  (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`)
  finds no clear support in the reference for instantiating the
  initializer of a `static constexpr` data member with the class, since
  the reference delays the initialization of a static data member until
  a use needs its definition. Every compiler instantiates the
  initializer with the class, and INST-33 follows the compilers.
- INST-12, INST-P1: the same review reads the constraints of a member
  of a local class as left out of the instantiation of the class
  ({question}`Q-0392`).
- INST-3: every compiler instantiates the class when a pointer to it
  converts to `void*` in a call. The rule allows either result.
- INST-11: every compiler instantiates an unused virtual function when
  the program defines an object of the class, and none does for
  `sizeof`. The rule allows either result.
- INST-15: every compiler rejects both tests with its depth message
  ({rule}`TR-13`). On the chain of function templates, Clang 22 and 18
  then end with a segmentation fault.
- INST-28: the baseline builds use no `-O` flag and fail to link in
  each test. The `-O2` report builds inline the inline function and
  run, and fail to link for the others. The rule is `no-diagnostic` and
  allows each result, so the tests record the outcome of each build and
  decide nothing by the optimization level (decision 0047).
- INST-35: two units with an explicit instantiation definition of one
  specialization link with no message on every compiler that links.
  For an explicit instantiation in one unit and an explicit
  specialization in the other, the link also gives no message. Clang 22
  and GCC 16 at `-O0` then call the explicit specialization from both
  units. At `-O2`, the unit with the explicit instantiation inlines the
  template's body and gets its result. The rule allows each result.
- INST-36: every compiler gives one counter to each specialization of
  a function template and of a generic lambda's call operator, and a
  different closure type to each specialization.
- INST-32: every compiler takes the declarations at the end of the
  unit for a function. In two units, the `-O0` builds use one
  definition for both units, and the `-O2` builds inline each call.
  The rule allows each result.

### Probe grids

The tables are in [`tables.md`](https://github.com/sempuki/tidal/blob/main/tests/templates/instantiation/grids/tables.md) beside the grid files.

The probe grids are in `tests/templates/instantiation/grids/`.

Uses of a class specialization (INST-1 to INST-3). The class has a
`static_assert` whose condition is the column. A `reject` in the second
column means that the use instantiates the class.

Uses of functions and variables (INST-5, INST-7, INST-8). The bodies
and initializers have a `static_assert` whose condition is the column.

### Sources

The clean-room writer probed each rule with the eight baseline
compilers and the two `-O2` report compilers, and wrote the grids
above. The rejections name the reason in each compiler's own words, for
example "explicit instantiation of undefined function template" and
"duplicate explicit instantiation" (clang-diag-sema).

The clean-room writer probed each item of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
with the same compilers. Where the compilers differ from the outcome
that the review predicts, the rules follow the compilers, and the notes
above say so.

This table gives the source of each rule. Inventory items have the
numbers of
`reports/review/2026-10-31-inventory-dependent-names-and-instantiation.md`:

| Rule | Source |
|---|---|
| INST-1 to INST-3 | grid class-uses; inventory H4, H15, H16 |
| INST-4 | diagnostics; inventory H1 |
| INST-5, INST-6 | probes; inventory H2, H3, H8, H18 |
| INST-7, INST-8 | grid function-uses; inventory H5, H12, H14, H18, H20, H21 |
| INST-9, INST-10 | probes; inventory H7, H9 |
| INST-11 | probes; inventory H22 |
| INST-12 | probes; inventory B5 |
| INST-13 | diagnostics; inventory H10 |
| INST-14, INST-15 | diagnostics; inventory H13 |
| INST-16 to INST-18 | probes; clang-diag-sema; inventory G2, G3, G7, G10 |
| INST-19 | diagnostics; inventory G10 |
| INST-20 to INST-26 | diagnostics; clang-diag-sema; inventory G1, G5, G6, G8, G13, G15 to G17 |
| INST-27, INST-28 | two-unit probes; inventory G11, G12 |
| INST-29 | clang-diag-sema; inventory G14 |
| INST-30 to INST-32 | probes; inventory I1, I2, I4, I6 to I8, I10 |
| INST-34, INST-35 | probes; two-unit probes; gap review items 36, 39 |

Gap review items 24 to 39 also gave probes for INST-1, INST-5 to
INST-7, INST-9, INST-10, INST-12, INST-20, INST-22, INST-23, INST-25,
INST-27, and INST-31 to INST-33.

Inventory items left out of this page:

- G4 and H6 belong to {doc}`constraints`.
- G9 and G18, a destructor and the declaration of a variable in an
  explicit instantiation, have no probe.
- H11 and I5 are {rule}`EXS-9` and {rule}`EXS-10`. H19 is
  {rule}`IF-13`. H14 is {rule}`DED-28` and {rule}`DED-29`.
- H17, I3, I11, and I12 have no probe that shows a difference.
- I9 is about modules, which are outside the charter.

Inputs: clang-diag-sema.

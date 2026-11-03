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

The implementation instantiates a specialization only when a use needs
its definition. A program can therefore name a specialization whose
definition would not be valid, as long as no use needs that
definition. A use needs the definition in these cases:

| Entity | Instantiated by | Not instantiated by | Rule |
|---|---|---|---|
| class | an object, `sizeof`, a member access, `delete`, a conversion to a base, a function definition or call with the class by value | a pointer or reference type, an alias, a template argument, a function declaration | INST-1, INST-2 |
| function | a call, taking the address, `decltype` of a call when the return type is deduced, a braced initializer that names a `constexpr` function | `decltype`, `sizeof`, or `noexcept` of a call otherwise | INST-7 |
| `static` data member, variable template | a read or other use of the value | `decltype` | INST-8 |
| default argument | a call that leaves out the argument | a call that gives the argument, an explicit instantiation | INST-9 |
| default member initializer | a constructor or an aggregate initialization that does not initialize the member | a constructor or an aggregate initialization that does | INST-10 |

A discarded statement of `if constexpr` is not instantiated
({rule}`IF-13`). An exception specification is instantiated when a use
needs the specification ({rule}`EXS-9`).
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
| member template | the declaration only |
| unscoped enumeration | the whole definition |
| scoped enumeration | the declaration only ({question}`Q-0232`) |

An error in a declaration made with the class makes the use of the
class not valid (INST-6). Such an error can come from the arguments, for
example a `static` data member whose dependent type becomes a function
type. An error in a definition that waits makes
only the use that needs the definition not valid.
:::

:::{principle} INST-P3
:rules: INST-16 INST-17 INST-18 INST-20 INST-21 INST-22 INST-23 INST-24 INST-25 INST-26 INST-27 INST-28 INST-29 INST-34 INST-35
:exceptions: INST-19

An explicit instantiation lets a program make the definitions of chosen
specializations in one translation unit, and use them from the
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
(INST-27).

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
:rules: INST-14
:exceptions: INST-15

An instantiation can need other instantiations, which can need others
in turn. Each implementation sets a limit on the depth of this
nesting. A chain with no end exceeds every limit.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   explicit_instantiation: ["extern"] "template" `declaration`
```

The `declaration` declares one specialization, without a body or an
initializer: a class with its class key and template-id, a function,
a member function, a `static` data member, or a variable whose name is
a template-id. With `extern`, the form is an explicit instantiation
declaration. Without `extern`, the form is an explicit instantiation
definition.

## Constraints

### Implicit instantiation

:::{rule} INST-1
:kind: required
:tests: templates/instantiation/class-object.cpp templates/instantiation/class-sizeof.cpp templates/instantiation/class-member-access.cpp templates/instantiation/class-delete.cpp templates/instantiation/class-base-conversion.cpp templates/instantiation/class-function-definition-parameter.cpp templates/instantiation/class-function-definition-return.cpp templates/instantiation/class-call-return.cpp templates/instantiation/class-deduction-from-derived.cpp templates/instantiation/class-deduction-from-derived-error.cpp

A specialization of a class template is instantiated when a use needs
a complete class, or when the definition can change the meaning of the
use. These uses instantiate the class:

- an object, `sizeof`, and pointer arithmetic,
- a member access, also through a pointer,
- `delete` through a pointer,
- a conversion of a pointer or reference to a base class,
- the definition of a function whose parameter or return type is the
  class,
- a call to a function that returns the class,
- a deduction that searches the bases of the class ({rule}`DED-15`).
:::

:::{rule} INST-2
:kind: required
:tests: templates/instantiation/class-pointer-only.cpp templates/instantiation/class-defined-after-pointer.cpp

A pointer or reference type, an alias, a template argument, and a
function declaration can name a specialization of a class template
without its instantiation.
:::

:::{rule} INST-4
:kind: reject
:tests: templates/instantiation/class-declared-not-defined.cpp templates/instantiation/class-defined-after-pointer.cpp

If the class template is declared and not yet defined at a use that
needs a complete class, the specialization is incomplete there. The
use is not valid ({rule}`TSP-20` gives the same for an explicit
specialization).
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
:tests: templates/dependent-names/dependent-error-at-instantiation.cpp templates/dependent-names/value-dependent-bound-fails.cpp templates/instantiation/class-base-conversion.cpp templates/instantiation/class-delete.cpp templates/instantiation/class-member-access.cpp templates/instantiation/class-member-declaration-fails.cpp templates/instantiation/class-object.cpp templates/instantiation/class-sizeof.cpp templates/instantiation/class-unscoped-enumeration.cpp templates/instantiation/default-argument-used.cpp templates/instantiation/default-member-initializer-used.cpp templates/instantiation/explicit-class-members.cpp templates/instantiation/explicit-declaration-inline-still-made.cpp templates/instantiation/explicit-member-class.cpp templates/instantiation/function-address.cpp templates/instantiation/function-called.cpp templates/instantiation/function-deduced-return.cpp templates/instantiation/local-class-member.cpp templates/instantiation/member-function-called.cpp templates/instantiation/static-member-used.cpp templates/instantiation/variable-template-used.cpp templates/instantiation/class-static-member-function-type.cpp templates/instantiation/member-partial-specializations-collide.cpp templates/instantiation/class-function-definition-parameter.cpp templates/instantiation/class-function-definition-return.cpp templates/instantiation/class-call-return.cpp templates/instantiation/function-unevaluated-braced.cpp templates/instantiation/function-unevaluated-braced-untaken.cpp templates/instantiation/default-argument-explicit-specialization.cpp templates/instantiation/default-member-initializer-aggregate-left-out.cpp templates/instantiation/local-class-default-argument.cpp templates/instantiation/local-class-member-initializer.cpp templates/instantiation/local-class-exception-specification.cpp templates/instantiation/local-enumeration.cpp templates/instantiation/class-deduction-from-derived-error.cpp

Each declaration and definition that an instantiation makes must be
valid for the arguments of the specialization. These include:

- the member declarations made with a class (INST-5),
- the definition of an unscoped member enumeration, which is made with
  the class,
- each definition that a use or an explicit instantiation needs
  (INST-1, INST-7 to INST-10, INST-12, INST-17, INST-27).

A member declared without a function declarator must not get a
function type from the arguments. Two member partial specializations
must not become the same partial specialization ({question}`Q-0251`).
:::

:::{rule} INST-7
:kind: required
:tests: templates/instantiation/function-unevaluated.cpp templates/instantiation/function-called.cpp templates/instantiation/function-address.cpp templates/instantiation/function-deduced-return.cpp templates/instantiation/member-function-called.cpp templates/instantiation/function-unevaluated-braced.cpp templates/instantiation/function-unevaluated-braced-untaken.cpp templates/instantiation/function-unevaluated-parentheses.cpp templates/instantiation/function-unevaluated-sizeof-constant.cpp

A function template specialization or a member function of a class
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
:tests: templates/instantiation/local-class-member.cpp templates/instantiation/local-class-default-argument.cpp templates/instantiation/local-class-member-initializer.cpp templates/instantiation/local-class-exception-specification.cpp templates/instantiation/local-enumeration.cpp

A class or enumeration declared in the body of a function template is
part of the function. The instantiation of the function instantiates
the local class with these parts, also those that no use names:

- member function bodies and exception specifications,
- default member initializers,
- default arguments ({question}`Q-0253`).
:::

:::{rule} INST-36
:kind: required
:tests: templates/instantiation/local-entities-per-specialization.cpp

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
- a function with a deduced return type is named with `auto`,
- a variable template with a placeholder type is named with the
  deduced type ({question}`Q-0256`),
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

An explicit instantiation must not name a special member function that
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
:::

:::{rule} INST-29
:kind: reject
:tests: templates/instantiation/explicit-declaration-internal.cpp templates/instantiation/explicit-definition-internal.cpp templates/instantiation/explicit-declaration-internal-class.cpp templates/instantiation/explicit-declaration-internal-variable.cpp

An explicit instantiation declaration must not name a specialization
with internal linkage. An explicit instantiation definition can
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
the points differ. A constant evaluation that needs a declaration after
the first point is such a case ({question}`Q-0257`).
:::

:::{rule} INST-33
:kind: reject
:tests: templates/instantiation/class-point-not-later.cpp

An instantiation does not see a declaration after its point of
instantiation. A class specialization that needs such a declaration is
not valid. INST-32 covers a function specialization ({question}`Q-0257`).
:::

## Undefined Behavior

:::{rule} INST-15
:kind: undefined
:tests: templates/instantiation/recursion-endless.cpp templates/instantiation/recursion-endless-function.cpp

If an instantiation needs an endless chain of other instantiations, the
behavior is undefined.
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
:kind: impl-defined
:tests: templates/instantiation/recursion-within-limit.cpp templates/instantiation/recursion-function-within-limit.cpp

The greatest depth of nested instantiations is implementation-defined.
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

```cpp
// unit A
template <class T> T twice(T x) { return x + x; }
extern template int twice(int);          // the definition is elsewhere
int a = twice(4);

// unit B
template <class T> T twice(T x) { return x + x; }
template int twice(int);                 // the definition
```

Points of instantiation:

```cpp
namespace shop { struct Item {}; }
template <class T> struct Price { static constexpr int value = cost(T{}); };

namespace shop { constexpr int cost(Item) { return 3; } }
int p = Price<shop::Item>::value;        // 3: cost is declared before the point
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 114
of 136 tests.

The questions of the cluster of EDG differences (cluster 5) on this page
stay open under decision 0051: {question}`Q-0231`, {question}`Q-0235`,
{question}`Q-0237`, and {question}`Q-0256`. On 2026-11-03, Clang 22 and
GCC 16 still agree with the provisional answer of each, and EDG 7.0
still differs. Reporting these EDG differences to the EDG project waits
on the maintainer. The source-reader gap review of the cluster
(`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads
Q-0235 as EDG does, with doubt, and that question waits on the
maintainer too.

The questions of the cluster of template syntax corners (cluster 6) on
this page stay open under decision 0051: {question}`Q-0233`,
{question}`Q-0234`, {question}`Q-0236`, {question}`Q-0254`,
{question}`Q-0255`, and
{question}`Q-0346`. On
2026-11-03, Clang 22, GCC 16, and EDG 7.0 still split on each of them.
{question}`Q-0235`, {question}`Q-0237`, and {question}`Q-0256` belong
to the same cluster and keep their cluster 5 notes. For
{question}`Q-0235`, a probe of that day found that Clang 22 joins EDG
7.0 for a variable template. The source-reader gap review of the
cluster (`reports/review/2026-11-03-gap-review-question-cluster-6.md`)
agrees with Q-0233, Q-0234, Q-0236, Q-0254, and Q-0255, and with
Q-0346 with doubt. It reads the variable form of Q-0235 as Clang 22 and
EDG 7.0 do, with doubt. INST-21 keeps one answer for every kind of
template, and the variable form waits on the maintainer.

The clean-room writer probed each rule with the eight baseline
compilers and the two `-O2` report compilers, and wrote the grids
below. The rejections name the reason in each compiler's own words, for
example "explicit instantiation of undefined function template" and
"duplicate explicit instantiation" (clang-diag-sema).

The clean-room writer probed each item of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
with the same compilers. Where the compilers differ from the outcome
that the review predicts, the rules follow the compilers, and the
bullets below say so.

- INST-5: EDG instantiates the initializer of an unused `inline static`
  data member with the class. Clang and GCC do not
  ({question}`Q-0231`).
- INST-5: GCC instantiates the enumerators of a scoped enumeration with
  the class. Clang and EDG do not ({question}`Q-0232`).
- INST-25: GCC accepts `static` and an attribute after `template` in an
  explicit instantiation. Clang and EDG reject both. Every compiler
  rejects an attribute before `template`. Clang and GCC accept
  `thread_local`, and EDG rejects it ({question}`Q-0233`). The split
  is the same with `extern`, and for an attribute after `struct` in the
  explicit instantiation of a class.
- INST-5: every compiler accepts an explicit specialization of a
  deleted member function after the class is instantiated. The review
  predicts that the class makes the deleted definition, so that the
  explicit specialization is a second definition.
- INST-6: Clang rejects a class whose two member partial
  specializations become the same. GCC and EDG accept the class
  ({question}`Q-0251`).
- INST-7: every compiler instantiates a `constexpr` function that a
  braced initializer in `decltype` names. Clang and GCC also do so on
  the branch of a conditional that is not taken, and EDG does not. GCC
  also instantiates a `constexpr` function in a `sizeof` operand
  without braces, and Clang and EDG do not ({question}`Q-0252`).
- INST-9: a call to an explicit specialization that leaves out an
  argument instantiates the template's default argument on every
  compiler.
- INST-12: Clang and EDG instantiate the default argument of a local
  class's member function with the enclosing function. GCC does not
  ({question}`Q-0253`).
- INST-20: Clang and EDG accept a variable template definition after
  its explicit instantiation definition. GCC rejects it
  ({question}`Q-0254`). Every compiler accepts the same order for a
  function and emits the specialization. Every compiler accepts the
  same order for a class template's static data member.
- INST-22: Clang and EDG reject an explicit instantiation whose
  `noexcept` differs from the template. GCC accepts it
  ({question}`Q-0255`). The split is the same for `noexcept(false)`,
  for a member function, and with `extern`. Clang and GCC accept `int` for a variable
  template declared with `auto`, and EDG rejects it
  ({question}`Q-0256`).
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
- INST-29: GCC accepts an explicit instantiation declaration of a
  function template with internal linkage. Clang and EDG reject it
  ({question}`Q-0234`). For a class template in an unnamed namespace
  and for a `static` variable template, GCC and EDG accept it, and
  Clang rejects it.
- INST-21: EDG accepts an unqualified explicit instantiation outside
  the template's namespace, after a using-directive. Clang and GCC
  reject it ({question}`Q-0235`). For a variable template, Clang and
  EDG accept the unqualified name, and GCC rejects it. For a function
  template, every compiler rejects it.
- INST-22: Clang rejects an explicit instantiation that leaves out an
  array bound. GCC and EDG accept it ({question}`Q-0236`). Every
  compiler accepts an explicit instantiation declaration that leaves
  out the bound.
- INST-22: GCC accepts an explicit instantiation of a variable whose
  stated type differs from the specialization's, and keeps the
  specialization's type. Clang rejects every such type. EDG rejects it
  except for a bound that differs where the template has no bound
  ({question}`Q-0346`).
- INST-19: EDG accepts an explicit specialization after an explicit
  instantiation declaration. Clang and GCC reject it
  ({question}`Q-0237`).
- INST-3: every compiler instantiates the class when a pointer to it
  converts to `void*` in a call. The rule allows either result.
- INST-11: every compiler instantiates an unused virtual function when
  the program defines an object of the class, and none does for
  `sizeof`. The rule allows either result.
- INST-15: the behavior happens during translation, so no run test
  covers the rule.
- INST-14: Clang stops at a depth of 1024, GCC at 900, and EDG at 64
  nested class instantiations. INST-15 allows any result for an endless
  chain, and every compiler rejects the test.
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
- INST-31, INST-32: a `constexpr` function template is used in an
  ordinary function, outside constant evaluation, and the function that
  its call finds is declared after that function. GCC takes the
  declarations at the end of the unit. Clang and EDG instantiate at
  the first point and reject ({question}`Q-0310`). The call operator of
  a lambda is `constexpr` when it can be, so a generic lambda gives the
  same split.
- INST-36: every compiler gives one counter to each specialization of
  a function template and of a generic lambda's call operator, and a
  different closure type to each specialization.
- INST-32: every compiler takes the declarations at the end of the
  unit for a function. In two units, the `-O0` builds use one
  definition for both units, and the `-O2` builds inline each call.
  The rule allows each result.

Uses of a class specialization (INST-1 to INST-3). The class has a
`static_assert` whose condition is the column. A `reject` in the second
column means that the use instantiates the class:

| | valid body | failing body |
|---|---|---|
| pointer variable | accept | accept |
| reference parameter | accept | accept |
| function returning the class, declared | accept | accept |
| alias | accept | accept |
| template argument | accept | accept |
| pointer comparison | accept | accept |
| cast from void pointer | accept | accept |
| object | accept | reject |
| sizeof | accept | reject |
| member access through a pointer | accept | reject |
| pointer arithmetic | accept | reject |
| delete through a pointer | accept | reject |
| conversion to a base pointer | accept | reject |
| call with one void pointer parameter | accept | reject |

Uses of functions and variables (INST-5, INST-7, INST-8). The bodies
and initializers have a `static_assert` whose condition is the column:

| | valid body | failing body |
|---|---|---|
| no use | accept | accept |
| decltype of a call | accept | accept |
| sizeof of a call | accept | accept |
| noexcept of a call | accept | accept |
| decltype of a call, deduced return type | accept | reject |
| call | accept | reject |
| address | accept | reject |
| discarded branch | accept | accept |
| explicit instantiation declaration | accept | accept |
| member function, class used | accept | accept |
| member function, call | accept | reject |
| static data member, class used | accept | accept |
| static data member, read | accept | reject |
| variable template, decltype | accept | accept |
| variable template, read | accept | reject |

The probe grids are in `tests/templates/instantiation/grids/`. This
table gives the source of each rule. Inventory items have the numbers of
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

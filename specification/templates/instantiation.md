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
uses. Constraints on members are not specified yet
*(concepts referenced)*. Modules are outside the charter.

Depends on: template declarations, explicit and partial specialization,
dependent names, the one-definition rule.

## Principles

:::{principle} INST-P1
:rules: INST-1 INST-2 INST-4 INST-7 INST-8 INST-9 INST-10 INST-12 INST-13
:exceptions: INST-3 INST-11

The implementation instantiates a specialization only when a use needs
its definition. A program can therefore name a specialization whose
definition would not be valid, as long as no use needs that
definition. A use needs the definition in these cases:

| Entity | Instantiated by | Not instantiated by | Rule |
|---|---|---|---|
| class | an object, `sizeof`, a member access, `delete`, a conversion to a base | a pointer or reference type, an alias, a template argument, a function declaration | INST-1, INST-2 |
| function | a call, taking the address, `decltype` of a call when the return type is deduced | `decltype`, `sizeof`, or `noexcept` of a call otherwise | INST-7 |
| `static` data member, variable template | a read or other use of the value | `decltype` | INST-8 |
| default argument | a call that leaves out the argument | a call that gives the argument | INST-9 |
| default member initializer | a constructor that does not initialize the member | a constructor that does | INST-10 |

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
class not valid (INST-6). An error in a definition that waits makes
only the use that needs the definition not valid.
:::

:::{principle} INST-P3
:rules: INST-16 INST-17 INST-18 INST-20 INST-21 INST-22 INST-23 INST-24 INST-25 INST-26 INST-27 INST-28 INST-29
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
type, and a class (INST-27).
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

A function or `static` data member has a point after the declaration
that uses it, and another point at the end of the translation unit.
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
:tests: templates/instantiation/class-object.cpp templates/instantiation/class-sizeof.cpp templates/instantiation/class-member-access.cpp templates/instantiation/class-delete.cpp templates/instantiation/class-base-conversion.cpp

A specialization of a class template is instantiated when a use needs
a complete class, or when the definition can change the meaning of the
use. These uses instantiate the class:

- an object, `sizeof`, and pointer arithmetic,
- a member access, also through a pointer,
- `delete` through a pointer,
- a conversion of a pointer or reference to a base class.
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
:tests: templates/instantiation/class-member-definitions-unused.cpp templates/instantiation/class-inline-static-member-unused.cpp templates/instantiation/class-scoped-enumeration-unused.cpp

The instantiation of a class makes the declarations of its members. It
does not make these definitions:

- member function bodies, default arguments, and exception
  specifications,
- default member initializers,
- the definitions and initializers of `static` data members, also
  `inline` ones ({question}`Q-0231`),
- the definitions of nested classes, scoped enumerations
  ({question}`Q-0232`), and member templates.
:::

:::{rule} INST-6
:kind: reject
:tests: templates/dependent-names/dependent-error-at-instantiation.cpp templates/dependent-names/value-dependent-bound-fails.cpp templates/instantiation/class-base-conversion.cpp templates/instantiation/class-delete.cpp templates/instantiation/class-member-access.cpp templates/instantiation/class-member-declaration-fails.cpp templates/instantiation/class-object.cpp templates/instantiation/class-sizeof.cpp templates/instantiation/class-unscoped-enumeration.cpp templates/instantiation/default-argument-used.cpp templates/instantiation/default-member-initializer-used.cpp templates/instantiation/explicit-class-members.cpp templates/instantiation/explicit-declaration-inline-still-made.cpp templates/instantiation/explicit-member-class.cpp templates/instantiation/function-address.cpp templates/instantiation/function-called.cpp templates/instantiation/function-deduced-return.cpp templates/instantiation/local-class-member.cpp templates/instantiation/member-function-called.cpp templates/instantiation/static-member-used.cpp templates/instantiation/variable-template-used.cpp

Each declaration and definition that an instantiation makes must be
valid for the arguments of the specialization. These include:

- the member declarations made with a class (INST-5),
- the definition of an unscoped member enumeration, which is made with
  the class,
- each definition that a use or an explicit instantiation needs
  (INST-1, INST-7 to INST-10, INST-12, INST-17, INST-27).
:::

:::{rule} INST-7
:kind: required
:tests: templates/instantiation/function-unevaluated.cpp templates/instantiation/function-called.cpp templates/instantiation/function-address.cpp templates/instantiation/function-deduced-return.cpp templates/instantiation/member-function-called.cpp

A function template specialization or a member function of a class
specialization is instantiated when a use needs its definition, such as
a call or taking its address. `decltype`, `sizeof`, and `noexcept` of a
call do not instantiate the function, unless the type of the call needs
a deduced return type.
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
:tests: templates/instantiation/default-argument-not-used.cpp templates/instantiation/default-argument-used.cpp

A default argument of a function template or of a member function is
instantiated when a call uses the default argument. A call that gives
the argument does not instantiate the default.
:::

:::{rule} INST-10
:kind: required
:tests: templates/instantiation/default-member-initializer-not-used.cpp templates/instantiation/default-member-initializer-used.cpp

A default member initializer of a class specialization is instantiated
when a constructor uses the initializer. A constructor that initializes
the member itself does not use the initializer.
:::

:::{rule} INST-12
:kind: required
:tests: templates/instantiation/local-class-member.cpp

A class declared in the body of a function template is part of the
function. The instantiation of the function instantiates the local
class with its member function bodies, also those that no use names.
:::

:::{rule} INST-13
:kind: reject
:tests: templates/instantiation/friend-defined-twice.cpp templates/instantiation/friend-defined-per-specialization.cpp

A friend function defined in a class template is defined again by each
instantiation of the class. Two class specializations must not define
the same function.
:::

### Explicit instantiation

:::{rule} INST-16
:kind: required
:tests: templates/instantiation/explicit-definition-used-elsewhere.cpp templates/instantiation/explicit-member-function-only.cpp

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
:tests: templates/instantiation/explicit-after-specialization.cpp

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
:tests: templates/instantiation/explicit-function-not-defined.cpp templates/instantiation/explicit-variable-not-defined.cpp templates/instantiation/explicit-class-not-defined.cpp templates/instantiation/explicit-declaration-class-not-defined.cpp

An explicit instantiation definition of a function or variable needs
the definition of its template before it, in the same translation unit.
An explicit instantiation of a class, with or without `extern`, needs
the definition of the class template.
:::

:::{rule} INST-21
:kind: reject
:tests: templates/instantiation/explicit-in-class.cpp templates/instantiation/explicit-in-block.cpp templates/instantiation/explicit-other-namespace.cpp templates/instantiation/explicit-enclosing-namespace.cpp templates/instantiation/explicit-unqualified-outside-namespace.cpp

An explicit instantiation must be at namespace scope, in the namespace
of the template or in a namespace that encloses it. Outside the
namespace of the template, the name must be qualified
({question}`Q-0235`).
:::

:::{rule} INST-22
:kind: reject
:tests: templates/instantiation/explicit-not-declared.cpp templates/instantiation/explicit-not-template.cpp templates/instantiation/explicit-wrong-type.cpp templates/instantiation/explicit-deduced-return-written.cpp templates/instantiation/explicit-deduced-arguments.cpp templates/instantiation/explicit-array-bound-left-out.cpp

An explicit instantiation must name a specialization of a template that
is declared before it. The declared type must be the type of the
specialization, with these allowances:

- trailing template arguments that deduction finds can be left out
  ({rule}`DED-32`),
- a function with a deduced return type is named with `auto`,
- the bound of an array type can be left out ({question}`Q-0236`).
:::

:::{rule} INST-23
:kind: reject
:tests: templates/instantiation/explicit-two-declarators.cpp templates/instantiation/explicit-initializer.cpp templates/instantiation/explicit-class-no-key.cpp

An explicit instantiation of a function or variable has one declarator
and no initializer or body. An explicit instantiation of a class has a
class key.
:::

:::{rule} INST-24
:kind: reject
:tests: templates/instantiation/explicit-implicit-member.cpp

An explicit instantiation must not name a special member function that
the implementation declares implicitly.
:::

:::{rule} INST-25
:kind: reject
:tests: templates/instantiation/explicit-inline.cpp templates/instantiation/explicit-constexpr.cpp templates/instantiation/explicit-attribute-before.cpp templates/instantiation/explicit-storage-class.cpp templates/instantiation/explicit-attribute-after.cpp

An explicit instantiation must not have these:

- `inline`, `constexpr`, or `consteval`,
- a storage class specifier,
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
:tests: templates/instantiation/explicit-definition-used-elsewhere.cpp templates/instantiation/explicit-declaration-then-definition.cpp templates/instantiation/explicit-declaration-inline-still-made.cpp templates/instantiation/explicit-declaration-constant.cpp

An {term}`explicit instantiation declaration` tells the implementation
that some translation unit has the explicit instantiation definition.
The unit then need not make the definition of a function that is not
inline, of a `static` data member, or of a variable. These are still
instantiated:

- inline functions, including member functions defined in the class,
- `constexpr` functions and functions with a deduced return type,
- the class itself.
:::

:::{rule} INST-29
:kind: reject
:tests: templates/instantiation/explicit-declaration-internal.cpp templates/instantiation/explicit-definition-internal.cpp

An explicit instantiation declaration must not name a specialization
with internal linkage. An explicit instantiation definition can
({question}`Q-0234`).
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
:tests: templates/instantiation/function-point-constant.cpp templates/instantiation/function-point-sees-earlier.cpp templates/instantiation/function-point-end-of-unit.cpp

A specialization of a function or of a `static` data member has these
points of instantiation:

- right after the namespace-scope declaration that uses the
  specialization, or the point of the specialization that uses it,
- the end of the translation unit,
- an explicit instantiation definition.

A constant evaluation uses the declarations before the first point.
:::

:::{rule} INST-33
:kind: reject
:tests: templates/instantiation/class-point-not-later.cpp templates/instantiation/function-point-constant.cpp

An instantiation does not see a declaration after its point of
instantiation. A class specialization, or a constant evaluation of a
function specialization, that needs such a declaration is not valid.
:::

## Undefined Behavior

:::{rule} INST-15
:kind: undefined
:tests: templates/instantiation/recursion-endless.cpp

If an instantiation needs an endless chain of other instantiations, the
behavior is undefined.
:::

## Implementation Requirements

:::{rule} INST-3
:kind: unspecified
:tests: templates/instantiation/class-overload-void-pointer.cpp

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
:tests: templates/instantiation/recursion-within-limit.cpp

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
:tests: templates/instantiation/function-point-end-of-unit.cpp templates/instantiation/function-point-better-later.cpp templates/instantiation/points-in-two-units.cpp

All the points of instantiation of a specialization, in one translation
unit or in several, must give the specialization the same meaning. An
implementation need not diagnose a break of this rule.
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 79
of 87 tests.

The clean-room writer probed each rule with the eight baseline
compilers and the two `-O2` report compilers, and wrote the grids
below. The rejections name the reason in each compiler's own words, for
example "explicit instantiation of undefined function template" and
"duplicate explicit instantiation" (clang-diag-sema).

- INST-5: EDG instantiates the initializer of an unused `inline static`
  data member with the class. Clang and GCC do not
  ({question}`Q-0231`).
- INST-5: GCC instantiates the enumerators of a scoped enumeration with
  the class. Clang and EDG do not ({question}`Q-0232`).
- INST-25: GCC accepts `static` and an attribute after `template` in an
  explicit instantiation. Clang and EDG reject both. Every compiler
  rejects an attribute before `template` ({question}`Q-0233`).
- INST-29: GCC accepts an explicit instantiation declaration of a
  function template with internal linkage. Clang and EDG reject it
  ({question}`Q-0234`).
- INST-21: EDG accepts an unqualified explicit instantiation outside
  the template's namespace, after a using-directive. Clang and GCC
  reject it ({question}`Q-0235`).
- INST-22: Clang rejects an explicit instantiation that leaves out an
  array bound. GCC and EDG accept it ({question}`Q-0236`).
- INST-19: EDG accepts an explicit specialization after an explicit
  instantiation declaration. Clang and GCC reject it
  ({question}`Q-0237`).
- INST-3: every compiler instantiates the class when a pointer to it
  converts to `void*` in a call. The rule allows either result.
- INST-11: every compiler instantiates an unused virtual function when
  the program defines an object of the class, and none does for
  `sizeof`. The rule allows either result.
- INST-14: Clang stops at a depth of 1024, GCC at 900, and EDG at 64
  nested class instantiations. INST-15 allows any result for an endless
  chain, and every compiler rejects the test.
- INST-28: the unoptimized builds fail to link in each test. The `-O2`
  builds inline the inline function and run, and fail to link for the
  others. The rule allows each result.
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

Inventory items left out of this page:

- G4 and H6 belong to constraints *(concepts referenced)*.
- G9 and G18, a destructor and the declaration of a variable in an
  explicit instantiation, have no probe.
- H11 and I5 are {rule}`EXS-9` and {rule}`EXS-10`. H19 is
  {rule}`IF-13`. H14 is {rule}`DED-28` and {rule}`DED-29`.
- H17, I3, I11, and I12 have no probe that shows a difference.
- I9 is about modules, which are outside the charter.

Inputs: clang-diag-sema.

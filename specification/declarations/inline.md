# The inline specifier

`inline` lets a program define a function or variable in each
translation unit that uses the entity. All these definitions are one
entity. This page covers:

- where `inline` can occur,
- which declarations are inline without the keyword,
- inline static data members,
- inline functions across translation units.

Inline namespaces are with namespaces. LNK-4 gives the rule for one
entity with one address.

Depends on: storage duration and linkage, function definitions, class
definitions and members.

## Principles

:::{principle} INL-P1
:rules: INL-1 INL-5 INL-6 INL-7

`inline` means that each {term}`translation unit` that uses an entity defines
the entity, and that all the definitions are one entity
({principle}`STO-P4`). These rules follow from the principle.

- A unit that uses the entity must define it, because no other unit's
  definition is promised. A unit that does not use the entity need not
  define it.
- Every unit must agree that the entity is `inline`, and the
  definitions must match, because a static local variable inside an
  inline function is still one object for the program.
- An entity with internal {term}`linkage` is never shared, so `inline` on it
  changes nothing across units.
:::

:::{principle} INL-P2
:rules: INL-2

As a declaration specifier, `inline` has a meaning only for a function
or a variable with static storage, so the specifier belongs nowhere
else. An `inline` namespace is a separate use of the keyword
({rule}`NS-3`). `inline` is a promise
about definitions, so the keyword must come no later than the
definition, and a unit defines an inline function once.
:::

:::{principle} INL-P3
:rules: INL-3 INL-4 INL-8

A definition inside a class body is meant to be seen in every unit
that sees the class, so it is inline without the keyword:

- member and friend functions defined there,
- `constexpr` members,
- the special members the implementation writes.

A static data member can join these definitions by being declared
`inline` in the class. The member then needs no definition outside the
class.
:::

## Syntax

`inline` is a declaration specifier. It occurs with other specifiers
before the {term}`declarator`.

## Constraints

:::{rule} INL-1
:kind: required
:tests: declarations/inline/function-and-variable.cpp declarations/inline/inline-before-definition.cpp declarations/inline/inline-internal-linkage.cpp declarations/inline/inline-unused-not-defined.cpp declarations/inline/inline-variable-const-default-external.cpp declarations/inline/inline-virtual.cpp declarations/inline/constexpr-variable-internal.cpp declarations/inline/inline-friend-declaration-not-definition.cpp foundations/odr/header-constant-three-forms.cpp

`inline` applies to functions and to variables with static {term}`storage duration`. These rules apply:

- A function can be declared `inline` before its definition.
- An inline function that no code uses needs no definition.
- An inline entity with internal linkage is a separate entity in each
  translation unit.
- An inline `const` variable at namespace scope has external linkage. A
  `constexpr` variable at namespace scope is not inline, so each
  translation unit has its own object.
- A friend declaration can be `inline` without being a definition.
- A virtual function can be `inline`.
:::

:::{rule} INL-2
:kind: reject
:tests: declarations/inline/inline-after-definition.cpp declarations/inline/inline-block-function-declaration.cpp declarations/inline/inline-block-variable.cpp declarations/inline/inline-class.cpp declarations/inline/inline-main.cpp declarations/inline/inline-nonstatic-member.cpp declarations/inline/inline-parameter.cpp declarations/inline/inline-redefinition-same-tu.cpp declarations/inline/inline-static-local-in-function.cpp declarations/inline/inline-typedef.cpp

`inline` must not occur on:

- a parameter, a block-scope variable, or a block-scope function
  declaration,
- a type alias, a class definition, or `main`,
- a non-static data member,
- a declaration of a function after its definition
  ({question}`Q-0068`).

An inline function must not be defined two times in one translation
unit.
:::

:::{rule} INL-3
:kind: required
:tests: declarations/inline/constexpr-function-inline.cpp declarations/inline/deleted-function-inline.cpp declarations/inline/inline-friend-definition.cpp declarations/inline/member-defined-in-class.cpp declarations/inline/static-constexpr-member-inline.cpp declarations/inline/implicit-special-members-inline.cpp

These declarations are inline without the keyword:

- a member function or friend function defined in its class,
- a `constexpr` or `consteval` function,
- a `static constexpr` data member,
- a deleted function,
- an implicitly declared {term}`special member function`.
:::

:::{rule} INL-4
:kind: required
:tests: declarations/inline/inline-static-member-out-of-class.cpp declarations/inline/inline-static-member.cpp declarations/inline/static-member-not-inline-needs-definition.cpp declarations/inline/inline-nonconst-static-member-out-of-class.cpp

An inline static data member is defined in its class, with its
initializer, and needs no definition outside the class. These rules
also apply:

- A static data member declared in its class can also be defined
  `inline` outside the class.
- A static data member that is not inline needs one definition outside
  its class if the member is used.
- INL-8 states what the type of an inline static data member must be.
:::

:::{rule} INL-8
:kind: reject
:tests: declarations/inline/inline-static-member-incomplete.cpp declarations/inline/inline-static-member-declared-later.cpp

The type of an inline static data member (INL-4) must be complete at its
definition in the class.
:::

## Runtime Behavior

:::{rule} INL-5
:kind: required
:tests: declarations/inline/static-local-shared.cpp declarations/inline/static-local-member-shared.cpp

A static local variable of an inline function with external linkage is
one object in the program. All translation units that call the function
share the object.
:::

## Implementation Requirements

:::{rule} INL-6
:kind: no-diagnostic
:tests: declarations/inline/inline-undefined-used.cpp declarations/inline/inline-variable-undefined-used.cpp

An inline function or variable that a translation unit uses must be
defined in that translation unit. An implementation need not diagnose a
missing definition before linking.
:::

:::{rule} INL-7
:kind: no-diagnostic
:tests: declarations/inline/inline-in-one-unit-only.cpp declarations/inline/inline-different-definitions.cpp

If a function or variable with external linkage is `inline` in one
translation unit, it must be `inline` in each translation unit that
declares it. LNK-6 requires the definitions to match. An implementation
need not diagnose a violation of either rule. {rule}`ODR-10` states the
general rule for matching definitions.
:::

## Examples

```cpp
// counter.h, included by several source files
inline int next_id() {
  static int last = 0;   // one object for the whole program
  return ++last;
}
struct Config {
  static inline int retries = 3;    // defined here, no .cpp needed
  int limit() const { return 10; }  // inline: defined in the class
};
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 35
of 37 tests.

- INL-2: GCC accepts `inline` on a declaration after the definition.
  Clang and EDG reject the declaration ({question}`Q-0068`).
- INL-1: all compilers accept an `inline` friend declaration that is
  not a definition.
- INL-7: the test with two different definitions accepts either body.
  The test with `inline` in one unit runs at `-O0`. At `-O2`, GCC and
  Clang emit no out-of-line definition, and the other unit's call does
  not link. INL-7 allows both results.
- INL-6: EDG reports a used inline function with no definition during
  translation. Clang and GCC report the missing definition when linking.
  All three report a used inline variable with no definition during
  translation.

Inputs: clang-sema-decl, clang-diag-sema, edg-decls.

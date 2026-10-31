# The inline specifier

`inline` lets a program define a function or variable in each translation
unit that uses it. All these definitions are one entity. This page covers
where `inline` can occur, which declarations are inline without the
keyword, inline static data members, and inline functions across
translation units.

Inline namespaces are with namespaces. LNK-4 gives the rule for one
entity with one address.

Depends on: storage duration and linkage, function definitions, class
definitions and members.

## Syntax

`inline` is a declaration specifier. It occurs with other specifiers
before the declarator.

## Constraints

:::{rule} INL-1
:kind: required
:tests: declarations/inline/function-and-variable.cpp declarations/inline/inline-before-definition.cpp declarations/inline/inline-internal-linkage.cpp declarations/inline/inline-unused-not-defined.cpp declarations/inline/inline-variable-const-default-external.cpp declarations/inline/inline-virtual.cpp declarations/inline/constexpr-variable-internal.cpp declarations/inline/inline-friend-declaration-not-definition.cpp

`inline` applies to functions and to variables with static storage
duration. These rules apply:

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

An inline function must not be defined two times in one translation unit.
:::

:::{rule} INL-3
:kind: required
:tests: declarations/inline/constexpr-function-inline.cpp declarations/inline/deleted-function-inline.cpp declarations/inline/inline-friend-definition.cpp declarations/inline/member-defined-in-class.cpp declarations/inline/static-constexpr-member-inline.cpp declarations/inline/implicit-special-members-inline.cpp

These declarations are inline without the keyword:

- a member function or friend function defined in its class,
- a `constexpr` or `consteval` function,
- a `static constexpr` data member,
- a deleted function,
- an implicitly declared special member function.
:::

:::{rule} INL-4
:kind: required
:tests: declarations/inline/inline-static-member-incomplete.cpp declarations/inline/inline-static-member-out-of-class.cpp declarations/inline/inline-static-member.cpp declarations/inline/static-member-not-inline-needs-definition.cpp declarations/inline/inline-nonconst-static-member-out-of-class.cpp

An inline static data member is defined in its class, with its
initializer. It needs no definition outside the class. Its type must be
complete there. A static data member declared in its class can also be
defined `inline` outside it. A static data member that is not inline
needs one definition outside its class if it is used.
:::

## Dynamic Semantics

:::{rule} INL-5
:kind: required
:tests: declarations/inline/static-local-shared.cpp

A static local variable of an inline function with external linkage is
one object in the program. All translation units that call the function
share it.
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
declares it. LNK-6 requires its definitions to match. An implementation
need not diagnose a violation of either rule.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 33 of
34 tests.

- INL-2: GCC accepts `inline` on a declaration after the definition.
  Clang and EDG reject it ({question}`Q-0068`).
- INL-1: all compilers accept an `inline` friend declaration that is
  not a definition.
- INL-7: the test with two different definitions accepts either body.
- INL-6: EDG reports a used inline function with no definition during
  translation. Clang and GCC report it when linking. All three report a
  used inline variable with no definition during translation.

Inputs: clang-sema-decl, clang-diag-sema, edg-decls.

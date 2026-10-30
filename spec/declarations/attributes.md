# Standard attributes

An attribute gives more information about a declaration, a statement, or a
type. Most standard attributes do not change what a valid program does.
They let an implementation warn or optimize (decision 0015).

This page covers the standard attributes `assume`, `carries_dependency`,
`deprecated`, `fallthrough`, `likely`, `maybe_unused`, `no_unique_address`,
`nodiscard`, `noreturn`, and `unlikely`.

## Syntax

```{eval-rst}
.. productionlist:: attributes
   attribute_spec: "[[" ["using" `attr_namespace` ":"]
                 : [`attribute` {"," `attribute`}] "]]"
   attribute: [`attr_namespace` "::"] `name` ["(" `tokens` ")"]
```

An attribute specifier comes before the declaration or statement that it
applies to. It can also come after the name of an entity in a
{term}`declarator`.

## Legality Rules

:::{rule} ATTR-1
:kind: required
:tests: declarations/attributes/syntax-lists.cpp declarations/attributes/unknown-attribute-ignored.cpp

An attribute specifier can contain several attributes, and several
specifiers can follow each other. `[[using ns: a, b]]` applies the
namespace `ns` to each attribute. An implementation ignores an attribute
that it does not recognize.
:::

:::{rule} ATTR-2
:kind: reject
:tests: declarations/attributes/attribute-argument-not-string.cpp declarations/attributes/attribute-too-many-arguments.cpp declarations/attributes/attribute-unexpected-argument.cpp declarations/attributes/noreturn-on-variable.cpp declarations/attributes/fallthrough-misplaced.cpp declarations/attributes/fallthrough-not-before-case.cpp declarations/attributes/fallthrough-on-declaration.cpp

A standard attribute must have the correct arguments and must apply to a
correct entity:

- `deprecated` and `nodiscard` have no argument or one string literal.
- `noreturn` has no argument and applies only to functions
  ({question}`Q-0021`).
- `fallthrough` is a statement on its own in a `switch`. It is
  immediately before a `case` or `default` label.
:::

:::{rule} ATTR-8
:kind: required
:tests: declarations/attributes/likely-unlikely.cpp

`[[likely]]` and `[[unlikely]]` can apply to a statement or a label. They
do not change what the program does.
:::

:::{rule} ATTR-11
:kind: required
:tests: declarations/attributes/carries-dependency.cpp

`[[carries_dependency]]` can apply to a function or a parameter. It is
about the memory model *(referenced)*. It does not change what a program
with one thread does.
:::

:::{rule} ATTR-12
:kind: required
:tests: declarations/attributes/noreturn-return-statement.cpp declarations/attributes/noreturn-main.cpp

The body of a `noreturn` function can contain a `return` statement.
`main` can be declared `noreturn`.
:::

:::{rule} ATTR-13
:kind: reject
:tests: declarations/attributes/assume-on-declaration.cpp

An assumption must be a statement on its own. It must not apply to a
declaration.
:::

## Dynamic Semantics

:::{rule} ATTR-15
:kind: required
:tests: declarations/attributes/assume-not-evaluated.cpp

An assumption does not evaluate its expression. Side effects in the
expression do not occur.
:::

## Undefined Behavior

:::{rule} ATTR-3
:kind: undefined
:tests: declarations/attributes/noreturn-ok.cpp

`[[noreturn]]` on a function states that a call to it never returns. If a
call returns, the behavior is undefined.
:::

:::{rule} ATTR-10
:kind: undefined
:tests: declarations/attributes/assume-false-constant.cpp declarations/attributes/assume-true-constant.cpp

`[[assume(expression)]];` states that `expression` is true at that point.
If the expression is false there, the behavior is undefined.
:::

## Implementation Requirements

:::{rule} ATTR-9
:kind: impl-defined
:tests: declarations/attributes/no-unique-address.cpp

`[[no_unique_address]]` can apply to a data member. If the type of the
member is empty, the implementation can give the member no storage of its
own. This choice is implementation-defined.
:::

## Recommended Practice

:::{rule} ATTR-4
:kind: recommended
:tests: declarations/attributes/deprecated-use-warns.cpp declarations/attributes/deprecated-declaration-only.cpp

`[[deprecated]]` marks an entity that programs should not use. An
implementation should warn at each use of the entity, and show the reason
if there is one. It should not warn at a declaration or definition.
:::

:::{rule} ATTR-5
:kind: recommended
:tests: declarations/attributes/nodiscard-ignored-warns.cpp declarations/attributes/nodiscard-void-cast.cpp declarations/attributes/nodiscard-type.cpp

An implementation should warn if a program discards the result of these
calls:

- a call to a function with `[[nodiscard]]`,
- a call that returns a type with `[[nodiscard]]`.

It should not warn if the program casts the result to `void`.
:::

:::{rule} ATTR-6
:kind: recommended
:tests: declarations/attributes/maybe-unused-silences.cpp declarations/attributes/unused-warns.cpp

`[[maybe_unused]]` can apply to a variable, parameter, function, or type.
An implementation should not warn that such an entity is unused.
:::

:::{rule} ATTR-7
:kind: recommended
:tests: declarations/attributes/fallthrough-silences.cpp declarations/attributes/implicit-fallthrough-warns.cpp

Control can go from one `case` of a `switch` into the next. If
`[[fallthrough]];` is immediately before the next label, an implementation
should not warn. If it is not there, an implementation can warn.
:::

:::{rule} ATTR-14
:kind: recommended
:tests: declarations/attributes/noreturn-returns.cpp

An implementation should warn if control can return from a `noreturn`
function.
:::

## Examples

```cpp
[[nodiscard]] int f();
f();                       // warning
(void)f();                 // no warning

struct Empty {};
struct Packed { [[no_unique_address]] Empty e; int v; };
// sizeof(Packed) == sizeof(int) on every compiler in the matrix

int i = 1;
[[assume(++i > 0)]];       // i is still 1

switch (k) {
  case 0:
    r += 1;
    [[fallthrough]];       // no warning
  case 1:
    r += 2;
}

[[noreturn]] int x;        // rejected: noreturn applies to functions
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 26 of
29 tests. EDG skips the tests that use `-Werror`.

- An implementation should warn if a `noreturn` function can return. Clang
  and GCC give this warning ("function declared 'noreturn' should not
  return").
- For ATTR-4 to ATTR-7, Clang and GCC give the warnings ("is deprecated",
  "ignoring return value", "unused variable", "may fall through"). The
  tests change them to errors with `-Werror`.
- For ATTR-10, all compilers reject a false assumption during
  {term}`constant evaluation` ("assumption evaluated to false", "failed
  'assume' attribute assumption", "assumption failed"). Clang 18 does not
  implement `assume` ({question}`Q-0020`).
- Divergence ATTR-D1 is about `[[noreturn]] int x;`. GCC accepts it with a
  warning. Clang and EDG reject it. The provisional answer is that it is
  rejected ({question}`Q-0021`).

Inputs: clang-sema-declattr, clang-sema-stmtattr, edg-attribute,
clang-diag-sema (see `inputs/MANIFEST.md`).

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

## Constraints

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
:tests: declarations/attributes/attribute-argument-not-string.cpp declarations/attributes/attribute-too-many-arguments.cpp declarations/attributes/attribute-unexpected-argument.cpp declarations/attributes/noreturn-on-variable.cpp declarations/attributes/fallthrough-misplaced.cpp declarations/attributes/fallthrough-not-before-case.cpp declarations/attributes/fallthrough-on-declaration.cpp declarations/attributes/likely-and-unlikely.cpp declarations/attributes/nodiscard-on-variable.cpp declarations/attributes/noreturn-not-first.cpp declarations/attributes/fallthrough-end-of-loop.cpp declarations/attributes/no-unique-address-bitfield.cpp declarations/attributes/carries-dependency-redeclared.cpp

A standard attribute must have the correct arguments and must apply to a
correct entity:

- `deprecated` and `nodiscard` have no argument or one string literal.
- `noreturn` has no argument and applies only to functions
  ({question}`Q-0021`).
- `fallthrough` is a statement on its own in a `switch`. It is
  immediately before a `case` or `default` label. A `fallthrough` at
  the end of a loop body inside the `switch` is not.

These uses are also not valid:

- `likely` and `unlikely` on the same statement or label ({question}`Q-0046`),
- `noreturn` on a declaration of a function that was first declared
  without it,
- `nodiscard` on a variable, and `no_unique_address` on a bit-field
  ({question}`Q-0021`),
- `carries_dependency` on a declaration of a function that was first
  declared without it ({question}`Q-0108`).
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
:tests: declarations/attributes/assume-on-declaration.cpp declarations/attributes/assume-not-bool.cpp declarations/attributes/assume-comma.cpp

An assumption must be a statement on its own. It must not apply to a
declaration.

The expression of an assumption must be convertible to `bool` by
{term}`contextual conversion`, although it is not evaluated. It must not have a comma operator at its
top level.
:::

:::{rule} ATTR-17
:kind: reject
:tests: declarations/attributes/double-bracket-lambda-subscript.cpp declarations/attributes/double-bracket-parenthesized-ok.cpp declarations/attributes/using-prefix-ok.cpp declarations/attributes/using-prefix-qualified.cpp

Two adjacent `[` tokens start an attribute specifier. A subscript whose
index starts with a lambda needs parentheses around the index. A `using`
prefix in an attribute specifier applies to every attribute in it, so an
attribute in that specifier must not have its own namespace.
:::

## Dynamic Semantics

:::{rule} ATTR-15
:kind: required
:tests: declarations/attributes/assume-not-evaluated.cpp

An assumption does not evaluate its expression. Side effects in the
expression do not occur.
:::

:::{rule} ATTR-16
:kind: required
:tests: declarations/attributes/no-unique-address-same-type.cpp

Two members of the same empty type have distinct addresses, even if both
are declared `no_unique_address`.
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
:tests: declarations/attributes/deprecated-use-warns.cpp declarations/attributes/deprecated-declaration-only.cpp declarations/attributes/deprecated-enumerator-warns.cpp declarations/attributes/deprecated-added-later.cpp

`[[deprecated]]` marks an entity that programs should not use. An
implementation should warn at each use of the entity, and show the reason
if there is one. It should not warn at a declaration or definition.

This also applies to an enumerator declared `deprecated`. A later
declaration can add `deprecated` to an entity declared without it. Uses
after that declaration should warn.
:::

:::{rule} ATTR-5
:kind: recommended
:tests: declarations/attributes/nodiscard-ignored-warns.cpp declarations/attributes/nodiscard-void-cast.cpp declarations/attributes/nodiscard-type.cpp declarations/attributes/nodiscard-constructor-warns.cpp declarations/attributes/nodiscard-reference-return.cpp

An implementation should warn if a program discards the result of these
calls:

- a call to a function with `[[nodiscard]]`,
- a call that returns a type with `[[nodiscard]]`.

It should not warn if the program casts the result to `void`, or if the
call returns a reference to such a type.

This also applies to a temporary that a `nodiscard` constructor creates
and the program discards.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 37 of 46 tests. EDG skips the tests that use `-Werror`.

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
- ATTR-2, `carries_dependency`: GCC accepts the attribute added on a
  redeclaration, because it ignores the attribute. Clang and EDG reject
  it ({question}`Q-0108`). GCC also ignores `no_unique_address` on a
  bit-field ({question}`Q-0021`). Clang 18 ignores `assume`, so it
  accepts a comma in it ({question}`Q-0020`).

Inputs: clang-sema-declattr, clang-sema-stmtattr, edg-attribute,
clang-diag-sema (see `inputs/MANIFEST.md`).

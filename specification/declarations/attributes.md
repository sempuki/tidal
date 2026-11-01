# Standard attributes

An attribute gives more information about a declaration, a statement, or
a type. Most standard attributes do not change what a valid program
does. They let an implementation warn or optimize (decision 0015).

This page covers these standard attributes:

- `assume`,
- `carries_dependency`,
- `deprecated`,
- `fallthrough`,
- `likely`,
- `maybe_unused`,
- `no_unique_address`,
- `nodiscard`,
- `noreturn`,
- `unlikely`.

## Principles

:::{principle} ATTR-P1
:rules: ATTR-1 ATTR-17 ATTR-2 ATTR-13 ATTR-18 ATTR-19

An attribute attaches information to a declaration, a statement, or a
type, in a form that an implementation can skip. The double brackets
make an attribute specifier, several attributes can share one, and an
attribute that the implementation does not know is ignored. Each
standard attribute has a fixed set of arguments and a fixed set of
places where it can go, so a standard attribute is not skippable in this
sense.
:::

:::{principle} ATTR-P2
:rules: ATTR-8 ATTR-11 ATTR-12 ATTR-15 ATTR-16 ATTR-9

A standard attribute does not change what a valid program does.

- A likelihood hint changes nothing.
- A dependency hint changes nothing in a single thread.
- An assumption is not evaluated, so its side effects never occur.
- Only `no_unique_address` can change a program's layout, and even
  `no_unique_address` keeps two members of one type at different
  addresses.
:::

:::{principle} ATTR-P3
:rules: ATTR-3 ATTR-10

`noreturn` and `assume` are promises that the program makes to the
implementation. `noreturn` promises that a call never returns, and
`assume` promises that an expression is true. The implementation can
rely on a promise, so a broken promise has no defined behavior.
:::

:::{principle} ATTR-P4
:rules: ATTR-4 ATTR-5 ATTR-6 ATTR-7 ATTR-14

The other attributes exist to turn warnings on or off.

| Attribute | Asks for |
|---|---|
| `deprecated`, `nodiscard` | A warning at a use |
| `maybe_unused`, `fallthrough` | Silence for a usual warning |

These attributes are advice to the implementation, and a program that
ignores the attributes stays valid.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   attribute_spec: "[[" ["using" `identifier` ":"]
                 :    [`attribute` {"," `attribute`}] "]]"
   attribute: [`identifier` "::"] `identifier` [`attribute_args`]
   attribute_args: "(" {`balanced_token`} ")"
   balanced_token: "(" {`balanced_token`} ")"
                 : | "[" {`balanced_token`} "]"
                 : | "{" {`balanced_token`} "}"
                 : | `token`
   token: any single token other than "(", ")", "[", "]", "{", or "}"
```

An attribute specifier comes before the declaration or statement that it
applies to. An attribute specifier can also come after the name of an
entity in a {term}`declarator`. ATTR-19 lists every position.

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
:tests: declarations/attributes/attribute-argument-not-string.cpp declarations/attributes/attribute-too-many-arguments.cpp declarations/attributes/attribute-unexpected-argument.cpp declarations/attributes/noreturn-on-variable.cpp declarations/attributes/fallthrough-misplaced.cpp declarations/attributes/fallthrough-not-before-case.cpp declarations/attributes/fallthrough-on-declaration.cpp declarations/attributes/likely-and-unlikely.cpp declarations/attributes/nodiscard-on-variable.cpp declarations/attributes/noreturn-not-first.cpp declarations/attributes/fallthrough-end-of-loop.cpp declarations/attributes/no-unique-address-bitfield.cpp declarations/attributes/carries-dependency-redeclared.cpp declarations/attributes/fallthrough-end-of-if-branch.cpp declarations/attributes/fallthrough-end-of-else-branch.cpp declarations/attributes/likely-with-argument.cpp declarations/attributes/unlikely-with-argument.cpp declarations/attributes/maybe-unused-with-argument.cpp declarations/attributes/fallthrough-with-argument.cpp declarations/attributes/no-unique-address-with-argument.cpp

A standard attribute must have the correct arguments and must apply to a
correct entity:

- `deprecated` and `nodiscard` have no argument or one string literal.
- `noreturn` has no argument and applies only to functions
  ({question}`Q-0021`).
- `fallthrough`, `likely`, `unlikely`, `maybe_unused`, and
  `no_unique_address` have no argument.
- `fallthrough` is a statement on its own in a `switch`. The next
  statement that can run after it must have a `case` or `default` label,
  so a `fallthrough` can end a branch of an `if` before such a label. A
  `fallthrough` at the end of a loop body inside the `switch` is not
  valid.

These uses are also not valid:

- `likely` and `unlikely` on the same statement or label
  ({question}`Q-0046`),
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

`[[likely]]` and `[[unlikely]]` can apply to a statement or a label. The
two attributes do not change what the program does.
:::

:::{rule} ATTR-11
:kind: required
:tests: declarations/attributes/carries-dependency.cpp

`[[carries_dependency]]` can apply to a function or a parameter. The
attribute is about the memory model *(memory model referenced)* and does
not change what a program with one thread does.
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

An assumption must be a statement on its own and must not apply to a
declaration.

The expression of an assumption must be convertible to `bool` by
{term}`contextual conversion`, although the expression is not evaluated.
The expression must not have a comma operator at its top level.
:::

:::{rule} ATTR-17
:kind: reject
:tests: declarations/attributes/double-bracket-lambda-subscript.cpp declarations/attributes/double-bracket-parenthesized-ok.cpp declarations/attributes/using-prefix-ok.cpp declarations/attributes/using-prefix-qualified.cpp

Two adjacent `[` tokens start an attribute specifier. A subscript whose
index starts with a lambda needs parentheses around the index. A `using`
prefix in an attribute specifier applies to every attribute in the
specifier, so each of those attributes must not have its own namespace.
:::

:::{rule} ATTR-18
:kind: reject
:tests: declarations/attributes/pointer-attribute-after-cv.cpp declarations/attributes/pointer-attribute-before-cv.cpp

In a pointer declarator, an attribute specifier comes after the
cv-qualifiers of the `*` ({question}`Q-0123`).
:::

:::{rule} ATTR-19
:kind: required
:tests: declarations/attributes/attribute-positions.cpp declarations/attributes/attribute-positions-base-and-new-bound.cpp

An attribute specifier can be written in each position in the table.
The specifier applies to the entity in the last column.

| Position | Example | Applies to |
|---|---|---|
| Before a declaration | `[[a]] int x;` | Each entity that the declaration declares |
| After the name in a declarator | `int x [[a]];` | The entity with that name |
| After the type specifiers of a declaration | `int [[a]] x;` | The type |
| After `*` and its cv-qualifiers, or after `&` | `int* [[a]] p;` | The pointer or reference type |
| After an array bound in a declarator | `int list[3] [[a]];` | The array type |
| After the parameter list of a function | `void f() [[a]];` | The function type |
| Before a parameter | `void f([[a]] int x);` | The parameter |
| After `struct`, `class`, `union`, or `enum` | `struct [[a]] S {};` | The class or enumeration |
| After the name of an enumerator | `enum E { red [[a]] };` | The enumerator |
| After `namespace` | `namespace [[a]] tools {}` | The namespace |
| After the name in an alias declaration | `using Count [[a]] = int;` | The alias |
| Before a base class | `struct D : [[a]] Base {};` | The base class |
| After the type in a `new` expression, or after its array bound | `new int [[a]]` | The type |
| Before a statement or a label | `[[a]] return 0;` | The statement or the label |
:::

## Runtime Behavior

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
:tests: declarations/attributes/noreturn-ok.cpp declarations/attributes/noreturn-returns-sanitizer.cpp

`[[noreturn]]` on a function states that a call to the function never
returns. If a call returns, the behavior is undefined.
:::

:::{rule} ATTR-10
:kind: undefined
:tests: declarations/attributes/assume-false-constant.cpp declarations/attributes/assume-true-constant.cpp declarations/attributes/assume-false-sanitizer.cpp

`[[assume(expression)]];` states that `expression` is true at that
point. If the expression is false there, the behavior is undefined.
:::

## Implementation Requirements

:::{rule} ATTR-9
:kind: impl-defined
:tests: declarations/attributes/no-unique-address.cpp

`[[no_unique_address]]` can apply to a data member. If the type of the
member is empty, the implementation can give the member no storage of
its own. This choice is implementation-defined.
:::

## Recommended Practice

:::{rule} ATTR-4
:kind: recommended
:tests: declarations/attributes/deprecated-use-warns.cpp declarations/attributes/deprecated-declaration-only.cpp declarations/attributes/deprecated-enumerator-warns.cpp declarations/attributes/deprecated-added-later.cpp

`[[deprecated]]` marks an entity that programs should not use. An
implementation should warn at each use of the entity, and show the
reason if there is one. The implementation should not warn at a
declaration or definition.

This rule also applies to an enumerator declared `deprecated`. A later
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

The implementation should not warn if the program casts the result to
`void`, or if the call returns a reference to such a type.

This rule also applies to a temporary that a `nodiscard` constructor
creates and the program discards.
:::

:::{rule} ATTR-6
:kind: recommended
:tests: declarations/attributes/maybe-unused-silences.cpp declarations/attributes/unused-warns.cpp declarations/attributes/unused-plain-class-warns.cpp declarations/attributes/unused-set-not-read-warns.cpp declarations/attributes/unused-structured-binding-warns.cpp declarations/attributes/unused-class-object-no-warning.cpp declarations/attributes/unused-member-ctor-no-warning.cpp declarations/attributes/unused-base-dtor-no-warning.cpp declarations/attributes/unused-nested-member-no-warning.cpp declarations/attributes/unused-member-plain-class-warns.cpp

An implementation should warn about a variable in a block that the
program never reads, including:

- a variable that the program only assigns,
- a structured binding declaration,
- a variable of a class type, if creating and destroying the variable
  runs no user-provided constructor or destructor.

Creating a variable runs the constructors of its class, its bases, and
its members, also of array elements. Destroying the variable runs the
destructors of the same classes. An implementation should not warn about
a variable whose creation or destruction runs a user-provided
constructor or destructor, or about a variable, parameter, function, or
type with `[[maybe_unused]]`.
:::

:::{rule} ATTR-7
:kind: recommended
:tests: declarations/attributes/fallthrough-silences.cpp declarations/attributes/implicit-fallthrough-warns.cpp declarations/attributes/fallthrough-consecutive-labels.cpp declarations/attributes/fallthrough-if-else-both-break.cpp declarations/attributes/fallthrough-if-without-else-warns.cpp declarations/attributes/fallthrough-after-noreturn.cpp declarations/attributes/fallthrough-after-jump.cpp

Control *falls through* when the statements after one label of a
`switch` can reach the next label. If `[[fallthrough]];` is immediately
before the next label, an implementation should not warn, and otherwise
an implementation can warn when control falls through. Control does not
fall through in these cases:

- The label is directly followed by the next label.
- The statements end with `break`, `continue`, `return`, `goto`, or
  `throw`.
- The statements end with a call of a `noreturn` function.
- The statements end with an `if` and an `else` whose branches each end
  in one of these ways.
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 59
of 72 tests. EDG skips the tests that use `-Werror`.

- An implementation should warn if a `noreturn` function can return.
  Clang and GCC give this warning ("function declared 'noreturn' should
  not return").
- For ATTR-4 to ATTR-7, Clang and GCC give the warnings ("is
  deprecated", "ignoring return value", "unused variable", "may fall
  through"). The tests change the warnings to errors with `-Werror`.
  {doc}`/foundations/translation` gives the flag for each warning.
- ATTR-6: Clang, GCC, and EDG give no warning for an unused variable of
  a class type with a user-provided constructor or destructor. The three
  also give no warning when a base, a member, or an element of an array
  member has a user-provided constructor or destructor. For a
  class whose only special feature is a default member initializer,
  Clang and EDG warn and GCC does not. Clang and GCC warn for an unused
  condition variable and an unused range-based `for` variable. EDG does
  not.
- ATTR-7: Clang warns for a missing `[[fallthrough]];` only under
  `-Wimplicit-fallthrough`. GCC also warns under `-Wextra`. Both agree
  on the cases where control falls through.
- For ATTR-10, all compilers reject a false assumption during
  {term}`constant evaluation` ("assumption evaluated to false", "failed
  'assume' attribute assumption", "assumption failed"). Clang 18 does
  not implement `assume` ({question}`Q-0020`).
- ATTR-19: every compiler accepts an attribute with an unknown namespace
  in each position of the table, and warns that the attribute is
  ignored. GCC 14 rejects the position before a base class and the
  position after the array bound of a `new` expression. GCC 16 accepts
  both ({question}`Q-0177`).
- Divergence ATTR-D1 is about `[[noreturn]] int x;`. GCC accepts it with
  a warning. Clang and EDG reject it. The provisional answer is that the
  declaration is rejected ({question}`Q-0021`).
- ATTR-2 has these divergences:

  - `carries_dependency`: GCC accepts the attribute added on a
    redeclaration, because GCC ignores the attribute. Clang and EDG
    reject the redeclaration ({question}`Q-0108`).
  - GCC also ignores `no_unique_address` on a bit-field
    ({question}`Q-0021`).
  - Clang 18 ignores `assume`, so Clang 18 accepts a comma in an
    assumption ({question}`Q-0020`).

Inputs: clang-sema-declattr, clang-sema-stmtattr, edg-attribute,
clang-diag-sema (see `inputs/MANIFEST.md`).

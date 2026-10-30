# Standard attributes

An attribute, written `[[name]]` or `[[name(arguments)]]`, attaches extra
information to a declaration, a statement, or a type. Tidal covers the
standard attributes: `noreturn`, `deprecated`, `fallthrough`,
`maybe_unused`, `nodiscard`, `likely`, `unlikely`, `no_unique_address`,
`assume`, and `carries_dependency`.

Most standard attributes don't change what a valid program does. They let
an implementation warn, or optimize, and the rules for them describe
*recommended practice*: behavior an implementation should provide but
need not (decision 0015). Rules are prefixed `ATTR`.

Depends on: declarations *(referenced)*, statements *(referenced)*,
classes *(referenced)*, [constant evaluation](../glossary.md#constant-evaluation).

Inputs: clang-sema-declattr, clang-sema-stmtattr, edg-attribute,
clang-diag-sema (see `inputs/MANIFEST.md`).

## Syntax

    attribute-spec = "[[" [ "using" namespace ":" ] [ attribute { "," attribute } ] "]]"
    attribute      = [ namespace "::" ] name [ "(" tokens ")" ]

An attribute specifier comes before the declaration or statement it
applies to, or after the name of the entity in a declarator.

## Rules

### ATTR-1 (required)

An attribute specifier may hold several attributes separated by commas, and
several specifiers may be written in a row. `[[using ns: a, b]]` applies the
namespace `ns` to each attribute in the list. An attribute the
implementation doesn't recognize is ignored and doesn't make the program
invalid.

Tests: declarations/attributes/syntax-lists.cpp, declarations/attributes/unknown-attribute-ignored.cpp

### ATTR-2 (reject)

A program is rejected if a standard attribute has arguments of the wrong
kind or number, or is applied where it doesn't belong:

- `deprecated` and `nodiscard` take at most one string literal;
- `noreturn` takes no arguments and applies only to functions (question
  Q-0021);
- `fallthrough` appears only as a statement on its own, inside a
  `switch`, directly before a `case` or `default` label.

Tests: declarations/attributes/attribute-argument-not-string.cpp, declarations/attributes/attribute-too-many-arguments.cpp, declarations/attributes/attribute-unexpected-argument.cpp, declarations/attributes/noreturn-on-variable.cpp, declarations/attributes/fallthrough-misplaced.cpp, declarations/attributes/fallthrough-not-before-case.cpp, declarations/attributes/fallthrough-on-declaration.cpp

### ATTR-3 (undefined)

`[[noreturn]]` on a function declares that calls to it never return. If a
call does return, the behavior is undefined. A function body that can
return, including one with a `return` statement, is still accepted, and
`main` may be declared `noreturn`. *Recommended:* an implementation
warns when a `noreturn` function can return.

Tests: declarations/attributes/noreturn-ok.cpp, declarations/attributes/noreturn-returns.cpp, declarations/attributes/noreturn-return-statement.cpp, declarations/attributes/noreturn-main.cpp

### ATTR-4 (recommended)

`[[deprecated]]` or `[[deprecated("reason")]]` on a declaration marks
the entity as discouraged. An implementation should warn wherever the entity
is used, including the reason if one is given. Declaring or defining it
gives no warning.

Tests: declarations/attributes/deprecated-use-warns.cpp, declarations/attributes/deprecated-declaration-only.cpp

### ATTR-5 (recommended)

When a function declared `[[nodiscard]]` is called, or a function returns
a type declared `[[nodiscard]]`, an implementation should warn if the
result is discarded. A cast to `void` discards the result explicitly and
should not warn.

    [[nodiscard]] int f();
    f();          // warning
    (void)f();    // no warning

Tests: declarations/attributes/nodiscard-ignored-warns.cpp, declarations/attributes/nodiscard-void-cast.cpp, declarations/attributes/nodiscard-type.cpp

### ATTR-6 (recommended)

`[[maybe_unused]]` on a variable, parameter, function, or type tells the
implementation not to warn that the entity is unused.

Tests: declarations/attributes/maybe-unused-silences.cpp, declarations/attributes/unused-warns.cpp

### ATTR-7 (recommended)

Control falling from one `case` of a `switch` into the next may be
deliberate. `[[fallthrough]];` directly before the next label says it is.
An implementation should not warn about a fall-through marked this way, and
may warn about one that isn't.

Tests: declarations/attributes/fallthrough-silences.cpp, declarations/attributes/implicit-fallthrough-warns.cpp

### ATTR-8 (required)

`[[likely]]` and `[[unlikely]]` on a statement or label hint how often
that path runs. They don't change what the program does.

Tests: declarations/attributes/likely-unlikely.cpp

### ATTR-9 (impl-defined)

`[[no_unique_address]]` on a data member allows the implementation to give
the member no storage of its own when its type is empty, so it may share an
address with another member. Whether it does is up to the implementation.

    struct Empty {};
    struct Packed { [[no_unique_address]] Empty e; int v; };
    // sizeof(Packed) == sizeof(int) on every compiler in the matrix

Tests: declarations/attributes/no-unique-address.cpp

### ATTR-10 (undefined)

`[[assume(expression)]];` is a statement that states `expression` is
true. The expression is never evaluated, so its side effects never happen.
If it would be false at that point, the behavior is undefined. An
assumption may only be written as a statement on its own, never on a
declaration.

    int i = 1;
    [[assume(++i > 0)]];   // i is still 1

Tests: declarations/attributes/assume-not-evaluated.cpp, declarations/attributes/assume-false-constant.cpp, declarations/attributes/assume-true-constant.cpp, declarations/attributes/assume-on-declaration.cpp

### ATTR-11 (required)

`[[carries_dependency]]` on a function or parameter is accepted. It
relates to the memory model *(referenced)* and doesn't change what a
single-threaded program does.

Tests: declarations/attributes/carries-dependency.cpp

## Evidence

Results are in `tests/RESULTS.md`. All seven compilers agree on 26 of 29
tests. EDG skips the tests that use `-Werror`.

- **ATTR-3, ATTR-4 to ATTR-7.** Clang and GCC give the recommended
  warnings ("function declared 'noreturn' should not return", "is
  deprecated", "ignoring return value", "unused variable", "may fall
  through"). The tests turn them into errors with `-Werror`.
- **ATTR-10.** All compilers reject a false assumption during constant
  evaluation ("assumption evaluated to false", "failed 'assume' attribute
  assumption", "assumption failed"). Clang 18 doesn't implement
  `assume` (question Q-0020).

## Divergences

### ATTR-D1: noreturn on a variable

GCC accepts `[[noreturn]] int x;` with a warning. Clang and EDG reject it.
**Provisional answer:** rejected. See Q-0021.

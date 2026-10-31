# Linkage specifications

A linkage specification gives declarations a *language linkage*, such as
`extern "C"`. A function with C language linkage can be called from code
in C, and its name is not mangled. This page covers the forms of a linkage
specification, where it can occur, and the rules for names with C
language linkage.

Depends on: storage duration and linkage, namespaces, function
definitions.

## Principles

:::{principle} LSP-P1
:rules: LSP-1 LSP-2 LSP-3

A linkage specification labels declarations with the language whose
calling and naming conventions they use. It is a label and not a scope,
so its braces hide nothing, and a declaration inside it means what it
would mean outside. It labels functions and variables at namespace
scope, and nothing else. The form without braces behaves as `extern`
does.
:::

:::{principle} LSP-P2
:rules: LSP-4

C has no namespaces and no overloading, so a C name is its spelling
alone. All declarations of one spelling with C language linkage are one
entity, wherever they sit, and two C functions cannot share a name. A
C name also cannot share its spelling with a global variable, because
the linker would see one symbol.
:::

:::{principle} LSP-P3
:rules: LSP-5 LSP-6

Two choices stay with the implementation: the set of languages beyond
C and C++ that it can link with, and whether a function's type records
the language it is linked as. A program that must work everywhere treats
a pointer to a C function as a type of its own.
:::

## Syntax

```{eval-rst}
.. productionlist:: lsp
   linkage_spec: "extern" `string_literal` "{" {`declaration`} "}"
               : "extern" `string_literal` `declaration`
```

The string names the language, `"C"` or `"C++"`. Declarations without a
linkage specification have C++ language linkage. In this page, a *C
name* is a function or variable that has C language linkage.

## Constraints

:::{rule} LSP-1
:kind: required
:tests: declarations/linkage-specifications/block-form.cpp declarations/linkage-specifications/cpp-linkage.cpp declarations/linkage-specifications/linkage-in-namespace.cpp declarations/linkage-specifications/members-cpp-linkage.cpp declarations/linkage-specifications/nested-innermost.cpp declarations/linkage-specifications/single-form.cpp declarations/linkage-specifications/static-in-c-block.cpp declarations/linkage-specifications/block-form-no-scope.cpp

A linkage specification gives its language linkage to the functions and
variables that it declares. In nested linkage specifications, the
innermost applies. These rules apply:

- A linkage specification can occur in a namespace. Its braces do not
  make a scope, so names declared in them are visible after them.
- A `static` function in an `extern "C"` block has internal linkage.
- Member functions of a class have C++ language linkage, also in an
  `extern "C"` block, so they can be overloaded.
:::

:::{rule} LSP-2
:kind: reject
:tests: declarations/linkage-specifications/block-scope.cpp declarations/linkage-specifications/c-linkage-main.cpp declarations/linkage-specifications/c-linkage-template.cpp declarations/linkage-specifications/class-scope.cpp declarations/linkage-specifications/single-form-static.cpp

A linkage specification can only occur at namespace scope. It must not
occur in a block or a class. A template must not have C language linkage.
The form without braces must not declare a `static` entity.
`main` must not have C language linkage ({question}`Q-0076`).
:::

:::{rule} LSP-3
:kind: required
:tests: declarations/linkage-specifications/block-form-defines.cpp declarations/linkage-specifications/single-form-declaration-only.cpp declarations/linkage-specifications/single-form-with-init-defines.cpp

The form `extern "C" int x;` declares a variable and does not define it,
as `extern` does. With an initializer, it defines the variable. A
declaration in braces, such as `extern "C" { int y; }`, is a definition
if it would be one without the braces.
:::

:::{rule} LSP-4
:kind: required
:tests: declarations/linkage-specifications/c-and-cpp-overload.cpp declarations/linkage-specifications/c-cross-unit.cpp declarations/linkage-specifications/c-name-across-namespaces.cpp declarations/linkage-specifications/c-name-conflict-types.cpp declarations/linkage-specifications/c-name-conflicts-global.cpp declarations/linkage-specifications/c-overload-rejected.cpp declarations/linkage-specifications/c-variable-and-function-conflict.cpp declarations/linkage-specifications/c-variable-named-main.cpp declarations/linkage-specifications/redeclare-c-then-plain.cpp declarations/linkage-specifications/redeclare-linkage-mismatch.cpp

A C name is known by its spelling alone. These rules follow:

- C names with the same spelling in different namespaces are one entity.
  They must agree in type and kind.
- Two C functions with one name are not overloads, so the second is not
  valid. A C function and a C++ function can overload.
- A C name must not have the spelling of a global variable with C++
  linkage ({question}`Q-0075`), or the spelling `main`
  ({question}`Q-0076`).
- A later declaration without a linkage specification keeps the C
  language linkage. A C++ declaration must not be followed by a C
  declaration of the same function.
:::

## Implementation Requirements

:::{rule} LSP-5
:kind: impl-defined
:tests: declarations/linkage-specifications/unknown-language.cpp

The languages other than `"C"` and `"C++"` that an implementation
supports are implementation-defined. The implementations in the matrix
support no others, and they reject an unknown language.
:::

:::{rule} LSP-6
:kind: impl-defined
:tests: declarations/linkage-specifications/c-function-pointer-assign.cpp declarations/linkage-specifications/function-type-linkage-overload.cpp

Each implementation chooses if a function type records its language
linkage ({question}`Q-0074`). If it does, a pointer to a C function and
a pointer to a C++ function have different types. The first does not
convert to the second.
:::

## Examples

```cpp
extern "C" {
  int c_add(int, int);           // callable from C
  struct Point { int x, y; };    // types have no language linkage
}
extern "C" int c_errno;          // declaration only

namespace io {
  extern "C" int c_add(int, int);  // the same function as ::c_add
}
extern "C" int c_add(double, double);  // rejected: C functions cannot overload
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 25 of
29 tests.

- LSP-6: EDG treats C language linkage as part of the function type.
  Clang and GCC do not ({question}`Q-0074`).
- LSP-4: GCC accepts a C language linkage name that matches a global
  variable ({question}`Q-0075`).
- LSP-2, LSP-4: GCC 14 and Clang 18 accept C language linkage on `main`
  and on a variable named `main` ({question}`Q-0076`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls.

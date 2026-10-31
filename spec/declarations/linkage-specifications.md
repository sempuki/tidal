# Linkage specifications

A linkage specification gives declarations a *language linkage*, such as
`extern "C"`. A function with C language linkage can be called from code
in C, and its name is not mangled. This page covers the forms of a linkage
specification, where it can occur, and the rules for names with C
language linkage.

Depends on: storage duration and linkage, namespaces, function
definitions.

## Syntax

```{eval-rst}
.. productionlist:: lsp
   linkage_spec: "extern" `string_literal` "{" {`declaration`} "}"
               : "extern" `string_literal` `declaration`
```

The string names the language, `"C"` or `"C++"`. Declarations without a
linkage specification have C++ language linkage.

## Constraints

:::{rule} LSP-1
:kind: required
:tests: declarations/linkage-specifications/block-form.cpp declarations/linkage-specifications/cpp-linkage.cpp declarations/linkage-specifications/linkage-in-namespace.cpp declarations/linkage-specifications/members-cpp-linkage.cpp declarations/linkage-specifications/nested-innermost.cpp declarations/linkage-specifications/single-form.cpp declarations/linkage-specifications/static-in-c-block.cpp

A linkage specification gives its language linkage to the functions and
variables that it declares. In nested linkage specifications, the
innermost applies. These rules apply:

- A linkage specification can occur in a namespace.
- A `static` function in an `extern "C"` block has internal linkage.
- Member functions of a class have C++ language linkage, also in an
  `extern "C"` block, so they can be overloaded.
:::

:::{rule} LSP-2
:kind: reject
:tests: declarations/linkage-specifications/block-scope.cpp declarations/linkage-specifications/c-linkage-main.cpp declarations/linkage-specifications/c-linkage-template.cpp declarations/linkage-specifications/class-scope.cpp

A linkage specification can only occur at namespace scope. It must not
occur in a block or a class. A template must not have C language linkage.
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

A function or variable with C language linkage is known by its name
alone. These rules follow:

- Declarations with C language linkage and the same name in different
  namespaces are one entity. They must agree in type and kind.
- Two functions with C language linkage cannot overload each other. A
  function with C language linkage can overload one with C++ linkage.
- A name with C language linkage must not also be the name of a global
  variable with C++ linkage ({question}`Q-0075`), or `main`
  ({question}`Q-0076`).
- A later declaration without a linkage specification keeps the C
  language linkage. A declaration with C++ linkage must not be followed
  by one with C linkage.
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

It is implementation-defined if language linkage is part of a function
type ({question}`Q-0074`). If it is, a pointer to a function with C
language linkage does not convert to a pointer to a function type with
C++ linkage.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 23 of
27 tests.

- LSP-6: EDG treats C language linkage as part of the function type.
  Clang and GCC do not ({question}`Q-0074`).
- LSP-4: GCC accepts a C language linkage name that matches a global
  variable ({question}`Q-0075`).
- LSP-2, LSP-4: GCC 14 and Clang 18 accept C language linkage on `main`
  and on a variable named `main` ({question}`Q-0076`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls.

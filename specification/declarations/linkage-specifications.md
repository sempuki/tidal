# Linkage specifications

A linkage specification gives declarations a *language linkage*, such as
`extern "C"`. A function with C language linkage can be called from code
in C, and its name is not mangled. This page covers:

- the forms of a linkage specification,
- where it can occur,
- the rules for names with C language linkage.

Depends on: storage duration and linkage, namespaces, function
definitions.

## Principles

:::{principle} LSP-P1
:rules: LSP-1 LSP-2 LSP-3

A linkage specification labels declarations with the language whose
calling and naming conventions they use. The specification does not
make a scope, so its braces hide nothing, and a declaration inside the
braces means what the declaration would mean outside. A linkage
specification labels the functions and variables that it declares at
namespace scope. It also labels each function type in its declarations,
such as the type of a callback parameter or a type alias. Classes and
other types get no label. The form without braces behaves as `extern`
does.
:::

:::{principle} LSP-P2
:rules: LSP-4 LSP-7 LSP-8

C has no namespaces and no overloading, so a C name is its spelling
alone. All declarations of one spelling with C language linkage are one
entity, wherever they sit, and two C functions cannot share a name. A
C name also cannot share its spelling with a global variable, because
the linker would see one symbol. A translation unit sees only its own
declarations, so a clash between two units needs no diagnostic.
:::

:::{principle} LSP-P3
:rules: LSP-5 LSP-6

The implementation chooses the set of languages beyond C and C++ that it
can link with. The implementation also chooses whether a function's type
records the language that the function is linked as. A program that must
work everywhere treats a pointer to a C function as a type of its own.
Such a program names the type with an alias in an `extern "C"` block,
and passes only C functions where the type is expected.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   linkage_spec: "extern" `string_literal` "{" {`declaration`} "}"
               : | "extern" `string_literal` `declaration`
```

The string names the language, `"C"` or `"C++"`. Declarations without a
linkage specification have C++ language linkage. In this page, a *C
name* is a function or variable that has C language linkage.

## Constraints

:::{rule} LSP-1
:kind: required
:tests: declarations/linkage-specifications/block-form.cpp declarations/linkage-specifications/cpp-linkage.cpp declarations/linkage-specifications/linkage-in-namespace.cpp declarations/linkage-specifications/members-cpp-linkage.cpp declarations/linkage-specifications/nested-innermost.cpp declarations/linkage-specifications/single-form.cpp declarations/linkage-specifications/static-in-c-block.cpp declarations/linkage-specifications/block-form-no-scope.cpp declarations/linkage-specifications/c-callback-alias.cpp declarations/linkage-specifications/c-callback-c-function.cpp declarations/linkage-specifications/c-parameter-type-linkage.cpp

A linkage specification gives its language linkage to the functions and
variables that it declares. It also gives the linkage to each function
type in its declarations, such as a parameter type or a type alias
(LSP-6). In nested linkage specifications, the innermost applies. These
rules apply:

- `extern "C++"` gives C++ language linkage, which a declaration
  without a linkage specification has too. So outside an `extern "C"`
  block, `extern "C++"` changes nothing.
- A linkage specification can occur in a namespace. The braces of a
  linkage specification do not make a scope, so names declared in the
  braces are visible after the closing brace.
- A `static` function in an `extern "C"` block has internal linkage.
- Member functions of a class have C++ language linkage, also in an
  `extern "C"` block, so they can be overloaded.
:::

:::{rule} LSP-2
:kind: reject
:tests: declarations/linkage-specifications/block-scope.cpp declarations/linkage-specifications/c-linkage-main.cpp declarations/linkage-specifications/c-linkage-template.cpp declarations/linkage-specifications/class-scope.cpp declarations/linkage-specifications/single-form-static.cpp

A linkage specification can only occur at namespace scope. It must not
occur in a block or a class. These rules also apply:

- A template must not have C language linkage.
- The form without braces must not declare a `static` entity.
- `main` must not have C language linkage ({question}`Q-0076`).
:::

:::{rule} LSP-3
:kind: required
:tests: declarations/linkage-specifications/block-form-defines.cpp declarations/linkage-specifications/single-form-declaration-only.cpp declarations/linkage-specifications/single-form-with-init-defines.cpp

The form `extern "C" int x;` declares a variable and does not define it,
as `extern` does. With an initializer, the form defines the variable. A
declaration in braces, such as `extern "C" { int y; }`, is a definition
if the declaration would be a definition without the braces.
:::

:::{rule} LSP-4
:kind: required
:tests: declarations/linkage-specifications/c-and-cpp-overload.cpp declarations/linkage-specifications/c-cross-unit.cpp declarations/linkage-specifications/c-name-across-namespaces.cpp declarations/linkage-specifications/redeclare-c-then-plain.cpp declarations/linkage-specifications/c-static-function-two-namespaces.cpp declarations/linkage-specifications/c-static-variable-two-namespaces.cpp declarations/linkage-specifications/c-static-not-joined.cpp declarations/linkage-specifications/c-name-unnamed-namespace.cpp declarations/linkage-specifications/c-name-unnamed-namespace-other-unit.cpp declarations/linkage-specifications/c-name-unnamed-namespace-redefined.cpp

A C name is known by its spelling alone. These rules follow:

- C names with the same spelling in different namespaces are one entity,
  and one candidate in overload resolution ({rule}`OVL-1`).
- A C name in an unnamed namespace is one entity with the other C
  names of its spelling, also in other translation units. {rule}`LNK-5` does
  not apply to the C name.
- A C name that `static` gives internal linkage is an entity of its own
  namespace only. C names with one spelling and internal linkage in two
  namespaces are two entities
  ({question}`Q-0369`).
- A C function and a C++ function can overload.
- A later declaration without a linkage specification keeps the C
  language linkage.

LSP-7 lists what is not valid.
:::

:::{rule} LSP-7
:kind: reject
:tests: declarations/linkage-specifications/c-name-conflict-types.cpp declarations/linkage-specifications/c-name-conflicts-global.cpp declarations/linkage-specifications/c-name-before-global.cpp declarations/linkage-specifications/c-function-conflicts-global-variable.cpp declarations/linkage-specifications/c-name-global-function-allowed.cpp declarations/linkage-specifications/c-overload-rejected.cpp declarations/linkage-specifications/c-variable-and-function-conflict.cpp declarations/linkage-specifications/c-variable-named-main.cpp declarations/linkage-specifications/redeclare-linkage-mismatch.cpp

These rules also follow from LSP-4:

- C names with the same spelling in different namespaces must agree in
  type and kind.
- Two C functions with one name are not overloads, so the second is not
  valid.
- A C name of a variable or a function must not have the spelling of a
  global variable with C++ linkage, in either order of the declarations
  ({question}`Q-0075`). A global function with C++ linkage can have the
  spelling of a C name.
- A C name must not have the spelling `main` ({question}`Q-0076`).
- A C++ declaration must not be followed by a C declaration of the same
  function.
:::

## Implementation Requirements

:::{rule} LSP-5
:kind: impl-defined
:tests: declarations/linkage-specifications/unknown-language.cpp declarations/linkage-specifications/unknown-language-block.cpp

The languages other than `"C"` and `"C++"` that an implementation
supports are implementation-defined. The implementations in the matrix
support no others, and they reject an unknown language.
:::

:::{rule} LSP-6
:kind: impl-defined
:tests: declarations/linkage-specifications/c-function-pointer-assign.cpp declarations/linkage-specifications/function-type-linkage-overload.cpp declarations/linkage-specifications/c-callback-alias.cpp declarations/linkage-specifications/c-parameter-type-linkage.cpp declarations/linkage-specifications/c-function-pointer-assign-ilp32.cpp

Each implementation chooses if a function type records its language
linkage ({question}`Q-0074`). If the implementation does, a pointer to a
C function and a pointer to a C++ function have different types, and
neither converts to the other. If not, the two types are the same, so
two declarations that differ only in that linkage declare one function.
EDG records the linkage, and Clang and GCC do not, on x86-64 and with
`-m32`.
:::

:::{rule} LSP-8
:kind: no-diagnostic
:tests: declarations/linkage-specifications/c-variable-and-global-variable-two-units.cpp declarations/linkage-specifications/global-variable-and-c-variable-two-units.cpp

LSP-7 asks for a diagnostic when one declaration of the clash comes
before the other in one translation unit. When the two declarations are
in different translation units, the program is not valid, and an
implementation need not diagnose it. An example is a C variable in one
unit and a global variable with C++ language linkage and the same
spelling in another.
:::

## Examples

% tidal-example: reject
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

A C function that takes a callback expects a C function. An alias in
the `extern "C"` block names that type:

% tidal-example: reject
```cpp
extern "C" {
  using callback = int(int);     // a function type with C language linkage
  int apply(callback* f, int v); // defined in C
  int triple(int v) { return 3 * v; }
}
int twice(int v) { return 2 * v; }

apply(triple, 2);                // 6 on every implementation
apply(twice, 2);                 // rejected where the type records the linkage (LSP-6)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 32
of 45 tests.

One question on this page settled on 2026-11-03 under decision 0054 after the source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`). For each, the review supports the answer with no doubt, and a principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0075` | LSP-P2 | GCC 16 |

- LSP-6: EDG treats C language linkage as part of the function type.
  Clang and GCC do not ({question}`Q-0074`). EDG gives the linkage to a
  function type in a parameter and in a type alias inside an
  `extern "C"` block, and rejects a C++ function as the argument for
  either. EDG also rejects a C function as the initializer of a pointer
  to a C++ function type, such as `void (*)()`. Clang and GCC accept
  each of these programs.
- LSP-6: two functions whose parameter types differ only in the
  linkage of a function type are two overloads on EDG. On Clang and
  GCC the second declaration declares the first function again. Every
  compiler accepts the two declarations.
- LSP-4, LSP-7: GCC accepts a C language linkage name that matches a
  global variable ({question}`Q-0075`). Clang and EDG reject the program
  in either order, for a C variable and for a C function. Every
  compiler accepts a global function with C++ linkage that has the
  spelling of a C name.
- LSP-2, LSP-4: GCC 14 and Clang 18 accept C language linkage on `main`
  and on a variable named `main` ({question}`Q-0076`).
- LSP-4, LNK-5: Clang and GCC treat a C name in an unnamed namespace as
  one entity with the other C names of its spelling. Clang rejects a
  second definition in another namespace, and GCC fails at assembly
  with one symbol for the two. EDG accepts the two definitions
  ({question}`Q-0369`). The gap review of the rules that phase 7
  rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`)
  reads the reference as EDG does: the name has internal linkage and so
  no language linkage. Tidal keeps the answer of Clang and GCC
  (`documentation/decisions/pending.md`, row 4).
- LSP-8: Clang and GCC link a C variable in one unit with a global
  variable of the same spelling in another, in either order, and give
  no diagnostic. The gap review of the rules that phase 7 rewrote reads
  the reference as requiring no diagnostic when neither declaration is
  reachable from the other.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls.

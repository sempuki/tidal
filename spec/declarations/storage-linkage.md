# Storage duration and linkage

Every variable has a *storage duration*, which decides when its storage
exists, and most names have a *linkage*, which decides whether the same name
in different scopes or translation units refers to the same entity. The
storage class specifiers `static`, `extern`, and `thread_local` set both,
together with where the declaration appears.

This section covers variables and functions declared at namespace scope and
block scope. Class members (`static` members, `mutable`), dynamic storage
(`new` and `delete`), threads, modules, and language linkage (`extern "C"`)
are *(referenced)*.

Rules prefixed `STO` cover storage duration and the specifiers. Rules
prefixed `LNK` cover linkage.

Depends on: declarations *(referenced)*, namespaces *(referenced)*,
initialization *(referenced)*, object lifetime *(referenced)*, the one
definition rule *(referenced)*.

Inputs: clang-sema-decl, gcc-cp-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

## Syntax

    storage-class = "static" | "extern" | "thread_local" | "mutable"

At most one storage class may appear in a declaration, except that
`thread_local` may be combined with `static` or `extern`. `mutable`
belongs to class members *(referenced)*.

## Rules

### STO-1 (required)

A variable declared in a block without `static`, `thread_local`, or
`extern` has *automatic* storage. Each time control reaches its
declaration, a new object is created. It is destroyed when control leaves
the block. Within a block, the last object created is the first one
destroyed. Each active call of a function, including a recursive
call, has its own objects.

Tests: declarations/storage-linkage/automatic-lifetime.cpp

### STO-2 (required)

A variable declared `static` in a block has *static* storage, so there is
one object for the whole program run. It is initialized the first time control
reaches its declaration, and never if control never reaches it. Later
passes through the declaration skip the initializer, and the variable keeps
its value between calls.

    int next() {
      static int counter = 0;   // initialized on the first call only
      return ++counter;         // 1, 2, 3, ...
    }

Tests: declarations/storage-linkage/block-static-once.cpp, declarations/storage-linkage/block-static-not-reached.cpp

### STO-3 (required)

Variables declared at namespace scope also have static storage. Before any
other initialization, the storage of every variable with static storage is
set to zero, so a variable with no initializer starts as zero, a null
pointer, or an array of zeros.

Tests: declarations/storage-linkage/zero-initialized.cpp

### STO-4 (required)

A variable declared `thread_local` has *thread* storage, so each thread
has its own object *(threads referenced)*. `thread_local` may be combined with
`static` or `extern`, and in a block it implies `static`, so within one
thread it behaves as STO-2 describes. Every declaration of a variable must
agree on whether it is `thread_local`. A program where they disagree is
rejected.

Tests: declarations/storage-linkage/thread-local-block.cpp, declarations/storage-linkage/thread-local-combinations.cpp, declarations/storage-linkage/thread-local-after-plain.cpp, declarations/storage-linkage/plain-after-thread-local.cpp

### STO-5 (reject)

A program that uses `register` as a storage class is rejected.

Tests: declarations/storage-linkage/register.cpp

### STO-6 (reject)

A program is rejected if a declaration

- has two storage classes, other than `thread_local` with `static` or
  `extern`,
- combines a storage class with `typedef`,
- gives a storage class to a function parameter,
- declares a function `thread_local`,
- declares a namespace-scope variable `mutable`,
- declares a function `static` inside a block,
- declares `main` `static`, or
- gives a storage class to a declaration that only defines a class, as in
  `static struct S { int v; };`.

Tests: declarations/storage-linkage/two-storage-classes.cpp, declarations/storage-linkage/storage-class-on-typedef.cpp, declarations/storage-linkage/static-parameter.cpp, declarations/storage-linkage/extern-parameter.cpp, declarations/storage-linkage/thread-local-function.cpp, declarations/storage-linkage/mutable-namespace.cpp, declarations/storage-linkage/static-block-function.cpp, declarations/storage-linkage/static-main.cpp, declarations/storage-linkage/storage-class-on-class.cpp

### STO-7 (required)

`extern` on a variable declared in a block makes the name refer to a
variable with linkage outside the block, even when a local variable of the
same name is visible in an enclosing block. Such a declaration can't have an
initializer, and it can't follow a local variable of the same name in the
same block. At namespace scope, `extern` with an initializer is a
definition.

    int shared = 5;
    int read() {
      int shared = 1;
      { extern int shared; return shared; }   // 5
    }

Tests: declarations/storage-linkage/block-extern-same-unit.cpp, declarations/storage-linkage/block-extern-initializer.cpp, declarations/storage-linkage/block-extern-after-local.cpp, declarations/storage-linkage/extern-with-initializer.cpp

### LNK-1 (required)

A variable or function declared at namespace scope has *external* linkage
unless a rule below says otherwise. The same name with external linkage in
different translation units refers to one entity.

Tests: declarations/storage-linkage/external-default.cpp

### LNK-2 (required)

`static` on a namespace-scope variable or function gives it *internal*
linkage. Each translation unit that declares the name has its own entity,
and other translation units can't refer to it.

Tests: declarations/storage-linkage/static-internal.cpp, declarations/storage-linkage/static-not-visible.cpp

### LNK-3 (required)

A namespace-scope variable whose type is `const`, including one declared
`constexpr`, has internal linkage, unless it is also `volatile`, declared
`inline`, or declared `extern`.

    const int limit = 1;           // internal: each unit has its own
    extern const int answer = 42;  // external

Tests: declarations/storage-linkage/const-internal.cpp, declarations/storage-linkage/extern-const-external.cpp, declarations/storage-linkage/volatile-const-external.cpp

### LNK-4 (required)

An `inline` variable or function with external linkage may be defined in
every translation unit that uses it. All of those definitions are one
entity, with one address.

Tests: declarations/storage-linkage/inline-one-entity.cpp

### LNK-5 (required)

Every name declared in an unnamed namespace has internal linkage
*(namespaces referenced)*.

Tests: declarations/storage-linkage/unnamed-namespace-internal.cpp

### LNK-6 (no-diagnostic)

An entity with external linkage that is not `inline` may be defined only
once in the whole program. An `inline` entity may be defined in several
translation units, but every definition must be the same. A program that
breaks either requirement is invalid, and an implementation need not
diagnose it. Linkers usually report a second definition of a non-inline
entity. Different definitions of an `inline` entity usually go unreported.

Tests: declarations/storage-linkage/duplicate-variable.cpp, declarations/storage-linkage/duplicate-function.cpp, declarations/storage-linkage/inline-different-bodies.cpp

### LNK-7 (no-diagnostic)

A variable or function that the program uses must be defined somewhere in
the program. A program that uses one with no definition is invalid, and an
implementation need not diagnose it. Declarations that are never used need
no definition (question Q-0011).

Tests: declarations/storage-linkage/missing-definition.cpp, declarations/storage-linkage/unused-missing-definition.cpp

### LNK-8 (required)

A later declaration of a name with `extern`, or of a function with no
storage class, keeps the linkage of an earlier `static` declaration, so
the name stays internal. A `static` declaration that follows a declaration
with external linkage is rejected.

    static int x = 1;
    extern int x;          // still internal
    extern int y;
    static int y;          // rejected

Tests: declarations/storage-linkage/static-then-extern.cpp, declarations/storage-linkage/extern-then-static.cpp, declarations/storage-linkage/function-then-static.cpp

### LNK-9 (required)

Variables declared in a block without `extern` have no linkage. They are
unrelated to any other entity with the same name. A function declared in a
block, and a variable declared `extern` in a block, refer to an entity
with linkage at namespace scope, which may be defined in another
translation unit.

Tests: declarations/storage-linkage/block-function-declaration.cpp, declarations/storage-linkage/block-variables-no-linkage.cpp

## Evidence

All seven compilers agree on all 40 tests. Results are in
`tests/RESULTS.md`. 21 tests span two translation units (decision 0014).
EDG checks each unit and does not link.

- **LNK-6.** Each translation unit compiles without a diagnostic. The
  linker reports a second definition of a non-inline variable or function
  ("multiple definition of"). Two different definitions of an `inline`
  function link silently, and the body that runs varies. With Clang 22 and
  GCC 16 at `-O0`, both units call whichever definition was linked first.
  At `-O2`, each unit uses its own. The test accepts any of these results.
- **LNK-7.** Each translation unit compiles without a diagnostic. Only the
  linker reports the missing definition ("undefined reference to").
- **STO-6.** Clang reports a storage class on a class-only declaration as
  a pedantic warning, which becomes an error under `-pedantic-errors`. GCC
  and EDG reject it in every mode.

## Divergences

None.

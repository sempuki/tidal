# Namespaces and using

A *namespace* is a named scope at namespace level that groups declarations.
Its members are named from outside as `N::member`. A *using declaration*
(`using N::name;`) declares one name from a namespace in the current scope.
A *using directive* (`using namespace N;`) makes every name in a namespace
visible to unqualified lookup in the current scope. Rules are prefixed `NS`.

Name lookup in general, argument-dependent lookup, and using declarations
inside classes belong to other topics *(referenced)*.

Depends on: scopes *(referenced)*, name lookup *(referenced)*,
[linkage](storage-linkage.md), overload resolution *(referenced)*.

Inputs: clang-sema-declcxx, clang-diag-sema, gcc-cp-decl, edg-decls (see
`inputs/MANIFEST.md`).

## Syntax

    namespace-def   = [ "inline" ] "namespace" [ path ] "{" { declaration } "}"
    path            = [ "inline" ] name { "::" [ "inline" ] name }
    namespace-alias = "namespace" name "=" qualified-name ";"
    using-decl      = "using" qualified-name ";"
    using-directive = "using" "namespace" qualified-name ";"

## Rules

### NS-1 (required)

A namespace definition may appear only at namespace scope, never in a block.
Defining a namespace that already exists reopens it and adds members. From
outside, a member is named as `N::member`, and `::name` names a member of
the global namespace. A program is rejected if it names a member without
qualification from outside its namespace (absent NS-7 or NS-8), defines a
namespace in a block, or gives a namespace the name of another entity in
the same scope.

Tests: declarations/namespaces/define-and-qualify.cpp, declarations/namespaces/global-qualifier.cpp, declarations/namespaces/unqualified-outside.cpp, declarations/namespaces/block-scope-namespace.cpp, declarations/namespaces/name-conflict.cpp

### NS-2 (required)

`namespace A::B::C { ... }` defines or reopens each namespace in the path,
as if they were written nested. A name in the path after the first may be
marked `inline`.

Tests: declarations/namespaces/nested-definition.cpp, declarations/namespaces/nested-inline-definition.cpp

### NS-3 (required)

The members of an `inline` namespace are also members of the enclosing
namespace, so `Lib::version` finds `Lib::V2::version` when `V2` is
inline. A namespace must be declared `inline` at its first definition if
it is ever inline. Later definitions may leave `inline` out. A program is
rejected if it reopens a non-inline namespace as `inline`.

    namespace Lib {
      inline namespace V2 { int version = 2; }
      namespace V1 { int version = 1; }
    }
    // Lib::version is 2, Lib::V1::version is 1

Tests: declarations/namespaces/inline-namespace.cpp, declarations/namespaces/inline-then-reopen-plain.cpp, declarations/namespaces/inline-on-reopen.cpp

### NS-4 (required)

The members of an unnamed namespace can be named without qualification in
the enclosing scope, and have internal linkage (LNK-5).

Tests: declarations/namespaces/unnamed-namespace.cpp

### NS-5 (required)

`namespace L = A::B;` declares `L` as another name for `A::B`. A program
is rejected if it declares `L` as an alias for a different namespace in
the same scope.

Tests: declarations/namespaces/alias.cpp, declarations/namespaces/alias-conflict.cpp

### NS-6 (required)

A member already declared in a namespace may be defined outside it with a
qualified name, as in `int N::x = 7;`. The definition must be in a
namespace that encloses `N`. A program is rejected if the member wasn't
declared in `N` first, or if the definition is in a namespace that
doesn't enclose `N`.

Tests: declarations/namespaces/out-of-namespace-definition.cpp, declarations/namespaces/undeclared-qualified-definition.cpp, declarations/namespaces/definition-in-unrelated-namespace.cpp

### NS-7 (required)

`using N::name;` declares `name` in the current scope as a name for the
member of `N`. For a function, it covers every overload of `name`
declared in `N` so far. Overloads declared in `N` later aren't included.
The same using declaration may be repeated (in a block too, question
Q-0019). A program is rejected if the using declaration lacks a qualified
name, names a namespace, or conflicts with another declaration of `name`
in the same scope, whichever comes first.

    namespace N { int f(int); }
    using N::f;
    namespace N { int f(double); }
    f(1.5);   // calls N::f(int): f(double) came after the using declaration

Tests: declarations/namespaces/using-declaration.cpp, declarations/namespaces/using-declaration-overloads.cpp, declarations/namespaces/using-declaration-snapshot.cpp, declarations/namespaces/using-declaration-repeat-namespace.cpp, declarations/namespaces/using-declaration-repeat-block.cpp, declarations/namespaces/using-declaration-unqualified.cpp, declarations/namespaces/using-declaration-namespace.cpp, declarations/namespaces/using-declaration-conflict.cpp, declarations/namespaces/using-declaration-then-conflict.cpp

### NS-8 (required)

`using namespace N;` makes the members of `N` visible to unqualified
lookup in the current scope, until the scope ends. It is transitive: the
members of namespaces named in using directives inside `N` become visible
too. Names declared in the current scope hide them. When two namespaces
made visible this way have a member with the same name, an unqualified use
of that name is ambiguous, and a program that uses it is rejected. The
directive itself is fine. A program is rejected if the directive names
something other than a namespace.

Tests: declarations/namespaces/using-directive.cpp, declarations/namespaces/using-directive-transitive.cpp, declarations/namespaces/using-directive-local-hides.cpp, declarations/namespaces/using-directive-ambiguous-unused.cpp, declarations/namespaces/using-directive-ambiguous-use.cpp, declarations/namespaces/using-directive-block-scope.cpp, declarations/namespaces/using-directive-not-namespace.cpp

## Evidence

Results are in `tests/RESULTS.md`. All seven compilers agree on 31 of 32
tests.

## Divergences

### NS-D1: repeated using declaration in a block

Clang 22 and 18 and GCC 16 accept it. GCC 14 and EDG 7.0 reject it.
**Provisional answer:** allowed. See Q-0019.

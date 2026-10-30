# Namespaces and using

A *namespace* is a named scope that groups declarations at namespace
scope. A *using declaration* declares one member of a namespace in the
current scope. A *using directive* makes all members of a namespace
visible to unqualified lookup in the current scope.

Name lookup in general, argument-dependent lookup, and using declarations
in classes are other topics *(referenced)*.

## Syntax

```{eval-rst}
.. productionlist:: namespaces
   namespace_def: ["inline"] "namespace" [`ns_path`]
                : "{" {`declaration`} "}"
   ns_path: ["inline"] `name` {"::" ["inline"] `name`}
   namespace_alias: "namespace" `name` "=" `qualified_name` ";"
   using_decl: "using" `qualified_name` ";"
   using_directive: "using" "namespace" `qualified_name` ";"
```

## Legality Rules

:::{rule} NS-1
:kind: required
:tests: declarations/namespaces/define-and-qualify.cpp declarations/namespaces/global-qualifier.cpp declarations/namespaces/unqualified-outside.cpp declarations/namespaces/block-scope-namespace.cpp declarations/namespaces/name-conflict.cpp

A namespace definition must be at namespace scope. A second definition of
the same namespace adds members to it. A program must use these forms to
name a member from outside its namespace:

- `N::member` for a member of `N`,
- `::member` for a member of the global namespace,
- a name that a using declaration or using directive makes visible.

A namespace must not have the name of another entity in the same scope.
:::

:::{rule} NS-2
:kind: required
:tests: declarations/namespaces/nested-definition.cpp declarations/namespaces/nested-inline-definition.cpp

`namespace A::B::C` defines or reopens each namespace in the path, in
nested order. Each name after the first can have `inline`.
:::

:::{rule} NS-3
:kind: required
:tests: declarations/namespaces/inline-namespace.cpp declarations/namespaces/inline-then-reopen-plain.cpp declarations/namespaces/inline-on-reopen.cpp

The members of an `inline` namespace are also members of the enclosing
namespace. An inline namespace must have `inline` in its first definition.
A later definition can omit `inline`. A namespace that is first defined
without `inline` must not be defined with `inline` later.
:::

:::{rule} NS-4
:kind: required
:tests: declarations/namespaces/unnamed-namespace.cpp

The enclosing scope can use the members of an unnamed namespace without
qualification. These members have internal {term}`linkage` (LNK-5).
:::

:::{rule} NS-5
:kind: required
:tests: declarations/namespaces/alias.cpp declarations/namespaces/alias-conflict.cpp

`namespace L = A::B;` makes `L` a second name for `A::B`. In the same
scope, `L` must not be an alias for a different namespace.
:::

:::{rule} NS-6
:kind: required
:tests: declarations/namespaces/out-of-namespace-definition.cpp declarations/namespaces/undeclared-qualified-definition.cpp declarations/namespaces/definition-in-unrelated-namespace.cpp

A program can define a member of `N` outside `N` with a qualified name.
The member must have a declaration in `N` first. The definition must be
in a namespace that encloses `N`.
:::

:::{rule} NS-7
:kind: required
:tests: declarations/namespaces/using-declaration.cpp declarations/namespaces/using-declaration-overloads.cpp declarations/namespaces/using-declaration-snapshot.cpp declarations/namespaces/using-declaration-repeat-namespace.cpp declarations/namespaces/using-declaration-repeat-block.cpp declarations/namespaces/using-declaration-unqualified.cpp declarations/namespaces/using-declaration-namespace.cpp declarations/namespaces/using-declaration-conflict.cpp declarations/namespaces/using-declaration-then-conflict.cpp

`using N::name;` declares `name` in the current scope for the member of
`N`. For a function, it includes the overloads that `N` declares before
the using declaration. A using declaration:

- must have a qualified name,
- must not name a namespace,
- must not conflict with another declaration of `name` in the same scope,
- can occur again in the same scope, also in a block
  ({question}`Q-0019`).
:::

:::{rule} NS-8
:kind: required
:tests: declarations/namespaces/using-directive.cpp declarations/namespaces/using-directive-transitive.cpp declarations/namespaces/using-directive-local-hides.cpp declarations/namespaces/using-directive-ambiguous-unused.cpp declarations/namespaces/using-directive-ambiguous-use.cpp declarations/namespaces/using-directive-block-scope.cpp declarations/namespaces/using-directive-not-namespace.cpp

`using namespace N;` makes the members of `N` visible to unqualified
lookup until the current scope ends. The rules for this are:

- The directive must name a namespace.
- The members of namespaces that using directives in `N` name also become
  visible.
- A name that the current scope declares hides these members.
- If two such namespaces have a member with the same name, an unqualified
  use of that name is ambiguous. A program must not use an ambiguous name.
:::

## Examples

```cpp
namespace Lib {
  inline namespace V2 { int version = 2; }
  namespace V1 { int version = 1; }
}
// Lib::version is 2, Lib::V1::version is 1

namespace N { int f(int); }
using N::f;
namespace N { int f(double); }
f(1.5);                  // calls N::f(int)

namespace L = Long::Nested::Name;
int N::x = 7;            // N must declare x first

namespace A { int x; }
namespace B { int x; }
using namespace A;
using namespace B;       // valid
int y = x;               // rejected: x is ambiguous
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 31 of
32 tests.

Divergence NS-D1 is about a repeated using declaration in a block. Clang
22, Clang 18, and GCC 16 accept it. GCC 14 and EDG 7.0 reject it. The
provisional answer is that it is valid ({question}`Q-0019`).

Inputs: clang-sema-declcxx, clang-diag-sema, gcc-cp-decl, edg-decls (see
`inputs/MANIFEST.md`).

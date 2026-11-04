# Namespaces and using

A *namespace* is a named scope that groups declarations at namespace
scope. A *using declaration* declares one member of a namespace in the
current scope. A *using directive* makes all members of a namespace
visible to unqualified lookup in the current scope.

These topics are on other pages:

- name lookup in general, and qualified lookup through using directives
  ({principle}`NAM-P2`, {principle}`NAM-P5`, {principle}`NAM-P6`),
- argument-dependent lookup ({rule}`NAM-5`, {rule}`NAM-27`),
- using declarations in classes ({rule}`DER-3`).

## Principles

:::{principle} NS-P1
:rules: NS-1 NS-10 NS-2 NS-11 NS-6 NS-15 NS-5 NS-14

A namespace is a scope with a name that never closes. A program can:

- reopen it,
- nest it by a path,
- define a member outside it with a qualified name,
- give it a second name.

From outside, a member is reached through its namespace
({principle}`NAM-P2`), and the namespace itself takes a name in the
scope that contains it, so no other entity there can have that name.
:::

:::{principle} NS-P2
:rules: NS-3 NS-12 NS-4 NS-13

An `inline` namespace and an unnamed namespace lend their members to
the enclosing namespace.

| Namespace | Lends its members | Result | A name declared in both places is ambiguous for |
|---|---|---|---|
| `inline` | By name | A qualified name through the enclosing namespace finds the members. | a qualified use through the enclosing namespace (NS-12) |
| Unnamed | Without a name | Its members have internal linkage. | an unqualified use (NS-13). A qualified use finds the direct member ({rule}`NAM-13`). |

Neither declaration hides the other, so a lookup that reaches both
places finds two entities.
:::

:::{principle} NS-P3
:rules: NS-7 NS-16 NS-8 NS-17 NS-9

A using declaration and a using directive both let a program use a
namespace's names without qualification. The using declaration is the
stronger of the two.

| | Using declaration | Using directive |
|---|---|---|
| What the form is | A declaration in the current scope | Weaker. The directive only makes the members visible to lookup. |
| Another declaration in the current scope | Conflicts with the other declaration, as any declaration does | Hides the members |
| Other effects | Carries the overloads that the using declaration saw | Two directives can make a name ambiguous. |

Neither form belongs in a class.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   namespace_def: ["inline"] "namespace" [`ns_path`]
                :    "{" {`declaration`} "}"
   ns_path: ["inline"] `identifier` {"::" ["inline"] `identifier`}
   namespace_alias: "namespace" `identifier` "=" `qualified_name` ";"
   using_decl: "using" ["typename"] `qualified_name` ["..."]
             : {"," ["typename"] `qualified_name` ["..."]} ";"
   using_directive: "using" "namespace" `qualified_name` ";"
```

A `using_decl` with `...` expands a pack of bases ({rule}`PACK-21`).

## Constraints

:::{rule} NS-1
:kind: required
:tests: declarations/namespaces/define-and-qualify.cpp declarations/namespaces/global-qualifier.cpp

A second definition of the same namespace adds members to it. A program
can name a member from outside its namespace in these forms:

- `N::member` for a member of `N`,
- `::member` for a member of the global namespace,
- a name that a using declaration or using directive makes visible,
- an unqualified function call that argument-dependent lookup resolves
  ({rule}`NAM-5`),
- an unqualified name in the parts of a declaration that come after a
  qualified name of the member, such as the body of `N::f`
  ({rule}`NAM-21`).

NS-10 lists what is not valid.
:::

:::{rule} NS-10
:kind: reject
:tests: declarations/namespaces/unqualified-outside.cpp declarations/namespaces/block-scope-namespace.cpp declarations/namespaces/name-conflict.cpp declarations/namespaces/using-directive-block-scope.cpp

A namespace definition must be at namespace scope. A program must use
the forms that NS-1 lists to name a member from outside its namespace
({rule}`NAM-8`).
A namespace must not have the name of another entity in the same scope.
:::

:::{rule} NS-2
:kind: required
:tests: declarations/namespaces/nested-definition.cpp declarations/namespaces/nested-inline-definition.cpp

`namespace A::B::C` defines or reopens each namespace in the path, in
nested order. Each name after the first can have `inline`. NS-11 lists
what is not valid.
:::

:::{rule} NS-11
:kind: reject
:tests: declarations/namespaces/nested-first-inline.cpp declarations/namespaces/nested-first-inline-both.cpp

The first name in the path of a nested namespace definition must not be
`inline`.
:::

:::{rule} NS-3
:kind: required
:tests: declarations/namespaces/inline-namespace.cpp declarations/namespaces/inline-then-reopen-plain.cpp declarations/namespaces/qualified-definition-inline-namespace-member.cpp declarations/namespaces/qualified-definition-inline-member-inside-named.cpp

The members of an `inline` namespace are also members of the enclosing
namespace. A later definition of an inline namespace can omit `inline`.
NS-12 lists what is not valid.
:::

:::{rule} NS-12
:kind: reject
:tests: declarations/namespaces/inline-on-reopen.cpp declarations/namespaces/inline-namespace-duplicate.cpp

An inline namespace must have `inline` in its first definition. A
namespace that is first defined without `inline` must not be defined
with `inline` later.

If a name is declared both in an inline namespace and in its enclosing
namespace, a qualified use through the enclosing namespace is ambiguous.
:::

:::{rule} NS-4
:kind: required
:tests: declarations/namespaces/unnamed-namespace.cpp declarations/namespaces/unnamed-namespace-nested-members.cpp

The enclosing scope can use the members of an unnamed namespace without
qualification. These members have internal {term}`linkage` (LNK-5).
NS-13 lists what is not valid. Lookup treats an unnamed namespace as a
using directive in the namespace that contains it ({rule}`NAM-13`,
{rule}`NAM-23`). So a qualified name `N::x` also finds a member `x` of
an unnamed namespace in `N`.
:::

:::{rule} NS-13
:kind: reject
:tests: declarations/namespaces/unnamed-namespace-duplicate.cpp declarations/namespaces/unnamed-namespace-duplicate-function.cpp

If a name is declared both in an unnamed namespace and in the enclosing
namespace, an unqualified use is ambiguous.
:::

:::{rule} NS-5
:kind: required
:tests: declarations/namespaces/alias.cpp declarations/namespaces/alias-of-alias-block.cpp

`namespace L = A::B;` makes `L` a second name for `A::B`. NS-14 lists
what is not valid.
:::

:::{rule} NS-14
:kind: reject
:tests: declarations/namespaces/alias-conflict.cpp declarations/namespaces/alias-not-reopen.cpp

In the same scope, a namespace alias `L` must not be an alias for a
different namespace. A namespace definition must not use `L` to reopen
the namespace that `L` names.
:::

:::{rule} NS-6
:kind: required
:tests: declarations/namespaces/out-of-namespace-definition.cpp declarations/namespaces/qualified-definition-storage-class.cpp declarations/storage-linkage/qualified-static-after-extern.cpp

A program can define a member of `N` outside `N` with a qualified name.
The definition can have `extern` or `static`, and {rule}`LNK-10` applies
as to any later declaration. NS-15 lists what is not valid.
:::

:::{rule} NS-15
:kind: reject
:tests: declarations/namespaces/undeclared-qualified-definition.cpp declarations/namespaces/definition-in-unrelated-namespace.cpp declarations/namespaces/qualified-definition-inside-own-namespace.cpp declarations/namespaces/qualified-definition-inline-namespace-member.cpp declarations/namespaces/qualified-definition-inline-member-inside-named.cpp declarations/namespaces/qualified-definition-inline-member-inside-inline.cpp declarations/namespaces/qualified-definition-through-directive.cpp declarations/namespaces/qualified-definition-unnamed-namespace-member.cpp

For a definition of a member of `N` outside `N`, these rules apply:

- The member must have a declaration first, in `N` or in an inline
  namespace of `N` ({rule}`NS-3`). A declaration that a using directive
  or an unnamed namespace in `N` makes visible does not count.
- The definition must be in a namespace that encloses `N`, and not in
  `N` itself ({question}`Q-0371`). For a member of an inline namespace
  of `N`, the check uses that inline namespace in place of `N`
  ({question}`Q-0375`).
:::

:::{rule} NS-7
:kind: required
:tests: declarations/namespaces/using-declaration.cpp declarations/namespaces/using-declaration-overloads.cpp declarations/namespaces/using-declaration-snapshot.cpp declarations/namespaces/using-declaration-repeat-namespace.cpp declarations/namespaces/using-declaration-repeat-block.cpp declarations/namespaces/using-list.cpp

`using N::name;` declares `name` in the current scope for the member of
`N`. A using declaration also follows these rules:

- For a function, the using declaration includes the overloads that
  `N` declares before the using declaration.
- A using declaration can list several names separated by commas.
- The same using declaration can occur again in the same scope, also
  in a block ({question}`Q-0019`).

NS-16 lists what is not valid.
:::

:::{rule} NS-16
:kind: reject
:tests: declarations/namespaces/using-declaration-unqualified.cpp declarations/namespaces/using-declaration-namespace.cpp declarations/namespaces/using-declaration-conflict.cpp declarations/namespaces/using-declaration-then-conflict.cpp

A using declaration:

- must have a qualified name,
- must not name a namespace,
- must not conflict with another declaration of `name` in the same
  scope.
:::

:::{rule} NS-8
:kind: required
:tests: declarations/namespaces/using-directive.cpp declarations/namespaces/using-directive-transitive.cpp declarations/namespaces/using-directive-local-hides.cpp declarations/namespaces/using-directive-ambiguous-unused.cpp declarations/namespaces/directive-sees-later-members.cpp

`using namespace N;` makes the members of `N` visible to unqualified
lookup until the current scope ends. These rules apply:

- Members that `N` declares after the directive are visible too.
- The members of namespaces that using directives in `N` name also
  become visible.
- A name that the current scope declares hides these members.
- If two such namespaces have a member with the same name, an
  unqualified use of that name is ambiguous.

Unqualified lookup finds the members at the level of the innermost
namespace that contains both the directive and the named namespace. So a
global variable with the same name gives an ambiguous use. NS-17 lists
what is not valid.
:::

:::{rule} NS-17
:kind: reject
:tests: declarations/namespaces/using-directive-ambiguous-use.cpp declarations/namespaces/using-directive-not-namespace.cpp declarations/namespaces/using-directive-global-ambiguity.cpp

A using directive must name a namespace. A program must not use a name
that NS-8 makes ambiguous.
:::

:::{rule} NS-9
:kind: reject
:tests: declarations/namespaces/directive-in-class.cpp declarations/namespaces/using-class-member-at-namespace.cpp

A using directive must not occur in a class. A using declaration at
namespace scope must not name a class member.
:::

## Examples

% tidal-example: reject
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
namespace M { extern const int build; }
extern const int M::build = 2026;  // extern on a qualified definition
namespace M { static int count(); extern int total; }
static int M::count() { return 1; }  // valid: count is static already
static int M::total = 0;             // rejected: total has external
                                     // linkage (LNK-10)

namespace A { int x; }
namespace B { int x; }
using namespace A;
using namespace B;       // valid
int y = x;               // rejected: x is ambiguous

int v = 1;
namespace C { int v = 2; }
int read() {
  using namespace C;     // C::v counts as a member of the global
  return v;              // namespace here: rejected, v is ambiguous
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 50
of 53 tests.

- NS-15: GCC and EDG reject a definition with the name `N::f` inside
  `N` itself. Clang accepts it with a warning ("extra qualification")
  ({question}`Q-0371`). Every
  compiler accepts `N::f` for a member of an inline namespace of `N`.
  Inside `N`, GCC and Clang accept that definition, and EDG rejects it
  ({question}`Q-0375`). Inside the
  inline namespace, every compiler rejects it.
  Every compiler rejects `N::f` for a function that only a using
  directive or an unnamed namespace in `N` declares.

Divergence NS-D1 is about a repeated using declaration in a block. Clang
22, Clang 18, and GCC 16 accept the repeated declaration. GCC 14 and
EDG 7.0 reject it. The provisional answer is that the repeated
declaration is valid ({question}`Q-0019`).

Inputs: clang-sema-declcxx, clang-diag-sema, edg-decls (see
`inputs/MANIFEST.md`).

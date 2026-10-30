# Structured bindings

A structured binding declaration declares several names at one time. Each
name refers to one element of an object. An element is an array element,
a data member of a class, or an element from the tuple protocol.

## Syntax

```{eval-rst}
.. productionlist:: structured-bindings
   sb_decl: ["static" | "thread_local"] {`cv`} "auto" ["&" | "&&"]
          :    "[" `name` {"," `name`} "]" `sb_init` ";"
   sb_init: "=" `expression` | "(" `expression` ")" | "{" `expression` "}"
   cv: "const" | "volatile"
```

## Legality Rules

:::{rule} SB-2
:kind: reject
:tests: declarations/structured-bindings/count-mismatch-array.cpp declarations/structured-bindings/count-mismatch-class.cpp

The number of names must be equal to the number of elements. The number
of elements is one of these:

- the bound of the array,
- the number of data members (SB-3),
- the tuple size (SB-5).
:::

:::{rule} SB-3
:kind: required
:tests: declarations/structured-bindings/aggregate-members.cpp declarations/structured-bindings/members-in-base.cpp declarations/structured-bindings/private-member.cpp declarations/structured-bindings/anonymous-union-member.cpp declarations/structured-bindings/members-split-across-bases.cpp declarations/structured-bindings/union-rejected.cpp declarations/structured-bindings/scalar-rejected.cpp

If the hidden variable has class type and SB-5 does not apply, the names
refer to its non-static data members in declaration order. These members
must:

- all be in the same class, which can be a base class,
- all be public,
- not include an anonymous union.

The type of the hidden variable must not be a union or a scalar type.
:::

:::{rule} SB-4
:kind: required
:tests: declarations/structured-bindings/binding-types.cpp

For an array element or a data member, `decltype` of a name gives the
declared type of the element. The cv-qualifiers of the hidden variable are
added. The result is never a reference type.
:::

:::{rule} SB-5
:kind: required
:tests: declarations/structured-bindings/tuple-protocol.cpp declarations/structured-bindings/tuple-count-mismatch.cpp declarations/structured-bindings/tuple-element-missing.cpp

If `std::tuple_size<E>` is a complete type, `E` uses the tuple protocol.
`E` is the type of the hidden variable. Its `value` gives the number of
elements. Element `i` comes from a call to `get<i>()` *(lookup
referenced)*. Its type is `std::tuple_element<i, E>::type`, which must
exist *(library referenced)*.
:::

:::{rule} SB-7
:kind: required
:tests: declarations/structured-bindings/initializer-equals.cpp declarations/structured-bindings/initializer-parens.cpp declarations/structured-bindings/initializer-braces.cpp declarations/structured-bindings/static-and-thread-local.cpp declarations/structured-bindings/explicit-type.cpp declarations/structured-bindings/pointer-declarator.cpp declarations/structured-bindings/decltype-auto.cpp declarations/structured-bindings/constexpr-binding.cpp declarations/structured-bindings/no-initializer.cpp declarations/structured-bindings/empty-bindings.cpp declarations/structured-bindings/as-class-member.cpp declarations/structured-bindings/as-parameter.cpp declarations/structured-bindings/with-other-declarator.cpp declarations/structured-bindings/parenthesized.cpp declarations/structured-bindings/constrained.cpp declarations/structured-bindings/template.cpp

A structured binding declaration uses `auto`. It can add `const`,
`volatile`, `&`, `&&`, `static`, and `thread_local`. It must have an
initializer after `=`, in parentheses, or in braces. The declaration must
not:

- use an explicit type, `auto*`, or `decltype(auto)`,
- be `constexpr` ({question}`Q-0007`),
- have an empty list of names,
- be a class member or a function parameter,
- have other declarators,
- put the list of names in parentheses,
- use a constrained `auto` ({question}`Q-0016`),
- be a template.
:::

:::{rule} SB-8
:kind: reject
:tests: declarations/structured-bindings/duplicate-names.cpp declarations/structured-bindings/underscore-twice.cpp declarations/structured-bindings/names-in-scope.cpp declarations/structured-bindings/own-initializer.cpp

The names in one declaration must all be different. This includes two `_`
names. A name must not be declared already in the same scope. A name must
not occur in the initializer of its own declaration.
:::

## Dynamic Semantics

:::{rule} SB-1
:kind: required
:tests: declarations/structured-bindings/array-copy.cpp declarations/structured-bindings/array-reference.cpp

A structured binding declaration creates one hidden variable. The
variable has the written `auto`, cv-qualifiers, and reference, and the
initializer initializes it. Each name refers to one element of the hidden
variable.
:::

:::{rule} SB-6
:kind: required
:tests: declarations/structured-bindings/lifetime-extended.cpp

If the hidden variable is a reference bound to a temporary, the temporary
exists as long as the names do (REF-4).
:::

## Examples

```cpp
int a[2] = {1, 2};
auto [x, y] = a;          // x refers to a copy of a[0]
x = 10;                   // a[0] is still 1
auto& [p, q] = a;         // p refers to a[0]

struct S { int m; const double n; };
S s{1, 2.0};
auto& [r, t] = s;         // decltype(r) is int, decltype(t) is const double
const auto [u, v] = s;    // decltype(u) is const int

int b[3];
auto [i, j] = b;          // rejected: 3 elements, 2 names
auto [k, k] = a;          // rejected: duplicate name
auto [c, d]{a};           // valid: braced initializer
```

## Evidence

All seven compilers agree on 34 of 36 tests. The results are in
`tests/RESULTS.md`.

- **SB-7.** Clang and GCC accept `constexpr` structured bindings and two
  `_` names as extensions from a later language version. They reject them
  only under `-pedantic-errors`. Clang reports an empty list of names as a
  pedantic diagnostic.
- **SB-7, braced array initializer.** Clang 18.1.3 rejects `auto [r, s]{a};`
  when `a` is an array. Clang 22, GCC, and EDG accept it. The provisional
  answer is that it is valid. See {question}`Q-0015`.
- **SB-7, constrained `auto`.** EDG 7.0 accepts `Any auto [x, y] = a;`.
  Clang and GCC reject it. The provisional answer is that it is rejected.
  See {question}`Q-0016`.

Inputs: clang-sema-declcxx, gcc-cp-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

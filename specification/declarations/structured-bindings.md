# Structured bindings

A structured binding declaration declares several names at one time.
Each name refers to one element of an object. An element is one of
these:

- an array element,
- a data member of a class,
- an element from the tuple protocol.

## Principles

:::{principle} SB-P1
:rules: SB-1 SB-6 SB-12

A structured binding declaration is one ordinary variable with several
names for its parts.

```text
initializer
    |
    v
hidden variable   (the written auto, cv-qualifiers, reference)
    |
    +-- element 1  <-- name 1
    +-- element 2  <-- name 2
    +-- ...
```

Each name refers to one element of the hidden variable. So each name is
an lvalue, whatever the initializer was, and a temporary that the hidden
variable binds to lives as long as the names do ({principle}`OBJ-P3`).
:::

:::{principle} SB-P2
:rules: SB-2 SB-3 SB-13 SB-4 SB-5 SB-17 SB-14 SB-9 SB-10 SB-16

The type of the hidden variable decides what its elements are, in a
fixed order of tests:

| Test | Type of the hidden variable | Elements |
|---|---|---|
| 1 | An array | Its elements |
| 2 | A type with a tuple size | The results of `get` |
| 3 | Any other class | Its data members, in declaration order |

The number of names must equal the number of elements. A name for an
array element or a data member has the declared type of its element,
with the qualifiers of the hidden variable added, so a binding to a
`mutable` member or a bit-field behaves as that member does. A name for
an element of the tuple protocol has the type that
`std::tuple_element` gives, as written.
:::

:::{principle} SB-P3
:rules: SB-7 SB-15 SB-8 SB-11

The declaration is deliberately narrow and has these limits:

- `auto` as its type,
- a few specifiers,
- one {term}`declarator` with one list of names.

Each name is a new declaration in the enclosing scope, so the names must
be distinct and must not repeat a name of that scope
({principle}`NAM-P1`).
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   sb_decl: ["static" | "thread_local"] {`cv`} "auto" ["&" | "&&"]
          :    "[" `identifier` {"," `identifier`} "]" `sb_init` ";"
   sb_init: "=" `assignment_expr` | "(" `assignment_expr` ")" | "{" `assignment_expr` "}"
```

## Constraints

:::{rule} SB-2
:kind: reject
:tests: declarations/structured-bindings/count-mismatch-array.cpp declarations/structured-bindings/count-mismatch-class.cpp declarations/structured-bindings/tuple-count-mismatch.cpp

The *elements* of the type of the hidden variable are the elements of an
array, the results of `get` for a type with a tuple size (SB-5), or the
non-static data members of any other class (SB-3). The number of names
must be equal to the number of elements. The number of elements is one
of these:

- the bound of the array,
- the number of data members (SB-3),
- the tuple size (SB-5).
:::

:::{rule} SB-3
:kind: required
:tests: declarations/structured-bindings/aggregate-members.cpp declarations/structured-bindings/members-in-base.cpp declarations/structured-bindings/friend-binds-private.cpp

If the hidden variable has class type and SB-5 does not apply, the names
refer to the non-static data members of the class in declaration order.
SB-13 lists what is not valid.
:::

:::{rule} SB-13
:kind: reject
:tests: declarations/structured-bindings/private-member.cpp declarations/structured-bindings/anonymous-union-member.cpp declarations/structured-bindings/members-split-across-bases.cpp declarations/structured-bindings/union-rejected.cpp declarations/structured-bindings/scalar-rejected.cpp declarations/structured-bindings/tuple-const-hidden-variable.cpp

The members that SB-3 names must:

- all be in the same class, which can be a base class,
- all be accessible at the declaration, for example in a member function
  or a friend of the class ({rule}`ACC-2`),
- not include an anonymous union.

The type of the hidden variable must not be a union or a {term}`scalar type`.
:::

:::{rule} SB-4
:kind: required
:tests: declarations/structured-bindings/binding-types.cpp declarations/structured-bindings/reference-member-binding-type.cpp declarations/structured-bindings/tuple-const-specialization.cpp

For an array element or a data member, `decltype` of a name gives the
declared type of the element, with the {term}`cv-qualifiers <cv-qualifier>` of the hidden
variable added. If the data member has a reference type, the result is
that reference type. For the tuple protocol, the result is
`std::tuple_element<i, E>::type` with nothing added (SB-5).
:::

:::{rule} SB-5
:kind: required
:tests: declarations/structured-bindings/tuple-protocol.cpp declarations/structured-bindings/tuple-index-size-type.cpp declarations/structured-bindings/tuple-index-int-parameter.cpp declarations/structured-bindings/tuple-const-hidden-variable.cpp declarations/structured-bindings/tuple-const-specialization.cpp declarations/structured-bindings/tuple-get-order.cpp declarations/structured-bindings/tuple-size-without-value.cpp declarations/structured-bindings/tuple-free-get.cpp declarations/structured-bindings/tuple-member-get-wins.cpp declarations/structured-bindings/tuple-member-get-not-template.cpp declarations/structured-bindings/tuple-get-ordinary-lookup.cpp

If `std::tuple_size<E>` is a complete type, `E` uses the tuple protocol.
`E` is the type of the hidden variable `e`, with its cv-qualifiers, so
`const auto&` looks for `std::tuple_size<const C>`. The `value` member of
`std::tuple_size<E>` gives the number of elements. Element `i` comes
from one of these calls, and its type is
`std::tuple_element<i, E>::type` ({rule}`LSUP-3`):

- `e.get<i>()`, if member lookup of `get` in `E` finds a member function
  template,
- otherwise `get<i>(e)`, where only {term}`argument-dependent lookup`
  ({rule}`NAM-5`) finds `get`.

`i` is a constant of type `std::size_t` *(library referenced)*
({question}`Q-0376`). The calls to `get` occur in the
order of the elements. If
`std::tuple_size<E>` is complete but has no `value` member, the names
refer to data members, as SB-3 gives ({question}`Q-0048`). SB-14 lists
what is not valid.
:::

:::{rule} SB-14
:kind: reject
:tests: declarations/structured-bindings/tuple-element-missing.cpp declarations/structured-bindings/tuple-get-ordinary-lookup.cpp

In the tuple protocol of SB-5, these must hold for each element:

- `std::tuple_element<i, E>::type` exists,
- the call to `get` that SB-5 gives is valid.
:::

:::{rule} SB-17
:kind: required
:tests: declarations/structured-bindings/tuple-get-argument-category.cpp declarations/structured-bindings/tuple-get-prvalue-lives.cpp declarations/structured-bindings/tuple-hidden-reference-kind.cpp

In the tuple protocol of SB-5, the call to `get` passes `e` as an
lvalue when the hidden variable is declared with `&`, and as an xvalue
in every other case. Each name refers to a hidden reference to its
element type. The reference is an lvalue reference if the call gives an
lvalue, and an rvalue reference otherwise, so a prvalue result lives as
long as the names ({rule}`REF-4`).
:::

:::{rule} SB-7
:kind: required
:tests: declarations/structured-bindings/initializer-equals.cpp declarations/structured-bindings/initializer-parens.cpp declarations/structured-bindings/initializer-braces.cpp declarations/structured-bindings/static-and-thread-local.cpp

A structured binding declaration uses `auto` and can add `const`,
`volatile`, `&`, `&&`, `static`, and `thread_local`. The initializer
comes after `=`, in parentheses, or in braces. SB-15 lists what is not
valid.
:::

:::{rule} SB-15
:kind: reject
:tests: declarations/structured-bindings/explicit-type.cpp declarations/structured-bindings/pointer-declarator.cpp declarations/structured-bindings/decltype-auto.cpp declarations/structured-bindings/constexpr-binding.cpp declarations/structured-bindings/no-initializer.cpp declarations/structured-bindings/empty-bindings.cpp declarations/structured-bindings/as-class-member.cpp declarations/structured-bindings/as-parameter.cpp declarations/structured-bindings/with-other-declarator.cpp declarations/structured-bindings/parenthesized.cpp declarations/structured-bindings/constrained.cpp declarations/structured-bindings/template.cpp

A structured binding declaration must have an initializer in one of the
forms that SB-7 gives. The declaration must not:

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

The names in one declaration must all be different. The rule applies
also to two `_` names. A name must not be declared already in the same
scope. A name must not occur in the initializer of its own declaration.
:::

:::{rule} SB-9
:kind: required
:tests: declarations/structured-bindings/mutable-member.cpp declarations/structured-bindings/mutable-member-const-reference.cpp

A binding to a `mutable` member can be changed, even if the hidden
variable is `const`.
:::

:::{rule} SB-10
:kind: required
:tests: declarations/structured-bindings/bit-field-assign.cpp declarations/structured-bindings/bit-field-assign-wraps.cpp

A binding to a bit-field member refers to the bit-field ({rule}`BF-1`).
A program can assign to the binding. SB-16 lists what is not valid.
:::

:::{rule} SB-16
:kind: reject
:tests: declarations/structured-bindings/bit-field-address.cpp declarations/structured-bindings/bit-field-address-copy.cpp

A program must not take the address of a binding to a bit-field member.
:::

:::{rule} SB-11
:kind: reject
:tests: declarations/structured-bindings/inline-specifier.cpp declarations/structured-bindings/extern-specifier.cpp declarations/structured-bindings/constinit-specifier.cpp declarations/structured-bindings/static-thread-local-ok.cpp

The only specifiers that can occur with `auto` in a structured binding
declaration are `static`, `thread_local`, `const`, and `volatile`.
`inline`, `extern`, and `constinit` are not valid ({question}`Q-0107`
for `constinit`).
:::

:::{rule} SB-12
:kind: required
:tests: declarations/structured-bindings/names-are-lvalues.cpp declarations/structured-bindings/names-are-lvalues-const-rvalue-reference.cpp

Each name of a structured binding is an lvalue. The rule holds also when
the hidden variable is an rvalue reference or the initializer is a
prvalue.
:::

## Runtime Behavior

:::{rule} SB-1
:kind: required
:tests: declarations/structured-bindings/array-copy.cpp declarations/structured-bindings/array-reference.cpp declarations/structured-bindings/array-hidden-copy.cpp

A structured binding declaration creates one hidden variable, and each
name refers to one element of that variable. The hidden variable has
these properties:

- It has the written `auto`, cv-qualifiers, and reference.
- The initializer initializes the hidden variable.
- For an array initializer without a reference, the hidden variable is
  an array with the same element type and bound, and each element is
  copied. The initializer does not decay to a pointer.
:::

:::{rule} SB-6
:kind: required
:tests: declarations/structured-bindings/lifetime-extended.cpp declarations/structured-bindings/lifetime-extended-rvalue-reference.cpp

If the hidden variable is a reference bound to a temporary, the
temporary exists as long as the names do (REF-4).
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

class Range {             // private data, and a member get
  int low, high;
public:
  template <std::size_t I> int get() const { return I == 0 ? low : high; }
};
// std::tuple_size and std::tuple_element specialized for Range only
Range g;
auto [lo, hi] = g;        // tuple protocol: E is Range
const auto& [cl, ch] = g; // rejected: E is const Range, which has no
                          // tuple size, so the names go to the private
                          // data members
```

## Evidence

All eight compilers agree on 60 of 65 tests. The results are in
`tests/RESULTS.md`.

- **SB-15.** Clang and GCC accept `constexpr` structured bindings and
  two `_` names as extensions from a later language version. The two
  compilers reject these forms only under `-pedantic-errors`. Clang
  reports an empty list of names as a pedantic diagnostic.
- **SB-8.** EDG 7.0 rejects two `_` names in one declaration in strict
  mode ("has already been declared"). So all eight compilers reject the
  declaration in the test's mode.
- **SB-7, braced array initializer.** Clang 18.1.3 rejects
  `auto [r, s]{a};` when `a` is an array. Clang 22, GCC, and EDG accept
  the declaration. The settled answer is that the declaration is
  valid. See {question}`Q-0015`.
- **SB-15, constrained `auto`.** EDG 7.0 accepts `Any auto [x, y] = a;`.
  Clang and GCC reject the declaration. The provisional answer is that
  the declaration is rejected. See {question}`Q-0016`.
- **SB-11.** EDG 7.0 and GCC 14 accept `constinit` on a structured
  binding. Clang and GCC 16 reject such a binding ({question}`Q-0107`).
- **SB-5, type of the index.** Clang and EDG pass the index of `get`
  as a `std::size_t`, so `template <auto I>` deduces that type. GCC
  passes an `int`. Every compiler accepts a member `get` with an `int`
  parameter. The provisional answer is `std::size_t`
  ({question}`Q-0376`).

Inputs: clang-sema-declcxx, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

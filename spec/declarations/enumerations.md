# Enumerations

An enumeration is a distinct type with named integer values, its
*enumerators*. A *scoped* enumeration (`enum class` or `enum struct`) keeps
its enumerators in its own scope. An *unscoped* enumeration (`enum`) puts
its enumerators in the enclosing scope. The *underlying type* is the
integer type that holds the values.

## Syntax

```{eval-rst}
.. productionlist:: enumerations
   enum_def: `enum_head` [":" `type`]
           :    "{" [`enumerator` {"," `enumerator`} [","]] "}"
   enum_opaque: `enum_head` ":" `type` ";"
              : | ("enum" "class" | "enum" "struct") `name` ";"
   enum_head: "enum" ["class" | "struct"] `name`
   enumerator: `name` ["=" `constant`]
```

## Static Semantics

:::{rule} ENUM-1
:kind: required
:tests: declarations/enumerations/enumerator-values.cpp declarations/enumerations/enumerator-refers-earlier.cpp declarations/enumerations/duplicate-values.cpp declarations/enumerations/enumerator-not-constant.cpp declarations/enumerations/duplicate-enumerator.cpp declarations/enumerations/implicit-value-overflow-scoped.cpp

An enumerator without a value gets the value of the previous enumerator
plus one. The first enumerator without a value gets 0. A value must be a
constant expression, and it can use earlier enumerators. These conditions
apply:

- Several enumerators can have the same value.
- Two enumerators must not have the same name.
- An implicit value must fit in a fixed underlying type (ENUM-4).
:::

:::{rule} ENUM-2
:kind: required
:tests: declarations/enumerations/unscoped-names-in-enclosing-scope.cpp declarations/enumerations/unscoped-converts-to-int.cpp declarations/enumerations/mixed-enum-arithmetic.cpp declarations/enumerations/int-to-unscoped-rejected.cpp declarations/enumerations/unscoped-name-conflict.cpp

The enumerators of an unscoped enumeration are declared in the enclosing
scope. They can also be named through the enumeration. A value converts
implicitly to an integer by {term}`integer promotion`. An integer does not
convert implicitly to the enumeration. An enumerator name must not be
declared already in the enclosing scope.
:::

:::{rule} ENUM-3
:kind: required
:tests: declarations/enumerations/scoped-names-scoped.cpp declarations/enumerations/scoped-no-conversion.cpp declarations/enumerations/scoped-no-arithmetic.cpp declarations/enumerations/scoped-compare-same-type.cpp declarations/enumerations/scoped-compare-int.cpp declarations/enumerations/scoped-explicit-conversion.cpp declarations/enumerations/increment-enum.cpp

The enumerators of a scoped enumeration are named only through the
enumeration. A value converts to and from an integer only with an
explicit cast. Two values of the same scoped enumeration can be compared.
A program must not:

- use a scoped value in arithmetic,
- compare a scoped value with an integer,
- initialize an integer from a scoped value,
- apply `++` or `--` to an enumeration without a user-declared operator
  *(referenced)*.
:::

:::{rule} ENUM-4
:kind: required
:tests: declarations/enumerations/scoped-default-underlying-int.cpp declarations/enumerations/fixed-underlying.cpp declarations/enumerations/scoped-value-too-large.cpp declarations/enumerations/fixed-underlying-narrowing.cpp declarations/enumerations/fixed-underlying-non-integral.cpp declarations/enumerations/fixed-bool.cpp

A scoped enumeration without `: type` has the underlying type `int`. A
fixed underlying type after `:` must be an integer type. Each enumerator
value must fit in the underlying type without narrowing.

The fixed underlying type can be `bool` or a character type. A
cv-qualifier on the underlying type is ignored.
:::

:::{rule} ENUM-7
:kind: required
:tests: declarations/enumerations/opaque-declaration.cpp declarations/enumerations/opaque-unscoped-without-type.cpp declarations/enumerations/opaque-mismatch.cpp declarations/enumerations/redeclare-scoped-as-unscoped.cpp

An enumeration can be declared without its enumerators before its
definition. Its underlying type must be known, so it must be scoped or
have `: type`. All declarations must agree on scoped or unscoped and on
the underlying type.
:::

:::{rule} ENUM-8
:kind: required
:tests: declarations/enumerations/list-init-fixed.cpp declarations/enumerations/list-init-no-fixed.cpp declarations/enumerations/list-init-narrowing.cpp declarations/enumerations/copy-list-from-integer.cpp

A variable of an enumeration type with a fixed underlying type can be
initialized from an integer in braces. This includes all scoped
enumerations. The conversion to the underlying type must not narrow. An
enumeration without a fixed underlying type cannot be initialized this
way.

This applies only to direct-list-initialization. Initialization with `=`
and a braced integer is not valid.
:::

:::{rule} ENUM-9
:kind: required
:tests: declarations/enumerations/using-enum.cpp declarations/enumerations/using-enum-conflict.cpp declarations/enumerations/using-enum-not-enum.cpp declarations/enumerations/using-declaration-enumerator.cpp declarations/enumerations/using-enum-opaque.cpp

`using enum E;` declares the enumerators of `E` in the current scope. `E`
must be an enumeration. No enumerator name of `E` can be declared already
in that scope.

The enumeration must be complete at the `using enum` declaration. An
opaque declaration is not enough ({question}`Q-0047`). A using declaration can
also name one enumerator of a scoped enumeration, such as `using C::R;`.
:::

:::{rule} ENUM-10
:kind: required
:tests: declarations/enumerations/distinct-types.cpp

Each enumeration is a distinct type. It is different from its underlying
type and from all other enumerations. An enumerator has the type of its
enumeration. Unary `+` on an unscoped value gives its promoted integer
type.
:::

## Undefined Behavior

:::{rule} ENUM-6
:kind: undefined
:tests: declarations/enumerations/cast-in-range.cpp declarations/enumerations/cast-out-of-range.cpp declarations/enumerations/cast-fixed-any-value.cpp declarations/enumerations/empty-enum-range.cpp

An enumeration with a fixed underlying type can hold all values of that
type. An enumeration without one holds the values of the smallest set of
bits that can represent all its enumerators. If a program converts a
value outside that range to the enumeration, the behavior is undefined.

An enumeration without enumerators and without a fixed type can hold the
values 0 and 1.
:::

## Implementation Requirements

:::{rule} ENUM-5
:kind: impl-defined
:tests: declarations/enumerations/unscoped-underlying-holds-values.cpp declarations/enumerations/unscoped-size-lp64.cpp declarations/enumerations/implicit-value-overflow-unscoped.cpp declarations/enumerations/unscoped-value-too-large.cpp

The underlying type of an unscoped enumeration without `: type` is
implementation-defined. It is an integer type that can hold all
enumerator values. It is not wider than `int` unless a value needs more.
If no standard integer type can hold all values, the implementation can
use a wider type or reject the enumeration.
:::

## Examples

```cpp
enum E { A, B, C = 10, D, F = B + 1 };   // 0, 1, 10, 11, 2

enum Color { Red, Green };
int i = Green;                 // 1
Color c = 0;                   // rejected: no implicit conversion

enum class S { X, Y };
bool lt = S::X < S::Y;         // true
auto bad = S::X + 1;           // rejected
int n = static_cast<int>(S::Y);   // 1

enum U : unsigned char { P = 255 };   // valid
enum V : unsigned char { Q = 300 };   // rejected: narrowing
enum class W { R = 1LL << 40 };       // rejected: int is too small

enum G { M = 1, N = 4 };       // values fit in 3 bits: 0 to 7
G g7 = static_cast<G>(7);      // valid
G g8 = static_cast<G>(8);      // undefined

enum class H : int;            // declared before the definition
H h{5};                        // valid: fixed underlying type

using enum Color;
Color k = Red;                 // Red without Color::
```

## Evidence

All seven compilers agree on 39 of 41 tests. The results are in
`tests/RESULTS.md`.

- **ENUM-5.** With `-fshort-enums`, Clang and GCC give `enum { S = 1 }` an
  underlying type of 1 byte. By default its size is the size of `int`.
- **ENUM-5, no standard underlying type.** GCC accepts an unscoped
  enumeration that needs more than 64 bits, and uses a 128-bit type. Clang
  and EDG reject it. The provisional answer is that this is
  implementation-defined. See {question}`Q-0018`.
- **ENUM-6.** Clang rejects `static_cast<E>(8)` during constant evaluation
  ("outside the valid range of values [0, 7]"). GCC and EDG accept it. The
  provisional answer is that the behavior is undefined. See
  {question}`Q-0017`.

Inputs: clang-sema-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

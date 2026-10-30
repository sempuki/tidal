# Enumerations

An enumeration is a distinct integer-like type whose named values, its
*enumerators*, are listed in its definition. A *scoped* enumeration
(`enum class` or `enum struct`) keeps its enumerators inside its own scope
and doesn't convert implicitly to integers. An *unscoped* enumeration
(`enum`) puts its enumerators in the enclosing scope and converts
implicitly. Either kind may name a fixed *underlying type*, the integer
type that holds its values. Rules are prefixed `ENUM`.

Depends on: [usual arithmetic conversions](../glossary.md#usual-arithmetic-conversions),
[integer promotion](../glossary.md#integer-promotion),
[list initialization](initialization/list.md), name lookup *(referenced)*,
[constant evaluation](../glossary.md#constant-evaluation).

Inputs: clang-sema-decl, gcc-cp-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

## Syntax

    enum-def    = enum-head [ ":" type ] "{" [ enumerator { "," enumerator } [ "," ] ] "}"
    enum-opaque = enum-head ":" type ";" | ( "enum class" | "enum struct" ) name ";"
    enum-head   = "enum" [ "class" | "struct" ] name
    enumerator  = name [ "=" constant ]

## Rules

### ENUM-1 (required)

The first enumerator without `= value` is 0, and each later one without a
value is one more than the enumerator before it. A value must be a constant
expression, and it may use earlier enumerators. Several enumerators may
share a value. A program is rejected if two enumerators have the same name,
if a value is not a constant, or if an implicit value doesn't fit a fixed
underlying type (ENUM-4).

    enum E { A, B, C = 10, D, F = B + 1 };   // 0, 1, 10, 11, 2

Tests: declarations/enumerations/enumerator-values.cpp, declarations/enumerations/enumerator-refers-earlier.cpp, declarations/enumerations/duplicate-values.cpp, declarations/enumerations/enumerator-not-constant.cpp, declarations/enumerations/duplicate-enumerator.cpp, declarations/enumerations/implicit-value-overflow-scoped.cpp

### ENUM-2 (required)

The enumerators of an unscoped enumeration are declared in the enclosing
scope, and may also be named through the enumeration (`Color::Red`). Its
values convert implicitly to integers by integer promotion, so they work
in arithmetic and comparisons, including with another enumeration type.
Integers don't convert implicitly to the enumeration. A program is rejected
if an enumerator's name is already declared in the enclosing scope.

Tests: declarations/enumerations/unscoped-names-in-enclosing-scope.cpp, declarations/enumerations/unscoped-converts-to-int.cpp, declarations/enumerations/mixed-enum-arithmetic.cpp, declarations/enumerations/int-to-unscoped-rejected.cpp, declarations/enumerations/unscoped-name-conflict.cpp

### ENUM-3 (required)

The enumerators of a scoped enumeration are named only through the
enumeration, as `E::A`. Its values convert to and from integers only with
an explicit cast. Two values of the same scoped enumeration can be compared.
A program is rejected if it uses a scoped value in arithmetic, compares it
with an integer, or initializes an integer from it. Neither kind of
enumeration supports `++` or `--` without a user-declared operator
*(referenced)*.

    enum class E { A, B };
    E::A < E::B                  // fine
    E::A + 1                     // rejected
    static_cast<int>(E::B)       // 1

Tests: declarations/enumerations/scoped-names-scoped.cpp, declarations/enumerations/scoped-no-conversion.cpp, declarations/enumerations/scoped-no-arithmetic.cpp, declarations/enumerations/scoped-compare-same-type.cpp, declarations/enumerations/scoped-compare-int.cpp, declarations/enumerations/scoped-explicit-conversion.cpp, declarations/enumerations/increment-enum.cpp

### ENUM-4 (required)

A scoped enumeration without `: type` has the underlying type `int`. A
fixed underlying type, written after `:`, must be an integer type, and
every enumerator value must fit in it without narrowing. A program is
rejected if it doesn't.

    enum E : unsigned char { A = 255 };   // fine
    enum F : unsigned char { B = 300 };   // rejected
    enum class G { C = 1LL << 40 };       // rejected: int is too small

Tests: declarations/enumerations/scoped-default-underlying-int.cpp, declarations/enumerations/fixed-underlying.cpp, declarations/enumerations/scoped-value-too-large.cpp, declarations/enumerations/fixed-underlying-narrowing.cpp, declarations/enumerations/fixed-underlying-non-integral.cpp

### ENUM-5 (impl-defined)

The implementation chooses the underlying type of an unscoped enumeration
without `: type`. It is an integer type that holds every enumerator value,
and it is no wider than `int` unless a value needs more. When no standard
integer type holds every value, an implementation may use a wider type of
its own or reject the enumeration (question Q-0018).

Tests: declarations/enumerations/unscoped-underlying-holds-values.cpp, declarations/enumerations/unscoped-size-lp64.cpp, declarations/enumerations/implicit-value-overflow-unscoped.cpp, declarations/enumerations/unscoped-value-too-large.cpp

### ENUM-6 (undefined)

An enumeration with a fixed underlying type can hold any value of that
type. One without a fixed type can hold the values of the smallest group of
bits that represents all its enumerators. Converting a value outside that
range to the enumeration has undefined behavior (question Q-0017).

    enum E { A = 1, B = 4 };     // values fit in 3 bits: 0 to 7
    static_cast<E>(7)            // fine
    static_cast<E>(8)            // undefined

Tests: declarations/enumerations/cast-in-range.cpp, declarations/enumerations/cast-out-of-range.cpp, declarations/enumerations/cast-fixed-any-value.cpp

### ENUM-7 (required)

An enumeration may be declared before it is defined, without its
enumerators, if its underlying type is known: a scoped enumeration, or one
with `: type`. Later declarations must agree on scoped or unscoped and on
the underlying type. A program is rejected if it declares an unscoped
enumeration this way without `: type`, or if declarations disagree.

Tests: declarations/enumerations/opaque-declaration.cpp, declarations/enumerations/opaque-unscoped-without-type.cpp, declarations/enumerations/opaque-mismatch.cpp, declarations/enumerations/redeclare-scoped-as-unscoped.cpp

### ENUM-8 (required)

A variable of an enumeration type with a fixed underlying type, including
any scoped enumeration, may be initialized from an integer in braces, as
in `E e{5};`, provided the conversion to the underlying type doesn't
narrow. A program is rejected if the enumeration has no fixed type, or if
the value narrows.

Tests: declarations/enumerations/list-init-fixed.cpp, declarations/enumerations/list-init-no-fixed.cpp, declarations/enumerations/list-init-narrowing.cpp

### ENUM-9 (required)

`using enum E;` declares the enumerators of `E` in the current scope, so
they can be used without `E::`. A program is rejected if `E` isn't an
enumeration, or if one of its enumerator names is already declared there.

Tests: declarations/enumerations/using-enum.cpp, declarations/enumerations/using-enum-conflict.cpp, declarations/enumerations/using-enum-not-enum.cpp

### ENUM-10 (required)

Each enumeration is a distinct type, different from its underlying type and
from every other enumeration. An enumerator has the type of its
enumeration. Unary `+` on an unscoped value gives its promoted integer
type.

Tests: declarations/enumerations/distinct-types.cpp

## Evidence

Results are in `tests/RESULTS.md`. All seven compilers agree on 39 of 41
tests.

- **ENUM-5.** With `-fshort-enums`, Clang and GCC give `enum { S = 1 }`
  a 1-byte underlying type. By default it is the size of `int`. GCC uses a
  128-bit type when no standard type holds every value. See ENUM-D2.
- **ENUM-6.** Clang rejects the out-of-range conversion during constant
  evaluation ("outside the valid range of values [0, 7]"). See ENUM-D1.

## Divergences

### ENUM-D1: value outside an enumeration's range

Clang rejects `static_cast<E>(8)` during constant evaluation. GCC and EDG
accept it. **Provisional answer:** undefined. See Q-0017.

### ENUM-D2: no standard underlying type

GCC accepts an unscoped enumeration that needs more than 64 bits. Clang and
EDG reject it. **Provisional answer:** implementation-defined. See Q-0018.

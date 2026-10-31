# Fundamental types

A fundamental type is built into the language. This page lists the
fundamental types and the type specifiers that name them:

- `void` and the type of `nullptr`,
- `bool`,
- the character types,
- the signed and unsigned integer types,
- the floating-point types.

The page also gives the integer and floating-point conversion ranks, and
the size, alignment, and range of each type on the matrix targets.

Depends on: translation, execution, and behavior classes; declarators
and cv-qualifiers.

## Principles

:::{principle} TYP-P1
:rules: TYP-5 TYP-6 TYP-7

Each fundamental type is distinct, even where two types have the same
size and values. A program names an integer type by combining
specifiers, and the order of the specifiers does not matter. Each
specifier adds one property:

| Property | Specifiers |
|---|---|
| Sign | `signed`, `unsigned` |
| Size | `short`, `long`, `long long` |
| Base type | `int`, `char`, `double` |

A combination that asks for two values of one property, or a property
that the base type does not have, names no type.
:::

:::{principle} TYP-P2
:rules: TYP-4 TYP-8 TYP-9 TYP-11

Arithmetic needs to know which of two types is wider, so the integer
types and the floating-point types each form a ladder of ranks. A
signed and an unsigned type of the same size share a rank. Two types
can have the same size and still have different ranks, as `long` and
`long long` do on LP64. The values of an integer type follow from its
width, because every integer type uses two's complement. `bool` has two
values only.
:::

:::{principle} TYP-P3
:rules: TYP-1 TYP-2 TYP-3 TYP-10 TYP-12 TYP-13

The language fixes the order of the sizes, and the target fixes the
sizes themselves. The target also picks the signedness of `char` and
the underlying types of the wide character types. The two matrix
targets differ in these choices:

- the sizes of `long` and of pointers,
- the size of `long double`,
- the alignment of 8-byte types.

A program that depends on a size can check the size with `static_assert`
on `sizeof`.
:::

## Constraints

:::{rule} TYP-5
:kind: required
:tests: foundations/types/distinct-character-types.cpp foundations/types/distinct-same-size.cpp foundations/types/distinct-same-size-ilp32.cpp

Each fundamental type is a distinct type. These pairs are different
types, whatever their sizes:

- `char`, `signed char`, and `unsigned char`,
- `wchar_t` and its underlying type (TYP-3),
- `char8_t`, `char16_t`, `char32_t`, and their underlying types,
- `int`, `long`, and `long long`, and the unsigned type of each.
:::

:::{rule} TYP-6
:kind: required
:tests: foundations/types/specifiers-equivalent.cpp foundations/types/specifier-long-double.cpp

An integer type is named by specifiers in any order. These rules give
the type:

- `short`, `long`, or `long long` gives the size, and none gives the
  size of `int`.
- `unsigned` gives the unsigned type. `signed` or no sign gives the
  signed type.
- `int` can be added to any of these specifiers and changes nothing.
  `signed` or `unsigned` alone means `int` of that sign.

`signed char` and `unsigned char` name those types. `long double` names
the widest floating-point type, with the specifiers in either order.
:::

:::{rule} TYP-7
:kind: reject
:tests: foundations/types/specifier-long-short.cpp foundations/types/specifier-long-long-long.cpp foundations/types/specifier-signed-unsigned.cpp foundations/types/specifier-unsigned-float.cpp foundations/types/specifier-long-float.cpp foundations/types/specifier-short-char.cpp foundations/types/specifier-unsigned-bool.cpp foundations/types/specifier-signed-wchar.cpp foundations/types/specifier-unsigned-char8.cpp foundations/types/specifier-short-double.cpp foundations/types/specifier-long-char.cpp foundations/types/specifier-int-char.cpp foundations/types/specifier-unsigned-char16.cpp foundations/types/specifier-signed-char32.cpp foundations/types/specifier-short-float.cpp foundations/types/specifier-long-bool.cpp foundations/types/specifier-long-long-double.cpp

A program must not combine type specifiers in these ways:

- two sizes, such as `long short`, or `long` three times,
- both `signed` and `unsigned`,
- a size or a sign with `bool`, `wchar_t`, `char8_t`, `char16_t`,
  `char32_t`, or `float`,
- a size with `char`, or `short` or `long long` with `double`,
- two base types, such as `int char`.
:::

:::{rule} TYP-8
:kind: required
:tests: foundations/types/rank-long-unsigned-lp64.cpp foundations/types/rank-long-unsigned-ilp32.cpp foundations/types/rank-promotions.cpp

The integer conversion ranks are in this order, from lowest to highest:

1. `bool`,
2. `char`, `signed char`, and `unsigned char`,
3. `short`,
4. `int`,
5. `long`,
6. `long long`.

An unsigned type has the rank of its signed type. `char8_t`,
`char16_t`, `char32_t`, and `wchar_t` have the rank of their underlying
types (TYP-3).
:::

:::{rule} TYP-9
:kind: required
:tests: foundations/types/rank-floating.cpp

The floating-point conversion ranks are in the order `float`, `double`,
`long double`. An arithmetic operation between an integer and a
floating-point type has the floating-point type, whatever the integer's
rank.
:::

## Runtime Behavior

:::{rule} TYP-4
:kind: required
:tests: foundations/types/integer-ranges-lp64.cpp foundations/types/integer-ranges-ilp32.cpp foundations/types/int-max-overflow.cpp

Every integer type uses two's complement. For a type whose size in
TYP-1 is `B` bytes, let `N` be `8 * B`. The table gives the bounds:

| Kind | Lowest | Highest |
|---|---|---|
| signed | -2<sup>N-1</sup> | 2<sup>N-1</sup> - 1 |
| unsigned | 0 | 2<sup>N</sup> - 1 |
:::

:::{rule} TYP-11
:kind: required
:tests: foundations/types/bool-values.cpp

`bool` has the two values `false` and `true`. A conversion to `bool`
gives `false` for zero and `true` for any other value. In arithmetic,
`false` is 0 and `true` is 1.
:::

:::{rule} TYP-13
:kind: required
:tests: foundations/types/unicode-characters-unsigned.cpp

`char8_t` has the size and values of `unsigned char`. `char16_t` and
`char32_t` are unsigned on every matrix target.
:::

## Implementation Requirements

:::{rule} TYP-1
:kind: impl-defined
:tests: foundations/types/size-integers-lp64.cpp foundations/types/size-integers-ilp32.cpp foundations/types/size-characters-lp64.cpp foundations/types/size-characters-ilp32.cpp foundations/types/size-floating-lp64.cpp foundations/types/size-floating-ilp32.cpp foundations/types/size-pointers-lp64.cpp foundations/types/size-pointers-ilp32.cpp foundations/types/size-result-type-lp64.cpp foundations/types/size-result-type-ilp32.cpp

The size of each fundamental type is implementation-defined, except
that `char`, `signed char`, `unsigned char`, and `char8_t` have size 1.
The sizes do not decrease along the order of TYP-8. The table gives the
sizes in bytes on the matrix targets:

| Type | x86-64 Linux | `-m32` |
|---|---|---|
| `bool` | 1 | 1 |
| `short` | 2 | 2 |
| `int` | 4 | 4 |
| `long` | 8 | 4 |
| `long long` | 8 | 8 |
| `wchar_t` | 4 | 4 |
| `char16_t` | 2 | 2 |
| `char32_t` | 4 | 4 |
| `float` | 4 | 4 |
| `double` | 8 | 8 |
| `long double` | 16 | 12 |
| a pointer, and the type of `nullptr` | 8 | 4 |
| the type of `sizeof` and `alignof` | `unsigned long` | `unsigned int` |
:::

:::{rule} TYP-2
:kind: impl-defined
:tests: foundations/types/align-lp64.cpp foundations/types/align-ilp32.cpp foundations/types/align-member-ilp32.cpp foundations/types/align-long-double-member-ilp32.cpp

The alignment of each fundamental type is implementation-defined. On
x86-64 Linux each type's alignment equals its size. On `-m32` the types
in this table have a smaller alignment, also as a member of a class:

| Type | `-m32`, GCC and Clang | `-m32`, EDG |
|---|---|---|
| `long long` | 4 | 8 ({question}`Q-0120`) |
| `double` | 4 | 4 |
| `long double` | 4 | 4 |
:::

:::{rule} TYP-3
:kind: impl-defined
:tests: foundations/types/char-signed.cpp foundations/types/char-signed-ilp32.cpp foundations/types/wchar-signed.cpp foundations/types/unicode-characters-unsigned.cpp

Whether `char` is signed is implementation-defined. The underlying types
of `wchar_t`, `char16_t`, and `char32_t` are implementation-defined. The
table gives the choices on both matrix targets:

| Type | Signed | Underlying type |
|---|---|---|
| `char` | yes | none (TYP-5) |
| `wchar_t` | yes | `int` |
| `char16_t` | no | `unsigned short` |
| `char32_t` | no | `unsigned int` |
:::

:::{rule} TYP-10
:kind: impl-defined
:tests: foundations/types/float-precision.cpp foundations/types/long-double-precision.cpp foundations/types/long-double-precision-ilp32.cpp

The formats of the floating-point types are implementation-defined. On
both matrix targets, the table gives the bits of precision:

| Type | Bits of precision |
|---|---|
| `float` | 24 |
| `double` | 53 |
| `long double` | 64 |
:::

:::{rule} TYP-12
:kind: impl-defined
:tests: foundations/types/align-member-ilp32.cpp foundations/types/align-long-double-member-ilp32.cpp

The padding between the members of a class follows the alignment of
each member (TYP-2). On `-m32`, a `char` followed by a `double` takes 12
bytes, and a `char` followed by a `long double` takes 16 bytes. A `char`
followed by a `long long` takes 12 bytes with GCC and Clang, and 16
with EDG.
:::

## Examples

```cpp
static_assert(sizeof(int) == 4);        // true on both matrix targets
unsigned long size = sizeof(long);      // 8 on x86-64, 4 on -m32
long unsigned int same_type = 0;        // the type unsigned long
long short broken = 0;                  // rejected: two sizes
auto mixed = 1L + 1u;                   // long on x86-64,
                                        // unsigned long on -m32
static_assert(char(200) < 0);           // char is signed on both targets
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on
every test. EDG skips the `-m32` tests.

- TYP-1, TYP-2, TYP-10, TYP-12: the values come from probes on both
  targets. Clang's target description for x86 gives the same sizes and
  alignments, and the 96-bit `long double` with 32-bit alignment on
  i386.
- TYP-3: Clang's target description sets the signedness of `char` and
  the underlying types of `wchar_t`, `char16_t`, and `char32_t` for each
  target. `char8_t` always takes the rank of `unsigned char` (TYP-13).
- TYP-7: Clang reports a sign on `wchar_t` through an error that a
  warning flag controls (`-Wsigned-unsigned-wchar`). GCC's message for
  the same error names `typeof`.
- TYP-8: Clang gives each character type the rank of its underlying
  type. A program sees these ranks only through promotion (CONV-1).

Inputs: clang-targets-x86, clang-targetinfo, clang-ast-context,
edg-targ-def.

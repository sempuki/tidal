# List initialization and aggregates

Initialization with a braced list, as in `int x{1};` or `S s = {1, 2};`.
Braces forbid narrowing conversions, fill arrays and aggregates member by
member, and allow designated initializers. Character arrays may be
initialized from string literals.

An *aggregate* here is an array, or a class with only public non-static data
members and no user-declared constructors *(classes referenced)*. Rules are
prefixed `LIST`.

Depends on: [declarators](../declarators.md), [storage duration](../storage-linkage.md),
[value category](../../glossary.md#value-category), conversions *(referenced)*,
classes *(referenced)*, [constant evaluation](../../glossary.md#constant-evaluation).

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).

## Syntax

    braced = "{" [ clause { "," clause } [ "," ] ] "}"
    clause = expression | braced | designator "=" ( expression | braced )
           | designator braced
    designator = "." name

## Rules

### LIST-1 (required)

A scalar may be initialized from braces holding one expression, or from
empty braces (INIT-2). A program is rejected if the braces hold more than
one expression.

Tests: declarations/initialization/list/scalar-braces.cpp, declarations/initialization/list/scalar-braces-two-values.cpp

### LIST-2 (reject)

Inside braces, a program is rejected if a value must undergo a *narrowing*
conversion, one that may lose information:

- floating-point to integer, always, even for `1.0`;
- integer to floating-point, unless the value is a constant that the
  target type represents exactly;
- `long double` to `double` or `float`, or `double` to `float`,
  unless the value is a constant within the target's range;
- integer to a narrower or differently signed integer, unless the value is
  a constant that fits;
- pointer to `bool`.

The same conversions are allowed in parentheses (INIT-4).

    char c{65};         // fine: 65 fits
    char d{300};        // rejected
    float f{0.1};       // fine: in range
    float g{16777217};  // rejected: not exact in float

Tests: declarations/initialization/list/narrowing-float-to-int.cpp, declarations/initialization/list/narrowing-float-to-int-exact.cpp, declarations/initialization/list/narrowing-constant-out-of-range.cpp, declarations/initialization/list/narrowing-constant-fits.cpp, declarations/initialization/list/narrowing-negative-to-unsigned.cpp, declarations/initialization/list/narrowing-nonconstant-int.cpp, declarations/initialization/list/narrowing-double-to-float-in-range.cpp, declarations/initialization/list/narrowing-double-to-float-out-of-range.cpp, declarations/initialization/list/narrowing-double-to-float-nonconstant.cpp, declarations/initialization/list/narrowing-int-to-float-inexact.cpp, declarations/initialization/list/narrowing-int-to-float-exact.cpp, declarations/initialization/list/narrowing-pointer-to-bool.cpp, declarations/initialization/list/narrowing-in-aggregate.cpp, declarations/initialization/list/narrowing-not-in-parens.cpp

### LIST-3 (required)

An aggregate is initialized member by member, in declaration order, from a
braced list or, without braces around the values, from parentheses. A
member with no value in the list takes its default member initializer if it
has one, and otherwise becomes zero. A program is rejected if the list has
more values than the aggregate has members.

    struct S { int a; double b; int c = 7; };
    S s{1};       // a = 1, b = 0.0, c = 7
    S t(1, 2.5);  // a = 1, b = 2.5, c = 7

Tests: declarations/initialization/list/aggregate-members-in-order.cpp, declarations/initialization/list/aggregate-missing-members-zero.cpp, declarations/initialization/list/aggregate-default-member-initializer.cpp, declarations/initialization/list/aggregate-too-many.cpp, declarations/initialization/list/aggregate-paren-init.cpp

### LIST-4 (required)

Inner braces around a member that is itself an array or aggregate may be
left out. The values then fill that member's elements in order before
moving to the next member.

    int a[2][2] = {5, 6, 7, 8};      // {{5, 6}, {7, 8}}
    struct A { int v[2]; int w; };
    A s = {5, 6, 7};                 // v = {5, 6}, w = 7

Tests: declarations/initialization/list/brace-elision.cpp

### LIST-5 (required)

A braced list for a class aggregate may name members with `.member =`
or `.member {...}`. Named members must appear in declaration order, and
members left out are initialized as in LIST-3. A program is rejected if
designators are out of order, repeat a member, are mixed with values
without designators, chain members (`.a.b`), name array elements
(`[1] =`), or appear in the initializer of a scalar.

    struct S { int a; int b; int c; };
    S s{.a = 1, .c = 3};   // b = 0
    S t{.b = 2, .a = 1};   // rejected

Tests: declarations/initialization/list/designated.cpp, declarations/initialization/list/designated-nested-braces.cpp, declarations/initialization/list/designated-out-of-order.cpp, declarations/initialization/list/designated-mixed.cpp, declarations/initialization/list/designated-nested.cpp, declarations/initialization/list/designated-array.cpp, declarations/initialization/list/designated-repeated.cpp, declarations/initialization/list/designator-on-scalar.cpp

### LIST-6 (required)

A character array may be initialized from a string literal, optionally in
braces. The array's bound, if left out, counts the terminating null
character. The literal's character type must match the element type,
except that a UTF-8 literal (`u8"..."`) may also initialize a `char` or
`unsigned char` array. A program is rejected if the literal, including its
null character, is longer than the array, or if the types don't match.

    char a[] = "ab";        // char[3]
    char b[2] = "ab";       // rejected
    char c[] = u8"x";       // fine
    char8_t d[] = "x";      // rejected

Tests: declarations/initialization/list/string-literal-array.cpp, declarations/initialization/list/string-literal-u8-to-char8.cpp, declarations/initialization/list/string-literal-too-long.cpp, declarations/initialization/list/string-literal-wrong-type.cpp, declarations/initialization/list/string-literal-u8-to-char.cpp, declarations/initialization/list/string-literal-plain-to-char8.cpp, declarations/initialization/list/string-literal-u8-to-signed-char.cpp

### LIST-7 (required)

The values in a braced list are evaluated left to right, each one
completely before the next begins.

Tests: declarations/initialization/list/braced-order.cpp

## Evidence

All seven compilers agree on all 38 tests. Results are in
`tests/RESULTS.md`.

- **LIST-5.** Clang accepts array designators, chained designators, and
  mixing designated and plain values as C99 extensions, and rejects them
  only under `-pedantic-errors`. GCC and EDG reject them.

## Divergences

None.

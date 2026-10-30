# List initialization and aggregates

List-initialization uses a braced list, as in `int x{1};` or
`S s = {1, 2};`. This page covers scalars, aggregates, and character
arrays.

An *aggregate* is an array, or a class that has only public non-static
data members and no user-declared constructors *(classes referenced)*.

## Syntax

```{eval-rst}
.. productionlist:: init_list
   braced: "{" [`clause` ("," `clause`)* [","]] "}"
   clause: `expression` | `braced`
         : | `designator` "=" (`expression` | `braced`)
         : | `designator` `braced`
   designator: "." `name`
```

## Legality Rules

:::{rule} LIST-1
:kind: required
:tests: declarations/initialization/list/scalar-braces.cpp declarations/initialization/list/scalar-braces-two-values.cpp

A scalar can be initialized from braces that hold one expression, or from
empty braces (INIT-2). The braces must not hold more than one expression.
:::

:::{rule} LIST-2
:kind: reject
:tests: declarations/initialization/list/narrowing-float-to-int.cpp declarations/initialization/list/narrowing-float-to-int-exact.cpp declarations/initialization/list/narrowing-constant-out-of-range.cpp declarations/initialization/list/narrowing-constant-fits.cpp declarations/initialization/list/narrowing-negative-to-unsigned.cpp declarations/initialization/list/narrowing-nonconstant-int.cpp declarations/initialization/list/narrowing-double-to-float-in-range.cpp declarations/initialization/list/narrowing-double-to-float-out-of-range.cpp declarations/initialization/list/narrowing-double-to-float-nonconstant.cpp declarations/initialization/list/narrowing-int-to-float-inexact.cpp declarations/initialization/list/narrowing-int-to-float-exact.cpp declarations/initialization/list/narrowing-pointer-to-bool.cpp declarations/initialization/list/narrowing-in-aggregate.cpp declarations/initialization/list/narrowing-not-in-parens.cpp

In braces, a value must not go through a *narrowing* conversion. These
conversions are narrowing:

- floating-point to integer, for all values;
- integer to floating-point, unless the value is a constant that the
  target type holds exactly;
- `long double` to `double` or `float`, or `double` to `float`, unless
  the value is a constant in the range of the target type;
- integer to an integer type that cannot hold all its values, unless the
  value is a constant that fits;
- pointer to `bool`.

The same conversions are permitted in parentheses (INIT-4).
:::

:::{rule} LIST-3
:kind: required
:tests: declarations/initialization/list/aggregate-members-in-order.cpp declarations/initialization/list/aggregate-missing-members-zero.cpp declarations/initialization/list/aggregate-default-member-initializer.cpp declarations/initialization/list/aggregate-too-many.cpp declarations/initialization/list/aggregate-paren-init.cpp

An aggregate is initialized member by member in the order of declaration.
The values can be in braces or in parentheses. A member without a value
gets its default member initializer, or zero if it has none. The list
must not have more values than the aggregate has members.
:::

:::{rule} LIST-4
:kind: required
:tests: declarations/initialization/list/brace-elision.cpp

The inner braces for a member that is an array or an aggregate can be
omitted. The values then fill the elements of that member in order. The
next values go to the next member.
:::

:::{rule} LIST-5
:kind: required
:tests: declarations/initialization/list/designated.cpp declarations/initialization/list/designated-nested-braces.cpp declarations/initialization/list/designated-out-of-order.cpp declarations/initialization/list/designated-mixed.cpp declarations/initialization/list/designated-nested.cpp declarations/initialization/list/designated-array.cpp declarations/initialization/list/designated-repeated.cpp declarations/initialization/list/designator-on-scalar.cpp

A braced list for a class aggregate can name members with `.member =` or
`.member {...}`. Members without a designator get values as LIST-3 gives.
The designators must obey these conditions:

- They must be in the order of member declaration.
- Each member must occur one time only.
- All values in the list must have a designator, or none.
- A designator must name one member, without a chain such as `.a.b`.
- A designator must not name an array element, such as `[1] =`.
- The initializer of a scalar must not use a designator.
:::

:::{rule} LIST-6
:kind: required
:tests: declarations/initialization/list/string-literal-array.cpp declarations/initialization/list/string-literal-u8-to-char8.cpp declarations/initialization/list/string-literal-too-long.cpp declarations/initialization/list/string-literal-wrong-type.cpp declarations/initialization/list/string-literal-u8-to-char.cpp declarations/initialization/list/string-literal-plain-to-char8.cpp declarations/initialization/list/string-literal-u8-to-signed-char.cpp

A character array can be initialized from a string literal, with or
without braces. These conditions apply:

- If the bound is omitted, it counts the terminating null character.
- The literal, with its null character, must fit in the array.
- The character type of the literal must match the element type.
- A UTF-8 literal (`u8"..."`) can also initialize a `char` or
  `unsigned char` array.
:::

## Dynamic Semantics

:::{rule} LIST-7
:kind: required
:tests: declarations/initialization/list/braced-order.cpp

The values in a braced list are evaluated from left to right. Each value
is evaluated completely before the next value starts.
:::

## Examples

```cpp
int a{5};                 // 5
int b{1, 2};              // rejected

char c{65};               // 65 fits in char
char d{300};              // rejected: narrowing
float f{0.1};             // in range of float
float g{16777217};        // rejected: not exact in float
struct P { int v; };
P p(2.5);                 // v == 2: parentheses permit narrowing

struct S { int a; double b; int c = 7; };
S s{1};                   // a == 1, b == 0.0, c == 7
S t(1, 2.5);              // a == 1, b == 2.5, c == 7

int m[2][2] = {5, 6, 7, 8};   // {{5, 6}, {7, 8}}
struct A { int v[2]; int w; };
A e = {5, 6, 7};              // v == {5, 6}, w == 7

struct D { int a; int b; int c; };
D x{.a = 1, .c = 3};      // b == 0
D y{.b = 2, .a = 1};      // rejected: out of order

char h[] = "ab";          // char[3]
char i[2] = "ab";         // rejected: too long
char j[] = u8"x";         // valid
char8_t k[] = "x";        // rejected: wrong character type
```

## Evidence

All seven compilers agree on all 38 tests. The results are in
`tests/RESULTS.md`.

- **LIST-5.** Clang accepts array designators, chained designators, and a
  mix of designated and plain values as C99 extensions. It rejects them
  only under `-pedantic-errors`. GCC and EDG reject them.

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).

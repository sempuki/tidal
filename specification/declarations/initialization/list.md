# List initialization and aggregates

List-initialization uses a braced list, as in `int x{1};` or
`S s = {1, 2};`. This page covers:

- scalars,
- aggregates,
- character arrays.

An *aggregate* is an array, or a class with these properties (the
{doc}`classes chapter </classes/index>` defines the terms):

- no user-declared or inherited constructors,
- no private or protected direct non-static data members,
- no virtual functions,
- base classes that are all public and not virtual.

## Principles

:::{principle} LIST-P1
:rules: LIST-1 LIST-11 LIST-2 LIST-7

Braces are the careful form of initialization. A value in braces must
not lose information on its way to the target type, so a
{term}`narrowing conversion` is rejected, unless the value is a constant that fits. The
values are evaluated from left to right, each one complete before the
next. A scalar takes one value, and no nesting.
:::

:::{principle} LIST-P2
:rules: LIST-3 LIST-12 LIST-4 LIST-13 LIST-5 LIST-14 LIST-8 LIST-16 LIST-9 LIST-10

An aggregate is a class or array with nothing to hide, so a braced list
initializes its parts directly, in the order they are declared, bases
first.

- Inner braces can be left out where the values fill a member without
  ambiguity.
- A member with no value in the list gets its default member initializer
  or an empty list.
- A designator names the member that a value goes to, in declaration
  order.
- A list of one value of the aggregate's own type copies that value.
- Parentheses give the same initialization with one value per part and
  no elision.
:::

:::{principle} LIST-P3
:rules: LIST-6 LIST-15

A character array is an aggregate of characters, so a string literal
initializes the array, with the terminating null included. A literal
initializes an array of its own character type. A plain or UTF-8
literal also initializes arrays of some other one-byte character types.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   braced_init: "{" [`init_clause` ["..."] {"," `init_clause` ["..."]} [","]] "}"
   init_clause: `assignment_expr` | `braced_init`
              : | `designator` "=" (`assignment_expr` | `braced_init`)
              : | `designator` `braced_init`
   designator: "." `identifier`
```

An `init_clause` with `...` is a {term}`pack expansion` ({rule}`PACK-13`).

## Constraints

:::{rule} LIST-1
:kind: required
:tests: declarations/initialization/list/scalar-braces.cpp declarations/initialization/list/scalar-braces-pointer-and-return.cpp

A scalar can be initialized from braces that hold one expression, or
from empty braces (INIT-2). LIST-11 lists what is not valid.
:::

:::{rule} LIST-11
:kind: reject
:tests: declarations/initialization/list/scalar-braces-two-values.cpp declarations/initialization/list/scalar-nested-braces.cpp

The braces for a scalar must not hold more than one expression, and that
expression must not itself be a braced list.
:::

:::{rule} LIST-2
:kind: reject
:tests: declarations/initialization/list/narrowing-float-to-int.cpp declarations/initialization/list/narrowing-float-to-int-exact.cpp declarations/initialization/list/narrowing-constant-out-of-range.cpp declarations/initialization/list/narrowing-constant-fits.cpp declarations/initialization/list/narrowing-negative-to-unsigned.cpp declarations/initialization/list/narrowing-nonconstant-int.cpp declarations/initialization/list/narrowing-double-to-float-in-range.cpp declarations/initialization/list/narrowing-double-to-float-out-of-range.cpp declarations/initialization/list/narrowing-double-to-float-nonconstant.cpp declarations/initialization/list/narrowing-int-to-float-inexact.cpp declarations/initialization/list/narrowing-int-to-float-exact.cpp declarations/initialization/list/narrowing-pointer-to-bool.cpp declarations/initialization/list/narrowing-in-aggregate.cpp declarations/initialization/list/narrowing-not-in-parens.cpp declarations/initialization/list/narrowing-bit-field-fits.cpp declarations/initialization/list/narrowing-member-pointer-to-bool.cpp declarations/initialization/list/narrowing-unscoped-enum.cpp declarations/initialization/list/narrowing-unscoped-enum-values-fit.cpp declarations/initialization/list/narrowing-nonconstant-long-ilp32.cpp overloading/conversion-sequences/chosen-narrowing.cpp declarations/initialization/list/narrowing-integer-to-bool.cpp declarations/initialization/list/narrowing-floating-to-bool.cpp declarations/initialization/list/narrowing-constant-to-bool-out-of-range.cpp declarations/initialization/list/narrowing-constant-fits-bool.cpp templates/concepts/requirement-narrowing-in-braces-false.cpp

In braces, a value must not go through a {term}`narrowing conversion`
({rule}`CONV-21`). The same conversions are permitted in parentheses
(INIT-4).
:::

:::{rule} LIST-3
:kind: required
:tests: declarations/initialization/list/aggregate-members-in-order.cpp declarations/initialization/list/aggregate-paren-init.cpp declarations/initialization/list/aggregate-with-base.cpp declarations/initialization/list/aggregate-member-converting-constructor.cpp

An aggregate is initialized in the order of declaration. Its base
classes are initialized first, then its members. The values can be in
braces or in parentheses. LIST-12 lists what is not valid.
:::

:::{rule} LIST-12
:kind: reject
:tests: declarations/initialization/list/aggregate-too-many.cpp declarations/initialization/list/missing-reference-member.cpp templates/concepts/requirement-too-many-values-nested-aggregate.cpp statements/if/constexpr-requirement-too-many-values-discards-binding.cpp

The list for an aggregate must not have more values than the aggregate
has bases and members. A reference member without a default member
initializer must have a value in the list.
:::

:::{rule} LIST-4
:kind: required
:tests: declarations/initialization/list/brace-elision.cpp declarations/initialization/list/no-elision-when-direct.cpp declarations/initialization/list/empty-base-braces-ok.cpp templates/concepts/requirement-too-many-values-nested-aggregate.cpp

The inner braces for a member that is an array or an aggregate can be
omitted. The values then fill the elements of that member in order. The
next values go to the next member. The inner braces are not omitted for
a value that can initialize the member directly, such as an object of
the member's type. LIST-13 lists what is not valid.
:::

:::{rule} LIST-13
:kind: reject
:tests: declarations/initialization/list/empty-base-no-elision.cpp declarations/initialization/list/empty-member-no-elision.cpp

A member that is an empty aggregate, such as an empty base, must have
its own braces if later values follow.
:::

:::{rule} LIST-5
:kind: required
:tests: declarations/initialization/list/designated.cpp declarations/initialization/list/designated-nested-braces.cpp declarations/initialization/list/union-designator.cpp

A braced list for a class aggregate can name members with `.member =` or
`.member {...}`. Members without a designator get values as LIST-3
gives. For a union, one designator selects the member to initialize.
Without a designator, the first member of the union is initialized.
LIST-14 lists what is not valid.
:::

:::{rule} LIST-14
:kind: reject
:tests: declarations/initialization/list/designated-out-of-order.cpp declarations/initialization/list/designated-mixed.cpp declarations/initialization/list/designated-nested.cpp declarations/initialization/list/designated-array.cpp declarations/initialization/list/designated-repeated.cpp declarations/initialization/list/designator-on-scalar.cpp overloading/conversion-sequences/chosen-designator-order.cpp classes/unions/anonymous-union-two-designators.cpp

The designators in a braced list must obey these conditions:

- They must be in the order of member declaration.
- Each member must occur one time only.
- At most one designator can name a member of one anonymous union
  ({rule}`UNI-7`).
- All values in the list must have a designator, or none.
- A designator must name one member, without a chain such as `.a.b`.
- A designator must not name an array element, such as `[1] =`.
- The initializer of a scalar must not use a designator.
:::

:::{rule} LIST-6
:kind: required
:tests: declarations/initialization/list/string-literal-array.cpp declarations/initialization/list/string-literal-u8-to-char8.cpp declarations/initialization/list/string-literal-u8-to-char.cpp declarations/initialization/list/char-array-rest-zero.cpp declarations/initialization/list/string-literal-plain-to-signed-unsigned.cpp declarations/initialization/list/string-literal-reference-temporary.cpp

A character array can be initialized from a string literal, with or
without braces, if the table gives the array's element type for the
literal.

| Literal | Element types |
|---|---|
| Plain (`"..."`) | `char`, `signed char`, `unsigned char` |
| UTF-8 (`u8"..."`) | `char8_t`, `char`, `unsigned char` |
| UTF-16 (`u"..."`) | `char16_t` |
| UTF-32 (`U"..."`) | `char32_t` |
| Wide (`L"..."`) | `wchar_t` |

These conditions apply:

- If the bound is omitted, the bound counts the terminating null
  character.
- The elements after the literal and its null character become zero.

The same holds for the temporary array of a reference ({rule}`REF-14`,
{question}`Q-0166`). LIST-15 lists what is not valid.
:::

:::{rule} LIST-15
:kind: reject
:tests: declarations/initialization/list/string-literal-too-long.cpp declarations/initialization/list/string-literal-wrong-type.cpp declarations/initialization/list/string-literal-plain-to-char8.cpp declarations/initialization/list/string-literal-u8-to-signed-char.cpp declarations/initialization/list/string-literal-wide-to-char32.cpp

When a string literal initializes a character array, these conditions
apply:

- The literal, with its null character, must fit in the array.
- The element type must be one that LIST-6 gives for the kind of the
  literal.
:::

:::{rule} LIST-9
:kind: reject
:tests: declarations/initialization/list/paren-nested-ok.cpp declarations/initialization/list/paren-no-brace-elision.cpp

In parenthesized aggregate initialization, each value initializes one
whole member or base. The values must not be split across the members of
a nested aggregate ({question}`Q-0043`).
:::

:::{rule} LIST-16
:kind: reject
:tests: declarations/initialization/list/omitted-member-explicit-default.cpp declarations/initialization/list/omitted-member-explicit-default-designated.cpp

An `explicit` {term}`default constructor` cannot initialize a member that has no
value in the list (LIST-8). Such a member must have a value in the list.
:::

## Runtime Behavior

:::{rule} LIST-7
:kind: required
:tests: declarations/initialization/list/braced-order.cpp foundations/evaluation-order/parenthesized-aggregate-order.cpp

The values in a braced list, and in a parenthesized list that
initializes an aggregate, are evaluated from left to right. Each value
is evaluated completely before the next value starts.
:::

:::{rule} LIST-8
:kind: required
:tests: declarations/initialization/list/aggregate-missing-members-zero.cpp declarations/initialization/list/aggregate-default-member-initializer.cpp declarations/initialization/list/missing-class-member-default.cpp

A member without a value in the list is initialized from its default
member initializer. If the member has no default member initializer, the
member is initialized from an empty braced list. A scalar member becomes
zero. A member of class type gets its default constructor
({rule}`CTOR-2`). LIST-16 lists what is not valid.
:::

:::{rule} LIST-10
:kind: required
:tests: declarations/initialization/list/single-same-class-copies.cpp declarations/initialization/list/single-same-class-argument.cpp

If a braced list for an aggregate has one element of the same class
type, the aggregate is copied from that element. {rule}`ICS-18` gives
the conversion sequence of such a list.
:::

## Examples

% tidal-example: reject
```cpp
int a{5};                 // 5
int b{1, 2};              // rejected

char c{65};               // 65 fits in char
char d{300};              // rejected: narrowing
long l = 1;
int il{l};                // rejected on x86-64; valid on -m32,
                          // where long and int have the same size
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

struct Swapped { int c; int a; };
int pick(D, long);
int pick(Swapped, int);
int z = pick({.a = 1, .c = 3}, 0);
                          // rejected: the int argument chooses
                          // pick(Swapped, int), and for Swapped the
                          // designators are out of order (ICS-8)

char h[] = "ab";          // char[3]
char i[2] = "ab";         // rejected: too long
char j[] = u8"x";         // valid
char8_t k[] = "x";        // rejected: wrong character type
unsigned char n[] = "x";  // valid
signed char o[] = u8"x";  // rejected: wrong character type
```

## Evidence

All eight compilers agree on 71 of 75 tests. The results are in
`tests/RESULTS.md`.

### Divergences

- {question}`Q-0042` (LIST-2, CONV-21; settled on LIST-P1): whether a
  bit-field whose values all fit the target narrows. It does not, as
  Clang 22 and GCC 16 say. EDG 7.0, Clang 18, and GCC 14 reject the
  initialization as narrowing.
- {question}`Q-0331` (LIST-2, CONV-21; open): which values an unscoped
  enumeration without a fixed type has for narrowing. It has the values
  that {rule}`ENUM-22` gives, as GCC uses, so an enumeration with the
  enumerators 0 and 100 initializes an `unsigned char`. Clang and EDG
  use every value of the underlying type and reject the program.
- {question}`Q-0043` (LIST-9; settled on LIST-P2): whether
  parenthesized aggregate initialization allows brace elision. It does
  not, as Clang and GCC say. EDG 7.0 accepts the initialization.
- {question}`Q-0166` (LIST-6, REF-14; settled on {principle}`REF-P3`):
  whether one braced string literal can initialize the temporary array
  of a reference. It can, as Clang and EDG say. GCC rejects the literal.
- {question}`Q-0167` (LIST-6; settled on {principle}`ICS-P5`): whether
  one braced plain literal prefers a reference to a `char` array over
  one to an `unsigned char` array. It does not, and the call is
  ambiguous, as Clang and EDG say. GCC chooses `char`.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `narrowing-bit-field-fits.cpp` | LIST-2, CONV-21 | accept | Clang 22: accept; Clang 18: reject | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0042` |
| `narrowing-unscoped-enum-values-fit.cpp` | LIST-2, CONV-21 | accept | reject | accept | reject | {question}`Q-0331` |
| `paren-no-brace-elision.cpp` | LIST-9 | reject | reject | reject | accept | {question}`Q-0043` |
| `string-literal-reference-temporary.cpp` | LIST-6, REF-14 | accept | accept | reject | accept | {question}`Q-0166` |

### Notes

- LIST-2, CONV-21: the table of {rule}`CONV-21` says "integral type",
  so `bool` is a target of the integral rows. Every compiler rejects
  `bool b{i}` for an `int` variable `i`, `bool b{2}`, and `bool b{1.0}`,
  and accepts `bool b{1}`.
- LIST-2: with `-m32`, `long` and `int` have the same size, so a `long`
  value that is not constant initializes an `int` in braces. Without
  `-m32`, every compiler rejects the same initialization.
- LIST-14: Clang accepts array designators, chained designators, and a
  mix of designated and plain values as C99 extensions, and rejects
  them only under `-pedantic-errors`. GCC and EDG reject them.
- LIST-6: every compiler accepts each pair of literal and element type
  in the table, for an array variable, with and without braces. Every
  compiler rejects the other pairs of the five literal kinds and the
  seven character types.
- LIST-8: under `-Wextra`, Clang and GCC warn about a list that leaves
  out a member with no default member initializer
  (`-Wmissing-field-initializers`, and in Clang
  `-Wmissing-designated-field-initializers` for a designated list).
  They warn for positional and designated lists, and for a member of
  class type whose own members all have defaults. They do not warn for
  empty braces `{}`, or for a member left out that has a default member
  initializer.
- LIST-4: under `-Wall`, Clang warns when a list omits the braces of a
  member that is an aggregate (`-Wmissing-braces`). GCC does not warn.

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).

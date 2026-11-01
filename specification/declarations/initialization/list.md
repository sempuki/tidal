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
not lose information on its way to the target type, so a narrowing
conversion is rejected, unless the value is a constant that fits. The
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
   braced_init: "{" [`init_clause` {"," `init_clause`} [","]] "}"
   init_clause: `assignment_expr` | `braced_init`
              : | `designator` "=" (`assignment_expr` | `braced_init`)
              : | `designator` `braced_init`
   designator: "." `identifier`
```

## Constraints

:::{rule} LIST-1
:kind: required
:tests: declarations/initialization/list/scalar-braces.cpp

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
:tests: declarations/initialization/list/narrowing-float-to-int.cpp declarations/initialization/list/narrowing-float-to-int-exact.cpp declarations/initialization/list/narrowing-constant-out-of-range.cpp declarations/initialization/list/narrowing-constant-fits.cpp declarations/initialization/list/narrowing-negative-to-unsigned.cpp declarations/initialization/list/narrowing-nonconstant-int.cpp declarations/initialization/list/narrowing-double-to-float-in-range.cpp declarations/initialization/list/narrowing-double-to-float-out-of-range.cpp declarations/initialization/list/narrowing-double-to-float-nonconstant.cpp declarations/initialization/list/narrowing-int-to-float-inexact.cpp declarations/initialization/list/narrowing-int-to-float-exact.cpp declarations/initialization/list/narrowing-pointer-to-bool.cpp declarations/initialization/list/narrowing-in-aggregate.cpp declarations/initialization/list/narrowing-not-in-parens.cpp declarations/initialization/list/narrowing-bit-field-fits.cpp declarations/initialization/list/narrowing-member-pointer-to-bool.cpp declarations/initialization/list/narrowing-unscoped-enum.cpp foundations/conversion-sequences/chosen-narrowing.cpp

In braces, a value must not go through a *narrowing* conversion. These
conversions are narrowing:

| From | To | Not narrowing if |
|---|---|---|
| Floating-point | Integer | Never |
| Integer | Floating-point | The value is a constant that the target type holds exactly. |
| `long double` | `double` or `float` | The value is a constant in the range of the target type. |
| `double` | `float` | The value is a constant in the range of the target type. |
| Integer | An integer type that cannot hold all its values | The value is a constant that fits. |
| Pointer or pointer to member | `bool` | Never |

An unscoped enumeration without a fixed type counts as an integer
source. A bit-field counts by its width, so a bit-field whose values all
fit in the target is not narrowing ({question}`Q-0042`). The same
conversions are permitted in parentheses (INIT-4).
:::

:::{rule} LIST-3
:kind: required
:tests: declarations/initialization/list/aggregate-members-in-order.cpp declarations/initialization/list/aggregate-paren-init.cpp declarations/initialization/list/aggregate-with-base.cpp

An aggregate is initialized in the order of declaration. Its base
classes are initialized first, then its members. The values can be in
braces or in parentheses. LIST-12 lists what is not valid.
:::

:::{rule} LIST-12
:kind: reject
:tests: declarations/initialization/list/aggregate-too-many.cpp declarations/initialization/list/missing-reference-member.cpp

The list for an aggregate must not have more values than the aggregate
has bases and members. A reference member without a default member
initializer must have a value in the list.
:::

:::{rule} LIST-4
:kind: required
:tests: declarations/initialization/list/brace-elision.cpp declarations/initialization/list/no-elision-when-direct.cpp declarations/initialization/list/empty-base-braces-ok.cpp

The inner braces for a member that is an array or an aggregate can be
omitted. The values then fill the elements of that member in order. The
next values go to the next member. The inner braces are not omitted for
a value that can initialize the member directly, such as an object of
the member's type. LIST-13 lists what is not valid.
:::

:::{rule} LIST-13
:kind: reject
:tests: declarations/initialization/list/empty-base-no-elision.cpp

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
:tests: declarations/initialization/list/designated-out-of-order.cpp declarations/initialization/list/designated-mixed.cpp declarations/initialization/list/designated-nested.cpp declarations/initialization/list/designated-array.cpp declarations/initialization/list/designated-repeated.cpp declarations/initialization/list/designator-on-scalar.cpp foundations/conversion-sequences/chosen-designator-order.cpp

The designators in a braced list must obey these conditions:

- They must be in the order of member declaration.
- Each member must occur one time only.
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
:tests: declarations/initialization/list/omitted-member-explicit-default.cpp

An `explicit` default constructor cannot initialize a member that has no
value in the list (LIST-8). Such a member must have a value in the list.
:::

## Runtime Behavior

:::{rule} LIST-7
:kind: required
:tests: declarations/initialization/list/braced-order.cpp expressions/sequencing/parenthesized-aggregate-order.cpp

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
:tests: declarations/initialization/list/single-same-class-copies.cpp

If a braced list for an aggregate has one element of the same class
type, the aggregate is copied from that element. {rule}`ICS-18` gives
the conversion sequence of such a list.
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
unsigned char n[] = "x";  // valid
signed char o[] = u8"x";  // rejected: wrong character type
```

## Evidence

All eight compilers agree on 57 of 60 tests. The results are in
`tests/RESULTS.md`.

- **LIST-14.** Clang accepts array designators, chained designators, and
  a mix of designated and plain values as C99 extensions, and rejects
  them only under `-pedantic-errors`. GCC and EDG reject them.
- **LIST-6.** Every compiler accepts each pair of literal and element
  type in the table, for an array variable, with and without braces.
  Every compiler rejects the other pairs of the five literal kinds and
  the seven character types. For a reference to a character array, GCC
  rejects a braced literal that needs a temporary array. Clang and EDG
  make the temporary ({question}`Q-0166`).
- **LIST-6.** For one braced plain literal, GCC prefers a parameter
  that is a reference to a `char` array over one to an `unsigned char`
  array. Clang and EDG find the call ambiguous ({question}`Q-0167`).

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).

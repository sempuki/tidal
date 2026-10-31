# Enumerations

An enumeration is a distinct type with named integer values, its
*enumerators*. A *scoped* enumeration (`enum class` or `enum struct`)
keeps its enumerators in its own scope. An *unscoped* enumeration
(`enum`) puts its enumerators in the enclosing scope. The *underlying
type* is the integer type that holds the values.

## Principles

:::{principle} ENUM-P1
:rules: ENUM-10 ENUM-1 ENUM-14 ENUM-12 ENUM-13

An enumeration is a type of its own whose values are integers. The
enumeration is different from its underlying type and from every other
enumeration, and each enumerator has the enumeration type. Enumerators
take values in sequence, each one more than the last unless the program
gives a value, and a value can use the enumerators before it. An
enumerator's name exists only after its value, so the value cannot name
the enumerator ({principle}`NAM-P1`).
:::

:::{principle} ENUM-P2
:rules: ENUM-2 ENUM-15 ENUM-3 ENUM-16 ENUM-11 ENUM-9 ENUM-20

The two kinds of enumeration differ in where their names go and how
freely their values convert.

| | Unscoped | Scoped |
|---|---|---|
| Enumerators | Go into the enclosing scope | Stay inside the enumeration |
| Values | Promote to an integer, as C does | Convert to or from an integer only with a cast, so they take part in no arithmetic and stand in no condition |

`using enum` brings the enumerators of either kind into a scope by name.
:::

:::{principle} ENUM-P3
:rules: ENUM-4 ENUM-17 ENUM-5 ENUM-6 ENUM-21 ENUM-7 ENUM-18 ENUM-8 ENUM-19

The underlying type holds the values.

| | Fixed by the program, or scoped | Not fixed |
|---|---|---|
| Underlying type | Known from the first declaration | A type wide enough, chosen by the implementation |
| Values of the enumeration | Every value of the type | Only the bits the enumerators need. A value outside that range has no meaning. |

When the type is known from the first declaration, the enumeration can
be declared before its enumerators, and an integer in braces can
initialize a variable of the enumeration type. A cast to such an
enumeration converts the value to the underlying type, so every integer
gives a value of the enumeration.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   enum_def: `enum_head` [":" `type_specifiers`]
           :    "{" [`enumerator` {"," `enumerator`} [","]] "}"
   enum_opaque: `enum_head` ":" `type_specifiers` ";"
              : | ("enum" "class" | "enum" "struct") `identifier` ";"
   enum_head: "enum" ["class" | "struct"] {`attribute_spec`} [[`nested_name`] `identifier`]
   enumerator: `identifier` {`attribute_spec`} ["=" `constant_expr`]
```

## Constraints

:::{rule} ENUM-1
:kind: required
:tests: declarations/enumerations/enumerator-values.cpp declarations/enumerations/enumerator-refers-earlier.cpp declarations/enumerations/duplicate-values.cpp declarations/enumerations/implicit-after-explicit.cpp

An enumerator without a value gets its value as follows:

- If the first enumerator of the list has no value, the enumerator
  gets 0.
- A later enumerator without a value gets the value of the previous
  enumerator plus one, also when the later enumerator is the first one
  without a value.

A value can use earlier enumerators, and several enumerators can have
the same value. ENUM-14 lists what is not valid.
:::

:::{rule} ENUM-14
:kind: reject
:tests: declarations/enumerations/enumerator-not-constant.cpp declarations/enumerations/duplicate-enumerator.cpp declarations/enumerations/implicit-value-overflow-scoped.cpp

These conditions apply to the enumerators of ENUM-1:

- A value must be a constant expression.
- Two enumerators must not have the same name.
- An implicit value must fit in a fixed underlying type (ENUM-4).
:::

:::{rule} ENUM-2
:kind: required
:tests: declarations/enumerations/unscoped-names-in-enclosing-scope.cpp declarations/enumerations/unscoped-converts-to-int.cpp declarations/enumerations/mixed-enum-arithmetic.cpp

The enumerators of an unscoped enumeration are declared in the enclosing
scope. The enumerators can also be named through the enumeration. A
value converts implicitly to an integer by {term}`integer promotion`.
ENUM-15 lists what is not valid.
:::

:::{rule} ENUM-15
:kind: reject
:tests: declarations/enumerations/int-to-unscoped-rejected.cpp declarations/enumerations/unscoped-name-conflict.cpp

An integer does not convert implicitly to an unscoped enumeration
(ENUM-2). An enumerator name must not be declared already in the
enclosing scope.
:::

:::{rule} ENUM-3
:kind: required
:tests: declarations/enumerations/scoped-compare-same-type.cpp declarations/enumerations/scoped-explicit-conversion.cpp

The enumerators of a scoped enumeration are named through the
enumeration. A value converts to and from an integer with an explicit
cast. Two values of the same scoped enumeration can be compared. ENUM-16
lists what is not valid.
:::

:::{rule} ENUM-16
:kind: reject
:tests: declarations/enumerations/scoped-names-scoped.cpp declarations/enumerations/scoped-no-conversion.cpp declarations/enumerations/scoped-no-arithmetic.cpp declarations/enumerations/scoped-compare-int.cpp declarations/enumerations/increment-enum.cpp

The forms that ENUM-3 allows are the only valid forms. A program must
not:

- name an enumerator of a scoped enumeration without the enumeration,
- convert a scoped value to or from an integer without an explicit cast,
- use a scoped value in arithmetic,
- compare a scoped value with an integer,
- initialize an integer from a scoped value,
- apply `++` or `--` to an enumeration without a user-declared operator
  ({rule}`OPR-6`).
:::

:::{rule} ENUM-4
:kind: required
:tests: declarations/enumerations/scoped-default-underlying-int.cpp declarations/enumerations/fixed-underlying.cpp declarations/enumerations/fixed-bool.cpp

A scoped enumeration without `: type` has the underlying type `int`. A
fixed underlying type comes after `:`. ENUM-17 lists what is not valid.

The fixed underlying type can be `bool` or a character type. A
cv-qualifier on the underlying type is ignored.
:::

:::{rule} ENUM-17
:kind: reject
:tests: declarations/enumerations/scoped-value-too-large.cpp declarations/enumerations/fixed-underlying-narrowing.cpp declarations/enumerations/fixed-underlying-non-integral.cpp

A fixed underlying type (ENUM-4) must be an integer type. Each
enumerator value must fit in the underlying type without narrowing.
:::

:::{rule} ENUM-7
:kind: required
:tests: declarations/enumerations/opaque-declaration.cpp

An enumeration can be declared without its enumerators before its
definition. ENUM-18 lists what such a declaration needs.
:::

:::{rule} ENUM-18
:kind: reject
:tests: declarations/enumerations/opaque-unscoped-without-type.cpp declarations/enumerations/opaque-mismatch.cpp declarations/enumerations/redeclare-scoped-as-unscoped.cpp

If an enumeration is declared without its enumerators (ENUM-7), its
underlying type must be known, so the enumeration must be scoped or have
`: type`. All declarations must agree on scoped or unscoped and on the
underlying type.
:::

:::{rule} ENUM-8
:kind: required
:tests: declarations/enumerations/list-init-fixed.cpp

A variable of an enumeration type with a fixed underlying type can be
initialized from an integer in braces. This rule includes all scoped
enumerations. ENUM-19 lists what is not valid.
:::

:::{rule} ENUM-19
:kind: reject
:tests: declarations/enumerations/list-init-no-fixed.cpp declarations/enumerations/list-init-narrowing.cpp declarations/enumerations/copy-list-from-integer.cpp

In the initialization that ENUM-8 allows, the conversion to the
underlying type must not narrow. An enumeration without a fixed
underlying type cannot be initialized from an integer in braces.

Initialization from an integer in braces is valid only as
direct-list-initialization. Initialization with `=` and a braced
integer is not valid.
:::

:::{rule} ENUM-9
:kind: required
:tests: declarations/enumerations/using-enum.cpp declarations/enumerations/using-declaration-enumerator.cpp declarations/enumerations/using-enum-alias.cpp

`using enum E;` declares the enumerators of `E` in the current scope.
`E` can be a type alias of an enumeration. A using declaration can also
name one enumerator of a scoped enumeration, such as `using C::R;`.
ENUM-20 lists what is not valid.
:::

:::{rule} ENUM-20
:kind: reject
:tests: declarations/enumerations/using-enum-conflict.cpp declarations/enumerations/using-enum-not-enum.cpp declarations/enumerations/using-enum-opaque.cpp

In `using enum E;` (ENUM-9), `E` must be an enumeration. No enumerator
name of `E` can be declared already in that scope.

The enumeration must be complete at the `using enum` declaration. An
opaque declaration is not enough ({question}`Q-0047`).
:::

:::{rule} ENUM-10
:kind: required
:tests: declarations/enumerations/distinct-types.cpp declarations/enumerations/enumerator-type-inside-list.cpp declarations/enumerations/enumerator-type-char-initializer.cpp declarations/enumerations/enumerator-type-enum-initializer.cpp declarations/enumerations/enumerator-type-short-initializer.cpp

Each enumeration is a distinct type. The enumeration is different from
its underlying type and from all other enumerations. After the closing
brace, an enumerator has the type of its enumeration. Unary `+` on an
unscoped value gives its promoted integer type.

Inside the enumerator list, the enumeration is not complete, so an
enumerator has another type there. Every scoped enumeration has a fixed
underlying type.

| Underlying type | Enumerator | Type inside the list |
|---|---|---|
| Fixed | Any | The underlying type |
| Not fixed | With an initializer | The type of the initializer, without promotion. An initializer of enumeration type gives that enumeration type. |
| Not fixed | Without an initializer | The type of the enumerator before it, if that type can hold its value (ENUM-13) |

:::

:::{rule} ENUM-11
:kind: reject
:tests: declarations/enumerations/scoped-not-condition.cpp declarations/enumerations/scoped-not-logical.cpp declarations/enumerations/scoped-explicit-bool-ok.cpp

A value of a scoped enumeration does not convert to `bool` by
{term}`contextual conversion`. Such a value must not be a condition or
an operand of `!`, `&&`, or `||`. An explicit cast to `bool` is valid.
:::

:::{rule} ENUM-12
:kind: required
:tests: declarations/enumerations/enumerator-scope-after-value.cpp

The name of an enumerator is declared after its value expression. In
that expression, the name refers to an outer entity with that name.
:::

## Runtime Behavior

:::{rule} ENUM-21
:kind: required
:tests: declarations/enumerations/cast-fixed-converts-to-underlying.cpp

If an integer converts to an enumeration with a fixed underlying type,
the integer first converts to the underlying type ({rule}`CONV-2`,
{rule}`CONV-3`). The enumeration value is the result of that
conversion. The integer can be outside the range of the underlying type.
:::

## Undefined Behavior

:::{rule} ENUM-6
:kind: undefined
:tests: declarations/enumerations/cast-in-range.cpp declarations/enumerations/cast-out-of-range.cpp declarations/enumerations/cast-fixed-any-value.cpp declarations/enumerations/empty-enum-range.cpp declarations/enumerations/cast-out-of-range-sanitizer.cpp

An enumeration with a fixed underlying type can hold all values of that
type. An enumeration without a fixed underlying type holds the values of
the smallest set of bits that can represent all its enumerators. If a
program converts a value outside that range to the enumeration, the
behavior is undefined.

An enumeration without enumerators and without a fixed type can hold the
values 0 and 1.
:::

## Implementation Requirements

:::{rule} ENUM-13
:kind: unspecified
:tests: declarations/enumerations/enumerator-type-grows.cpp

Inside the enumerator list of an enumeration without a fixed type, an
enumerator without an initializer whose value the type of the
enumerator before it cannot hold has an integer type that can hold the
value. Which type is unspecified ({question}`Q-0114`).
:::

:::{rule} ENUM-5
:kind: impl-defined
:tests: declarations/enumerations/unscoped-underlying-holds-values.cpp declarations/enumerations/unscoped-size-lp64.cpp declarations/enumerations/implicit-value-overflow-unscoped.cpp declarations/enumerations/unscoped-value-too-large.cpp

The underlying type of an unscoped enumeration without `: type` is
implementation-defined. The underlying type is an integer type that can
hold all enumerator values, and it is not wider than `int` unless a
value needs more. If no standard integer type can hold all values, the
implementation can use a wider type or reject the enumeration.
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
U u = static_cast<U>(300);     // 44: 300 converts to unsigned char

enum class H : int;            // declared before the definition
H h{5};                        // valid: fixed underlying type

using enum Color;
Color k = Red;                 // Red without Color::
```

## Evidence

All eight compilers agree on 56 of 59 tests. The results are in
`tests/RESULTS.md`.

- **ENUM-5.** With `-fshort-enums`, Clang and GCC give `enum { S = 1 }`
  an underlying type of 1 byte. By default its size is the size of
  `int`.
- **ENUM-5, no standard underlying type.** GCC accepts an unscoped
  enumeration that needs more than 64 bits, and uses a 128-bit type.
  Clang and EDG reject the enumeration. The provisional answer is that
  the choice is implementation-defined. See {question}`Q-0018`.
- **ENUM-6.** Clang rejects `static_cast<E>(8)` during constant
  evaluation ("outside the valid range of values [0, 7]"). GCC and EDG
  accept the cast. The provisional answer is that the behavior is
  undefined. See {question}`Q-0017`.
- **ENUM-21.** Clang, GCC, and EDG agree that `static_cast` of 300 to an
  enumeration with underlying type `unsigned char` gives 44 during
  constant evaluation. Clang and GCC give the same value at run time.

Inputs: clang-sema-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

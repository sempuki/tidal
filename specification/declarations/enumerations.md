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

Inside the list, an enumeration without a fixed type is not complete
yet, so each enumerator has the type of its own value:

- With an initializer, the enumerator has the type of the initializer.
- Without one, the enumerator keeps the type of the enumerator before
  it.
- If that type cannot hold the new value, the enumerator gets an integer
  type that can hold the value. The implementation chooses which.

After the closing brace, every enumerator has the enumeration type.
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
So the enumerators must exist at that point, and the definition with the
list of enumerators must come first. An opaque declaration names no
enumerator, so `using enum` cannot use it, although the opaque
declaration makes the enumeration complete.
:::

:::{principle} ENUM-P3
:rules: ENUM-4 ENUM-17 ENUM-5 ENUM-22 ENUM-6 ENUM-21 ENUM-7 ENUM-18 ENUM-8 ENUM-19

The underlying type holds the values.

| | Fixed by the program, or scoped | Not fixed |
|---|---|---|
| Underlying type | Known from the first declaration | A type wide enough, chosen by the implementation. If no integer type that the implementation provides is wide enough, the implementation can reject the enumeration. |
| Values of the enumeration | Every value of the type | Only the bits the enumerators need. A value outside that range has no meaning. |

When the type is known from the first declaration, the enumeration is
{term}`complete` from that declaration on, so the enumeration can be
declared before its enumerators, and an integer in braces can
initialize a variable of the enumeration type. A cast to such an
enumeration converts the value to the underlying type, so every integer
gives a value of the enumeration. An enumeration that the program
fills from integers by a cast therefore has a fixed type, and no such
cast is undefined.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   enum_def: `enum_head` [":" `type_specifiers`]
           :    "{" [`enumerator` {"," `enumerator`} [","]] "}"
   enum_opaque: "enum" ["class" | "struct"] {`attribute_spec`} [`nested_name`] `identifier`
              :    [":" `type_specifiers`] ";"
   enum_head: "enum" {`attribute_spec`} [[`nested_name`] `identifier`]
            : | "enum" ("class" | "struct") {`attribute_spec`} [`nested_name`] `identifier`
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
Arithmetic between two different enumerations is valid and deprecated
({rule}`ADD-16`, {rule}`MUL-16`, {rule}`BIT-8`). ENUM-15 lists what is
not valid.
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
:tests: declarations/enumerations/scoped-compare-same-type.cpp declarations/enumerations/scoped-explicit-conversion.cpp declarations/enumerations/scoped-functional-cast.cpp

The enumerators of a scoped enumeration are named through the
enumeration. A value converts to and from an integer with an explicit
cast, which includes the form `F(5)` ({rule}`CAST-9`). Two values of the same scoped enumeration can be compared. ENUM-16
lists what is not valid.
:::

:::{rule} ENUM-16
:kind: reject
:tests: declarations/enumerations/scoped-names-scoped.cpp declarations/enumerations/scoped-no-conversion.cpp declarations/enumerations/scoped-no-arithmetic.cpp declarations/enumerations/scoped-compare-int.cpp declarations/enumerations/increment-enum.cpp declarations/enumerations/scoped-parentheses-from-integer.cpp

The forms that ENUM-3 allows are the only valid forms. A program must
not:

- name an enumerator of a scoped enumeration without the enumeration,
- convert a scoped value to or from an integer without an explicit cast,
- use a scoped value in arithmetic,
- compare a scoped value with an integer,
- initialize an integer from a scoped value,
- initialize a scoped enumeration from an integer in parentheses, as
  in `F f(5);`,
- apply `++` or `--` to an enumeration without a user-declared operator
  ({rule}`OPR-6`).
:::

:::{rule} ENUM-4
:kind: required
:tests: declarations/enumerations/scoped-default-underlying-int.cpp declarations/enumerations/fixed-underlying.cpp declarations/enumerations/fixed-bool.cpp

A scoped enumeration without `: type` has the underlying type `int`. A
fixed underlying type comes after `:`. ENUM-17 lists what is not valid.

The fixed underlying type can be `bool` or a character type. A
{term}`cv-qualifier` on the underlying type is ignored.
:::

:::{rule} ENUM-17
:kind: reject
:tests: declarations/enumerations/scoped-value-too-large.cpp declarations/enumerations/fixed-underlying-narrowing.cpp declarations/enumerations/fixed-underlying-non-integral.cpp

A fixed underlying type (ENUM-4) must be an integer type. Each
enumerator value must fit in the underlying type without a
{term}`narrowing conversion`.
:::

:::{rule} ENUM-7
:kind: required
:tests: declarations/enumerations/opaque-declaration.cpp declarations/enumerations/opaque-declaration-member.cpp

An enumeration can be declared without its enumerators before its
definition, in an {term}`opaque enumeration declaration`. ENUM-18 lists what such a declaration needs.
:::

:::{rule} ENUM-18
:kind: reject
:tests: declarations/enumerations/opaque-unscoped-without-type.cpp declarations/enumerations/opaque-mismatch.cpp declarations/enumerations/redeclare-scoped-as-unscoped.cpp declarations/type-names/enum-fixed-mismatch.cpp declarations/type-names/enum-fixed-after-not-fixed.cpp declarations/type-names/enum-scoped-long-then-default.cpp declarations/type-names/enum-scoped-mismatch.cpp declarations/type-names/forward-enum-base-mismatch.cpp declarations/type-names/forward-enum-no-base.cpp

If an enumeration is declared without its enumerators (ENUM-7), its
underlying type must be known, so the enumeration must be scoped or have
`: type`. All declarations must agree on scoped or unscoped and on the
underlying type. A declaration without `: type` gives a scoped
enumeration the type `int` and gives an unscoped enumeration no fixed
type. So after `enum E : int;`, the definition `enum E { a };` is not
valid.
:::

:::{rule} ENUM-8
:kind: required
:tests: declarations/enumerations/list-init-fixed.cpp declarations/enumerations/list-init-fixed-member-and-new.cpp

A variable of an enumeration type with a fixed underlying type can be
initialized from an integer in braces. This rule includes all scoped
enumerations. ENUM-19 lists what is not valid.
:::

:::{rule} ENUM-19
:kind: reject
:tests: declarations/enumerations/list-init-no-fixed.cpp declarations/enumerations/list-init-narrowing.cpp declarations/enumerations/copy-list-from-integer.cpp

In the initialization that ENUM-8 allows, the conversion to the
underlying type must not be a {term}`narrowing conversion`. An enumeration without a fixed
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

The enumerators of `E` must be declared at the `using enum`
declaration, so the definition of `E` must come before it. An opaque
declaration makes `E` {term}`complete` and declares no enumerator
({question}`Q-0047`).
:::

:::{rule} ENUM-10
:kind: required
:tests: declarations/enumerations/fixed-complete-inside-list.cpp declarations/enumerations/distinct-types.cpp declarations/enumerations/enumerator-type-inside-list.cpp declarations/enumerations/enumerator-type-char-initializer.cpp declarations/enumerations/enumerator-type-enum-initializer.cpp declarations/enumerations/enumerator-type-short-initializer.cpp

Each enumeration is a distinct type. The enumeration is different from
its underlying type and from all other enumerations. After the closing
brace, an enumerator has the type of its enumeration. Unary `+` on an
unscoped value gives its promoted integer type.

Inside the enumerator list, an enumerator has another type. An
enumeration without a fixed underlying type is not {term}`complete`
there. An enumeration with a fixed underlying type is complete there.
Every scoped enumeration has a fixed underlying type.

| Underlying type | Enumerator | Type inside the list |
|---|---|---|
| Fixed | Any | The underlying type |
| Not fixed | With an initializer | The type of the initializer, without promotion and without cv-qualifiers. An initializer of enumeration type gives that enumeration type. |
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
:tests: declarations/enumerations/enumerator-scope-after-value.cpp declarations/enumerations/enumerator-scope-scoped.cpp

The name of an enumerator is declared after its value expression. In
that expression, the name refers to an outer entity with that name.
:::

## Runtime Behavior

:::{rule} ENUM-22
:kind: required
:tests: declarations/enumerations/cast-in-range.cpp declarations/enumerations/cast-fixed-any-value.cpp declarations/enumerations/empty-enum-range.cpp

The values of an enumeration are these:

- With a fixed underlying type, all values of that type.
- Without a fixed underlying type, the values of the smallest set of
  bits that can represent all its enumerators.
- Without enumerators and without a fixed type, the values 0 and 1.
:::

:::{rule} ENUM-21
:kind: required
:tests: declarations/enumerations/cast-fixed-converts-to-underlying.cpp declarations/enumerations/cast-fixed-wide-integer.cpp

If an integer converts to an enumeration with a fixed underlying type,
the integer first converts to the underlying type ({rule}`CONV-2`,
{rule}`CONV-3`). The enumeration value is the result of that
conversion. The integer can be outside the range of the underlying type.
:::

## Undefined Behavior

:::{rule} ENUM-6
:kind: undefined
:tests: declarations/enumerations/cast-out-of-range.cpp declarations/enumerations/cast-out-of-range-sanitizer.cpp declarations/enumerations/bytes-out-of-range-sanitizer.cpp

If a program converts a value that is not a value of the enumeration
(ENUM-22) to the enumeration, the behavior is undefined. If an object of
an enumeration type gets such a value by other means, such as a copy of
the bytes of another object, a read of the object is undefined
({question}`Q-0336`).
:::

## Implementation Requirements

:::{rule} ENUM-13
:kind: unspecified
:tests: declarations/enumerations/enumerator-type-grows.cpp declarations/enumerations/enumerator-type-grows-unsigned.cpp

Inside the enumerator list of an enumeration without a fixed type, an
enumerator without an initializer whose value the type of the
enumerator before it cannot hold has an integer type that can hold the
value. Which type is unspecified ({question}`Q-0114`).
:::

:::{rule} ENUM-5
:kind: impl-defined
:tests: declarations/enumerations/unscoped-underlying-holds-values.cpp declarations/enumerations/unscoped-size-lp64.cpp declarations/enumerations/implicit-value-overflow-unscoped.cpp declarations/enumerations/unscoped-value-too-large.cpp declarations/enumerations/unscoped-size-wide-lp64.cpp declarations/enumerations/unscoped-size-ilp32.cpp

The underlying type of an unscoped enumeration without `: type` is
implementation-defined. The underlying type is an integer type that can
hold all enumerator values, and it is not wider than `int` unless a
value needs more. If no {term}`integer type` can hold all values, the
implementation can use a wider type or reject the enumeration. The
matrix compilers take the first type that holds all values, on x86-64
and with `-m32`, and try the types in this order:

| Compiler | No value is negative | A value is negative |
|---|---|---|
| Clang, GCC | the unsigned forms of `int`, `long`, and `long long` | `int`, `long`, `long long` |
| EDG | `int`, `long`, and `long long`, each followed by its unsigned form | the same |
:::

## Examples

% tidal-example: reject
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
H h{5};                        // valid: H is complete, with a fixed type
using enum H;                  // rejected: no enumerator of H is declared yet

using enum Color;
Color k = Red;                 // Red without Color::
```

A last enumerator can count the others, so a table of names can be
checked against the enumeration. `Count` is a value of the type too, and
a `switch` over the type has a case for it:

```cpp
enum class Level : unsigned char { Low, Mid, High, Count };
constexpr const char* names[] = {"low", "mid", "high"};
static_assert(sizeof names / sizeof names[0] ==
              static_cast<int>(Level::Count));  // fails if a name is missing
```

## Evidence

All eight compilers agree on 73 of 76 tests. The results are in
`tests/RESULTS.md`.

- **ENUM-5.** With `-fshort-enums`, Clang and GCC give `enum { S = 1 }`
  an underlying type of 1 byte. By default its size is the size of
  `int`.
- **ENUM-5, the matrix types.** A test with no standard header observes
  the size of the underlying type and the values that it holds. The
  tests check the sizes, which agree on every compiler. The types of
  the table come from the writer's probes with each compiler's builtin
  query (decision 0051). The families differ only in signedness. An
  enumeration with no enumerators gets `unsigned int` from Clang and
  GCC, and `int` from EDG.
- **ENUM-5, no standard underlying type.** GCC accepts an unscoped
  enumeration that needs more than 64 bits, and uses a 128-bit type.
  Clang and EDG reject the enumeration. The provisional answer is that
  the choice is implementation-defined. See {question}`Q-0018`.
- **ENUM-6.** Clang rejects `static_cast<E>(8)` during constant
  evaluation ("outside the valid range of values [0, 7]"). GCC and EDG
  accept the cast. The provisional answer is that the behavior is
  undefined. See {question}`Q-0017`.
- **ENUM-6.** At run time, the undefined behavior sanitizer of Clang
  and GCC (`-fsanitize=undefined`) does not stop the conversion of an
  out-of-range value. The sanitizer stops at the first read of the
  stored value ("load of value 5, which is not a valid value for type").
  The sanitizer stops the same read when the value came from the bytes
  of an `int` ({question}`Q-0336`).
- **ENUM-10.** Clang and GCC accept `sizeof` of an enumeration with a
  fixed underlying type inside its own enumerator list, and reject it
  for an enumeration without one.
- **TR-12.** Clang and GCC reject `enum class` and `enum struct` with
  no name.
- **ENUM-2.** Clang and GCC accept arithmetic between two different
  unscoped enumerations under `-pedantic-errors`, with a deprecation
  warning that is on by default. EDG accepts it too.
- **ENUM-21.** Clang, GCC, and EDG agree that `static_cast` of 300 to an
  enumeration with underlying type `unsigned char` gives 44 during
  constant evaluation. Clang and GCC give the same value at run time.

Inputs: clang-sema-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

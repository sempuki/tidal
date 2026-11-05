# Standard conversions

A standard conversion changes a value to another type where an operator,
an initialization, or a call needs that type. This page covers these
conversions:

- between {term}`arithmetic types <arithmetic type>`,
- to `bool`,
- between pointer types.

## Principles

:::{principle} CONV-P1
:rules: CONV-1 CONV-13 CONV-19 CONV-20

An operand takes the shape that the operation needs before the operation
sees it:

| Operand | Becomes | Why |
|---|---|---|
| An integer narrower than `int` | An `int`. For a bit-field the width decides. | The width limits the values that the integer can hold. |
| An array used as a value | A pointer to its first element | The operations on values have no use for an array itself. |
| A function name | A pointer to the function | The operations on values have no use for a function itself. |
| A `float` | A `float`, unchanged | Arithmetic on `float` is an operation of its own. |
:::

:::{principle} CONV-P2
:rules: CONV-2 CONV-3 CONV-21
:exceptions: CONV-12

A conversion between integer types keeps the value when the target can
hold it. When the target cannot, the target keeps the low bits, so the
result is the value modulo the target's range, for signed and unsigned
targets alike. A conversion to `bool` gives `false` for zero and `true`
for any other value, and a null pointer counts as zero.

A conversion that can change the value in this way, or drop a fraction,
is a {term}`narrowing conversion`. Braces and the constants of
{rule}`CE-19` reject it. A constant whose value the target holds is
exempt, because its value does not change.

A conversion from an integer type to a floating-point type counts as
narrowing by its types alone. `int` to `double` narrows, although
`double` holds every `int` value on the matrix targets. Only a constant
is exempt. The operator `<=>` also rejects narrowing, but it lets
this conversion through ({rule}`CMP-13`).
:::

:::{principle} CONV-P4
:rules: CONV-4 CONV-5 CONV-6 CONV-7 CONV-11

A conversion to or from a floating-point type keeps the value when the
target can hold it exactly. The table gives the results:

| Target | Value | Result |
|---|---|---|
| Floating-point | Cannot be held exactly | One of the two nearest values |
| Floating-point | Beyond the target's range | The largest finite value or infinity, by the implementation's choice |
| Integer, from floating-point | Fits after its fraction is dropped | The value with its fraction dropped |
| Integer, from floating-point | Does not fit after its fraction is dropped | No answer, so the behavior is undefined |
:::

:::{principle} CONV-P3
:rules: CONV-8 CONV-9 CONV-10 CONV-14 CONV-15 CONV-16 CONV-17 CONV-18

A pointer converts implicitly only in the direction that loses no
safety:

- toward `void`,
- toward a base class,
- toward more cv-qualifiers at every level,
- toward a function type without `noexcept`,
- toward a pointer to member of a derived class,
- from an array of known bound to one of unknown bound.

Each reverse direction would claim something the pointer cannot promise,
so no reverse direction is implicit. A null pointer comes only from
`nullptr` or an integer literal with the value zero, such as `0`, `0L`,
or `(0)`. Any other zero is a number, and a number is not a pointer.
The character literal `'\0'`, the literal `false`, and an integer
variable that holds zero are such numbers. For the same reason a
pointer is not a number, so no pointer converts implicitly to an
integer. A base that is virtual, ambiguous, or not accessible gives no
single path, so a conversion through such a base is not implicit. Such
a conversion still takes part in overload resolution. A call that
chooses it is rejected, and an overload with an ellipsis does not win
in its place ({rule}`ICS-8`).
:::

## Constraints

:::{rule} CONV-1
:kind: required
:tests: foundations/conversions/integral-promotion.cpp foundations/conversions/bitfield-promotes.cpp foundations/conversions/enum-promotion.cpp foundations/conversions/bitfield-promotion-by-width.cpp foundations/conversions/enum-promotion-no-fixed-type.cpp foundations/types/rank-promotions.cpp

The following operands go through {term}`integer promotion` in
arithmetic:

- `bool`, the character types, `short`, and `unsigned short`. Such a
  type promotes to `int` if `int` can hold all its values, and
  otherwise to `unsigned int`. On both matrix targets, `char32_t`
  promotes to `unsigned int`, and the other types promote to `int`.
- a bit-field, by its width and not by its declared type (BF-6). A
  bit-field of width 3 declared `long long` promotes to `int`; a
  bit-field of width 40 keeps `long long`,
- an unscoped enumeration. With a fixed underlying type, the value
  converts to that type first and then promotes as that type does.
  Without a fixed underlying type, the candidates are tried by rank from
  `int` upward, the signed type of each rank before the unsigned one,
  and the first that can hold all the values of the enumeration ({rule}`ENUM-22`)
  is the result.

A `float` operand does not change to `double` in arithmetic.
:::

:::{rule} CONV-19
:kind: required
:tests: foundations/conversions/uac-floating.cpp expressions/decltype/arithmetic-common-type.cpp foundations/conversions/uac-promoted-same.cpp foundations/conversions/uac-same-signedness.cpp foundations/conversions/uac-unsigned-rank-not-lower.cpp foundations/conversions/uac-signed-holds-unsigned-lp64.cpp foundations/conversions/uac-unsigned-of-signed-lp64.cpp foundations/conversions/uac-unsigned-of-signed-ilp32.cpp foundations/conversions/binary-operator-common-type-contexts.cpp

The {term}`usual arithmetic conversions` give the two operands of a
binary arithmetic operator one common type. The first step that applies
decides the common type:

1. If either operand has a floating-point type, the common type is the
   floating-point type of higher rank ({rule}`TYP-9`). Extended
   floating-point types have their own ranks
   *(extended floating-point types referenced)*. If neither rank is
   higher, the operation is not valid.
2. Otherwise both operands are promoted (CONV-1). If the promoted
   operands have the same type, that type is the common type.
3. If both types are signed, or both are unsigned, the common type is
   the one of higher rank ({rule}`TYP-8`).
4. If the unsigned type's rank is not lower than the signed type's, the
   common type is the unsigned type.
5. If the signed type can hold every value of the unsigned type, the
   common type is the signed type.
6. Otherwise the common type is the unsigned type of the signed type's
   rank.

When the common type is unsigned, a negative signed operand converts to
a large value ({rule}`CONV-2`). The operator `<=>` rejects such a
{term}`narrowing conversion` ({rule}`CMP-13`).
:::

:::{rule} CONV-20
:kind: required
:tests: foundations/conversions/implicit-conversion-contexts.cpp foundations/conversions/implicit-conversion-more-contexts.cpp foundations/conversions/implicit-conversion-member-and-new-contexts.cpp foundations/conversions/binary-operator-common-type-contexts.cpp

An implicit conversion happens in each of these contexts. The rule named
for each context gives the target type:

- the initializer of a variable ({rule}`INIT-4`),
- a member initializer and a default member initializer, to the type of
  the member ({rule}`CTOR-5`),
- the initializer of `new T(e)`, to `T`, and the size of an array
  `new`, to the type of `sizeof` ({rule}`NEW-2`),
- the initializer of a reference that binds to a temporary, to the
  referenced type ({rule}`REF-7`),
- a function argument ({rule}`PFX-6`),
- the standard conversions before and after the conversion function in
  a user-defined conversion sequence ({rule}`ICS-3`),
- a default argument, to the type of its parameter ({rule}`DEFARG-10`),
- the expression of a `return` statement ({rule}`JMP-3`),
- the right operand of an assignment ({rule}`ASN-1`),
- the result of `op` in a compound assignment `E1 op= E2`, to the type
  of `E1` ({rule}`ASN-5`),
- the operands of the binary operators `*`, `/`, `%`, `+`, `-`, `&`,
  `|`, `^`, and the comparison operators, to their common type
  (CONV-19). Each operand of a shift is promoted alone
  ({rule}`SHF-4`).
- the step of `++` or `--` on a type that {term}`integer promotion`
  changes, back to the operand's type ({rule}`UN-20`),
- the second and third operands of `?:`, to the result type
  ({rule}`COND-3`),
- a condition, by {term}`contextual conversion` ({rule}`IF-3`),
- an array bound, an enumerator value, and a case label (CE-11, CE-19),
- the condition of a `switch` ({rule}`SW-2`),
- the parameter of a {term}`handler`, from the {term}`exception object`, by the
  conversions of {rule}`TRY-2` only.
:::

:::{rule} CONV-21
:kind: required
:tests: declarations/initialization/list/narrowing-constant-fits.cpp declarations/initialization/list/narrowing-constant-fits-bool.cpp declarations/initialization/list/narrowing-int-to-float-exact.cpp declarations/initialization/list/narrowing-double-to-float-in-range.cpp declarations/initialization/list/narrowing-nonconstant-long-ilp32.cpp declarations/initialization/list/narrowing-integer-to-bool.cpp declarations/initialization/list/narrowing-floating-to-bool.cpp declarations/initialization/list/narrowing-constant-to-bool-out-of-range.cpp declarations/initialization/list/narrowing-bit-field-fits.cpp declarations/initialization/list/narrowing-unscoped-enum-values-fit.cpp

A {term}`narrowing conversion` is an implicit conversion in this table,
unless the last column holds:

| From | To | Not narrowing if |
|---|---|---|
| Floating-point type | {term}`Integral type <integral type>`, `bool` included | Never |
| Integral type | Floating-point type | The value is a constant that the target type holds exactly. |
| `long double` | `double` or `float` | The value is a constant in the range of the target type. |
| `double` | `float` | The value is a constant in the range of the target type. |
| Integral type | An integral type that cannot hold all its values on the target ({rule}`TYP-1`), `bool` included | The value is a constant that fits. |
| Pointer or pointer to member | `bool` | Never |

An unscoped enumeration source counts as an integral source. Without a
fixed type, its values are those that {rule}`ENUM-22` gives
({question}`Q-0331`). A bit-field source counts by its width, so a
bit-field whose values all fit in the target is not narrowing
({question}`Q-0042`).
:::

:::{rule} CONV-8
:kind: reject
:tests: foundations/conversions/null-pointer-constants.cpp foundations/conversions/nonzero-to-pointer.cpp foundations/conversions/integer-variable-to-pointer.cpp foundations/conversions/false-not-null-pointer.cpp foundations/conversions/null-member-pointer-from-zero.cpp foundations/conversions/parenthesized-zero-null.cpp foundations/conversions/char-zero-not-null.cpp

`nullptr` and an integer literal with the value zero convert to a null
pointer of any pointer type and to a null pointer to member of any
member type. A zero literal in parentheses counts. A character literal
with the value zero is not an integer literal, so it does not count
({question}`Q-0100`). Other values must not convert to a pointer
type implicitly. This rule applies to an integer variable with the value
zero and to the literal `false`.
:::

:::{rule} CONV-9
:kind: reject
:tests: foundations/conversions/pointer-to-void-pointer.cpp foundations/conversions/void-pointer-to-pointer.cpp foundations/conversions/const-pointer-to-void-pointer.cpp foundations/conversions/function-pointer-to-void-pointer.cpp foundations/conversions/deep-const-every-level.cpp foundations/conversions/deep-const-not-at-every-level.cpp

A pointer to an object type converts implicitly to a pointer to `void`
with the same or more {term}`cv-qualifiers <cv-qualifier>`. These
conversions must not occur implicitly:

- a pointer to `void` to another pointer type,
- a conversion that removes a cv-qualifier,
- a pointer to a function to a pointer to `void`,
- across several levels of pointers, a conversion that adds `const` at
  one level without adding it at every level above that level.
:::

:::{rule} CONV-10
:kind: reject
:tests: foundations/conversions/noexcept-function-pointer.cpp foundations/conversions/potentially-throwing-to-noexcept.cpp foundations/conversions/member-function-noexcept-drop.cpp foundations/conversions/member-function-noexcept-add.cpp

A pointer to a `noexcept` function converts to a pointer to a function
without `noexcept`. The opposite conversion must not occur implicitly.
The same rules apply to pointers to member functions.
:::

:::{rule} CONV-12
:kind: reject
:tests: foundations/conversions/nullptr-to-bool-copy.cpp foundations/conversions/nullptr-to-bool-direct.cpp

`nullptr` converts to `bool` only in direct initialization, with
parentheses or braces. The conversion gives `false`.
:::

:::{rule} CONV-13
:kind: required
:tests: foundations/conversions/array-function-to-pointer.cpp foundations/conversions/array-function-to-pointer-constant.cpp

An array lvalue can be used as a pointer to element zero of the array.
A function name can be used as a pointer that calls that function.
:::

:::{rule} CONV-14
:kind: required
:tests: foundations/conversions/derived-to-base.cpp foundations/conversions/null-derived-to-second-base.cpp

A pointer or reference to a derived class converts implicitly to a
pointer or reference to a base class (DER-5). A null pointer converts to
a null pointer, also when the base {term}`subobject` is at a nonzero offset.
CONV-17 lists the bases that do not allow the conversion.
:::

:::{rule} CONV-15
:kind: required
:tests: foundations/conversions/member-pointer-base-to-derived.cpp foundations/conversions/member-function-pointer-base-to-derived.cpp

A member pointer that names a member of a base class can be used as a
member pointer of a derived class without a cast. The member pointer
still names the member that the base declares.
:::

:::{rule} CONV-17
:kind: reject
:tests: foundations/conversions/member-pointer-virtual-base.cpp foundations/conversions/member-pointer-ambiguous-base.cpp foundations/conversions/ambiguous-base.cpp foundations/conversions/member-pointer-derived-to-base.cpp overloading/conversion-sequences/chosen-private-base.cpp overloading/conversion-sequences/chosen-ambiguous-base.cpp

These conversions must not occur implicitly:

- the conversion of CONV-14 to a base that is ambiguous (DER-10) or
  not accessible ({rule}`ACC-12`),
- the conversion of CONV-15 through a {term}`virtual base` or an ambiguous
  base, or to a base that is not accessible (MPTR-11),
- a pointer to a member of a derived class to a pointer to a member of
  its base.

A conversion that this rule forbids still forms a conversion sequence
in overload resolution ({rule}`ICS-8`). With `take(Base*)` and
`take(...)`, a call `take(&derived)` through a private or ambiguous
base chooses `take(Base*)` and is rejected.
:::

:::{rule} CONV-18
:kind: reject
:tests: foundations/conversions/pointer-to-integer.cpp foundations/conversions/nullptr-to-integer.cpp foundations/conversions/member-pointer-to-integer.cpp foundations/translation/ill-formed-diagnosed.cpp

These values must not convert implicitly to an integer type:

- a pointer,
- a pointer to member,
- `nullptr`.

An array, such as a string literal, becomes a pointer first, so the
array does not convert either. The conversion to `bool` is the exception
(CONV-3, CONV-12), and `reinterpret_cast` can convert a pointer
explicitly (CAST-6).
:::

:::{rule} CONV-16
:kind: required
:tests: foundations/conversions/array-pointer-unknown-bound.cpp foundations/conversions/array-pointer-unknown-bound-argument.cpp

A pointer to an array with a known bound converts implicitly to a
pointer to an array of the same element type with no bound.
:::

## Runtime Behavior

:::{rule} CONV-2
:kind: required
:tests: foundations/conversions/to-unsigned-modulo.cpp foundations/conversions/to-signed-modulo.cpp expressions/assignment/compound-narrow-operand-promoted.cpp expressions/bitwise/negative-with-unsigned.cpp

An integer value that converts to another integer type keeps its low N
bits, where N is the width of the target type. The target can be signed
or unsigned. Signed types use two's complement.
:::

:::{rule} CONV-3
:kind: required
:tests: foundations/conversions/to-bool.cpp foundations/conversions/member-pointer-to-bool.cpp foundations/conversions/nan-to-bool.cpp

A conversion to `bool` gives `false` for these values:

- zero and negative zero,
- a null pointer,
- a null pointer to member.

The conversion gives `true` for other values, including a NaN.
:::

:::{rule} CONV-4
:kind: required
:tests: foundations/conversions/floating-to-integral-truncates.cpp foundations/conversions/floating-to-integral-truncates-run.cpp

A floating-point value converts to an integer type by discarding the
fractional part. The result rounds toward zero.
:::

:::{rule} CONV-6
:kind: required
:tests: foundations/conversions/integral-to-floating-exact.cpp foundations/conversions/floating-narrowing-in-range.cpp

When a value converts to a floating-point type that can represent it
exactly, the result is that value.
:::

## Undefined Behavior

:::{rule} CONV-5
:kind: undefined
:tests: foundations/conversions/floating-to-integral-out-of-range.cpp foundations/conversions/floating-to-unsigned-negative.cpp foundations/conversions/negative-fraction-to-unsigned.cpp foundations/conversions/floating-to-integral-sanitizer.cpp

If a floating-point value, without its fractional part, is outside the
range of the target integer type, the conversion is undefined. The
test is on the value after the fractional part is dropped, so a value
between -1 and 0 converts to zero, and -1 to an unsigned type is
undefined.
:::

## Implementation Requirements

:::{rule} CONV-7
:kind: impl-defined
:tests: foundations/conversions/integral-to-floating-rounding.cpp foundations/conversions/integral-to-floating-nearest.cpp foundations/conversions/integral-to-floating-nearest-ilp32.cpp foundations/conversions/integral-to-floating-nearest-run.cpp

If a value is between two adjacent values of the target floating-point
type, the result is one of the two. The implementation defines which
one. The matrix compilers choose in this way:

| Compiler | x86-64 Linux | `-m32` |
|---|---|---|
| Clang, GCC, EDG | the nearer value, and at a tie the value whose last bit of precision is zero ({rule}`TYP-14`) | the same |
:::

:::{rule} CONV-11
:kind: impl-defined
:tests: foundations/conversions/floating-narrowing-out-of-range.cpp foundations/conversions/floating-narrowing-out-of-range-run.cpp foundations/conversions/floating-narrowing-infinity.cpp foundations/conversions/floating-narrowing-infinity-ilp32.cpp foundations/conversions/floating-narrowing-infinity-run.cpp

If the target floating-point type has infinities, a value above the
largest finite value of the type converts to the largest finite value or
to infinity. The implementation defines which one ({question}`Q-0028`).
The conversion is not undefined, so it can occur in {term}`constant evaluation`.
The matrix compilers choose in this way:

| Compiler | x86-64 Linux | `-m32` |
|---|---|---|
| Clang, GCC | infinity | infinity |
| EDG | rejects the conversion during constant evaluation | the same |
:::

## Examples

% tidal-example: reject
```cpp
unsigned u = -1;           // ~0u
unsigned char uc = 300;    // 44: the low 8 bits
signed char sc = 200;      // -56
int i = 3.99;              // 3
int big = 1e10;            // undefined
float f = 16777217;        // 16777216.0f or 16777218.0f
float g = 16777219;        // 16777220.0f on every matrix target
long n = &i;               // rejected: a pointer is not a number
int* p = 0;                // a null pointer
int zero = 0;
int* q = zero;             // rejected
bool b = nullptr;          // rejected
bool c(nullptr);           // false

decltype(1u + -1) w;       // unsigned: step 4 of CONV-19
decltype('a' + 'b') x;     // int: step 2 of CONV-19

unsigned char byte = 200;
unsigned total = 4000000000u;
decltype(total + byte) y;  // unsigned: byte becomes int, then step 4
total += byte;             // 4000000200, with no signed step
```

## Evidence

The results are in `tests/RESULTS.md`.

### Divergences

- {question}`Q-0100` (CONV-8; settled on CONV-P3): whether a character
  literal with the value zero is a null pointer constant. It is not, as
  Clang and GCC say. EDG accepts it.
- {question}`Q-0331` (LIST-2, CONV-21; open): which values an unscoped
  enumeration without a fixed type has for narrowing. The values that
  {rule}`ENUM-22` gives, as GCC uses. So GCC accepts
  `unsigned char c{e};` when the enumerators of `e`'s type are 0 and
  100. Clang and EDG use every value of the underlying type and reject
  the program.
- {question}`Q-0042` (LIST-2, CONV-21; settled on LIST-P1): whether a
  bit-field whose values all fit the target is narrowed. It is not, as
  Clang 22 and GCC 16 say. EDG rejects the initialization, and so do
  Clang 18 and GCC 14.
- {question}`Q-0028` (CONV-11; open): whether a floating value above
  the target's finite range gives an implementation-defined result. It
  does, and Clang and GCC give infinity, during constant evaluation and
  at run time. EDG rejects the conversion during constant evaluation.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `declarations/initialization/list/narrowing-bit-field-fits.cpp` | LIST-2, CONV-21 | accept | Clang 22: accept; Clang 18: reject | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0042` |
| `declarations/initialization/list/narrowing-unscoped-enum-values-fit.cpp` | LIST-2, CONV-21 | accept | reject | accept | reject | {question}`Q-0331` |
| `char-zero-not-null.cpp` | CONV-8 | reject | reject | reject | accept | {question}`Q-0100` |
| `floating-narrowing-infinity-ilp32.cpp` | CONV-11 | accept | accept | accept | reject | {question}`Q-0028` |
| `floating-narrowing-infinity.cpp` | CONV-11 | accept | accept | accept | reject | {question}`Q-0028` |
| `floating-narrowing-out-of-range.cpp` | CONV-11 | accept | accept | accept | reject | {question}`Q-0028` |

### Notes

- CONV-5: all compilers reject the conversion during constant
  evaluation. At run time, the sanitizer
  (`-fsanitize=float-cast-overflow`) stops the program with a nonzero
  status on Clang and GCC.
- CONV-7: every compiler in the matrix rounds to the nearest value, and
  to the even value at a tie, during constant evaluation and at run
  time. {rule}`TYP-14` gives the same rounding for floating-point
  arithmetic. The tests convert from `int` and from `double` to
  `float`.
- CONV-11: EDG builds no program, so its run-time choice is not
  observed. The base expectation of each test follows Clang and GCC,
  and the tests record EDG's choice as an override.
- CONV-7, CONV-11 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. The
  `-m32` tests check the choices during constant evaluation. In the
  writer's probes, the run tests give the same results with `-m32` in
  the GCC 14, Clang 18, and Clang 22 containers.

### Sources

Inputs: clang-sema-expr, clang-diag-sema.

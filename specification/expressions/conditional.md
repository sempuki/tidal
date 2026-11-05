# The conditional operator

The operator `?:` evaluates a condition. Then the operator evaluates and
gives one of two other operands, as the condition selects.

## Principles

:::{principle} COND-P1
:rules: COND-1 COND-2 COND-6 COND-7 COND-10 COND-14 COND-15

`?:` is a selection. The condition is converted to `bool` by {term}`contextual conversion`
and evaluated first, with all its {term}`side effects <side effect>`. Then exactly
one of the two other operands is evaluated, and the other is not touched
at all:

```text
   c ? a : b

   evaluate c, with all its side effects
        |
        +-- true  --> evaluate a --> result   (b is not touched)
        |
        +-- false --> evaluate b --> result   (a is not touched)
```

So during constant evaluation only the selected operand has to
be constant. The operator groups to the right, and its middle operand
is a {term}`full expression` that can hold a comma or an assignment without
parentheses.
:::

:::{principle} COND-P2
:rules: COND-3 COND-4 COND-5 COND-11 COND-13 COND-16

Two operands have to give one result, with one type and one value
category, and the result type and category are decided before either
operand is evaluated.

- If both are lvalues of one type apart from
  {term}`cv-qualifiers <cv-qualifier>`, and the cv-qualifiers of one
  include those of the other, the result is an lvalue with the larger
  set. A `const Box` lvalue and a `Box` lvalue give a `const Box`
  lvalue.
- If neither set includes the other, as for a `const` and a `volatile`
  lvalue, the result depends on the type. Two such lvalues of a
  non-class type give a prvalue. Two such lvalues of one class type
  are rejected. A class operand converts toward the other operand only
  when the other has every cv-qualifier of its own, so neither operand
  converts.
- An operand that throws takes no part, because it gives no value.
- A `void` operand can pair only with another `void`.
- If two class types each convert to the other, there is no single
  answer, and the expression is rejected. A conversion counts in this
  check when it can be formed, even if its function is deleted or not
  accessible ({question}`Q-0378`). So a `Wrap` with a deleted
  `operator Tag()` still converts to `Tag`.
- Otherwise the result is a prvalue, and each operand is converted to
  the result type. Operands of one type keep that type. Other
  arithmetic operands convert by the
  {term}`usual arithmetic conversions`.

A null pointer constant has no pointer type of its own. So a pointer and
a null pointer constant give the pointer type, and `nullptr` with
`nullptr` or with the literal `0` gives the type of `nullptr`.

Operands of different types, at least one of them a class, first try to
convert one operand to the type of the other. If neither converts, the
operator goes to built-in candidates, as an overloaded operator does
({principle}`OVL-P4`). Overload resolution picks the candidate, and the
class operands reach it through their conversion functions. The
candidates are these:

- `(L, R)` for every pair of promoted arithmetic types, `float` among
  them, with the result of the usual arithmetic conversions,
- `(T, T)` for each pointer type, pointer-to-member type, and scoped
  enumeration type `T`, with the result `T`.

An unscoped enumeration reaches the arithmetic candidates through its
promotion, so the enumeration has no candidate of its own.
:::

:::{principle} COND-P3
:rules: COND-8 COND-9 COND-12

The selected operand becomes the result with as little copying as the
categories allow. A prvalue result of class type initializes its target
directly ({principle}`VC-P2`), so no copy happens when both operands
are prvalues, and a copy is needed only when one of them is an lvalue.
An lvalue result that names a bit-field keeps the limits of a
bit-field.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   conditional_expr: `logical_or_expr` ["?" `expression` ":" `assignment_expr`]
```

## Constraints

:::{rule} COND-1
:kind: reject
:tests: expressions/conditional/condition-converted.cpp expressions/conditional/condition-not-convertible.cpp

The first operand must be convertible to `bool` by {term}`contextual
conversion`.
:::

:::{rule} COND-3
:kind: required
:tests: expressions/conditional/lvalue-result.cpp expressions/conditional/arithmetic-prvalue.cpp expressions/conditional/same-type-prvalue.cpp expressions/conditional/array-function-operands.cpp expressions/conditional/mixed-category-prvalue.cpp expressions/conditional/pointer-operands.cpp expressions/conditional/bitfield-or-enum.cpp expressions/conditional/class-one-way.cpp expressions/conditional/mixed-enum-types.cpp expressions/conditional/cv-lvalues-and-strings.cpp expressions/conditional/two-zeros.cpp expressions/conditional/member-pointer-nullptr.cpp expressions/conditional/xvalue-operands.cpp expressions/conditional/const-volatile-lvalues-prvalue.cpp expressions/conditional/const-and-plain-lvalue.cpp expressions/conditional/derived-base-lvalue.cpp expressions/conditional/class-converts-to-int.cpp expressions/conditional/zero-and-nullptr.cpp expressions/conditional/class-converts-best-conversion.cpp expressions/conditional/class-converts-to-reference.cpp expressions/conditional/class-base-target-keeps-cv.cpp expressions/conditional/class-base-does-not-convert-to-derived.cpp expressions/conditional/class-const-and-volatile-lvalues.cpp expressions/conditional/class-converts-to-decayed-array.cpp expressions/conditional/class-same-type-toward-more-qualified.cpp expressions/conditional/nonclass-converts-to-class.cpp

The second and third operands give one result type and
{term}`value category`. The first row that matches the operands gives
the result. For the rows after the first four, an array operand becomes
a pointer to its first element, a function operand becomes a pointer to
the function, and the cv-qualifiers of a non-class operand are removed.

| Operands | Result |
|---|---|
| Lvalues of the same type apart from cv-qualifiers, where the cv-qualifiers of one include those of the other | An lvalue with the larger set |
| Lvalues of the same non-class type apart from cv-qualifiers, where neither set includes the other, as for `const int` and `volatile int` | A prvalue. Two such lvalues of class type go to the last row ({question}`Q-0378`). |
| Two xvalues | An xvalue |
| An lvalue and an xvalue | A prvalue |
| Two operands of the same type | A prvalue of that type, with no promotion. `true ? 'a' : 'b'` has type `char`, and two enumerators of `E` give `E`. |
| Both of arithmetic or enumeration type | A prvalue of the type that the {term}`usual arithmetic conversions` give. A `char` and a `short` give `int`. |
| A pointer, and a pointer or a null pointer constant | A pointer prvalue with the combined cv-qualifiers |
| Two `nullptr`, or `nullptr` and the literal `0` | The type of `nullptr` ({question}`Q-0102`) |
| A pointer to member and `nullptr` | A pointer to member |
| Operands of different types, at least one of class type, where exactly one operand converts for the other (COND-16) | The result of the conversion, with the rows above applied again. A derived class lvalue and a base class lvalue give a base class lvalue. A class that converts to `int` and an `int` give an `int` prvalue. An `int` and a class with a constructor from `int` give a prvalue of the class. |
:::

:::{rule} COND-4
:kind: required
:tests: expressions/conditional/throw-operand.cpp expressions/conditional/void-operands.cpp

A throw-expression operand does not affect the result. In that case the
other operand gives the result its type and category. If both operands
have type `void`, the result is `void`.
:::

:::{rule} COND-13
:kind: reject
:tests: expressions/conditional/void-and-value.cpp expressions/conditional/value-and-void.cpp

Apart from a throw-expression (COND-4), a `void` operand with a value
operand is not valid.
:::

:::{rule} COND-5
:kind: reject
:tests: expressions/conditional/no-common-type.cpp expressions/conditional/pointer-and-int.cpp expressions/conditional/class-ambiguous.cpp expressions/conditional/class-ambiguous-conversion.cpp expressions/conditional/two-classes-no-candidate.cpp expressions/conditional/two-classes-several-conversions.cpp expressions/conditional/class-base-less-qualified-rejected.cpp expressions/conditional/class-base-const-derived-prvalue-rejected.cpp expressions/conditional/class-const-and-volatile-lvalues.cpp expressions/conditional/class-deleted-conversion-counts.cpp expressions/conditional/class-private-conversion-counts.cpp expressions/conditional/nonclass-and-class-both-ways.cpp

The second and third operands must have a common type by COND-3 or
COND-4. If each operand converts for the other by COND-16, the
expression is ambiguous and not valid. If neither converts, and COND-11
finds no candidate, the expression is not valid.
:::

:::{rule} COND-6
:kind: required
:tests: expressions/conditional/grouping.cpp expressions/conditional/precedence-assignment.cpp

The operator groups from right to left. The third operand can be an
assignment.
:::

:::{rule} COND-14
:kind: reject
:tests: expressions/conditional/gnu-omitted-operand.cpp expressions/conditional/gnu-omitted-operand-constant.cpp

The second operand of the conditional operator must be present (COND-6).
:::

:::{rule} COND-7
:kind: required
:tests: expressions/conditional/constant-unselected-fails.cpp expressions/conditional/constant-unselected-null-dereference.cpp

During {term}`constant evaluation`, the operand that is not selected can
contain any expression.
:::

:::{rule} COND-15
:kind: reject
:tests: expressions/conditional/constant-selected-fails.cpp expressions/conditional/constant-selected-division-by-zero.cpp

During {term}`constant evaluation`, the selected operand must be a
constant expression. COND-7 gives the operand that is not selected.
:::

:::{rule} COND-8
:kind: reject
:tests: expressions/conditional/class-lvalue-prvalue-copy.cpp expressions/conditional/class-prvalue-lvalue-private-copy.cpp

If one operand is an lvalue and the other a prvalue of the same class
type, the result is a prvalue copied from the selected operand. The copy
must be valid, so a class with a deleted copy constructor is rejected.
:::

:::{rule} COND-9
:kind: reject
:tests: expressions/conditional/bitfield-address.cpp expressions/conditional/bitfield-address-unevaluated.cpp

An lvalue result that refers to a bit-field operand ({rule}`BF-1`) has
the limits of a bit-field. A program must not apply `&` to the result.
:::

:::{rule} COND-10
:kind: required
:tests: expressions/conditional/comma-in-middle.cpp expressions/conditional/comma-in-middle-assignments.cpp

The second operand can be a comma expression without parentheses.
:::

:::{rule} COND-11
:kind: required
:tests: expressions/conditional/two-classes-convert-to-int.cpp expressions/conditional/two-classes-convert-to-pointer.cpp expressions/conditional/two-classes-promoted-types.cpp expressions/conditional/two-classes-several-conversions.cpp expressions/conditional/two-classes-no-candidate.cpp expressions/conditional/two-classes-scoped-enumeration.cpp expressions/conditional/two-classes-float.cpp

Overload resolution ({rule}`OVL-9`) over the built-in
operator candidates picks a conversion for each operand if all of these
hold:

- the operands have different types,
- at least one is a class,
- neither converts to the other (COND-16).

The candidates are these, and each gives a prvalue
({question}`Q-0373`):

| Parameters | Result |
|---|---|
| `(L, R)` for every pair of {term}`promoted arithmetic types <promoted arithmetic type>` `L` and `R` | the type that the {term}`usual arithmetic conversions` give |
| `(T, T)` for each pointer type, pointer-to-member type, and scoped enumeration type `T` | `T` |

If no candidate is viable, or no candidate is better than all others,
the expression is not valid ({rule}`COND-5`).
:::

:::{rule} COND-16
:kind: required
:tests: expressions/conditional/class-converts-to-reference.cpp expressions/conditional/class-converts-best-conversion.cpp expressions/conditional/class-ambiguous-conversion.cpp expressions/conditional/class-base-less-qualified-rejected.cpp expressions/conditional/class-base-const-derived-prvalue-rejected.cpp expressions/conditional/class-base-does-not-convert-to-derived.cpp expressions/conditional/class-base-target-keeps-cv.cpp expressions/conditional/class-const-and-volatile-lvalues.cpp expressions/conditional/class-converts-to-decayed-array.cpp expressions/conditional/class-deleted-conversion-counts.cpp expressions/conditional/class-private-conversion-counts.cpp expressions/conditional/class-same-type-toward-more-qualified.cpp expressions/conditional/nonclass-and-class-both-ways.cpp expressions/conditional/nonclass-converts-to-class.cpp expressions/conditional/class-derived-prvalue-against-base-xvalue.cpp

For COND-3, COND-5, and COND-11, when the two operands have different
types and at least one has class type, each operand `X` is tried
against the other operand `Y`. `T` is the type of `Y`. The first of
these steps that applies decides whether `X` converts:

1. If `Y` is an lvalue, and a `T&` {term}`binds directly <bind directly>`
   to `X`, `X` converts to an lvalue of type `T`.
2. If `Y` is an xvalue, and a `T&&` binds directly to `X`, `X` converts
   to an xvalue of type `T`.
3. Otherwise `X` converts to a prvalue of the target in this table, if
   `X` has an {term}`implicit conversion sequence` to the target.

In steps 1 and 2, a reference does not bind directly to a class
prvalue, because the prvalue first becomes a temporary object
({question}`Q-0401`). So a `Derived`
prvalue against a `Base` xvalue converts by step 3, to a prvalue of
`Base`.

| `X` and `Y` | Target |
|---|---|
| One class type apart from cv-qualifiers | `T`, if `T` has every cv-qualifier of the type of `X`. Otherwise `X` does not convert. |
| `T` is a base class of the class of `X` | That base class with the cv-qualifiers of `X` |
| The class of `X` is a base class of the class of `T` | None. `X` does not convert, and a converting constructor of the derived class takes no part. |
| Any other pair | `T` after an array becomes a pointer, a function becomes a pointer, and a non-class type drops its cv-qualifiers |

So a `const Box` lvalue and a `Box` prvalue give a `const Box`
prvalue, because only the `Box` operand converts. For a `const Derived`
lvalue and a `Base` prvalue, the `Derived` operand converts to a
`const Base` prvalue, and the `Base` operand does not convert. The two
operands still differ in cv-qualifiers, no row of COND-3 gives them one
type, and the expression is not valid (COND-5).

Step 3 asks only whether a conversion sequence can be formed. Access
control, a bit-field operand, and a deleted conversion function do not
stop it ({question}`Q-0378`). A
conversion that counts and then cannot be used, such as a private
conversion function, makes the expression not valid.

If both operands convert, the expression is not valid (COND-5). If the
implicit conversion sequence of step 3 is ambiguous, the expression is
not valid.
:::

## Runtime Behavior

:::{rule} COND-2
:kind: required
:tests: expressions/conditional/only-selected-evaluated.cpp expressions/conditional/condition-first.cpp expressions/conditional/condition-side-effects-first.cpp

The condition is evaluated first, with all its side effects. Then
exactly one of the other operands is evaluated. The operand that is not
selected is not evaluated.
:::

:::{rule} COND-12
:kind: required
:tests: expressions/conditional/prvalue-no-copy.cpp expressions/conditional/prvalue-no-copy-argument.cpp

If both operands are prvalues of the same class type, the result is a
prvalue that initializes its target directly. No copy or move
constructor is called, so the class needs none.
:::

## Examples

% tidal-example: reject
```cpp
int x = 1, y = 2;
(c ? x : y) = 10;              // fine: both are lvalues of type int
auto a = c ? 1 : 2.5;          // double
auto k = c ? 'a' : 'b';        // char: one type, no promotion
auto s = c ? "ab" : "xyz";     // const char*
auto p = c ? &x : nullptr;     // int*
int& r = c ? x : throw 1;      // the type of x
int v = n == 0 ? 10 : n == 1 ? 11 : 12;   // groups as n == 0 ? 10 : (...)
auto bad = c ? &x : 1;         // rejected: no common type

const int& larger(const int& a, const int& b) { return a < b ? b : a; }
const int& top = larger(1, 2); // dangles: the temporaries 1 and 2 end
                               // with the full expression (REF-5)
int m = larger(x, y);          // fine: x and y outlive the call
```

## Evidence

All eight compilers agree on 62 of 68 tests. The results are in
`tests/RESULTS.md`.

### Divergences

- {question}`Q-0102` (COND-3; settled on COND-P2): whether a
  conditional with the literal `0` and `nullptr` has the type of
  `nullptr`. It has, as Clang and EDG say. GCC rejects the conditional.
- {question}`Q-0373` (COND-11; settled on COND-P2): which built-in
  candidates `?:` has for two class operands that convert to a scoped
  enumeration or to `float`. Every pair of promoted arithmetic types,
  `float` among them, and one candidate for each scoped enumeration, as
  Clang says. GCC rejects the scoped enumeration, and EDG gives
  `double` for `float`.
- {question}`Q-0378` (COND-16, COND-3, COND-5; open): which class
  operands of `?:` convert in two forms. A deleted conversion function
  counts, as GCC and EDG say, and Clang leaves it out and accepts. A
  `const` and a `volatile` lvalue of one class type do not convert, as
  Clang and GCC say, and EDG accepts.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `class-const-and-volatile-lvalues.cpp` | COND-16, COND-3, COND-5 | reject | reject | reject | accept | {question}`Q-0378` |
| `class-deleted-conversion-counts.cpp` | COND-16, COND-5 | reject | accept | reject | reject | {question}`Q-0378` |
| `class-derived-prvalue-against-base-xvalue.cpp` | COND-16 | accept | accept | reject | accept | |
| `two-classes-float.cpp` | COND-11 | accept | accept | accept | reject | {question}`Q-0373` |
| `two-classes-scoped-enumeration.cpp` | COND-11 | accept | accept | reject | accept | {question}`Q-0373` |
| `zero-and-nullptr.cpp` | COND-3 | accept | accept | reject | accept | {question}`Q-0102` |

### Notes

- COND-6: Clang and GCC accept `a ?: b` as an extension, and reject the
  form under `-pedantic-errors`. EDG rejects the form in all modes.
- COND-3: operands of two different enumeration types give `int`. Clang
  warns that this use is deprecated (`-Wdeprecated-enum-compare-conditional`).
  GCC warns of an enumeration mismatch (`-Wenum-compare`). Both warnings
  are on by default, and `-pedantic-errors` does not make them errors.
  Every compiler accepts the program in strict mode.
- COND-11: every compiler gives `int` for two classes that convert to
  one unscoped enumeration.
- COND-16: the gap reviews of the text phase 7 wrote
  (`reports/review/2026-11-04-gap-review-phase-7-additions.md` and
  `reports/review/2026-11-04-gap-review-phase-7-repairs.md`) read the
  three targets of step 3, the conversion of a non-class operand to a
  class, and the test that ignores access and deleted functions. Every
  newest compiler gives the targets of the table, converts an `int` to
  a class through a constructor, and counts a private conversion
  function.
- COND-16: the gap review of the text phase 8 wrote on expressions
  (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`)
  reads the base-class row with no cv condition. The probes found no
  program where the condition changes the result, because a base class
  operand never converts to its derived class on any newest compiler,
  not even through a converting constructor. The row follows the review,
  and the new row records the base class operand.

### Sources

Inputs: clang-sema-expr, clang-diag-sema.

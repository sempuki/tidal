# Logical operators

The operators `&&` and `||` combine two `bool` values, and `!`
negates one. `&&` and `||` evaluate their right operand only if the left
operand does not decide the result. These rules apply to the built-in
operators. An overloaded operator is a function call and evaluates both
operands *(overload resolution referenced)*.

## Syntax

```{eval-rst}
.. productionlist:: logical
   logical_or: `logical_and` {"||" `logical_and`}
   logical_and: `inclusive_or` {"&&" `inclusive_or`}
   logical_not: "!" `unary_expr`
```

## Constraints

:::{rule} LOG-1
:kind: required
:tests: expressions/logical/result-type.cpp

The result of `&&`, `||`, and `!` has type `bool` and is a prvalue.
:::

:::{rule} LOG-2
:kind: required
:tests: expressions/logical/contextual-conversion.cpp

Each operand is converted to `bool` by {term}`contextual conversion`.
Arithmetic values, pointers, and classes with an `explicit operator bool`
are valid operands.
:::

:::{rule} LOG-3
:kind: reject
:tests: expressions/logical/operand-not-convertible.cpp expressions/logical/operand-scoped-enum.cpp expressions/logical/operand-void.cpp

Each operand must be convertible to `bool` by contextual conversion. A
class without a suitable conversion, a scoped enumeration, and `void` are
not valid operands.
:::

:::{rule} LOG-6
:kind: required
:tests: expressions/logical/precedence.cpp

`!` is a unary operator and binds more tightly than `==`. `&&` binds
more tightly than `||`. Both bind less tightly than `|` and group from
left to right.
:::

:::{rule} LOG-7
:kind: required
:tests: expressions/logical/constant-unevaluated-division.cpp expressions/logical/constant-unevaluated-call.cpp expressions/logical/constant-evaluated-call.cpp

During {term}`constant evaluation`, a right operand that is not evaluated
has no effect on the result. It can contain an operation that constant
evaluation does not permit. If the right operand is evaluated, such an
operation makes the expression not constant.
:::

## Dynamic Semantics

:::{rule} LOG-10
:kind: required
:tests: expressions/logical/values.cpp

`a && b` is `true` if both operands are `true`. `a || b` is `true`
if one or both operands are `true`. `!a` is `true` if `a` is
`false`.
:::

:::{rule} LOG-4
:kind: required
:tests: expressions/logical/short-circuit-and.cpp expressions/logical/short-circuit-or.cpp

If the left operand of `&&` is `false`, the right operand is not
evaluated. If the left operand of `||` is `true`, the right operand is
not evaluated.
:::

:::{rule} LOG-5
:kind: required
:tests: expressions/logical/left-before-right.cpp

The left operand is evaluated completely, with all its side effects,
before the right operand starts.
:::

:::{rule} LOG-11
:kind: required
:tests: expressions/logical/overloaded-and-evaluates-both.cpp

An overloaded `&&` or `||` *(referenced)* evaluates both operands. The
left operand is evaluated before the right operand.
:::

## Recommended Practice

:::{rule} LOG-8
:kind: recommended
:tests: expressions/logical/parentheses-warns.cpp expressions/logical/parentheses-silences.cpp

An implementation should warn if `&&` is an operand of `||` without
parentheses.
:::

:::{rule} LOG-9
:kind: recommended
:tests: expressions/logical/not-on-lhs-warns.cpp expressions/logical/not-on-lhs-silences.cpp

An implementation should warn if `!` applies only to the left operand of a
comparison, as in `!a == b`.
:::

## Examples

```cpp
int* p = find();
if (p && p->ready) use(p);      // p->ready runs only if p is not null

constexpr bool safe(int d) { return d != 0 && 10 / d > 1; }
static_assert(!safe(0));        // 10 / 0 is not evaluated

int i = 0;
(i++ == 0) && (i == 1)          // true: i++ is complete first

a || b && c                     // a || (b && c); compilers warn
!a == b                         // (!a) == b; compilers warn
```

## Evidence

All seven compilers agree on all 17 tests. The results are in
`tests/RESULTS.md`.

- LOG-8: Clang warns under `-Wlogical-op-parentheses` and GCC under
  `-Wparentheses`. The test uses `-Wparentheses`, which includes both.
- LOG-9: Clang and GCC warn under `-Wlogical-not-parentheses`.
- EDG doesn't take these warning flags and skips the recommended-practice
  tests.

Inputs: clang-sema-expr, clang-diag-sema.

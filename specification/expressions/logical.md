# Logical operators

The operators `&&` and `||` combine two `bool` values, and `!`
negates one. `&&` and `||` evaluate their right operand only if the left
operand does not decide the result. These rules apply to the built-in
operators. An overloaded operator is a function call and evaluates both
operands ({rule}`OPR-7`).

## Principles

:::{principle} LOG-P1
:rules: LOG-1 LOG-2 LOG-3 LOG-6 LOG-10 LOG-12 LOG-13

The logical operators work on truth values. Each operand is converted to
`bool` by contextual conversion, the same conversion a condition uses.
So each of these is a valid operand:

- any arithmetic value or unscoped enumeration value,
- a pointer, or an array or a function, which converts to a pointer,
- a pointer to member,
- `nullptr`,
- a class with an `explicit operator bool`.

Anything that does not convert that way is rejected. The result is a
`bool` prvalue ({principle}`VC-P1`). `&&` binds more tightly than `||`,
both bind less tightly than the bitwise operators, and each operator has
an alternative spelling as a word.
:::

:::{principle} LOG-P2
:rules: LOG-4 LOG-5 LOG-7 LOG-14
:exceptions: LOG-11

The built-in `&&` and `||` evaluate the left operand first, completely,
and evaluate the right operand only when the left one has not already
decided the result:

```text
   a && b                        a || b

   evaluate a fully              evaluate a fully
     |                             |
     +-- false --> done            +-- true  --> done
     |                             |
     +-- true  --> evaluate b      +-- false --> evaluate b
```

So the right operand can depend on the left one
having been true or false, and during constant evaluation an operation
that is never reached is never checked. An overloaded `&&` or `||` is
an ordinary function call, so it evaluates both operands, left first.
:::

:::{principle} LOG-P3
:rules: LOG-8 LOG-9

An implementation should warn about `&&` inside `||` without
parentheses, and about `!` applied to only the left side of a
comparison. In both forms, precedence suggests a mistake.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   logical_and_expr: `or_expr` {"&&" `or_expr`}
   logical_or_expr: `logical_and_expr` {"||" `logical_and_expr`}
```

## Constraints

:::{rule} LOG-1
:kind: required
:tests: expressions/logical/result-type.cpp expressions/logical/result-prvalue.cpp

The result of `&&`, `||`, and `!` has type `bool` and is a prvalue.
:::

:::{rule} LOG-2
:kind: required
:tests: expressions/logical/contextual-conversion.cpp expressions/logical/operand-enum-and-conversion.cpp expressions/logical/operand-array-and-function.cpp

Each operand is converted to `bool` by {term}`contextual conversion`.
These are valid operands:

- arithmetic values and values of unscoped enumerations,
- pointers, and arrays and functions, which convert to pointers,
- classes with a conversion function to `bool` or to one of these
  types, including an `explicit operator bool`.
:::

:::{rule} LOG-3
:kind: reject
:tests: expressions/logical/operand-not-convertible.cpp expressions/logical/operand-scoped-enum.cpp expressions/logical/operand-void.cpp expressions/logical/operand-ambiguous-conversion.cpp expressions/logical/operand-throw.cpp

Each operand must be convertible to `bool` by contextual conversion.
These are not valid operands:

- a class without a suitable conversion,
- a class whose conversion to `bool` is ambiguous,
- a scoped enumeration,
- an expression of type `void`, such as a call or a `throw`
  expression.
:::

:::{rule} LOG-6
:kind: required
:tests: expressions/logical/precedence.cpp expressions/logical/alternative-tokens-precedence.cpp

`!` is a unary operator and binds more tightly than `==`. `&&` binds
more tightly than `||`. Both bind less tightly than `|` and group from
left to right.
:::

:::{rule} LOG-7
:kind: required
:tests: expressions/logical/constant-unevaluated-division.cpp expressions/logical/constant-unevaluated-call.cpp

During {term}`constant evaluation`, a right operand that is not
evaluated has no effect on the result. The right operand can contain an
operation that constant evaluation does not permit.
:::

:::{rule} LOG-14
:kind: reject
:tests: expressions/logical/constant-evaluated-call.cpp expressions/logical/constant-evaluated-division.cpp

During {term}`constant evaluation`, if the right operand is evaluated,
an operation that constant evaluation does not permit makes the
expression not constant (LOG-7).
:::

:::{rule} LOG-12
:kind: required
:tests: expressions/logical/nullptr-operand.cpp expressions/logical/member-pointer-operand.cpp

An operand of `!`, `&&`, or `||` can be `nullptr` or a pointer to
member. The operand converts to `bool` by {term}`contextual conversion`,
so `nullptr` and a null pointer to member give `false`
({question}`Q-0101`).
:::

:::{rule} LOG-13
:kind: required
:tests: expressions/logical/alternative-tokens.cpp expressions/logical/alternative-tokens-precedence.cpp expressions/logical/alternative-tokens-in-declarators.cpp

Each word in the table is another spelling of an operator, and no
header is needed ({rule}`KW-2`). A word is the same token as its
operator in every context, so `int and r = f();` declares an rvalue
reference.

| Word | Operator | Word | Operator |
|---|---|---|---|
| `and` | `&&` | `bitand` | `&` |
| `or` | `\|\|` | `bitor` | `\|` |
| `not` | `!` | `xor` | `^` |
| `not_eq` | `!=` | `compl` | `~` |
| `and_eq` | `&=` | `or_eq` | `\|=` |
| `xor_eq` | `^=` | | |
:::

## Runtime Behavior

:::{rule} LOG-10
:kind: required
:tests: expressions/logical/values.cpp expressions/logical/values-at-run-time.cpp

- `a && b` is `true` if both operands are `true`.
- `a || b` is `true` if one or both operands are `true`.
- `!a` is `true` if `a` is `false`.
:::

:::{rule} LOG-4
:kind: required
:tests: expressions/logical/short-circuit-and.cpp expressions/logical/short-circuit-or.cpp expressions/logical/short-circuit-chain.cpp

If the left operand of `&&` is `false`, the right operand is not
evaluated. If the left operand of `||` is `true`, the right operand is
not evaluated.
:::

:::{rule} LOG-5
:kind: required
:tests: expressions/logical/left-before-right.cpp expressions/logical/temporaries-end-with-full-expression.cpp

The left operand is evaluated completely, with all its side effects,
before the right operand starts. A temporary that the left operand
creates lives to the end of the full expression ({rule}`VC-6`).
:::

:::{rule} LOG-11
:kind: required
:tests: expressions/logical/overloaded-and-evaluates-both.cpp expressions/logical/overloaded-or-evaluates-both.cpp expressions/logical/overloaded-left-first.cpp

An overloaded `&&` or `||` ({rule}`OPR-7`) evaluates both operands. The
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

An implementation should warn if `!` applies only to the left operand of
a comparison, as in `!a == b`.
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

if (Lock(m).held() && ready())  // the Lock lives to the end of the condition
  run();
int and moved = make();         // int&& moved: and is the token &&
```

## Evidence

All eight compilers agree on 33 of 34 tests. The results are in
`tests/RESULTS.md`.

- LOG-8: Clang warns under `-Wlogical-op-parentheses` and GCC under
  `-Wparentheses`. The test uses `-Wparentheses`, which includes both.
- LOG-9: Clang and GCC warn under `-Wlogical-not-parentheses`.
- EDG doesn't take these warning flags and skips the
  recommended-practice tests.
- LOG-12: GCC 14 rejects `nullptr` as an operand. GCC 16, Clang, and EDG
  accept it. GCC 14 has a defect that GCC 16 fixes ({question}`Q-0101`).
- LOG-2: Clang warns that an array or a function as an operand always
  converts to `true`, and GCC warns under `-Waddress`. Every compiler
  accepts the program.
- LOG-5: every compiler destroys the temporaries of both operands after
  the right operand, in the reverse order of their creation.

Inputs: clang-sema-expr, clang-diag-sema.

# Postfix expressions

A postfix expression applies an operator after its operand. The postfix
expressions are subscripts, function calls, member access, and postfix
`++` and `--`.

## Syntax

```{eval-rst}
.. productionlist:: expr_postfix
   postfix_expr: `primary_expr`
               : `postfix_expr` "[" `expression` "]"
               : `postfix_expr` "(" [`expression_list`] ")"
               : `postfix_expr` ("." | "->") `name`
               : `postfix_expr` ("++" | "--")
```

## Static Semantics

:::{rule} PFX-1
:kind: required
:tests: expressions/postfix/subscript-is-pointer-arithmetic.cpp expressions/postfix/subscript-enum-index.cpp

The built-in subscript `a[i]` is the same as `*(a + i)`. The operands can
be in either order, so `i[a]` is also valid. The index can be an integer
or an unscoped enumeration.
:::

:::{rule} PFX-2
:kind: reject
:tests: expressions/postfix/subscript-two-integers.cpp expressions/postfix/subscript-floating-index.cpp expressions/postfix/subscript-scoped-enum-index.cpp

For a built-in subscript, one operand must be an array or a pointer to an
object type. The other operand must have integer or unscoped enumeration
type. A top-level comma in a subscript is covered by COMMA-6.
:::

:::{rule} PFX-3
:kind: required
:tests: expressions/postfix/subscript-categories.cpp expressions/postfix/subscript-prvalue-array.cpp

A subscript of an array lvalue is an lvalue. A subscript of an array
prvalue or xvalue is an xvalue.
:::

:::{rule} PFX-5
:kind: reject
:tests: expressions/postfix/call-argument-count.cpp expressions/postfix/call-too-many-arguments.cpp expressions/postfix/call-defaults-and-ellipsis.cpp

A function call must give one argument for each parameter that has no
default argument. It must not give more arguments than parameters, unless
the function has an ellipsis.
:::

:::{rule} PFX-6
:kind: required
:tests: expressions/postfix/call-copy-initializes.cpp expressions/postfix/call-explicit-constructor.cpp

Each parameter is copy-initialized from its argument. An `explicit`
constructor cannot do this conversion.
:::

:::{rule} PFX-9
:kind: required
:tests: expressions/postfix/call-result-category.cpp

A call to a function that returns an lvalue reference is an lvalue. A call
to a function that returns an rvalue reference is an xvalue. Other calls
are prvalues.
:::

:::{rule} PFX-10
:kind: required
:tests: expressions/postfix/member-access.cpp expressions/postfix/member-of-prvalue.cpp

`E.m` names the member `m` of the class object `E`. `E->m` is the same as
`(*E).m`. A static member can be named through an object. A member of a
prvalue is an xvalue.
:::

:::{rule} PFX-11
:kind: reject
:tests: expressions/postfix/arrow-on-non-pointer.cpp expressions/postfix/dot-on-pointer.cpp expressions/postfix/no-such-member.cpp

The operand of `.` must be a class object, and the operand of `->` must be
a pointer to a class. The class must have the named member.
:::

:::{rule} PFX-13
:kind: reject
:tests: expressions/postfix/postfix-result-prvalue.cpp expressions/postfix/postfix-not-lvalue.cpp expressions/postfix/postfix-on-const.cpp expressions/postfix/postfix-on-bool.cpp expressions/postfix/postfix-decrement-bool.cpp expressions/postfix/postfix-on-void-pointer.cpp

The operand of postfix `++` or `--` must be a modifiable lvalue. It must
not have type `bool` or be a pointer to `void`. The result is a prvalue.
:::

## Dynamic Semantics

:::{rule} PFX-7
:kind: required
:tests: expressions/postfix/call-through-pointer.cpp

A call through a pointer to a function calls that function. `p(a)` and
`(*p)(a)` have the same effect.
:::

:::{rule} PFX-12
:kind: required
:tests: expressions/postfix/postfix-increment.cpp

Postfix `++` adds one to its operand, and postfix `--` subtracts one. The
result is the value before the change. On a pointer, the change is one
element.
:::

## Undefined Behavior

:::{rule} PFX-4
:kind: undefined
:tests: expressions/postfix/subscript-out-of-range.cpp

A subscript that reads an element outside the array is undefined.
:::

:::{rule} PFX-8
:kind: undefined
:tests: expressions/postfix/call-null-pointer.cpp

A call through a null function pointer is undefined.
:::

## Examples

```cpp
int a[3] = {10, 20, 30};
a[1];             // 20
1[a];             // 20
a[3];             // undefined

int f(int a, int b = 2);
f(1);             // f(1, 2)
f();              // rejected

struct S { int v; };
S obj{3};
S* ptr = &obj;
ptr->v;           // same as (*ptr).v

int i = 5;
int old = i++;    // old is 5, i is 6
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 27 of
28 tests.

- PFX-3: EDG gives `int&` for a subscript of an array member of a
  prvalue. Clang and GCC give `int&&` ({question}`Q-0034`).
- PFX-4, PFX-8: all compilers reject these during constant evaluation.
- PFX-13: Clang rejects `++` on `bool` as a pedantic diagnostic.

Inputs: clang-sema-expr, clang-diag-sema.

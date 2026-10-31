# Postfix expressions

A postfix expression applies an operator after its operand. The postfix
expressions are subscripts, function calls, member access, and postfix
`++` and `--`.

## Principles

:::{principle} PFX-P1
:rules: PFX-1 PFX-2 PFX-3 PFX-4

A subscript is pointer arithmetic followed by a dereference: `a[i]` is
`*(a + i)`, so the operands can come in either order, one must be an
array or a pointer to an object, and the other an integer. The result
keeps the array's category, an lvalue for an array lvalue and an xvalue
for an array rvalue ({principle}`VC-P1`). A subscript outside the array
dereferences a pointer to nothing, which is undefined (ADD-P3).
:::

:::{principle} PFX-P2
:rules: PFX-5 PFX-6 PFX-7 PFX-8 PFX-9 PFX-15

A call passes one argument to each parameter, so the counts must match,
with default arguments filling in and an ellipsis taking any extra.
Each parameter is a new object that its argument copy-initializes, so
an `explicit` constructor does not take part. The call's result has the
category of what the function returns: a reference gives an lvalue or
an xvalue, and a value gives a prvalue. Calling through a pointer calls
the function it points at, with or without `*`. A null function pointer
points at no function, so the call is undefined. Whether a class with
its own copy or destruction can go through an ellipsis is each
implementation's choice.
:::

:::{principle} PFX-P3
:rules: PFX-10 PFX-11 PFX-14

Member access names a member of a class object, and `->` is member
access through a pointer. The object expression is always evaluated,
even for a static member. The member keeps the object's category and
cv-qualifiers, so a member of a prvalue is an xvalue and a member of a
`const` object is `const` unless it is `mutable` or a reference. A
member function reached this way is only good for one thing, a call,
and a call needs complete types for what it passes and returns.
:::

:::{principle} PFX-P4
:rules: PFX-12 PFX-13

Postfix `++` and `--` change the object like the prefix forms (UN-P3),
with one difference: the result is the old value, as a prvalue, so the
operand's limits are the same and the result cannot be changed.
:::

## Syntax

```{eval-rst}
.. productionlist:: expr_postfix
   postfix_expr: `primary_expr`
               : `postfix_expr` "[" `expression` "]"
               : `postfix_expr` "(" [`expression_list`] ")"
               : `postfix_expr` ("." | "->") `name`
               : `postfix_expr` ("++" | "--")
```

## Constraints

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
:tests: expressions/postfix/member-access.cpp expressions/postfix/member-of-prvalue.cpp expressions/postfix/static-member-object-evaluated.cpp expressions/postfix/mutable-and-reference-member-of-const.cpp

`E.m` names the member `m` of the class object `E`. `E->m` is the same as
`(*E).m`. A static member can be named through an object, and the
object expression is still evaluated. A member of a prvalue is an
xvalue. A `mutable` member of a `const` object can be changed. A reference
member of a `const` object gives an lvalue of the referenced type,
without `const`.
:::

:::{rule} PFX-11
:kind: reject
:tests: expressions/postfix/arrow-on-non-pointer.cpp expressions/postfix/dot-on-pointer.cpp expressions/postfix/no-such-member.cpp

The operand of `.` must be a class object, and the operand of `->` must be
a pointer to a class. The class must have the named member.
:::

:::{rule} PFX-13
:kind: reject
:tests: expressions/postfix/postfix-result-prvalue.cpp expressions/postfix/postfix-not-lvalue.cpp expressions/postfix/postfix-on-const.cpp expressions/postfix/postfix-on-bool.cpp expressions/postfix/postfix-decrement-bool.cpp expressions/postfix/postfix-on-void-pointer.cpp expressions/postfix/increment-enum.cpp expressions/postfix/increment-incomplete-pointer.cpp expressions/postfix/increment-function-pointer.cpp

The operand of postfix `++` or `--` must be a modifiable lvalue. It must
not have type `bool` or an enumeration type. A pointer operand must
point to a complete object type. The result is a prvalue.
:::

:::{rule} PFX-14
:kind: reject
:tests: expressions/postfix/member-function-not-value.cpp expressions/postfix/call-incomplete-return.cpp expressions/postfix/call-incomplete-parameter.cpp

A program must not:

- use the name of a non-static member function, reached through member
  access, other than to call it,
- call a function whose return type or parameter type is an incomplete
  class, except in the operand of `decltype` (UNEV-1).
:::

## Runtime Behavior

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
:tests: expressions/postfix/subscript-out-of-range.cpp expressions/postfix/subscript-write-past-end-constant.cpp expressions/postfix/subscript-form-beyond-end-constant.cpp expressions/postfix/subscript-out-of-range-sanitizer.cpp expressions/postfix/subscript-past-end-heap-sanitizer.cpp

If a subscript reads or writes an element outside the array, the
behavior is undefined. Forming the address of an element more than one
past the end is also undefined.
:::

:::{rule} PFX-8
:kind: undefined
:tests: expressions/postfix/call-null-pointer.cpp

A call through a null function pointer is undefined.
:::

## Implementation Requirements

:::{rule} PFX-15
:kind: conditionally-supported
:tests: expressions/postfix/ellipsis-nontrivial-class.cpp

Passing an object of a class with a non-trivial copy constructor, move
constructor, or destructor to an ellipsis parameter is conditionally
supported. GCC and EDG accept it. Clang rejects it.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 37 of 39 tests.

- PFX-3: EDG gives `int&` for a subscript of an array member of a
  prvalue. Clang and GCC give `int&&` ({question}`Q-0034`).
- PFX-4, PFX-8: all compilers reject these during constant evaluation.
- PFX-13: Clang rejects `++` on `bool` as a pedantic diagnostic.
- PFX-15: GCC and EDG accept a class with a non-trivial copy
  constructor as an ellipsis argument. Clang rejects it. Support is
  conditional.

Inputs: clang-sema-expr, clang-diag-sema.

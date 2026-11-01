# Postfix expressions

A postfix expression applies an operator after its operand. The postfix
expressions are these:

- subscripts,
- function calls,
- member access,
- postfix `++` and `--`.

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
:rules: PFX-5 PFX-6 PFX-7 PFX-8 PFX-9 PFX-15 PFX-16 PFX-17 PFX-19

A call passes one argument to each parameter, so the counts must match,
with default arguments filling in and an ellipsis taking any extra. Each
parameter is a new object that its argument copy-initializes, so an
`explicit` constructor does not take part. The call's result has the
category of what the function returns. A reference gives an lvalue or an
xvalue, and a value gives a prvalue. Calling through a pointer calls the
function that the pointer points at, with or without `*`. A null
function pointer points at no function, so the call is undefined. A
call through a pointer of another function type is undefined too. Each
implementation chooses whether a class with its own copy or destruction
can go through an ellipsis.
:::

:::{principle} PFX-P3
:rules: PFX-10 PFX-11 PFX-14 PFX-20 PFX-21

Member access names a member of a class object, and `->` is member
access through a pointer. The object expression is always evaluated,
even for a static member. The member keeps the object's category and
cv-qualifiers, so a member of a prvalue is an xvalue and a member of a
`const` object is `const` unless the member is `mutable` or a reference.
A member function reached this way can only be called, and a call needs
complete types for what it passes and returns.
:::

:::{principle} PFX-P4
:rules: PFX-12 PFX-13 PFX-18

Postfix `++` and `--` change the object like the prefix forms (UN-P3),
except that the result is the old value, as a prvalue. So the operand's
limits are the same and the result cannot be changed. An unsigned value
wraps around, and an overflow of a signed type is undefined, as for the
prefix forms.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   postfix_expr: `primary`
               : | `postfix_expr` "[" [`expression_list`] "]"
               : | `postfix_expr` "(" [`expression_list`] ")"
               : | `postfix_expr` ("." | "->") ["template"] `id_expression`
               : | `postfix_expr` ("++" | "--")
               : | `functional_cast` | `named_cast` | `dynamic_cast`
   expression_list: `argument` ["..."] {"," `argument` ["..."]}
   argument: `assignment_expr` | `braced_init`
```

An `argument` with `...` is a pack expansion ({rule}`PACK-13`).

## Constraints

:::{rule} PFX-1
:kind: required
:tests: expressions/postfix/subscript-is-pointer-arithmetic.cpp expressions/postfix/subscript-enum-index.cpp

The built-in subscript `a[i]` is the same as `*(a + i)`. The operands
can be in either order, so `i[a]` is also valid. The index can be an
integer or an unscoped enumeration.
:::

:::{rule} PFX-2
:kind: reject
:tests: expressions/postfix/subscript-two-integers.cpp expressions/postfix/subscript-floating-index.cpp expressions/postfix/subscript-scoped-enum-index.cpp

For a built-in subscript, one operand must be an array or a pointer to
an object type. The other operand must have integer or unscoped
enumeration type. A top-level comma in a subscript is covered by
COMMA-6.
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
default argument. The call must not give more arguments than
parameters, unless the function has an ellipsis. For an overloaded call,
{rule}`OVL-22` applies this count to each candidate. The rule is a
specific case of {rule}`OVL-35`.
:::

:::{rule} PFX-6
:kind: required
:tests: expressions/postfix/call-copy-initializes.cpp

Each parameter is copy-initialized from its argument.
:::

:::{rule} PFX-16
:kind: reject
:tests: expressions/postfix/call-explicit-constructor.cpp

An `explicit` constructor cannot do the conversion of PFX-6.
:::

:::{rule} PFX-9
:kind: required
:tests: expressions/postfix/call-result-category.cpp

A call to a function that returns an lvalue reference is an lvalue. A
call to a function that returns an rvalue reference is an xvalue. Other
calls are prvalues.
:::

:::{rule} PFX-10
:kind: required
:tests: expressions/postfix/member-access.cpp expressions/postfix/member-of-prvalue.cpp expressions/postfix/static-member-object-evaluated.cpp expressions/postfix/mutable-and-reference-member-of-const.cpp foundations/names/member-access-lookup-kinds.cpp

`E.m` names the member `m` of the class object `E`, which member lookup
finds ({rule}`NAM-37`). `E->m` is the same as `(*E).m`. Member access
also follows these rules:

- A static member can be named through an object, and the object
  expression is still evaluated.
- A member of a prvalue is an xvalue.
- A `mutable` member of a `const` object can be changed.
- A reference member of a `const` object gives an lvalue of the
  referenced type, without `const`.
:::

:::{rule} PFX-11
:kind: reject
:tests: expressions/postfix/arrow-on-non-pointer.cpp expressions/postfix/dot-on-pointer.cpp expressions/postfix/no-such-member.cpp declarations/declarators/function-declaration-ambiguity.cpp foundations/names/member-access-no-enclosing-scope.cpp foundations/names/member-access-no-free-function.cpp foundations/names/member-access-type-member.cpp expressions/postfix/member-access-incomplete-class.cpp expressions/postfix/member-access-class-being-defined.cpp expressions/postfix/member-access-later-member-in-body.cpp

The operand of `.` must be a class object, and the operand of `->` must
be a pointer to a class. Member lookup ({rule}`NAM-37`) must find the
named member, and the member must not be a type. The class must be
complete, or be the class whose definition contains the access. There
the access sees only the members declared before it ({rule}`NAM-6`,
{question}`Q-0316`).
:::

:::{rule} PFX-20
:kind: reject
:tests: expressions/postfix/member-function-not-called.cpp expressions/postfix/member-function-statement.cpp expressions/postfix/member-function-called.cpp foundations/overload-resolution/call-parenthesized-member-access.cpp

A member access `x.f` or `p->f` that names a non-static member function
must be the operand of a call, also inside parentheses ({rule}`VC-2`).
A static member function reached this way is an ordinary function.
:::

:::{rule} PFX-13
:kind: reject
:tests: expressions/postfix/postfix-result-prvalue.cpp expressions/postfix/postfix-not-lvalue.cpp expressions/postfix/postfix-on-const.cpp expressions/postfix/postfix-on-bool.cpp expressions/postfix/postfix-decrement-bool.cpp expressions/postfix/postfix-on-void-pointer.cpp expressions/postfix/increment-enum.cpp expressions/postfix/increment-incomplete-pointer.cpp expressions/postfix/increment-function-pointer.cpp

The operand of postfix `++` or `--` must obey these rules:

- The operand is a modifiable lvalue.
- The operand does not have type `bool` or an enumeration type.
- A pointer operand points to a complete object type.

The result is a prvalue.
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

:::{rule} PFX-17
:kind: reject
:tests: expressions/postfix/call-non-function.cpp expressions/postfix/call-object-pointer.cpp expressions/postfix/call-class-without-call-operator.cpp expressions/postfix/call-class-with-call-operator.cpp expressions/postfix/call-class-converts-to-function-pointer.cpp

The operand before the parentheses of a call must be one of these:

- a function, or a pointer or reference to one,
- an object of a class that has an `operator()`,
- an object of a class with a conversion function to a pointer to a
  function.
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
:tests: expressions/postfix/postfix-increment.cpp expressions/postfix/postfix-unsigned-wraps.cpp expressions/postfix/postfix-narrow-signed-wraps.cpp

Postfix `++` adds one to its operand, and postfix `--` subtracts one.
The result is the value before the change. On a pointer, the change is
one element, and on an integer, the change follows {rule}`UN-20`.
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

:::{rule} PFX-21
:kind: undefined
:tests: expressions/postfix/member-call-null-pointer-sanitizer.cpp expressions/postfix/member-call-null-pointer-constant.cpp

A call of a non-static member function through a null pointer, as in
`p->f()`, is undefined. This holds also when the function reads no
member. The call `(*p).f()` is the same call ({rule}`UN-7`).
:::

:::{rule} PFX-19
:kind: undefined
:tests: expressions/postfix/call-wrong-function-type-sanitizer.cpp

If a call reaches a function through a pointer or reference whose
function type is not the type of that function, the behavior is
undefined ({rule}`CAST-6`). Two types that differ only in `noexcept`
are different here too ({question}`Q-0178`).
:::

:::{rule} PFX-18
:kind: undefined
:tests: expressions/postfix/postfix-increment-overflow-constant.cpp expressions/postfix/postfix-decrement-overflow-constant.cpp expressions/postfix/postfix-increment-overflow-sanitizer.cpp expressions/postfix/postfix-pointer-past-end-constant.cpp

The behavior of postfix `++` and `--` is undefined in these cases:

- The operand has a signed type that {term}`integer promotion` does not
  change, and the result is outside the range of that type.
- A pointer moves outside its array, other than to one past the last
  element ({question}`Q-0031`).
:::

## Implementation Requirements

:::{rule} PFX-15
:kind: conditionally-supported
:tests: expressions/postfix/ellipsis-nontrivial-class.cpp

Passing an object to an ellipsis parameter is conditionally supported if
the class of the object has one of these that is non-trivial:

- a copy constructor,
- a move constructor,
- a destructor.

GCC and EDG accept such an argument, and Clang rejects it.
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
unsigned u = 0;
u--;              // the largest unsigned value
int top = 2147483647;
top++;            // undefined
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 60
of 67 tests.

- PFX-3: EDG gives `int&` for a subscript of an array member of a
  prvalue. Clang and GCC give `int&&` ({question}`Q-0034`).
- PFX-4, PFX-8: all compilers reject these cases during constant
  evaluation.
- PFX-12: EDG rejects `++` on a `short` that holds its largest value
  during constant evaluation. Clang and GCC give the converted value
  ({question}`Q-0124`).
- PFX-18: all compilers reject a signed overflow during constant
  evaluation, and UBSan reports one at run time. GCC accepts a pointer
  moved past the end of an array during constant evaluation. Clang and
  EDG reject it ({question}`Q-0031`).
- PFX-13: Clang rejects `++` on `bool` as a pedantic diagnostic.
- PFX-15: GCC and EDG accept a class with a non-trivial copy
  constructor as an ellipsis argument. Clang rejects it. Support is
  conditional.
- PFX-11: GCC and EDG accept a member access in a member declaration
  of the class being defined, to a member declared earlier. Clang
  rejects it because the class is incomplete ({question}`Q-0316`).
- PFX-21: UBSan reports a member call through a null pointer on Clang
  and GCC. Clang and GCC reject the call during constant evaluation.
  EDG accepts it ({question}`Q-0318`).
- PFX-19: the Clang sanitizer stops a call through a pointer whose
  parameter types differ from the function's. GCC has no such check, and
  the undefined rule allows its result. No implementation checks a
  difference in `noexcept` alone ({question}`Q-0178`).

Inputs: clang-sema-expr, clang-diag-sema.

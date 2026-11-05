# Value categories and temporaries

Each expression has a type and a {term}`value category`. The category is
lvalue, xvalue, or prvalue. The category controls what a program can do
with the result, and when a temporary object exists.

## Principles

:::{principle} VC-P1
:rules: VC-1 VC-2 VC-3 VC-12 VC-13 VC-14

Every expression denotes an object, a function, or a value. The
table lists the categories:

| Category | Identity | What it is |
|---|---|---|
| *lvalue* | Yes | Something a program can take the address of, assign to, and bind a reference to. |
| *xvalue* | Yes | An object whose contents the program has given up, so a move can take them. |
| *prvalue* | Not yet | A value, a result to be stored, used, or dropped. |

The form of an expression sets its category, and its type does not. The
name of a variable is an lvalue even when the variable has rvalue
reference type, because a name is a name. A function call has the
category of what the function returns: a reference gives an lvalue or an
xvalue, and anything else gives a prvalue.
:::

:::{principle} VC-P2
:rules: VC-4 VC-5 VC-8 VC-11

A prvalue is not an object until something needs an object. If a prvalue
initializes an object of its own type, that object is the result, and no
copy and no move takes place. There is no object to copy from. The
language makes a {term}`temporary object` from a prvalue only where an object
is unavoidable:

- a reference binds to the prvalue,
- a member or element is accessed,
- an array becomes a pointer,
- a class value is discarded.

{term}`Constant evaluation <constant evaluation>` follows the same model.
:::

:::{principle} VC-P3
:rules: VC-6 VC-9

A temporary object lives to the end of the {term}`full expression` that made it,
and temporaries of one full expression end in the reverse of their
order of creation. The {term}`lifetime` of every temporary therefore follows
from where full expressions begin and end. Each of the following is a
separate full expression:

- the initializer of a {term}`declarator`,
- the initializer of a member,
- a condition,
- a `return` expression,
- the initialization of each array element by a constructor with a
  default argument.

All the initializers of one aggregate in braces are one full
expression, so `int a[2] = {f(T{1}), f(T{2})};` ends both temporaries
after both elements are initialized. A function call, with its default
arguments, belongs to the full expression that contains the call. A reference can keep a temporary
alive longer (REF-4).
:::

:::{principle} VC-P4
:rules: VC-7 VC-10 VC-15 VC-16

The implementation keeps some freedom when a program passes or returns
by value. The caller makes and ends each parameter object, so an
exception from either step belongs to the caller. The implementation can make an extra copy of a {term}`trivially copyable`
argument or return value, and it chooses when a parameter
object ends. A program must not depend on the address of such an object
or on the point where the object is destroyed.
:::

## Constraints

:::{rule} VC-1
:kind: required
:tests: foundations/values/lvalues.cpp foundations/values/rvalue-reference-name-is-lvalue.cpp foundations/values/more-categories.cpp foundations/values/member-access-lvalue.cpp

These expressions are lvalues:

- the name of a variable or a function, including a variable of rvalue
  reference type,
- a string literal,
- `*p`, `++x`, `--x`, and the result of an assignment,
- a call to a function that returns an lvalue reference, and a cast to
  an lvalue reference type,
- a subscript of an array lvalue,
- a data member of an lvalue, with `.` or `->`,
- a comma expression whose right operand is an lvalue (COMMA-2).
:::

:::{rule} VC-14
:kind: reject
:tests: foundations/values/rvalue-reference-name-binding.cpp foundations/values/rvalue-reference-parameter-name-binding.cpp

The name of a variable of rvalue reference type is an lvalue (VC-1). An
rvalue reference must not bind to that name directly.
:::

:::{rule} VC-2
:kind: required
:tests: foundations/values/prvalues.cpp foundations/values/cv-prvalue-dropped.cpp foundations/values/assign-to-class-prvalue.cpp

These expressions are prvalues:

- a literal other than a string literal, including `nullptr`,
- `this` and an enumerator,
- a call to a function that returns a type that is not a reference, and
  a cast to a type that is not a reference,
- `x++`, `x--`, `&x`, and the result of an arithmetic operator,
- a member access that names a non-static member function. Such an
  expression can only be called ({rule}`PFX-20`).

A prvalue of a type that is not a class type has no {term}`cv-qualifiers <cv-qualifier>`. So a
function can declare a return type such as `const int`, and a call to
the function still gives a plain `int`. A class prvalue can be assigned
to, because the assignment is a call of a member function. ASN-6 covers
a braced list as the right operand.
:::

:::{rule} VC-12
:kind: reject
:tests: foundations/values/address-of-prvalue.cpp foundations/values/assign-to-prvalue.cpp foundations/values/member-function-access-not-address.cpp foundations/values/assign-to-scalar-prvalue.cpp expressions/additive/result-prvalue.cpp expressions/shift/result-prvalue.cpp expressions/conditional/mixed-category-prvalue.cpp expressions/conditional/const-volatile-lvalues-prvalue.cpp expressions/primary/this-prvalue.cpp expressions/primary/name-enumerator-prvalue.cpp

VC-2 gives the prvalues. A program must not take the address of a
prvalue. A prvalue of a type that is not a class cannot be assigned to.
:::

:::{rule} VC-3
:kind: required
:tests: foundations/values/xvalues.cpp foundations/values/member-reference-of-prvalue.cpp foundations/values/rvalue-ref-to-function-cast.cpp foundations/values/static-member-of-prvalue.cpp foundations/values/xvalue-array-subscript.cpp foundations/values/xvalue-member-pointer.cpp

These expressions are xvalues:

- a cast to an rvalue reference to an object type,
- a call to a function that returns an rvalue reference to an object
  type,
- a non-static data member of an xvalue or of a class prvalue, if the
  member is not a reference,
- a subscript of an array xvalue or array prvalue ({rule}`PFX-3`,
  {question}`Q-0034`),
- `.*` applied to an xvalue, for a data member.

A cast to an rvalue reference to a function type gives an lvalue. A
reference member and a static data member give lvalues.
:::

:::{rule} VC-13
:kind: reject
:tests: foundations/values/address-of-xvalue.cpp foundations/values/address-of-xvalue-call.cpp

A program must not take the address of an xvalue (VC-3).
:::

:::{rule} VC-11
:kind: reject
:tests: foundations/values/discarded-call-deleted-destructor.cpp foundations/values/decltype-call-deleted-destructor.cpp

A discarded class prvalue makes a temporary object, so its destructor
must be usable. In the operand of `decltype`, a call makes no temporary,
and the destructor is not needed (UNEV-1).
:::

## Runtime Behavior

:::{rule} VC-4
:kind: required
:tests: foundations/values/guaranteed-elision.cpp foundations/values/materialization-binds.cpp foundations/values/prvalue-no-object-until-needed.cpp

A prvalue that initializes an object of the same type initializes that
object directly. No temporary object exists, and no copy or move
constructor is called. A temporary object comes into existence in the
cases that VC-5 and VC-8 list, and not before.
:::

:::{rule} VC-5
:kind: required
:tests: foundations/values/discarded-prvalue-destroyed.cpp foundations/values/discarded-prvalue-braced.cpp

If an expression statement is only a class prvalue, the program makes a
temporary object from the prvalue. The program calls the destructor of
the temporary when the statement ends.
:::

:::{rule} VC-6
:kind: required
:tests: foundations/values/temporaries-reverse-order.cpp foundations/values/temporaries-reverse-order-comma.cpp

The life of a temporary object ends when its full expression ends. A
reference bound to the temporary or to a member of the temporary
(REF-4), or a range-based `for` (RFOR-4), can make the life longer. The
temporaries that end with a full expression go away in the reverse order
of their creation.
:::

:::{rule} VC-8
:kind: required
:tests: foundations/values/materialize-reference-binding.cpp foundations/values/materialize-member-call.cpp foundations/values/materialize-member-access.cpp foundations/values/materialize-array-subscript.cpp foundations/values/materialize-array-decay.cpp foundations/values/temporary-constant-evaluation.cpp foundations/values/materialize-pointer-to-member.cpp foundations/values/array-prvalue-argument.cpp

A prvalue becomes a temporary object in these cases, in addition to
VC-5:

- a reference binds to the prvalue ({rule}`REF-2`, {rule}`REF-3`),
- a member function call or member access on a class prvalue,
- a subscript of an array prvalue,
- the conversion of an array prvalue to a pointer,
- `.*` applied to a class prvalue.

Constant evaluation creates temporaries in the same cases.
:::

:::{rule} VC-9
:kind: required
:tests: foundations/values/argument-lives-to-end.cpp foundations/values/default-argument-temporary.cpp foundations/values/return-temporaries-before-locals.cpp foundations/values/declarators-separate.cpp foundations/values/condition-temporaries.cpp foundations/values/aggregate-element-temporaries.cpp foundations/values/array-element-default-argument.cpp foundations/evaluation-order/full-expression-statement.cpp foundations/evaluation-order/full-expression-for-clauses.cpp foundations/evaluation-order/full-expression-subexpression-not.cpp foundations/values/member-initializer-full-expression.cpp foundations/values/init-capture-temporary.cpp

A *full expression* is an expression that is not part of a larger
expression. The temporaries of a full expression end with the full
expression. The following are full expressions:

- the expression of an expression statement,
- the initializer of one declarator, in a declaration with several,
- the initializer of one member, in a member initializer list or as a
  default member initializer,
- a condition,
- each of the three clauses of a `for` statement,
- the expression of a `return` statement. The temporaries of this
  expression end before the local variables of the function.

A temporary in a function argument or a default argument lives until
the end of the full expression that contains the call. All initializers
of one aggregate are one full expression. If a constructor with a
default argument initializes each element of an array, the temporaries
of one element end before the next element is initialized. The
initializer of an init capture belongs to the full expression that
contains the lambda expression.
:::

:::{rule} VC-16
:kind: required
:tests: foundations/values/parameter-belongs-to-caller.cpp foundations/values/parameter-conversion-throws-in-caller.cpp

The caller initializes and destroys the parameter objects of a call. An
exception from either step leaves the call expression in the caller and
does not leave the called function. So a non-throwing called function
does not end the program ({rule}`UNW-5`).
:::

## Implementation Requirements

:::{rule} VC-7
:kind: unspecified
:tests: foundations/values/trivial-parameter-address.cpp foundations/values/trivial-parameter-address-constructor.cpp

An implementation can make an extra temporary copy of a trivially
copyable class object for a function argument or a return value
({rule}`CPY-19`). Whether a parameter and the caller's object have the
same address is unspecified.
:::

:::{rule} VC-10
:kind: impl-defined
:tests: foundations/values/parameter-destruction-point.cpp classes/constructors/parameter-destroyed-by-callee-constant.cpp

Each implementation chooses where to destroy a parameter object, and
documents the choice. The point is either the return from the function
or the end of the caller's full expression. On the x86-64 Linux targets
in the matrix, the caller destroys the parameter object at the end of
the caller's full expression. The caller's end of a parameter object is
a destruction, so a called function that destroys its parameter makes
that end a second destruction ({rule}`DTOR-7`).
:::

:::{rule} VC-15
:kind: unspecified
:tests: foundations/evaluation-order/parameter-destruction-order.cpp foundations/evaluation-order/parameter-destruction-order-three.cpp

The parameter objects of one call are destroyed in an unspecified order.
:::

## Examples

% tidal-example: reject
```cpp
int x = 0;
decltype((x))                      // int&: an lvalue
decltype((x + 1))                  // int: a prvalue
decltype((static_cast<int&&>(x)))  // int&&: an xvalue

int&& r = 1;
int&& s = r;                       // rejected: r is an lvalue

struct NoCopy {
  NoCopy(int);
  NoCopy(const NoCopy&) = delete;
};
NoCopy a = NoCopy(NoCopy(5));      // fine: no copy occurs
```

## Evidence

All eight compilers agree on 62 of 64 tests. The results are in
`tests/RESULTS.md`.

### Divergences

- {question}`Q-0034` (VC-3, PFX-3; settled on PFX-P1): whether a
  subscript of an array xvalue is an xvalue. It is, as Clang and GCC
  say. EDG gives an lvalue.
- {question}`Q-0054` (VC-10, DTOR-7; settled on CTOR-P4): whether
  constant evaluation rejects a second destruction, such as of a
  parameter that the called function destroyed. It does, as Clang and
  GCC say. EDG accepts the second destruction.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `classes/constructors/parameter-destroyed-by-callee-constant.cpp` | DTOR-7, VC-10 | reject | reject | reject | accept | {question}`Q-0054` |
| `xvalue-array-subscript.cpp` | VC-3 | accept | accept | accept | reject | {question}`Q-0034` |

### Notes

- VC-15: Clang 22 destroys the second parameter of a call first, and GCC
  16 the first.
- VC-7: the test accepts either result.
- VC-10: the test accepts either point. Clang and GCC destroy the
  parameter at the end of the full expression.
- VC-16: on Clang and GCC, the caller catches an exception from the copy
  into a parameter and from the parameter's destructor, also when the
  called function is non-throwing.
- VC-9: on Clang and GCC, the temporaries of a member initializer end
  before the next member is initialized. The temporaries of an init
  capture end with the full expression that contains the lambda.
- VC-10 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. In the
  writer's probes, the run tests of the rule give the same
  results with `-m32` in the GCC 14, Clang 18, and Clang 22 containers.

### Sources

Inputs: clang-sema-expr, clang-diag-sema.

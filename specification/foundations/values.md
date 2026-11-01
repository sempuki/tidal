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
language makes a temporary object from a prvalue only where an object
is unavoidable:

- a reference binds to the prvalue,
- a member or element is accessed,
- an array becomes a pointer,
- a class value is discarded.

Constant evaluation follows the same model.
:::

:::{principle} VC-P3
:rules: VC-6 VC-9

A temporary object lives to the end of the full expression that made it,
and temporaries of one full expression end in the reverse of their
order of creation. The lifetime of every temporary therefore follows
from where full expressions begin and end. Each of the following is a
separate full expression:

- the initializer of a declarator,
- the initializer of a member,
- a condition,
- a `return` expression,
- the initialization of each array element.

A function call, with its default arguments, belongs to the full
expression that contains the call. A reference can keep a temporary
alive longer (REF-4).
:::

:::{principle} VC-P4
:rules: VC-7 VC-10 VC-15 VC-16

The implementation keeps some freedom when a program passes or returns
by value. The caller makes and ends each parameter object, so an
exception from either step belongs to the caller. The implementation can make an extra copy of a trivially
copyable argument or return value, and it chooses when a parameter
object ends. A program must not depend on the address of such an object
or on the point where the object is destroyed.
:::

## Constraints

:::{rule} VC-1
:kind: required
:tests: expressions/value-categories/lvalues.cpp expressions/value-categories/rvalue-reference-name-is-lvalue.cpp expressions/value-categories/more-categories.cpp expressions/value-categories/member-access-lvalue.cpp

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
:tests: expressions/value-categories/rvalue-reference-name-binding.cpp

The name of a variable of rvalue reference type is an lvalue (VC-1). An
rvalue reference must not bind to that name directly.
:::

:::{rule} VC-2
:kind: required
:tests: expressions/value-categories/prvalues.cpp expressions/value-categories/cv-prvalue-dropped.cpp expressions/value-categories/assign-to-class-prvalue.cpp

These expressions are prvalues:

- a literal other than a string literal, including `nullptr`,
- `this` and an enumerator,
- a call to a function that returns a type that is not a reference, and
  a cast to a type that is not a reference,
- `x++`, `x--`, `&x`, and the result of an arithmetic operator,
- a member access that names a non-static member function. Such an
  expression can only be called.

A prvalue of a type that is not a class type has no cv-qualifiers. So a
function can declare a return type such as `const int`, and a call to
the function still gives a plain `int`. A class prvalue can be assigned
to, because the assignment is a call of a member function. ASN-6 covers
a braced list as the right operand.
:::

:::{rule} VC-12
:kind: reject
:tests: expressions/value-categories/address-of-prvalue.cpp expressions/value-categories/assign-to-prvalue.cpp expressions/value-categories/member-function-access-not-address.cpp expressions/value-categories/assign-to-scalar-prvalue.cpp expressions/additive/result-prvalue.cpp expressions/shift/result-prvalue.cpp expressions/conditional/mixed-category-prvalue.cpp expressions/conditional/const-volatile-lvalues-prvalue.cpp expressions/primary/this-prvalue.cpp expressions/primary/name-enumerator-prvalue.cpp

VC-2 gives the prvalues. A program must not take the address of a
prvalue. A prvalue of a type that is not a class cannot be assigned to.
:::

:::{rule} VC-3
:kind: required
:tests: expressions/value-categories/xvalues.cpp expressions/value-categories/member-reference-of-prvalue.cpp expressions/value-categories/rvalue-ref-to-function-cast.cpp expressions/value-categories/static-member-of-prvalue.cpp expressions/value-categories/xvalue-array-subscript.cpp expressions/value-categories/xvalue-member-pointer.cpp

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
:tests: expressions/value-categories/address-of-xvalue.cpp

A program must not take the address of an xvalue (VC-3).
:::

:::{rule} VC-11
:kind: reject
:tests: expressions/value-categories/discarded-call-deleted-destructor.cpp expressions/value-categories/decltype-call-deleted-destructor.cpp

A discarded class prvalue makes a temporary object, so its destructor
must be usable. In the operand of `decltype`, a call makes no temporary,
and the destructor is not needed (UNEV-1).
:::

## Runtime Behavior

:::{rule} VC-4
:kind: required
:tests: expressions/value-categories/guaranteed-elision.cpp expressions/value-categories/materialization-binds.cpp expressions/value-categories/prvalue-no-object-until-needed.cpp

A prvalue that initializes an object of the same type initializes that
object directly. No temporary object exists, and no copy or move
constructor is called. A temporary object comes into existence in the
cases that VC-5 and VC-8 list, and not before.
:::

:::{rule} VC-5
:kind: required
:tests: expressions/value-categories/discarded-prvalue-destroyed.cpp

If an expression statement is only a class prvalue, the program makes a
temporary object from the prvalue. The program calls the destructor of
the temporary when the statement ends.
:::

:::{rule} VC-6
:kind: required
:tests: expressions/value-categories/temporaries-reverse-order.cpp

The life of a temporary object ends when its full expression ends. A
reference bound to the temporary or to a member of the temporary
(REF-4), or a range-based `for` (RFOR-4), can make the life longer. The
temporaries that end with a full expression go away in the reverse order
of their creation.
:::

:::{rule} VC-8
:kind: required
:tests: expressions/value-categories/materialize-reference-binding.cpp expressions/value-categories/materialize-member-call.cpp expressions/value-categories/materialize-member-access.cpp expressions/value-categories/materialize-array-subscript.cpp expressions/value-categories/materialize-array-decay.cpp expressions/value-categories/temporary-constant-evaluation.cpp expressions/value-categories/materialize-pointer-to-member.cpp expressions/value-categories/array-prvalue-argument.cpp

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
:tests: expressions/value-categories/argument-lives-to-end.cpp expressions/value-categories/default-argument-temporary.cpp expressions/value-categories/return-temporaries-before-locals.cpp expressions/value-categories/declarators-separate.cpp expressions/value-categories/condition-temporaries.cpp expressions/value-categories/aggregate-element-temporaries.cpp expressions/value-categories/array-element-default-argument.cpp expressions/sequencing/full-expression-statement.cpp expressions/sequencing/full-expression-for-clauses.cpp expressions/sequencing/full-expression-subexpression-not.cpp expressions/value-categories/member-initializer-full-expression.cpp expressions/value-categories/init-capture-temporary.cpp

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
:tests: expressions/value-categories/parameter-belongs-to-caller.cpp

The caller initializes and destroys the parameter objects of a call. An
exception from either step leaves the call expression in the caller and
does not leave the called function. So a non-throwing called function
does not end the program ({rule}`UNW-5`).
:::

## Implementation Requirements

:::{rule} VC-7
:kind: unspecified
:tests: expressions/value-categories/trivial-parameter-address.cpp

An implementation can make an extra temporary copy of a trivially
copyable class object for a function argument or a return value
({rule}`CPY-19`). Whether a parameter and the caller's object have the
same address is unspecified.
:::

:::{rule} VC-10
:kind: impl-defined
:tests: expressions/value-categories/parameter-destruction-point.cpp

Each implementation chooses where to destroy a parameter object, and
documents the choice. The point is either the return from the function
or the end of the caller's full expression. On the x86-64 Linux targets
in the matrix, the caller destroys the parameter object at the end of
the caller's full expression.
:::

:::{rule} VC-15
:kind: unspecified
:tests: expressions/sequencing/parameter-destruction-order.cpp

The parameter objects of one call are destroyed in an unspecified order.
:::

## Examples

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

All eight compilers agree on 55 of 56 tests. The results are in
`tests/RESULTS.md`.

- VC-15: Clang 22 destroys the second parameter of a call first, and GCC
  16 the first.
- VC-3: EDG gives an lvalue for a subscript of an array xvalue. Clang
  and GCC give an xvalue ({question}`Q-0034`).
- VC-7: the test accepts either result.
- VC-10: the test accepts either point. Clang and GCC destroy the
  parameter at the end of the full expression.
- VC-16: on Clang and GCC, the caller catches an exception from the copy
  into a parameter and from the parameter's destructor, also when the
  called function is non-throwing.
- VC-9: on Clang and GCC, the temporaries of a member initializer end
  before the next member is initialized. The temporaries of an init
  capture end with the full expression that contains the lambda.

Inputs: clang-sema-expr, clang-diag-sema.

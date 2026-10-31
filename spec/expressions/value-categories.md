# Value categories and temporaries

Each expression has a type and a {term}`value category`. The category is
lvalue, xvalue, or prvalue. It controls what a program can do with the
result, and when a temporary object exists.

## Principles

:::{principle} VC-P1
:rules: VC-1 VC-2 VC-3

Every expression denotes an object, a function, or a value. An *lvalue*
names something with an identity: a program can take its address, assign
to it, and bind a reference to it. A *prvalue* is a value with no
identity yet, a result to be stored, used, or dropped. An *xvalue* is an
object that has an identity but whose contents the program has given up,
so a move can take them.

The form of an expression sets its category, and its type does not. The
name of a variable is an lvalue even when the variable has rvalue
reference type, because a name is a name. A function call has the
category of what the function returns: a reference gives an lvalue or an
xvalue, and anything else gives a prvalue.
:::

:::{principle} VC-P2
:rules: VC-4 VC-5 VC-8

A prvalue is not an object until something needs an object. If a prvalue
initializes an object of its own type, that object is the result, and no
copy and no move takes place. There is no object to copy from. The
language makes a temporary object from a prvalue only where an object
is unavoidable: a reference binds to it, a member or element is
accessed, an array becomes a pointer, or a class value is discarded.
Constant evaluation follows the same model.
:::

:::{principle} VC-P3
:rules: VC-6 VC-9

A temporary object lives to the end of the full expression that made it,
and temporaries of one full expression end in the reverse of their
order of creation. The lifetime of every temporary therefore follows
from where full expressions begin and end. The initializer of a
declarator, a condition, a `return` expression, and the initialization
of each array element are separate full expressions. A function call,
with its default arguments, belongs to the full expression that contains
the call. A reference can keep a temporary alive longer (REF-4).
:::

:::{principle} VC-P4
:rules: VC-7 VC-10

Passing and returning by value is where the implementation keeps some
freedom. It can make an extra copy of a trivially copyable argument or
return value, and it chooses when a parameter object ends. A program
must not depend on the address of such an object or on the point where
it is destroyed.
:::

## Constraints

:::{rule} VC-1
:kind: required
:tests: expressions/value-categories/lvalues.cpp expressions/value-categories/rvalue-reference-name-is-lvalue.cpp expressions/value-categories/rvalue-reference-name-binding.cpp expressions/value-categories/more-categories.cpp

These expressions are lvalues:

- the name of a variable or a function, including a variable of rvalue
  reference type,
- a string literal,
- `*p`, `++x`, `--x`, and the result of an assignment,
- a call to a function that returns an lvalue reference, and a cast to an
  lvalue reference type,
- a subscript of an array lvalue,
- a comma expression whose right operand is an lvalue (COMMA-2).
:::

:::{rule} VC-2
:kind: required
:tests: expressions/value-categories/prvalues.cpp expressions/value-categories/address-of-prvalue.cpp expressions/value-categories/assign-to-prvalue.cpp expressions/value-categories/cv-prvalue-dropped.cpp

These expressions are prvalues:

- a literal other than a string literal, including `nullptr`,
- `this` and an enumerator,
- a call to a function that returns a type that is not a reference, and a
  cast to a type that is not a reference,
- `x++`, `x--`, `&x`, and the result of an arithmetic operator.

A prvalue of a type that is not a class type has no cv-qualifiers. A
program must not take the address of a prvalue or assign to it.
:::

:::{rule} VC-3
:kind: required
:tests: expressions/value-categories/xvalues.cpp expressions/value-categories/address-of-xvalue.cpp expressions/value-categories/member-reference-of-prvalue.cpp expressions/value-categories/rvalue-ref-to-function-cast.cpp expressions/value-categories/static-member-of-prvalue.cpp

These expressions are xvalues:

- a cast to an rvalue reference to an object type,
- a call to a function that returns an rvalue reference to an object
  type,
- a non-static data member of an xvalue or of a class prvalue, if the
  member is not a reference.

A cast to an rvalue reference to a function type gives an lvalue. A
reference member and a static data member give lvalues. A program must
not take the address of an xvalue.
:::

## Dynamic Semantics

:::{rule} VC-4
:kind: required
:tests: expressions/value-categories/guaranteed-elision.cpp expressions/value-categories/materialization-binds.cpp expressions/value-categories/prvalue-no-object-until-needed.cpp

A prvalue that initializes an object of the same type initializes that
object directly. No temporary object exists, and no copy or move
constructor is called. A temporary object comes into existence only when
a reference binds to the prvalue or when a member of it is accessed.
:::

:::{rule} VC-5
:kind: required
:tests: expressions/value-categories/discarded-prvalue-destroyed.cpp

If an expression statement is only a class prvalue, the program makes a
temporary object from it. The program calls its destructor when that
statement ends.
:::

:::{rule} VC-6
:kind: required
:tests: expressions/value-categories/temporaries-reverse-order.cpp

The life of a temporary object ends when its full expression ends. A
reference can make the life longer (REF-4). The temporaries of one full
expression go away in the reverse order of their creation.
:::

:::{rule} VC-8
:kind: required
:tests: expressions/value-categories/materialize-member-call.cpp expressions/value-categories/materialize-member-access.cpp expressions/value-categories/materialize-array-subscript.cpp expressions/value-categories/materialize-array-decay.cpp expressions/value-categories/temporary-constant-evaluation.cpp expressions/value-categories/materialize-pointer-to-member.cpp

A prvalue becomes a temporary object in these cases, in addition to
VC-4 and VC-5:

- a member function call or member access on a class prvalue,
- a subscript of an array prvalue,
- the conversion of an array prvalue to a pointer,
- `.*` applied to a class prvalue.

Constant evaluation creates temporaries in the same cases.
:::

:::{rule} VC-9
:kind: required
:tests: expressions/value-categories/argument-lives-to-end.cpp expressions/value-categories/default-argument-temporary.cpp expressions/value-categories/return-temporaries-before-locals.cpp expressions/value-categories/declarators-separate.cpp expressions/value-categories/condition-temporaries.cpp expressions/value-categories/aggregate-element-temporaries.cpp expressions/value-categories/array-element-default-argument.cpp

Each of these is a separate full expression, so its temporaries end with
it:

- the initializer of one declarator, in a declaration with several,
- a condition,
- the expression of a `return` statement. Its temporaries end before the
  local variables of the function.

A temporary in a function argument or a default argument lives until
the end of the full expression that contains the call. All initializers
of one aggregate are one full expression. If a constructor with a
default argument initializes each element of an array, the temporaries
of one element end before the next element is initialized.
:::

## Implementation Requirements

:::{rule} VC-7
:kind: unspecified
:tests: expressions/value-categories/trivial-parameter-address.cpp

An implementation can make an extra temporary copy of a trivially
copyable class object for a function argument or a return value
*(classes referenced)*. It is unspecified if a parameter and the
caller's object have the same address.
:::

:::{rule} VC-10
:kind: impl-defined
:tests: expressions/value-categories/parameter-destruction-point.cpp

Each implementation chooses one of two points to destroy a parameter
object, and documents it. The first is the return from the function. The
second is the end of the caller's full expression. On the x86-64 Linux
targets in the matrix, the caller destroys it at the second point.
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

All seven compilers agree on all 33 tests. The results are in
`tests/RESULTS.md`.
- VC-7: the test accepts either result.
- VC-10: the test accepts either point. Clang and GCC destroy the
  parameter at the end of the full expression.

Inputs: clang-sema-expr, clang-diag-sema.

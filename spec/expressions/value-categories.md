# Value categories and temporaries

Each expression has a type and a {term}`value category`. The category is
lvalue, xvalue, or prvalue. It controls what a program can do with the
result, and when a temporary object exists.

## Legality Rules

:::{rule} VC-1
:kind: required
:tests: expressions/value-categories/lvalues.cpp expressions/value-categories/rvalue-reference-name-is-lvalue.cpp expressions/value-categories/rvalue-reference-name-binding.cpp

These expressions are lvalues:

- the name of a variable or a function, including a variable of rvalue
  reference type,
- a string literal,
- `*p`, `++x`, `--x`, and the result of an assignment,
- a call to a function that returns an lvalue reference,
- a subscript of an array lvalue.
:::

:::{rule} VC-2
:kind: required
:tests: expressions/value-categories/prvalues.cpp expressions/value-categories/address-of-prvalue.cpp expressions/value-categories/assign-to-prvalue.cpp

These expressions are prvalues:

- a literal other than a string literal, including `nullptr`,
- a call to a function that returns a type that is not a reference,
- `x++`, `x--`, `&x`, and the result of an arithmetic operator.

A program must not take the address of a prvalue or assign to it.
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

All seven compilers agree on all 11 tests. The results are in
`tests/RESULTS.md`.

Inputs: clang-sema-expr, clang-diag-sema, gcc-cp-typeck.

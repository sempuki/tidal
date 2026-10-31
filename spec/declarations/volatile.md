# Volatile

A `volatile` qualifier marks an object whose accesses the program must
perform as written, for example a device register. This page covers what
`volatile` means for accesses, volatile member functions, references and
pointers to volatile objects, copies of volatile class objects, and the
operations on volatile objects that implementations warn about.

Depends on: declarators and cv-qualifiers, reference binding, standard
conversions, copy and move.

## Syntax

`volatile` is a {term}`cv-qualifier`. It can follow a member function's
parameter list, as `const` can.

## Constraints

:::{rule} VOL-2
:kind: required
:tests: declarations/volatile/nonvolatile-member-on-volatile.cpp declarations/volatile/overload-on-volatile.cpp declarations/volatile/static-const-volatile-in-class.cpp declarations/volatile/volatile-member-function.cpp declarations/volatile/volatile-member-of-volatile-object.cpp declarations/volatile/volatile-member-on-plain.cpp declarations/volatile/volatile-object-ordinary-ctor.cpp

A member function declared `volatile` can be called on a volatile object
and on other objects. A member function that is not `volatile` cannot be
called on a volatile object. Two member functions can overload on
`volatile`. A member of a volatile object is volatile. A `static const
volatile` integer member can have an initializer in its class
({question}`Q-0080`). A volatile object of class type is initialized by
an ordinary constructor. It is volatile after the constructor ends.
:::

:::{rule} VOL-3
:kind: required
:tests: declarations/volatile/const-cast-removes.cpp declarations/volatile/const-ref-volatile-no-temporary.cpp declarations/volatile/overload-param-volatile-ref.cpp declarations/volatile/pointer-add-volatile.cpp declarations/volatile/pointer-remove-volatile.cpp declarations/volatile/ref-plain-to-volatile.cpp declarations/volatile/ref-volatile-to-plain.cpp

A conversion can add `volatile` to the target of a pointer or reference.
It cannot remove it, except by `const_cast`. So a reference that is not
`volatile` does not bind to a volatile object, and a `const volatile`
reference does not bind to a temporary. Overload resolution can choose
between `T&` and `volatile T&`.
:::

:::{rule} VOL-4
:kind: required
:tests: declarations/volatile/assign-to-volatile-class.cpp declarations/volatile/copy-volatile-class-rejected.cpp declarations/volatile/copy-volatile-scalar-ok.cpp declarations/volatile/volatile-class-user-copy.cpp

A volatile object of scalar type can be copied. The implicit copy
functions of a class take `const T&`, which does not bind to a volatile
object. A volatile class object can only be copied or assigned with
functions that the class declares for volatile operands.
:::

:::{rule} VOL-7
:kind: reject
:tests: declarations/volatile/volatile-constructor.cpp declarations/volatile/volatile-destructor.cpp

A constructor or a destructor must not be declared `volatile`.
:::

## Dynamic Semantics

:::{rule} VOL-1
:kind: required
:tests: declarations/volatile/access-run.cpp declarations/volatile/constant-read-rejected.cpp declarations/volatile/volatile-in-type-traits.cpp declarations/volatile/volatile-local-loop.cpp declarations/volatile/const-volatile-external.cpp

Each read and each write of an object through a volatile lvalue is an
observable effect *(referenced)*. The implementation performs it. The
type of a volatile variable keeps `volatile`, but the result of an
arithmetic operation on its value does not. Constant evaluation cannot
read a volatile object. A `const volatile` variable at namespace scope has
external linkage, so all translation units share it.
:::

## Undefined Behavior

:::{rule} VOL-6
:kind: undefined
:tests: declarations/volatile/nonvolatile-access-to-volatile-constant.cpp

If a program accesses a volatile object through a glvalue that is not
volatile, the behavior is undefined.
:::

## Recommended Practice

:::{rule} VOL-5
:kind: recommended
:tests: declarations/volatile/assignment-result-warns.cpp declarations/volatile/compound-no-warning.cpp declarations/volatile/increment-warns.cpp declarations/volatile/volatile-structured-binding.cpp declarations/volatile/volatile-parameter-warns.cpp declarations/volatile/volatile-return-warns.cpp declarations/volatile/pointer-to-volatile-parameter-ok.cpp

An implementation should warn about these uses, which are deprecated:

- `++` and `--` on a volatile object,
- the value of an assignment to a volatile object, used in a larger
  expression,
- a structured binding declaration with `volatile`.

Compound assignment, such as `v += 1` and `v |= 1`, is not deprecated.
CV-9 gives the warnings for volatile parameter and return types.
:::

## Examples

```cpp
struct Uart {
  unsigned status;
  unsigned data;
  unsigned read() volatile { return data; }
};
volatile Uart* const port = nullptr;   // address set by the platform

void send(volatile Uart& u, unsigned c) {
  while ((u.status & 1) == 0) {}       // each test reads status again
  u.data = c;                          // one write
  u.status |= 2;                       // valid, not deprecated
}
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 32 of
33 tests.

- VOL-1: the tests check that programs with volatile accesses compile and
  run. A test cannot observe if an access was removed.
- VOL-2: Clang rejects an in-class initializer for a `static const
  volatile` member. GCC and EDG accept it ({question}`Q-0080`).
- VOL-5: Clang and GCC warn under `-Wall` for each listed use, and not for
  compound assignment.

Inputs: clang-sema-expr, clang-sema-decl, clang-diag-sema, edg-decls.

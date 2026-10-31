# Objects, storage, and lifetime

An object is a region of storage with a type. Every object has a storage
duration that says when its storage exists, and a lifetime that says
when the object itself exists in that storage. Subobjects, temporaries,
and the order in which objects end all follow from this model. This
page states it once; the pages on initialization, temporaries,
constructors, and unions cite it.

Depends on: storage duration and linkage, value categories and
temporaries, constructors and destructors, new and delete.

## Principles

:::{principle} OBJ-P1
:rules: OBJ-1 OBJ-7

An object comes into existence when its initialization completes and
goes out of existence when its destructor completes, or when its storage
is released or reused. Between those two points the object has a stable
address, and no other complete object shares it. An object of scalar
type has a lifetime that starts with its storage, so it can exist with
no value yet.

A class object is built from its subobjects, and their lifetimes nest
inside its own. The bases and members exist before the constructor body
runs and still exist while the destructor body runs. So a constructor
body sees complete parts, and a destructor body sees them too.
:::

:::{principle} OBJ-P2
:rules: OBJ-3 OBJ-4 OBJ-5 OBJ-6 OBJ-9
:exceptions: OBJ-11

The storage duration of an object decides who ends it and when, and one
law covers every case: objects end in the reverse of the order in which
they began. A block ends its automatic objects when control leaves it,
by any route. A program ends its static objects after `main` returns, in
the reverse of their construction order, which is the order of their
definitions for objects at namespace scope and the order of first use
for objects local to a function. An array ends its elements from the
last to the first. A temporary ends with the full expression that made
it. A dynamic object is the one case with no fixed end: it exists until
`delete` ends it, however many scopes have come and gone.
:::

:::{principle} OBJ-P3
:rules: OBJ-8

A reference is not an object. Binding a reference to an object gives
the object no extra life, so a reference can outlive what it refers to.
The one exception is a reference that binds to a temporary: the
temporary then lives as long as the reference, because otherwise the
reference would be dead at birth.
:::

:::{principle} OBJ-P4
:rules: OBJ-2 OBJ-10

Outside its lifetime an object has no value, and a program must not read
or write it. This covers a scalar before its first write, a member that
no initializer gave a value, a variable whose block has ended, a
dynamic object after `delete`, and an object whose storage another
object has taken. Constant evaluation tracks every lifetime, so it
rejects each of these.
:::

## Runtime Behavior

:::{rule} OBJ-1
:kind: required
:tests: foundations/objects/lifetime-begins-after-init.cpp foundations/objects/members-subobjects-end-after-body.cpp foundations/objects/scalar-lifetime-with-storage.cpp foundations/objects/construction-order.cpp

The lifetime of a class object begins when its constructor completes
and ends when its destructor completes. Its bases and members are
initialized before the constructor body runs and destroyed after the
destructor body runs. The bases come first, in the order of the base
list, then the members, in the order of their declarations. Destruction
runs in the reverse order. The lifetime of a scalar object begins when its
storage is obtained, before it has a value.
:::

:::{rule} OBJ-3
:kind: required
:tests: foundations/objects/block-reverse-order.cpp foundations/objects/nested-block-ends-first.cpp foundations/objects/early-return-destroys.cpp foundations/objects/loop-body-each-iteration.cpp foundations/objects/constant-evaluation-same-order.cpp

Objects with automatic storage duration are destroyed when control
leaves their block, in the reverse of the order of their construction.
This applies to a `return`, and to each iteration of a loop. Constant
evaluation follows the same order.
:::

:::{rule} OBJ-4
:kind: required
:tests: foundations/objects/static-order.cpp foundations/objects/function-static-first-use.cpp foundations/objects/static-never-reached-not-constructed.cpp foundations/objects/thread-local-main-thread.cpp

Objects with static storage duration at namespace scope are constructed
before `main` runs, in the order of their definitions within a
translation unit. A static object local to a function is constructed
when control first reaches its declaration, and not at all if control
never does. These objects are destroyed after `main` returns, in the
reverse of the order in which their construction completed. A
`thread_local` object exists for the life of its thread, and the main
thread's objects are constructed by their first use at the latest.
:::

:::{rule} OBJ-5
:kind: required
:tests: foundations/objects/dynamic-outlives-scope.cpp foundations/objects/dynamic-not-destroyed-without-delete.cpp

An object that `new` creates lives until `delete` destroys it. The end
of the scope that created it has no effect on it, and without `delete`
its destructor never runs.
:::

:::{rule} OBJ-6
:kind: required
:tests: foundations/objects/array-elements-order.cpp

The elements of an array are constructed in order from the first and
destroyed in the reverse of that order.
:::

:::{rule} OBJ-7
:kind: required
:tests: foundations/objects/address-stable.cpp foundations/objects/distinct-objects-distinct-addresses.cpp

The address of an object does not change during its lifetime. Two
complete objects that exist at the same time have different addresses,
and so do two elements of one array. An object of an empty class has
size 1 for this reason.
:::

:::{rule} OBJ-8
:kind: required
:tests: foundations/objects/reference-does-not-extend-named.cpp foundations/objects/reference-to-temporary-extends.cpp

A reference to a named object does not extend the lifetime of that
object. A reference that binds to a temporary, or to a member of a
temporary, extends the temporary to the lifetime of the reference
(REF-4).
:::

:::{rule} OBJ-9
:kind: required
:tests: foundations/objects/temporary-ends-with-full-expression.cpp

A temporary lives while its full expression is evaluated and no longer.
A function that takes the temporary by reference returns first, and the
temporary ends after that (VC-6).
:::

## Undefined Behavior

:::{rule} OBJ-2
:kind: undefined
:tests: foundations/objects/read-before-init-constant.cpp foundations/objects/read-after-end-constant.cpp foundations/objects/uninitialized-member-read-constant.cpp foundations/objects/deleted-object-read-constant.cpp foundations/objects/reference-after-end-constant.cpp foundations/objects/read-after-block-ends-sanitizer.cpp foundations/objects/reference-does-not-extend-named.cpp

If a program reads an object outside its lifetime, or reads a scalar
object or member that has no value yet, the behavior is undefined. This
includes a read through a pointer or reference to a variable whose
block has ended, and a read of a dynamic object after `delete`.
Constant evaluation rejects each of these.
:::

:::{rule} OBJ-10
:kind: undefined
:tests: foundations/objects/storage-reuse-ends-old-object-constant.cpp

If the storage of an object is reused for another object, the first
object's lifetime ends. A later read through a reference to the first
object is undefined, and constant evaluation rejects it.
:::

## Implementation Requirements

:::{rule} OBJ-11
:kind: unspecified
:tests: foundations/objects/thread-local-init-point.cpp

It is unspecified if a `thread_local` object with a constructor is
constructed before `main` runs or when the thread first uses it. In the
matrix, the host compilers and GCC 14 construct it before `main`, and
the containers' Clang 18, Clang 22, and GCC 16 construct it at first
use.
:::

## Examples

```cpp
struct Log { int id; Log(int i); ~Log(); };   // constructor and destructor print id

Log first(1);                 // constructed before main
Log second(2);                // ... then this; destroyed in reverse after main

int main() {
  Log a(3);
  {
    Log b(4);                 // ends here, before c begins
  }
  Log* p = new Log(5);        // outlives every scope until delete
  Log c(6);
  delete p;                   // 5 ends now
}                             // then 6, then 3
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on all
27 tests. EDG runs no programs, so its results for the run tests say
only that the programs are valid.

- OBJ-11: the test accepts either construction point. Three containers
  construct the object at first use.
- OBJ-4: the test for the destruction order of static objects makes a
  static object declared first, and so destroyed last, check the order
  in its destructor.

Inputs: clang-ast-exprconstant, clang-diag-ast, clang-sema-decl.

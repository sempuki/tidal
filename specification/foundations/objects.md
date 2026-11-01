# Objects, storage, and lifetime

An object is a region of storage with a type. Every object has a storage
duration that says when its storage exists, and a lifetime that says
when the object itself exists in that storage. The rules for the
following topics follow from this model:

- subobjects,
- temporaries,
- the order in which objects end.

This page states the model once; the pages on initialization,
temporaries, constructors, and unions cite the model. The page on raw
storage ({principle}`STG-P2`) states how a program creates objects in
storage that it obtains itself, and how it reuses that storage.

Depends on: storage duration and linkage, value categories and
temporaries, constructors and destructors, new and delete.

## Principles

:::{principle} OBJ-P1
:rules: OBJ-1 OBJ-7

An object comes into existence when its initialization completes and
goes out of existence when its destructor completes, or when its storage
is released or reused. Between those two points the object has a stable
address, and no other complete object shares that address. An object
of scalar type has a lifetime that starts with its storage, so the
object can exist with no value yet.

A class object is built from its subobjects, and the lifetimes of the
subobjects nest inside the lifetime of the class object. So the
constructor body and the destructor body both see complete parts:

```text
                                bases and      the object
                                members exist  exists
  bases and members are built        |
  constructor body runs              |
  initialization completes           |              |
  ...                                |              |
  destructor body runs               |              |
  destructor completes                              |
```
:::

:::{principle} OBJ-P2
:rules: OBJ-3 OBJ-4 OBJ-5 OBJ-6 OBJ-9
:exceptions: OBJ-11

The storage duration of an object decides who ends the object and when.
In every case, objects end in the reverse of the order in which they
began.

| Objects | Ended by | When |
|---|---|---|
| Automatic objects | Their block | When control leaves the block, by any route |
| Static objects | The program | After `main` returns, in the reverse of their construction order. That order is the order of their definitions at namespace scope, and the order of first use for objects local to a function. |
| Array elements | The array | From the last to the first |
| A temporary | Its full expression | When the full expression that made it ends |
| A dynamic object | `delete` | No fixed end. It exists until `delete` ends it, however many scopes have come and gone. |
:::

:::{principle} OBJ-P3
:rules: OBJ-8

A reference is not an object. Binding a reference to an object gives
the object no extra life, so a reference can outlive what it refers to.
If a reference binds to a temporary, the temporary lives as long as the
reference, because otherwise the reference would be dead at birth.
:::

:::{principle} OBJ-P4
:rules: OBJ-2 OBJ-10

Outside its lifetime an object has no value, and a program must not read
or write the object. This principle covers the following objects:

- a scalar before its first write,
- a member that no initializer gave a value,
- a variable whose block has ended,
- a dynamic object after `delete`,
- an object whose storage another object has taken.

Because constant evaluation tracks every lifetime, it rejects each of
these cases.
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
runs in the reverse order. The lifetime of a scalar object begins when
its storage is obtained, before the object has a value.
:::

:::{rule} OBJ-3
:kind: required
:tests: foundations/objects/block-reverse-order.cpp foundations/objects/nested-block-ends-first.cpp foundations/objects/early-return-destroys.cpp foundations/objects/loop-body-each-iteration.cpp foundations/objects/constant-evaluation-same-order.cpp

Objects with automatic storage duration are destroyed when control
leaves their block, in the reverse of the order of their construction.
This rule applies to a `return`, to each iteration of a loop, and to
constant evaluation. The rule assumes that a function body runs to its
end without suspension *(coroutines referenced)*.
:::

:::{rule} OBJ-4
:kind: required
:tests: foundations/objects/static-order.cpp foundations/objects/function-static-first-use.cpp foundations/objects/static-never-reached-not-constructed.cpp foundations/objects/thread-local-main-thread.cpp

Objects with static storage duration at namespace scope are constructed
before `main` runs, in the order of their definitions within a
translation unit. A static object local to a function is constructed
when control first reaches its declaration, and not at all if control
never does. Both kinds of object are destroyed after `main` returns, in
the reverse of the order in which their construction completed. A
`thread_local` object exists for the life of its thread, and the main
thread's objects are constructed by their first use at the latest.
:::

:::{rule} OBJ-5
:kind: required
:tests: foundations/objects/dynamic-outlives-scope.cpp foundations/objects/dynamic-not-destroyed-without-delete.cpp

An object that `new` creates lives until `delete` destroys it. The end
of the scope that created the object has no effect on the object, and
without `delete` its destructor never runs.
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
size 1 because of this requirement.
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
temporary ends after the function returns (VC-6).
:::

## Undefined Behavior

:::{rule} OBJ-2
:kind: undefined
:tests: foundations/objects/read-before-init-constant.cpp foundations/objects/read-after-end-constant.cpp foundations/objects/uninitialized-member-read-constant.cpp foundations/objects/deleted-object-read-constant.cpp foundations/objects/reference-after-end-constant.cpp foundations/objects/read-after-block-ends-sanitizer.cpp foundations/objects/reference-does-not-extend-named.cpp expressions/constant-expressions/copy-unset-member.cpp expressions/constant-expressions/copy-unset-array-element.cpp declarations/initialization/forms/default-init-member-no-value.cpp declarations/initialization/forms/default-init-member-not-named-no-value.cpp statements/switch/skipped-declaration-read-first.cpp declarations/initialization/static/member-read-before-constructor-constant.cpp expressions/unary/indirection-dangling-constant.cpp classes/constructors/use-after-dtor-constant.cpp classes/constructors/use-after-delete-sanitizer.cpp foundations/objects/write-after-end-constant.cpp classes/unions/chain-assign-other-part-no-value-constant.cpp

If a program reads or writes an object outside its lifetime, or reads
a scalar object or member that has no value yet, the behavior is
undefined. OBJ-2 is the rule for each such access, and these cases are
included:

- an access through a pointer or reference to a variable whose block
  has ended,
- an access to a dynamic object after `delete`,
- an access to a class object after its destructor runs,
- an implicit copy or move function ({rule}`CPY-8`) of an object with a
  scalar part that has no value, because the function reads each scalar
  part.

{rule}`UN-7` and {rule}`DTOR-7` refer to OBJ-2 for these accesses.
Constant evaluation rejects each of them.
:::

:::{rule} OBJ-10
:kind: undefined
:tests: foundations/objects/storage-reuse-ends-old-object-constant.cpp

If the storage of an object is reused for another object, the first
object's lifetime ends. A later read through a reference to the first
object is undefined, and constant evaluation rejects the read.
{rule}`STG-9` gives the case where the reference reaches the new object
instead.
:::

## Implementation Requirements

:::{rule} OBJ-11
:kind: unspecified
:tests: foundations/objects/thread-local-init-point.cpp

The implementation constructs a `thread_local` object with a constructor
before `main` runs or when the thread first uses the object, and the
choice is unspecified. In the matrix, the host compilers and GCC 14
construct the object before `main`, and the containers' Clang 18, Clang
22, and GCC 16 construct it at first use.
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 38
of 40 tests. EDG runs no programs, so its results for the run tests say
only that the programs are valid.

- OBJ-11: the test accepts either construction point. Three containers
  construct the object at first use.
- OBJ-4: the test for the destruction order of static objects makes a
  static object declared first, and so destroyed last, check the order
  in its destructor.
- OBJ-2: a read or write after the end of a lifetime breaks OBJ-2, and
  the tests of such accesses cite OBJ-2. UN-7 and DTOR-7 name OBJ-2 for
  these accesses, in the way that decision 0042 links overlapping
  `reject` rules. Every compiler rejects a read or a write through a
  pointer to a variable whose block has ended during constant
  evaluation.
- OBJ-2: an implicit copy of an object with a part that has no value is
  a read of that part ({question}`Q-0115`). No sanitizer in the matrix
  reports the copy at run time.
- OBJ-10: the `vptr` sanitizer of GCC stops a read of a member through
  a reference to an object whose storage another object reuses. The
  Clang sanitizers do not, so no run test covers the rule.

Inputs: clang-ast-exprconstant, clang-diag-ast, clang-sema-decl.

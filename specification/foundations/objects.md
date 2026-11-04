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
:rules: OBJ-1 OBJ-6 OBJ-7

An object comes into existence when its initialization completes and
goes out of existence when its destructor is called, or when its storage
is released or reused. Between those two points the object has a stable
address, and no other complete object shares that address. An object
of {term}`scalar type` that has no initializer begins its lifetime with its
storage, so the object can exist with no value yet.

These events end the lifetime of an object:

| Event | Objects | Rule |
|---|---|---|
| Control leaves the block | automatic objects | OBJ-3 |
| The {term}`full expression` ends | temporaries | OBJ-9 |
| `delete` | dynamic objects | OBJ-5 |
| The program or the thread ends | static and thread objects | OBJ-4 |
| An explicit destructor call | any object | {rule}`DTOR-6` |
| The enclosing object or array ends | {term}`subobjects <subobject>` and elements | OBJ-1, OBJ-6 |
| Another object takes the storage | any object | OBJ-10, {rule}`STG-6` |
| The storage is released, without a destructor call | any object | {rule}`STG-27` |

A class object is built from its subobjects. The constructor body runs
before the lifetime of the object begins, and the destructor body runs
after the lifetime ends. Both bodies see complete parts, because the
bases and members exist during both bodies:

```text
                                   bases and      the object
                                   members exist  exists
  bases and members are built           |
  constructor body runs                 |
  initialization completes              |              |
  ...                                   |              |
  destructor call starts                |
  destructor body runs                  |
  bases and members are destroyed
```
:::

:::{principle} OBJ-P2
:rules: OBJ-3 OBJ-4 OBJ-5 OBJ-6 OBJ-9 OBJ-12
:exceptions: OBJ-11

The {term}`storage duration` of an object decides who ends the object and when.
In every case, objects end in the reverse of the order in which they
began.

| Objects | Ended by | When |
|---|---|---|
| Automatic objects | Their block | When control leaves the block, by any route |
| Thread objects | Their thread | When the thread ends. The objects of the main thread end before every static object. |
| Static objects | The program | After `main` returns, or when the program calls the exit function *(library referenced)*, in the reverse of their construction order. That order follows the definitions within a {term}`translation unit` ({rule}`SINIT-2`), and the first use for objects local to a function. |
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

Because {term}`constant evaluation` tracks every lifetime, it rejects each of
these cases.
:::

## Runtime Behavior

:::{rule} OBJ-1
:kind: required
:tests: foundations/objects/lifetime-begins-after-init.cpp foundations/objects/scalar-lifetime-with-storage.cpp exceptions/unwinding/delegating-target-completed-destructor-runs.cpp foundations/objects/scalar-write-in-own-initializer-constant.cpp foundations/objects/scalar-write-through-pointer-in-own-initializer-constant.cpp

The {term}`lifetime` of an object begins and ends at these points:

| Object | Lifetime begins | Lifetime ends |
|---|---|---|
| A class object | Its initialization completes. That is the constructor, with the body of a delegating constructor, or the initialization of each element of an aggregate. | Its destructor call starts. |
| A scalar object | Its storage is obtained and its initializer completes. A scalar with no initializer begins with its storage, before it has a value. | It is destroyed ({rule}`DTOR-6`), or its storage is released or reused (OBJ-10). |

So a read or a write of a scalar during its own initializer is outside
its lifetime ({rule}`OBJ-2`, {question}`Q-0368`).
An object that {rule}`STG-8` creates begins its lifetime when the
operation creates it, and {rule}`STG-27` gives the end of storage.
:::

:::{rule} OBJ-3
:kind: required
:tests: foundations/objects/block-reverse-order.cpp foundations/objects/nested-block-ends-first.cpp foundations/objects/early-return-destroys.cpp foundations/objects/loop-body-each-iteration.cpp foundations/objects/constant-evaluation-same-order.cpp foundations/objects/unbraced-substatement-ends-object.cpp

Objects with {term}`automatic storage` duration are destroyed when control
leaves their block, in the reverse of the order of their construction.
A branch or a loop body without braces counts as a block here
({rule}`IF-8`, {rule}`LOOP-4`). This rule applies to every exit, such as
the end of the block, a jump, a `return`, an exception, and the end of
each iteration of a loop. The rule also applies to constant
evaluation. The rule assumes that a function body runs to its
end without suspension *(coroutines referenced)*.
:::

:::{rule} OBJ-4
:kind: required
:tests: foundations/objects/static-order.cpp foundations/objects/function-static-first-use.cpp foundations/objects/static-never-reached-not-constructed.cpp foundations/objects/thread-local-main-thread.cpp foundations/objects/static-constant-initialized-destroyed-in-place.cpp foundations/objects/thread-local-destroyed-before-static.cpp

Objects with static or thread storage duration begin and end at these
points:

- An object at namespace scope is constructed in the order of
  {rule}`SINIT-2` and {rule}`SINIT-3`. {rule}`SINIT-5` gives whether
  the construction happens before `main` runs.
- A static object local to a function is constructed when control first
  reaches its declaration, and not at all if control never does.
- Static objects are destroyed after `main` returns, in the reverse of
  the order in which their construction completed. A constant-initialized
  object ({rule}`SINIT-1`) takes the place that a dynamic initialization at
  its definition would give it.
- A `thread_local` object exists for the life of its thread
  ({rule}`SINIT-6`). The main thread's objects are destroyed before any
  static object.

{rule}`UNW-5` gives the case of an exception that leaves such a
construction or destruction.
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
:tests: foundations/objects/array-elements-order.cpp foundations/objects/construction-order.cpp foundations/objects/members-subobjects-end-after-body.cpp foundations/objects/virtual-bases-first.cpp

The parts of an object are constructed in this order and destroyed in
the reverse order:

- In a class object, the {term}`virtual bases <virtual base>` come
  first, in a depth-first, left-to-right walk of the graph of bases
  ({rule}`DER-7`). The other direct bases follow in the order of the
  base list, then the members, in the order of their declarations. The
  parts are constructed before the constructor body runs and destroyed
  after the destructor body runs.
- In an array, the elements come in order from the first.
:::

:::{rule} OBJ-7
:kind: required
:tests: foundations/objects/address-stable.cpp foundations/objects/distinct-objects-distinct-addresses.cpp

The address of an object does not change during its lifetime. Two
complete objects that exist at the same time have different addresses,
and so do two elements of one array. String literal objects are the
exception, because they can share storage ({rule}`PRIM-9`). An object of
an empty class has a size above zero because of this requirement
({rule}`CLS-11`).
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
:tests: foundations/objects/temporary-ends-with-full-expression.cpp foundations/objects/temporary-member-access.cpp

A temporary lives while its full expression is evaluated, unless a
rule gives it another end. A function that takes the temporary by
reference returns first, and the temporary ends after the function
returns (VC-6). {rule}`REF-4` and {rule}`RFOR-4` extend a temporary,
and {rule}`VC-9` ends the temporaries of a default argument for each
element of an array.
:::

## Undefined Behavior

:::{rule} OBJ-2
:kind: undefined
:tests: foundations/objects/read-before-init-constant.cpp foundations/storage/allocation-array-unbuilt-element-constant.cpp foundations/objects/read-after-end-constant.cpp foundations/objects/uninitialized-member-read-constant.cpp foundations/objects/deleted-object-read-constant.cpp foundations/objects/reference-after-end-constant.cpp foundations/objects/read-after-block-ends-sanitizer.cpp foundations/objects/reference-does-not-extend-named.cpp expressions/constant-expressions/copy-unset-member.cpp expressions/constant-expressions/copy-unset-array-element.cpp declarations/initialization/forms/default-init-member-no-value.cpp declarations/initialization/forms/default-init-member-not-named-no-value.cpp statements/switch/skipped-declaration-read-first.cpp declarations/initialization/static/member-read-before-constructor-constant.cpp expressions/unary/indirection-dangling-constant.cpp classes/constructors/use-after-dtor-constant.cpp classes/constructors/use-after-delete-sanitizer.cpp foundations/objects/write-after-end-constant.cpp classes/unions/chain-assign-other-part-no-value-constant.cpp declarations/initialization/references/returned-reference-temporary-sanitizer.cpp declarations/functions/return-deduction/decltype-auto-paren-local-sanitizer.cpp foundations/objects/scalar-write-in-own-initializer-constant.cpp foundations/objects/scalar-write-through-pointer-in-own-initializer-constant.cpp classes/constructors/init-order-reads-later-run.cpp foundations/objects/storage-reuse-ends-old-object-sanitizer.cpp classes/constructors/base-init-calls-member-of-member-constant.cpp

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
  part. An unnamed bit-field is padding and no such part ({rule}`BF-12`).

Constant evaluation rejects each of them. The specific cases of OBJ-2
are {rule}`CTOR-12`, {rule}`DTOR-7`, {rule}`INIT-1`, {rule}`NEW-3`,
{rule}`OBJ-10`, {rule}`REF-5`, {rule}`RET-6`, {rule}`STG-19`, and
{rule}`UNI-6` (decision 0052). {rule}`UN-7` refers to OBJ-2 for these
accesses.
:::

:::{rule} OBJ-10
:kind: undefined
:tests: foundations/objects/storage-reuse-ends-old-object-constant.cpp foundations/objects/storage-reuse-member-of-member-constant.cpp foundations/objects/storage-reuse-ends-old-object-sanitizer.cpp

If the storage of an object is reused for another object, the first
object's lifetime ends. A later read through a reference to the first
object is undefined, and constant evaluation rejects the read.
{rule}`STG-9` gives the case where the reference reaches the new object
instead.
:::

:::{rule} OBJ-12
:kind: undefined
:tests: foundations/objects/static-local-after-destruction.cpp foundations/objects/static-local-used-by-earlier-static.cpp foundations/objects/thread-local-local-after-destruction.cpp

A `static` or `thread_local` variable of a block can end (OBJ-4) while
another `static` or `thread_local` object has yet to be destroyed. If
the destruction of that other object then calls the function, and
control passes through the declaration of the ended variable, the
behavior is undefined. An example is a call of the
function from the destructor of another static object that ends later.

Other code that runs after the end, such as a function registered to
run at program exit, falls under {rule}`OBJ-2` when it uses the ended
object.
:::

## Implementation Requirements

:::{rule} OBJ-11
:kind: impl-defined
:tests: foundations/objects/thread-local-init-point.cpp foundations/objects/thread-local-member-init-point.cpp foundations/objects/thread-local-constant-variable-starts-nothing.cpp foundations/objects/thread-local-started-by-other-variable.cpp

The implementation constructs a `thread_local` object at namespace
scope that is not `inline` and has a constructor either before the
thread runs its first statement, or later. The choice is
implementation-defined, as {rule}`SINIT-6` gives. A later construction
happens before the thread first uses the object, or another
`thread_local` variable of the same translation unit that is not
`inline` and has dynamic initialization. A use of a variable with
constant initialization does not start it. At which point before that
use, and in which threads, the construction runs is also
implementation-defined.

Every compiler of the matrix defers the construction. The first use of
one such variable of the unit constructs all of them.
:::

## Examples

```cpp
struct Log { int id; Log(int i); ~Log(); };   // constructor and destructor print id

Log first(1);                 // constructed first (before main on the matrix)
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 56
of 61 tests. EDG runs no programs, so its results for the run tests say
only that the programs are valid.

- OBJ-11: the tests accept either construction point for one object.
  Every Clang and GCC configuration defers the construction, and the
  first use of one `thread_local` variable of the unit with dynamic
  initialization constructs the others too. The choice is
  implementation-defined, as for {rule}`SINIT-6`. The gap review of the
  rules that phase 7 added
  (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) reads
  the reference as starting a deferred construction before the first
  use of any `thread_local` variable of the unit. A use of a variable
  with constant initialization starts nothing on Clang and GCC, and
  OBJ-11 keeps that result (`documentation/decisions/pending.md`,
  row 4).
- OBJ-1: during constant evaluation, Clang rejects a write to a scalar
  in its own initializer, directly or through a pointer. GCC and EDG
  accept both writes ({question}`Q-0368`).
- OBJ-1: the lifetime of a class object ends when its destructor call
  starts. The destructor body and its parts still run, as
  {principle}`OBJ-P1` draws. No compiler of the matrix gives a result
  that separates this point from the end of the destructor.
- OBJ-4: Clang and GCC destroy a constant-initialized object at the
  place of its definition in the order. A dynamically initialized
  object defined before it is destroyed after it. Clang and GCC destroy
  the main thread's `thread_local` objects before the static objects.
- OBJ-4: the order of construction follows {rule}`SINIT-2`. A class
  template's static data member is not ordered with the other
  variables of its unit ({rule}`SINIT-3`).
- OBJ-6: every compiler builds the virtual bases first, in the order of
  a depth-first walk of the bases.
- OBJ-12: Clang and GCC run the tests, and no sanitizer of the matrix
  reports the pass through the declaration, also for a `thread_local`
  block variable. The test checks only that
  the program compiles. Only the front end's interpreter stops OBJ-12
  at run time. The interpreter stops at the pass through the declaration
  in each of the three tests and names OBJ-12 (decision 0053).
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
  Clang sanitizers do not. The `vptr` sanitizers of Clang and GCC both
  stop a call of a member function through such a reference
  (`storage-reuse-ends-old-object-sanitizer.cpp`).

Inputs: clang-ast-exprconstant, clang-diag-ast, clang-sema-decl.

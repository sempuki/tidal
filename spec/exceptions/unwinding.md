# Unwinding and termination

When an exception leaves a block, the objects of that block end in the
reverse of the order in which they began, as they would on any other
exit. This page covers:

- that unwinding,
- the parts of an object that exist when a constructor throws,
- the storage that a `new` expression obtained,
- the cases in which the program ends instead of unwinding.

These cases are:

- an exception that leaves a `noexcept` function,
- an exception that leaves a destructor during unwinding,
- an exception that leaves `main`,
- an exception with no handler.

Depends on: objects and lifetime, constructors and destructors,
exception specifications, new and delete.

## Principles

:::{principle} UNW-P1
:rules: UNW-1 UNW-4

A throw is a way of leaving a block, and leaving a block ends the
block's automatic objects in the reverse of their construction order
({principle}`OBJ-P2`). The search for a handler runs that exit for
every block between the throw and the handler, innermost first, so by
the time the handler runs, nothing that the thrown-from code created on
the way still exists:

```text
try {                        block C
  {                          block B
    {                        block A
      throw ...;   ------->  1. A's automatic objects end
    }                        2. B's automatic objects end
  }                          3. C's automatic objects end
}
catch (...) {                4. the handler runs
}
```

Objects that the throw did not reach are never created. Static objects
are not part of any of those blocks, so unwinding leaves them alone, and
an exception that leaves the initialization of a static local variable
leaves the variable not yet initialized, so the next call tries again.
:::

:::{principle} UNW-P2
:rules: UNW-2 UNW-3

A constructor that throws has built some parts and not others, and only
the parts that exist can be ended ({principle}`OBJ-P1`).

| What throws | What happens |
|---|---|
| A constructor | The bases and members that it completed are destroyed, in reverse. The object's own destructor never runs, because the object never existed. |
| The construction of an array | The same holds for the elements built so far. |
| A delegating constructor whose target completed | The object is complete, so its destructor does run. |
| The constructor in a `new` expression | The object never existed, but its storage does. The storage goes back through the deallocation function that matches the allocation function, placement arguments included, and only if such a match exists. |
:::

:::{principle} UNW-P3
:rules: UNW-5 UNW-6 UNW-8
:exceptions: UNW-7

An exception has to end in a handler. Where it cannot, the program ends
instead. A `noexcept` function promises that no exception leaves it, so
an exception that reaches its boundary ends the program, and the same
holds for a destructor, which is `noexcept` unless it says otherwise. A
destructor that runs during unwinding cannot throw either, because two
exceptions cannot be in flight at once. An exception that leaves `main`
or finds no handler at all has nowhere to go. A function can still
catch exceptions inside itself, and a destructor marked `noexcept(false)`
can throw when nothing else is in flight. The implementation decides
whether the objects on the stack are destroyed before the program ends
with no handler.
:::

## Runtime Behavior

:::{rule} UNW-1
:kind: required
:tests: exceptions/unwinding/block-objects-end-before-handler.cpp exceptions/unwinding/every-block-between-throw-and-handler.cpp exceptions/unwinding/nested-blocks-ended-in-order.cpp exceptions/unwinding/objects-after-throw-not-created.cpp exceptions/unwinding/handler-block-objects-unaffected.cpp

When an exception leaves a block, the automatic objects of that block
are destroyed in the reverse of their construction order. This repeats
for every block from the throw to the handler, innermost first, and all
of it completes before the handler runs. Objects declared after the
throw are not created. Objects of the handler's own block and of blocks
that enclose the try block are not affected.
:::

:::{rule} UNW-2
:kind: required
:tests: exceptions/unwinding/constructor-throws-parts-destroyed.cpp exceptions/unwinding/member-initializer-throws-earlier-parts-destroyed.cpp exceptions/unwinding/base-built-before-member-throws.cpp exceptions/unwinding/array-element-throws-built-destroyed.cpp exceptions/unwinding/delegating-target-completed-destructor-runs.cpp exceptions/unwinding/delegating-target-throws-no-destructor.cpp

These rules apply:

- If a constructor throws, from its body or from a member initializer,
  the bases and members that it completed are destroyed in reverse
  order, and the destructor of the object itself does not run.
- If an element of an array throws during construction, the elements
  built before it are destroyed in reverse order.
- If the target of a delegating constructor completed, the object is
  complete, and its destructor runs when the delegating constructor
  throws.
:::

:::{rule} UNW-3
:kind: required
:tests: exceptions/unwinding/new-constructor-throws-storage-released.cpp exceptions/unwinding/new-array-constructor-throws-storage-released.cpp exceptions/unwinding/placement-new-throws-placement-delete.cpp exceptions/unwinding/placement-new-no-matching-delete-leaks.cpp exceptions/unwinding/new-initializer-throws-global-delete.cpp

If the initialization in a `new` expression throws, the storage is
released by the deallocation function that matches the allocation
function that obtained it. The matching function is:

- the class-specific one if the allocation was class-specific,
- the array form for an array,
- a placement deallocation function with the same extra parameters for
  a placement form.

If no matching deallocation function exists, the storage is not
released. For the global allocation function, the implementation calls
the sized or the unsized global deallocation function.
:::

:::{rule} UNW-4
:kind: required
:tests: exceptions/unwinding/static-objects-survive-unwinding.cpp exceptions/unwinding/static-local-init-throws-retry.cpp

Unwinding does not destroy objects with static storage duration. If the
initialization of a static local variable throws, the variable is not
initialized, and the next time control reaches its declaration the
initialization runs again.
:::

:::{rule} UNW-5
:kind: required
:tests: exceptions/unwinding/leaves-noexcept-function-terminates.cpp exceptions/unwinding/leaves-implicit-noexcept-destructor-terminates.cpp exceptions/unwinding/leaves-destructor-during-unwinding-terminates.cpp exceptions/unwinding/leaves-main-terminates.cpp exceptions/unwinding/no-handler-terminates.cpp exceptions/unwinding/noexcept-boundary-is-the-function.cpp

The program ends, without returning from `main`, when an exception:

- leaves a function declared `noexcept`, also when the throw is in a
  function that it calls,
- leaves a destructor that is not declared `noexcept(false)`,
- leaves a destructor that runs during the unwinding of another
  exception,
- leaves `main`,
- has no matching handler in any enclosing try block.
:::

:::{rule} UNW-6
:kind: required
:tests: exceptions/unwinding/noexcept-false-destructor-can-throw.cpp exceptions/unwinding/noexcept-function-handles-inside.cpp

A `noexcept` function can throw and catch exceptions inside itself. A
destructor declared `noexcept(false)` can throw when no other exception
is being unwound, and a handler outside it catches the exception after
the destructor's object has ended.
:::

## Implementation Requirements

:::{rule} UNW-7
:kind: impl-defined
:tests: exceptions/unwinding/no-handler-unwinding-unobservable.cpp

If an exception has no matching handler, it is implementation-defined
whether the automatic objects between the throw and `main` are
destroyed before the program ends.
:::

## Recommended Practice

:::{rule} UNW-8
:kind: recommended
:tests: exceptions/unwinding/throw-in-noexcept-warns.cpp exceptions/unwinding/call-in-noexcept-no-warning.cpp exceptions/throw/throw-in-noexcept-warns.cpp

An implementation should warn about a `throw` expression written
directly in a `noexcept` function outside any try block that can catch
it. A call to a function that might throw does not need a warning.
:::

## Examples

```cpp
struct Lock { Lock(); ~Lock(); };
struct Buffer { Buffer(int n); ~Buffer(); };

void load() {
  Lock lock;                   // ends second, during unwinding
  Buffer data(1024);           // ends first
  throw 1;                     // nothing after this runs
}

int main() {
  try { load(); }
  catch (int) {}               // runs after data and lock have ended
}

struct Pair {
  Buffer a;
  Buffer b;
  Pair() : a(10), b(-1) {}     // if b's constructor throws, a is destroyed
};                             // and ~Pair never runs

void closed() noexcept { throw 2; }   // ends the program
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on all
30 tests. EDG runs no programs, so its results say only that the
programs are valid.

- UNW-5 and UNW-7: the harness sees only that the program ended with a
  nonzero status or a signal. It cannot see why, and it cannot see what
  the implementation did before ending. The test for UNW-7 shows the
  program ends; the choice it documents is not observable through the
  harness.
- UNW-3: the global deallocation test replaces both the sized and the
  unsized `operator delete`. Clang 18 calls the unsized form after a
  failed `new`, and the other compilers call the sized form.
- UNW-8: Clang and GCC warn under `-Wall`.

Inputs: clang-sema-declcxx, clang-sema-expr, clang-diag-sema,
edg-decls.

# Unwinding and termination

When an exception leaves a block, the objects of that block end in the
reverse of the order in which they began, as they would on any other
exit. This page covers:

- unwinding,
- the parts of an object that exist when a constructor throws,
- the storage that a `new` expression obtained,
- the cases in which the program ends instead of unwinding.

The cases in which the program ends are:

- an exception that leaves a `noexcept` function,
- an exception that leaves a destructor during unwinding,
- an exception that leaves `main`,
- an exception with no handler.

Depends on: objects and lifetime, constructors and destructors,
exception specifications, new and delete.

## Principles

:::{principle} UNW-P1
:rules: UNW-1 UNW-4 UNW-9

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

The temporaries of the full expression that threw end first, in the
reverse of their creation, and the rest of that full expression never
runs. Objects that the throw did not reach are never created. Static
objects are not part of any of those blocks, so unwinding leaves them
alone, and an exception that leaves the initialization of a static local
variable leaves the variable not yet initialized, so the next call tries
again. These rules assume that a function body runs to its end without
suspension *(coroutines referenced)*.
:::

:::{principle} UNW-P2
:rules: UNW-2 UNW-3

A constructor that throws has built some parts and not others, and only
the parts that exist can be ended ({principle}`OBJ-P1`).

| What throws | What happens |
|---|---|
| A constructor | The bases and members that it completed are destroyed, in reverse. The object's own destructor never runs, because the object never existed. |
| The construction of an array | The same holds for the elements built so far. |
| One placement `new` in a loop that builds elements one at a time | Only that element's parts are destroyed. The elements that earlier iterations built stay, and the program's own handler must destroy them. |
| A delegating constructor whose target completed | The object is complete, so its destructor does run. |
| The constructor in a `new` expression | The object never existed, but its storage does. The storage goes back through the deallocation function that matches the allocation function, placement arguments included, and only if such a match exists. |
:::

:::{principle} UNW-P3
:rules: UNW-5 UNW-6 UNW-8
:exceptions: UNW-7

An exception has to end in a handler. Where an exception cannot end in a
handler, the program ends instead. A `noexcept` function promises that
no exception leaves the function, so an exception that reaches the
function's boundary ends the program, and the same holds for a
destructor, which is `noexcept` unless the destructor says otherwise. A
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
are destroyed in the reverse of their construction order. The
destruction repeats for every block from the throw to the handler,
innermost first, and all of the destruction completes before the handler
runs. Objects declared after the
throw are not created. Objects of the handler's own block and of blocks
that enclose the try block are not affected.
:::

:::{rule} UNW-2
:kind: required
:tests: exceptions/unwinding/constructor-throws-parts-destroyed.cpp exceptions/unwinding/member-initializer-throws-earlier-parts-destroyed.cpp exceptions/unwinding/base-built-before-member-throws.cpp exceptions/unwinding/array-element-throws-built-destroyed.cpp exceptions/unwinding/delegating-target-completed-destructor-runs.cpp exceptions/unwinding/delegating-target-throws-no-destructor.cpp exceptions/unwinding/placement-loop-no-automatic-cleanup.cpp

These rules apply:

- If a constructor throws, from its body or from a member initializer,
  the bases and members that it completed are destroyed in reverse
  order, and the destructor of the object itself does not run.
- If an element of an array throws during construction, the elements
  built before that element are destroyed in reverse order.
- If the target of a delegating constructor completed, the object is
  complete, and its destructor runs when the delegating constructor
  throws.

The second rule covers an array that one declaration or one `new`
expression builds. Objects that a program builds one at a time, for
example with placement `new` in a loop, get no such cleanup.
:::

:::{rule} UNW-3
:kind: required
:tests: exceptions/unwinding/new-constructor-throws-storage-released.cpp exceptions/unwinding/new-array-constructor-throws-storage-released.cpp exceptions/unwinding/placement-new-throws-placement-delete.cpp exceptions/unwinding/placement-new-no-matching-delete-leaks.cpp exceptions/unwinding/new-initializer-throws-global-delete.cpp

If the initialization in a `new` expression throws, the storage is
released by the deallocation function that matches the allocation
function that obtained the storage. The matching function is:

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

- leaves a non-throwing function ({rule}`EXS-4`), also when the throw is
  in a function that the non-throwing function calls,
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
is being unwound, and a handler outside the destructor catches the
exception after the destructor's object has ended.
:::

:::{rule} UNW-9
:kind: required
:tests: exceptions/unwinding/full-expression-temporaries-unwound.cpp

When an exception leaves a full expression, the temporaries and
parameter objects of that full expression that still exist
({rule}`VC-10`) end, last created first. These objects end before the
automatic objects of the block. No later part of the full expression
runs.
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
:tests: exceptions/unwinding/throw-in-noexcept-warns.cpp exceptions/unwinding/call-in-noexcept-no-warning.cpp exceptions/throw/throw-in-noexcept-warns.cpp exceptions/unwinding/rethrow-in-noexcept-warns.cpp exceptions/unwinding/throw-caught-in-noexcept-no-warning.cpp exceptions/unwinding/throw-unmatched-in-noexcept-warns.cpp

An implementation should warn about a `throw` expression in a
non-throwing function if no enclosing try block in the function can
catch the exception. An enclosing try block can catch:

- a `throw` with an operand, if one of its handlers is `catch (...)` or
  matches the static type of the operand ({rule}`TRY-2`, {rule}`THR-4`),
- a rethrow, if one of its handlers is `catch (...)`. A rethrow in a
  handler is not enclosed by the try block of that handler
  ({rule}`TRY-8`).

A call to a function that might throw does not need a warning.
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

A growable array builds its elements in new storage with placement
`new` ({doc}`/foundations/storage`). If a copy throws, the handler
destroys the elements built so far, frees the storage, and rethrows.
The old storage is not touched, so the array keeps its elements:

```cpp
template <class T>
T* copy_into_new_storage(const T* from, int count, int capacity) {
  T* fresh = static_cast<T*>(::operator new(sizeof(T) * capacity));
  int built = 0;
  try {
    for (; built != count; ++built)
      new (fresh + built) T(from[built]);  // can throw
  } catch (...) {
    while (built > 0) fresh[--built].~T(); // no automatic cleanup (UNW-2)
    ::operator delete(fresh);
    throw;                                 // the caller keeps from
  }
  return fresh;                            // the caller then ends the old elements
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 34
of 35 tests. EDG runs no programs, so its results say only that the
programs are valid.

- UNW-5 and UNW-7: the harness sees only that the program ended with a
  nonzero status or a signal. The harness cannot see why the program
  ended or what the implementation did before ending. The test for UNW-7
  shows the program ends; the choice that the test documents is not
  observable through the harness.
- UNW-3: the global deallocation test replaces both the sized and the
  unsized `operator delete`. Clang 18 calls the unsized form after a
  failed `new`, and the other compilers call the sized form.
- UNW-8: Clang and GCC warn under `-Wall`. Clang warns when no handler
  matches the static type of the operand, and GCC does not. GCC warns
  about a rethrow in a handler whose try block holds no direct `throw`,
  and Clang does not ({question}`Q-0179`).
- UNW-9: Clang and GCC destroy the temporaries of the full expression,
  then the parameter object, then the block's objects, before the
  handler runs. The test also accepts a parameter that ends at the
  return of its function ({rule}`VC-10`).

Inputs: clang-sema-declcxx, clang-sema-expr, clang-diag-sema,
edg-decls.

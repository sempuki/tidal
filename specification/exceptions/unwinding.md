# Unwinding and termination

When an exception leaves a block, the objects of that block end in the
reverse of the order in which they began, as they would on any other
exit. This page covers:

- unwinding,
- the parts of an object that exist when a constructor throws,
- the storage that a `new` expression obtained,
- the cases in which the program ends instead of unwinding.

{rule}`UNW-5` lists the cases in which the program ends.

Depends on: objects and lifetime, constructors and destructors,
exception specifications, new and delete.

## Principles

:::{principle} UNW-P1
:rules: UNW-1 UNW-4

A throw is a way of leaving a block, and leaving a block ends the
block's automatic objects in the reverse of their construction order
({principle}`OBJ-P2`). {term}`Unwinding <unwinding>` runs that exit for
the full expression that threw and for every block between the throw
and the {term}`handler`, innermost first. By the time the handler runs, nothing
that the code inside the try block created still exists. In this
diagram, `fail()` throws:

```text
try {                           block C
  {                             block B
    {                           block A
      Temp().take(fail());  --> 1. the Temp temporary ends
    }                           2. A's automatic objects end
  }                             3. B's automatic objects end
}                               4. C's automatic objects end
catch (...) {                   5. the handler runs
}
```

No later part of the full expression runs, and objects that the throw
did not reach are never created. Static objects are not part of any of
those blocks, so unwinding leaves them alone. An exception that leaves
the initialization of a static local variable leaves the variable not
yet initialized, and the next call tries again. These rules assume that
a function body runs to its end without suspension *(coroutines referenced)*.
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
:rules: UNW-5 UNW-6 UNW-8 UNW-10
:exceptions: UNW-7

An exception has to end in a handler. Where an exception cannot end in a
handler, the program ends instead. A `noexcept` function promises that
no exception leaves the function, so an exception that reaches the
function's boundary ends the program. The same holds for a destructor,
which is `noexcept` unless it says otherwise or the destructor of a
base or member can throw ({rule}`DTOR-4`). A
destructor that runs during unwinding cannot throw either, because two
exceptions cannot be in flight at once. An exception that leaves `main`
or finds no handler at all has nowhere to go. A function can still
catch exceptions inside itself, and a destructor that is not `noexcept`
can throw when nothing else is in flight. When an exception finds no
handler or reaches a `noexcept` boundary, the implementation decides
which objects on the stack are destroyed before the program ends.

The end is abnormal. The program does not return from `main`, and the
implementation chooses what the environment sees (UNW-10). A
termination handler of the library can run code at that point
*(library referenced)*. A destructor cannot tell from the language
alone whether it runs because of an exception, so a cleanup object that
must act only on failure keeps a flag that the success path sets.
:::

## Runtime Behavior

:::{rule} UNW-1
:kind: required
:tests: exceptions/unwinding/block-objects-end-before-handler.cpp exceptions/unwinding/every-block-between-throw-and-handler.cpp exceptions/unwinding/nested-blocks-ended-in-order.cpp exceptions/unwinding/objects-after-throw-not-created.cpp exceptions/unwinding/handler-block-objects-unaffected.cpp exceptions/unwinding/full-expression-temporaries-unwound.cpp

When an exception leaves a {term}`full expression`, these objects
end, in this order, before the handler runs:

1. the temporaries and parameter objects of that full expression that
   still exist ({rule}`VC-10`), last created first,
2. the automatic objects of each block from the throw to the handler,
   innermost block first, each block in the order of {rule}`OBJ-3`.

No later part of the full expression runs, and objects declared after
the throw are not created. Objects of the handler's own block and of
blocks that enclose the try block are not affected.
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

Unwinding does not destroy objects with static {term}`storage duration`. If the
initialization of a static local variable throws, the variable is not
initialized, and the next time control reaches its declaration the
initialization runs again.
:::

:::{rule} UNW-5
:kind: required
:tests: exceptions/unwinding/leaves-noexcept-function-terminates.cpp exceptions/unwinding/leaves-implicit-noexcept-destructor-terminates.cpp exceptions/unwinding/leaves-destructor-during-unwinding-terminates.cpp exceptions/unwinding/leaves-main-terminates.cpp exceptions/unwinding/no-handler-terminates.cpp exceptions/unwinding/noexcept-boundary-is-the-function.cpp exceptions/unwinding/leaves-destructor-member-throwing-caught.cpp exceptions/unwinding/termination-abnormal-end.cpp

The program ends, without returning from `main`, in each of these
cases:

- An exception leaves a non-throwing function ({rule}`EXS-4`), also
  when the throw is in a function that the non-throwing function calls.
- An exception leaves a destructor that is `noexcept` by
  {rule}`DTOR-4`.
- An exception leaves a destructor that runs during the unwinding of
  another exception.
- An exception leaves `main`.
- An exception has no matching handler in any enclosing try block.
- A rethrow finds no {term}`handled exception` ({rule}`THR-7`).
- The copy into a by-value handler parameter throws ({rule}`TRY-18`).
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
:kind: withdrawn

This rule merged into {rule}`UNW-1`.
:::

## Implementation Requirements

:::{rule} UNW-10
:kind: impl-defined
:tests: exceptions/unwinding/termination-abnormal-end.cpp exceptions/unwinding/termination-no-handler.cpp

How the program ends under UNW-5 is implementation-defined. On the
matrix targets, Clang and GCC write a message to the error output,
destroy no object with static storage duration, and stop the program
with the signal `SIGABRT`.
:::

:::{rule} UNW-7
:kind: impl-defined
:tests: exceptions/unwinding/no-handler-unwinding-unobservable.cpp exceptions/unwinding/noexcept-boundary-unwinding-unobservable.cpp

If an exception has no matching handler, it is implementation-defined
whether the automatic objects between the throw and `main` are
destroyed before the program ends. If an exception reaches the boundary
of a non-throwing function (UNW-5), it is implementation-defined which
automatic objects between the throw and that boundary are destroyed
before the program ends.
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

A call to a function that can throw does not need a warning.
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

A guard runs a cleanup when its block ends, by return or by an
exception, unless the code dismisses it. Its destructor is `noexcept`
({rule}`DTOR-4`), so a cleanup that throws ends the program (UNW-5). A
guard in a loop body runs once for each pass:

```cpp
template <class F> struct Guard {
  F cleanup;
  bool active = true;
  Guard(F f) : cleanup(f) {}
  Guard(const Guard&) = delete;
  ~Guard() { if (active) cleanup(); }
  void dismiss() { active = false; }
};
template <class F> Guard<F> on_exit(F f) { return Guard<F>(f); }  // a prvalue:
                                                                  // no copy
void transfer(Account& from, Account& to, int amount) {
  from.take(amount);
  auto undo = on_exit([&] { from.give(amount); });
  to.give(amount);             // if this throws, undo gives the money back
  undo.dismiss();              // success: keep the change
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 38
of 39 tests. EDG runs no programs, so its results say only that the
programs are valid.

- UNW-5 and UNW-7: the harness sees only that the program ended with a
  nonzero status or a signal. The harness cannot see why the program
  ended or what the implementation did before ending. The test for UNW-7
  shows the program ends; the choice that the test documents is not
  observable through the harness.
- UNW-7: when an exception reaches the boundary of a `noexcept`
  function, Clang and GCC do not destroy the automatic objects of the
  `noexcept` function itself. Both destroy the objects of a function
  that it calls, between the throw and the boundary. If the `noexcept`
  function has a try block around the call whose handlers do not match,
  GCC destroys none of the objects and Clang still destroys the objects
  of the called function. With no handler at all, neither compiler
  destroys an automatic object. The output in these probes was flushed
  before each destructor returned.
- UNW-3: the global deallocation test replaces both the sized and the
  unsized `operator delete`. Clang 18 calls the unsized form after a
  failed `new`, and the other compilers call the sized form.
- UNW-8: Clang and GCC warn under `-Wall`. Clang warns when no handler
  matches the static type of the operand, and GCC does not. GCC warns
  about a rethrow in a handler whose try block holds no direct `throw`,
  and Clang does not ({question}`Q-0179`).
- UNW-10: Clang and GCC print "terminate called after throwing an
  instance of" and the type of the exception, then abort. The test
  gives a static object a destructor that would end the program with
  status 0, and the program still stops with `SIGABRT`.
- UNW-1: Clang and GCC destroy the temporaries of the full expression,
  then the parameter object, then the block's objects, before the
  handler runs. The test also accepts a parameter that ends at the
  return of its function ({rule}`VC-10`).

Inputs: clang-sema-declcxx, clang-sema-expr, clang-diag-sema,
edg-decls.

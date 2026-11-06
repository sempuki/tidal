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
:rules: UNW-2 UNW-3 UNW-11

A constructor that throws has built some parts and not others, and only
the parts that exist can be ended ({principle}`OBJ-P1`).

| What throws | What happens |
|---|---|
| A constructor | The bases and members that it completed are destroyed, in reverse. The object's own destructor never runs, because the object never existed. |
| Aggregate initialization, or the captures of a lambda | The members that were initialized so far are destroyed, in reverse. A temporary that a reference member keeps alive ends with them. |
| The construction of an array | The same holds for the elements built so far. |
| A destructor | The members and bases that the destructor has not reached are still destroyed. |
| One placement `new` in a loop that builds elements one at a time | Only that element's parts are destroyed. The elements that earlier iterations built stay, and the program's own handler must destroy them. |
| A delegating constructor whose target completed | The object is complete, so its destructor does run. |
| The constructor in a `new` expression | The object never existed, but its storage does. The storage goes back through the deallocation function that lookup finds for the type, with the same placement arguments, and only if such a function exists. |

A placement deallocation function that is also a usual deallocation
function would be called for storage that the placement allocation did
not get in the usual way, so such a pair is not valid.
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

## Constraints

:::{rule} UNW-11
:kind: reject
:tests: exceptions/unwinding/placement-new-usual-delete-rejected.cpp exceptions/unwinding/placement-new-global-usual-delete-rejected.cpp

The deallocation function that a placement `new` expression selects
for the cleanup of UNW-3 must not be a usual deallocation function. For
example, a class declares a placement allocation function with one
extra size parameter and a sized deallocation function. The sized
function takes the same extra parameter, so a placement `new` expression
for the class is not valid. The global sized deallocation function
exists in every program ({principle}`NEW-P3`), so a global placement
allocation function with one extra size parameter makes a placement
`new` expression with that form not valid in the same way.
:::

## Runtime Behavior

:::{rule} UNW-1
:kind: required
:tests: exceptions/unwinding/block-objects-end-before-handler.cpp exceptions/unwinding/every-block-between-throw-and-handler.cpp exceptions/unwinding/nested-blocks-ended-in-order.cpp exceptions/unwinding/objects-after-throw-not-created.cpp exceptions/unwinding/handler-block-objects-unaffected.cpp exceptions/unwinding/full-expression-temporaries-unwound.cpp exceptions/unwinding/return-local-destructor-throws-result-destroyed.cpp exceptions/unwinding/return-local-destructor-throws-caught-inside.cpp exceptions/unwinding/return-local-destructor-throws-result-order.cpp

When an exception leaves a {term}`full expression`, these objects
end, in this order, before the handler runs:

1. the temporaries and parameter objects of that full expression that
   still exist ({rule}`VC-10`), last created first,
2. the automatic objects of each block from the throw to the handler,
   innermost block first, each block in the order of {rule}`OBJ-3`.

No later part of the full expression runs, and objects declared after
the throw are not created. Objects of the handler's own block and of
blocks that enclose the try block are not affected.

A return statement can build the result of the call and then throw
from the destructor of a local object or a temporary as it leaves the
function. The result is then also destroyed, after the objects of the
blocks that the exception leaves, and before the handler runs
({question}`Q-0365`). If a handler in the same function catches the
exception, a later return statement builds a new result.
:::

:::{rule} UNW-2
:kind: required
:tests: exceptions/unwinding/constructor-throws-parts-destroyed.cpp exceptions/unwinding/member-initializer-throws-earlier-parts-destroyed.cpp exceptions/unwinding/base-built-before-member-throws.cpp exceptions/unwinding/array-element-throws-built-destroyed.cpp exceptions/unwinding/delegating-target-completed-destructor-runs.cpp exceptions/unwinding/delegating-target-throws-no-destructor.cpp exceptions/unwinding/placement-loop-no-automatic-cleanup.cpp exceptions/unwinding/aggregate-init-throws-members-destroyed.cpp exceptions/unwinding/lambda-capture-throws-captures-destroyed.cpp exceptions/unwinding/reference-member-temporary-ended.cpp exceptions/unwinding/destructor-throws-members-still-destroyed.cpp

These rules apply:

- If a constructor throws, from its body or from a member initializer,
  the bases and members that it completed are destroyed in reverse
  order, and the destructor of the object itself does not run. The
  constructor can be written, defaulted, or inherited.
- If the initialization of an aggregate throws, the members that were
  initialized before the throw are destroyed in reverse order. The
  same holds for the captures of a lambda's closure object, in the
  order in which they were initialized. That order is unspecified, and
  every compiler of the matrix uses the order of the capture list
  ({rule}`LAM-28`).
- If a reference member of the object keeps a temporary alive, the
  temporary ends when the member would be destroyed in that order.
- If an element of an array throws during construction, the elements
  built before that element are destroyed in reverse order.
- If a destructor throws, the members and bases that it has not yet
  destroyed are still destroyed in reverse order, before a handler
  runs.
- If the target of a delegating constructor completed, the object is
  complete, and its destructor runs when the delegating constructor
  throws.

The second rule covers an array that one declaration or one `new`
expression builds. Objects that a program builds one at a time, for
example with placement `new` in a loop, get no such cleanup.
:::

:::{rule} UNW-3
:kind: required
:tests: exceptions/unwinding/new-constructor-throws-storage-released.cpp exceptions/unwinding/new-array-constructor-throws-storage-released.cpp exceptions/unwinding/placement-new-throws-placement-delete.cpp exceptions/unwinding/placement-new-no-matching-delete-leaks.cpp exceptions/unwinding/new-initializer-throws-global-delete.cpp exceptions/unwinding/class-new-only-global-delete.cpp exceptions/unwinding/new-single-object-sized-delete.cpp exceptions/unwinding/new-array-destructor-sized-delete.cpp exceptions/unwinding/new-extended-alignment-aligned-delete.cpp

If the initialization in a `new` expression throws, the storage is
released by a deallocation function. The `new` expression finds the
deallocation function by name, as {rule}`NEW-13` finds the allocation
function:

- For a class type, the search looks in the class first, unless the
  expression starts with `::`. If the class declares no deallocation
  function, the search finds the global one. The choice does not depend
  on which allocation function ran.
- An array uses the array form.
- A placement form uses the one deallocation function whose extra
  parameters match the extra parameters of the allocation function. If
  no such function exists, the storage is not released.

For the global deallocation function of a type that is not a placement
form:

- A type whose alignment is more than the {term}`default new alignment`
  uses a form that takes the alignment ({rule}`NEW-28`).
- A single object of complete type whose alignment is not more than the
  default new alignment uses the sized form.
- An array whose element type has a destructor that is not trivial
  uses the sized form.
- In the other cases, the implementation calls the sized or the unsized
  form.
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
:tests: exceptions/unwinding/leaves-noexcept-function-terminates.cpp exceptions/unwinding/leaves-implicit-noexcept-destructor-terminates.cpp exceptions/unwinding/leaves-destructor-during-unwinding-terminates.cpp exceptions/unwinding/leaves-main-terminates.cpp exceptions/unwinding/no-handler-terminates.cpp exceptions/unwinding/noexcept-boundary-is-the-function.cpp exceptions/unwinding/leaves-destructor-member-throwing-caught.cpp exceptions/unwinding/termination-abnormal-end.cpp exceptions/unwinding/static-destructor-throws-terminates.cpp exceptions/unwinding/destructor-throws-during-unwinding-stops.cpp exceptions/handlers/main-try-block-misses-static-init.cpp exceptions/handlers/main-try-block-misses-static-destructor.cpp

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
- The initializer of a static or `thread_local` variable declared
  outside any block throws.
- The destructor of a static or `thread_local` object throws.

The cases have one cause in common. An exception leaves a function that
the exception machinery or the program's start and end call directly,
and no handler of the program can be active there.
:::

:::{rule} UNW-6
:kind: required
:tests: exceptions/unwinding/noexcept-false-destructor-can-throw.cpp exceptions/unwinding/noexcept-function-handles-inside.cpp exceptions/unwinding/destructor-throws-members-still-destroyed.cpp

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
:tests: exceptions/unwinding/no-handler-unwinding-unobservable.cpp exceptions/unwinding/noexcept-boundary-unwinding-unobservable.cpp exceptions/unwinding/no-handler-objects-kept.cpp exceptions/unwinding/noexcept-boundary-callee-objects-destroyed.cpp exceptions/unwinding/noexcept-boundary-own-objects-kept.cpp exceptions/unwinding/noexcept-boundary-unmatched-try.cpp exceptions/unwinding/destructor-throws-during-unwinding-stops.cpp

If an exception has no matching handler, it is implementation-defined
whether the automatic objects between the throw and `main` are
destroyed before the program ends. If an exception reaches the boundary
of a non-throwing function (UNW-5), it is implementation-defined which
automatic objects between the throw and that boundary are destroyed
before the program ends. Clang and GCC choose in this way on the matrix
targets:

| Case | Clang | GCC |
|---|---|---|
| no matching handler | no object is destroyed | no object is destroyed |
| the boundary of a non-throwing function, objects of the functions that it calls | destroyed | destroyed |
| the same boundary, objects of the non-throwing function | not destroyed | not destroyed |
| the same boundary, if a try block of the non-throwing function holds the call and no handler of the try block matches | only the objects of the called functions are destroyed | no object is destroyed |

In the other cases of UNW-5, the program ends where the case arises,
and no further object is destroyed. For example, a destructor that
throws during unwinding ends the program, and the objects that
unwinding has not yet reached are not destroyed.
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

% tidal-example: fragment
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

// UNW-8 recommends a warning for these throws in a noexcept function:
void a() noexcept { try { throw 1; } catch (int) { throw; } }  // warn: the
                               // rethrow is outside the try block
void b() noexcept { try { throw 1; } catch (long) {} }   // warn: no handler
                               // matches int
void c() noexcept { try { g(); } catch (...) { throw; } } // warn
void d() noexcept { try { throw 1; } catch (int) {} }    // no warning
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

% tidal-example: fragment
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 51
of 60 tests. EDG runs no programs, so its results say only that the
programs are valid.

### Divergences

- {question}`Q-0365` (UNW-1; settled on UNW-P1): whether the result of
  a return statement is destroyed when a local destructor then throws.
  It is, and it ends after the local objects built before it, as GCC
  does. Clang 18, Clang 22, and the host Clang never destroy the
  result.
- {question}`Q-0179` (UNW-8; open): which throws in a non-throwing
  function get a warning. UNW-8 recommends one for each throw that no
  enclosing handler catches by static type, and a rethrow needs
  `catch (...)`. GCC gives no warning for an unmatched handler, Clang
  none for a rethrow from `catch (...)`, and EDG none at all.

### Split tests

EDG runs no programs, so its cell for a run test gives the result of
the compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `new-array-destructor-sized-delete.cpp` | UNW-3 | run | Clang 22: run; Clang 18: run-fail | run | accept | |
| `new-single-object-sized-delete.cpp` | UNW-3, NEW-22 | run | Clang 22: run; Clang 18: run-fail | run | accept | |
| `noexcept-boundary-unmatched-try.cpp` | UNW-7 | run | run | run-fail | accept | |
| `placement-new-global-usual-delete-rejected.cpp` | UNW-11 | reject | Clang 22: reject; Clang 18: accept | reject | reject | |
| `reference-member-temporary-ended.cpp` | UNW-2 | run | run | GCC 16: run; GCC 14: run-fail | accept | |
| `return-local-destructor-throws-caught-inside.cpp` | UNW-1 | run | run-fail | run | accept | {question}`Q-0365` |
| `return-local-destructor-throws-result-destroyed.cpp` | UNW-1 | run | run-fail | run | accept | {question}`Q-0365` |
| `return-local-destructor-throws-result-order.cpp` | UNW-1 | run | run-fail | run | accept | {question}`Q-0365` |
| `throw-unmatched-in-noexcept-warns.cpp` | UNW-8 | reject | reject | accept | skips | {question}`Q-0179` |

### Notes

- UNW-8: Clang and GCC give these warnings by default, with no warning
  flag. `-Wall` neither adds nor removes them. The tests use `-Wall
  -Werror`, so a warning rejects the program, and EDG skips the tests.
  Clang's warning says that the function "can still throw", and GCC's
  says that the `throw` "will always call terminate". Both warn about
  a rethrow inside a handler of a try block in the function
  (`rethrow-in-noexcept-warns.cpp`).
- UNW-5, UNW-7: the harness sees only that the program ended with a
  nonzero status or a signal, and cannot see why. The tests ending in
  `-unobservable` show only that the program ends. The other UNW-7
  tests make the choice observable. The destructor of the object in
  question ends the program with status 0, so the run succeeds only if
  the object is destroyed.
- UNW-7: when an exception reaches the boundary of a `noexcept`
  function, Clang and GCC do not destroy the automatic objects of the
  `noexcept` function itself. Both destroy the objects of a function
  that it calls, between the throw and the boundary. If the `noexcept`
  function has a try block around the call whose handlers do not match,
  GCC destroys none of the objects and Clang still destroys the objects
  of the called function. With no handler at all, neither compiler
  destroys an automatic object. GCC 16 at `-O2` destroys no object of
  a called function either. The table of UNW-7 gives the choices at
  `-O0`. EDG builds no program, so its choice is not observed. The
  base expectation of each test follows Clang's column of the UNW-7
  table, and the tests record GCC's choice as an override.
- UNW-7, UNW-10 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. In the
  writer's probes, the run tests of the two rules give the same results
  with `-m32` in the GCC 14, Clang 18, and Clang 22 containers.
- UNW-3: the global deallocation test replaces both the sized and the
  unsized `operator delete`. After a failed `new` of a single object,
  and of an array whose element type has a destructor, Clang 18 calls
  the unsized form and the other compilers call the sized form. Newer
  Clang calls the sized form by default. For an array whose element
  type has a trivial destructor, Clang 22 calls the sized form and GCC
  16 the unsized form.
- UNW-3: for a type with extended alignment, Clang and GCC call the
  form that takes the alignment and no size, for a single object and
  for an array. The source-reader gap review of the exceptions chapter
  (`reports/review/2026-11-03-gap-review-exceptions.md`) reads the
  C++23 text as asking for the sized form for a single object of
  complete type, also with extended alignment. UNW-3 allows both forms
  there, so the compilers' choice and the review's reading both fit
  (`documentation/decisions/pending.md`, row 4).
- UNW-3, UNW-11: Clang and GCC release the storage of a class that
  declares only an allocation function through the global deallocation
  function. Both reject a placement `new` expression whose cleanup
  selects a sized deallocation function of the class.
- UNW-2: Clang and GCC destroy the initialized members of an aggregate,
  the initialized captures of a closure object, and the temporary that
  a reference member keeps alive, in reverse order. GCC 14 destroys the
  members but never destroys the temporary of a reference member
  (`reference-member-temporary-ended.cpp`). When a destructor throws,
  Clang and GCC still destroy the members.
- UNW-5, UNW-7: Clang and GCC end the program when the constructor or
  the destructor of an object at namespace scope throws. When a
  destructor throws during unwinding, neither destroys the objects that
  unwinding has not reached.
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

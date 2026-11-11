# P3: Port suite, C# persona

Date: 2026-11-11

Commit read: 18bb9b3a1d2e7039c6680e575a4c29b64738ff4e

Role: clean-room reader (decision 0062, item E). The persona is an experienced C# programmer who has never written C++. The reader read only files under `specification/` and `reports/clean-room/P3/`.

The reader returned this report as a message, because the write of `report.md` was refused. The integrator saved it here with the reader's programs and the C# originals, which a clean-room writer wrote for the suite.

## Pages read

- `specification/index.md`
- `specification/glossary.md` (searched for terms only)
- `specification/foundations/translation.md`
- `specification/foundations/language-support.md` (first 260 lines)
- `specification/foundations/objects.md` (principles, OBJ-9)
- `specification/foundations/values.md` (VC-P4, VC-4)
- `specification/foundations/odr.md` (ODR-3)
- `specification/foundations/keywords.md` (KW-1, by search)
- `specification/expressions/lambdas.md` (principles, LAM-1 to LAM-6)
- `specification/expressions/new-delete.md` (principles)
- `specification/expressions/primary.md` (PRIM-9)
- `specification/classes/virtual.md` (principles, syntax, VIRT-2)
- `specification/classes/constructors.md` (principles, DTOR-1 to DTOR-4)
- `specification/classes/copy-move.md` (principles)
- `specification/classes/members.md` (CLS-P2)
- `specification/declarations/inline.md` (INL-4, INL-8)
- `specification/declarations/auto.md` (principles)
- `specification/statements/range-for.md` (principles, RFOR-2)
- `specification/statements/jumps.md` (JMP-P2, JMP-4)
- `specification/templates/declarations.md` (TPL-P1, TPL-P5)
- `specification/templates/instantiation.md` (INST-P1, INST-1 to INST-5)

## Rules noted before writing

| Construct | C# form | Rule and principle IDs used |
|---|---|---|
| Entry point, exit status | `static int Main` returning the failure count | TR-P3, TR-2, TR-3 |
| No headers | none | LSUP-P1, LSUP-4. No language-support type was needed in the end. |
| Lazy stages | `IEnumerable<T>` with `yield return` | KW-1 (coroutines out of scope), RFOR-P1, RFOR-2 |
| Stage chain lifetime | GC keeps each stage alive | OBJ-P2, OBJ-9, OBJ-P4 |
| Generic stages and functions | generic methods | TPL-P1, TPL-P5, DED-P1, DED-P2, INST-P1, INST-5 |
| Lambdas | `x => ...`, `Func<T,U>` | LAM-P1, LAM-P2, LAM-P4, LAM-1, LAM-5, LAM-6 |
| Delegates and events | `Action<int>`, `event`, `+=`, `-=` | LAM-P4, VIRT-P1, VIRT-P3, VIRT-P4, NEW-P1, NEW-P2, CPY-P2, CPY-P3 |
| Interfaces | `interface IState` | VIRT-P1, VIRT-P2, VIRT-P4, VIRT-2 |
| Deleting a state through the base | GC | VIRT-P3, NEW-P2 |
| `using` block | `IDisposable` | OBJ-P2, JMP-P2, JMP-4, CTOR-P4, DTOR-1 |
| Static field | `static bool Closed` | CLS-P2, INL-4, INL-8 |
| String equality | `ev == "coin"` | PRIM-9 |
| Name a call's result type | type inference | ODR-3, INST-2, INST-7 |

## Summary

All three final programs build with no warning on g++ and clang++ in every configuration. Each exits 0, runs clean under the sanitizers, and runs to `result: 0` on tidal-fe. All three first tries also build and pass on both compilers. Two first tries stopped on tidal-fe.

The three biggest problems:

1. **Tidal has no form for a lazy generator** (P3-1). Coroutines are out of scope (KW-1), so `yield return` has no equivalent. A C# programmer has to invent a pull protocol (`next(T&)`), an iterator class, and an end marker from the range-for rules alone. The specification has no example of an iterator over a computed sequence.
2. **The front end stops on a common pattern of class templates** (P3-7, P3-8, P3-9). The pattern is a base class template that takes the derived class as its argument. tidal-fe rejected it twice (NAM-9, TYN-7) and stopped once (range-based for), and both compilers accept it. The rejects come from member definitions that tidal-fe makes while the derived class is still incomplete. INST-5 says that the class instantiation does not make these definitions.
3. **The delegate idiom is one sentence** (P3-2). LAM-P4 names "a common base with a virtual call operator". The rest came from five other pages: ownership through `new`, a virtual destructor, deleted copies, a list of handlers, and a snapshot during the call.

## Findings

### P3-1: no lazy-generator form, and no iterator example

- Pages: `foundations/keywords.md`, `statements/range-for.md`
- Rules: KW-1, RFOR-P1, RFOR-2
- Kind: missing, needs example

What happened: Pipeline.cs uses `yield return` for each stage. The reader searched for `co_yield` and found only KW-1, which keeps the keyword but puts coroutines out of scope. The range-for page gives the protocol: `begin`, `end`, prefix `++`, `*`, `!=`, and `begin` and `end` of different types. The page has no example of an iterator class over a sequence that is computed on demand, or of a sentinel end type. The reader wrote a `Cursor<S>` class and an `EndMark` type from RFOR-2. The first try used a base class template that gives `begin` and `end` to every stage. That idea (a base that takes the derived class as its template argument) is outside knowledge.

Suggestion: state on the range-for page, or on an index page for readers from other languages, that a lazy sequence is a class with its own state and a `next` step, because coroutines are out of scope. Add a short informative example with an iterator over a computed range and a separate end type.

### P3-2: the delegate form needs an example

- Pages: `expressions/lambdas.md`, `classes/virtual.md`, `expressions/new-delete.md`, `classes/copy-move.md`
- Rules: LAM-P4, LAM-2, VIRT-P1, VIRT-P3, NEW-P1, NEW-P2, CPY-P3
- Kind: needs example

What happened: LAM-P4 says that a program without the library keeps closures of different types "through a common base with a virtual call operator". That sentence was enough to start. The reader then made four decisions:

- Each closure has one owner.
- The holder deletes through the base, so the base has a virtual destructor (VIRT-P3).
- The copy operations are deleted, so that two holders never delete one closure (NEW-P2).
- The handler list is copied before the call, so that a handler can remove itself. This is C# semantics, not a C++ rule.

Each step was findable, but each was on a different page.

Suggestion: give LAM-P4 a short informative example. The example would have an abstract callable class template with a virtual call operator and a virtual destructor, and a derived template that holds the closure.

### P3-3: naming the result type of a closure call

- Pages: `foundations/odr.md`, `templates/instantiation.md`, `expressions/decltype.md` (not read)
- Rules: ODR-3, INST-2, INST-7
- Kind: outside knowledge, needs example

What happened: The Select stage needs the type that its function returns, as C# infers `U` in `Select<T, U>`. The reader declared a function template `fake<T>()` with no definition and used it only in `decltype(fake<F&>()(fake<T&>()))`. ODR-3 (only an evaluated expression uses a function) and INST-7 make the form valid. The idea came from outside knowledge of the library helper that does the same job.

Suggestion: add an informative example to the decltype page. It would name the result of a call on a value of some type, using a function that is declared only, and cite ODR-3.

### P3-4: string equality is a trap that only a search finds

- Pages: `expressions/primary.md`, `expressions/comparison.md`
- Rules: PRIM-9
- Kind: hard to find

What happened: Machine.cs compares strings with `==`. A C# reader would write `ev == "coin"` with `const char*` operands. That compares addresses, and PRIM-9 makes the result unspecified for two literals with the same characters. The reader found PRIM-9 only by searching for "string literal". The comparison page does not mention string literals. The reader wrote a `same` function that compares character by character.

Suggestion: add a note to the equality rules of the comparison page. The note would say that `==` on two `const char*` compares addresses, and link to PRIM-9.

### P3-5: a stage that holds its source by reference dangles

- Pages: `foundations/objects.md`
- Rules: OBJ-9, OBJ-P2, OBJ-P4
- Kind: needs example

What happened: In C#, each stage keeps its source alive. In C++, `auto s = Select(Where(Naturals(), ...), ...)` makes temporaries that end with the full expression (OBJ-9). The reader made each stage hold its source by value to avoid the problem. The probe `probes/dangling-stage.cpp` holds the source by reference. tidal-fe reports `undefined [STG-22, OBJ-2]` for it. Both compilers with `-fsanitize=address` report stack-use-after-scope. So the rules and the tools are right. The rule text gives no example of a chained call that leaves a member reference to a temporary.

Suggestion: add an informative example under OBJ-9. It would show a class that stores a reference to a temporary argument, which dangles after the declaration.

### P3-6: INST-1 and INST-5 do not say when a member's return type is made

- Pages: `templates/instantiation.md`
- Rules: INST-1, INST-5, INST-P1
- Kind: unclear

What happened: INST-1 says that "the definition of a function whose parameter or return type is the class" instantiates the class. INST-5 says that the instantiation of a class template does not make member function bodies. Consider a member function defined in a class template whose return type is a specialization of another class template. The two rules do not say whether that return type is instantiated with the enclosing class or only when the member's definition is made. Both compilers wait. tidal-fe does not wait (P3-7).

Suggestion: add to INST-5 that the parameter and return types of a member function definition are needed only when the definition is made.

### P3-7: tidal-fe rejects with NAM-9 a member definition that is never used

- Pages: `templates/instantiation.md`
- Rules named by the stop: NAM-9. Related: INST-5.
- Kind: front end
- Reduction: `reductions/crtp-return-type.cpp`

What happened: `pipeline_first_try.cpp` stopped with `rejected [NAM-9] at line 18: no type named 'value_type'`. The reduction has a base `Base<D>` with an in-class `Cursor<D> begin()` body and a `struct Derived : Base<Derived>`. Nothing calls `begin`. tidal-fe makes `Cursor<Derived>` while `Derived` is incomplete. Both compilers accept and return 0. tidal-fe accepts a declaration of `begin()` with no body.

Suggestion: make member definitions of a class template, including their return types, only when a use needs them (INST-5).

### P3-8: tidal-fe rejects with TYN-7 a constructor body during the base's instantiation

- Pages: `templates/instantiation.md`, `classes/constructors.md`
- Rules named by the stop: TYN-7. Related: INST-5.
- Kind: front end
- Reduction: `reductions/crtp-constructor-body.cpp`

What happened: After a workaround for P3-7, pipeline.cpp stopped with `rejected [TYN-7] at line 22: member access into an incomplete class`. In the reduction, `Base<D>::begin()` returns `Cursor<D>(static_cast<D*>(this))`, and the constructor of `Cursor<D>` calls a member of `D`. tidal-fe checks the constructor while `D` is incomplete. tidal-fe accepts a plain member function with the same body. The stop occurs whether the call is in the member initializer or in the constructor body. Both compilers accept and return 0.

Suggestion: the same as P3-7.

### P3-9: tidal-fe does not run a range-based for whose begin and end come from such a base

- Pages: `statements/range-for.md`
- Rules: RFOR-2. The stop names no rule.
- Kind: front end
- Reduction: `reductions/range-for-crtp-base.cpp`

What happened: With the Fold loop written as `for (auto x : src)`, tidal-fe stopped with `unsupported at line 111: range-based for`. The reduction is a class `R : Base<R>` whose base gives `begin` and `end`. With `Base<int>` in place of `Base<R>`, tidal-fe runs the loop. tidal-fe also runs a range-for in each of these cases:

- over an array,
- over a class with its own `begin` and `end`,
- over a sentinel end type,
- inside a function template,
- over inherited members of a non-template base.

Suggestion: support member `begin` and `end` found in a base that is a specialization over the derived class.

The final pipeline.cpp avoids P3-7 to P3-9 with free `begin` and `end` function templates, which RFOR-2 finds through argument-dependent lookup.

### P3-10: tidal-fe does not run an operator= whose parameter is a class

- Pages: `classes/operators.md` (not read), `classes/copy-move.md`
- Rules: the stop names no rule.
- Kind: front end
- Reduction: `reductions/operator-assign-class-argument.cpp`

What happened: `events_first_try.cpp` stopped with `unsupported at line 88: class assignment`. The line is `onceOnly = [&](int) {...};`, which calls a template `operator=` of `Action<int>`. The reduction is a non-template `A& operator=(B)` with a class `B`. tidal-fe runs an `operator=` with an `int` parameter, and it also runs a call written as `a.operator=(3)`. Both compilers accept the reduction and return 0. The final events.cpp uses a named member `Set` in place of `operator=`.

Suggestion: run a user-declared `operator=` like any other member function call, whatever the class of its argument, and name the rule in the message.

### P3-11: copy-initialization from a closure with deleted copies

- Pages: `foundations/values.md`
- Rules: VC-4, CTOR-P5, CPY-P3
- Kind: outside knowledge

What happened: events.cpp declares `Action<int> logIt = [&log](int v) {...};`. `Action` has deleted copy operations and a converting constructor template. VC-4 covers a prvalue of the same type. The reader knew from outside knowledge that the converting constructor initializes `logIt` directly with no copy. The reader found no rule that says so for an initializer of another type.

Suggestion: state in the initialization pages that copy-initialization from another type through a converting constructor makes no copy of the result, or link that case to VC-4.

### P3-12: "closure type" is not in the glossary

- Pages: `glossary.md`, `expressions/lambdas.md`
- Rules: LAM-2
- Kind: hard to find

What happened: The glossary uses "closure type" in three entries and has no entry for it. The opening of the lambdas page defines the term. The glossary also has no entry for iterator or range.

Suggestion: add glossary entries for "closure type" and "range" that link to the lambdas and range-for pages.

## Attempt log

### pipeline.cpp

1. First try (`pipeline_first_try.cpp`):
   - each stage is a class template that holds its source by value and has a `next(T&)` pull function,
   - a CRTP base `Enumerable<D>` gives `begin` and `end`, with a `Cursor<S>` iterator,
   - Fold uses `for (auto x : src)`.

   It built and passed on both compilers in every configuration. tidal-fe: `rejected [NAM-9]` (P3-7).
2. The element type became a separate parameter: `Cursor<S, T>` and `Enumerable<D, T>`. tidal-fe: `unsupported: range-based for` (P3-9).
3. The loop was written out by hand. tidal-fe: `rejected [TYN-7]` (P3-8).
4. Final: the base class was removed in favor of free `begin` and `end` function templates (RFOR-2), and the range-based for was restored. Passed everywhere.

### events.cpp

1. First try (`events_first_try.cpp`):
   - `Callable<Arg>` has a virtual call operator and a virtual destructor,
   - `CallableOf<Arg, F>` holds the closure,
   - `Action<Arg>` owns one `Callable` through `new`,
   - `Event<Arg>` has `+=` and `-=`, and takes a snapshot of the handler list in `Invoke`.

   It built and passed on both compilers in every configuration. tidal-fe: `unsupported: class assignment` (P3-10).
2. Final: the template `operator=` was replaced with a named member `Set`. Passed everywhere.

### machine.cpp

1. First try:
   - `IState` has pure virtual `On` and `Name` and a virtual destructor,
   - `Locked` and `Unlocked` use `override`,
   - `Machine` deletes the old state on each change and has `static inline bool Closed`,
   - a `Using` guard calls `Dispose` from its destructor,
   - a `same` function compares strings.

   It built and passed everywhere, also on tidal-fe. The first try is the final version.

## Results

Each cell gives the build result and the exit status. The configurations are:

- strict: `-std=c++23 -pedantic-errors -Wall -Wextra`
- pedantic: `-pedantic-errors` alone
- sanitizers: `-std=c++23 -pedantic-errors -fsanitize=undefined,address`

No build gave a warning, and no sanitizer run printed a report.

| File | g++ strict | clang++ strict | g++ pedantic | clang++ pedantic | g++ sanitizers | clang++ sanitizers | tidal-fe |
|---|---|---|---|---|---|---|---|
| pipeline_first_try.cpp | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | rejected [NAM-9] line 18 |
| pipeline.cpp | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | result: 0 |
| events_first_try.cpp | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | unsupported: class assignment, line 88 |
| events.cpp | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | result: 0 |
| machine_first_try.cpp | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | result: 0 |
| machine.cpp | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | ok, 0 | result: 0 |

Both compilers accept each reduction in `reductions/`, and each exits 0:

| Reduction | tidal-fe |
|---|---|
| crtp-return-type.cpp | rejected [NAM-9] line 4 |
| crtp-constructor-body.cpp | rejected [TYN-7] line 7 |
| range-for-crtp-base.cpp | unsupported: range-based for, line 14 |
| operator-assign-class-argument.cpp | unsupported: class assignment, line 6 |

## Host-only claims

These claims rest on the host g++ and clang++ alone. A writer should probe them on the matrix before acting.

- The four reductions are valid and return 0 (P3-7 to P3-10).
- A compiler does not check a member function definition in a class template until the member is used, even when its return type is a specialization over an incomplete class (P3-6, P3-7).
- The compilers accept `Action<int> x = closure;` when `Action` has deleted copy operations and a converting constructor template (P3-11).
- A function template that is declared only and used in `decltype` needs no definition at link time (P3-3).
- The probe `probes/dangling-stage.cpp` gives `stack-use-after-scope` under `-fsanitize=address` (P3-5).
- All six programs build with no warning under `-Wall -Wextra`.

## What worked well

- LAM-P4 named the delegate technique directly, so the reader did not reach for `std::function`.
- VIRT-P3 and VIRT-P4 gave the interface port in one reading: a pure virtual function, `override`, and a virtual destructor for `delete` through the base. The machine port built and ran on the first try.
- OBJ-P2 and JMP-P2 state that automatic objects end when control leaves the block by any route. That is the guarantee of the C# `using` block.
- PRIM-9 is a clear rule once found, and it kept the machine port from depending on literal addresses.
- INL-4 answered the static field question in one rule.
- tidal-fe found the dangling-stage trap and named a precise rule pair (STG-22, OBJ-2). Its stops were easy to reduce.

# X10: P11, exceptions that stay inside a constant evaluation

Date: 2026-11-03. Exercise kind: Propose. Role: clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md, CLAUDE.md, section 1 of `documentation/style-guide.md`, and section 4 of `documentation/exercises.md`.
- P11 in `reports/clean-room/roadmap/designer.md`. Sections 1 and 2, item 2, and the P11 rows of `reports/clean-room/roadmap-3/designer.md`.
- `reports/clean-room/X8/report.md`, as the model for this report's form. `reports/clean-room/X2/report.md`, so that this report does not repeat its findings.
- In full: `foundations/constant-evaluation.md`, `exceptions/throw.md`, `exceptions/handlers.md`, `exceptions/unwinding.md`, `foundations/language-support.md`, `declarations/exception-specifications.md`.
- The cited rules of other pages: TR-P1, TR-P4, TR-5, TR-10 (translation), UN-18 (unary), CEV-1, CX-1, CX-6, CINIT-1 and the `consteval` example (constexpr), IF-16, IF-18 (if), SA-3, VIRT-10, NEW-35, NEW-37, NEW-38, NEW-39 (new-delete), DED-24, CPT-26, DTOR-4, CPY-12, SINIT-1 to SINIT-7, OBJ-P2, and the glossary.
- The tests that CE-1, CE-8, CE-31, THR-1, THR-8, THR-10 and NEW-39 cite, and a scan of every test that contains both `catch` and `constexpr` or `consteval`.
- A search of `questions/` for questions on throw, handlers and constant evaluation.

Compiler runs: g++ 16.2.1 and clang++ 22.1.8 under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X10/probes`. The runs used `-std=c++23 -pedantic-errors` to see what programs do today. A second set used `-std=c++2c -pedantic-errors` to see whether a later version takes the space (roadmap-3, section 1.1). The report gives the results where they support a claim. The harness did not let the reader save this report as a file, so the integrator saves it.

Totals: 53 rule changes (3 new, 12 amended, 38 unchanged but must state their effect). No existing test changes its expected outcome. 12 findings, X10-1 to X10-12.

## 1. Design

### 1.1 The feature

Today CE-8 rejects a constant evaluation that evaluates a throw-expression. So a `constexpr` function that reports bad input with `throw` cannot serve a constant evaluation on that input, even when the function catches the exception itself. P11 lets an evaluation throw and catch. An evaluation is rejected only when an exception leaves it.

```cpp
constexpr int digit(char c) {
  if (c < '0' || c > '9') throw c;
  return c - '0';
}
constexpr int parse_or(const char* s, int fallback) {
  try {
    int v = 0;
    for (; *s; ++s) v = v * 10 + digit(*s);
    return v;
  } catch (char) {
    return fallback;
  }
}
static_assert(parse_or("42", 0) == 42);
static_assert(parse_or("4x", -1) == -1);   // P11: valid. Today: rejected
static_assert(digit('x') == 0);            // still rejected: the exception leaves
```

No function changes its run-time behavior. A function now gives the same result in both modes (TR-5, CE-P1).

### 1.2 The boundary (amended CE-8)

The amended CE-8 says that an exception must not leave a constant evaluation. The rule covers each way an exception can start (1.6). A throw is valid when a handler catches the exception, and the try block of that handler was entered by the same evaluation.

The evaluation is the one that CE-30 and CE-P4 define. Each place in the CE-P4 table starts its own evaluation, also inside a function that another evaluation runs. An exception cannot pass from one evaluation to another. Three cases follow:
- A `constexpr` variable inside a `constexpr` function is its own evaluation. A `try` around the declaration does not catch an exception from its initializer. The program is rejected, also when no call ever reaches the enclosing function. Both compilers reject `try { constexpr int w = g(); } catch (int) {}` when `g` throws, in both modes.
- A call to a `consteval` function outside an immediate function context is its own evaluation (CEV-1). A `try` around the call at run time does not catch. With a `consteval` function `g` that throws, both compilers reject `int main() { try { return g(-1); } catch (int) { return 0; } }` in both modes.
- Inside an immediate function context, a call to a `consteval` function is part of the caller's evaluation. A `consteval` function can catch an exception that a `consteval` function it calls throws.

### 1.3 The exception object in an evaluation (new rule X10:new-2)

THR-13 says the implementation chooses the storage of the exception object, without an allocation function. So CE-7 does not apply to the exception object. X10:new-2 says what applies instead:
- The exception object is an object that the evaluation creates (CE-P2). The evaluation can read it (CE-3) and change it (CE-5). A handler by reference that changes the object and rethrows works as at run time (THR-6).
- Its initialization, any copy or move, and its destructor are calls under CE-2. An evaluation cannot throw an object of a class whose destructor is not `constexpr`.
- The object ends by THR-8. Because no exception leaves the evaluation (1.2), the object always ends inside the evaluation.
- A pointer or reference to the exception object must not be the result of the evaluation (amended CE-14). In `-std=c++2c` mode both compilers reject a result that points to the exception object. GCC's message names the exception object.

The proposal adds no literal-type rule for the exception object. The exception object follows the rule for a temporary, and Tidal has no clear rule for the type of a temporary (X10-11). GCC in `-std=c++2c` mode accepts a thrown class with a `volatile` member.

Round 1's D25 asked where the storage of the exception object comes from. THR-13 now answers it for run time. X10:new-2 answers it for an evaluation.

### 1.4 Unwinding and handlers in an evaluation (new rule X10:new-1)

X10:new-1 says that in an evaluation, a throw, the search for a handler, unwinding and rethrow follow the run-time rules:
- UNW-1, UNW-2, UNW-3 and UNW-9 run. Each destructor that unwinding calls is a call under CE-2.
- When a constructor in a `new` expression fails, UNW-3 releases the storage, and that storage counts as freed for CE-7. Unwinding does not free other storage, so CE-7 rejects an evaluation that leaves it. In the probe, an evaluation runs `new int(1)`, then a `throw` that it catches, and has no `delete`. Both compilers reject it.
- TRY-1 and TRY-2 select the handler. TRY-2 matches by type relations that Tidal already defines, as the roadmap noted. The evaluation needs no run-time type library.
- THR-6, THR-8, THR-12 and TRY-8 hold. A rethrow passes on the same object.
- Objects end in an evaluation in the same order as at run time (X10-6). GCC in `-std=c++2c` mode gives the same order in a constant evaluation and in a run. The order covers a handler parameter, the old exception object and the new exception object.

### 1.5 Termination inside an evaluation (amended CE-31)

Today CE-31 rejects an evaluation that reaches a point where the program terminates. CE-31 adds that a thrown exception comes before each such point, so CE-8 always rejects first. Under P11 a throw is no longer rejected by itself, so that argument fails. CE-31 now stands on its own. The points are:
- an exception that leaves a non-throwing function (THR-9, UNW-5), also when a handler outside that function would catch it,
- a rethrow with no exception being handled (THR-7),
- a copy into a by-value handler parameter that throws (TRY-18),
- an exception that leaves a destructor during unwinding (UNW-5).

GCC in `-std=c++2c` mode rejects each of the four and names termination in its message. Clang rejects them too, because Clang rejects every throw. No single rule in Tidal lists all four today (X10-9).

D27 from round 1 is closed. CE-31 now says what an evaluation does at a termination point.

### 1.6 Exceptions that the language throws (new rule X10:new-3; amended VIRT-10, NEW-39)

Two core rules throw an object without a throw-expression:
- a failed `dynamic_cast` to a reference throws `std::bad_cast` (VIRT-10),
- an erroneous bound in `new T[n]` throws `std::bad_array_new_length` (NEW-35).

Today CE-8 rejects the first and NEW-39 rejects the second. X10:new-3 says that in an evaluation, each of these throws its object as `throw C()` would. CE-2 applies to the constructor and the destructor of `C`. The amended CE-8 then covers both, so a handler in the evaluation can catch them.

LSUP-8 gives a program that includes no header a shape in which the library defines the destructor. That destructor is not `constexpr`, so CE-2 rejects the throw in such a program. GCC in `-std=c++2c` mode agrees in both cases. With the library's headers it catches the failed cast and the erroneous bound. With the LSUP-8 shape it rejects the call of a constructor that is not `constexpr`.

`std::bad_alloc` (NEW-38) does not arise. An evaluation that runs out of storage exceeds an implementation limit (CE-10).

### 1.7 What does not change

- Run-time behavior, including THR-10's run-time throw.
- `noexcept(e)` stays `false` for an operand with a throw-expression (UN-18). The `noexcept` specifier and function types are untouched.
- `if consteval` (IF-18) is still available. A function no longer needs it to have an error path that works in both modes.
- The implementation limits of CE-10. Unwinding counts as steps.

### 1.8 Alternatives considered

- **Keep CE-8 as it is.** Then every function that serves both modes needs two error paths. Rejected, because the feature exists to remove that split.
- **Let an exception that leaves the evaluation become the diagnostic.** The program is rejected either way. A recommended practice to name the exception's type in the diagnostic would help. The proposal leaves the diagnostic to implementations.
- **Let an exception pass from a nested evaluation to a handler in the enclosing function.** A nested evaluation (CE-30) runs at translation time whether or not any call reaches it. A handler in the enclosing function has no meaning there. Rejected.
- **Let an exception leave a `consteval` call outside an immediate function context and throw at run time.** After translation the call has no run-time form. Rejected.
- **Keep the exceptions of 1.6 rejected.** This keeps one special case in CE-8 and one in NEW-39. It also treats `throw std::bad_cast()` and a failed cast differently, although both throw the same object. Rejected.
- **Require a literal type for the exception object.** CE-P1 says the objects of an evaluation need literal types. CE-20 states the requirement for variables only, and the compilers split on temporaries (X10-11). The proposal waits for that answer.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X10:new-1 | new | none | required | In an evaluation, a throw, the search for a handler, unwinding and rethrow follow THR, TRY and UNW. Storage that UNW-3 releases counts as freed for CE-7 (1.4). | none |
| X10:new-2 | new | none | required | The exception object of an evaluation is an object of that evaluation. Its initialization, copies and destructor are calls under CE-2. It ends inside the evaluation (1.3). | none |
| X10:new-3 | new | none | required | In an evaluation, a failed `dynamic_cast` to a reference and an erroneous `new` bound throw their objects as a throw-expression would. CE-2 applies to the class (1.6). | none |
| CE-P1 | amended | principle | principle | The list of what an evaluation can do gains "throw and catch an exception". | none |
| CE-P2 | amended | principle | principle | The objects that an evaluation creates include the exception object. An exception must end in a handler inside the evaluation. | none |
| CE-P3 | amended | principle | principle | "An exception has no handler at translation time" becomes "an exception that leaves the evaluation ends it". The sentence on termination stands alone. | none |
| CE-1 | amended | required | required | "`try` blocks, if no exception is thrown" becomes "`try` blocks and handlers, and a throw that a handler in the same evaluation catches". | none |
| CE-8 | amended | reject | reject | An exception must not leave a constant evaluation. A handler outside the evaluation does not count, also one around a place of CE-P4 or around a `consteval` call (1.2). | none |
| CE-14 | amended | reject | reject | A result must not refer to the exception object of the evaluation either (1.3). | none |
| CE-31 | amended | reject | reject | Stands alone. Lists the four termination points of 1.5 and drops the claim that CE-8 always rejects first. | none |
| THR-P2 | amended | principle | principle | In an evaluation, the exception object belongs to the evaluation (X10:new-2). | none |
| THR-P5 | amended | principle | principle | "A `throw` has nowhere to go" becomes "a `throw` can reach a handler in the same evaluation, and an exception that leaves the evaluation makes it not constant". | none |
| THR-10 | amended | required | required | Points to CE-8 and X10:new-1 for constant evaluation. | none |
| VIRT-10 | amended | required | required | "CE-8 rejects a failed cast of a reference" becomes "the failed cast throws as X10:new-3 says". | none |
| NEW-39 | amended | reject | reject | The erroneous bound throws as X10:new-3 says. The evaluation is rejected when the exception leaves it or when CE-2 fails. | none |
| CE-P4 | unchanged, state effect | principle | principle | Each place in the table is also a boundary for exceptions. | none |
| CE-2 | unchanged, state effect | reject | reject | Applies to the exception object's functions and to every destructor that unwinding calls. | none |
| CE-3 | unchanged, state effect | reject | reject | The evaluation can read the exception object as one of its own objects. | none |
| CE-5 | unchanged, state effect | reject | reject | A handler can change the exception object. | none |
| CE-7 | unchanged, state effect | reject | reject | Unwinding does not free storage, except by UNW-3. CE-7 does not apply to the exception object (THR-13). | none |
| CE-9 | unchanged, state effect | reject | reject | A use of the exception object after it ends is undefined and rejected. | none |
| CE-10 | unchanged, state effect | impl-defined | impl-defined | Unwinding counts toward the step limit. | none |
| CE-30 | unchanged, state effect | reject | reject | The boundary of 1.2. | none |
| TR-5 | unchanged, state effect | required | required | A function that throws and catches gives the same result in both modes. | none |
| THR-1 | unchanged, state effect | required | required | Holds in an evaluation (X10-1 on its text). | none |
| THR-3 | unchanged, state effect | reject | reject | Holds in an evaluation. | none |
| THR-6 | unchanged, state effect | required | required | A rethrow in an evaluation passes on the same object. | none |
| THR-7 | unchanged, state effect | required | required | A rethrow with no exception is a termination point for CE-31. | none |
| THR-8 | unchanged, state effect | required | required | The exception object ends inside the evaluation. | none |
| THR-9 | unchanged, state effect | required | required | A termination point for CE-31, also when a handler outside the function would catch. | none |
| THR-12 | unchanged, state effect | required | required | Holds in an evaluation. | none |
| THR-13 | unchanged, state effect | required | required | The storage has no allocation function, so CE-7 does not count it. | none |
| TRY-1 | unchanged, state effect | required | required | Selects the handler in an evaluation. | none |
| TRY-2 | unchanged, state effect | required | required | Matches by type relations alone, so an evaluation needs no run-time type library. | none |
| TRY-8 | unchanged, state effect | required | required | Holds in an evaluation. | none |
| TRY-18 | unchanged, state effect | required | required | A termination point for CE-31. | none |
| UNW-1 | unchanged, state effect | required | required | Runs in an evaluation. | none |
| UNW-2 | unchanged, state effect | required | required | Runs in an evaluation. | none |
| UNW-3 | unchanged, state effect | required | required | The released storage counts as freed for CE-7. | none |
| UNW-5 | unchanged, state effect | required | required | Each case is a termination point for CE-31. | none |
| UNW-6 | unchanged, state effect | required | required | A non-throwing function can throw and catch inside itself in an evaluation. | none |
| UNW-7 | unchanged, state effect | impl-defined | impl-defined | Does not arise in an evaluation, because an exception with no handler leaves the evaluation. | none |
| UNW-9 | unchanged, state effect | required | required | Runs in an evaluation. | none |
| EXS-4 | unchanged, state effect | required | required | `constexpr` does not make a function non-throwing, so a `constexpr` function can let an exception reach its caller in an evaluation. | none |
| UN-18 | unchanged, state effect | required | required | `noexcept(e)` stays `false` for a throw-expression. | none |
| CEV-1 | unchanged, state effect | reject | reject | A call outside an immediate function context must catch its exceptions inside itself. Inside an immediate function context, an exception can pass between `consteval` calls. | none |
| IF-18 | unchanged, state effect | required | required | A function no longer needs `if consteval` for an error path that works in both modes. | none |
| LSUP-8 | unchanged, state effect | required | required | The shape with a library-defined destructor fails CE-2 in an evaluation (1.6). | none |
| DED-24 | unchanged, state effect | required | required | A template argument whose evaluation throws and catches is no longer a substitution failure (4.3). | none |
| CPT-26 | unchanged, state effect | required | required | A requirement whose evaluation throws and catches now holds (4.3). | none |
| SINIT-1 | unchanged, state effect | required | required | A variable whose initializer throws and catches becomes constant-initialized (4.3). | none |
| SINIT-4 | unchanged, state effect | unspecified | unspecified | For such a variable, an earlier initializer now always reads the value, never zero (4.3). | none |
| CINIT-1 | unchanged, state effect | reject | reject | A `constinit` variable can have an initializer that throws and catches. | none |

## 3. Interactions

**Constant evaluation (CE-P1 to CE-P4, CE-1, CE-2, CE-3, CE-5, CE-7, CE-8, CE-9, CE-14, CE-30, CE-31).** The feature changes CE-8 from a ban on any throw to a ban on an exception that leaves the evaluation. CE-30 gives the boundary, as round 3 pointed out. Until now no test depends on CE-31 alone, because every reject test that CE-31 cites also cites CE-8. Under P11, CE-31 carries the termination cases. Today CE-8 names only a throw-expression and a failed reference cast, but NEW-39 cites CE-8 for the erroneous bound (X10-10).

**Throw (THR-P2, THR-P5, THR-1, THR-3, THR-6 to THR-10, THR-12, THR-13).** The proposal must say where the exception object lives in an evaluation. THR-P2 says the object belongs to no scope, and OBJ-P2 has no row for it. THR-13 settles the run-time storage. Writing X10:new-2 showed four smaller gaps in how the throw page makes the object:
- the condition for a move (X10-1),
- a copy that throws (X10-2),
- the order in which objects end (X10-6),
- the names for the exception being handled (X10-7).

**Handlers (TRY-1, TRY-2, TRY-8, TRY-18).** An evaluation runs the matching rule of TRY-2. TRY-2 covers pointers and classes. Both compilers also match pointers to members, and they convert a pointer to a `noexcept` function to a plain function pointer. TRY-2 covers neither case (X10-4, X10-5). TRY-5 does not require the copy constructor that a by-value handler uses (X10-3). TRY-8 and THR-7 state one rule twice (X10-8).

**Unwinding (UNW-1 to UNW-3, UNW-5 to UNW-7, UNW-9).** Unwinding runs unchanged. UNW-5, THR-7 and TRY-18 each give some of the cases where the program terminates. The page's introduction lists only four of them (X10-9).

**Exception specifications (EXS-4) and unary (UN-18).** Not touched. A `noexcept` function inside an evaluation keeps its boundary. A `constexpr` function is not non-throwing by default, so it can pass an exception to its caller in an evaluation.

**Language support (LSUP-8) and the exceptions the language throws (VIRT-10, NEW-35, NEW-38, NEW-39).** X10:new-3 makes a language throw follow the throw-expression rules. An evaluation cannot throw an object of the LSUP-8 shape. For programs without a header, the outcome is the same as today.

**`constexpr` and `consteval` (CX-1, CINIT-1, CEV-1, IF-18).** More initializers become constant. The constexpr page has an example of a `consteval` constructor that checks its input with `throw`. That constructor still rejects bad input, because the exception leaves the call.

**Templates and static initialization (DED-24, CPT-26, SINIT-1, SINIT-4).** When an evaluation fails, these rules take a fallback. An evaluation that throws and catches no longer fails, so the fallback no longer applies. Section 4.3 gives the effect.

**A later version.** GCC 16 in `-std=c++2c` mode accepts each probe in this report that P11 makes valid. It rejects each probe that P11 keeps rejected. Clang 22 rejects every evaluation that throws, in both modes. In C++23 the space is free. GCC's next-version mode already uses the space with the same design.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- A constant evaluation that throws, and catches the exception in the same evaluation.
- A `constexpr` or `constinit` variable, a `static_assert`, a template argument, or a `consteval` call whose evaluation throws and catches.
- In such an evaluation, a failed `dynamic_cast` to a reference or an erroneous `new` bound that a handler catches, when the class of the thrown object has `constexpr` members.

### 4.2 Undefined becomes defined

None. P11 touches no `undefined` rule.

### 4.3 Valid programs that change meaning

Some rules turn a failed evaluation into a fallback. Each of these rules now takes the other branch for an evaluation that throws and catches. `documentation/exercises.md` lists X10 as a proposal that does not change existing behavior. The probes below show that it does.

- **Substitution (DED-24).** With `g<N>()` that throws and catches for negative `N`:
  ```cpp
  template <int N, class = S<g<N>()>> int pick(int) { return 1; }
  template <int N> int pick(long) { return 2; }
  ```
  Today `pick<-1>(0)` returns 2 on both compilers. It returns 1 under GCC in `-std=c++2c` mode.
- **Requires expressions (CPT-26).** For `N == -1`, `requires { S<g<N>()>{}; }` is `false` today on both compilers. It is `true` under GCC in `-std=c++2c` mode.
- **Static initialization (SINIT-1, SINIT-4).**
  ```cpp
  constexpr int f() { try { throw 1; } catch (int) { return 5; } return 0; }
  extern int a;
  int b = a;
  int a = f();
  ```
  Today `a` is dynamically initialized, and SINIT-4 lets `b` read 0 or 5. Both compilers give 0. Under P11, `a` is constant-initialized, so `b` is 5. GCC in `-std=c++2c` mode gives 5. P11 narrows an unspecified choice, and it changes the value that both compilers give today.

The run-time behavior of a function does not change.

### 4.4 Valid programs that become invalid

Substitution can remove a candidate today. Under P11 that candidate can become viable, and the call can then be ambiguous:

```cpp
template <int N, int = g<N>()> int pick() { return 1; }
template <int N> int pick() { return 2; }
int main() { return pick<-1>(); }
```

In C++23 mode both compilers accept this program, and it returns 2. GCC in `-std=c++2c` mode rejects the call as ambiguous. The same holds for any overload set that relied on a throw to remove a candidate, with or without concepts. No new `reject` rule makes a valid program invalid. CE-31 and the amended CE-8 reject only evaluations that CE-8 rejects today.

### 4.5 Evidence

All runs used g++ 16.2.1 and clang++ 22.1.8. Every probe in the table is rejected by both compilers in C++23 mode.

| Probe | GCC, `-std=c++2c` | Clang, `-std=c++2c` | P11 |
|---|---|---|---|
| throw caught in the same evaluation | accepted | rejected | valid |
| `constinit` with such an initializer | accepted | rejected | valid |
| throw from a non-throwing function, caught by its caller | rejected, "terminate" | rejected | rejected (CE-31) |
| nested `constexpr` variable inside `try` | rejected | rejected | rejected (CE-30) |
| `consteval` call inside `try` at run time | rejected | rejected | rejected (CEV-1) |
| `consteval` calls `consteval`, caller catches | accepted | rejected | valid |
| rethrow after change by reference | accepted | rejected | valid |
| rethrow with no exception | rejected | rejected | rejected (CE-31) |
| `new` before a caught throw, never freed | rejected, "not deallocated" | rejected | rejected (CE-7) |
| destructor runs in unwinding | accepted | rejected | valid |
| by-value handler copy throws | rejected, "terminate" | rejected | rejected (CE-31) |
| destructor throws during unwinding | rejected, "terminate" | rejected | rejected (CE-31) |
| result points to the exception object | rejected, names the exception object | rejected | rejected (CE-14) |
| failed reference cast, caught, library header | accepted | rejected | valid |
| erroneous `new` bound, caught, library header | accepted | rejected | valid |
| failed reference cast, caught, LSUP-8 shape | rejected, constructor not `constexpr` | rejected | rejected (CE-2) |

No existing test changes its outcome:
- In every reject test of CE-8, the exception leaves the evaluation.
- CE-31 alone still rejects the two reject tests that cite CE-31.
- The tests of NEW-39 and the failed-cast test have no handler.

The proposal needs a new test for each row of the table above.

## 5. Findings

**X10-1** (wrong; `exceptions/throw.md`; THR-P1, THR-1). THR-1 says that a local variable operand is moved. The compilers move only in some cases:

| Operand | Both compilers |
|---|---|
| a local variable declared outside the innermost try block around the `throw`: `S s; try { throw s; } catch (...)` | copy |
| a local variable declared inside that try block: `try { S s; throw s; }` | move |
| a parameter, with a try block in the function body around the `throw` | copy |
| a parameter, with no try block around the `throw`: `void f(S s) { throw s; }` | move |

The existing test `local-variable-moved.cpp` covers only the second row. THR-P1's reason ("the variable is about to end") also fails for the first row. Suggestion: state the condition in THR-1. The variable or parameter must not outlive the innermost try block that encloses the `throw`. Add run tests for the cases in the table.

**X10-2** (missing; `exceptions/throw.md`; THR-1, THR-3). No rule says what happens when the copy or move that makes the exception object throws. Both compilers propagate the new exception, and a handler for its type catches it. In the probe, `throw g` copies a static `g`, and the copy constructor throws `7`. With handlers `catch (int)` and `catch (S&)`, the program exits with 7 on both compilers. It does not terminate. TRY-18 covers only the copy into a handler parameter, which does terminate, so a reader can guess wrong. Suggestion: a sentence in THR-1 and a run test.

**X10-3** (missing; `exceptions/handlers.md`; TRY-5, TRY-2, THR-3). TRY-5 does not require that a by-value handler's copy constructor be usable and accessible. THR-3 states that requirement only for the `throw`. Both compilers reject `catch (S s)` when the copy constructor of `S` is deleted or private. They reject it also when the thrown operand is a prvalue. Suggestion: add an item to TRY-5, or a new reject rule, with the two reject tests.

**X10-4** (missing; `exceptions/handlers.md`; TRY-2). TRY-2 covers pointers to objects and to functions. It does not cover pointers to members. Both compilers do these four things:
- catch `nullptr` with a handler of type `int B::*`,
- catch `&D::n` with a handler of type `const int D::*`, which is a qualification conversion,
- do not catch `&B::m` with a handler of type `int D::*`, because no base-to-derived conversion of a member pointer applies,
- catch `&D::m`, whose type is `int B::*`, with a handler of type `int B::*`.

TRY-2 has the item "`T` is a pointer and `E` is the type of `nullptr`". Tidal does not say whether "pointer" includes a pointer to member, so the first case is unclear. Suggestion: add pointer-to-member items to TRY-2 and four run tests.

**X10-5** (missing; `exceptions/handlers.md`; TRY-2, EXS-2). Both compilers catch `&f`, where `f` is `noexcept`, with a handler of type `void (*)()`. TRY-2 lists no conversion that drops `noexcept` from a function pointer. The test `match-function-pointer-not-void.cpp` covers only the conversion to `void*`. Suggestion: add the function pointer conversion to the list in TRY-2, with a run test.

**X10-6** (unclear; `exceptions/throw.md`, `exceptions/unwinding.md`; THR-8, UNW-1). THR-8 says that a new exception thrown from a handler ends the old exception object. No rule gives the order of three events:
- the by-value handler parameter ends,
- the old exception object ends,
- the outer handler starts.

Both compilers end the handler parameter first, then the old exception object, and then run the outer handler. They do so at run time, and GCC does so in an evaluation. UNW-1 covers only automatic objects. The test `new-throw-in-handler-ends-old.cpp` checks a count of destructor calls and no order. P11 needs the order, because an evaluation can observe it through `constexpr` destructors. Suggestion: state the order in THR-8, and add a run test that records it.

**X10-7** (unclear; `exceptions/throw.md`, `exceptions/handlers.md`, glossary; THR-P4, THR-6, THR-7, TRY-P4, TRY-8). The pages use five phrases for one idea: "the exception that a handler is handling", "being handled", "the current exception", "the handled exception", and "the innermost active handler". None has a glossary entry. No rule says when a handler becomes active or stops being active. Consider a try block nested in a handler. After the inner handler ends, `throw;` rethrows the outer exception on both compilers. A reader must infer this from the word "innermost" in "innermost active handler". The glossary also has no entry for "exception object" or "handler". Suggestion: one defined term, such as "the handled exception", with a rule for when a handler starts and stops handling. Add glossary entries for "exception object" and "handler".

**X10-8** (hard to find; `exceptions/throw.md`, `exceptions/handlers.md`; THR-P4, THR-7, TRY-P4, TRY-8). THR-7 and the last sentence of TRY-8 state the same rule: a rethrow with no exception being handled ends the program. Each page has a test named `rethrow-outside-handler-terminates.cpp`. THR-6 and TRY-8 also both say that a rethrow passes on the same object with its changes. A proposal must amend both pages, as round 1 found for CE-8 and THR-10 (D26). Suggestion: keep each statement on one page, and let the other page cite it.

**X10-9** (missing; `exceptions/unwinding.md`, `foundations/constant-evaluation.md`; UNW-5, THR-7, TRY-18, CE-31). The introduction of the unwinding page lists four cases in which the program ends. UNW-5 lists five. THR-7 (a rethrow with no exception) and TRY-18 (a by-value handler copy that throws) add two more on other pages. CE-31 cites UNW-5 and THR-7 and omits TRY-18. Today CE-31 needs no list, because CE-8 rejects first. Under P11 the list decides which evaluations are rejected. Suggestion: one list of termination points in UNW-5 that includes THR-7 and TRY-18, cited by CE-31 and by the page introduction.

**X10-10** (wrong; `foundations/constant-evaluation.md`, `expressions/new-delete.md`; CE-8, NEW-39). NEW-39 says that the throw of an erroneous bound "is not valid there ({rule}`CE-8`)". CE-8 names a throw-expression and a failed `dynamic_cast` of a reference. It does not name the erroneous bound. CE-8's test list does not include the two NEW-39 tests either. Suggestion: add the erroneous bound to CE-8 with a citation of NEW-39, or state CE-8 for an exception from any cause.

**X10-11** (unclear; `foundations/constant-evaluation.md`; CE-P1, CE-20, CE-23). CE-P1 says that the objects an evaluation makes need a literal type. CE-20 states the requirement only for variables. No rule covers a temporary, and the compilers split in C++23 mode. Take a class with a `volatile` member and a `constexpr` constructor and destructor. In a constant evaluation, GCC rejects `V(1).k()` and Clang accepts it. P11 adds a third kind of object, the exception object, and cannot say which rule applies to it. Suggestion: a rule that says whether a temporary needs a literal type, a test, and a question for the split.

**X10-12** (needs example; `foundations/constant-evaluation.md`, `declarations/constexpr.md`; CE-30, CE-P4, CEV-1). CE-30 says that each place in CE-P4 starts its own evaluation. No example or test shows the effect on a statement around the place. Two such cases are a `try` block around a `constexpr` variable in a `constexpr` function, and a `try` block around a `consteval` call at run time. Both compilers reject both programs today. CE-8 is the rule that rejects them, and CE-30 is not needed. So a reader can believe that the surrounding run-time `try` takes part. Suggestion: an example in CE-30 and a test for each case. The tests keep their outcome under P11, and under P11 they show the boundary.

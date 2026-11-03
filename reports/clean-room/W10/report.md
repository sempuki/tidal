# W10 report: errors

- Date: 2026-11-03
- Role: clean-room reader
- Persona: Go programmer
- Programs: transaction.cpp, error_codes.cpp, cleanup_guard.cpp, noexcept_functions.cpp

## Summary

All four programs built cleanly on the first attempt with `g++` (GCC 16.2.1) and `clang++` (Clang 22.1.8). The flags were `-std=c++23 -pedantic-errors -Wall -Wextra`. Neither compiler gave a warning, every program exited 0, and every program also exited 0 under `-fsanitize=undefined,address`. The exception pages were enough to write correct code. The problems came from eight probes that tested edge cases the programs depend on.

The three biggest problems:

1. **UNW-5 and EXS-P3 contradict DTOR-4 (W10-1).** UNW-5 says an exception leaving any destructor not declared `noexcept(false)` ends the program. DTOR-4 says a destructor with no specifier is potentially throwing when a member's destructor is. Both compilers follow DTOR-4. A probe threw out of such a destructor, and a handler caught the exception.
2. **Termination is not described in a way a program can observe (W10-2).** The pages say "the program ends" but do not say what the environment sees. I observed a message on stderr and SIGABRT, exit status 134. A Go programmer wants to know whether this is like an unrecovered panic, and whether static objects or other cleanup run.
3. **A program with no headers needs outside knowledge for two common error-handling tools (W10-3, W10-4).** The page gives `std::exception` no message member, so exception text needs `what()` from outside. No page mentions a way for a destructor to tell an exceptional exit from a normal one, so a rollback guard needs an explicit `commit()`.

## Construct-to-rule notes (made before writing)

**transaction.cpp**
- Declaring `std::exception` with no include: LSUP-8 and the language-support example. The class has a virtual destructor, and the library defines the destructor.
- Custom error classes derived from `std::exception`, and a handler for a base catching a derived object: TRY-2, TRY-P2, LSUP-8.
- `throw InsufficientFunds(from)` makes the exception object directly from a prvalue: THR-1, THR-P1.
- Rollback in a destructor unless committed: the objects of a block end on any exit, a throw included (OBJ-P2, UNW-1, UNW-P1). The destructor is noexcept with no specifier (DTOR-4), so it is safe during unwinding (UNW-5).
- Rollback of a whole batch in `catch (...)` followed by `throw;`, which passes on the same object: TRY-8, THR-6, TRY-3 (the catch-all comes last).
- Derived handler before base handler: TRY-1 (first match wins) and TRY-4 (warning on a shadowed handler).
- `static_assert` on a `constexpr` function that contains a `throw` on an untaken path: THR-10, CE-8, THR-P5. Early `return` rolls back in constant evaluation through a `constexpr` destructor: CE-1, OBJ-P2.

**error_codes.cpp**
- A result struct holding a value and an `enum class` code, unpacked with structured bindings (SB-P1). `[[nodiscard]]` on the type: ATTR-5.
- The low layer is `noexcept` (EXS-1). Checked with the `noexcept` operator (UN-18), whose operand is not evaluated (UN-13).
- Code to exception: `throw` on a path that constant evaluation does not take (CE-8, THR-10).
- Exception to code: a `noexcept` template that catches everything inside itself (UNW-6). A `try` block is valid in constant evaluation if nothing throws (CE-1). A lambda call is valid in constant evaluation (LAM-P1, CE-1).
- A lambda without `noexcept` is potentially throwing: EXS-4.

**cleanup_guard.cpp**
- Guard class template holding a closure and calling it in the destructor: OBJ-P2, UNW-1, DTOR-4.
- Reverse order of guards, as with Go's defer stack: OBJ-P2 ("reverse of the order in which they began").
- `Defer g([&]{...})` deduces `F` from the constructor: CTAD-P1, CTAD-1. Capture by reference: LAM-P2, with the dangling risk in LAM-P5.
- Deleted copy so that one guard owns the cleanup: CPY-P2. A factory returns a prvalue, which needs no copy or move: CPY-P4, VC-P2.
- The guard's destructor is noexcept, so a cleanup that throws ends the program: DTOR-4, UNW-5.
- A guard in a loop runs at the end of each iteration, unlike Go's defer: OBJ-P2 (automatic objects end with their block).

**noexcept_functions.cpp**
- Specifier forms and conditional `noexcept(expr)`: EXS-1, EXS syntax. `noexcept` operator: UN-18.
- `constexpr` does not imply noexcept, and a lambda needs an explicit `noexcept`: EXS-4.
- Specifier in function pointer types, dropping is fine, and adding is rejected: EXS-2, CONV-10. Overloads on a noexcept function pointer parameter: EXS-2, ICS-26.
- Dependent specifier in a template: EXS-9, EXS-10, EXS-P5.
- Implicit members follow their parts: EXS-4, EXS-P3, DTOR-4.
- Override keeps the promise: EXS-3, EXS-P2.
- A noexcept wrapper that catches inside: UNW-6. An exception that leaves it would end the program: THR-9, UNW-5.
- `noexcept(false)` destructor throwing when nothing else is in flight: UNW-6.

## Findings

### W10-1. UNW-5 and EXS-P3 say less than DTOR-4 about destructors

- Pages: exceptions/unwinding.md (UNW-5, UNW-P3), declarations/exception-specifications.md (EXS-P3), classes/constructors.md (DTOR-4), expressions/unary.md (UN-18).
- Kind: wrong.
- What happened: UNW-5 lists "leaves a destructor that is not declared `noexcept(false)`" as a cause of termination. UNW-P3 says a destructor "is `noexcept` unless the destructor says otherwise". DTOR-4 and UN-18 say something different. A destructor with no specifier is noexcept only if the destructors of all bases and members are. Probe p1 has a class with no destructor specifier, a member whose destructor is `noexcept(false)`, and a destructor body that throws. Both compilers report the destructor as potentially throwing. The exception leaves the destructor, and a handler in `main` catches it, so the program exits 0. UNW-5 predicts termination. The EXS-P3 table also says a function the program writes without a specifier is "Potentially throwing". That is not true for a user-written destructor whose parts all have noexcept destructors. EXS-4 gets the destructor case right by pointing to DTOR-4.
- Suggestion: In UNW-5, replace "not declared `noexcept(false)`" with "non-throwing ({rule}`DTOR-4`)". Change the sentence in UNW-P3 the same way. Give destructors their own row in the EXS-P3 table, or a note that DTOR-4 governs them.

### W10-2. What termination looks like from outside is not stated

- Pages: exceptions/unwinding.md (UNW-5, UNW-7, UNW-P3), exceptions/throw.md (THR-9, THR-P4).
- Kind: unclear.
- What happened: The pages say "the program ends" or "terminates", and "without returning from `main`". They do not say whether the end is normal or abnormal, what exit status a caller sees, or whether static objects are destroyed. They also do not say whether a program can run code at that point. The Evidence segment mentions "a nonzero status or a signal" only as a harness detail. Probes p2, p3, and p6 printed a "terminate called" message and died with SIGABRT (status 134) on both compilers. For a Go programmer the question is whether this matches an unrecovered panic, which prints a message and exits with status 2 after running deferred calls. Taken together with UNW-7, the answer is "maybe no cleanup at all". I had to piece that together.
- Suggestion: Add one rule or principle sentence that says what termination is: an abnormal end, statics not destroyed, and the status chosen by the implementation. If a termination hook exists in the library, mark it *(library referenced)*.

### W10-3. `std::exception` has no message member in the language-support shape

- Pages: foundations/language-support.md (LSUP-8, Evidence "Scope" bullet).
- Kind: outside knowledge.
- What happened: The page shows `std::exception` with a constructor and a virtual destructor only. The page states that the text member functions are left to the library. A Go programmer expects `Error() string`. To carry a message through a handler for `std::exception`, I had to know the name and exact signature of the library's virtual text member. Probe p4 declared that member from outside knowledge, overrode it, and linked and ran on both compilers. In the four main programs I avoided messages and used codes in derived classes instead.
- Suggestion: Either give the shape of the library's message member in LSUP-8 as *(library referenced)*, or say plainly that a program with no headers should put its own virtual message member in its own base class. An example of the second option would help.

### W10-4. No way to tell an exceptional exit from a normal exit in a destructor

- Pages: exceptions/unwinding.md, exceptions/handlers.md.
- Kind: missing.
- What happened: A rollback guard that undoes work only when an exception is in flight needs to know whether unwinding is in progress. No page mentions any such facility, even as *(library referenced)*. I used an explicit `commit()` flag in transaction.cpp. The flag works, and early `return` also rolls back, which suits the Go pattern `defer tx.Rollback()` with `tx.Commit()`. I only knew to look for something else because of outside knowledge.
- Suggestion: Add a sentence to the unwinding page saying that a destructor cannot tell from the language alone why it runs, and mark the library facility *(library referenced)*. Show the commit-flag pattern in an example.

### W10-5. The scope-guard and transaction patterns have no example

- Pages: exceptions/unwinding.md (Examples), classes/constructors.md (DTOR-4), classes/copy-move.md (CPY-P4).
- Kind: needs example.
- What happened: The unwinding example shows `Lock` and `Buffer` with destructors. No page shows the most common use of unwinding for a reader from Go: a guard object that holds a closure, can be dismissed, and runs at block exit. Several facts had to come from four pages:
  - the closure runs on throw and on return (UNW-1, OBJ-P2),
  - the guard's destructor is noexcept even when the closure can throw, so a throwing cleanup ends the program (DTOR-4, UNW-5; probe p2),
  - a guard with deleted copy can come from a factory only as a prvalue, because `return d;` of a named guard is rejected (CPY-P4; probe p8),
  - a guard ends at its block, so one in a loop body runs each iteration, unlike Go's function-scoped defer.
- Suggestion: Add a short guard example to the unwinding page with a dismiss flag, and a comment on each of the facts above.

### W10-6. Querying noexcept on destructors and on calls with reference arguments needs an idiom

- Pages: expressions/unary.md (UN-18, UN-13), declarations/exception-specifications.md.
- Kind: needs example, hard to find.
- What happened: The `noexcept` operator lives in expressions/unary.md. The exception-specifications page names "the `noexcept` operator" in EXS-9 without a link. To ask whether a destructor is noexcept, I wrote `noexcept(static_cast<T*>(nullptr)->~T())`. To ask about a call that takes references, I dereferenced a null pointer cast inside the unevaluated operand. Both rely on UN-13 (the operand is not evaluated), and both compilers accept them. No example shows these forms. The UN-18 example `noexcept(Calm())` tests construction and destruction together, which answers a different question.
- Suggestion: Link UN-18 from the exception-specifications page. Add an example that checks one destructor alone, and one call with a reference argument, in a program with no headers.

## Attempt log

| Step | File | Compiler | Result |
|---|---|---|---|
| 1 | transaction.cpp | g++, clang++, both sanitized | built with no warnings, exit 0 |
| 2 | error_codes.cpp | g++, clang++, both sanitized | built with no warnings, exit 0 |
| 3 | cleanup_guard.cpp | g++, clang++, both sanitized | built with no warnings, exit 0 |
| 4 | noexcept_functions.cpp | g++, clang++, both sanitized | built with no warnings, exit 0 |
| 5 | probe/p1_member_dtor.cpp | both | exit 0; the exception left a destructor with no specifier and was caught (W10-1) |
| 6 | probe/p2_defer_throws.cpp | both | no warning; terminate, SIGABRT 134 (DTOR-4 as stated; W10-2, W10-5) |
| 7 | probe/p3_unwind_throw.cpp | both | terminate, 134, as UNW-5 says for a throw during unwinding |
| 8 | probe/p4_what.cpp | both | links and runs with the message member declared from outside knowledge (W10-3) |
| 9 | probe/p5_nodiscard.cpp | both | warning on a dropped `[[nodiscard]]` result, as ATTR-5 says |
| 10 | probe/p6_throw_noexcept.cpp | both | warning (GCC `-Wterminate`, Clang `-Wexceptions`), then terminate, as UNW-8 and THR-9 say |
| 11 | probe/p7_handler_order.cpp | both | warning on a derived handler after its base, as TRY-4 says |
| 12 | probe/p8_guard_by_value_return.cpp | both | rejected: deleted copy used for `return d;` of a named guard (CPY-P4; W10-5) |

None of the four programs needed a fix.

## What worked well

- The handler-selection diagram in TRY-P1 and the match list in TRY-2 were exact. Derived-first ordering and the TRY-4 warning behaved as written.
- UNW-P1's block diagram made the cleanup order obvious. UNW-1's statement that all destruction completes before the handler runs let me check that the resources were closed inside the handler.
- CE-1 and CE-8 together told me exactly how to `static_assert` code that contains `try` and `throw`. The rule is that nothing may throw during the evaluation. A `throw` on an untaken path, and a `try` with no throw, both worked in constant evaluation on both compilers.
- The EXS pages were complete for the `noexcept` program. Function pointer types, overloads on noexcept, conditional specifiers in templates, override rules, and the "constexpr and lambdas are not implied noexcept" rule all matched.
- LSUP-8's no-header shape for `std::exception` linked against the library on both compilers.
- UNW-6 states the key pattern for "must not throw": catch inside the noexcept function.

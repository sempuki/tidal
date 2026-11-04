# Predict batches: exceptions, reader E

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-handlers (92 programs), R-throw (41), R-unwinding (39)

All three answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R-handlers/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-throw/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-unwinding/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ:
- handlers: p52, p71, p72, p87
- throw: none
- unwinding: p15

## RPE-1

- Pages and rules: exceptions/handlers, TRY-4
- Kind: Evidence missing
- What happened: Handlers p05 and p17 are built with `-Wall -Werror`. p05 has a handler for a derived class after a handler for its base. p17 has two handlers for `int`. The outcome depends on whether the compilers give the TRY-4 warning, and with which flags. The Evidence lists no compiler for TRY-4. The Evidence says that 88 of 92 tests agree, so I guessed that every compiler warns, and I answered reject for both. That guess comes from the count, not from the page.
- Suggestion: Add an Evidence line for TRY-4. It should say which compilers warn about a shadowed handler, and whether they warn by default or only under `-Wall`.

## RPE-2

- Pages and rules: exceptions/handlers, TRY-12, TRY-13
- Kind: unclear
- What happened: Handlers p52 and p71 have a function try block whose handler flows off the end of a function that returns `int`. The Evidence says that Clang warns, GCC does not, and EDG rejects "without any flag". So I answered `depends`. Two points were unclear:
  - TRY-12 is a recommendation, so I could not tell whether the EDG rejection counts as a valid outcome or as an EDG fault.
  - On GCC the program compiles and runs, and `main` returns 1, so the run fails. The page does not say which outcome the test expects for GCC.

  For p72 and p87 (TRY-13, `-Wall -Werror`), the Evidence says what Clang and GCC do, but not what EDG does.
- Suggestion: For each TRY-12 and TRY-13 test, give the outcome on each compiler, and include EDG for TRY-13. Say whether the EDG rejection for TRY-12 conforms.

## RPE-3

- Pages and rules: exceptions/unwinding, UNW-8, Q-0179
- Kind: hard to apply
- What happened: Unwinding p04 (`-Wall -Werror`) is a `noexcept` function. Inside it, a try block throws 1, and the handler for `int` rethrows. Under UNW-8 the rethrow is not enclosed by any try block, so the function should get a warning. The Evidence says that GCC warns about a rethrow only when the try block of the handler holds no direct `throw`, and that Clang does not warn about rethrows. The try block in p04 holds a direct `throw`, so I predicted that neither compiler warns, and answered compiles. I had to work this out from a negative statement about another case.
- Suggestion: Add an example to UNW-8 that shows each rethrow case and whether each compiler warns. Include the case where the try block holds a direct `throw`.

## RPE-4

- Pages and rules: exceptions/unwinding, UNW-8
- Kind: Evidence missing
- What happened: Unwinding p15 (`-Wall -Werror`) is a `noexcept` function whose try block throws an `int` and has only a handler for `long`. The Evidence says that Clang warns when no handler matches the static type of the operand, and that GCC does not. So I answered `depends`. The Evidence does not say what EDG does under `-Wall`. The same gap applies to p02, p37, and p39, where I assumed that EDG gives no error.
- Suggestion: Add EDG's behavior to the UNW-8 Evidence line.

## RPE-5

- Pages and rules: exceptions/throw, THR-1
- Kind: unclear
- What happened: Throw p05 throws a parameter from inside a try block in the function body. THR-1 says a parameter is moved if each try block around the `throw` "also contains the declaration of the variable". From the rule text alone I could not tell whether a parameter's declaration counts as contained by a try block in the body. The Evidence settles the case: the parameter is copied. I answered runs (copied) from the Evidence. Throw p39 (no try block, so the parameter is moved) matched the rule text directly.
- Suggestion: In THR-1, add one sentence that says a parameter is never inside a try block of its own function body or function try block. Add an example with a parameter that is copied and one that is moved.

## RPE-6

- Pages and rules: exceptions/handlers, TRY-2, TRY-18
- Kind: needs an example
- What happened: Handlers p19 throws a `Derived` prvalue, and the handler takes `Base` by value. `Base`'s copy constructor throws, and `Derived`'s copy constructor does not. TRY-2 says a copy to a base type slices, and TRY-18 says a throwing copy into the parameter ends the program. I predicted run-fail, because the parameter is made by `Base`'s copy constructor from the base part. The pages do not say which constructor makes a by-value base parameter.
- Suggestion: Add a sentence to TRY-2 saying that a by-value handler for a base is initialized by the base's copy constructor from the base part of the exception object.

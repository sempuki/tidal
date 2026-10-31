# Spec-only exercises

Spec-only readers (decision 0029) check the specification by using it.
This page lists the exercises they run. Decision 0030 sets the scheme.

Every exercise has three properties: a **kind**, a **persona**, and the
**pages** it targets. The reader reads only Tidal's own files, writes
findings by rule ID, and never cites ISO wording. Reports go under
`reports/spec-only/<exercise-id>/`.

## 1. Kinds

| Kind | The reader... | It finds | Scoring |
|---|---|---|---|
| Write (W) | builds a program for a task given in prose | missing task-level guidance, rules that are hard to find | by hand, from the report |
| Port (P) | translates a short program from the persona's own language | idioms with no stated equivalent | by hand |
| Predict (R) | reads a Tidal test with its directives removed, and says whether it compiles, links, runs, or has undefined behavior, citing rules | wrong, ambiguous, and missing rules | automatic, against the test's expectations |
| Diagnose (D) | gets a program and a compiler or sanitizer message, names the rule that explains it, and fixes the program | rules that do not match how compilers report errors | automatic for the rule ID; the fix is built and run |

Write and Port need tasks written by hand, and they test whether a
person can use the spec. Predict and Diagnose are generated from the
test suite, one batch per page, and together they reach every rule.

## 2. Personas

A persona is the reader's background language. Each one comes with
habits that the spec has to correct, so each exercise uses the persona
whose habits are most likely to mislead it.

| Persona | Habits that the spec must correct | Best for |
|---|---|---|
| Java | garbage collection, reference semantics, checked exceptions | ownership, copy and move, lifetime |
| C# | value and reference types, properties, `using` | classes, conversions, initialization |
| OCaml | variants and pattern matching, immutable data | unions, `switch`, `const`, conversions |
| Haskell | purity, laziness, type classes | constant evaluation, lambdas, sequencing |
| Rust | ownership and borrows, checked overflow in debug builds | arithmetic, moves, references |
| Go | error values, `defer`, multiple returns, no inheritance | exceptions, destructors, structured bindings |
| Ada | representation clauses, range checks, packages | low-level layout, interop, linkage |
| Swift | definite initialization, optionals, value types | initialization, objects, lifetime |

Predict and Diagnose batches use a neutral persona: a programmer new to
C++.

## 3. Write suites

Each suite has three to five tasks. A task names what the program must
do and how it reports success (`static_assert` and `main`'s return
value, as the tests do). Tasks never use `#include`, because the
library is out of scope.

| ID | Theme | Persona | Target pages | Tasks |
|---|---|---|---|---|
| W0 | Pilot | Java | classes, exceptions, enumerations, constant evaluation | Done: bank accounts, ring buffer, owning array, shapes, calculator |
| W1 | Numbers and bits | Rust | primary, conversions, additive, multiplicative, shift, bitwise, unary, assignment | checked integer arithmetic that refuses to overflow; a fixed-point number type; a bit set over an array of unsigned words; a checksum over a byte array; a parser for decimal and hexadecimal digit strings |
| W2 | Comparison and logic | OCaml | comparison, logical, conditional, comma, evaluation-order, values | a three-way compare for a record type; a short-circuit validator chain; a clamp and a sort of three values; a test that shows which operand orders are fixed |
| W3 | Control flow | Go | blocks, if, switch, loops, range-for, jumps | a state machine for a vending machine; a tokenizer for arithmetic text; a binary search; a nested-loop search that leaves both loops early |
| W4 | Pointers and memory | Java | declarators, new-delete, objects, postfix, alignment | a singly linked list with insert and remove; a 2D grid stored in one array; a bump-pointer arena; a dynamic array that grows |
| W5 | Functions | Go | definitions, default-arguments, deleted, return-deduction, auto, decltype, inline | a function table for commands; overloads with deleted conversions; a function that returns two values; recursion with a deduced return type |
| W6 | Initialization | Swift | initialization forms, list, references, static, structured-bindings, constexpr (`constinit`) | a configuration table; a struct with default member values; reference members and the life of temporaries; unpacking records with bindings |
| W7 | Value types | C# | members, access, constructors, initializer-lists, conversion-functions, operators, comparisons | a rational number; a date with validation; a money type that refuses mixed currencies; a small string with fixed capacity |
| W8 | Ownership | Java | copy-move, constructors, new-delete, unions, unwinding | a unique owner of one object; a shared owner with a count; a tagged union that owns its payload; a container that stays valid if an element constructor throws |
| W9 | Inheritance | OCaml | derived, virtual, casts, member-pointers, access | a plugin interface with two implementations; a visitor over an expression tree; a mixin base with protected state; dispatch through member pointers |
| W10 | Errors | Go | throw, handlers, unwinding, exception-specifications | a transaction that rolls back on error; error codes beside exceptions; a cleanup guard in place of `defer`; `noexcept` functions that must not throw |
| W11 | Compile time | Haskell | constexpr, constant-evaluation, static-assert, enumerations, translation | a lookup table built at compile time; compile-time unit tests for a math function; an enum-to-name table checked at compile time; a `consteval` validator |
| W12 | Callbacks | Haskell | lambdas, member-pointers, auto, decltype | an event dispatcher; a sort that takes a comparator; a function that composes two functions; a counter captured by a lambda |
| W13 | Low level | Ada | volatile, alignment, unions (bit-fields), casts, attributes, linkage-specifications | a device-register map; a packed header with bit-fields; a function with C linkage; a buffer aligned for a wider type |
| W14 | Program structure | Ada | storage-linkage, namespaces, inline, static initialization, names, type-names, translation | a program in three translation units; a namespace with internal helpers; a shared constant across units; a counter whose initialization order matters |

Port suites (P1 to P8) follow one per persona after the Write suites.
Each gives three short programs in the persona's language that use its
typical idioms, and asks for a C++ program with the same behavior.

## 4. Predict and Diagnose batches

`R-<page>` and `D-<page>` name a batch for one spec page, for example
`R-additive`. A batch takes the page's tests, removes every `tidal-`
comment, and asks for one answer per program. Scoring compares the
answer with the test's expectations. A wrong answer that cites a rule
is a finding against that rule. A Diagnose batch uses only tests that
carry a `tidal-error` pattern, and shows the reader the matching
compiler message.

## 5. Order

1. Predict batches for the Foundations pages, then the expressions
   pages. They are cheap and scored without reading.
2. W8 and W1, which target the areas where the pilot found the most
   problems.
3. The other Write suites in table order.
4. Diagnose batches, after the tool that builds them exists.
5. Port suites.

## 6. Coverage

Each report lists the rule IDs it cites. The coverage table counts,
for each page, the exercises that reached it and the findings against
it. A page with no Write suite and no finding gets one before the
catalog counts as done.

## 7. Status

| Exercise | State | Report |
|---|---|---|
| W0 | done | `reports/spec-only/java-programmer/` |
| Front end | running | `reports/spec-only/rust-frontend/` |
| All others | planned | |

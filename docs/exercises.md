# Clean-room exercises

Clean-room readers (decision 0029) check the specification by using it.
This page lists the exercises they run. Decision 0030 sets the scheme.

Every exercise has three properties: a **kind**, a **persona**, and the
**pages** it targets. The reader reads only Tidal's own files, writes
findings by rule ID, and never cites ISO wording. Reports go under
`reports/clean-room/<exercise-id>/`.

## 1. Kinds

| Kind | The reader... | It finds | Scoring |
|---|---|---|---|
| Write (W) | builds a program for a task given in prose | missing task-level guidance, rules that are hard to find | by hand, from the report |
| Port (P) | translates a short program from the persona's own language | idioms with no stated equivalent | by hand |
| Predict (R) | reads a Tidal test with its directives removed, and says whether it compiles, links, runs, or has undefined behavior, citing rules | wrong, ambiguous, and missing rules | automatic, against the test's expectations |
| Diagnose (D) | gets a program and a compiler or sanitizer message, names the rule that explains it, and fixes the program | rules that do not match how compilers report errors | automatic for the rule ID; the fix is built and run |
| Propose (X) | writes the design and impact sections of a proposal for a new feature, citing the rules it changes or meets | missing rules that a change to the language must state its effect on | by hand, from the report |

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

Propose uses a language designer (decision 0037). The designer knows
what a proposal must settle and takes all of its C++ knowledge from
Tidal. It does not recall WG21 papers or the ISO text, and its features
are its own inventions.

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

## 4. Propose exercises

Each exercise takes one proposal from the designer's portfolio
(`reports/clean-room/roadmap/designer.md`). The reader writes its
design and impact sections, cites the rules it changes, and lists each
change in a table: rule ID, kind of change, old class, new class, and
the tests affected. Together the four reach 47 of the 68 pages.

| ID | Proposal | Changes existing behavior |
|---|---|---|
| X1 | P3, left-to-right evaluation everywhere | yes |
| X2 | P13, checked `noexcept` | yes |
| X3 | P5, automatic objects start at zero | yes |
| X4 | P7, temporaries live through reference-returning chains | yes |

## 5. Predict and Diagnose batches

`R-<page>` and `D-<page>` name a batch for one spec page, for example
`R-additive`. A batch takes the page's tests, removes every `tidal-`
comment, and asks for one answer per program. Scoring compares the
answer with the test's expectations. A wrong answer that cites a rule
is a finding against that rule. Two outcomes let a reader be right
about the hard cases: `undefined` (the program compiles and its run has
undefined behavior) and `depends` (the compilers differ, as the spec
records). A Diagnose batch uses only tests that
carry a `tidal-error` pattern, and shows the reader the matching
compiler message.

`tools/exercises.py predict PAGE build/exercises/R-<name>` builds a
Predict batch in a shuffled order and keeps its key in
`build/exercise-keys/`. `tools/exercises.py score build/exercises/R-<name>
ANSWERS` scores the reader's answers and lists the wrong ones.

## 6. Order

1. Predict batches for the Foundations pages, then the expressions
   pages. They are cheap and scored without reading.
2. W8 and W1, which target the areas where the pilot found the most
   problems.
3. The other Write suites in table order.
4. Diagnose batches, after the tool that builds them exists.
5. Port suites.

## 7. Coverage

Each report lists the rule IDs it cites. The coverage table counts,
for each page, the exercises that reached it and the findings against
it. A page with no Write suite and no finding gets one before the
catalog counts as done.

## 8. Status

| Exercise | State | Report |
|---|---|---|
| W0 | done | `reports/clean-room/java-programmer/` |
| Front end | done: 2589/3199 pass, 0 false rejects (`conformance.tsv`) | `reports/clean-room/rust-frontend/` |
| R-translation, R-objects, R-values, R-names, R-conversions, R-evaluation-order, R-constant-evaluation | done, 231/231 | `reports/clean-room/R-foundations/` |
| Designer roadmap | done, findings D1 to D33 | `reports/clean-room/roadmap/designer.md` |
| X1, X2, X3, X4 | done: 30, 20, 22, and 17 rule changes; findings X1-1 to X4-16 | `reports/clean-room/X1/` to `X4/` |
| All others | planned | |

## 9. Prompts

Each exercise runs as one agent with a prompt built from these parts.
Fill in the angle brackets.

### Shared rules (every prompt)

> Your only documentation is the Tidal specification under
> ~/Development/tidal/spec/ (start with spec/index.md, spec/glossary.md,
> and spec/foundations/translation.md). Read only files under spec/,
> docs/, and <the exercise's own directories>. Do not read tests/,
> build/exercise-keys/, questions/, reports/, or inputs/, and do not use
> git log or git show. Do not use the web. Do not consult or recall the
> ISO C++ standard, its drafts, cppreference, WG21 papers, or compiler
> source. Never quote or paraphrase ISO wording or cite ISO section
> names. When you use knowledge the spec did not give you, log it as an
> "outside knowledge" finding. Write only in <output directory>, and run
> no git command that changes anything. If a file write is refused,
> return its content in your final message. Write plainly: short
> sentences, no em-dashes, no "not X but Y".

### Write and Port

> You are an experienced <persona> programmer who has never written C++.
> Write these programs from the spec alone, one .cpp file each, without
> #include: <tasks from the suite>. Before writing, note the rule and
> principle IDs that tell you how to write each construct. Build each
> with `g++` and `clang++` using `-std=c++23 -pedantic-errors -Wall
> -Wextra`, run it, and check that it exits 0. Build once more with
> `-fsanitize=undefined,address` and run it. When a compiler rejects or
> warns, or the program misbehaves, record what the spec said that led
> you there, then fix the program. Report: a summary with the three
> biggest problems; findings, each with pages and rule IDs, kind
> (missing, unclear, wrong, hard to find, needs example, outside
> knowledge), what happened, and a suggestion in your own words; an
> attempt log; and what worked well.

For Port, replace the task list with three short programs in the
persona's language and ask for C++ programs with the same behavior.

### Predict

> You are a programmer new to C++. Do these batches in order:
> build/exercises/R-<page>, ... For each, read its README.md and the
> spec pages you need, and predict each program's outcome without
> compiling anything. When general C++ knowledge disagrees with the
> spec, answer from the spec and say what it is missing. Write each
> batch's answers to its answers.json. In your final message, list spec
> findings with batch and program ID, rule IDs, what you needed, and a
> suggestion.

### Front end

The front end prompt gives milestones (lexer and conformance harness,
parser from the Syntax segments, `reject` checks, constant evaluation
for `static_assert`) and asks for findings of kind missing, ambiguous,
contradictory, grammar gap, untestable, outside the spec, or needs
example. The reader may also read `tests/` and `questions/`, and writes
only in `frontend/` and its report directory. It can use any Rust crate
that does not encode knowledge of C++, and it covers the pages that are
new or changed (decision 0035). `tools/frontend_check.py` compares its
results on every check with a committed baseline.

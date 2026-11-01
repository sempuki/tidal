# Roadmap, round 3: what to do after phase 5, from a programmer's view

Date: 2026-11-01. Decision 0029. This report comes from a clean-room reader in the role of a programmer who wants to write ordinary C++ programs from Tidal alone. The reader read AGENTS.md, CLAUDE.md, the charter, roadmap, status, exercises catalog, and decisions 0033, 0039, 0046, and 0050. The reader also read the round 2 programmer report and these pages in part: language-support, storage, types, translation, primary, unions, members, copy-move, new-delete, lambdas, and unwinding. The reader read `questions/README.md` and the phase 5 reports (W15, D-classes, R-templates A to C, X5 to X8, and front-end report 9). The reader counted with `grep`, `awk`, and a short Python script.

The reader built five sketches in the session scratchpad with host GCC 16.2.1 and Clang 22.1.8 (`-std=c++23 -pedantic-errors -Wall -Wextra`) and ran them through `frontend/target/release/tidal-fe run`. No file in the repository was written. Statements that rest on general knowledge say so.

## 1. What a programmer still cannot do from the specification alone

Phase 5 closed the round 2 walls. I can now build a growable vector with placement `new` (STG-5, LSUP-7), aligned allocation (LSUP-5, NEW-26), and destruction (DTOR-6). I can make a union member active by construction (UNI-14), keep the pointer that placement `new` returns (STG-P4), and ship templates in headers (`odr.md` now mentions "template" 33 times). My vector sketch, with a throwing copy constructor and an over-aligned element, ran clean under both compilers and both sanitizers.

These gaps remain.

### 1.1 Most of the specification has never been used to write a program

- `documentation/exercises.md` section 8 lists W0 and W15 as the only Write suites run. W1 to W14 and all Port suites are "planned".
- Predict and Diagnose batches have reached 22 of the 85 pages: 7 Foundations pages, the 11 template pages, and 4 classes and exceptions pages. No expressions, statements, or declarations page has had a batch. Seven Foundations pages have had none, among them overload resolution (44 rules, 164 tests), conversion sequences, storage, language-support, and odr.
- Predict and Diagnose scores are close to perfect: 231/231, 305/305, 339/342, 445/445, and 143/143. Across 1089 Predict programs the readers filed 56 findings, about 5 per 100 programs. W15 filed 9 findings from 5 programs, 4 of them of kind "missing".
- 297 of 1632 rules still have one test.

### 1.2 Character values are not stated

- PRIM-5 gives the type of a character literal and lists escape forms. No rule gives the value of `'a'`, `'0'`, or `'\n'`.
- No rule says the digit characters have consecutive values. No test computes `c - '0'` (`grep` finds 0 files).
- "Ordinary encoding" appears in PRIM-23 and near line 48 of `primary.md`. The glossary does not define it.
- W1 asks for "a parser for decimal and hexadecimal digit strings" and W3 for "a tokenizer for arithmetic text". Both need these facts. General knowledge says the reference guarantees consecutive digits and leaves the rest of the encoding to the implementation.

### 1.3 Some language-support operations cannot be written or linked without a header

Decision 0041 gives shapes a program can declare. Some operations have no shape that works:

| Operation | What happens today | Evidence |
|---|---|---|
| the launder operation | a self-declared `std::launder` fails to link on both compilers | W15-2, `language-support.md` line 22 |
| `typeid` comparison | my sketch declares `std::type_info` with `operator==`; both compilers compile it and both fail to link it | sketch `rtti.cpp`; `typeid` has no page and 2 tests |
| reading a value's bits in constant evaluation | no core construct does it; at run time a loop over `unsigned char` works (STG-14) | 0 mentions of `bit_cast` in `specification/` and `tests/` |
| "T is trivially destructible", "trivially copyable" | not nameable as a library trait; see 1.4 | CLS-19, CPY-19 markers |
| the default new alignment as a value | NEW-27 names a predefined macro, *(preprocessor referenced)* | `new-delete.md` near line 584 |

GCC and Clang both accept `__builtin_launder`, `__builtin_bit_cast`, `__is_trivially_destructible`, `__is_trivially_copyable`, and `__is_standard_layout` (sketch `bi.cpp`). No page or test uses them.

### 1.4 Class properties are still partial lists

- CLS-19 (standard-layout) and CPY-19 (trivially copyable) still say "others that this page does not list yet *(class properties referenced)*". These are 2 of the 13 remaining markers.
- I can observe a trivial destructor and a trivial copy constructor with a union probe. A union whose member has a nontrivial special member gets a deleted one (UNI-3), and a requires expression detects that. My sketch `triv.cpp` builds a conditionally trivial optional this way, and both compilers agree.
- The same probe fails for the move constructor. A union's deleted defaulted move takes no part in overload resolution, so the probe falls back to copy (inference from both compilers' results). So a test cannot observe every property this way.
- Decision 0050 still defers items S and T "for a decision on how to observe a property that only the library names".

### 1.5 A program cannot produce output

- TR-P5 lists "the data the program writes out" as observable behavior. No rule says how a program writes data.
- A program can declare `extern "C" int puts(const char*);` (linkage-specifications page), and both compilers link it. Nothing in Tidal says the C library is there. The interpreter stops with "function 'puts' has no definition in the program".
- Tests and exercises report through `static_assert` and the exit status, and that works. A new reader's first program prints, though (general knowledge).

### 1.6 The interpreter cannot yet check ordinary programs

Front-end round 9 runs 942 of 1203 run tests. 225 are unsupported. Section 4 of report 9 lists generic lambdas, overloaded operators the front end leaves unknown, the object representation, and `new` with a replaced allocation function. My sketches gave these results:

| Sketch | GCC, Clang | `tidal-fe run` |
|---|---|---|
| growable vector with aligned allocation | exit 0, sanitizers clean | "cast of unknown operand" at the call of the aligned `::operator new` by name |
| same, without aligned allocation | exit 0 | "bytes of storage that holds an object of another type" in a throwing copy constructor during growth (likely the implicit-object choice of G81, inference) |
| hello world through `puts` | prints, exit 0 | no definition for `puts` |
| conditionally trivial optional through a union probe | exit 0 | rejects a correct `static_assert` (SA-1). The front end appears not to treat the union's destructor as deleted inside a requires expression (inference). |
| typeid through a self-declared `type_info` | link failure on both | not run |

### 1.7 No guide covers generic code or raw storage

`guides/index.md` lists five guides, none about templates, containers, or programs across files. Round 2 item 7 asked for three. Phase 5 did not schedule them. W15-7 records the slot-union idiom as outside knowledge. Decision 0046 keeps all guides unpublished.

## 2. Ranked work items

| # | Item | Size | Risk | Changes scope |
|---|---|---|---|---|
| 1 | Run the Write suites W1 to W14, first wave of six | one session per suite | low | no |
| 2 | Character values and the ordinary encoding | 3 to 5 rules, about 15 tests | medium | no |
| 3 | Language-support round 3: operations that need the implementation | about half of `language-support.md`, plus a page for `typeid` | medium | yes (extends 0041) |
| 4 | Class properties: the full lists (lift S and T) | about `types.md` | medium to high | no (lifts a deferral) |
| 5 | Predict and Diagnose batches for the 63 pages with none | one session per 4 pages | low | no |
| 6 | Interpreter round 10 on whole programs | large | medium | no |
| 7 | Guides: a container, a constrained algorithm, a program across files, the program's boundary | one session per guide | low | no |
| 8 | Write programs become matrix run tests | small | low | no |
| 9 | Stale markers and hand-written counts | tiny | none | no |

### Item 1. Run the Write suites W1 to W14

**What it is.** Run the planned Write suites in a first wave that covers the most pages: W1 (numbers and bits), W3 (control flow), W4 (pointers and memory), W8 (ownership), W7 (value types), and W14 (program structure). Run the rest in a second wave. Each program also goes through `tidal-fe run`.

**Evidence.** Section 1.1. Two of fifteen suites have run. Write found "missing" rules at a far higher rate than Predict did. W4's arena and dynamic array and W8's exception-safe container now have the rules they waited for (F1, F2, UNI-14). Nobody has tried them.

**What it unblocks.** Findings on the 60 or so pages no task has used, and the evidence that items 2 to 7 need.

**Size.** One session per suite, six in the first wave.

**Risk.** Low. Reports pass `tools/similarity.py` as usual.

### Item 2. Character values and the ordinary encoding

**What it is.** Rules on `primary.md` that give:

- the value of a one-character literal as its code unit in the ordinary encoding,
- that the decimal digits have consecutive values,
- the values of the simple escapes on the matrix targets, as a table,
- the ordinary encoding of the matrix targets (implementation-defined, recorded as a table per decision 0036),
- a glossary entry for "ordinary encoding".

**Evidence.** Section 1.2. 0 tests compute a digit's value. W1 and W3 need it.

**What it unblocks.** Parsers, tokenizers, checksums over text, and the W1 and W3 tasks.

**Size.** 3 to 5 rules, about 15 tests.

**Risk.** Medium for similarity. General knowledge says the reference states the digit guarantee in a short, well-known sentence, so the writer should state it as a table or a property. The charter keeps lexical structure beyond G1 out. Literal values belong to primary expressions, which are in scope.

### Item 3. Language-support round 3: operations that need the implementation

**What it is.** A source-reader inventory, then rows for each operation that a program cannot write in the core language:

- the launder operation,
- the bit copy between types of equal size, also in constant evaluation,
- the triviality and layout properties, as named queries,
- the `typeid` expression and the shape of `std::type_info`, with its comparison,
- the default new alignment as a value without the macro.

For each operation, decide how a test reaches it. I see two options:

| Option | For | Against |
|---|---|---|
| A support header under `tests/` that declares each `std` name and defines it with a builtin that GCC, Clang, and EDG share | Tests stay free of the library. The front end implements a handful of builtins. The rule text names only the `std` operation and its behavior. | The builtins are extensions (general knowledge), so the header is not portable C++. EDG's support for each builtin is unverified. |
| Tests may include the implementation's `<new>`, `<bit>`, `<type_traits>`, and `<typeinfo>` | Real headers, real behavior | The front end cannot parse library headers. Results depend on the library version in each container. |

**Recommendation.** The support header. Record "do all three compilers accept each builtin" as a question, and check it on the EDG container before drafting.

**Evidence.** Section 1.3. The launder link failure (W15-2), the `type_info` link failure, and the NEW-27 macro.

**What it unblocks.** Item 4's tests, conditionally trivial wrappers, type-erased small buffers that need laundering, and RTTI-based dispatch in W9.

**Size.** About half of `language-support.md`. A short `typeid` section can sit on the postfix page or a new page.

**Risk.** Medium. Similarity risk is low to moderate if the rows stay as behavior tables. Overrides are likely on EDG.

### Item 4. Class properties: the full lists

**What it is.** The complete conditions for trivially copyable, standard-layout, trivial special members, and implicit-lifetime types. These are deferred items S and T.

**Evidence.**

- The CLS-19 and CPY-19 markers.
- STG-8 creates implicit objects of implicit-lifetime types, so the storage page leans on a term with no full definition (glossary line 278).
- CNS-21 makes triviality depend on constraints, and the front end does not model it.
- Section 1.4 shows that union probes observe some properties, and the move probe fails.

**What it unblocks.** Byte copies with a stated guarantee, conditionally trivial `Optional` and `Result`, and the interpreter's choice of implicit objects (G81).

**Size.** About `types.md`, 8 to 12 rules.

**Risk.** Medium to high. The lists are long enumerations, and general knowledge says the reference states them in a familiar shape. A source-reader inventory and a probe grid should come first, as for items L and M. Item 3 should come first so the tests can name the properties.

### Item 5. Predict and Diagnose batches for the 63 pages with none

**What it is.** Batches in this order: overload resolution and conversion sequences, storage and language-support, the initialization pages, then the expressions, statements, and declarations pages. Item 5 also adds a hold-out set (see process change 2).

**Evidence.** Section 1.1. Overload resolution has 26 overrides and 17 questions and has never been predicted. Overload resolution is the place where a programmer's guess most often differs from the compiler's choice (general knowledge).

**What it unblocks.** Wrong and unclear rules on pages that no reader has checked by use.

**Size.** One session per four pages, about 16 sessions.

**Risk.** Low.

### Item 6. Interpreter round 10 on whole programs

**What it is.** Run the Write programs (item 1), the W15 programs, and this report's sketches. Close the constructs they hit: allocation functions called by name with an alignment, a throwing copy into allocated storage, generic lambdas, overloaded operators, deleted union special members in requires expressions, and `initializer_list` constructors. Decide whether the interpreter gives `extern "C"` functions such as `puts` a stub, so a reader can see output.

**Evidence.** Section 1.6, and section 4 of front-end report 9.

**What it unblocks.** A reader can check a program and its undefined behavior without a compiler (decision 0048).

**Size.** Large.

**Risk.** Medium. The implicit-object model should follow item 4.

### Item 7. Guides

**What it is.** Four how-to guides, each about the size of `guides/resource-owner.md`:

- a container in raw storage, which is now writable from F1, F2, and UNI-14,
- a constrained algorithm with hand-written `move`, traits, and concepts,
- a program across files with templates in headers,
- the program's boundary, covering `main`, `argv`, the exit status, and how a program calls a C function under `extern "C"`, with output named as outside Tidal.

Item 7 also needs a decision on where guides are published (decision 0046 leaves them unpublished).

**Evidence.** Section 1.7, W15-7, and TR-P5's "data the program writes out".

**What it unblocks.** The everyday path through 358 template rules and 27 storage rules. Guides found facts that no rule states in phase 2 (decision 0045).

**Size.** One session per guide.

**Risk.** Low.

### Item 8. Write programs become matrix run tests

**What it is.** The W0 and W15 programs, and later Write programs, go under `tests/` as run tests that cite their rules. They then run on all eight compilers and in the interpreter.

**Evidence.** The W15 programs live only in `reports/clean-room/W15/`. A later rule change can break them without any check failing.

**What it unblocks.** A regression signal at the level of a whole program.

**Size.** Small.

**Risk.** Low. The programs are clean-room work.

### Item 9. Stale markers and hand-written counts

**What it is.** Small repairs:

- STG-5 still says the placement form is *(library referenced)*. LSUP-7 now gives that form.
- `status.md` says 283 questions are open and 16 are compiler defects. The index has 288 provisional, 1 open, and 20 compiler defects.
- TR-P5's "data the program writes out" needs a pointer to item 7's guide or a note that output is outside Tidal.

**Size.** Tiny. **Risk.** None.

## 3. Scope changes

| Topic | Proposal | For | Against |
|---|---|---|---|
| Operations that need the implementation (item 3), and `typeid` | Add them, as decision 0041 rows plus a support header | Decision 0041's reasoning applies. The core language or an ordinary program depends on each one, and a program cannot write or link any of them. F2 already moved allocation shapes in by the same reasoning. | Each row moves the core and library line. The builtins are extensions. |
| Program output | Keep I/O out. Add the boundary guide (item 7), and say in TR-P5 that output goes through functions outside Tidal. | Programs print (general knowledge), and TR-P5 already counts output as observable. | I/O is all library. The exit status covers every test and exercise. |
| The preprocessor beyond `#include "file"` | Keep out. Item 3 replaces the one macro value a program needs (NEW-27). | `assert` and conditional compilation are common (general knowledge). | Large topic, high similarity risk, and no sketch needed it once item 3 exists. |
| Threads and the memory model | Keep out for phase 6 | Newer programs use threads (general knowledge). | Large, high risk, and outcomes vary across the matrix. No sketch needed them. |
| Coroutines, modules, extended floating-point types | Keep out | none from my sketches | as the charter says |

## 4. Process changes

1. **Write before Predict.** Predict scores are 98 to 100 percent and yield about 5 findings per 100 programs. Write yields about 2 findings per program. Give Write suites the first claim on exercise sessions in phase 6.
2. **Hold-out Predict programs.** The tests and the page Examples come from the same writers, and a reader can match one to the other. Each Predict batch should include about 20 programs that a separate clean-room writer builds from the task catalog, with outcomes taken from the matrix.
3. **Every Write program runs in the interpreter.** Each "unsupported" stop becomes a front-end work item, and each wrong stop becomes a finding.
4. **Generate the prose counts on `status.md`.** `tools/status_table.py` already makes the table. The question counts in the prose have drifted (item 9).
5. **Roadmap sketches are kept.** Round 2 and round 3 sketches live only in reports or scratch directories. A roadmap round should save its sketches under its report directory, so the next round can rerun them.
6. **Coverage by use.** Add to `exercises.md` section 7 a column per page that records whether a Write, Predict, or Diagnose exercise has reached the page. Today a reader must work this out by hand (22 of 85 pages for Predict and Diagnose).

## 5. Suggested order

Start item 9 and the first wave of item 1 at once. Run item 2 beside them, since W1 and W3 need it. Item 3 follows, then item 4, which needs item 3's queries. Items 5 and 6 run beside the others. Item 7 follows the first Write wave, and item 8 follows each Write suite.

## 6. Doubts for the clean-room writers

- Whether GCC, Clang, and EDG all accept `__builtin_launder`, `__builtin_bit_cast`, and the `__is_trivially_*` queries. Host GCC and Clang do. EDG is unchecked.
- Whether a union probe for the move constructor can ever observe triviality, given that a deleted defaulted move takes no part in overload resolution.
- Whether the values of the simple escapes belong in a rule or in a per-target table.
- Whether the interpreter should give `puts` a stub, or whether Tidal's position is that output is not modeled.

# Roadmap, round 5: what to do after phase 7, from a programmer's view

Date: 2026-11-03. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs. The reader read AGENTS.md, CLAUDE.md, the round 4 programmer report (`reports/clean-room/roadmap-4/programmer.md`) and its kept sketches, the charter, roadmap, status, style guide, specification format, exercises catalog, decisions 0033, 0039, 0044, 0050, 0051, and 0054, and `documentation/decisions/pending.md`. The reader also read parts of `language-support.md`, `storage.md`, `dependent-names.md`, `instantiation.md`, `concepts.md`, and `names.md`, the headers of `tests/programs/`, `questions/README.md` with Q-0226, Q-0322, and Q-0353 to Q-0356, front-end reports 15 to 17, the phase 7 Predict rows of `exercises.md`, the opening sections of the 2026-11-03 gap reviews, and `git log --oneline 2798b37~1..HEAD` (22 commits). The reader did not read the other round 5 reports. All counts come from read-only `grep` and Python scripts on the tree at commit `7d8c982`.

The reader built the front end from `7d8c982` into the session scratchpad, ran `tidal-fe conform` on all 6337 tests, and reran the twelve round 4 sketches. The reader wrote three new programs (405 lines) and reduced each interpreter stop to a short program. Each program was built with host GCC 16.2.1 and Clang 22.1.8 (`-std=c++23 -pedantic-errors -Wall -Wextra`, then `-fsanitize=undefined,address`), checked on the EDG 7.0 container (`cpfe --c++23 --strict`), and run through `tidal-fe run`. The reader also extracted the 181 `cpp` example blocks of `specification/` and checked each one alone on GCC, Clang, and `tidal-fe check`. The sketches are in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/rm5/programmer/sketches/`. No file in the repository was written. Statements that rest on general knowledge say so.

## 1. The state from a programmer's view

Phase 7 took the program-level work that round 4 ranked first and third. All 67 Write programs are run tests, and all 67 run to result 0 in the interpreter. The matrix found four compiler defects in them. Phase 7 ran no exercise by use, though. It added Predict batches, gap reviews, and Propose reports, and no Write, Port, Diagnose, or Guide session. My three new programs found more interpreter faults than any phase 7 report about whole programs did, and one of the faults is a false report of undefined behavior.

### 1.1 Counts

| Measure | Round 4 (`3d98c93`) | Now (`7d8c982`) | Source |
|---|---|---|---|
| Rule headers | 1678 | 1691 (1676 in force, 15 withdrawn) | `:::{rule}` headers and `:kind:` lines in `specification/` |
| Content pages | 85 | 85 | pages with a rule |
| Tests | 6032 | 6337 | files with `tidal-rules` under `tests/` |
| Rules with one test | 3 | 8 | Single-test column of `status.md` |
| Write programs as run tests | 0 | 67 under `tests/programs/` | `tidal-expect: run` in that directory |
| Write programs that run to a result in `tidal-fe` | 50 of 67 | 67 of 67 | front-end report 17, and this report's run |
| Rules cited by a program test | not measured | 477 of 1676 | `tidal-rules` in `tests/programs/` |
| Pages reached by a program test | not measured | 80 of 85 | the pages of those rules |
| Rules cited only by a program test | not measured | 0 | the same script |
| Pages with a Predict batch | 75 of 85 | 85 of 85 | answers files under `reports/clean-room/R-*` |
| Pages with a Diagnose batch | 4 | 4 | `exercises.md` section 8 |
| Port suites | 0 of 8 | 0 of 8 | `exercises.md` section 3 |
| Write sessions in the phase | 14 | 0 | `git log 2798b37~1..HEAD` |
| Guides | 5 (last change 2026-10-31) | 5 (last change 2026-10-31) | `guides/` |
| Questions | 352: 310 open, 30 settled, 12 compiler defects | 376: 209 open, 154 settled, 13 compiler defects | `questions/README.md` |
| *(referenced)* markers | 29 | 32: library 20, extended floating-point 5, coroutines 3, preprocessor 2, memory model 1, threads 1 | `status.md`, and a count of `referenced)` |
| Front end, tests that pass | 5595 of 6032 | 6068 of 6337 (5929 pass, 139 differ from EDG only) | `conformance.tsv` |
| Front end, run-unsupported | 112 | 65 | `conformance.tsv` |
| Front end, false rejects and wrong run results | not counted | 4 and 1 | `conformance.tsv` |
| Example blocks that GCC and Clang accept alone | not measured | 51 of 181 | this report's script |
| Tests that open `namespace std` | not counted | 200, 25 of them program tests | `grep` |

The 4 false rejects are `flow-off-end-constant-paths.cpp` (FN-7), `tuple-index-size-type.cpp` (SB-5), `two-classes-promoted-types.cpp` (COND-11), and `char-named-escape.cpp` (PRIM-5). The wrong run result is `return-local-destructor-throws-caught-inside.cpp` (UNW-1). The most common run-unsupported reasons are a `new` expression (8), a constructor call (6), `_Exit` with no definition (4), and a block-scope static used before its declaration ran (4).

### 1.2 Round 4 items that phase 7 took and left

| Round 4 item | Phase 7 | Result |
|---|---|---|
| 1. Write programs and sketches become run tests | item WP | Done for the 67 Write programs. The round 4 sketches stayed in `reports/clean-room/roadmap-4/programmer-sketches/`, and none became a test. |
| 2. Port suites, and a second Write wave | not taken | Decision 0054 defers Port suites. No Write task ran. |
| 3. Interpreter round on whole programs | item FE, rounds 15 to 17 | Done for the listed targets. Generic lambdas, aligned allocation, the tuple protocol, virtual bases, and the two W15 false rejects now run. |
| 4. A header-free prelude | not taken as a file | The language-support page now shows each shape in full (the ordering classes with every comparison, the placement forms, the exception classes). No test compiles the shapes together (section 1.5). |
| 5. Guides | not taken | Decision 0054 defers guides. |
| 6. Diagnose batches, and Predict for 10 pages | item PR | Predict done: 839 of 840, findings RPA to RPE (46). Diagnose not taken. |
| 7. The program's boundary | partly, in front-end round 17 | The interpreter links a C name by its spelling across units. A C library function with no definition still stops a run (`puts`, `_Exit`, `malloc`, `free`). |
| 8. Q-0322 with EDG evidence, and `typeid` | not taken | Q-0322 still says "EDG has not been probed". `typeid` appears 11 times on 5 pages and has no rule. |
| 9. Records that drift | partly, item R1 | The front-end row of `exercises.md` and the roadmap rows are current. Other records still drift (section 1.6). |

Round 4 process changes 2, 5, and 6 happened. The matrix runs each program on EDG, the round 4 sketches are kept, and each front-end round reruns the programs. Changes 3 and 4 did not happen. Phase 7 ran 840 more Predict programs and no Write task, and no coverage table exists.

### 1.3 Program tests find compiler defects

The 67 program tests carry 6 overrides. Each override's note names a question.

| Program | Compiler | Question |
|---|---|---|
| W9 `visitor.cpp` | EDG 7.0 | Q-0355, settled: EDG calls the base's pure function in constant evaluation |
| W9 `memptr.cpp`, W9 `mixin.cpp` | EDG 7.0 | Q-0353: EDG loses the member that a converted member pointer names |
| W12 `compose.cpp` | EDG 7.0 | Q-0354, settled: EDG checks a call in a generic lambda too early |
| W2 `order.cpp` | GCC 14 | Q-0356, compiler defect |
| W6 `bindings.cpp` | Clang 18 | Q-0015, settled |

Four new questions came from 67 programs. Each of these programs is ordinary code (general knowledge).

### 1.4 Sketches

The round 4 sketches, rerun at `7d8c982`:

| Sketch | Round 4 | Now |
|---|---|---|
| `wordcount.cpp`, `bits.cpp` | result 0 | result 0 |
| `smallvec.cpp`, `lambda_generic.cpp` | unsupported (generic lambda) | result 0 |
| `aligned.cpp` | unsupported (no allocation function) | result 0 |
| `condtriv2.cpp` | wrong reject (SA-1) | result 0 |
| `condtriv.cpp` | not listed | unsupported: "static_assert: concept" |
| `hello.cpp` | unsupported (`puts`) | unsupported (`puts`) |
| `typeid.cpp` | unsupported | unsupported |

The new sketches:

| Sketch | GCC, Clang, sanitizers | EDG 7.0 | `tidal-fe run` |
|---|---|---|---|
| `evaluator.cpp` (154 lines): tokenizer, parser, a tree of owned nodes, virtual `eval`, errors as exceptions | exit 0, clean | accepted | unsupported: "conversion function" |
| `evaluator_no_bool.cpp`: the same with the converting constructor template removed, `make` returning the base owner, and `explicit operator bool` removed | exit 0, clean | not run | result 0 |
| `hashmap_first_try.cpp`: an open-addressing map in raw storage with a key concept | both reject | not run | rejected (CPT-34) |
| `hashmap.cpp`: the map after three fixes | exit 0, clean | accepted | false reject (TR-12) |
| `hashmap_cast.cpp`: the same with the member definitions in the class | exit 0, clean | not run | undefined (STG-23), which is correct |
| `hashmap_kept_pointer.cpp`: the same, keeping the pointers that placement `new` returns | exit 0, clean | not run | false undefined (OBJ-2) |
| `prelude.cpp` (149 lines): every shape from the language-support page in one program | exit 0, clean | accepted | false reject (NAM-8) |
| `prelude_qualified.cpp`: the same with `std::size_t` in the specializations | exit 0 | not run | result 0 |

I reduced each stop to a short program. Each reduction builds and exits 0 on GCC and Clang with both sanitizers, and EDG accepts each one.

| Reduction | Lines | `tidal-fe run` | What the specification says |
|---|---|---|---|
| `two_byte_rows.cpp`: two `int` objects made by placement `new` in the two rows of `unsigned char key[2][4]` | 7 | undefined, OBJ-2: the first `int` "has ended" | STG-4: the smaller byte array (each row) hosts each object. STG-6: the second object takes no storage of the first. |
| `template_member_alias.cpp`: a member alias of a class template, named without qualification in a member function defined outside the class | 3 | rejected, TR-12 | NAM-21: after `M<T>::put`, the declaration sees the class members. The non-template form passes. |
| `move_assign_only.cpp`: a class that declares only a move assignment, assigned from a prvalue | 2 | unsupported: "class assignment" | an implicitly deleted copy assignment and a usable move assignment. With an explicit `= delete` on the copy assignment, the program runs. |
| `owner_convert.cpp`: an owner of a derived object returned as an owner of its base, through a converting constructor template | 12 | unsupported: "conversion function" | an ordinary converting constructor |
| `prelude.cpp`, line 110: `template <> struct std::tuple_size<Range> { static constexpr size_t value = 2; };` | 1 | rejected, NAM-8 | NAM-21. The line is the language-support page's own example. |

The `explicit operator bool` stop in `evaluator.cpp` did not reduce. A short owner template with the same conversion function runs. In the evaluator without the converting constructor template, removing only that line lets the whole program run to result 0.

My first hash map had three mistakes, and Tidal has a rule for each one. CPT-34 rejects a type after `->` in a compound requirement. DEP-10 says a dependent call with an `int` argument does not find a function declared after the template. DEP-26 rejects `this->template each(...)` with no argument list. GCC reported only the second. Clang reported the first and the third. I wrote these from memory and did not look first. The rules were easy to find once a compiler named the construct.

The `hashmap_cast.cpp` stop is correct, and it matters most to a programmer. The map reads its keys through `reinterpret_cast<K*>(key)` on a byte array. STG-29 says such a cast reaches a byte and does not reach the object that placement `new` made there. GCC, Clang, both sanitizers, and EDG are all silent. Writing raw storage this way is common (general knowledge). The casts page uses `reinterpret_cast` 15 times and does not link STG-29, so a programmer who starts from the cast does not find the rule.

### 1.5 Page examples

The specification has 181 `cpp` example blocks with 3005 lines. No tool compiles them. Alone, 51 blocks compile on both GCC and Clang. The other 130 need context from the page. `tidal-fe check` agrees on 49 of the 51. One of the two others is deliberate (`templates/declarations.md`, a TPL-38 reject that EDG also gives, Q-0197). The other is the tuple protocol example of `language-support.md`, which the front end rejects with NAM-8 and then ACC-2. No test copies that example. Tests write `decltype(sizeof 0)` in the specialization, and the front end accepts that form.

### 1.6 Records that drift

- `documentation/roadmap.md`, Deferred, still lists S and T. Phase 6 item O did the class property lists.
- `language-support.md` line 596 still says `typeid` waits "for roadmap item T".
- Q-0322 still says "EDG has not been probed". Round 4 recorded the EDG 7.0 results.
- `exercises.md` section 7 still describes a coverage table that does not exist.
- `exercises.md` section 6 still lists the order of the first exercises ("W8 and W1", then Diagnose, then Port). Every Write suite is done, and Diagnose and Port have waited since phase 5.

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Tests from the reductions, and a front-end round that fixes them, false reports first | 1 false undefined verdict, 2 false rejects, 3 run stops in 405 lines; 4 false rejects and 1 wrong result in `conformance.tsv` | small for the tests, medium for the front end | low | trust in the interpreter's verdict, which decision 0048 promises |
| 2 | A second Write wave of larger programs | phase 7 ran no Write task; 3 sketches found 7 faults, 840 Predict programs found 46 | one session per task | low | findings at the rate of use; the 5 pages no program reaches |
| 3 | A tool that checks the page examples | 181 blocks, none checked; 1 false reject among the 51 that compile alone | small | low | examples that a reader can copy with confidence |
| 4 | Port suites P1 to P8 | deferred since phase 5; "outside knowledge" and "needs example" findings in every Write suite | one session per suite | low | the idioms a programmer brings from another language |
| 5 | Guides, first a container in raw storage | the STG-29 trap that no compiler or sanitizer reports; the casts page does not link STG-29 | one session per guide, and a decision on decision 0046 | low | a way into the specification by task |
| 6 | Diagnose batches | 4 pages since phase 5; 2916 tests carry error patterns | one session per 4 pages | low | a check that a compiler message leads to a rule |
| 7 | The program's boundary in the interpreter | `_Exit` stops 4 tests, `malloc` 2, `free` 1; `puts` stops `hello.cpp` | small | low to medium (scope) | a first program that prints, and tests that end with `_Exit` |
| 8 | Q-0322 with EDG facts, and `typeid` | Q-0322 out of date; `typeid` has no rule | small, then about 5 rules | medium | a decision on facts |
| 9 | Records that drift | section 1.6 | tiny | none | trust in the roadmap and catalog |

### Item 1. Tests from the reductions, and a front-end round

**What it is.** A clean-room writer adds the five reductions of section 1.4 as tests under the rules they exercise:

- `two_byte_rows.cpp` under STG-4 and STG-6, with a form that uses an array of structures with a byte-array member,
- `template_member_alias.cpp` under NAM-21, for the parameter list and the body, since every NAM-21 test uses a class that is not a template,
- `move_assign_only.cpp` under the rules for implicitly deleted copy assignment,
- `owner_convert.cpp` under the rules for converting constructors and copy initialization in a `return`,
- the tuple protocol example of `language-support.md` under LSUP-3, exactly as the page writes it.

Then a front-end round takes these tests, the 4 false rejects and the 1 wrong result in `conformance.tsv`, the `explicit operator bool` stop in `evaluator.cpp`, and `condtriv.cpp`.

**Evidence.** Section 1.4. The worst of the faults is the OBJ-2 verdict on `two_byte_rows.cpp`. A programmer who runs a correct program and reads "undefined" will change correct code or stop trusting the tool. Move-only owners, owners that convert to a base owner, and member aliases used in out-of-class definitions are everyday code (general knowledge). The 6337 tests do not cover these shapes, and the 67 program tests do not use them.

**What it unblocks.** A reader can trust both answers of the interpreter, "result 0" and "undefined". Each fault also gets a test that stays in the suite.

**Cost.** Small for the tests. Medium for the front end. Byte-array hosting and class assignment are central parts of the interpreter.

**Risk.** Low. Each reduction builds clean on GCC and Clang with sanitizers, and EDG accepts each one.

### Item 2. A second Write wave of larger programs

**What it is.** Four to six Write tasks of about 300 to 500 lines each. Each task combines many pages and becomes a program test.

- An expression evaluator with a tokenizer, a parser, a tree of owned nodes, and exceptions (the shape of `evaluator.cpp`).
- A generic hash map in raw storage with a concept for keys and member functions defined outside the class (the shape of `hashmap.cpp`). This task reaches `dependent-names.md` and `instantiation.md`.
- A multi-unit program with a template library in headers and an explicit instantiation in one unit.
- A small text-processing program that uses `if` with initializers, `switch`, and type aliases, so that `if.md`, `type-names.md`, and `keywords.md` get a program citation.

Each task asks the reader to keep a short reduction of every compiler or interpreter stop.

**Evidence.** Phase 7 ran no Write task. My three sketches (405 lines) found 7 faults. The 840 phase 7 Predict programs found 46 findings, about 5.5 per 100. The 67 phase 6 programs were small, and every one runs now. Five pages have no rule cited by a program test: `if.md`, `type-names.md`, `keywords.md`, `dependent-names.md`, and `instantiation.md`.

**What it unblocks.** New findings at the rate that use gives. Targets for the next front-end round.

**Cost.** One session per task.

**Risk.** Low. Larger programs take longer to reduce when a stop happens. The reduction step in the prompt handles this.

### Item 3. A tool that checks the page examples

**What it is.** A tool that extracts each `cpp` block from `specification/`, compiles the blocks that stand alone on the matrix, and runs `tidal-fe check` on them. A block that needs context names it with a marker, such as the block it continues. A block that shows a reject says so in a marker, and the tool expects the reject. `tools/check_all.sh` runs the tool.

**Evidence.** Section 1.5. Of the 51 blocks that compile alone, the front end falsely rejects one, the tuple protocol example. No test copies that block. A programmer copies examples first (general knowledge), and the language-support examples are the shapes that every header-free program copies (decision 0041). The tool also covers most of round 4 item 4 (the prelude). The page's blocks are the prelude, and the tool tests them.

**What it unblocks.** Examples that compile as written, on every compiler and the front end.

**Cost.** Small. A script like mine, plus markers on the 130 blocks that need context.

**Risk.** Low. The markers add a little to each page.

### Item 4. Port suites P1 to P8

**What it is.** The eight Port suites that `exercises.md` section 3 promises, one per persona. Each suite gives three short programs in the persona's language, written fresh by the catalog author, and asks for a C++ program with the same behavior.

**Evidence.** Port has waited since phase 5. The Write reports filed 32 "needs example" findings and 11 "outside knowledge" findings, which point at the step Port tests: finding the C++ form of a familiar idiom. My `evaluator.cpp` is such a port in practice. An owner of a node converting to an owner of its base is the C++ form of a common pattern in other languages (general knowledge), and the interpreter stops on it.

**What it unblocks.** Findings on idioms, and more program tests.

**Cost.** One session per suite.

**Risk.** Low. Port prompts must not copy known example programs (general knowledge).

### Item 5. Guides, first a container in raw storage

**What it is.** Three how-to guides, each about the size of `guides/resource-owner.md`:

- a container in raw storage: the byte array (STG-4), placement `new` and its kept pointer (STG-5, STG-29, LSUP-7), destruction (STG-6), and the trap of a cast of the array's address,
- a move-only owner: the deleted copy, the move constructor and move assignment, conversion to an owner of a base,
- a program across files with templates in headers.

Also a link from the `reinterpret_cast` section of `casts.md` to STG-29. Item 5 needs the maintainer's decision on publishing guides (decision 0046).

**Evidence.** `hashmap_cast.cpp` reads through `reinterpret_cast<K*>` on a byte array. Tidal calls that undefined (STG-29, STG-23). GCC, Clang, both sanitizers, and EDG say nothing. Only the interpreter reports it. A programmer who learned C++ elsewhere writes this form (general knowledge). The fix costs one pointer per slot or a union, and only a guide shows the fix next to the trap. Round 3 and round 4 both asked for guides, and decision 0054 deferred them.

**What it unblocks.** A way into the specification by task. The STG-29 trap is the most costly fact a programmer can miss, because no tool in the matrix reports it.

**Cost.** One session per guide.

**Risk.** Low. A guide adds no facts (`guides/index.md`).

### Item 6. Diagnose batches

**What it is.** Diagnose batches for the chapters with the most error patterns, starting with templates, overloading, and initialization.

**Evidence.** Diagnose has reached 4 pages, all in phase 5. 2916 tests carry `tidal-error` patterns. In my hash map, each of the three mistakes reached me as a compiler message, and I then looked for the rule. A programmer meets the specification through a compiler message more often than through a test (general knowledge).

**What it unblocks.** A measure of whether a compiler message leads to the rule.

**Cost.** One session per 4 pages.

**Risk.** Low.

### Item 7. The program's boundary in the interpreter

**What it is.** A decision on a small closed set of C functions that the interpreter treats as known: `_Exit`, `abort`, `exit`, `malloc`, `free`, `puts`, and `putchar`. The specification stays silent on I/O. The list belongs to the front end (decision 0035), unless the maintainer wants it in the specification.

**Evidence.** In `conformance.tsv`, `_Exit` stops 4 run tests, `malloc` 2, and `free` 1. `hello.cpp` still stops on `puts`. Round 17 made the interpreter link C names by spelling, so a closed list now fits the existing model.

**What it unblocks.** A first program that prints. Seven run tests.

**Cost.** Small.

**Risk.** Low to medium. The list must stay closed, or the interpreter grows a C library.

### Item 8. Q-0322 with EDG facts, and `typeid`

**What it is.** Record the round 4 EDG 7.0 results in Q-0322. EDG accepts the three class-property builtins and the default-new-alignment macro, rejects the bit-copy and launder builtins, and rejects a self-declared `std::type_info`. Then specify the core behavior of the `typeid` expression with the result type left referenced.

**Evidence.** Q-0322 says EDG has not been probed. Pending row 2 asks the maintainer to decide Q-0322 on that text. `typeid` appears 11 times on 5 pages with no rule. Item 5's guide also needs Q-0322, because the launder operation is the other way to reach an object in a byte array (STG-29).

**What it unblocks.** A maintainer decision on facts. A rule page for an expression in the core grammar.

**Cost.** A question update, then about 5 rules.

**Risk.** Medium. Tests of `typeid` cannot link without the library, so each test rejects or uses an unevaluated operand.

### Item 9. Records that drift

**What it is.** Fix the records of section 1.6:

- remove S and T from the roadmap's Deferred section, and give `typeid` its own entry,
- point `language-support.md` at that entry,
- update Q-0322,
- replace `exercises.md` section 6 with the current order, and either build the section 7 table or remove the section.

**Cost.** Tiny. **Risk.** None.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| I/O and the general library | Keep out. Item 7 covers the boundary in the interpreter. | Programs print (general knowledge). | The library is large and outside the charter. The exit status serves every test. |
| `#include <new>` for the launder operation | Keep out until the maintainer decides Q-0322. | A raw-storage container written the common way needs it (general knowledge). | Keeping the placement pointer works, and item 5's guide can show it. |
| The preprocessor beyond `#include "file"` | Keep out. | `assert` is common (general knowledge). | High similarity risk. No program here needed it. |
| Threads and the memory model | Keep out. | | Large, and no program here needed them. |
| Coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out (Q-0007, pending row 7). | | |

## 4. Process changes

1. **Every phase runs at least one exercise by use.** Write, Port, or Guide. Phase 7 ran none, and the faults in section 1.4 waited for a roadmap reader to find them.
2. **Roadmap sketches become tests.** The integrator adds each correct sketch and each reduction to `tests/` in the commit that acts on the round. The round 4 sketches stayed in `reports/`, and `condtriv.cpp` still stops with no test to track it.
3. **Each interpreter stop gets a reduction.** A report that names a stop also gives a program of at most about 15 lines that shows it. Every stop in section 1.4 reduced except one, and the reductions show which rule each fault concerns.
4. **Check the page examples** (item 3) in `tools/check_all.sh`.
5. **Rerun the roadmap sketches at the end of each front-end round,** beside the program tests.
6. **Generate the coverage table** of `exercises.md` section 7, or remove the section. Round 4 asked for the table, and it does not exist.

## 5. Suggested order

Item 9 and the test part of item 1 come first, because both are small. Item 3 comes next, so that new examples are checked from then on. The front-end part of item 1 then runs beside item 2, and item 2's reductions feed the same round. Item 4 follows the first Write tasks. Item 5 can start once item 1 has fixed the byte-array verdict, so the guide's programs run in the interpreter. Items 7 and 8 are small decisions for the maintainer and can come at any time. Item 6 runs beside the others.

## 6. Doubts for the clean-room writers

- Whether the OBJ-2 verdict on `two_byte_rows.cpp` comes from the interpreter's model of storage or from a reading of STG-4 that I miss. I read STG-4 as saying that each row hosts its own `int`.
- Why `explicit operator bool` stops `evaluator.cpp`. A short owner template with the same member runs, and I could not reduce the stop.
- Whether `move_assign_only.cpp` stops because the interpreter does not model an implicitly deleted copy assignment in overload resolution, or for another reason. With the copy assignment written `= delete`, the program runs.
- Whether the page-example tool of item 3 should treat each block alone, or join the blocks of one page in order.

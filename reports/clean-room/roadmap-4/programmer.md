# Roadmap, round 4: what to do after phase 6, from a programmer's view

Date: 2026-11-03. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and understand programs. The reader read AGENTS.md, CLAUDE.md, the round 3 programmer report, the charter, roadmap, status, exercises catalog, and decisions 0033, 0039, 0044, 0050, and 0051. The reader also read parts of `language-support.md`, `storage.md`, `new-delete.md`, and `translation.md`, `questions/README.md` and Q-0322, the Write reports W1 to W14, one Predict report per chapter, front-end report 14, the item O inventory and gap review, and `git log --oneline 39e9322..HEAD` (79 commits). The reader did not read the other round 4 reports. All counts come from read-only `grep` and Python scripts on the tree at commit `3d98c93`.

The reader built the front end from `3d98c93` into the session scratchpad and ran all 67 Write programs (W0 to W15) through `tidal-fe run`. The reader also wrote nine sketches and built them with host GCC 16.2.1 and Clang 22.1.8 (`-std=c++23 -pedantic-errors -Wall -Wextra`). Some sketches also ran on the EDG 7.0 container (`cpfe --c++23 --strict`), and each one ran through `tidal-fe run`. The sketches are in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/rm4-programmer/sketches/`. No file in the repository was written. Statements that rest on general knowledge say so.

## 1. The state from a programmer's view

Phase 6 did most of what round 3 asked for. I can now write an ordinary program from the specification alone and expect it to build. Every Write suite has run. In each suite the programs built and ran clean under both compilers and both sanitizers. Most phase 6 Write findings were about finding things, and the two repair commits acted on all of them.

### 1.1 Counts

| Measure | Round 3 (2026-11-01) | Now (`3d98c93`) | Source |
|---|---|---|---|
| Rules | 1632 | 1678 | `:::{rule}` headers in `specification/` |
| Content pages | 85 | 85 | `specification/`, without index, glossary, and rules pages |
| Rules with one test | 297 | 3 | `:tests:` lines |
| Write suites run | 2 of 16 (W0, W15) | 16 of 16 (W0 to W15), 67 programs | `exercises.md` section 8 |
| Pages with a Predict batch | 22 | 75 of 85 | answers files under `reports/clean-room/R-*` |
| Pages with a Diagnose batch | 4 | 4 | `exercises.md` section 8 |
| Port suites defined | 0 of 8 | 0 of 8 | `exercises.md` section 3 |
| Guides | 5 | 5 (last change 2026-10-31) | `guides/` |
| Questions open for opinions | 289 | 310 (309 provisional, 1 open); 352 in all, 30 settled, 12 compiler defects | `questions/README.md` |
| *(referenced)* markers | not counted the same way | 29: library 17, extended floating-point 5, coroutines 3, preprocessor 2, memory model 1, threads 1 | text of `specification/`, whitespace joined |
| Front end | round 9, 942 of 1203 run tests | round 14, 5595 of 6032 tests pass, 112 run-unsupported | `status.md`, `conformance.tsv` |
| Write programs that run to a result in `tidal-fe` | not measured | 50 of 67 | this report's run |
| Rules cited by a Write report or program | not measured | 560 of 1678 | IDs in `reports/clean-room/W*/` and `java-programmer/` |

### 1.2 Round 3 items that phase 6 took and left

| Round 3 item | Phase 6 | Result |
|---|---|---|
| 1. Write suites W1 to W14 | item W | Done. 57 programs, 107 findings. |
| 2. Character values | item C | Done. PRIM-30 to PRIM-33 and a glossary entry "ordinary encoding". |
| 3. Operations that need the implementation | not taken | Decision 0051 keeps the launder operation, `typeid`, and the bit copy referenced (Q-0322). |
| 4. Class properties | item O | Done, observed through behavior only (CPY-19, CLS-19, CLS-29 to CLS-32, UNI-15, UNI-16). |
| 5. Predict batches | item P | Done for expressions, statements, declarations, and classes. 10 pages have none. |
| 6. Interpreter on whole programs | item FE, rounds 10 to 14 | Byte memory, parse failures as rejects, choice exploration. Whole-program gaps remain (1.4). |
| 7. Guides | not taken | No new guide. Decision 0046 still keeps guides unpublished. |
| 8. Write programs become run tests | in item W's text | Not done. No test under `tests/` comes from a Write program. |
| 9. Stale markers and counts | item R0 | Done for the counts. `tools/status_table.py` writes them, and `check_specification.py` fails when one is stale. Other records drift (1.6). |

### 1.3 Write finds problems that Predict no longer finds

- Phase 6 Predict batches scored 3262 of 3272 (99.7 percent) and filed 158 findings, about 5 per 100 programs.
- Phase 6 Write suites filed 107 findings from 57 programs, about 1.9 per program.
- The Write findings by kind: needs example 32, missing 28, hard to find 25, unclear 22, outside knowledge 11, wrong 5. About half are navigation (needs example, hard to find). About a quarter are real gaps (missing).
- Independent readers each found two problems twice. The ordering classes need comparisons with a literal 0 (W2-1, W7-1). `const auto` with the tuple protocol needs a `const` specialization (W5-1, W6-2). Both problems concern the shapes of language-support types that a header-free program writes itself.
- No Write rule citation reaches three pages: `keywords.md`, `templates/dependent-names.md`, and `templates/instantiation.md`.

### 1.4 The interpreter stops on a quarter of the Write programs

I ran the 67 Write programs at `3d98c93`. Fifty give result 0, some after exploring choices. Fifteen stop as unsupported, and two stop with a wrong reject.

| Stop | Programs |
|---|---|
| wrong reject | W15 `largest.cpp` ("CE-16: 'largest' is declared but not defined at this point"), W15 `optional.cpp` ("DEL-1: the copy constructor of 'Buffer' is deleted"). Both build and exit 0 on GCC and Clang. |
| `static_assert` on a construct the checker does not evaluate | W12 `compose.cpp`, W12 `sort.cpp` (object of unknown type), W3 `nested.cpp` (class assignment), W5 `two_values.cpp` (variable template), W6 `bindings.cpp` (range-based for), W13 `header.cpp` (bit-field layout), W0 `calc.cpp` (array of unknown bound) |
| allocation and layout | W4 `vector.cpp` (aligned allocation function called by name), W13 `aligned.cpp` (new expression), W8 `tagged.cpp` (class layout) |
| other constructs | W8 `unique.cpp` (overloaded `->`), W9 `mixin.cpp` (virtual base), W5 `deleted.cpp` (class temporary), W5 `recursion.cpp` (step limit), W13 `clink.cpp` (no definition for a C function) |

The 112 run-unsupported tests give a similar spread. The most common reasons are a `new` expression (12), a virtual base (9), a function with no definition (6), a constructor call (6), and a call of an allocation function by name (4 for `operator new`, 2 for `operator delete`).

### 1.5 Sketches

| Sketch | GCC, Clang | EDG 7.0 | `tidal-fe run` |
|---|---|---|---|
| `wordcount.cpp`: count words and sum numbers with `c - '0'` | exit 0 | not run | result 0 |
| `bits.cpp`: a float's bits copied through `unsigned char` at run time | exit 0 | not run | result 0 |
| `smallvec.cpp`: a constrained growable vector, sorted with a generic lambda | exit 0, sanitizers clean | not run | unsupported: "call needing template deduction" at the generic lambda |
| same, with a lambda that names its parameter types | exit 0 | not run | runs; 2 explored runs, result 0 |
| `lambda_generic.cpp`: a two-line generic lambda | exit 0 | not run | unsupported: "object of unknown type" |
| `aligned.cpp`: `::operator new` with `std::align_val_t` | exit 0 | not run | unsupported: no definition of the allocation function |
| `condtriv2.cpp`: union probe for a deleted destructor in a requires expression | exit 0 | not run | wrong reject (SA-1). Round 3 saw the same stop. |
| `hello.cpp`: `puts` declared under `extern "C"` | prints, exit 0 | accepted | unsupported: no definition for `puts` |
| `typeid.cpp`: self-declared `std::type_info` | both fail to link | rejected: the declaration conflicts with EDG's reserved class | unsupported: `typeid` |
| `edg_builtins.cpp`: builtin queries, bit copy, launder, the new-alignment macro | accepted | accepts `__is_trivially_destructible`, `__is_trivially_copyable`, `__is_standard_layout`, and the default-new-alignment macro; rejects `__builtin_bit_cast` and `__builtin_launder` | not run |

My first `smallvec.cpp` declared the placement form without a body. Clang linked it, and GCC failed to link it. LSUP-7 says to define the form `inline`, and I had not read LSUP-7 first. A programmer who writes these shapes from memory meets this kind of one-compiler failure.

### 1.6 Records that drift

- `language-support.md` near line 561 says `typeid` waits "for roadmap item T". The roadmap's Deferred section still lists S and T, though item O did the class property lists.
- `exercises.md` section 8 says the front end is at "round 10 done: 4918/5480 pass". `status.md` gives round 14 and 5595 of 6032.
- `exercises.md` section 7 describes a coverage table that does not exist.
- The roadmap's phase 6 table marks Q, C, O, and FE as done. R0, G, W, P, and D carry no state, and the "become run tests" part of W is not done.
- Round 3 process change 5 asked each roadmap round to keep its sketches. `reports/clean-room/roadmap-3/` holds only the five reports, so this round could not rerun the round 3 sketches.

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Write programs and sketches become run tests | 67 programs live only under `reports/`; the text of item W asks for it | small | low | a whole-program regression check on 8 compilers and the interpreter |
| 2 | Port suites P1 to P8, and a second Write wave of larger programs | Write yields 1.9 findings per program, Predict 0.05; Port has never been defined | one session per suite | low | findings on idioms and on the 3 pages Write never reached |
| 3 | Interpreter round 15 on whole programs | 17 of 67 Write programs stop (1.4); 2 wrong rejects; generic lambdas | large | medium | checking a program without a compiler (decision 0048) |
| 4 | A header-free prelude: the language-support shapes as one tested set | W2-1, W7-1, W5-1, W6-2 found twice; 61 tests declare `strong_ordering`; 25 of 62 Write files open `namespace std`; my placement-form slip | small | low | every later Write and Port program, and the guides |
| 5 | Guides: four how-to guides and a decision on publishing them | 57 of 107 Write findings are navigation; no guide since 2026-10-31 | one session per guide | low | the everyday path through templates, raw storage, and multi-file programs |
| 6 | Diagnose batches, and Predict for the 10 pages with none | Diagnose reached 4 pages; 2798 tests carry error patterns; overload resolution has no Predict batch | one session per 4 pages | low | a check that a programmer can go from a compiler message to a rule |
| 7 | The program's boundary: output and C functions | `puts` stops the interpreter; TR-P5 counts output as observable and marks it library referenced | small | low to medium (scope) | a reader's first program in the interpreter |
| 8 | Q-0322 with EDG evidence, and the `typeid` expression | EDG rejects the bit-copy and launder builtins and a self-declared `type_info` | small decision, then about 5 rules | medium | a choice the maintainer can make on facts |
| 9 | Records that drift | section 1.6 | tiny | none | trust in the catalog and roadmap |

### Item 1. Write programs and sketches become run tests

**What it is.** Each Write program goes under `tests/` as a run test that cites the rules its report lists before writing. W14's multi-unit programs use the multi-file form of decision 0014. Each program then runs on the eight baseline compilers and in `tools/frontend_check.py`. The round 4 roadmap sketches go in too, where they are correct programs.

**Evidence.** 67 programs, none under `tests/`. Decision 0051 item W says each Write program "becomes a run test", and the roadmap row W does not say done. Phase 6 changed many rules that these programs use (STG-28, LSUP-12, SB-5, CE-10). No check reran the programs. My run found two W15 programs that the front end now rejects wrongly, and no check had reported them.

**What it unblocks.** A regression signal at the level of a whole program, and a ready list of targets for item 3.

**Cost.** Small. The programs exist. Each needs `tidal-` headers and a rule list.

**Risk.** Low. The programs are clean-room work. Some use `-Wall` behavior that differs across the matrix, and EDG has run only W13's programs. A few programs will need overrides.

### Item 2. Port suites, and a second Write wave of larger programs

**What it is.** The item has two parts.

- Define and run the eight Port suites that `exercises.md` section 3 promises, one per persona. Each gives three short programs in the persona's language and asks for a C++ program with the same behavior.
- Run a second Write wave with fewer, larger tasks. Each task is about 300 to 500 lines and spreads across many pages. Examples are an expression evaluator with a tokenizer, a parser, and a tree of owned nodes; a small hash map with a generic interface; and a multi-unit program with a template library in headers. One task in the wave should need `templates/dependent-names.md` and `templates/instantiation.md`, which no Write program has cited.

**Evidence.** Section 1.3. Write finds about 40 times as many problems per program as Predict does. Port suites test the step that W findings keep pointing at, which is how a programmer from another language finds the C++ form of an idiom ("outside knowledge" 11 times, "needs example" 32 times). Phase 6 Write tasks were small, and each touched a few pages. Larger programs combine rules from many pages, and the remaining gaps are likely to sit there (general knowledge).

**What it unblocks.** New findings at the current rate, and the evidence for items 4 and 5.

**Cost.** One session per suite: eight Port suites and three or four larger tasks.

**Risk.** Low. Port prompts must avoid copying known example programs (general knowledge), so the catalog author should write each source program fresh.

### Item 3. Interpreter round 15 on whole programs

**What it is.** A front-end round whose targets come from programs:

- the two wrong rejects in W15 (`largest.cpp`, `optional.cpp`) and the union probe in `condtriv2.cpp`,
- generic lambdas at run time,
- calls of allocation and deallocation functions by name, aligned forms included,
- `static_assert` on class assignment, range-based for, variable templates, and generic callables,
- overloaded `->`, virtual bases, and class temporaries,
- layout for bit-fields and unions where a page gives it (BF-9, CLS-29 to CLS-32).

**Evidence.** Sections 1.4 and 1.5. Generic lambdas and allocation by name were on the round 3 list, and both still stop.

**What it unblocks.** A reader can check most programs and their undefined behavior without a compiler, as decision 0048 promises.

**Cost.** Large.

**Risk.** Medium. Item 1 should come first, so that each fix is checked against the programs.

### Item 4. A header-free prelude

**What it is.** One file of the declarations that a program with no header writes for itself:

- the type of `sizeof`, `std::align_val_t`, and `std::nothrow_t`,
- the placement forms,
- the ordering classes with their 0 comparisons and conversions,
- `std::initializer_list` and the tuple protocol templates,
- `std::byte` and `std::construct_at`,
- the exception classes.

Each part cites its LSUP rule. The file lives under `tests/` as a support header, like `placement.h` and `construct.h` today. One run test includes all of it. The language-support page links the file as its complete example.

The prelude only gathers shapes that decision 0041 already allows. It adds no names and no builtins.

**Evidence.**

- Independent readers each found two problems twice (W2-1 with W7-1, W5-1 with W6-2), and both problems concern these shapes.
- 61 tests declare `strong_ordering` themselves. 25 of the 62 Write source files open `namespace std`.
- Three support headers already exist (`placement.h` in 25 tests, `construct.h` in 8, `byte.h` in 3).
- My placement form without `inline` linked on Clang and failed on GCC.

**What it unblocks.** Write and Port programs that start from a correct base, shorter tests, and guides that include one file.

**Cost.** Small.

**Risk.** Low. A test that includes the whole prelude can hide a rule that only part of it exercises, so the existing per-rule tests stay.

### Item 5. Guides

**What it is.** Four how-to guides, each about the size of `guides/resource-owner.md`:

- a container in raw storage (STG-28, LSUP-7, NEW-40, the exception-safe growth of W8),
- a value type with comparisons (DCMP, LSUP-12, LSUP-13, the prelude of item 4),
- a program across files with templates in headers (W14-1 found that `#include` sits on `odr.md` and `translation.md` does not link to it),
- the program's boundary (item 7).

Item 5 also needs the maintainer's decision on publishing guides (decision 0046).

**Evidence.** Section 1.3. Phase 6 repairs added many examples to pages, so each idiom now has a home. A programmer still has to know which page holds it. Round 3 item 7 asked for the same guides, and phase 6 did not schedule them.

**What it unblocks.** A way into the specification by task. Each Write persona reached for one.

**Cost.** One session per guide.

**Risk.** Low. A guide adds no facts (`guides/index.md`), so a guide that needs a new fact files a finding.

### Item 6. Diagnose batches, and Predict for the 10 pages with none

**What it is.**

- Diagnose batches for the chapters with the most error patterns, starting with templates, overloading, and initialization. A programmer meets the specification most often through a compiler error (general knowledge).
- Predict batches for the 10 pages with none: `overload-resolution.md`, `conversion-sequences.md`, `storage.md`, `language-support.md`, `types.md`, `odr.md`, `keywords.md`, `handlers.md`, `throw.md`, and `unwinding.md`.

**Evidence.** Diagnose has reached 4 pages, all in phase 5. 2798 tests carry `tidal-error` patterns. Overload resolution is where a programmer's guess most often differs from the compiler's choice (general knowledge), and it has had no Predict batch.

**What it unblocks.** A measure of whether a programmer can find a rule ID from a compiler message, and use by a reader of the last unused pages.

**Cost.** About 3 Predict sessions and 6 to 8 Diagnose sessions.

**Risk.** Low. Predict scores are near 100 percent, so these batches should come after items 1 to 4.

### Item 7. The program's boundary

**What it is.** Two small decisions and one note.

- Decide whether the interpreter treats a small closed set of C functions declared under `extern "C"` (`puts`, `putchar`, `abort`, `exit`) as known, so a reader sees output. The specification itself stays silent on I/O.
- Decide whether TR-P5 says that a program writes data only through functions outside Tidal, with a link to the boundary guide.
- Add a note on `translation.md` that names `main`, `argv`, the exit status, and the termination of UNW-10 as all that Tidal says about the outside of a program.

**Evidence.** `hello.cpp` and W13's `clink.cpp` stop in the interpreter. The glossary entry for observable behavior marks "the data that the program writes out" as library referenced. Round 3 raised the same point.

**What it unblocks.** A first program that prints, run under the interpreter.

**Cost.** Small.

**Risk.** Low to medium. The stub list must stay closed, or the interpreter grows a C library.

### Item 8. Q-0322 with EDG evidence, and the `typeid` expression

**What it is.** Record the EDG results in Q-0322 and ask the maintainer to decide. Then specify the core behavior of the `typeid` expression, with the `type_info` comparison left referenced.

**Evidence.** Q-0322 says "EDG has not been probed". My probe on EDG 7.0 in strict mode gives these results:

| Operation | EDG 7.0 strict |
|---|---|
| `__is_trivially_destructible`, `__is_trivially_copyable`, `__is_standard_layout` | accepted |
| the default-new-alignment macro | accepted |
| `__builtin_bit_cast` | rejected, unknown identifier |
| `__builtin_launder` | rejected, unknown identifier |
| a self-declared `std::type_info` | rejected, conflicts with a reserved class |

So option 2 of Q-0322 (a support header built on shared builtins) works for the class property queries and fails for the bit copy and the launder operation. EDG may accept the two builtins in another mode (general knowledge). I did not probe another mode.

For a programmer the current choice costs little. STG-P4 keeps the placement pointer, so code rarely needs the launder operation. `bits.cpp` copies bits at run time with no undefined step. The bit copy in constant evaluation has no workaround. `typeid` has no rule at all. Five pages name it, and the language-support page points to a deferred item that phase 6 did not take.

**What it unblocks.** A decision on facts, and a rule page for an expression in the core grammar.

**Cost.** A question update, then about 5 rules for `typeid`: operand evaluation, polymorphic and other operands, the null case, and the result's type left referenced.

**Risk.** Medium. Tests of `typeid` cannot link without the library, so each test is a reject test or uses an unevaluated operand. EDG rejects a self-declared `type_info`, so a test needs an EDG override or no `type_info` declaration at all.

### Item 9. Records that drift

**What it is.** Fix the items in section 1.6:

- mark the state of each phase 6 row on the roadmap,
- remove S and T from Deferred, and move `typeid` to its own entry,
- point `language-support.md` at that entry,
- have a tool write the front-end row of `exercises.md` section 8, as R0 did for `status.md`.

**Cost.** Tiny. **Risk.** None.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| I/O and the general library | Keep out. Item 7 covers the boundary. | Programs print (general knowledge). | The library is large and outside the charter. The exit status serves every test. |
| The preprocessor beyond `#include "file"` | Keep out. | `assert` and conditional compilation are common (general knowledge). | High similarity risk. No Write program needed it. |
| Threads and the memory model | Keep out. | Newer programs use threads (general knowledge). | Large, and outcomes vary across the matrix. No Write program needed them. |
| Coroutines, modules, extended floating-point types | Keep out. | none from the Write suites or my sketches | as the charter says |
| Builtins in tests | Keep out, except as Evidence in probes (decision 0051). | | Item 8's table shows that EDG lacks two of them. |
| Language versions after C++23 | Keep out (Q-0007). | | |

## 4. Process changes

1. **Write programs are tests.** Every correct Write, Port, and roadmap program goes under `tests/` in the commit that acts on its report (item 1).
2. **Run each Write program on EDG.** Only W13 did. An EDG rejection of a program that GCC and Clang accept is a finding or an override.
3. **Shift exercise sessions from Predict to Write, Port, and Diagnose.** In phase 6, Predict found about 5 problems per 100 programs and Write about 190. Round 3 asked for hold-out Predict programs, and no batch used them. With scores at 99.7 percent, more Predict batches add little.
4. **Generate the coverage table.** `exercises.md` section 7 describes a table of the exercises and findings per page. A tool can count rule IDs in reports and write the table. My script found that Write reports and programs cite 560 of 1678 rules.
5. **Keep the roadmap sketches.** Round 3 asked for this, and no directory holds the round 3 sketches. The integrator can copy this round's sketches from `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/rm4-programmer/sketches/` into `reports/clean-room/roadmap-4/`.
6. **Rerun all Write programs in the interpreter at the end of each front-end round,** and list the results in the round's report.

## 5. Suggested order

Start item 9 and item 1 at once. Item 4 comes next, so that item 2 starts from the prelude. Items 2 and 3 then run in parallel, and item 3 takes its targets from the tests of item 1. Items 7 and 8 are small decisions for the maintainer and can come at any time. Item 5 follows the first Port suites, and item 6 runs beside the others.

## 6. Doubts for the clean-room writers

- Whether EDG accepts `__builtin_bit_cast` and `__builtin_launder` in a mode other than `--strict`.
- Whether the W15 wrong rejects in `largest.cpp` and `optional.cpp` are new in rounds 10 to 14 or older. No earlier report ran these programs on the front end.
- Whether a prelude header (item 4) fits decision 0041, which speaks of shapes a program declares, or needs a decision of its own.
- Whether the interpreter's stub list for C functions (item 7) is a front-end choice under decision 0035 or a specification choice.

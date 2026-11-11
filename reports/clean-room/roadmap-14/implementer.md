# Roadmap, round 14: what to do after phase 16, from an implementer's view

Date: 2026-11-11. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone and uses the specification as the design input. The evidence is the two front ends (`frontend/src/` and `frontend/next/`), their reports, ledger, baseline, marks file, and takeover list in `reports/clean-room/rust-frontend/`, the design of the rewrite (`frontend/DESIGN-rewrite.md`), and the phase 16 exercise and review records. Commit read: `f77bbe05`.

Inputs:
- AGENTS.md, CLAUDE.md
- `reports/clean-room/roadmap-13/implementer.md`, the model for this report's form
- `documentation/`: status, roadmap (the phase 16 table), decisions 0060, 0063, and 0064, and `decisions/pending.md`
- `git log b73a7d2d..HEAD` with every commit body (17 commits)
- `frontend/DESIGN-current.md`, `frontend/DESIGN-rewrite.md`, and `frontend/TEMPLATE-MODEL.md`
- `reports/clean-room/rust-frontend/`: `report-rw1.md` and `report-27.md` in full, `rewrite-marks.tsv`, `takeover.tsv`, `ledger.md`, and `conformance.tsv` at `f77bbe05` (round 27) and `a91ccc77` (round 26)
- `reports/clean-room/X24/report.md` (the design headings, section 4, and the host-only claims)
- `reports/clean-room/ledger.md` (the 15 open rows)
- `reports/review/2026-11-11-gap-review-phase-16.md` and `2026-11-11-read-of-rv9-repair-diff.md` (the Result sections and the headings)
- `questions/`: the status and Waits for lines of the questions that failing tests and the G166 to G179 rows name
- `tests/`: the headers of every test, and the stop line of each parse disagreement
- `frontend/next/src/` (the layout, `syntax/mod.rs`, and `classes/members/syntax.rs`), `tools/frontend_check.py`, and `frontend/src/` by line counts only

I did not read the other round 14 reports, `inputs/`, or `build/exercise-keys/`. I ran no compiler, so this report makes no host-only claim.

**Measurements.** I ran `tools/frontend_check.py --next` once under the run lock. Its lines match the phase 16 close commit:

```
frontend: 6967/7073 pass, 6990/7073 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 0 new (0 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 74 tests fail their check, of which 36 need the template model (marks file) and 38 count toward the gate of decisions 0062 and 0063; 0/166 program tests do not run to their result; 9 tests out of scope
frontend: 2154/2782 passing rejects name a cited rule (1966) or a linked rule (188) in their first error; `tidal-fe conform` gives the table per page
next: unit: 88 tests pass
next: lex: 7191/7191 files give the old front end's tokens
next: parse: 6475/6913 tests parse as their outcome needs
next: parse: 159 of the 438 disagreements are reject tests that stop on the line of the old front end's first error
next: owned: 0 tests (takeover list)
```

The scripts and their output are in `reports/clean-room/roadmap-14/implementer-scratch/`. `check-next-output.txt` holds the tool's whole output, with the 438 disagreements. `parse_causes.py` reads the line of each stop from the test and sorts each disagreement into a G row by a pattern. It writes `parse-causes.tsv`. The patterns are my judgment, and they split G166 from G167 differently from report RW1 (section 1). `causes.py` sorts the 74 failing tests: the 36 marked ones by model section, and the 38 unmarked ones into cause groups from report 27 section 8, by my judgment. `step_scope.py` sorts every test by the build step whose pages hold every rule on its `tidal-rules:` line (`DESIGN-rewrite.md` section 3). `questions.py` lists the questions that the failing tests and the G rows name.

## 1. Where the specification and the front end stand

| Measure | Round 13 (7015 tests, `a6a2b16c`) | Now (7073 tests, `f77bbe05`) |
|---|---|---|
| Full pass | 6895 (98.3%) | 6967 (98.5%) |
| Check pass | 6926 | 6990 |
| Completion line (decision 0060) | 80 tests (32 marked, 48 unmarked), 5 of 152 program tests, 9 out of scope | 74 tests (36 marked, 38 unmarked), 0 of 166 program tests, 9 out of scope |
| `MISSED-REJECT` in scope | 38 | 38 (21 marked, 17 unmarked) |
| `unsupported` in scope | 30 | 26 (8 marked, 18 unmarked) |
| `FALSE-REJECT` in scope | 5 | 4 (3 marked, 1 unmarked) |
| `EDG-AGREES` (a failure) | 7 | 6 (4 marked, 2 unmarked) |
| `run-unsupported` | 31 | 23 |
| Passing rejects whose first error names a cited or linked rule | 2139 of 2766 (77%) | 2154 of 2782 (77%) |
| Rules | 1700 | 1700 (no new rule) |
| Questions open | 197 | 200 |
| Front-end findings (G) | G1 to G165, 0 open | G1 to G180, 14 open (G166 to G179) |
| Exercise ledger rows open | 16 | 15 (X24-1 to X24-8, RV10-R-1 to RV10-R-3, RV10-R-5 to RV10-R-8) |
| Old front-end source | 52487 lines, 28 files | 52954 lines, 28 files; 7 over 2000 lines |
| New front-end source | none | 10197 lines, 208 files; the longest is `expressions/primary/lexical.rs` (857) |

### What phase 16 took from round 13

| Round 13 item | Phase 16 result |
|---|---|
| 1. The rewrite, steps 1 and 2 | Done in RW (`8f487785`). Every measure of the item holds except the parse count, which ends at 6475 of 6913 with a cause for each disagreement. G166 to G179 hold the 14 grammar gaps. |
| 2. Front-end round 27 on the old code | Done (`aa8c8e60`). Every program test runs. P3-9, P3-10, P4-1, and P4-2 pass. The round marked the three candidates of item 1 and one RV9 test. |
| 3. Text for the P4 and X25 rows | Done in F6 (`b3f17197`). |
| 4. Gap review RV9, with the read of its repair diff | Done (`b310bf11`, `8d33fcb1`, `b24e224f`). The read opened RV10-R-1 to RV10-R-8. |
| 5. Implement I8 on instantiation | Not taken. Decision 0064 defers it. |

Of round 13's process changes, decision 0064 took change 2 (the gate is a measure) and change 4 (two copies change together). Round 27 used change 4 once: the G180 lexer change is in both crates (`aa8c8e60`). Change 7 holds in part: report 27 keeps report 26's form, and its `next:` lines are in the commit message only. Change 5 holds for `DESIGN-current.md` (round 27, 52954 lines) and `TEMPLATE-MODEL.md` section 6 (36 tests). Change 1 is not in place. `DESIGN-rewrite.md` sections 4 and 5 still give 32 tests, and its build table still gives 17, 11, and 4 by step, where the marks file now gives 17, 13, and 6. Change 3 holds in part: the T5, F6, and RV9 commits name item FE for each failing test, and none says whether a test is marked. Change 6 (a first-error target) is still an open point of `DESIGN-rewrite.md` section 10.

### The 74 tests of the completion line

| Measure | Round 24 | Round 25 | Round 26 | Round 27 |
|---|---|---|---|---|
| In-scope tests that fail their check | 127 | 104 | 77 | 74 |
| Of these, marked for the template model | 16 (by report) | 30 (by report) | 32 | 36 |
| Of these, unmarked | | 74 | 45 | 38 |
| New tests in the round, and how many fail their check at its end | 92, 10 | 60, 6 | 69, 3 | 66, 1 (marked) |

All 38 unmarked tests failed at round 26 with the same cause. No test that passed its check at round 26 fails now. Of round 13's 48 unmarked tests, 3 are now marked, and 7 pass.

By chapter:

| Chapter | Marked | Unmarked | All |
|---|---|---|---|
| templates | 34 | 20 | 54 |
| expressions | 0 | 5 | 5 |
| classes | 1 | 4 | 5 |
| foundations | 0 | 4 | 4 |
| overloading | 0 | 3 | 3 |
| declarations | 0 | 2 | 2 |
| statements | 1 | 0 | 1 |
| **total** | 36 | 38 | 74 |

Report 27 section 8 says that the 38 are "19 missed rejects, 16 unsupported". Its own table and `conformance.tsv` give 17 missed rejects and 18 unsupported.

### The causes

The 36 marked tests, by the section of `TEMPLATE-MODEL.md` that the marks file names, and the build step of `DESIGN-rewrite.md` section 5 that takes them over:

| Model section | Step | Tests | Status now | Rules |
|---|---|---|---|---|
| 1, dependence marks | 5 | 4 | 1 missed reject, 1 unsupported, 2 EDG-AGREES | DEP-15 with DEP-30 or DEP-1 (2), DEP-4 (2) |
| 2, the current instantiation | 5 | 6 | 3 false rejects, 2 missed rejects, 1 unsupported | DEP-24 (4), DEP-11, CPY-15 |
| 5, checks at the definition | 5 | 7 | 7 missed rejects | CNS-29 (2), DEP-37 (2), PACK-8 (2), IF-22 |
| 3, a specialization | 6 | 13 | 8 missed rejects, 3 unsupported, 2 EDG-AGREES | the points of INST-30 to INST-33 (6), INST-5 (2), INST-12 (2), INST-15, TSP-31, PACK-26 |
| 4, the immediate context | 7 | 6 | 3 missed rejects, 3 unsupported | DED-24 (2), DED-29 (2), TARG-24, DEP-20 |

So step 5 takes 17, step 6 takes 13, and step 7 takes 6.

The 38 unmarked tests (`causes.py`):

| Cause | Tests | Rules | Where the old code is |
|---|---|---|---|
| Template checks outside the marks (missed rejects) | 8 | TARG-15, TARG-7, CPT-37, CNS-13, TPL-31, TPL-7, ORD-3, TSP-23 | `templ.rs`, `cons.rs` (rewritten) |
| Template constructs outside the marks (unsupported) | 6 | TARG-14, CPT-1, CPT-20, CNS-30, PACK-3, PACK-5 | `templ.rs`, `cons.rs` (rewritten) |
| Class template argument deduction and constant parameters of class template type | 6 | CTAD-1 sites (2), CTAD-1 with TPL-11 (2), OPR-8 with TPL-11, CTAD-19 with CTAD-23 | `ctad.rs`, `templ.rs` (rewritten) |
| Constant evaluation of a construct (unsupported) | 4 | PRIM-27, CE-3, LSUP-13, OVL-40 | `consteval.rs`, `interp/` (moved) |
| Constant evaluation and immediate functions (missed rejects) | 3 | CTOR-12, CE-3, CEV-1 | `consteval.rs`, `interp/` (moved); CEV-1 in the checks |
| Missed rejects in other chapters | 5 | DCMP-11, UNEV-1, PFX-11, NAM-8, OVL-20 | the checks (rewritten) |
| Unsupported constructs in other chapters | 3 | LAM-6, LAM-18, ICS-47 | the checks (rewritten) |
| EDG-AGREES | 2 | CPY-27, CNS-32 | `special.rs`, `cons.rs` (rewritten) |
| False reject | 1 | SB-5 | the checks (rewritten) |
| **total** | 38 | | |

Nine of the 74 tests rest on an open question. Seven are marked: Q-0427 (3, waits for a use of DEP-P5), Q-0432 (2, a use of DEP-P2), Q-0433 (1, a review), and Q-0441 (1, a review and newest families that agree). Two are unmarked: Q-0339 (CPY-27, newest families that agree) and Q-0406 (PFX-11, `pending.md` row 4).

### Run stops

23 tests pass their check and stop in a run, down from 31. No program test stops. By the build step whose pages hold their rules (`step_scope.py`): 5 sit in step 3's pages, 9 in step 4's, and 9 in the templates pages. They are report 25's groups: the conversion-function calls by name, the destructor function try blocks, the placement deallocation forms, OVL-42 (a class with a defaulted copy assignment), and DEP-20 (`type-only-positions.cpp`).

### Where the rewrite stands

- **The crate.** `frontend/next/` holds `source`, `diag`, `lex`, `syntax`, `sem::names`, the `syntax.rs` part of 62 pages with 257 nonterminals, and an empty `proposals` registry. `conform`, `target`, `machine`, and the typed parts of `sem` do not exist yet. `tidal-next check` gives `unsupported` for each unit that parses, and `conform` refuses.
- **The tree.** The parser makes one generic node type, `syntax::Node { prod, span, kids }`, where `DESIGN-rewrite.md` section 2 sketches typed syn nodes (`Decl | Stmt | Expr | ...`). Step 3's analysis reads one or the other, and the design does not yet say which.
- **Deferred parts of a class.** `DESIGN-rewrite.md` section 2 keeps a member function body, a default argument, and a default member initializer as token ranges. Section 10 and the code keep only the bodies, and parse default arguments and default member initializers in place. NAM-6, INST-30, and RV10-R-1 treat the class as complete in all three.
- **Moved files.** Step 1 moved `lexer.rs` and `diag.rs`, and the driver took the subcommands of `main.rs`. These three have two copies. `conform.rs` has not moved, though `DESIGN-rewrite.md` section 10 lists it among the copied files.
- **Stale counts.** `DESIGN-rewrite.md` section 5 lists the 32 tests of decision 0064, and its build table says the marked count goes from 32 to 15, 4, and 0. The marks file holds 36, so the counts are 36, 19, 6, and 0. `TEMPLATE-MODEL.md` section 8's RV9 row says RV9 has yet to read FN-12 and INST-1. RV9 is done, and its repairs changed INST-30, INST-1, INST-P4, and FN-12 again.

**The parse count by cause.** Report RW1 gives a cause for each of the 438 disagreements at `1cc1696e`. The count of disagreements has not moved since, and the 19 tests judged since then all parse as they need. `parse_causes.py` gives this split now. G166 and G167 together hold 137 tree-side tests by both counts, and the two counts split them differently.

| Cause | Need a tree (report RW1) | Need a tree (`parse_causes.py`) | Reject tests (`parse_causes.py`) |
|---|---|---|---|
| G166: a template-id cast with a braced list | 89 | 81 | 20 |
| G167: a base or member initializer as a template-id or `decltype` | 48 | 56 | 29 |
| G168: an unqualified using-directive or namespace alias | 36 | 36 | 23 |
| G169: `using enum` | 4 | 4 | 3 |
| G170: attribute positions | 5 | 5 | 2 |
| G171: a template-id after an operator or destructor name | 10 | 11 | 1 |
| G172: an opaque enumeration in a class | 4 | 4 (2 by hand) | 0 |
| G173: a member template with no decl-specifiers | 2 | 2 | 2 |
| G174: a structured binding in an init-statement | 2 | 2 | 1 |
| G175: `throw()` | 3 | 3 | 0 |
| G176: a `static_assert` message of two literals | 1 | 1 | 0 |
| G177: a pointer to member in `conversion_type` | 2 | 2 | 1 |
| G178: a braced template argument | 2 | 2 | 0 |
| G179: `template` and a name with no list | 3 | 3 (1 by hand) | 0 |
| G158, out of scope | 2 | 2 | 0 |
| The step 2 oracle (an alias of the current instantiation; a marked test of model section 1) | 2 | 2 | 0 |
| Reject tests whose form the grammar does not derive, or a tree where the old first error names TR-12 | 0 | 0 | 140 |
| **total** | 215 | 216 | 222 |

212 tree-side tests wait for a G row. 82 reject tests stop at a G form before they reach the form they test. The other 140 reject disagreements are not findings (report RW1 section 4): either the stop is right by TR-12, or another rule rejects a form the grammar derives.

**The tests by build step.** `step_scope.py` sorts each test by the pages that hold the rules on its `tidal-rules:` line:

| Every rule of the test sits on the pages of | Tests | The old front end passes its check | Program tests | Parse disagreements (tree side, reject side) |
|---|---|---|---|---|
| step 3 (foundations, expressions, statements, declarations) | 3791 | 3775 | 43 | 53, 106 |
| step 4 (and classes, overloading, exceptions) | 1873 | 1860 | 55 | 29, 19 |
| steps 5 to 7 (any templates page) | 1409 | 1355 | 68 | 134, 97 |

A rule's page does not bound the constructs a test uses. Of the 3791 step 3 tests, 1890 contain none of the keywords of a later step's pages (`class`, `struct`, `union`, `template`, `typename`, `concept`, `requires`, `throw`, `try`, `catch`, `operator`, `virtual`, `friend`, `this`, `new`, `delete`, `std`, and `#include`). The old front end passes its check on 1885 of them. 26 of the 1890 disagree in the parse on the tree side: G168 (22), G170 (3), and G176 (1).

Five of the 36 marked tests do not parse yet. Four stop at G166 or G167, and `sizeof-value-dependent-operand-no-typename.cpp` waits for model section 1 in the oracle. 38 program tests do not parse: G167 (18), G168 (8), G166 (6), G171 (3), G169 (2), and G174 (1).

### The X24 and RV10 findings

| Rows | Who acts | Why the front end cares |
|---|---|---|
| X24-1 (OBJ-3, JMP-9, LOOP-3, RFOR-4) | text, then the machine | The order in which a jump ends a statement's own variables (a `for` initializer, a condition, a range `for`'s hidden variables) has no rule. The machine picks an order today. X24's probe `d1.cpp` is a host-only claim. |
| X24-3 (JMP-14, DEP-30) | text, a question | A `goto` to a missing label in a template that is never instantiated. The compilers split. The answer is a row of model section 5's definition checks, or not. |
| X24-2, X24-4, X24-6, X24-7 (JMP-6, FN-7, JMP-7, JMP-14, SW-3, JMP-5) | text | The prototype of P22 (item 6) reads JMP-6, JMP-7, and JMP-14 for its checks. |
| X24-5, X24-8 | glossary, none | No front-end work follows. |
| RV10-R-1 (INST-30, INST-1, INST-P4, CLS-2) | text | The class is complete in a default template argument, a `noexcept` operand, and a default member initializer. The new parser parses two of these in place (section 1). Model section 3 holds the points of instantiation. |
| RV10-R-2 (FN-12) | text, a test | A nested class defined after its enclosing class with a qualified name. Both front ends defer the check to the outermost class. |
| RV10-R-3 (OBJ-2, Q-0447) | text | A write through the name of an ended scalar union member creates an object. Constant evaluation follows OBJ-2 today. |
| RV10-R-5 (CPT-28) | text | A doubt on the variable template and `if constexpr` forms of one test. |
| RV10-R-6, RV10-R-7, RV10-R-8 | text | Small records. |

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | The grammar gaps G166 to G179: Syntax text first, then the parser | 14 open G rows; 212 tree-side parse disagreements and 82 reject tests wait for them; 5 marked tests and 38 program tests do not parse | small to medium | none |
| 2 | The rewrite, step 3: the core, the machine, the harness, and the pages outside classes and templates, with a shadow line and the first takeover | steps 1 and 2 are done; 3791 tests sit on step 3's pages, and 1890 of them use no later construct; no step after step 2 can start without `sem` and the machine | large | item 1 for G168, G170, and G176; none for the core |
| 3 | Front-end round 28 on the old code: phase 17's tests, the machine and constant-evaluation groups before step 3 copies them, and the records | the old front end is still the front end of record; 7 unmarked tests and 23 run stops sit in code that step 3 copies | small to medium | the phase's text items for their tests |
| 4 | Text for the X24 and RV10 rows | 15 open rows; X24-1 and X24-3 reach the machine and model section 5 | small each | none |
| 5 | Gap review RV10 of the 10 changed rules and item 1's grammar, with the read of its repair diff | INST-30, INST-1, INST-P4, and FN-12 changed in the RV9 repairs, and model section 3 rests on them; item 1 changes 14 parts of the grammar | medium | items 1 and 4 for their text |
| 6 | The P22 prototype on the new front end | X24 gives the design; the nonterminal point exists; the `Ext` node point and `statements::jumps` come with step 3 | small | item 2's statements pages |

### 1. The grammar gaps G166 to G179: Syntax text first, then the parser

**What it is.** A clean-room writer answers each of the 14 open G rows with text in the Syntax segment of its page. Then the clean-room writer in the front-end role adds the parser function for each new or changed production. The tests exist already, because the parse count found each gap in them.

**The order.** The rows that the step 3 tests need come first, then the rows with the most tests:

| Row | Page | Tests (report RW1) | Needed by | Text to decide |
|---|---|---|---|---|
| G168 | `declarations/namespaces` | 59 | step 3 (22 of the 26 plain step 3 tests) | `using_directive` and `namespace_alias` take `[nested_name] identifier` |
| G170 | `declarations/attributes` | 7 | step 3 | the attribute positions after the decl-specifiers, before a parameter, and before a base; or a sentence that names the places Tidal leaves out |
| G176 | `declarations/static-assert` | 1 | step 3 | a message of adjacent literals |
| G169, G172, G174, G175 | `declarations/enumerations`, `classes/members`, `statements/if`, `declarations/exception-specifications` | 7, 4, 2, 3 | step 3 and step 4 | `using enum`; `enum_opaque` in `member_decl`; `sb_decl` in `init_stmt`; `throw()` with EXS-5. G175 follows Q-0077's provisional answer (`pending.md` row 4b) |
| G166 | `expressions/casts` | 111 | steps 4 and 5; 4 marked tests and 6 program tests | `cast_type` takes `[nested_name] template_id`, and the text says which rules read `Box<int>(1)` as a cast and which as a call |
| G167 | `classes/derived`, `classes/constructors` | 75 | steps 4 and 5; 18 program tests | `base_spec` and `mem_init_id` take a template-id, and `base_spec` takes a `decltype_spec` |
| G171, G173, G177 | `templates/arguments`, `classes/constructors`, `templates/declarations`, `classes/members`, `classes/conversion-functions` | 11, 2, 2 | step 4 | a template-id that names an operator function; a destructor name with a template-id or `decltype`; a member template with no decl-specifiers; a pointer to member in `conversion_type` |
| G178, G179 | `templates/arguments`, `templates/dependent-names` | 2, 3 | step 5 | `braced_init` as a template argument; a name after `template` with no list where DEP-26 allows it. G179 follows Q-0226's provisional answer (`pending.md` row 4) |

Each change keeps the style guide's rules for Syntax segments (section 8), including the `shape` and `tidal-grammar` marks. Where a new production only names a form that a general production would derive, the writer marks it `shape`, and the parser writes a recognizer.

**What the parser needs.** One function per new production, by the unit test `syntax::tests::each_nonterminal_has_one_function_in_its_page`. A changed alternative changes one function. No G row needs a new question of the oracle. G166 needs the oracle's answer "is this a template" before `<`, which the parser already asks.

**The measure that ends the item.**
- Each of G166 to G179 is `commit` in the front-end ledger, with its text commit and its parser commit.
- `next: parse` on the tree side has 4 disagreements left: the 2 tests of G158 (out of scope) and the 2 of the step 2 oracle, which step 5 answers.
- The 82 reject tests that stop at a G form now parse past it. Each gives a tree, or a stop on the line of the old front end's first error.
- `next: unit` passes, with a unit test for each new production.
- The old front end's line stays at "0 worse". No G row changes an old status, because the old parser reads each form already.

**Evidence.** Section 1, "The parse count by cause". The parse count is the only measure of steps 1 and 2 that did not reach its target, and each gap is a finding against the text (report RW1 section 10). Step 3's shadow line cannot judge a test that does not parse.

**Size.** Small to medium. The text is 14 small changes on 14 pages. G166 needs a sentence beside the grammar on the cast and call readings. The parser work is about 20 functions.

**Prerequisites.** None.

**Risk.** Low. A Syntax change can make a test parse that should stop. The 140 reject disagreements that are not findings show the change at once in the parse lines.

### 2. The rewrite, step 3

**What it is.** The clean-room writer in the implementer role builds step 3 of `DESIGN-rewrite.md` section 5 in `frontend/next/`, under decisions 0029, 0035, 0060, and 0064.

**Scope in phase 17.** Step 3 in full. Step 4 comes in phase 18.

| Part | Work | From the old front end |
|---|---|---|
| a | Settle the form of the tree that analysis reads: accessor functions in each page's `syntax.rs` over the generic `syntax::Node`, or typed syn nodes as section 2 sketches. Write the choice into section 2. Settle the deferred parts of a class, so that sections 2 and 10 and the code agree with NAM-6. | none |
| b | The node-check point of section 7, before any expression page | none |
| c | `sem`: the type table with `DepType`, entities and scopes in place of `sem::names`, the typed tree with marks and spans, `Subst`, the `Check` context, and `Unit` | `types.rs` (531 lines) and `ast.rs` (408), moved |
| d | `target`, `machine`, the run parts, and `conform` | `layout.rs` (434); `interp/mod.rs` (1502) and `interp/f80.rs` (360), moved; `interp/expr.rs` (4892), `interp/stmt.rs` (996), `interp/bytes.rs` (1184), and `interp/run.rs` (538), moved and split by page; `conform.rs` (730) |
| e | `foundations::constant_evaluation` | `consteval.rs` (310), moved |
| f | The `check.rs` and `run.rs` parts of the 58 pages of foundations, expressions, statements, and declarations, for constructs outside classes and templates | the checks in `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `stmt.rs`, and the lookup of `parser.rs`, rewritten |
| g | The parts of `overloading::conversion_sequences` and `overloading::overload_resolution` that step 3's pages call for operands that are not classes. A class operand gives `unsupported`. | `ics.rs` and `ovl.rs`, rewritten in part |
| h | The `Ext` node point | none |

Part g is the edge of the page table that crosses from step 3 to step 4. `expressions::postfix`, `expressions::conditional`, `statements::range_for`, `expressions::new_delete`, and `declarations::initialization::forms` depend on the overloading pages. Calls of overloaded functions on fundamental types are common in the 1890 plain tests. So step 3 writes those two pages for non-class operands, and step 4 finishes them.

The machine keeps the shape of the old typed tree (`DESIGN-rewrite.md` section 8, "`ast.rs` moved; nodes gain marks and spans"). Then the run parts move with small changes, and the program tests run on the new front end as soon as analysis gives them a tree.

**What `tools/frontend_check.py` needs.**
1. **The shadow line.** Once `conform` moves, `--next` runs `tidal-next conform tests -v` beside the old binary and prints the shadow lines by the scope of `step_scope.py`:

   ```text
   next: shadow: step 3 plain: P/1890 pass their check, Q of the old front end's 1885
   next: shadow: step 3: P/3791 pass their check, Q of the old front end's 3775; R pass on the new front end only
   next: shadow: all: P/7073 pass their check; U unsupported
   ```

   The tool lists each test that the old front end passes and the new one fails in the step 3 scopes. The tool can sort tests by their rules line as `step_scope.py` does, and can read the page table's prefixes from `DESIGN-rewrite.md` section 3 or from the chapters of `specification/`.
2. **A shadow baseline.** `--update --next` writes `conformance-next.tsv`, one status per test from the new front end. A move to a failing status is printed. A move fails the check only for an owned test.
3. **The takeover.** For an owned test the tool takes the new front end's status into `conformance.tsv`, and the completion line adds "O owned by the new front end" after the marked count.

**The tests it takes over.** The marked tests stay with steps 5 to 7. The first takeover comes at the end of step 3, test by test, for the tests of step 3's scope that the new front end passes and the old one fails. `DESIGN-rewrite.md` section 10 leaves the unit of takeover open, and the phase 17 decision settles it for these tests. The candidates are the 10 unmarked tests of step 3's scope:

- `foundations/constant-evaluation/`: `read-extended-temporary-not-const.cpp`, `reference-to-local-not-usable.cpp` (CE-3)
- `foundations/language-support/ordering-defaulted-specified-needs-conversion.cpp` (LSUP-13)
- `foundations/names/adl-function-template-parameter-adds-nothing.cpp` (NAM-8)
- `expressions/primary/fold-expansion.cpp` (PRIM-27)
- `expressions/decltype/nested-class-member-evaluated.cpp` (UNEV-1)
- `expressions/lambdas/`: `capture-default-in-member-initializer.cpp` (LAM-6), `capture-visible-in-trailing-return.cpp` (LAM-18)
- `declarations/constexpr/lambda-escalates-runtime.cpp` (CEV-1)
- `declarations/structured-bindings/tuple-member-get-type-parameter-uses-free.cpp` (SB-5)

Most of these use a class, a template, or a pack beside their rule, so I expect step 3 to pass few of them. The two CE-3 tests are the likeliest. The takeover rows still exercise the list, the shadow baseline, and the completion line on a few tests before step 5 moves 17. `statements/if/constexpr-template-nondependent-call-checked.cpp` sits in step 3's scope by its rule (IF-22) and stays with step 5 by its mark.

**When the first marked takeover can come.** Step 5 needs the templates pages, and these need the class pages of step 4, because nearly every marked test declares a class template. Step 5 also needs G166 and G167 (item 1) for 3 of its 17 tests, and model section 1 in the oracle for one more. Step 6 needs G167 for one of its 13. With step 3 in phase 17 and step 4 in phase 18, step 5 comes in phase 19. Step 5 can come in phase 18 only if phase 18 does steps 4 and 5 together.

**How the old front end is kept.** Item 3 gives the old code's work. The old front end judges every test that the takeover list does not name, and its rounds keep "0 worse".

**The measure that ends the item.**
- `next: shadow: step 3 plain` passes each of the 1885 tests that the old front end passes, or each difference has a cause in the round report.
- `next: shadow: step 3` gives a count, and the round report sorts each test that the old front end passes and the new one does not by the later step whose construct it needs, by a G row, or by a fault.
- The 3 plain program tests run to their result on the new front end.
- `next: unit` passes, with unit tests in the `check.rs` and `run.rs` parts of each step 3 page and in `sem`, `target`, and `machine`.
- No file of `frontend/next/` passes about 2000 lines, and no module names a rule ID of another page (`DESIGN-rewrite.md` section 9).
- `next: parse` does not lose a test.
- The old front end's line stays at "0 worse" on every commit of the item.
- The takeover list holds a row for each candidate that the new front end passes, or none.

**Evidence.** Section 1. Steps 1 and 2 ended in phase 16, and every later step needs `sem` and the machine. The old front end's machine and constant evaluation move with small changes, so a shadow run of programs comes early. 3791 tests sit on step 3's pages, and the old front end passes 3775 of them.

**Size.** Large. About 11900 lines move (parts c, d, and e). The checks of part f are new code. The old checks for the four chapters sit in about 26000 lines of `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `stmt.rs`, and `parser.rs`, mixed with class and template code.

**Prerequisites.** Item 1 for G168, G170, and G176 before part f's shadow line. None for parts a to e.

**Risk.** High. Three points carry most of it:
- The tree's form (part a). An analysis written over the generic node and changed later would touch every page. The choice comes before the first page check.
- The pages that cross into step 4 (part g). A stub that gives `unsupported` for a class operand keeps the shadow line honest. A stub that guesses would give false passes.
- Analysis takes over the oracle from `sem::names`. The parse lines guard the oracle's answers, so a change in an answer shows as a parse disagreement.

**Alternative.** Steps 3 and 4 in phase 17, measured on the 5664 tests of both scopes (5635 old passes). This brings step 5 to phase 18. I rank the narrower scope first because step 4's old code (`class.rs`, `special.rs`, `dcmp.rs`, `ics.rs`, `ovl.rs`, and the class checks in `decl2.rs` and `expr.rs`) is rewritten, and the 1890 plain tests give step 3 a clear floor.

### 3. Front-end round 28 on the old code

**What it is.** A round in the form of report 27, with the `next:` lines in its section 1 and the completion line read with the marks file. Decision 0064 keeps the old front end as the front end of record, so the round judges every test that the takeover list does not name. The groups, in order:

1. **Phase 17's new tests**, as items 4 and 5 add them. Each failing new test is marked, named as a step 3 candidate, or fixed in the old code.
2. **The machine and constant evaluation, before step 3 copies them.** The constant-evaluation groups (PRIM-27, CE-3 twice, LSUP-13, OVL-40, and CTOR-12) and the 23 run stops sit in `consteval.rs` and `interp/`. Item 2 moves these files with small changes. A fix made before the copy carries over once. After the copy, each fix is a twin change. So the round takes this group first and tells item 2 when the files are ready to copy.
3. **Records.** Report 27 section 8's status sentence (17 missed rejects and 18 unsupported). `DESIGN-rewrite.md` sections 4 and 5: the marks file's 36 tests, and the build table's counts by step (17, 13, and 6). The design's section 10 line on the copied files, which names `conform.rs` before it moves. `TEMPLATE-MODEL.md` section 8: the RV9 row becomes a row for RV10, with INST-30, INST-1, INST-P4, and FN-12, and RV10-R-1 and RV10-R-2.

**What the round does not do (decision 0064).**
- No fix of the 36 marked tests.
- No fix in `templ.rs`, `cons.rs`, or `ctad.rs` for the 20 template tests outside the marks, unless the fix is small. The rewrite writes these files again in step 5.
- The 10 step 3 candidates of item 2 stay unfixed in the old code. The other unmarked tests (DCMP-11, PFX-11, OVL-20, ICS-47, CPY-27, CNS-32) sit in rewritten checks, and the round takes them only where the fix is small.
- No refactoring of the old code.

**Twin changes.** The old files with a copy in the new crate are `lexer.rs`, `diag.rs`, and `main.rs` today. As item 2 copies them, `types.rs`, `ast.rs`, `layout.rs`, `conform.rs`, `consteval.rs`, and the files of `interp/` join the list. A round that changes one of them makes the same change in `frontend/next/` and lists it in its report. The lex count catches a lexer change that reaches one copy only. The shadow line catches the others only on tests that both front ends run.

**Evidence.** Section 1. The old front end judges every test in phase 17. Its machine groups are the only failures in code that the rewrite moves rather than writes again.

**Size.** Small to medium. Group 2 is the largest part.

**Prerequisites.** Items 4 and 5 for their tests. None for groups 2 and 3.

**Risk.** Low. Group 2 and item 2's copy of `interp/` must not run at the same time. The integrator orders them.

### 4. Text for the X24 and RV10 rows

**What it is.** The clean-room writer acts on each open row, in this order:

1. **X24-1 (OBJ-3, JMP-9, LOOP-3, RFOR-4).** A rule for the end of a statement's own variables after a `goto`, a `return`, or a `throw`, from a probe of X24's `d1.cpp` on the matrix, with a run test. The machine's order is checked against the test.
2. **X24-3 (JMP-14, DEP-30).** A question, since the compilers split on a `goto` to a missing label in a template that is never instantiated. The provisional answer says whether model section 5's list of definition checks gains a row.
3. **RV10-R-1 and RV10-R-2 (INST-30, INST-1, INST-P4, CLS-2, FN-12).** The complete-class places that INST-30's exception leaves out, and FN-12 for a nested class defined after its enclosing class. Each gets a test where the compilers agree.
4. **RV10-R-3 (OBJ-2, Q-0447)** and **RV10-R-5 (CPT-28).** A condition for a scalar union member, and the variable template and `if constexpr` forms.
5. **X24-2, X24-4, X24-6, X24-7.** JMP-6's reach, FN-7, the lambda body and the local class in JMP-7 and JMP-14 against SW-3, and an example of one label name in a lambda body and its function.
6. **X24-5, RV10-R-6, RV10-R-7, RV10-R-8.** The glossary entry for label, and three small records.
7. **X24-8.** No change. The row becomes `text`.

**Evidence.** Section 1, "The X24 and RV10 findings". X24's host-only claims need probes on the matrix before X24-1 and X24-3 get tests.

**Size.** Small each. X24-1 and X24-3 may each need a question.

**Prerequisites.** None.

### 5. Gap review RV10 of the 10 changed rules and item 1's grammar, with the read of its repair diff

**What it is.** A source-reader gap review of the 10 rules that the phase 16 close lists (UNI-14, FN-12, CE-22, OBJ-1, OBJ-2, STG-6, CPT-28, INST-P4, INST-1, INST-30), item 4's text, and item 1's 14 Syntax changes with G166's sentence on casts and calls. It reads INST-30, INST-1, INST-P4, and FN-12 first. The item ends with the recorded read of its repair diff (decision 0063).

**Evidence.** Section 1. Model section 3 rests on INST-30 and INST-1, which changed in RV9's repairs and carry RV10-R-1. The grammar changes of item 1 are text that no source reader has read, and the parser follows them literally.

**Size.** Medium.

**Prerequisites.** Items 1 and 4 for their text.

### 6. The P22 prototype on the new front end

**What it is.** The prototype `src/proposals/p22.rs` of `DESIGN-rewrite.md` section 7, from X24's design: the alternatives at `jump_stmt` (the nonterminal point exists), the check of the `Ext` node with X24:new-1 and X24:new-2, and `Break { target }` and `Continue { target }` for the machine. Its tests live beside `reports/clean-room/X24/report.md`. The impact diff runs `tidal-next conform` with and without `--proposal P22`, and the round report compares it with X24 section 4 (2 tests change, no program changes).

**Evidence.** Decision 0064 had item E come first so that the first prototype follows a design. The parts the prototype needs (`statements::jumps`, `statements::blocks`, and the `Ext` node point) come with step 3.

**Size.** Small.

**Prerequisites.** Item 2's statements pages and part h.

**Risk.** Low. Until step 8 the impact diff's base is the tests the new front end passes, so the count is a lower bound (`DESIGN-rewrite.md` section 7).

### Dependency sketch

```
1 G rows: text ──> parser functions ──> next: parse (tree side 216 -> 4)
      G168, G170, G176 ──> 2f (step 3 pages, the plain shadow line)
      G166, G167 ──────────────────────────────> step 5 (phase 19)
2 step 3: a tree form, b node checks ──> c sem ──> d machine, conform ──> f pages ──> h Ext ──> 6 P22
                                       3 group 2 (machine fixes) ──┘
                                                      └──> first takeover (step 3 candidates)
4 text ──> 5 RV10 (with 1's grammar) ──> step 6 (model section 3, later)
```

Items 1, 2 (parts a to e), 3, and 4 can start together. Item 5 waits for the text of items 1 and 4. Item 6 waits for item 2's statements pages.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| `<cstdarg>`, named escapes, `#include <...>`, extended floating-point types | the library, a name table, real headers, *(referenced)* | G42 and G158 to G160 hold 9 tests; the completion line leaves them out; 2 of them are tree-side parse disagreements that stay | Keep the rows |
| ILP32 | in the matrix (EDG ILP32) | `target` moves in step 3 with both tables | Keep in scope |
| `no-diagnostic` rules | the class needs no diagnostic | 6 tests pass as `NO-DIAGNOSTIC-ALLOWED`; the shadow line keeps the status | Keep the status |
| Coroutines and lazy generators | outside the language goal | No test or front-end path needs them | Keep out |
| Threads, the memory model, modules | outside the language goal | No test or front-end path needs them | Keep out |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | X24-8 names similar forms as outside knowledge; P22's prototype runs only under its flag | Keep out |
| MSVC | not in the matrix | A second ABI would double `target`'s tables | Keep out |
| Prototypes of Propose designs | not part of the rules | P22 at step 3 (item 6); a design that changes run-time rules needs the sixth point of `DESIGN-rewrite.md` section 10 | P22 only, under its flag |
| Implement I8 on instantiation | deferred by decisions 0062, 0063, and 0064 | Step 6 writes model section 3 in phase 19 or later | Keep deferred until step 5 starts |

## 4. Process changes

1. **The design names the marks file and copies no count.** `DESIGN-rewrite.md` section 5 has gone stale at each round since report 25 (30, then 32, now 36). The section names the file and gives the step for each model section. `tools/frontend_check.py` prints the marked count by section (now 4, 6, 13, 6, and 7), so the build table needs no number.
2. **The parse count judges the tree side strictly.** A test that expects `accept`, `run`, `run-fail`, or `link-fail` must give a tree. For a reject test, the parse count reports the same-line count until step 3, and the shadow line judges the outcome from then on. The 140 reject disagreements that are not findings then leave the parse line.
3. **A G row lands with its parser function.** The commit that answers a G row with text, or the commit after it, adds the function, and its message gives the `next: parse` line.
4. **A list of the files with two copies.** `DESIGN-rewrite.md` section 8 gains a column with the commit that copied each old file. A round's report lists each change to a file in that column and its twin.
5. **The shadow line counts by build step.** `--next` prints the shadow line for the step 3 plain tests, the step 3 tests, and all tests, as item 2 gives. A shadow baseline (`conformance-next.tsv`) shows a regression of the new front end before it owns a test.
6. **The unit of takeover is the test for step 3's candidates.** The phase 17 decision settles the open point of `DESIGN-rewrite.md` section 10 for the tests that the new front end passes and the old one fails, and leaves the page as the unit of step 8.
7. **A front-end report prints the `next:` lines.** Report 27 left them to its commit message. Each round report gives them in section 1, with the shadow lines once they exist.
8. **The rewrite sets the first-error target before step 3's reject tests pass.** The old rate has stayed at 77% for eight phases. The new front end's target on owned reject tests is at least the old rate on the same pages.

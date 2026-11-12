# Roadmap, round 15: what to do after phase 17, from a language designer's view

Date: 2026-11-11. Decision 0029, with decisions 0037, 0039, 0060, 0064 and 0065. Role: clean-room reader, language-designer persona (`designer`). Inputs: AGENTS.md and CLAUDE.md; my round 14 report `reports/clean-room/roadmap-14/designer.md` (the model for this form) and its scratch scripts; `reports/clean-room/roadmap-7/portfolio.md` (P22 to P30, with P26 done by X26 and P30 new); `documentation/` (decisions 0060 to 0065, `decisions/pending.md` with its decision packet and the row 7 plan, the status page, the roadmap's phase 17 table, and sections 4, 8 and 9 of `exercises.md`); `git log fb0bcf87..HEAD` with every commit body; `frontend/DESIGN-rewrite.md` (sections 5, 7 and 10 in full) and `frontend/TEMPLATE-MODEL.md` section 8; `reports/clean-room/X26/report.md` with its scan outputs; `reports/clean-room/X24/report.md` sections 1.7, 1.8 and 2; `reports/clean-room/rust-frontend/report-rw3.md` and `takeover.tsv`; the RV10 gap review and the read of the RV10 repair diff in `reports/review/` (2026-11-11), at their result tables and their points; `reports/clean-room/ledger.md` and the front-end ledger's state column; the headers of the tests that X26's change table names; and `tools/propose_check.py` plain, with `--corpus`, and with `--strict`. Counts come from read-only Python scripts in `reports/clean-room/roadmap-15/designer-scratch/scripts/`, run on this tree and on copies of `fb0bcf87` (the tree that round 14 read, with the specification of `f77bbe05`) and `e3cb5a3c` (the tree that the RV10 review read), made with `git archive` outside the repository. Their outputs are beside the scripts. I did not read the other round 15 reports, `inputs/`, `build/exercise-keys/` or the sources under `frontend/`. I did not run `tools/run_tests.py`, `tools/frontend_check.py`, the front end or a compiler. Statements from outside Tidal's files are marked "general knowledge". Commit read: `7a849708`.

## 1. The state from a designer's view

### 1.1 Counts at 7a849708

| Measure | Round 14 (f77bbe05) | Now (7a849708) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1700 | 1700 |
| `required` / `reject` / `undefined` | 900 / 559 / 57 | 900 / 559 / 57 |
| `impl-defined` / `recommended` / `no-diagnostic` | 52 / 43 / 37 | 52 / 43 / 37 |
| `unspecified` / `conditionally-supported` / `withdrawn` | 32 / 5 / 15 | 32 / 5 / 15 |
| Rules and principles whose text changed in the phase (my diff, `:tests:` left out) | 18 (14 rules, 4 principles), none added | 26 (23 rules, 3 principles), none added |
| Of these, changed after the gap review read the tree | 10 after RV9 | 9 after RV10 (e3cb5a3c), all in the RV10 repairs, which the read of 88d14ea2 covers |
| Test files with a `tidal-expect` line | 7073 | 7148 |
| Of these, `reject` | 2840 | 2863 |
| Of these, `link-fail` / with `-m32` | 46 / 76 | 46 / 76 |
| Program corpus (`propose_check.py --corpus`) | 166 programs (99 Write, 67 sketches), 8 multi-unit, 189 files, 23005 lines | 178 programs (99 Write, 79 sketches), 8 multi-unit, 201 files, 25553 lines |
| Questions | 449 | 452 |
| Provisional / settled / compiler defects | 200 / 236 / 13 | 202 / 237 / 13 |
| Open questions in the packet's rows 1 to 9 | 122 of 200 (61%) | 123 of 202 (61%) |
| Propose reports | 25 | 26 (X26) |
| `propose_check.py`: conflicts | 31 (10 withdrawn, 4 class, 17 new-id) | 31 (10 withdrawn, 4 class, 17 new-id) |
| `propose_check.py`: rows changed since their report's commit | 168 | 185 |
| Change-table rows whose rule text changed in the phase (my diff) | 19, in 9 reports | 54, in 15 reports |
| `required` rules whose text contains "not valid" | 76 | 76 |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 51 of 559 | 52 of 559 |
| Old front end, full pass / passes its check | 6967 / 6990 of 7073 | 7049 / 7069 of 7148 |
| Completion line of decisions 0060 and 0064 | 74 fail their check (36 marked, 38 counted), 0 of 166 program tests do not run | 70 fail their check (38 marked, 32 counted), 1 of 178 program tests does not run, 9 out of scope |
| Marks file by model section | 36: 4, 6, 13, 6, 7 | 38: 4, 6, 15, 6, 7 |
| The rewrite's lines | `next: lex` 7191/7191, `next: parse` 6475/6913, `next: owned` 0 | `next: unit` 126, `next: lex` 7266/7266, `next: parse` 6828/6987, `next: shadow: step 3 plain` 1602/1897, `next: owned` 1327 |
| Exercise ledger rows | 1129; 15 `open` | 1164; 1054 `commit`, 54 `text`, 40 `question`, 3 `out of scope`, 13 `open`, 0 `planned` |
| Front-end ledger rows `open` | 14 (G166 to G179) | 2 (G182, G183) |

Phase 17 added no rule ID and changed no class. The 17 new changed rows of `propose_check.py` are 12 in X24, 2 in X26 (LOOP-3, DEP-30), 2 in X22 (ATTR-2, ATTR-P1) and 1 in X20 (ENUM-20). X26's 39 rows have 0 conflicts. `propose_check.py --strict` exits 1 on the whole set, from the 31 old conflicts.

### 1.2 What phase 17 did with the round 14 designer items

Decision 0065 took round 14 items 1, 2, 3 and 5, and parts of items 4 and 7. It deferred refresh notes and the index of what a test of validity can see.

| Round 14 item | Phase 17 | State |
|---|---|---|
| 1. X24's rows before step 3 | F7 (b511838d), FE (c0461b86) | done. X24-1, X24-2 and X24-4 to X24-7 by commit, X24-3 by Q-0451, X24-8 as text. The writer probed every host-only claim. No X24 refresh note exists |
| 2. Section 7 and the P22 prototype | R11 and RW2 (5f100f69) | done in part. Section 7 follows X24, the own IDs come with the `Ext` node point, and a unit test reads the kinds from X24's table. The diff over a base of 1819 tests changes exactly the two JMP-17 tests. `reports/clean-room/X24/tests/` does not exist, so the 19 prototype tests did not run |
| 3. X26 on P26 | E (3a7e991c) | done. 39 rows, 20 tests from an accepted outcome to `reject`, 0 of 166 programs change, 6 findings, 7 host-only claims |
| 4. The portfolio step | E, in part | P26 is marked done and P30 is new. P22's last column still names "The marker of the target". P25's section still says "Rating: ready". P26's section still says 126 programs. P29 still says 18 lines in 6 files and has no word on the user-suffix case. No prototype column |
| 5. The RV10 rows with design reach | F7 | done. RV10-R-3, R-5, R-6 and R-7 by commit |
| 6. Refresh notes and a refresh check | none | deferred again. 185 changed rows |
| 7. The records in the present | R11, E | in part. `exercises.md` section 8 gained X26, in the order X26, X24, X25, X23, X22. Section 4's prose still names `reports/clean-room/roadmap/designer.md` and ends at "with X22 on P23". `TEMPLATE-MODEL.md` section 8 and `DESIGN-rewrite.md` section 10 still call RV10-R-1 and RV10-R-3 open, and both are `commit`. Row 7's plan still says about 2640 tests and 7900 compilations |
| 8. The packet rows that reach proposals | none | the request of 2026-11-10 has no answer |
| 9. An index of what a test of validity can see | none | deferred. 52 of 559 `reject` rules |

Round 14's process changes:
- Taken: process change 2 (the prototype takes its kinds from the change table), process change 7 for section 7 (it follows X24), and process changes 8 and 9.
- Taken in part: process change 1. F7's body names X24. The RV10 repair 88d14ea2 changed OBJ-2, JMP-7, JMP-14 and DEP-30, which 17 change-table rows in 11 reports name, and its body names no done design. Process change 3 holds by result: the P22 diff matched X24's table, so no finding was needed.
- Not taken: process change 4 (no prototype tests beside X24), process change 5 (P26 and P29 counts), process change 6 (section 4's prose, section 8's order), and process change 7 for `TEMPLATE-MODEL.md` section 8.

### 1.3 Phase 17 text that reaches a done design

My diff of `fb0bcf87` against `7a849708` finds 54 change-table rows in 15 reports whose rule or principle text changed:

| Report | Rows | Effect on the design |
|---|---|---|
| X24 (P22) | ATTR-P1, DEP-30, FN-7, JMP-6, JMP-7, JMP-9, JMP-14, JMP-P1, JMP-P2, LOOP-3, OBJ-3, RFOR-4 | F7 wrote the `goto`, `return` and throw parts of X24's own findings into the text (X24-1, X24-2, X24-4 to X24-7). X24's amendments of OBJ-3, LOOP-3, FN-7, JMP-7 and JMP-14 now add only the labeled-jump part. The RV10 repairs then moved the jump conditions of a template with no specialization under DEP-30 (Q-0451, option 2) and kept the rejection in a discarded statement (option 1, with the doubt of RV11-R-6). X24 section 1.8 lists "Labeled jumps under DEP-30" as an alternative and says that the choice should follow X24-3. X24-3 now has its answer, so X24's DEP-30 row and its test `break-label-template-never-instantiated.cpp` (`reject`) follow the old model |
| X11 (P14) | JMP-6, JMP-7, JMP-9, JMP-14, JMP-P1, JMP-P2, LOOP-3, OBJ-2, OBJ-3 | A scope-exit block meets OBJ-3's new statement scope. Its order against a condition variable after a jump is now stated text, and X11 must place its blocks in that order. Rebase note |
| X6 (P10) | CE-3, DEP-30, JMP-6, JMP-7, JMP-9, JMP-14, JMP-P1, RFOR-4 | An expansion statement meets RFOR-4's range temporary after a jump, and CE-3's `mutable` sentence. Rebase note |
| X9 (P8) | JMP-6, JMP-9, JMP-P2, OBJ-1, OBJ-2, OBJ-3 | OBJ-2 now follows UNI-13's chain to a union member that a destructor call ended (RV10-12). X9 ends a lifetime at relocation, and the chain must say whether a relocated member counts as ended. Rebase note |
| X3 (P5) | CE-22, JMP-6, JMP-P2, OBJ-2 | CE-22 now says that a `mutable` member that is not active sets no limit. X3's narrowing of "has no value" keeps that case |
| X25 (P27) | OBJ-1, OBJ-2 | OBJ-1's array row names a default constructor that is not trivial (RV10-R-7). OBJ-2 keeps the array element out (RV10-R-3), which is X25's case, and RV11-R-5 finds that OBJ-2 leaves out "not deleted" for a class member |
| X26 (P26) | DEP-30, LOOP-3 | LOOP-3 gained the statement's end after any jump. X26's LOOP-3 row (the iteration expression is a discarded-value expression) still fits. DEP-30's new jump item leaves X26's row unchanged |
| X22 (P23a) | ATTR-2, ATTR-P1, DEP-30 | ATTR-2 lists `maybe_unused` on a type and before a using-directive. X22's marker rows fit |
| X6, X7, X10 | CE-3 | CE-3 says that no part of a `mutable` member of a usable variable is usable. X7's and X10's constant-evaluation rows fit |
| X12, X13, X18 | DEP-30, FN-7 | state effect. Fits |
| X20 (P17) | ENUM-20 | ENUM-20 says that the name in `using enum` must not be dependent. Fits |
| X5 (P9) | AUTO-11 | AUTO-11 lists four places for `decltype(auto)`. Fits |

X24 holds 12 of the 54 rows, and 11 of those are F7's answers to X24's own findings. X24's design is now the one done design whose text base moved the most, and the P22 prototype runs against it.

### 1.4 The 9 rules changed after RV10, ranked by design reach

| Rank | IDs | Done designs and proposals reached | Why a designer reads them |
|---|---|---|---|
| 1 | JMP-14, JMP-7, DEP-30 | X24 (all three), X6 and X11 (all three), X12, X13, X18, X22, X26 (DEP-30); P30 | Where a labeled jump's check runs in a template. RV11-R-6 on the discarded statement |
| 2 | OBJ-2 | X3, X9, X11, X25; P27 | The chain to an ended union member. RV11-R-5 |
| 3 | CE-3 | X6, X7, X10 | A `mutable` part in constant evaluation |
| 4 | ENUM-20, ATTR-2, AUTO-11 | X20, X22, X5 | One row each, state effect |
| 5 | TPL-4 | none | RV11-R-1 (a friend with two template heads) |

The read of 88d14ea2 covers all 9. No rule waits for a review from a designer's view.

### 1.5 Pages the Propose exercises have reached

All 85 pages are reached by rule or principle IDs cited in X1 to X26. One page is read by one exercise only: `declarations/namespaces.md` (X15, 9 commits to the page since). 15 pages are reached by two exercises. `expressions/comma.md` moved from one (X1) to two (X1, X26). The pages of the two ready proposals:

| Page | Exercises that reach it | Ready proposal that aims at it |
|---|---|---|
| `expressions/bitwise.md` | X1, X17 | P24 |
| `declarations/enumerations.md` | X12, X17, X20 | P24 |
| `overloading/overload-resolution.md` | 9 (X5 to X26) | P24 |
| `statements/jumps.md` | 7 (X3 to X24) | P30 |
| `statements/blocks.md` | X9, X11, X24, X26 | P30 |
| `declarations/functions/definitions.md` | 7 (X1 to X24) | P30 |

P24 reaches less-read pages. P30 reaches pages that X24 read in phase 16 and F7 changed in phase 17.

### 1.6 What a designer still cannot state

**What counts as an effect.** New (X26-3). COMMA-5, UNEV-3 and STMT-4 each recommend a warning for an expression with no effect, and no page says what an effect is. The host compilers differ. X26 defines a form-based "discardable operand" for its own rule. A later proposal on expression statements needs the same term.

**When a discarded class temporary ends.** New (X26-1, kind wrong). UNEV-P2 and UNEV-2 say "destroyed at once". COMMA-1 and its test say that the left operand's temporary lives to the end of the full expression.

**What happens to the value of a `for` iteration expression.** New (X26-2). The iteration expression is not in the list of discarded-value expressions.

**Whether a constraint check runs.** Unchanged. Q-0426 waits in row 4b.

**Whether a typed step over bytes is defined.** Unchanged. Q-0430 and Q-0442 sit in row 4a.

**What a standard attribute may do.** Q-0412 (row 9) unchanged. P24 and P28 choose a marker under it.

**Whether a later version has taken the space.** Row 7 unchanged. Its count is stale (section 1.2).

**How much code a change touches.** Measurable in part. The harness gives an impact diff for a design that uses the nonterminal and `Ext` node points, over the tests that pass without the flag (1819 of 7148, and 1 of 178 programs). The node-check point exists with no user. Rule overrides land at step 4 and definition checks at step 5.

**Which programs change meaning without a diagnostic.** 52 of 559 `reject` rules.

**How two changes compose.** No pair exercise has run. P30 extends P22's own rules, so a design of P30 is the first case.

**What a change costs.** Q-0183 keeps option 1.

These round 14 points are now answered: when a statement's own variables end after a jump (OBJ-3, LOOP-3, RFOR-4, JMP-9 at run time, with Q-0452 for the constant-evaluation order on GCC 16 and EDG), and whether a jump in a template needs its diagnostic at the definition (DEP-30 for a template with no specialization, JMP-14 for a discarded statement, Q-0451).

### 1.7 The portfolio and the stop rule

| # | Proposal | Round 14 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P26 | No overloaded comma, checked left operand | ready | done (X26) | X26 settled the five open points by a form check on the left operand | X26-1 to X26-6; the hook points of section 1.9. The entry's last column still says "The meaning of 'has no value' for a class result", which X26 settled. Its section still says 126 programs |
| P30 | A label on a block that `break` can leave | none | ready | X24 section 1.8; E added the entry | The entry says "It needs X24-1 answered first". F7 answered X24-1 and X24-4 (FN-7). The rest are design choices |
| P24 | Flag enumerations | ready | ready | | The marker under Q-0412 (row 9) and beside X12's `final`. ENUM-10 in row 4a (Q-0407). A prototype needs built-in candidates, which none of the five points offers. My scan finds one program (`r8-config.cpp`) and one rule test (`classes/operators/enum-operator.cpp`) that declare `operator\|` on a scoped enumeration |
| P29 | A pointer comparison with a string literal operand is not valid | partial | partial | | 24 lines in 9 program files (unchanged at 178 programs). `r12-sso.cpp` compares with `"x"_sv`, and the entry still says 18 lines in 6 files |
| P23b | Expressions checked against requirements | partial | partial | | No matching rule (X22-8), Q-0411, Q-0412, Q-0426, the strength of CNS-13 |
| P28 | Arithmetic that wraps on request | partial | partial | | Where the marker attaches |
| P8b | Relocation by bytes | partial | partial | | Row 2 (Q-0322), row 6 (Q-0343, CPY-19) |

Totals in the file: 2 ready (P24, P30), 3 partial (P23b, P28, P29), 5 done from P22 on, 0 blocked. After one more exercise, one proposal stays ready.

**P26 at 178 programs.** The 12 programs that T6 added after X26's commit read include `r14-fsm.cpp` and its second try. Each holds three folds over the comma. By my text read, each fold element is a call, an assignment, or a comma whose operands are an assignment and a compound assignment, so each passes X26:new-1. The corpus now has 10 files with a fold over the comma, and still 0 of 178 programs change. X26's scan, which compiles each file, has not run on them.

**The stop rule.** Round 7's rule stops new proposals if a round of three exercises closes fewer than five findings each and finds no wrong text. By the ledger, over X24 to X26:

| Exercise | Findings | `commit` | `text` | `question` | `open` | Kind "wrong" |
|---|---|---|---|---|---|---|
| X24 | 8 | 6 | 1 | 1 | 0 | 0 |
| X25 | 9 | 7 | 1 | 1 | 0 | 2 |
| X26 | 6 | 0 | 0 | 0 | 6 | 1 (X26-1) |

X24 and X25 each closed all their rows, and X26 found wrong text, so the rule does not stop the lane. X26's five rows that ask for text can close in phase 18.

### 1.8 The 13 open ledger rows, ranked by design reach

| Rank | Rows | What they reach | Who acts |
|---|---|---|---|
| 1 | X26-1 | UNEV-P2, UNEV-2, COMMA-1, VC-6. The cast to `void` that X26 keeps as the way to keep a value | writer |
| 2 | X26-2, X26-4 | LOOP-3 and the list of discarded-value expressions; a glossary entry. X26's LOOP-3 row and the term that X26:new-1 builds on | writer |
| 3 | X26-3 | COMMA-5, UNEV-3, STMT-4. The base of any proposal that turns a warning into a rule | writer, with X26's `w1.cpp` forms on the matrix |
| 4 | RV11-R-6 | JMP-14 in a discarded statement. X24's checks and P30's | writer, with a question line |
| 5 | RV11-R-5, F7-2 | OBJ-2, UNI-13, Q-0447. X25, X3, X9, X11 | writer |
| 6 | X26-5 | PRIM-28's example | writer |
| 7 | RV11-R-3 | ATTR-19's place table. A marker of P24 or P28 if it is an attribute | writer |
| 8 | RV11-R-1, RV11-R-2, RV11-R-7, RV11-R-8 | TPL-4, the casts paragraph, TARG-16 (X7's row), AUTO-11 | writer |
| 9 | X26-6 | outside knowledge: a library header in scope that declares `operator,` | writer, within `documentation/clean-room.md` |

X26's seven host-only claims rest on g++ 16 and clang++ 22. EDG did not run. The warning table of `w1.cpp` (X26-3) and the `s1.cpp` choice (X26:new-3) are the two that X26's design leans on.

The front-end ledger has G182 (a union partial specialization) and G183 (LIST-4 and conversion functions) open. Neither reaches a proposal.

### 1.9 The P22 prototype, and a P26 prototype through the node-check point

**What the P22 prototype showed.** `src/proposals/p22.rs` adds the two alternatives at `jump_stmt` and checks its node at the `Ext` node point (report-rw3 section 4). These points hold:
- The two points work end to end on a real design. The impact diff over 1819 tests changes exactly the two JMP-17 tests, from reject to accept, as X24's "Tests affected" column says. Their `f()` gives 5 and 6, as X24 section 2 says.
- The diff reaches 2 of X24's 33 rows through `tests/`. X24:new-1 to X24:new-3 are checked by the prototype's unit tests only, because the 19 prototype tests have no directory.
- A unit test reads X24's change table and checks the kinds of the own IDs. The prototype cannot drift from the table's kinds.
- The base holds 1 of 178 programs. X24's "no program changes" is measured on one program.
- "Run: none" holds for loops and `switch` in step 3. The statement-scope order of X24-1 has no owned test. The three `statement-scope-ends-after-*` tests print from destructors, so they need step 4.
- X24's step 5 line (X24:new-1 and X24:new-2 among the definition checks) follows X24's own model. Under Q-0451's answer, the text puts the jump conditions of a template with no specialization under DEP-30. If X24 follows its own alternative of section 1.8, the step 5 line is needed only for a discarded statement, and `break-label-template-never-instantiated.cpp` cannot be a `reject` test.
- No finding against the text. The first measured design agrees with its table.

**What a P26 prototype could measure at step 3.** I set X26's 39 rows against the point and the build step each needs. Ownership comes from `takeover.tsv`. The "uses" column comes from each test's text (`p26_reach.out`, `p26_owned_commas.out`):

| X26 rows | What the prototype does | Point | Build step | Tests |
|---|---|---|---|---|
| X26:new-1 for a non-dependent built-in comma; COMMA-1 to COMMA-8 state effect; COMMA-5 withdrawn | a node check on each comma node: the left operand must be a discardable operand | node checks | 3 | 7 of the 15 tests that fail X26:new-1 are owned: `argument-separator`, `array-bound-parenthesized`, `constant-expression`, `initializer-separator`, `result-lvalue-address`, `subscript-parenthesized`, `extended-temporary-through-cast`. `unused-left-void-cast` (owned) must stay `accept`. 12 owned files hold a valid built-in comma and must not change |
| the same rule on class operands | the same check | node checks | 4 | 6 tests use a class (`Probe`, `Note{1}`): `temporaries-end-with-full-expression`, `separators`, `lifetime-through-parens-and-conditional`, `full-expression-subexpression-not`, `constant-read-through-operators`, `temporaries-reverse-order-comma` |
| the same rule in a template argument or a class template | the same check | node checks | 5 | `template-argument`, `result-category` |
| UNEV-1, CPT-30: the unevaluated exemption | the check skips a comma inside an unevaluated operand | node checks, with a context the point does not give | 3 for `decltype` and `sizeof`; 5 for a requires expression | 0 owned. Both comma tests in an unevaluated operand use a class or a template |
| overloadable_operator, OPR-2, OPR-7, OVL-12; COMMA-7 withdrawn | reject any declaration of `operator,` | none of the five. A nonterminal point that removes an alternative, or a check on a declaration | 4 | 5 tests, each with a class |
| X26:new-2 (folds) | the check on the written-out fold | node checks at substitution | 5 | 41 test files with a fold over the comma, 0 owned; `type-dependent-fold` |
| X26:new-3, DED-P5, DED-29, CNS-7 | a violation at substitution is an error and never a substitution failure | node checks at substitution, with a class for the result the point does not give | 5 | no test in `tests/`; X26's `s1.cpp` |
| COMMA-P1, COMMA-P3, UNEV-P2, UNEV-2, UNEV-3, STMT-1, STMT-4, ATTR-5, LOOP-3, the glossary entry | text only | none | none | none |

So the step 3 part of a P26 prototype is one check at one point. It exercises 7 tests that change, 1 that must stay, and 12 owned files with valid commas, over the same base as P22. The diff should change exactly those 7 among owned tests, plus any of the other 8 that the base of 1819 holds. Any other change is a finding against X26.

**Whether a P26 prototype should come next.** Yes, the step 3 part, in phase 18. The node-check point has existed since RW2 with no user, and X26 already finds three gaps in it (X26 section 3, "The prototype hook"):
- A node check does not know whether its node is inside an unevaluated operand.
- Section 7 does not say how a node check's result at substitution is classed.
- No point removes `operator,` from the grammar or checks a declaration.

A user at step 3 tests the first gap now, while step 3 is still open, and the design of the other two can follow before steps 4 and 5 build on the point. The prototype does not measure P26's impact on programs. Every fold over the comma sits in a template, and the 178 programs need steps 4 to 8. Its value is a test of the hook with a second design and a second kind of point, and an independent count of 7 test flips against X26's table.

The prototype needs three things first. Section 7 answers the three gaps above, with the context flag in step 3 and the substitution class and the declaration check named for steps 5 and 4. X26's 16 new tests (X26 section 2) live in `reports/clean-room/X26/tests/` with `X26:new-n` IDs. X24's 19 tests live in `reports/clean-room/X24/tests/`, so the two prototypes run their own tests under the flag.

**After P26.** P30's prototype extends `p22.rs` at the same two points, at step 3, if the machine's `Break` can name a block. P24 needs a point for built-in candidates. P29's step 3 part would use the node-check point on an equality node with a string literal operand. P27 (X25) needs the run override that section 10 holds open.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Answer X26-1 to X26-5 and RV11-R-6, with matrix probes of X26's 7 host-only claims (EDG included); X26-6 as a writer check within the input rules | 1.6, 1.8 | 1 writer session with probes | low | a cast to `void` and a discardable term that later designs can cite; the base of P26's prototype and of a proposal on expression statements |
| 2 | Section 7 answers X26's three hook points; the prototype tests of X24 and X26 beside their reports; then the step 3 part of a P26 prototype through the node-check point | 1.9 | a design edit; 1 writer step for the tests; one module within the rewrite item | low | the node-check point with a user before step 3 closes; the second measured design |
| 3 | Propose X27 on P30, with its prototype at step 3 after the design | 1.5, 1.6, 1.7 | 1 reader session, 1 repair session; one module | low | the first composed design (P30 on P22); a ready proposal measured in the phase it is written |
| 4 | A refresh note for X24, then notes for X11, X6, X9, X3 and X25, and a refresh check in `propose_check.py` | 1.1: 185 rows; 1.3: 54 rows in 15 reports | 1 reader session, a small tool change | low | X24's DEP-30 choice and its prototype test list; reuse of the done designs |
| 5 | The portfolio step: P22, P25, P26, P29 and P30 lines, a prototype column, P24's point and its scan, and a new entry from X26's alternatives | 1.7, 1.9 | 1 short designer step | low | the choice of each later exercise and prototype |
| 6 | The records in the present: `exercises.md` sections 4 and 8, `TEMPLATE-MODEL.md` section 8, `DESIGN-rewrite.md` section 10, the row 7 count | 1.2 | a writer step | low | records that later items read |
| 7 | The RV11 rows with design reach: RV11-R-5 with F7-2, RV11-R-3 | 1.4, 1.8 | part of item 1's writer session | low | X25's and X3's base; the marker of P24 |
| 8 | The packet rows that reach proposals: row 4a's "LSUP-10, UNI-14", Q-0430 and Q-0442; row 4b's Q-0426; row 9's Q-0412; row 1's Q-0447, Q-0449 and Q-0452 | section 5 | maintainer decisions | low | X25's base, X22's marker, P24 |
| 9 | An index of what a test of validity can see | 1.1: 52 of 559 | 2 writer sessions | medium | section 4.3 of every proposal |

### Item 1. X26's rows

**What it is.** A clean-room writer probes X26's 7 host-only claims on the matrix, EDG included, and answers:
- X26-1: UNEV-P2 and UNEV-2 say that a discarded class temporary ends with its full expression (VC-6). COMMA-1's test already shows the order.
- X26-2: the list of discarded-value expressions in `decltype.md` gains the `for` iteration expression, and LOOP-3 links to it.
- X26-3: one rule states the no-effect warning for every discarded-value expression, and COMMA-5 and STMT-4 link to it. The Evidence lists the forms each family warns about, from the matrix run of X26's `w1.cpp` forms.
- X26-4: a glossary entry "discarded-value expression", linked from COMMA-1.
- X26-5: PRIM-28 cites OPR-6 for a class operand, and the Examples add a fold over an overloaded comma.
- X26-6: a writer lists, within `documentation/clean-room.md`, whether a library type in scope declares `operator,`. The answer is a ledger Record.
- RV11-R-6: the JMP-14 sentence on a discarded statement names its doubt in a Note, and Q-0451's line stays.

**Evidence.** Sections 1.6 and 1.8. X26-1 is the only `wrong` finding of the round.

**Cost.** One writer session with probes.

**Risk.** Low. A probe that disagrees with a host claim becomes a finding against X26.

**What it unblocks.** X26:new-1 then builds on stated terms, and a later proposal on expression statements starts from one warning rule.

### Item 2. The hook's gaps and the P26 prototype at step 3

**What it is.**
- `DESIGN-rewrite.md` section 7 states what context a node check sees (at least whether the node is inside an unevaluated operand), how a node check's result at substitution is classed (an error, never a substitution failure, unless the design says otherwise), and which point serves a design that removes a grammar alternative or rejects a declaration (a nonterminal point that removes, or a node check on declarations). The build table gives each its step.
- A clean-room writer writes X24's 19 tests and X26's 16 tests beside their reports, from each report's section 2, with `Xn:new-k` IDs.
- The rewrite writes `src/proposals/p26.rs` with the X26:new-1 check on built-in comma nodes and the unevaluated exemption. The diff prints its base and lists the tests whose status changes by direction.

**Evidence.** Section 1.9.

**Cost.** A design edit, a writer step for 35 tests, and one module.

**Risk.** Low. If step 3 work stops, the design edit and the tests still hold.

**What it unblocks.** The node-check point has a user before step 3 closes. The step 4 part (class operands, the declaration check) and the step 5 part (folds, substitution) follow with their steps.

### Item 3. X27 on P30

**What it is.** A Propose exercise on P30, the next proposal after X26. The design settles what the entry leaves open: which of nested loops inside a labeled block a `break` with that label leaves, how FN-7's new last item and ATTR-7 count a `break` that ends a block, whether a labeled `continue` may name a block, and whether the label may mark an `if` or a `try` block. It amends X24:new-1, so the change table names X24's rows as its base, and section 4.1 lists X24's prototype test `break-label-on-block.cpp`, which P30 turns from reject to accept. P30 changes no valid program under P22 (X24 section 1.8), so sections 4.3 and 4.4 are short.

I rank P30 above P24 for these reasons:
- Its prototype fits points that exist. It extends `p22.rs` at the nonterminal and `Ext` node points, at step 3, so the phase can measure the design it writes.
- It is the first composed design. Its diff runs with P22 and P30 together, against X24's tests and its own.
- It needs no maintainer decision. P24's marker waits on Q-0412 (row 9) and must not clash with X12's `final`, and ENUM-10 sits in row 4a.
- P24's prototype needs a point for built-in candidates, which none of the five offers, so a P24 design written now waits at least for step 4 and a new point.

P24 reaches less-read pages (section 1.5). It should take the exercise after X27, and its entry should first name the point it needs.

**Evidence.** Sections 1.5 to 1.7.

**Cost.** One reader session, one repair session, and one module.

**Risk.** Low. The pages are well read, so the exercise may find fewer text gaps than P24 would.

**What it unblocks.** The first measure of how two designs compose, and the second prototype at the `Ext` node point.

### Item 4. Refresh notes and a refresh check

Round 12 item 3, with phase 17's rows. First `reports/clean-room/X24/X24-refresh.md`:
- The `goto`, `return` and throw parts of X24's OBJ-3, LOOP-3, FN-7, JMP-7 and JMP-14 amendments are now text (b511838d). Each amendment row says what it still adds for a labeled jump.
- X24's DEP-30 row takes the alternative of section 1.8: the conditions of X24:new-1 and X24:new-2 join DEP-30's list for a template with no specialization, and a discarded statement keeps the rejection, as JMP-14 does. `break-label-template-never-instantiated.cpp` leaves the prototype test list, or becomes a discarded-statement test.
- The prototype's registry line names the refresh note as its base.

Then rebase notes for X11, X6, X9, X3 and X25 (section 1.3), and the round 14 list (X19, X18, X14, X8, X7, X20). `tools/propose_check.py` reads a note beside a report, counts its rows as "noted", and names the commit that changed each row. My `changed_since.py` does the diff part.

### Item 5. The portfolio step

- P22: the last column lists the refresh note and the 19 prototype tests, and drops "The marker of the target".
- P25: the rating line says done (X23).
- P26: the section gives 178 programs, 10 fold files, 0 programs changed, with X26's `scan_comma.py` as the base. The last column lists X26-1 to X26-6 and the three hook points.
- P29: 24 lines in 9 files, with `strlit_scan.py` as the base, and the user-suffix case of `r12-sso.cpp`.
- P30: drops "It needs X24-1 answered first", names FN-7's new last item, and names X24:new-1 and `break-label-on-block.cpp` as the rule and test it reverses.
- P24: names the point it needs (built-in candidates for a marked enumeration), `r8-config.cpp` and `enum-operator.cpp` as the files with a user `operator|`, and RV11-R-3 if the marker is an attribute.
- A prototype column: the points each proposal needs and its earliest build step (section 1.9).
- A new entry, P31, an expression statement that must be a discardable operand. X26 section 1.9 leaves the step to a later proposal. It turns STMT-4 into a rule, changes existing behavior, and needs item 1's answers to X26-2 to X26-4. Its step 3 prototype uses the node-check point on expression statements. With it, one proposal besides P24 stays ready after X27.

### Item 6. The records in the present

- `exercises.md` section 4 names `reports/clean-room/roadmap-7/portfolio.md`, and its prose ends at the latest round (X26 on P26). Section 8 lists X22 to X26 in order.
- `TEMPLATE-MODEL.md` section 8 and `DESIGN-rewrite.md` section 10 drop RV10-R-1 and RV10-R-3, which are `commit` (c0461b86, b511838d).
- `pending.md` row 7's plan takes its count from a tool (2863 `reject` tests, about 8590 compilations on three compilers).

### Item 7. The RV11 rows with design reach

- RV11-R-5 with F7-2: OBJ-2 and UNI-13 say that the class member's trivial default constructor must not be deleted, and a test holds the struct-member row of Q-0447.
- RV11-R-3: ATTR-19's place table gains a row for an attribute before a using-directive.

### Item 8. The packet rows that reach proposals

Section 5 gives the rows. Under "follow the compilers", rows 1 and 4a keep Q-0447, Q-0449, Q-0452, the item "LSUP-10, UNI-14", Q-0430 and Q-0442 at the answers that X25, X3 and X24 meet, with no test changed.

### Item 9. An index of what a test of validity can see

Round 10 item 9. 52 of 559 `reject` rules have an `accept` or `run` test that uses `requires`. X26 section 4.3 shows how much work a proposal spends on this question without an index.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them.
- **A prototype in the old front end.** The old front end goes at step 9.
- **P26's step 4 and step 5 parts in phase 18,** unless the rewrite reaches those steps. Each waits for its point.
- **A P24 prototype.** No point offers built-in candidates.
- **A run override point.** Only X25 and older done designs (X1, X3, X4) need it. Section 10 holds it.
- **P23b's exercise.** P23b needs a matching rule, Q-0411, Q-0412, Q-0426 and the CNS-13 strength.
- **P28's exercise** until its entry chooses where the marker attaches.
- **P8b** until the maintainer decides rows 2 and 6.
- **Other pair exercises.** X14 with X25 waits for item 4's notes. P30 on P22 is item 3.
- **Rules or `:since:` fields for later versions.** Row 7's probe makes an index only.
- **A source-reader designer** (Q-0116). Twenty-six exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2).
- **Full re-runs of old Propose reports.** Item 4's notes cover the changes that reach a design.

## 4. Process changes

1. **A repair names the done designs its text change reaches.** Rounds 8 to 14 process change, taken in part. F7 named X24. The RV10 repair 88d14ea2 changed OBJ-2, JMP-7, JMP-14 and DEP-30 with no word on the 17 rows in 11 reports that name them. Item 4's tool can print the list for the commit message.
2. **Prototype tests come before the prototype.** Round 14 process change 4, not taken. The P22 prototype checked X24's examples in unit tests only. Section 7 states that a prototype's diff includes the tests beside its report, and the build step that writes a prototype first checks that the directory exists.
3. **A prototype's report says which change-table rows its diff reached.** P22's diff reached 2 of 33 rows through `tests/`. The count shows how much of a design the harness checked and how much rests on unit tests.
4. **A hook gap that a Propose report names gets a ledger row.** X26 section 3 names three gaps in the prototype hook, and none has a row in either ledger. No phase item tracks them.
5. **A count in the portfolio comes with its script and base.** Still not taken. P26 says 126 programs, and P29 says 18 lines in 6 files.
6. **Keep the exercise records current.** Section 4's prose names the latest round, and section 8 lists the reports in order.
7. **The rewrite documents state the present.** `TEMPLATE-MODEL.md` section 8 and `DESIGN-rewrite.md` section 10 drop a review row once its points are closed.
8. **The stop rule stays.** The count covers X24 to X26, and X27 when it runs.
9. **Keep the designer's view in round 16** in this form. Rerun the counts of sections 1.1, 1.3, 1.5 and 1.8, the diff of changed rows against each done design, and the table of section 1.9 against the hook as built, with each prototype's diff and the rows it reached.

## 5. The decision packet from a designer's view

The packet in `documentation/decisions/pending.md` gives each row's questions and an upper bound on the tests whose expectation changes. The order below follows the number of done designs and proposals that each row reaches.

| Order | Row | What it settles | Tests whose expectation changes | Proposals and done designs it reaches |
|---|---|---|---|---|
| 1 | 4a, rule item "LSUP-10, UNI-14", and Q-0430, Q-0442 | whether `std::construct_at` may build inside a member that is not active in constant evaluation; the typed step over bytes (ADD-14, STG-28, STG-29) | 0 if the compilers; the row's bounds are 232 for its 19 rule items and 36 for its questions | X25:new-1 and X25's LSUP-10, UNI-14, STG-28 and ADD-14 rows; X9's STG-28 row; X14 through STG-16 |
| 2 | 1 | 61 questions, among them Q-0447 (OBJ-2), Q-0449 (CE-3, CE-22), Q-0452 (OBJ-3 in constant evaluation), Q-0425 (UNI-14), Q-0420 (DED-29), Q-0446 (OPR-P3) | 0 (152 tests name them) | X25, X3, X9 and X11 through Q-0447 and Q-0449; X24 and P30 through Q-0452; X19 through Q-0446; P23a and P23b through DED-29 |
| 3 | 4b, Q-0426 | whether a constraint check runs when a candidate that is not a template matches exactly (CNS-11, CPY-15) | 0 if the compilers; the row's bound is 53 | X22 and P23b, X9, and every design that adds a constraint |
| 4 | 9, Q-0412 | which effects a standard attribute may have | 0 under either option | P23a (X22), P23b, P24 and P28 choose a marker by Q-0412. X25 rejected an attribute marker under it |
| 5 | 4a, rule item TARG-16 | a braced list as a template argument | 0 if the compilers | X7's TARG-16 row; RV11-R-7 adds the scalar case |
| 6 | 4b, Q-0063 | whether a floating literal out of range is not valid (PRIM-15, OPR-8) | 0 if the compilers | X19's floating width suffixes |
| 7 | 4a, rule items DEP-15 with DEP-P4, and NEW-P5 | the current instantiation through a static member; the second search of `new` | 0 if the compilers | X22's exemption; X23's mirror case (NEW-30) |
| 8 | 4a, Q-0407 | ENUM-10's type for an enumerator from another enumeration | 0 | P24, X20 |
| 9 | 7 | Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 tests name them; the probe changes no expectation | the 17 Propose reports that name Q-0007, X24-8 and X26-6 through row 7, and the version line ("C++23 only") of every later Propose report |
| 10 | 9, Q-0183 and Q-0186 | a cost model; judgments from outside Tidal | 0 | section 4 and the outside-knowledge line of every Propose report (X26-6) |
| 11 | 2 and 6 | Q-0322; Q-0343 and Q-0041 | 0 for row 2; at most 5 for row 6 | P8b. Q-0041 is JMP-6, which F7 changed for X24-2; a labeled jump enters no scope, so X24 and P30 do not depend on it |
| 12 | 10 | 17 questions that Tidal answers as its own choice | 0 (33 tests name them) | Q-0020, which X4 and X6 cite |

Q-0451 (JMP-14, DEP-30) waits for a review and a principle, in no packet row. X24's refresh note (item 4) rests on its answer.

The request on the record asks for rows 1, 4a and 10. Under "follow the compilers", it reaches X25 in three places (Q-0425 and Q-0447 in row 1, the item "LSUP-10, UNI-14" in row 4a), X3 through Q-0447 and Q-0449, and X24 through Q-0452. Rows 4b and 9 reach the most designs after those and are not in the request. I found no proposal that rows 3, 5 and 8 reach.

## Host-only claims

None. I ran no compiler and no front end. Every behavior in this report comes from Tidal's files and the reports they hold.

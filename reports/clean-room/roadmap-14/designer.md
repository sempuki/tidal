# Roadmap, round 14: what to do after phase 16, from a language designer's view

Date: 2026-11-11. Decision 0029, with decisions 0037, 0039, 0060, 0062, 0063 and 0064. Role: clean-room reader, language-designer persona (`designer`). Inputs: AGENTS.md and CLAUDE.md; my round 13 report `reports/clean-room/roadmap-13/designer.md` (the model for this form) and its scratch scripts; `reports/clean-room/roadmap-7/portfolio.md` (P22 to P29, with P22 done by X24); `documentation/` (decisions 0060 to 0064, `decisions/pending.md` with its decision packet, the row 4 table and the row 7 plan, the status page, the roadmap's phase 16 table, and sections 4, 8 and 9 of `exercises.md`); `git log b73a7d2d..HEAD` with every commit body; `frontend/DESIGN-rewrite.md` (sections 5, 7 and 10 in full) and `frontend/TEMPLATE-MODEL.md` section 8; `reports/clean-room/rust-frontend/rewrite-marks.tsv`; the X24 report with its scan; `reports/clean-room/rust-frontend/report-rw1.md` and `report-27.md`; the RV9 gap review and the read of the RV9 repair diff in `reports/review/` (2026-11-11), at their result tables and their points; `reports/clean-room/ledger.md` and the front-end ledger's state column; the change-table rows of X1, X3, X6, X9 to X11, X19, X20 and X25 that phase 16 reached; `tests/statements/jumps/break-with-label.cpp`, the `for` heads of `tests/programs/roadmap/r13-intrusive.cpp`, and `tests/` headers for the counts; and `tools/propose_check.py` plain, with `--corpus`, and with `--strict`. Counts come from read-only Python scripts in `reports/clean-room/roadmap-14/designer-scratch/scripts/`, run on this tree and on copies of `b73a7d2d` (the tree that round 13 read, with the specification of `a6a2b16c`) and `1cc1696e` (the tree that the RV9 review read), made with `git archive` outside the repository. Their outputs are beside the scripts. I did not read the other round 14 reports, `inputs/`, `build/exercise-keys/` or the sources under `frontend/`. I did not run `tools/run_tests.py`, `tools/frontend_check.py`, the front end or a compiler. Statements from outside Tidal's files are marked "general knowledge". Commit read: `f77bbe05`.

## 1. The state from a designer's view

### 1.1 Counts at f77bbe05

| Measure | Round 13 (a6a2b16c) | Now (f77bbe05) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1700 | 1700 |
| `required` / `reject` / `undefined` | 900 / 559 / 57 | 900 / 559 / 57 |
| `impl-defined` / `recommended` / `no-diagnostic` | 52 / 43 / 37 | 52 / 43 / 37 |
| `unspecified` / `conditionally-supported` / `withdrawn` | 32 / 5 / 15 | 32 / 5 / 15 |
| Rules and principles whose text changed in the phase (my diff, `:tests:` left out) | 36 (28 rules, 8 principles), none added | 18 (14 rules, 4 principles), none added |
| Of these, changed after the gap review read the tree | 20 after RV8 | 10 (9 rules, 1 principle) after RV9 (1cc1696e), the list of the close commit |
| Test files with a `tidal-expect` line | 7015 | 7073 |
| Of these, `reject` | 2824 | 2840 |
| Of these, `link-fail` / with `-m32` | 46 / 76 | 46 / 76 |
| Program corpus (`propose_check.py --corpus`) | 152 programs (99 Write, 53 sketches), 8 multi-unit, 175 files, 20156 lines | 166 programs (99 Write, 67 sketches), 8 multi-unit, 189 files, 23005 lines |
| Questions | 446 | 449 |
| Provisional / settled / compiler defects | 197 / 236 / 13 | 200 / 236 / 13 |
| Open questions in the packet's rows 1 to 9 | 119 of 197 (60%) | 122 of 200 (61%) |
| Propose reports | 24 | 25 (X24) |
| `propose_check.py`: conflicts | 31 (10 withdrawn, 4 class, 17 new-id) | 31 (10 withdrawn, 4 class, 17 new-id) |
| `propose_check.py`: rows changed since their report's commit | 161 | 168 |
| Change-table rows whose rule text changed in the phase (my diff) | 16, in 9 reports | 19, in 9 reports |
| `required` rules whose text contains "not valid" | 75 | 76 (CPT-28 added) |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 51 of 559 | 51 of 559 |
| Old front end, full pass / passes its check | 6895 / 6926 of 7015 | 6967 / 6990 of 7073 |
| Completion line of decisions 0060 and 0064 | 80 fail their check (32 marked, 48 toward the gate), 5 of 152 program tests do not run | 74 fail their check (36 marked, 38 counted), 0 of 166 program tests do not run, 9 out of scope |
| Marks file by model section | 32: 4, 6, 11, 4, 7 | 36: 4, 6, 13, 6, 7 |
| The rewrite's lines | none | `next: unit` 88, `next: lex` 7191/7191, `next: parse` 6475/6913, `next: owned` 0 |
| Exercise ledger rows | 1097; 16 `open` | 1129; 1030 `commit`, 48 `text`, 33 `question`, 3 `out of scope`, 15 `open`, 0 `planned` |
| Front-end ledger rows `open` | none | 14 (G166 to G179) |

Phase 16 added no rule ID and changed no class. The 7 new changed rows of `propose_check.py` are all in X25 (UNI-14, CE-21, LSUP-10, OBJ-1, STG-P2, STG-6, OBJ-2). X24's 33 rows have 0 conflicts and 0 changed rows, though 4 commits changed `specification/` after its commit read (`5f0ffdd9`). `propose_check.py --strict` exits 1 on the whole set, from the 31 old conflicts.

### 1.2 What phase 16 did with the round 13 designer items

Decision 0064 took round 13 items 1, 2, 3, 7 and 8, and the P22 part of item 5. It deferred refresh notes and the index of what a test of validity can see.

| Round 13 item | Phase 16 | State |
|---|---|---|
| 1. The rewrite with the hook in its build order | RW (8f487785) | done. The driver takes `--proposal ID` and refuses an ID that the empty registry lacks (step 1). A unit test registers P22's syntax part at `jump_stmt` and gets an `Ext` node, and a TR-12 stop without it (step 2). Section 7 has the node-check point before step 3 and states the diff's base. Section 10 holds the run override as open. `conform` refuses until the machine moves at step 3 |
| 2. X24 on P22, then the stop-rule count | E (bc9c38f2) | done. 33 rows (3 new, 12 amended, 1 withdrawn, 17 state effect), 2 tests from `reject` to `accept`, 0 of 152 programs change, 8 findings, 11 host-only claims. No record holds the stop-rule count. Section 1.7 gives it |
| 3. The X25 rows and host-only claims | F6 (b3f17197), FE (aa8c8e60) | done. X25-1 to X25-4, X25-6 and X25-7 by commit, X25-5 by Q-0448, X25-9 as text. RV9 then read F6's LSUP-10 and UNI-14 against (RV9-7), and row 4a gained the rule item "LSUP-10, UNI-14" |
| 4. Refresh notes and a refresh check | none | deferred again. 168 changed rows, and phase 16 reached X25 in 7 rows |
| 5. The portfolio step | E, in part | P22 is marked done (X24). The P22 row's last column still says "The marker of the target. Version marking". P26 still says 126 programs and 7 files. P25's section still says "Rating: ready". No prototype column. P29 has no word on the user-suffix case |
| 6. The packet rows that reach proposals | none | the request of 2026-11-10 has no answer. Row 7's plan still says about 2640 tests and 7900 compilations (2840 `reject` tests now, so about 8520) |
| 7. The records in the present | R10 (54d51c9b), RW | section 8 of `exercises.md` gained the X24 and X25 rows, in the order X24, X25, X23, X22. Section 4's prose still names `reports/clean-room/roadmap/designer.md` and ends at "Round 7 ... with X22 on P23". `DESIGN-rewrite.md` sections 5 and 10 follow the marks file of the decision. `TEMPLATE-MODEL.md` section 8 has an RV9 row that says "decision 0064's item RV9 reviews them", and RV9 is done |
| 8. P4-7, P4-5, P4-3 and P4-4 | F6, FE | done. OPR-8 gives the numeric form the value of the literal without its suffix as `unsigned long long` or `long double` (P4-7) |
| 9. An index of what a test of validity can see | none | deferred. 51 of 559 `reject` rules. CPT-28's new sentence adds one case (section 1.6) |

Round 13's process changes:
- Taken: process change 2 (decision 0064 and section 7 say that the prototype follows X24). Process change 3 in the design text. Process change 5 for section 8. Process change 7 (a Propose ran, X24).
- Not taken: process change 1. Phase 16 changed 19 change-table rows in 9 reports. F6's body names the X25 findings it answers and no other done design. The RV9 repair 8d33fcb1 changed 10 rows in 4 reports (X3, X9, X11, X25), and its body names none. Process change 4 (P26 still says 126). Process change 6 for `TEMPLATE-MODEL.md` section 8 and for section 7's worked example, which predates X24 (section 1.10).

### 1.3 Phase 16 text that reaches a done design

My diff of `b73a7d2d` against `f77bbe05` finds 19 change-table rows in 9 reports whose rule or principle text changed:

| Report | Rows | Effect on the design |
|---|---|---|
| X25 (P27) | UNI-14, LSUP-10, OBJ-1, OBJ-2, STG-6, STG-P2, CE-21 | F6 wrote four X25 findings into the text: LSUP-10's third place (X25-1), OBJ-1's array row (X25-2), the link from STG-P2 to the storage-form table (X25-6), and CE-21's ended part with no value (X25-7). X25's LSUP-10 row is now close to the text. RV9-7 reads that third place against, and RV10-R-6 finds that the record leaves out "in the same evaluation", which X25's own row has. UNI-14 now says that the enclosing member does not become active at run time (X25-4), so a read through its name is undefined (UNI-6). X25's UNI-14 row ("at run time the new element is reachable through the member name") now reverses a stated rule. X25 section 4.2 must list that case as undefined made defined. OBJ-2 gained the item on an assignment after an explicit destructor call. RV10-R-3 finds that the item fails for a scalar union member named directly, and holds for an element of an array member, which is X25's case. STG-6's "all or part of the storage" fits X25's row |
| X3 (P5) | CE-21, CE-22, OBJ-2 | X3 narrows "has no value" in CE-21 and CE-22 to the opt-out. CE-21 now has a second source of no value, a part whose lifetime ended (Q-0447). CE-22 now says that a trivial copy of a union reads no part of its active member, with a limit for a `mutable` active member (Q-0448, Q-0449). Zero initialization does not revive an ended part, so X3's narrowing must keep the ended case and must state the union copy as unchanged |
| X9 (P8) | OBJ-1, OBJ-2, STG-6 | X9 ends a lifetime when a relocation completes. OBJ-1's new array row gives destruction as the end, so X9 adds relocation to that row too. OBJ-2's new item and STG-6 fit X9's rows |
| X19 (P1) | OPR-8 | OPR-8 now gives the numeric form a value (P4-7), and a `long double` form out of range follows PRIM-15 (F6-1), which RV9-6 reads against (row 4b, Q-0063). X19's width suffixes leave the search before OPR-8's forms. A floating width suffix meets PRIM-15 and Q-0063 directly. Rebase note |
| X1 (P3), X10 (P11) | CE-P3 | CE-P3 gained the union-copy sentences. X1 amends its order paragraph and X10 its exception paragraph. Rebase notes |
| X6 (P10) | CE-21 | state effect. An element value with an ended part has no value. Fits |
| X11 (P14) | OBJ-2 | state effect. Fits |
| X20 (P17) | TYN-10 | TYN-10 now accepts an attribute or alignment specifier in a specialization with no body (RV9-R-4). X20's row on a class key with an explicit alias name fits |

X25 holds 7 of the 19 rows. Four of its findings became text, and its two central rows (LSUP-10 and UNI-14) now meet a rule item in row 4a.

### 1.4 The 10 rules and principles changed after RV9, ranked by design reach

| Rank | IDs | Done designs and proposals reached | Why a designer reads them |
|---|---|---|---|
| 1 | UNI-14, OBJ-1, OBJ-2, STG-6 | X25 (all four rows), X9 (OBJ-1, OBJ-2, STG-6), X3 and X11 (OBJ-2); P27, P8b | The lifetime of an element in a union member, and the condition of RV10-R-3 |
| 2 | CE-22 | X3's row; X25 names it | The union copy in constant evaluation. Q-0449 waits for CE-P3 |
| 3 | CPT-28 | none by row | Section 4.3 of every proposal: a requires parameter type that is not valid gives false (RV10-R-5) |
| 4 | INST-1, INST-30, INST-P4, FN-12 | X7 names INST-1, X6 INST-30, X22 INST-P4 | When a class used in a member body is made (RV10-R-1, RV10-R-2) |

The next gap review reads all 10. For a designer, the review should also read OBJ-2's new item beside UNI-5, where RV10-R-3 finds two results for one program.

### 1.5 Pages the Propose exercises have reached

All 85 pages are reached by rule or principle IDs cited in X1 to X25. My scan finds the same two pages read by one exercise only:

| Page | Exercise | Commits to the page since that report | Proposal that aims at it |
|---|---|---|---|
| `declarations/namespaces.md` | X15 | 7 | none open |
| `expressions/comma.md` | X1 | 5 | P26 |

X24 adds a reading of `statements/jumps.md`, `loops.md`, `blocks.md` and `switch.md`. 15 pages are reached by two exercises.

### 1.6 What a designer still cannot state

**When a statement's own variables end after a jump.** New (X24-1). OBJ-3 ends objects when control leaves their block, and LOOP-3 names only `break` for a `for` initializer. No rule gives the end of an initializer variable, a condition variable, or the hidden variables of a range-based `for` (RFOR-4) after a `goto`, a `return` or a throw. The host compilers end them after the body's objects, statement by statement. P22's run part rests on the answer.

**Whether a jump in a template needs its diagnostic at the definition.** New (X24-3). JMP-14 and DEP-30 do not say whether a `goto` to a missing label in a template that is never instantiated must be rejected. The host compilers split. X24 checks its labeled jumps at the definition, as both hosts check a `break` outside a loop.

**Whether a constraint check runs.** Unchanged. Q-0426 waits in row 4b (CNS-11, CPY-15). X9, X22 and P23b depend on it.

**Whether a typed step over bytes is defined.** Unchanged. RV9 named Q-0430 with no new reading. Q-0430 and Q-0442 sit in row 4a.

**Whether `std::construct_at` may build inside a member that is not active during constant evaluation.** Stated provisionally. LSUP-10 now names the place (X25-1), and RV9-7 reads it against. Row 4a holds the rule item "LSUP-10, UNI-14", with every newest family at the matrix's answer. X25:new-1 rests on it.

**What a standard attribute may do.** Q-0412 (row 9) unchanged. X24 avoided the question by using labels.

**Whether a later version has taken the space.** X24 gives "C++23 only" and records outside knowledge as X24-8. Clang's diagnostic on the JMP-17 tests names a later C mode. Row 7's count is stale (section 1.2).

**How much code a change touches.** The hook exists for syntax only. The impact diff needs `tidal-next conform`, which refuses until build step 3.

**Which programs change meaning without a diagnostic.** 51 of 559 `reject` rules. CPT-28 now says that a requires parameter type that is not valid makes the expression false (RV9-14), and RV10-R-5 doubts that reach outside a concept. A proposal that makes a type valid or not valid in a requires parameter list changes a test of validity.

**How two changes compose.** No pair exercise has run.

**What a change costs.** Q-0183 keeps option 1.

These round 13 points are now answered: whether the member name reaches an element built inside a member that is not active (UNI-14, no, at run time and in constant evaluation), what value OPR-8's numeric form receives (P4-7), what a union's trivial copy reads (CE-22, Q-0448), and whether an ended element has a value (CE-21, Q-0447).

### 1.7 The portfolio and the stop rule

| # | Proposal | Round 13 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P22 | Labeled `break` and `continue` | ready | done (X24) | X24 settled the target, a `continue` to a `switch`, and the lambda body | X24-1 to X24-7. The prototype (section 1.10). The row's last column still names a marker, which X24 does not use |
| P26 | No overloaded comma, checked left operand | ready | ready | none | No design. By my scan of the 166 programs, none declares `operator,`, and 8 files hold a fold over the comma (unchanged). 3 programs have a comma in a `for` increment: `r5-hashmap-kept-pointer.cpp`, and `r13-intrusive.cpp` with its second try, new in phase 16, whose `++a, ++b` steps two pointers. 2 fold an assignment (`r7-registry/registry.h`, `r12-bigint-second-try.cpp`). So 0 to 5 programs become invalid, by whether an increment and an assignment count. The entry says 126 programs and 7 files |
| P24 | Flag enumerations | ready | ready | | The marker under Q-0412 (row 9). ENUM-10 in row 4a (Q-0407). Its prototype needs built-in candidates, which no point of section 7 offers |
| P29 | A pointer comparison with a string literal operand is not valid | partial | partial | | 24 lines in 9 program files compare with a string literal, each a class object (unchanged). `r12-sso.cpp` compares with `"x"_sv`, and the entry does not say whether a literal with a user suffix counts |
| P23b | Expressions checked against requirements | partial | partial | | No matching rule (X22-8, `out of scope` in the ledger), Q-0411, Q-0412, Q-0426, the strength of CNS-13 |
| P28 | Arithmetic that wraps on request | partial | partial | | Where the marker attaches |
| P8b | Relocation by bytes | partial | partial | | Row 2 (Q-0322), row 6 (Q-0343, CPY-19) |

Totals in the file: 2 ready (P24, P26), 3 partial (P23b, P28, P29), 4 done from P22 on (P22, P23a, P25, P27), 0 blocked. After an exercise on P26, one proposal stays ready.

**The stop rule.** Round 7's rule stops new proposals if a round of three exercises closes fewer than five findings each and finds no wrong text. By the ledger, over X22 to X25:

| Exercise | Findings | `commit` | `text` | `question` | `out of scope` | `open` | Kind "wrong" |
|---|---|---|---|---|---|---|---|
| X22 | 9 | 7 | 0 | 1 | 1 | 0 | 0 |
| X23 | 7 | 7 | 0 | 0 | 0 | 0 | 1 (X23-1) |
| X24 | 8 | 0 | 0 | 0 | 0 | 8 | 0 |
| X25 | 9 | 7 | 1 | 1 | 0 | 0 | 2 (X25-1, X25-3) |

X22 and X23 each closed seven, and X23 and X25 found wrong text, so the rule does not stop the lane. X24's eight rows are all open. Seven of them ask for text (X24-8 is outside knowledge), so X24 can close seven when a phase answers them.

### 1.8 Questions and principles

Phase 16 opened 3 questions (Q-0447 to Q-0449) and settled none. Q-0449 waits for a principle (CE-P3). All three sit in the reach of done designs: Q-0447 and Q-0449 in row 1 of the packet, and all three on CE-21 or CE-22, which X3 and X25 amend.

Principles that a Propose finding or design met, now:

| Principle | Finding or design | State |
|---|---|---|
| JMP-P1, JMP-P2 | X24 amends JMP-P1's table; X24-2 on JMP-P2's row "Forward past an initialization" | unchanged since X24 read them |
| ATTR-P1 | X24 amends the place table ("an identifier label"); X14-7, X22-6 | Q-0412, row 9 |
| CE-P3 | X1 and X10 amend it; F6 added the union copy; Q-0449 waits for it | X1's and X10's amendments rebase |
| STG-P2 | X25 amends it; X25-6 | X25-6 done; the table now has a label and STG-P2 links it |
| LSUP-P3 | X25 names it | F6 links the storage-form table |
| INST-P4 | X22 names it | RV9 added a diagram; RV10-R-1 |
| OPR-P3, PRIM-P1 | X19 | Q-0446, Q-0436 and Q-0440 wait, unchanged |
| LOOP-P1 | P22's model | unchanged |

### 1.9 The 15 open ledger rows, ranked by design reach

The exercise ledger has 15 `open` rows (X24-1 to X24-8, RV10-R-1 to RV10-R-3, RV10-R-5 to RV10-R-8). The front-end ledger has 14 (G166 to G179).

| Rank | Rows | What they reach | Who acts |
|---|---|---|---|
| 1 | X24-1 | OBJ-3, LOOP-3, JMP-9, RFOR-4. The run part of the P22 prototype, which section 7 says needs no code | writer, with the `d1.cpp` order on the matrix |
| 2 | X24-3 | JMP-14, DEP-30. The class of X24's checks in a template, and the step 5 part of the prototype | writer, with probes and a question for the host split |
| 3 | X24-4 | FN-7. If FN-7 counts a jump by its target, its page check serves P22 with no override | writer |
| 4 | X24-6, X24-7 | JMP-7, JMP-14, SW-3. The function boundary of the label search, which the prototype's check uses | writer, with `l1.cpp` and `l2.cpp` as tests |
| 5 | X24-2 | JMP-6, JMP-P2. Which jumps JMP-6 covers. X24's test `continue-label-past-later-declaration.cpp` rests on it | writer, with `k1.cpp` |
| 6 | RV10-R-3 | OBJ-2 beside UNI-5 and UNI-14. X25, X3, X9; Q-0447 | writer |
| 7 | RV10-R-6, RV10-R-7 | LSUP-10's record and OBJ-1's array row. X25's base | writer |
| 8 | RV10-R-5 | CPT-28. Section 4.3 of every proposal | writer |
| 9 | X24-5 | a glossary entry for label (JMP-P1, STMT-5, ATTR-P1) | writer |
| 10 | RV10-R-1, RV10-R-2 | INST-30, FN-12. No change-table row | writer |
| 11 | RV10-R-8, X24-8 | no design reach | writer for RV10-R-8; none for X24-8 |

X24's eleven host-only claims rest on g++ 16 and clang++ 22. No writer has probed them on the matrix, and EDG did not run. The `d1.cpp` order (X24-1) and the `t1.cpp` split (X24-3) are the two that X24's design leans on.

G166 to G179 are grammar gaps that 215 tests needing a tree and 223 reject tests meet. None names a statements page except G174 (`if` with a structured binding in its init-statement). The parse count lists each test. I did not run `--next`, so I cannot say whether every jumps, loops, blocks and switch test parses.

### 1.10 The rewrite and the P22 prototype

Decision 0064 says that the rewrite's first prototype follows X24. Steps 1 and 2 are in place, and the two points they own exist. The prototype's other needs land at steps 3 to 5. I set X24's 33 rows against the point and the build step each needs:

| X24 rows | What the prototype does | Point | Build step | Tests under the flag |
|---|---|---|---|---|
| `jump_stmt` (Syntax), JMP-17 (withdrawn) | the alternative at `jump_stmt` | nonterminals | 2 (its unit test exists) | the 2 JMP-17 tests, `reject` to `accept` |
| X24:new-1, X24:new-2, JMP-5, JMP-7, JMP-14 | the check of the `Ext` node: the label of the jump's own function, the marked statement after further labels and attribute lists, an enclosing loop or `switch`, a loop for `continue` | `Ext` nodes, and the proposal's own rule IDs | 3 | 7: `continue-label-names-switch`, `break-label-not-enclosing`, `break-label-on-block`, `break-label-at-block-end`, `break-label-from-lambda`, `break-label-same-name-in-lambda`, `break-label-chain` |
| X24:new-3, JMP-1, JMP-2, LOOP-1, LOOP-2, LOOP-3, LOOP-9, SW-1, CE-17, JMP-6 | none. The check gives an ordinary `Break { target }` or `Continue { target }`, and the machine and constant evaluation run it | none | 3 | 5: `break-label-outer-loop-constant`, `continue-label-outer-loop-run`, `break-label-switch-from-loop`, `continue-label-do-checks-condition`, `continue-label-past-later-declaration` |
| FN-7, ATTR-7 | none, if the page checks read the jump's target | none | 3 | 1: `flow-off-end-labeled-break-warns` |
| OBJ-3, JMP-9, RFOR-4 | none, if the machine ends each statement's own variables on any jump (X24-1) | none | 4, since the order tests print from destructors | 2: the two `ends-inner-statements` run tests |
| TRY-15, IF-17 | none | none | 4 (handlers) | 1: `break-label-out-of-handler` |
| DEP-30 (state effect), X24 section 1.7 | X24:new-1 and X24:new-2 join the checks of a template's definition | definition checks | 5 | 1: `break-label-template-never-instantiated` |
| JMP-P1, JMP-P2, JMP-8, SW-3, STMT-5, TR-12, ATTR-P1, the glossary entry | text only | none | none | none |

So the prototype holds code in two places, the alternative and the check, plus one line at step 5. 15 of its 19 tests can run at the end of step 3, 3 more at step 4, and the last at step 5. By my scan, the 190 tests under `tests/statements/` for jumps, loops, blocks, switch and range-based `for` cite rules of step 3's chapters only, and 67 of them declare a class, so the step 3 base holds about two thirds of them.

**When.** The prototype's code lands at the end of step 3, when the statements and expressions pages, constant evaluation and the `Ext` node point exist and `conform` runs. The prototype is step 3's last measure, so that the `Ext` node point has a user when it lands. Steps 4 and 5 add the four remaining tests to the base with no change to `p22.rs` except the definition-check line. If phase 17 does not finish step 3, the prototype waits, and phase 17 only prepares its inputs (items 1 and 2).

**How.** Section 7 needs five changes before the code is written:
- **The worked example follows X24.** Section 7's table uses `Xn:new-1` to `Xn:new-3` and puts the same-function condition in the third rule. X24 puts the same-function condition in its JMP-14 amendment and makes X24:new-3 the run-time rule (X24 section 3 says so). The table takes X24's 33 rows as above.
- **The proposal's own rule IDs arrive with the `Ext` node point.** Section 7 gives "add rules with the exercise's IDs" to the rule-override point, which lands at step 4. P22's check names X24:new-1 and X24:new-2 at step 3. The registry of a design's own IDs moves to step 3, and kind changes of existing rules stay at step 4.
- **The kinds of the new IDs come from the change table.** `diag` reads each rule's kind from `specification/`. X24:new-1 and X24:new-2 are `reject` and X24:new-3 is `required` only in X24's table. The harness reads them from the same table that `propose_check.py --strict` reads, so the prototype cannot drift from the design.
- **"Run. None." becomes a condition.** It holds once the machine ends statement-scope variables on any jump (X24-1). A run test of the `goto` form under `tests/` checks the machine without the flag.
- **The prototype tests exist before the code.** X24 section 2 names 17 new tests and 2 changed ones. They live in `reports/clean-room/X24/tests/` with `X24:new-n` IDs, as section 7 says, and no such directory exists yet. A clean-room writer writes them from X24's examples. No compiler accepts the form in C++ mode, so the matrix checks only their `goto` and flag twins.

**What the diff tests.** With and without `--proposal`, the diff should change exactly the 2 JMP-17 tests in `tests/` and pass the 19 tests beside X24. Any other status change is a finding against X24. The base matters for the programs. By my scan, 4 of the 166 programs use no class, template, try block, union or `#include` (`W13/clink.cpp`, `r4-bits.cpp`, `r4-hello.cpp`, `r4-lambda-generic.cpp`). X24's "no program changes" can be measured for few programs before step 8. `W3/nested.cpp` declares a struct, so it enters the base at step 4. Its migration (X24 section 1.9) makes a good prototype test: the `goto` form rewritten with a labeled `break`, in a `static_assert`.

**After P22.** The node-check point is the first work of step 3 and has no design to test against. P26 is its named user (section 7). A Propose design of P26 written in phase 17 gives that point a unit test, as P22's syntax part gave the nonterminal point one. P26's full prototype needs step 5, because every fold over the comma sits in a template. P24 needs built-in candidates, which no point offers. P27 (X25) needs the run override that section 10 holds open.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Answer X24-1 to X24-7 before step 3 writes the jumps, loops and objects pages, with matrix probes of X24's 11 host-only claims (EDG included) and a question for the X24-3 split; then an X24 refresh note | 1.6, 1.9 | 1 writer session with probes; 1 short reader step | low | page modules that follow the final text; a prototype with no override and no run code |
| 2 | Section 7 brought to X24, and the P22 prototype as step 3's last measure: own rule IDs with the `Ext` node point, kinds from the change table, the 19 tests beside X24, the diff with its base | 1.10 | a design edit; 1 writer step for the tests; part of the rewrite item | low | the first measured impact; a test of X24's sections 4.3 and 4.4 |
| 3 | Propose X26 on P26, early in phase 17, so the node-check point has a design to test | 1.5, 1.7, 1.10 | 1 reader session, 1 repair session | medium for the impact count | the node-check point's unit test; the second reading of `comma.md` |
| 4 | The portfolio step: P26 at 166 programs with its scripts, P22's and P25's lines, a prototype column, P29's user-suffix case, and a new entry from X24's alternatives | 1.7 | 1 short designer step | low | the choice of each later prototype; a ready proposal after X26 |
| 5 | The RV10 rows with design reach: RV10-R-3, RV10-R-6, RV10-R-7, RV10-R-5 | 1.4, 1.9 | part of item 1's writer session | low | X25's and X3's base; section 4.3 of later proposals |
| 6 | Refresh notes for X25, X3, X9, X19 and the earlier lists, and a refresh check | 1.1: 168 rows; 1.3: 19 rows in 9 reports | 1 reader session, a small tool change | low | reuse of the done designs |
| 7 | The records in the present: `exercises.md` sections 4 and 8, `TEMPLATE-MODEL.md` section 8, the row 7 count | 1.2 | a writer step | low | records that later items read |
| 8 | The packet rows that reach proposals: row 4a's "LSUP-10, UNI-14", Q-0430 and Q-0442; row 4b's Q-0426 and Q-0063; row 9's Q-0412; row 1's Q-0447 and Q-0449 | section 5 | maintainer decisions | low | X25's base, X22's marker, P24 |
| 9 | An index of what a test of validity can see | 1.1: 51 of 559; 1.6: CPT-28 | 2 writer sessions | medium | section 4.3 of every proposal |

### Item 1. X24's rows before step 3

**What it is.** A clean-room writer probes X24's 11 host-only claims on the matrix, EDG included, and answers:
- X24-1: OBJ-3 counts a statement's own scope as a block and ends its variables after the body's objects. LOOP-3 reads "after any jump out of the statement". A run test with the order of `d1.cpp` for a `goto`, and one for a `return`.
- X24-2: JMP-6 names the jumps it covers (a `goto`, and a transfer to a `case` or `default` label, SW-5), and says that `break`, `continue` and `return` only leave scopes. `k1.cpp` as a test.
- X24-3: JMP-14 or DEP-30 gives the class of a `goto` to a missing label in a template, with a question for the host split. The `t1.cpp` cases as tests.
- X24-4: FN-7 counts a jump by its target, so a `break` of an inner loop or `switch` does not leave the outer loop, and a `goto` out of it does.
- X24-5: a glossary entry for label. ATTR-P1 says "an identifier label".
- X24-6 and X24-7: JMP-7 and JMP-14 name both the lambda body and a member function of a local class, as SW-3 does. `l1.cpp` and `l2.cpp` as tests.

Then a reader writes `reports/clean-room/X24/X24-refresh.md`. The text change moves the `goto`, `return` and throw parts of X24's OBJ-3, LOOP-3, FN-7, JMP-7 and JMP-14 amendments into the current text. The note says what each amendment still adds for a labeled jump.

**Evidence.** Sections 1.6 and 1.9. Section 7 relies on the machine for the run part, and X24 shows that the text the machine follows has a gap.

**Cost.** One writer session with probes. The note is a short step.

**Risk.** Low. A probe that disagrees with the host claims becomes a finding against X24.

**What it unblocks.** Step 3 writes the statement and object pages from the final text. The P22 prototype then needs no rule override and no run code.

### Item 2. Section 7 and the P22 prototype

**What it is.** The five changes of section 1.10 to `DESIGN-rewrite.md` section 7, and the build table's step 3 row ending with "the P22 prototype (X24)". A clean-room writer writes the 19 tests under `reports/clean-room/X24/tests/` from X24's section 2, with `W3/nested.cpp`'s labeled form as a 20th. The rewrite then writes `src/proposals/p22.rs` at the end of step 3. The diff prints its base, lists the tests whose status changes by direction, and names any test outside the two JMP-17 tests. The step 4 and step 5 parts follow with their steps.

**Evidence.** Section 1.10.

**Cost.** A design edit and a writer step now. The prototype's code is two parts and a line, within the rewrite item.

**Risk.** Low. If step 3 does not end in phase 17, the tests and the design edit still hold.

**What it unblocks.** The first design that the harness measures. A diff that differs from X24's "Tests affected" column becomes the first finding of its kind.

### Item 3. X26 on P26

**What it is.** A Propose exercise on P26. The design settles what the entry leaves open: whether a call of an operator function and a call through a pointer count as calls, whether a cast to `void` keeps a value, whether an assignment or an increment counts as a left operand with no value, whether a fold over the comma is checked per element (PRIM-27), and when a class result has a value. It meets COMMA-P1, COMMA-P3, COMMA-1, COMMA-4, COMMA-5, COMMA-7, OPR-P2, OPR-7, PRIM-27, STMT-1 and STMT-4. P26 changes existing behavior, so sections 4.3 and 4.4 are full. My scan gives the starting count: 8 fold files, 3 programs with a comma in a `for` increment, 0 to 5 programs invalid. The design's change table names the node kinds its check reads, which the node-check point's unit test then uses.

**Evidence.** Sections 1.5, 1.7 and 1.10. `comma.md` has one reading (X1) and 5 commits since.

**Cost.** One reader session, one repair session.

**Risk.** Medium for the count. The text scan cannot tell a pointer increment from a class one, and the two `r13-intrusive` programs step pointers.

**What it unblocks.** The node-check point lands with a test. P26's prototype can follow at step 5.

### Item 4. The portfolio step

- P22: the last column lists X24-1 to X24-7 and the prototype, and drops "the marker of the target".
- P25: the rating line says done (X23).
- P26: 166 programs, 8 fold files, 3 programs with a comma in a `for` increment, 0 to 5 invalid. The scan scripts and outputs go beside the file, and the entry names X26 when it runs.
- P29: the user-suffix case of `r12-sso.cpp`, at 24 lines in 9 files.
- A prototype column: the points each proposal needs and its earliest build step (section 1.10).
- A new entry, P30, a label on a block that a `break` can leave. X24 section 1.8 defers it and states that it changes no program valid under P22. It needs X24-1 and the FN-7 change of item 1. With it, one proposal besides P24 stays ready after X26.

### Item 5. The RV10 rows with design reach

- RV10-R-3: OBJ-2's item gains the condition for a scalar union member named by member access, as UNI-5 and UNI-14 give. X25's case (an element of an array member) stays as the item says.
- RV10-R-6: the LSUP-10 Note and the row 4 item say "began in the same evaluation", as X25's row does.
- RV10-R-7: OBJ-1's array row names the default constructor.
- RV10-R-5: CPT-28 states its reach for a variable template and an `if constexpr` condition, or a Note records the doubt.

### Item 6. Refresh notes and a refresh check

Round 12 item 3, with phase 16's rows. X25 (seven rows, and the reversal of UNI-14 at run time), X3 (CE-21, CE-22), X9 (OBJ-1's array row and relocation), X19 (OPR-8's value and Q-0063), and rebase notes for X1, X10, X6, X11 and X20. The round 13 list stays (X19, X18, X9, X25, X14, X8, X7, X20, X6). `tools/propose_check.py` reads a note beside a report, counts its rows as "noted", and names the commit that changed each row. My `changed_since.py` does the diff part.

### Item 7. The records in the present

- `exercises.md` section 4 names `reports/clean-room/roadmap-7/portfolio.md`, and its prose ends at the latest round (X24 on P22). Section 8 lists the reports in order.
- `TEMPLATE-MODEL.md` section 8 replaces the RV9 row with the next review's blocks and RV10-R-1 and RV10-R-2.
- `pending.md` row 7's plan takes its count from a tool (2840 `reject` tests, about 8520 compilations).

### Item 8. The packet rows that reach proposals

Section 5 gives the rows. Under "follow the compilers", rows 1 and 4a keep Q-0447, Q-0449, the item "LSUP-10, UNI-14", Q-0430 and Q-0442 at the answers that X25 and X3 meet, with no test changed.

### Item 9. An index of what a test of validity can see

Round 10 item 9. 51 of 559 `reject` rules have an `accept` or `run` test that uses `requires`. CPT-28's parameter-list case goes in the index.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them.
- **A prototype in the old front end.** The old front end goes at step 9.
- **Prototypes of P26, P29, P24 or X25 in phase 17.** Each needs a point past step 3 or one that section 7 lacks.
- **A run override point.** Only X25 and older done designs (X1, X3, X4) need it. Section 10 holds it.
- **P23b's exercise.** P23b needs a matching rule, Q-0411, Q-0412, Q-0426 and the CNS-13 strength.
- **P28's exercise** until its entry chooses where the marker attaches.
- **P8b** until the maintainer decides rows 2 and 6.
- **Pair exercises.** X14 with X25 waits for item 6's notes. P30 with X24 waits for item 1.
- **Rules or `:since:` fields for later versions.** Row 7's probe makes an index only.
- **A source-reader designer** (Q-0116). Twenty-five exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2).
- **Full re-runs of old Propose reports.** Item 6's notes cover the changes that reach a design.

## 4. Process changes

1. **A repair names the done designs its text change reaches.** Rounds 8 to 13 process change, still not taken. Phase 16 changed 19 rows in 9 reports, and the RV9 repair alone changed 10 rows in 4 reports with no word. Item 6's tool can print the list for the commit message.
2. **A prototype takes its rules from the change table.** The registry line names the design and its commit read (P22, X24, `5f0ffdd9` or its refresh note). The new IDs and their kinds come from the table that `propose_check.py --strict` reads.
3. **A diff that differs from the change table is a finding.** The prototype's diff lists the tests whose status changes. A test missing from the table's "Tests affected" column, or a listed test that does not change, gets a ledger row against the report.
4. **Prototype tests come before the prototype.** A writer writes the report's test list beside the report before the code, as the change table comes before the design's code.
5. **A count in the portfolio comes with its script and base.** Still not taken. P26 says 126 programs where the corpus has 166.
6. **Keep the exercise records current.** Section 4's prose names the latest round, and section 8 lists the reports in order.
7. **The rewrite documents state the present.** Section 7's worked example follows the design it names, and `TEMPLATE-MODEL.md` section 8 drops a review once it is done.
8. **The stop rule stays.** The count covers X22 to X25, and X26 when it runs.
9. **Keep the designer's view in round 15** in this form. Rerun the counts of sections 1.1, 1.3, 1.5 and 1.9, the diff of changed rows against each done design, and the table of section 1.10 against the hook as built, with the prototype's diff once it exists.

## 5. The decision packet from a designer's view

The packet in `documentation/decisions/pending.md` gives each row's questions and an upper bound on the tests whose expectation changes. The order below follows the number of done designs and proposals that each row reaches.

| Order | Row | What it settles | Tests whose expectation changes | Proposals and done designs it reaches |
|---|---|---|---|---|
| 1 | 4a, rule item "LSUP-10, UNI-14", and Q-0430, Q-0442 | whether `std::construct_at` may build inside a member that is not active in constant evaluation; the typed step over bytes (ADD-14, STG-28, STG-29) | 0 if the compilers; the row's bounds are 228 for its rule items and 36 for its questions | X25:new-1 and X25's LSUP-10, UNI-14, STG-28 and ADD-14 rows; X9's STG-28 row; X14 through STG-16 |
| 2 | 4b, Q-0426 | whether a constraint check runs when a candidate that is not a template matches exactly (CNS-11, CPY-15) | 0 if the compilers; the row's bound is 53 | X22 and P23b, X9, and every design that adds a constraint |
| 3 | 1 | 60 questions, among them Q-0425 (UNI-14), Q-0447 (CE-21, OBJ-2), Q-0449 (CE-22), Q-0420 (DED-29), Q-0421 (LSUP-11), Q-0446 (OPR-P3) | 0 (144 tests name them) | X25 and X3 through Q-0425, Q-0447 and Q-0449; X19 through Q-0446; P23a and P23b through DED-29 |
| 4 | 9, Q-0412 | which effects a standard attribute may have | 0 under either option | P23a (X22), P23b, P24 and P28 choose a marker by Q-0412. X25 rejected an attribute marker under it |
| 5 | 4b, Q-0063 | whether a floating literal out of range, also through the `long double` form, is not valid (PRIM-15, OPR-8) | 0 if the compilers | X19's floating width suffixes |
| 6 | 4a, rule items DEP-15 with DEP-P4, and NEW-P5 | the current instantiation through a static member; the second search of `new` | 0 if the compilers | X22's exemption; X23's mirror case (NEW-30) |
| 7 | 4a, Q-0407 | ENUM-10's type for an enumerator from another enumeration | 0 | P24, X20 |
| 8 | 7 | Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 tests name them; the probe changes no expectation | the 17 Propose reports that name Q-0007, X24-8 through row 7, and the version line of P24, P26 and P29 |
| 9 | 9, Q-0183 and Q-0186 | a cost model; judgments from outside Tidal | 0 | section 4 and the outside-knowledge line of every Propose report (X24-8) |
| 10 | 2 and 6 | Q-0322; Q-0343 and Q-0041 | 0 for row 2; at most 5 for row 6 | P8b. Q-0041 is JMP-6, which X24-2 reads; a labeled jump enters no scope, so X24 does not depend on it |
| 11 | 10 | 17 questions that Tidal answers as its own choice | 0 (32 tests name them) | Q-0020, which X4 and X6 cite |

The request on the record asks for rows 1, 4a and 10. Under "follow the compilers", it reaches X25 in three places (Q-0425 and Q-0447 in row 1, the item "LSUP-10, UNI-14" in row 4a) and X3 through Q-0447 and Q-0449. Rows 4b and 9 reach the most designs after those and are not in the request. I found no proposal that rows 3, 5 and 8 reach.

## Host-only claims

None. I ran no compiler and no front end. Every behavior in this report comes from Tidal's files and the reports they hold.

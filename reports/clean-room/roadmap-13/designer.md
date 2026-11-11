# Roadmap, round 13: what to do after phase 15, from a language designer's view

Date: 2026-11-11. Decision 0029, with decisions 0037, 0039, 0060, 0061, 0062 and 0063. Role: clean-room reader, language-designer persona (`designer`). Inputs: AGENTS.md and CLAUDE.md; my round 12 report `reports/clean-room/roadmap-12/designer.md` (the model for this form) and its scratch scripts; `reports/clean-room/roadmap-7/portfolio.md` (P22 to P29, with P27 done by X25); `documentation/` (decisions 0060 to 0063, `decisions/pending.md` with its decision packet and its table of requests, the status page, the roadmap's phase 15 table, and sections 4, 8 and 9 of `exercises.md`); `git log 51bd5060..HEAD` with every commit body; `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md` and `frontend/DESIGN-rewrite.md` (section 7 in full); `reports/clean-room/rust-frontend/rewrite-marks.tsv`; the X25 and P4 reports; `reports/clean-room/rust-frontend/report-26.md` (sections 8 and 9); the four 2026-11-11 records of RV8 in `reports/review/` (the read of the RV7 repair diff, the two phase 15 gap reviews, and the read of the RV8 repair diff), at their result tables and the points that reach a design; `reports/clean-room/ledger.md`; the change tables of X6 to X9, X14, X18 to X20 and X25 at the rows that phase 15 changed; targeted reads of JMP-17 and its two tests; `tests/` headers for the counts; and `tools/propose_check.py` plain, with `--corpus`, and with `--strict`. Counts come from read-only Python scripts in `reports/clean-room/roadmap-13/designer-scratch/scripts/`, run on this tree and on copies of `bd87b27c` (the tree round 12 read) and `e7e972d6` (the tree the RV8 reviews read) made with `git archive` outside the repository. Their outputs are beside the scripts. I did not read the other round 13 reports, `inputs/`, `build/exercise-keys/` or the sources under `frontend/`. I did not run `tools/run_tests.py`, `tools/question_index.py --check`, the front end or a compiler. Statements from outside Tidal's files are marked "general knowledge". Commit read: `a6a2b16c`.

## 1. The state from a designer's view

### 1.1 Counts at a6a2b16c

| Measure | Round 12 (bd87b27c) | Now (a6a2b16c) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1700 | 1700 |
| `required` / `reject` / `undefined` | 900 / 559 / 57 | 900 / 559 / 57 |
| `impl-defined` / `recommended` / `no-diagnostic` | 52 / 43 / 37 | 52 / 43 / 37 |
| `unspecified` / `conditionally-supported` / `withdrawn` | 32 / 5 / 15 | 32 / 5 / 15 |
| Rules and principles whose text changed in the phase (my diff, `:tests:` left out) | 38, and DEP-40 and LOOP-9 added | 36 (28 rules, 8 principles), none added |
| Of these, changed after the gap review read the tree | 20 after RV7 | 20 (17 rules, 3 principles) after RV8 (e7e972d6), the list of the close commit |
| Test files with a `tidal-expect` line | 6947 | 7015 |
| Of these, `reject` | 2808 | 2824 |
| Of these, `link-fail` | 46 | 46 |
| Test files with `-m32` in `tidal-flags` | 76 | 76 |
| Program corpus (`propose_check.py --corpus`) | 138 programs (94 Write, 44 sketches), 8 multi-unit, 161 files, 17358 lines | 152 programs (99 Write, 53 sketches), 8 multi-unit, 175 files, 20156 lines |
| Questions | 434 | 446 |
| Provisional / settled / compiler defects | 185 / 236 / 13 | 197 / 236 / 13 |
| Open questions in the packet's rows 1 to 9 | 114 of 185 (62%) | 119 of 197 (60%) |
| Propose reports | 23 | 24 (X25) |
| `propose_check.py`: conflicts | 31 (10 withdrawn, 4 class, 17 new-id) | 31 (10 withdrawn, 4 class, 17 new-id) |
| `propose_check.py`: rows changed since their report's commit | 155 | 161 |
| Change-table rows whose rule text changed in the phase (my diff) | 27, in 12 reports | 16, in 9 reports |
| `required` rules whose text contains "not valid" | 75 | 75 |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 49 of 559 | 51 of 559 |
| Front end, full pass / passes its check | 6804 / 6831 of 6947 | 6895 / 6926 of 7015 |
| Completion line of decisions 0060, 0062 and 0063 | 107 fail their check, 3 of 138 program tests do not run, 9 out of scope | 80 fail their check (32 marked, 48 toward the gate), 5 of 152 program tests do not run, 9 out of scope |
| Marks file (`rewrite-marks.tsv`) by model section | none | 32: 4, 6, 11, 4, 7 for sections 1 to 5 |
| Exercise ledger rows | 1044; 26 `open` | 1097; 1004 `commit`, 16 `open`, 44 `text`, 30 `question`, 3 `out of scope`, 0 `planned` |
| Front-end ledger rows `open` | G164, G165 | none |

Phase 15 added no rule ID and changed no class. The 6 new changed rows of `propose_check.py` are STG-16 in X14, OPR-16 and PRIM-2 in X19, TYN-10 in X20, and STG-28 and ADD-14 in X25. X1 to X12 name no commit, so the tool counts no changed row for them. My diff finds 10 more rows there (section 1.3).

`propose_check.py --strict` exits 1 on the whole set. The X25 report alone reads 31 rows with 0 conflicts. Two of its rows changed after its commit (13e3f3c0), in the RV8 repair d6fd9cea.

### 1.2 What phase 15 did with the round 12 designer items

Phase 15 took round 12 items 1, 6 and 8. Decision 0063 deferred X24, the refresh notes, the shared scanner, the pair exercise and the index of visibility by name.

| Round 12 item | Phase 15 | State |
|---|---|---|
| 1. Portfolio step, then X25 on P27 | R9 (1f248814), E (bcf30b72) | done. P22 cites LOOP-9, P23b lists the answers and blockers, P27 cites ADD-14, STG-29, Q-0430 and G164, and P29 has an entry rated partial. X25 ran on P27 with 31 rows, 3 new rules, 1 test from `reject` to `accept`, and 9 findings. P26's entry now gives 7 files with a fold "of the 126 programs", a base that was 138 at the scan and is 152 now (section 1.7) |
| 2. X24 on P26, then the stop-rule count | none | deferred by decision 0063. The stop-rule count now covers X22, X23 and X25. X25 has 1 finding closed (X25-8) and 7 open |
| 3. Refresh notes and a refresh check | none | deferred. `X22/X19-refresh.md` is still the one note, and X19 is the done design that phase 15 reached most (section 1.3) |
| 4. A shared scanner and the order count | none | deferred. The prototype hook of `DESIGN-rewrite.md` section 7 gives the impact count from the harness, which can replace the scanner's syntax query (section 1.10) |
| 5. The packet rows that reach proposals | none | the request of 2026-11-10 has no answer. Row 4 gained Q-0439, Q-0442, and the rule items CTAD-17 and CTAD-16, and lost "ADD-14, ADD-P6". Row 7's plan still says about 2640 tests and 7900 compilations. There are 2824 `reject` tests, so about 8472 compilations |
| 6. The open rows with design reach | F5 (232ac618) | done. G164 (STG-28), G165 (VIRT-13), I7-10 and I7-5 (CTAD-P1, CTAD-5), P3-6 (INST-5, Q-0441), I7-7 and I7-8 (CTAD-14). RV8-A-1 then read STG-28's element-`k` case against, and the RV8 repair kept the compilers' answer (Q-0430, row 4a) |
| 7. Pair exercise X14 with X23 | none | deferred |
| 8. A prototype hook in the rewrite's design document | RD (14ebbde6), GC (17077248) | done. Section 7 of `DESIGN-rewrite.md` gives four extension points, a `--proposal` flag and P22 as the worked example. GC brought `TEMPLATE-MODEL.md` to the present. `DESIGN-rewrite.md` itself is already behind in two places (section 1.10) |
| 9. An index of what a test of validity can see | none | deferred. 51 of 559 `reject` rules. X25's section 4.3 is an instance |

Round 12's process changes:
- Taken: process change 5 (a Propose ran, X25). Process change 4 for `TEMPLATE-MODEL.md` (GC). Process change 2 in part, since P26's entry names the scan, but the scripts stay in `roadmap-12/designer-scratch/` and the entry's base number is wrong.
- Not taken: process change 1. Phase 15 changed 16 change-table rows in 9 reports, and no phase 15 commit body names a done design that its text change reaches. Process change 3: section 4 of `exercises.md` still opens with the portfolio path `reports/clean-room/roadmap/designer.md`, and its prose still ends at "Round 7 ... with X22 on P23". Section 4's table gained X25 (X25-8), and section 8 has no X25 row. Process change 6 has no check (75 rules).

### 1.3 Phase 15 text that reaches a done design

My diff of `bd87b27c` against `a6a2b16c` finds 16 change-table rows in 9 reports whose rule or principle text changed:

| Report | Rows | Effect on the design |
|---|---|---|
| X19 (P1) | OPR-8, OPR-16, OPR-P3, PRIM-2 | X19 amends OPR-8 and OPR-P3 so that a width suffix of the language never calls a literal operator, and states the effect on OPR-16 and PRIM-2. Phase 15 rewrote all four. OPR-8 now has a literal operator template with a pack of `char` and one with a class-type parameter for a string literal, and the numeric form wins over the template (Q-0435, Q-0446). OPR-16 rejects a numeric literal when only the raw form and the template exist. PRIM-2 says that a literal that calls a literal operator has no type list, and that its value must fit `unsigned long long` only for the numeric form (Q-0436, Q-0440). X19's amendment must say that a width suffix ends the search before OPR-8's choice among forms, and X19:new-2 must sit beside PRIM-2's new sentence. Q-0436 and Q-0440 wait for PRIM-P1, and Q-0446 for OPR-P3, a principle that X19 amends. P4-7 (open) asks what value the numeric form receives |
| X25 (P27) | STG-28, ADD-14 | STG-28 now states the cast to element `k` through a byte pointer as an exception to STG-16, and RV8-A-1 and the read of the RV8 repair read that case against (Q-0430). ADD-14 gained the integer-counterpart case (Q-0442). X25's rows ("the allocated array is the model"; "a `T*` to an element has the element type, so ADD-14 does not apply") still fit. X25's point that in-object storage needs no disputed step gains weight |
| X18 (P18) | VIRT-13, INST-5 | VIRT-13 now warns about a hiding function only when no derived function of that name overrides (G165). X18's row says that hiding without overriding stays a warning in a checked class. A checked class where one overload overrides and another hides now gets no warning from VIRT-13, so X18 must say whether the marker adds one. INST-5 now says that a member's parameter and return types need not be complete when the class is made. X18's row on member declarations fits |
| X9 (P8) | STG-28, VIRT-13 | X9's third warning of X9:new-12 is modeled on VIRT-13 and must say which reading it copies. STG-28's new sentence does not reach X9's growable-array example, which does no byte arithmetic |
| X14 (P20) | STG-16, REF-2 | X14 amends STG-16 for an under-aligned member. STG-16 now names STG-28's exception, so X14's amendment rebases onto two exceptions. REF-2 now leaves out a bit-field rvalue. X14's row binds a `const` reference to a copy of an under-aligned glvalue, and fits |
| X8 (P16) | OPR-10 | X8 adds a bullet to OPR-10's list. The list now also names the two kinds of literal operator template. Rebase note only |
| X7 (P12) | CTAD-5 | CTAD-5 now says that only declared constructors give guides. X7's row on a constructor with a value-specialized parameter fits for a declared constructor |
| X20 (P17) | TYN-10 | TYN-10 now allows attributes after the class key only in a forward declaration (RM12-2). X20's row on a class key with the name of an explicit alias fits |
| X6 (P10) | RFOR-P1 | RFOR-P1 gained a paragraph on a sequence computed on demand. X6 amends the principle for an expansion statement. Rebase note only |

X19 holds 4 of the 16 rows, and 3 of the 20 blocks changed after RV8 (OPR-8, OPR-16, OPR-P3). X19 already has the one refresh note in the repository (`X22/X19-refresh.md`), which predates phase 15.

### 1.4 The 20 rules and principles changed after RV8, ranked by design reach

| Rank | IDs | Done designs and proposals reached | Why a designer reads them |
|---|---|---|---|
| 1 | OPR-8, OPR-16, OPR-10, OPR-P3 | X19 (three rows), X8 (OPR-10); X13 and X19 name OPR-10 in prose | The choice among literal operator forms, which X19's width suffixes must leave before it starts |
| 2 | STG-28, STG-16, ADD-14, ADD-11 | X25 (STG-28, ADD-14), X9 (STG-28), X14 (STG-16; STG-28 in prose); P27's motivation | The element-`k` case that RV8-A-1 reads against, and the integer counterpart of Q-0442 |
| 3 | INST-5, INST-1, INST-2, FN-12 | X18 (INST-5); X7 names INST-1 | When a member's types are made, which FN-12 now states for an in-class definition (P4-1) |
| 4 | REF-2 | X14 | Rebase note |
| 5 | TYN-10, TYN-6 | X20 (TYN-10); X12 and X20 name TYN-6 | Rebase note |
| 6 | DEP-P1, DEP-40 | X15 and X22 name DEP-P1; P23b | Which calls bind at the definition |
| 7 | UNI-P3 | none; P27's entry cited it in round 12 | |
| 8 | CTAD-18, DTOR-8 | none | |

The next gap review reads all 20. For a designer, the review should also read OPR-P3 beside X19's amendment of it. A source reader can say whether a suffix of the language and a user suffix are told apart before the choice among forms.

### 1.5 Pages the Propose exercises have reached

All 85 pages are reached by rule or principle IDs cited in X1 to X25. X25 reaches 11 pages, all reached before. My scan finds the same two pages read by one exercise only:

| Page | Exercise | Commits to the page since that report | Proposal that aims at it |
|---|---|---|---|
| `declarations/namespaces.md` | X15 | 7 | none open |
| `expressions/comma.md` | X1 | 5 | P26 |

`statements/loops.md` and `statements/blocks.md` are reached by two exercises each (X6 and X11, X9 and X11). P22 aims at both.

### 1.6 What a designer still cannot state

**Whether a constraint check runs.** Unchanged. CNS-11 skips the constraints of template candidates when a candidate that is not a template matches every argument with no conversion, and CPY-15 relies on the skip. Q-0426 waits in row 4b, and 4 tests name it. X9, X22 and P23b depend on the answer.

**Whether a typed step over bytes is defined.** Narrower. STG-28 now gives element `k` of an allocated array through a byte pointer, as an exception to STG-16. RV8-A-1 reads that case against, and so does the read of the RV8 repair. Element 0 with no byte arithmetic agrees. Q-0442 adds the step from a pointer to the matching signed or unsigned integer type. Both sit in row 4a, and 4 tests name Q-0430 or Q-0442. X25 gives in-object storage a form that needs neither step, so the dispute now bears on storage outside the object.

**Whether `std::construct_at` may build inside a member that is not active during constant evaluation.** New. X25-1 finds that LSUP-10's two places do not cover such a member, which UNI-14 accepts. RV8-A-8 keeps the same doubt from RV6-A-13. A clean-room reader and a source reader found the same gap from two sides. X25:new-1 and the LSUP-10 amendment depend on it.

**Whether the member name reaches an element built inside an array member that is not active.** New (X25-4). RV8-A-8 reads that such a member does not become active, at run time or in constant evaluation, which answers X25-4 for the current text. P27 changes that answer on purpose.

**What a standard attribute may do.** Q-0412 still holds a provisional answer (row 9). X25 rejected an attribute marker for P27 under it, so the question now reaches X25's alternatives as well as P22, P23a, P23b and P24.

**Whether a later version has taken the space.** 17 Propose reports name Q-0007. X25 gives the version line "C++23 only" and records its outside knowledge as X25-9. Row 7's count is stale (section 1.2).

**How much code a change touches.** The text scans still cannot tell a class operand from a pointer or an assignment from a call. `DESIGN-rewrite.md` section 7 measures impact as the tests whose status differs with and without `--proposal`. That count comes only when the rewrite parses and checks the pages a proposal touches (section 1.10).

**Which programs change meaning without a diagnostic.** 51 of 559 `reject` rules have an `accept` or `run` test that uses `requires`. X25's section 4.3 found one such case by probe (`a9.cpp`).

**How two changes compose.** No pair exercise has run. X14 and X25 both change storage rules (STG-16 and STG-28 for X14, UNI-14 and STG-P2 for X25).

**What a change costs.** No change. Q-0183 keeps option 1.

These points of round 12 are now answered: which element a `T*` cast from a byte offset reaches (STG-28, kept against a review), whether VIRT-13 concerns each hidden base function (G165), whether implicit constructors give guides (CTAD-5), and when a member's parameter and return types are made (INST-5, FN-12).

### 1.7 The portfolio

| # | Proposal | Round 12 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P27 | An in-object array whose elements start their lives one at a time | ready | done (X25) | X25 designed the union form | X25-1 to X25-7 are open. Q-0425 (row 1) keeps the answer that X25 changes for the array form |
| P22 | Labeled `break` and `continue` | ready | ready | `DESIGN-rewrite.md` section 7 takes P22 as the worked example of the hook | No Propose design. Section 7 picks answers to the three points that the entry leaves open (target, `continue` to a `switch`, a lambda) without a Propose report. JMP-17 has 2 tests. By my scan, 1 program leaves nested loops by `goto` (`W3/nested.cpp`) |
| P26 | No overloaded comma, checked left operand | ready | ready | none | X24 has not run. By my scan of the 152 programs, none declares `operator,`, and 8 files hold a fold over the comma. 6 fold calls (`r10-list.cpp`, `r10-list-first-try.cpp`, `r10-tuple.cpp`, `r10-tuple-second-try.cpp`, `r8-fmt.cpp`, `r9-vec.cpp`). 2 fold an assignment (`r7-registry/registry.h`, and `r12-bigint-second-try.cpp`, new in phase 15). `r5-hashmap-kept-pointer.cpp` has `++step` as a left operand in a `for` increment, twice. So 0 to 3 programs become invalid, by whether an increment and an assignment count. The entry says "of the 126 programs" and 7 files |
| P24 | Flag enumerations | ready | ready | | The marker under Q-0412 (row 9). ENUM-10 in row 4a (Q-0407) |
| P29 | A pointer comparison with a string literal operand is not valid | ready by my reading, not in the file | partial | R9 added the entry | By my scan, 24 lines in 9 program files compare with a string literal, and each compares a class object. `r12-sso.cpp` compares a `StrView` with `"x"_sv`, a literal with a user suffix, so the entry must say whether such a literal is a string literal operand. No program becomes invalid by my reading |
| P23b | Expressions checked against requirements | partial | partial | none | No matching rule (X22-8). Q-0411, Q-0412, Q-0426. The CNS-13 strength (RV8-B-1 adds a doubt on its scope) |
| P28 | Arithmetic that wraps on request | partial | partial | | The marker, and whether it applies to a type, an expression or an operator |
| P8b | Relocation by bytes | partial | partial | | Row 2 (Q-0322). Row 6 (Q-0343, CPY-19) |

Totals in the file: 3 ready (P22, P24, P26), 3 partial (P23b, P28, P29), 3 done from P22 on (P23a, P25, P27), 0 blocked. P26 is the one ready proposal that changes existing behavior.

X25's section 4.3 counted the programs that a defined form would serve, as my round 12 report asked (9 programs, 4 with byte rows and 5 with a union slot for each element). Its section 4.4 found one run case that becomes undefined, where I expected none. The design took a construct that is valid today and changed its rules, which is the cheaper path for a designer and a change of existing behavior (X25-8).

### 1.8 Questions and principles

Phase 15 opened 12 questions (Q-0435 to Q-0446) and settled none. Six of them wait for a principle or its use: Q-0436 and Q-0440 for PRIM-P1, Q-0437 for CTAD-P1, Q-0443 for CTAD-P3, Q-0444 for CTOR-P1, Q-0446 for OPR-P3, and Q-0435 for a use of OPR-P3. Q-0196 still waits for TPL-P3, a principle that does not exist. The share line is 60% for rows 1 to 9 and 68% with row 10.

Principles that a Propose finding or design met, now:

| Principle | Finding or design | State |
|---|---|---|
| ATTR-P1, ATTR-P2, ATTR-P4 | X14-7, X22-6; X25 section 1.6 | Q-0412, row 9. Bears on P22, P23a, P23b, P24 and X25's alternatives |
| OPR-P3 | X19 amends it | phase 15 widened it (Q-0435), and Q-0446 waits for it. X19's amendment rebases |
| PRIM-P1 | X19 through PRIM-2 | Q-0436 and Q-0440 wait for it |
| STG-P2 | X25 amends it; X25-6 | the storage-form table sits after the examples and STG-P2 does not cite it |
| UNI-P1 | X25 amends it | unchanged in phase 15 |
| DEP-P6, CNS-P3 | X22-4 | RV8-B-1 doubts the scope of "need not diagnose" |
| DEP-P4 | X22 (the exemption of the current instantiation) | row 4a rule item with DEP-15 |
| NEW-P5 | X23-1 | row 4a rule item |
| LOOP-P1 | P22's model | unchanged since phase 14 |

### 1.9 The 16 open ledger rows, ranked by design reach

The exercise ledger has 16 `open` rows (P3-9, P3-10, P4-1 to P4-5, P4-7, X25-1 to X25-7, X25-9). The front-end ledger has none.

| Rank | Rows | What they reach | Who acts |
|---|---|---|---|
| 1 | X25-1, X25-4 | LSUP-10 and UNI-14 for a construction inside a member that is not active. The base of X25:new-1. RV8-A-8 gives a reading for both | writer, with probes on the matrix |
| 2 | X25-5, X25-7 | CE-22 for a trivial copy of a union, and CE-21 for an ended element. X25:new-3 and X25's CE-21 amendment | writer, with probes |
| 3 | X25-2, X25-3 | OBJ-1 has no row for an array, and the glossary entry for a lifetime differs from OBJ-1. X25's OBJ-1 and glossary rows | writer |
| 4 | P4-7 | the value that OPR-8's numeric form receives. X19's literals | writer |
| 5 | X25-6 | STG-P2 does not cite the storage-form table. P27's motivation and P8b | writer |
| 6 | P4-5 | the `Maybe` example lacks a move constructor, and its sentence cites CPY-8 where CPY-5 applies. X9's relocation | writer |
| 7 | P4-3, P4-4 | examples of an adaptor over any sequence, and of a member call's result type without the library | writer |
| 8 | P4-1, P4-2, P3-9, P3-10 | front-end faults with tests | front end |
| 9 | X25-9 | outside knowledge; no rule rests on it | none |

X25's report lists eight host-only claims. No writer has probed them on the matrix, and EDG did not run. Q-0425's split (GCC 16 accepts, Clang 22 rejects) is the claim that X25:new-1 rests on.

### 1.10 The rewrite and Propose

The gate of decisions 0062 and 0063 holds. Front-end round 26 ended with 45 failing tests outside the marks file, and the completion line now reads 48 after P4. The status page says that the phase 16 decision adds the rewrite and starts from `DESIGN-rewrite.md`. For a designer, the rewrite matters because section 7 makes a prototype a module of its own, and because the harness then gives a proposal's impact as a count.

Section 7 gives four extension points: nonterminals, `Ext` nodes, rule overrides and definition checks. I set each proposal against them, and against the build step that first holds the pages the proposal touches:

| Proposal | Points it needs | Point it needs that section 7 lacks | Earliest build step | What the diff measures |
|---|---|---|---|---|
| P22 | a nonterminal at `jump_stmt`, an `Ext` check that gives `Break { target }`, new rule IDs | none | 2 for its syntax part, 3 in full | the 2 JMP-17 tests; no program changes |
| P26 | a rule override (COMMA-5) and new rule IDs | a check on each comma node and on an `operator,` declaration | 5, because every fold over the comma sits in a template | section 4.4 over 152 programs |
| P29 | new rule IDs | a check on each equality node, after overload resolution has chosen | 4, to tell a class operand from a pointer | section 4.4 over 24 lines |
| P24 | a nonterminal on the enumeration head | built-in candidates that a marked type adds to overload resolution | 4 | the tests that declare an `operator\|` |
| P28 | a nonterminal and an `Ext` node with its own run, if the marker is a form of expression | a run override, if the marker is on a type | 3 | none until the marker is chosen |
| P23a (X22) | a definition check and a rule override | none | 5 | the one test of X22's section 4.4 |
| P23b | a definition check | a matching rule, which the design must supply first | 5 | none yet |
| P27 (X25) | none of the four fit | a run override of UNI-14 and LSUP-10 in the machine and in constant evaluation | 4 | the one test of X25 section 4.1, and `a9.cpp` |

- **P22 is the one ready proposal that the four points cover in full.** It is also the earliest, since its pages are in step 3 and its syntax part can run at step 2.
- **The proposals that change existing behavior need a fifth point.** P26 and P29 add a rejection to an existing construct, which no rule override can express, because no rule's condition matches theirs. A "node check" point, owned by `sem`, would run a prototype's check on each typed node of a named kind after the page's own check. For a dependent node it would run again at substitution, as model section 3 runs the page's check (DEP-28). Section 7 forbids a prototype to change a page module, so the point must exist before step 3 writes the expression pages.
- **A done design that changes run-time rules needs a sixth point.** X25 changes what the machine does for `std::construct_at` in a union member. X1, X3 and X4 change evaluation order, initial values and lifetimes. A "run override" by rule ID would serve them. No ready proposal needs it, so it can stay an open point of section 10.
- **The impact count is a lower bound until step 8.** The diff with and without `--proposal` covers only the tests that `tidal-next` passes without the flag. Programs pass to the new front end in step 8. A Propose report that cites the diff must state its base.

Section 7 also decides P22's three open points (the target, a `continue` that names a `switch`, a label outside a lambda) and names three new rules. Those choices belong to a Propose design. The prototype should follow a Propose report's change table, so that the diff tests a reader's design.

`DESIGN-rewrite.md` is already behind the tree in two places. Section 5 and the build order count 30 marked tests (5 for section 2, 10 for section 3), and the marks file holds 32 (6 and 11). Section 10 says that the `forward_class` gap has no ledger row, and RM12-2 answered it in d6fd9cea. `TEMPLATE-MODEL.md` section 8 has a row "RV8 ... decision 0063's item RV8 reviews them", and RV8 is done. The build order names no step for the `--proposal` flag or for any of the four points.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | The rewrite item with the hook in its build order: the flag at step 1, the nonterminal point at step 2 with P22's syntax part as its check, a node-check point in the design before step 3, and the impact diff with its base | 1.10 | part of the rewrite item; a design edit of section 7 | low | the first prototype; measured impact for P26 and P29 |
| 2 | Propose X24 on P22, so that the prototype has a design to follow; then the stop-rule count over X22 to X25 | 1.7, 1.10 | 1 reader session, 1 repair session | low | the P22 prototype at step 3; the stop rule's first full count |
| 3 | The X25 rows and host-only claims: X25-1 to X25-7 with matrix probes, EDG included | 1.6, 1.9 | 1 writer session with probes | low | X25's base; LSUP-10 and UNI-14 that X25-1 and RV8-A-8 both doubt |
| 4 | Refresh notes for X19, X18, X9, X25, X14 and the round 12 list, and a refresh check that names the commit that changed each row | 1.1: 161 rows; 1.3: 16 rows in 9 reports | 1 reader session, a small tool change | low | reuse of the done designs |
| 5 | The portfolio step: a prototype column, P26's count at 152 programs, P29's user-suffix case, P22's link to X24 | 1.7, 1.10 | 1 short designer step | low | the choice of each later prototype |
| 6 | The packet rows that reach proposals: row 4b's Q-0426, row 4a's Q-0430 and Q-0442, row 9's Q-0412, and row 7 with its count from a tool | section 5 | maintainer decisions; 1 writer step for the count | low | X22's marker, X25's base, P22 and P24 |
| 7 | The exercise and rewrite records in the present: `exercises.md` sections 4 and 8, `DESIGN-rewrite.md` sections 5 and 10, `TEMPLATE-MODEL.md` section 8 | 1.2, 1.10 | a writer step | low | records that later items read |
| 8 | P4-7, P4-5, P4-3 and P4-4 | 1.9 | part of the writer's session for open rows | low | X19's literals; the examples port suites ask for |
| 9 | An index of what a test of validity can see | 1.1: 51 of 559 | 2 writer sessions | medium | section 4.3 of every proposal |

### Item 1. The rewrite item, with the hook in its build order

**What it is.** The phase 16 decision adds the rewrite, as the status page says. For Propose, the item adds the flag, the nonterminal point, a node-check point in the design, and the impact diff.
- **The flag at step 1.** The driver gets `--proposal <id>` and `src/proposals/mod.rs` with an empty registry. A flag with no prototype changes no result, and `frontend_check.py` can check that.
- **The nonterminal point at step 2.** Step 2 writes the `syntax` core and its table of nonterminals. P22's syntax part (`break` or `continue` with an identifier) is the unit test of the point. A `next: parse` line with `--proposal P22` shows the 2 JMP-17 tests giving a tree in place of the TR-12 stop.
- **A node-check point in the design.** Section 7 adds a fifth point before step 3 writes the expression pages (section 1.10). Section 10 records the run override as open, with X25 and X1 as its users.
- **The impact diff with its base.** `tidal-next conform` with and without the flag prints the base (the tests that pass without the flag), the tests whose status changes by direction, and the programs that become invalid or change their result. Prototype tests with IDs such as `X24:new-1` live beside their Propose report, since `tests/` cites only Tidal's rule IDs. The harness takes that directory under the flag.

**Evidence.** Section 1.10. P22 is the one ready proposal that the four points cover, and the build order names no step for any point.

**Cost.** Part of the rewrite item. The design edit is a writer step.

**Risk.** Low for the flag and the nonterminal point. A point added after step 3 would need edits to the pages it reaches, which section 7 forbids.

**What it unblocks.** The P22 prototype at step 3, and a measured section 4.4 for P26 and P29 later.

### Item 2. X24 on P22, then the stop-rule count

**What it is.** A Propose exercise on P22. The design states which statement a label names, whether `continue` may name a `switch`, what a label outside a lambda body does (JMP-5, JMP-7), and the destruction order for a labeled `break` out of nested blocks (JMP-9). It meets JMP-P1, JMP-1, JMP-2, JMP-5, JMP-9, JMP-17, LOOP-P1, LOOP-9, NAM-P4 and STMT-5. P22 changes no existing behavior, so section 4.4 should find no program that becomes invalid, and section 4.3 should count the programs that leave nested loops by a flag or a `goto` (`W3/nested.cpp` by my scan). The report's change table is the table the prototype implements. Section 7's three rules then take X24's IDs.

After X24, count the findings of X22 to X25 and the share that the text closed. X22 closed 7 of 9, X23 7 of 7, and X25 1 of 9 so far.

**Evidence.** Sections 1.7 and 1.10. Section 7 chose P22's open points with no Propose report.

**Cost.** One reader session, one repair session.

**Risk.** Low. If X24 chooses differently from section 7, the prototype follows X24, and the change is one file.

**What it unblocks.** The first prototype, and the stop rule's count. P26 keeps its place for the exercise after X24, when the node-check point and step 5 can measure its section 4.4.

### Item 3. The X25 rows and host-only claims

A writer probes X25's eight host-only claims on the matrix, with EDG in constant evaluation, and answers:
- X25-1: whether LSUP-10 names a member of a union whose lifetime began in the evaluation. RV8-A-8 and RV6-A-13 hold the same doubt from the reference side.
- X25-4: whether UNI-14 lets the member name reach the new element at run time. RV8-A-8 reads that the member does not become active.
- X25-5 and X25-7: what a union's trivial copy reads (CE-22), and an example under CE-21 for an ended element.
- X25-2 and X25-3: a row for an array in OBJ-1, and the glossary entry for a lifetime.
- X25-6: STG-P2 cites the storage-form table.

### Item 4. Refresh notes and a refresh check

Round 12 item 3, with phase 15's rows.
- X19: OPR-8, OPR-16, OPR-P3 and PRIM-2 (section 1.3), beside the existing `X22/X19-refresh.md`. Q-0436, Q-0440 and Q-0446.
- X18 and X9: VIRT-13's new condition for a hiding function.
- X25: STG-28 as an exception to STG-16, and ADD-14's integer case.
- X14: STG-16's exception and REF-2's bit-field case.
- X8, X7, X20 and X6: rebase notes for OPR-10, CTAD-5, TYN-10 and RFOR-P1.
- The round 12 list: X22, X8, X7, X6, X5 and X15.
- `tools/propose_check.py` reads a note beside a report, counts the rows the note covers as "noted", names the latest commit that changed each row, and uses the commit that added a report when the report names none. My `changed_since.py` does the diff part.

### Item 5. The portfolio step

- A column for the prototype: the extension points each proposal needs and the earliest build step, from section 1.10.
- P26: 8 files with a fold over the comma at 152 programs, with `r12-bigint-second-try.cpp` among them, and 0 to 3 programs that become invalid by the choices on an increment and an assignment. The scan script and its output go beside the file.
- P29: the user-suffix case of `r12-sso.cpp`, and the scan count of 24 lines in 9 files.
- P22: the entry names X24 and says that the prototype follows X24's table.

### Item 6. The packet rows that reach proposals

Section 5 gives the rows. The writer step makes the row 7 plan's count come from a tool (2824 `reject` tests, about 8472 compilations). The request on the record asks for rows 1, 4a and 10. Under "follow the compilers", rows 1 and 4a keep Q-0425, Q-0430 and Q-0442 at the answers that X25 and the storage pages use, with no test changed.

### Item 7. The records in the present

- `exercises.md` section 4 names `reports/clean-room/roadmap-7/portfolio.md` and ends its prose at the latest round (X25 on P27). Section 8 gains an X25 row ("done: 31 rule changes, 1 test changes outcome; findings X25-1 to X25-9").
- `DESIGN-rewrite.md` section 5 and the build order count 32 marked tests (6 for section 2, 11 for section 3). Section 10 drops the `forward_class` point.
- `TEMPLATE-MODEL.md` section 8 replaces the RV8 row with the next review's blocks.

### Item 8. P4-7, P4-5, P4-3 and P4-4

A writer answers P4-7 in OPR-8 (the value of the literal without its suffix, with the rounding of a floating literal), which X19's refresh note then cites. P4-5 adds the move constructor to the `Maybe` example and cites CPY-5 and CPY-P2. P4-3 and P4-4 add the two examples.

### Item 9. An index of what a test of validity can see

Round 10 item 9, unchanged. 51 of 559 `reject` rules have an `accept` or `run` test that uses `requires`.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them. P4-3 asks for an adaptor over any sequence, which an example answers.
- **A prototype in today's front end.** A page's rules sit in 6 files at the median (`DESIGN-current.md`), and the code would go with the old front end.
- **A shared impact scanner.** The prototype diff gives the syntax query that P26 and P29 need. Until step 5, text scans with their scripts give the lower bounds.
- **Prototypes of P26, P29, P24 or X25 in phase 16.** Each needs a point that section 7 lacks or a build step past step 3.
- **P23b's exercise.** P23b needs a matching rule, Q-0411, Q-0412, Q-0426 and the CNS-13 strength.
- **P28's exercise** until its entry chooses where the marker attaches.
- **P8b** until the maintainer decides rows 2 and 6.
- **Pair exercises.** X14 with X23, or X14 with X25, waits for the refresh notes of item 4.
- **Rules or `:since:` fields for later versions.** Row 7's probe makes an index only.
- **A source-reader designer** (Q-0116). Twenty-four exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2).
- **Full re-runs of old Propose reports.** Item 4's notes cover the changes that reach a design.

## 4. Process changes

1. **A repair names the done designs its text change reaches.** Rounds 8 to 12 process change, still not taken. Phase 15 changed 16 rows in 9 reports, and no commit names them. Item 4's tool change can print the list for the commit message.
2. **A prototype follows a Propose report.** The prototype's rule table is the report's change table, and its new rule IDs are the report's IDs. An implementer who must choose a design point files it as a finding against the report.
3. **A Propose report that cites the prototype diff states its base.** The base is the number of tests and programs that `tidal-next` passes without the flag.
4. **A count in the portfolio comes with its script and base.** P26's entry names a scan and gives a base of 126 programs where the scan read 138. The scripts and their output go beside the portfolio.
5. **Keep the exercise records current.** Section 8 of `exercises.md` gains a row with each Propose report, and section 4's prose names the latest round. Rounds 10 to 12 process change, taken only for section 4's table.
6. **The rewrite documents state the present.** `DESIGN-rewrite.md` joins `TEMPLATE-MODEL.md` in this rule. Each front-end round that changes the marks file or a ledger row they cite updates them.
7. **A phase with an exercise item runs a Propose until the stop rule has its count.** Phase 15 ran X25. The count waits for X24.
8. **The stop rule stays.** The rule counts X22 to X25.
9. **Keep the designer's view in round 14** in this form. Rerun the counts of sections 1.1, 1.3, 1.5 and 1.9, the diff of changed rows against each done design, and the table of section 1.10 against the hook as built.

## 5. The decision packet from a designer's view

The packet in `documentation/decisions/pending.md` gives each row's questions and an upper bound on the tests whose expectation changes. The order below follows the number of done designs and proposals that each row reaches.

| Order | Row | What it settles | Tests whose expectation changes | Proposals and done designs it reaches |
|---|---|---|---|---|
| 1 | 4b, Q-0426 | whether a constraint check runs when a candidate that is not a template matches exactly (CNS-11, CPY-15) | 0 if the compilers; the row's bound is 50 (4 tests name Q-0426) | X22 and P23b (the checked marker), X9 (the relocation constructor), and every design that adds a constraint |
| 2 | 4a, Q-0430 and Q-0442 | whether a typed step over bytes, the cast to element `k` through a byte pointer, and the step from an integer counterpart are defined (ADD-14, STG-28, STG-29) | 0 if the compilers; the row's bound is 36 for its questions (4 tests name Q-0430 or Q-0442) | X25's motivation and its STG-28 and ADD-14 rows, X9's STG-28 row, X14 through STG-16 |
| 3 | 9, Q-0412 | which effects a standard attribute may have | 0 under either option | P22, P23a (X22), P23b and P24 choose a marker by Q-0412. X23 cites the question, and X25 rejected an attribute marker under it |
| 4 | 1 | 58 questions, among them Q-0425 (UNI-14), Q-0420 (DED-29), Q-0421 (LSUP-11) and Q-0438 (CTAD-18) | 0 (138 tests name them) | X25 through Q-0425 and Q-0421; P23a and P23b through DED-29 |
| 5 | 4a, rule items DEP-15 with DEP-P4, and NEW-P5 | whether a static constant member equal to the parameter names the current instantiation; whether `new` adds an alignment argument on a second search | 0 if the compilers | X22's exemption of the current instantiation; X23's mirror case (NEW-30) |
| 6 | 4a, Q-0407 | ENUM-10's type for an enumerator from another enumeration | 0 (no test names it) | P24, X20 |
| 7 | 7 | Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 tests name them; the probe changes no expectation | the version line of 17 Propose reports, and of P22, P24, P26 and P29 |
| 8 | 9, Q-0183 and Q-0186 | a cost model; judgments from outside Tidal | 0 | section 4 and the outside-knowledge line of every Propose report |
| 9 | 2 and 6 | Q-0322; Q-0343 and Q-0041 | 0 for row 2; at most 5 for row 6 | P8b (relocation by bytes cites CPY-19's trivially copyable list) |
| 10 | 10 | 17 questions that Tidal answers as its own choice | 0 (32 tests name them) | Q-0020 (how tests record a compiler that lacks a feature), which X4 and X6 cite |

The request on the record asks for rows 1, 4a and 10. Under "follow the compilers", it reaches X25 twice (Q-0425 in row 1, Q-0430 and Q-0442 in row 4a), and X22 and X23 through the DEP-15 and DEP-P4 item and NEW-P5. Rows 4b and 9 reach the most designs and are not in the request. I found no proposal that rows 3, 5 and 8 reach.

## Host-only claims

None. I ran no compiler and no front end. Every behavior in this report comes from Tidal's files.

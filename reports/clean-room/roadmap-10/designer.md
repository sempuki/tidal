# Roadmap, round 10: what to do after phase 12, from a language designer's view

Date: 2026-11-06. Decision 0029, with decisions 0037, 0039, 0059 and 0060. Role: clean-room reader, language-designer persona (`designer`). Inputs: AGENTS.md and CLAUDE.md; my round 9 report `reports/clean-room/roadmap-9/designer.md` (the model for this form) and its scratch scripts; `reports/clean-room/roadmap-7/portfolio.md` (P22 to P26); `documentation/` (decisions 0059 and 0060, `decisions/pending.md` with its decision packet, the status page, the roadmap's phase 12 and front-end sections, and sections 4, 6, 8 and 9 of `exercises.md`); `git log a15a877d..ebc109b9` with every commit body; the W20 and I5 reports (summary, score and findings); `reports/clean-room/ledger.md`; `reports/clean-room/rust-frontend/report-23.md` (sections 5 and 6), its ledger rows G153 to G157, and the `new-delete` rows of `conformance.tsv`; the two 2026-11-05 phase 12 gap reviews in `reports/review/` (result tables, and the RV5-B-4, B-5 and B-11 sections); the head and tail of the phase 12 close similarity report; the change tables of X1, X4, X5, X7, X10, X12, X14 to X17, X19 to X21 and X23 at the rows that phase 12 changed; X23's header; the X22 refresh note's name; targeted reads of TR-P1, TR-4, TR-17, SW-P1, REF-1, UNW-3, LAM-7, NEW-27, ALN-5, COMMA-5 and the fold text of `expressions/primary.md` under `specification/`; the status and **Waits for:** lines of Q-0412, Q-0196 and Q-0020; `tests/` headers for the `-m32`, `operator,` and fold scans; and `tools/propose_check.py` plain, with `--corpus`, with `--strict`, and with `--strict` on each report alone. Counts come from read-only Python scripts in `reports/clean-room/roadmap-10/designer-scratch/scripts/`, run on this tree and on copies of `a15a877d` and `df4b5539` made with `git archive`. I did not read the other round 10 reports, `inputs/` or `build/exercise-keys/`. I did not run `tools/similarity.py`, `tools/run_tests.py` or `tools/question_index.py`. Statements from outside Tidal's files are marked "general knowledge". Commit read: `ebc109b9`.

## 1. The state from a designer's view

### 1.1 Counts at ebc109b9

| Measure | Round 9 (a15a877d) | Now (ebc109b9) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1697 | 1698 (TR-17 is new) |
| `required` / `reject` / `undefined` | 898 / 558 / 57 | 898 / 559 / 57 |
| `impl-defined` / `recommended` / `no-diagnostic` | 52 / 43 / 37 | 52 / 43 / 37 |
| `unspecified` / `conditionally-supported` / `withdrawn` | 32 / 5 / 15 | 32 / 5 / 15 |
| Rules and principles whose text changed in the phase (my diff, `:tests:` left out) | 26 (17 rules, 9 principles) | 36 (32 rules, 4 principles), and TR-17 added |
| Of these, changed after RV5 read the tree (df4b5539) | | 20, the list of the close commit |
| Test files with a `tidal-expect` line | 6711 | 6790 |
| Of these, `reject` | 2745 | 2762 |
| Of these, `link-fail` | 45 | 46 |
| Files under `tests/` that name `-m32` (of them, with `-m32` in `tidal-flags`) | 85 (76) | 85 (76) |
| Program corpus (`propose_check.py --corpus`) | 103 programs (79 Write, 24 sketches), 8 multi-unit, 126 files, 12187 lines | 115 programs (86 Write, 29 sketches), 8 multi-unit, 138 files, 13615 lines |
| Questions | 418 | 422 |
| Provisional / settled / compiler defects | 174 / 231 / 13 | 176 / 233 / 13 |
| Open questions in the packet's rows 1 to 9 | 111 of 174 | 111 of 176 (63%) |
| Propose reports | 23 | 23 |
| `propose_check.py`: conflicts | 31 (10 withdrawn, 4 class, 17 new-id) | 31 (10 withdrawn, 4 class, 17 new-id) |
| `propose_check.py`: rows changed since their report's commit | 131 | 141 |
| Change-table rows whose rule text changed in the phase (my diff) | 17, in 11 reports | 24, in 14 reports |
| `required` rules whose text says "the program must" | 1 (ALN-4) | 0 |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 40 of 558 | 44 of 559 |
| Front end, full pass / passes its check | 6478 of 6711 | 6586 / 6626 of 6790 |
| Program tests that run on the front end | 102 of 103 | every program test (111 in report 23's count) |
| Ledger rows | 867; 762 `commit`, 26 `planned`, 68 `open` | 925; 878 `commit`, 0 `planned`, 17 `open`, 17 `text`, 10 `question`, 3 `out of scope` |

Phase 12 added one rule ID and changed no class of an existing rule. It changed the text of 32 rules and 4 principles. The 10 new changed rows of `propose_check.py` are 4 in X23, 2 in X20, and 1 each in X14, X16, X17 and X19.

`propose_check.py --strict` exits 1 on the whole set, and on X1, X2, X4 to X12 and X19 alone. X3, X13 to X18 and X20 to X23 exit 0. Both results are the same as in round 9. `tools/propose_check.py` has not changed since d9c18bcc.

### 1.2 What phase 12 did with the round 9 designer items

Phase 12 took round 9 item 1 in full and the first step of item 2. Decision 0059 deferred items 3, 4, 5, 7, 8 and 9 by name and the rest of item 2 with them.

| Round 9 item | Phase 12 | State |
|---|---|---|
| 1. X23-3, X23-6, X23-7, ALN-4's duty, TR-4's class | F2 (0f3f522f), RD (6c6f69f8) | done. NEW-27 ties the default new alignment to ALN-5's largest fundamental alignment, and NEW-22, UNW-3 and ALN-4 use NEW-28's threshold. STG-8 says each implicit object is aligned for its type. ALN-4 states the consequence (STG-23) in place of a duty. TR-17 is a new `reject` rule for exactly one `main`, with a link-fail test of two definitions. Section 8's Propose rows still run X23, X22, X21 |
| 2. A Propose prompt, X24 on P26, the stop-rule count | R6 (13c03237) wrote the prompt | prompt done. X24 deferred by decision 0059. The stop-rule count waits for X24 |
| 3. Refresh notes and a refresh check that reads them | none | deferred. `reports/clean-room/X22/X19-refresh.md` is still the one note |
| 4. A front-end prototype of P25 | none | deferred. Every `aligned-*` test of NEW-29 and NEW-30 now passes on the front end |
| 5. A shared impact scanner | none | deferred. Three scanners are kept (X21, X22, X23) |
| 6. The later-version probe | none | waits on the maintainer (row 7). The plan still says about 2640 tests and 7900 compilations. There are 2762 `reject` tests, so about 8286 compilations |
| 7. Pair exercises | none | deferred |
| 8. An index of what a test of validity can see | none | deferred. 44 of 559 `reject` rules now |
| 9. ILP32 runs | none as an item | NEW-27 has two `-m32` tests from 0bf1a546. The 76 tests with `-m32` flags are unchanged in number |

Round 9's process changes:
- Taken: process change 1 became decision 0059's rule "a planned row names a live item", and `check_specification.py` enforces it. Process change 3 in substance: TR-17 and the ALN-4 rewrite leave no `required` rule that says "the program must". Process change 4: section 9 has a Propose prompt.
- Not taken: process change 2. F2 and RD changed rows of X21, X23 and X14, and RV5 changed rows of X7, X12, X14, X15, X19 and X23. No commit names those reports as reached. Process change 5: section 4's prose still ends at "Round 7 ... with X22 on P23", and section 8 still lists X23 above X22 above X21.

### 1.3 Phase 12 text that reaches a done design

My diff of a15a877d against ebc109b9 finds 24 change-table rows in 14 reports whose rule text changed. The diff of df4b5539 against ebc109b9 gives the 20 IDs of the close commit and no new ID. These reach a design:

| Report | Rows | Effect on the design |
|---|---|---|
| X21 (P15) | TR-4, TR-P1, SINIT-6 | TR-4 no longer holds the policy for `main`. TR-17 holds it, and TR-P1 names TR-17 as the one rule whose diagnostic comes at link time. X21's TR-4 row cites a policy that has moved. X21:new-4 stays `no-diagnostic` under TR-P1. RV5-A-14 reads two definitions of `main` as open, and TR-17 takes the reading with a diagnostic |
| X23 (P25) | ALN-4, NEW-22, NEW-27, STG-5, STG-8, UNW-3 | The X23-3 repair made NEW-22, UNW-3 and ALN-4 use NEW-28's threshold. That is the amendment in X23's UNW-3 row and part of its ALN-4 row, so those rows now describe current text. RV5-B-4 reads NEW-P5 against the matrix, and NEW-P5 joins pending.md row 4 with the test `only-aligned-class-form-ordinary-type.cpp`. That is the case of Q-0300 that X23 names. RV5-B-5 says NEW-22 limits the sized form in a way the review does not read, and NEW-22 keeps the compilers' answer with an Evidence note |
| X14 (P20) | ALN-4, REF-1, STG-8 | ALN-4's second exception now states a consequence through STG-23. X14's amendment names one exception. REF-1 now lists four kinds of direct binding through a pointer conversion. X14's phrase "and not under-aligned" still attaches to "an lvalue that is not a bit-field", so the amendment applies as written. STG-8 now says an implicit object is aligned for its type. X14's row says implicit creation can make a packed object at any address, which holds when the packed class has alignment 1 |
| X12 (P19) | SW-P1 | SW-P1 now says the statement after the `switch` is advice and the program is valid without one. X12 amends SW-P1 so that a value with no case ends the program. The two fit. X12's amendment should restate SW-P1's new sentence |
| X7 (P12) | ORD-13 | ORD-13 now names the conditions it replaces. X7 adds value-specialized positions to those conditions, so the amendment must name the new phrase |
| X15 (P21) | NAM-21 | The friend sentence changed (RV5-A-10), and Q-0415 waits for a first review. X15:new-2 reads NAM-21's parts as positions inside `N`. No effect on the design that I can see |

The other rows change cases that the designs do not use: LAM-7 in X1, X4 and X5 (a change of term from "explicit" to "simple" capture), UNW-3 in X10, CONV-1 in X19, OVL-11 in X16 and X20, and COND-3 in X17 and X20.

Eight of the 20 IDs changed after RV5 have no row in a change table: AUTO-P3, BF-6, CLS-26, JMP-10, LAM-5, LAM-6, RET-6 and ICS-11. X9 names RET-6 and X5 names ICS-11 in prose only.

### 1.4 The 20 rules and principles changed after RV5, ranked by design reach

| Rank | IDs | Done designs reached | Why a designer reads them |
|---|---|---|---|
| 1 | TR-P1, TR-17 (new) | X21; every proposal | TR-P1's opening paragraph now gives the meaning of `reject`, `no-diagnostic` and `undefined`. Every new rule in a proposal picks its class from that paragraph |
| 2 | ALN-4, STG-5, UNW-3 | X14, X23, X10 | P25's rows and the pair of X14 and X23 (item 7) |
| 3 | STG-P3 | X14 and X23 in prose; W20-1, W20-2, W20-6 | The storage forms that a new in-object array design (item 5) starts from |
| 4 | SW-P1 | X12 | P19's model of an exhaustive `switch` |
| 5 | ORD-13, NAM-21, REF-1, SINIT-6, CONV-1 | X7, X15, X14, X21, X19 | Rebase notes only |
| 6 | LAM-5, LAM-6, LAM-7 | X1, X4, X5 (LAM-7) | A change of term only |
| 7 | AUTO-P3, RET-6, JMP-10, BF-6, CLS-26, ICS-11 | none | No design cites them |

The next gap review reads all 20. For a designer, RV6 should also read NEW-P5 and NEW-22 beside ALN-4, since RV5-B-4 and RV5-B-5 sit in P25's area.

### 1.5 Pages the Propose exercises have reached

All 85 pages are still reached by rule IDs cited in X1 to X23. No Propose exercise ran in phase 12, so the pages read by one exercise only are the same four:

| Page | Exercise | Commits to the page since that report | Proposal that aims at it |
|---|---|---|---|
| `declarations/namespaces.md` | X15 | 7 | none open |
| `expressions/comma.md` | X1 | 5 | P26 |
| `statements/loops.md` | X11 | 5 | P22 |
| `expressions/bitwise.md` | X17 | 1 | P24 |

Phase 12 made no net change to `comma.md`, `loops.md`, `bitwise.md`, `enumerations.md` or `namespaces.md`. It changed `jumps.md` (JMP-10, a `return` rule) and one line of `classes/operators.md`.

### 1.6 What a designer still cannot state

**Which class a rejection gets when a `required` rule states it.** TR-17 settled the case of `main`. Front-end finding G155 names a second case. NEW-29 is `required` and says a selected allocation function that is deleted or not accessible makes the expression not valid, and no `reject` rule names that break. G155 is `open` in the front-end ledger. A crude scan finds 74 `required` rules whose text contains "not valid". Many of them point to a `reject` rule, so 74 is an upper bound.

**What a standard attribute may do.** Q-0412 still holds a provisional answer and waits on the maintainer (row 9). No test names it.

**Whether a later version has taken the space.** 17 Propose reports name Q-0007. The row 7 plan is costed, and its count is stale (section 1.2).

**What a change costs.** No change. Q-0183 keeps option 1.

**How much code a change touches.** The three kept scanners use text and an injected header. P26 needs a query on syntax: a comma whose left operand is not a call and not of type `void`. A text scan cannot tell a comma operator from a separator (COMMA-4), and a header of deleted declarations cannot find a built-in comma. The front end parses and runs every program test now.

**Which programs change meaning without a diagnostic.** 44 of 559 `reject` rules have an `accept` or `run` test that uses `requires`.

**How two changes compose.** No pair exercise has run. ALN-4 changed again in phase 12, and X14 and X23 both amend it.

**Where substitution stops.** I5-2 reads DED-29's first list against its third paragraph, and I5-5 finds no rule for `T::type` with an incomplete `T`. P23a (X22) and P23b check template bodies against constraints, so their designs rest on where a substitution failure ends and a hard error starts.

### 1.7 The portfolio

| # | Proposal | Round 9 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P26 | No overloaded comma, checked left operand | ready | ready | | The portfolio entry. It names 3 pages and leaves out `expressions/primary.md` (PRIM-27, the fold). It counts 8 test files with `operator,` and one program. A scan for a declaration finds 5 test files and no program. The entry must also say whether an assignment, an increment, and each element of a fold count as a left operand that "has a value" |
| P24 | Flag enumerations | ready | ready | | The marker under Q-0412 (row 9). ENUM-10 in row 4a (Q-0407, no test names it) |
| P22 | Labeled `break` and `continue` | ready | ready | | The marker. A version line (row 7) |
| P23b | Expressions checked against requirements | partial | partial | | No matching rule (X22-8). Q-0411 doubt. Q-0412 for the marker. I5-2 and I5-5 on the substitution boundary |
| P8b | Relocation by bytes | partial | partial | | Row 2 (Q-0322). Row 6 (Q-0343, CPY-19) |

Totals: 23 done, 3 ready, 2 partial, 0 blocked. P26 is still the one ready proposal that changes existing behavior.

Rough counts for P26 by `grep` over the 115 programs:
- No program declares `operator,`. Five rule tests do: `comma-order-kept`, `comma-overload`, `overloaded-comma-order`, `overloaded-void-operand-built-in` and `builtin-none-for-address-comma`.
- Three programs fold over the comma: `r8-fmt.cpp` (calls), `r9-vec.cpp` (a call of `emplace_back`), and `r7-registry/registry.h` (an assignment `acc = fn(acc, rest)`).
- One program has a comma in a `for` increment with a left operand that is not a call: `r5-hashmap-kept-pointer.cpp`, `++step, i = (i + 1) % N`, twice.
- So at least 2 of 115 programs become invalid under P26 as the portfolio states it (`r5-hashmap-kept-pointer.cpp`, and `r7-registry` if its fold has two or more elements). The scan reads `for` heads and folds only, so the count is a lower bound.

W20 suggests a new entry. W20-1, W20-2 and W20-6 together say that no storage form in the text keeps the elements inside the object, gives a pointer and a length over them, and runs in constant evaluation. A designer would write that as P27, "an in-object array whose elements start their lifetimes one at a time", my own invention. P27 adds a form and changes no existing behavior. It is `partial` until W20-2 has an answer, because the design must know what an array member of a union means today.

### 1.8 Questions and principles

Phase 12 opened 4 questions (Q-0419 to Q-0422) and settled 2 (Q-0323, Q-0383). No open question waits for a use of a principle. Q-0196 waits for a principle that does not exist yet (TPL-P3), as in round 9. The share line is 63% for rows 1 to 9 and 70% with row 10, the same as at the start of the phase.

Principles that a Propose finding met, now:

| Principle | Finding | State |
|---|---|---|
| ATTR-P1, ATTR-P2, ATTR-P4 | X14-7, X22-6 | Q-0412, row 9. Bears on P22, P23a, P23b and P24 |
| STG-P3 | X23-6 | answered in 0f3f522f, and changed again by RV5-A-13 |
| NEW-P5 | X23-1 | text repaired. RV5-B-4 reads it against the matrix, and it joins row 4 |
| TR-P1 | X21-4 | answered by TR-17. RV5-A-14 and A-15 hold doubts on TR-17's two-unit case and the opening paragraph |
| DEP-P6 | X22 (the checked marker) | RV5-B-11 holds a doubt on the row for a static assertion, with state `text` |

### 1.9 The 17 open ledger rows, ranked by design reach

| Rank | Rows | What they reach | Who acts |
|---|---|---|---|
| 1 | W20-2 | P27 cannot start until the text says what an element of an array member of a union is. A question if the compilers or the text leave it open | writer |
| 2 | W20-1, W20-6 | The input for P27: a table of storage forms, with what each allows (contiguous pointer, constant evaluation, default constructor) | writer |
| 3 | I5-2, I5-5 | The substitution boundary under P23a and P23b | writer, with probes |
| 4 | W20-3 | CPY-15 against CNS-11: whether the by-value constructor template leaves the candidates before its constraints are checked. X9-8 found a wrong reason in CPY-P1 on the same template. Bears on X9's relocation constructor | writer, then front end |
| 5 | I5-1, I5-4 | DED-30 to DED-29, DED-15 to OVL-37: pointers between rules. No design reaches them | writer |
| 6 | I5-3, I5-6, I5-8, I5-7 | Missing statements and examples in deduction | writer |
| 7 | W20-5, W20-4 | CAST-1 and STG-16; a CMP-15 Evidence note on a GCC sanitizer build | writer |
| 8 | I5-9, I5-10 | front-end faults with reductions | front end |
| 9 | I5-11 | outside knowledge; DED-24 and DED-11 could absorb two parts | writer |

### 1.10 The distance to decision 0060's completion test

The second condition holds. Every program test runs to its expected result. The first condition fails on 164 tests: 6790 less the 6626 that pass their check. No front-end ledger row marks a test `out of scope`. Report 23 counts 204, because it adds 40 `run-unsupported` tests, which pass their check. By status, the 164 are 100 missed rejects, 58 unsupported, 4 false rejects and 2 `EDG-AGREES`. Of report 23's 204, `overloading/conversion-sequences` has the most (12), and `expressions/new-delete` has 10, of which 5 fail their check. None of the 10 is a NEW-29 or NEW-30 test.

For a designer, the rewrite (item FR) matters more than the count. Decision 0060 asks for modules by page with rule IDs in comments. A prototype of a proposal then changes the module of each page the proposal amends, behind a flag. A prototype written into today's front end would be removed with the old code.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Repair the Propose prompt and P26's portfolio entry, then X24 on P26, then the stop-rule count | 1.2, 1.5, 1.7; section 9's shared rules forbid `tests/` and `reports/` | 1 short writer step, 1 reader session, 1 repair session | low | the second reading of `comma.md`; the stop rule's first full count |
| 2 | Refresh notes for X21, X23, X22, X14, X12 and X7, and a refresh check that reads them and names the commit that changed each row | 1.1: 141 rows; 1.3 | 1 reader session, a small tool change | low | reuse of the done designs; item 7 |
| 3 | A shared impact scanner with a syntax query through the front end | 1.6; 1.7: the comma scan is a lower bound | 1 front-end or writer session | low | section 4.4 of X24 and later reports |
| 4 | The packet rows that reach proposals: row 9 (Q-0412), the NEW-P5 and Q-0300 items of row 4, and row 7 with its count corrected | section 5 | maintainer decisions; 1 writer step for the count | low | the markers of P22, P23 and P24; P25's mirror rule; version lines |
| 5 | P27 from W20: answer W20-2, write the storage-form table (W20-1, W20-6), then add P27 to the portfolio | 1.7, 1.9 | 1 writer session with probes, then a short designer step | low | a ready proposal that changes no existing behavior |
| 6 | The substitution boundary: I5-2 and I5-5, then P23b's rating again | 1.6, 1.9 | part of 1 writer session | low | P23b |
| 7 | Pair exercise X14 with X23 | 1.3: ALN-4 changed again | 1 reader session after item 2 | low | an interaction section that can be checked |
| 8 | A prototype hook in the FR design document, and a P25 prototype after FR's stage for `new-delete` | 1.10 | a paragraph in FR's document; later part of a front-end session | low | the first measured check of a Propose impact count |
| 9 | An index of what a test of validity can see | 1.1: 44 of 559 | 2 writer sessions | medium | section 4.3 of every proposal |

### Item 1. The Propose prompt, P26's entry, X24, and the stop-rule count

**What it is.** Four steps.
- **The prompt.** Section 9's shared rules say "Do not read tests/ ... reports/ ... and do not use git log". The Propose prompt tells the reader to run `propose_check.py` on the Propose reports it reads and to count programs under `tests/programs/`. The two conflict. The Principle use prompt shows the form of an exception. The Propose prompt should say that a Propose reader also reads `tests/programs/`, the Propose reports and the refresh notes beside them, and the rule tests that the scanner names. It should ask for a version line ("C++23 only", or the row 7 index once it exists) and for the change table in the form `--strict` reads. It should name the portfolio of the round (`roadmap-7/portfolio.md` for P26).
- **P26's entry.** Add `expressions/primary.md` and PRIM-27 to the pages. Replace "8 test files, one of them a program" with 5 test files and no program. Add three cases to "Not settled": an assignment or an increment as the left operand, each element of a fold over the comma, and a left operand of class type that "has a value".
- **X24 on P26.** The design states what a left operand may be, whether a call of an operator function or through a pointer counts, and whether a fold is checked per element or once. Section 4.4 counts at least `r5-hashmap-kept-pointer.cpp` and `r7-registry` as becoming invalid, with item 3's scanner if it exists. P26 meets OPR-7, OPR-P2 and the overloaded operators that I3 modeled.
- **The stop rule.** After X24, count X22 to X24's findings and the share the text closed. X22 closed 6 of 9 in the text and X23 closed 4 of 7.

**Evidence.** Sections 1.2, 1.5 and 1.7. `comma.md` has one reading (X1), and X1 names no commit.

**Cost.** One short writer step for the prompt and the entry, one reader session, one repair session.

**Risk.** Low for similarity. P26 changes existing behavior, so its 4.3 and 4.4 sections need the full count.

**What it unblocks.** The second reading of `comma.md`, and the stop rule.

### Item 2. Refresh notes and a refresh check that reads them

**What it is.** Round 9 item 3, with phase 12's rows.
- X21: TR-4's row now rests on TR-17 and TR-P1. X21:new-4 keeps `no-diagnostic`.
- X23: the rows that F2 adopted (UNW-3, and ALN-4's threshold), and RV5-B-4 and B-5 as open doubts in P25's area.
- X22: Q-0412 against `[[checked]]`, and DEP-P6's independent case with RV5-B-11.
- X14: ALN-4's second exception in its STG-23 form, REF-1's longer list.
- X12: SW-P1's new sentence. X7: ORD-13's new phrase.
- `tools/propose_check.py` reads a note beside a report and counts the rows the note covers as "noted". It names the latest commit that changed each row. It uses the commit that added a report when the report names none, so X1 to X12 get a changed-row count.

**Evidence.** 141 changed rows, up from 131. The tool has not changed since d9c18bcc. `X19-refresh.md` is the one note.

**Cost.** One reader session for the notes, a small tool change.

**Risk.** Low. The check stays a warning without `--strict`.

**What it unblocks.** Item 7, and reuse of X21, X22 and X23 in later designs.

### Item 3. A shared impact scanner with a syntax query

Round 9 item 5, with a new need. P26's count needs to find a built-in comma by the form of its left operand, and neither method of the kept scanners can. A scanner under `tools/` takes the text queries of X21 and X22, the injected-header method of X23, and a syntax query that the front end answers. It honors `tidal-flags` and reports `tests/programs/` apart from the rule tests. If the front end gains a query mode, FR's design document should keep it.

### Item 4. The packet rows that reach proposals

Section 5 gives the rows. The writer step is to make the row 7 plan's count come from a tool (2762 `reject` tests, about 8286 compilations), so the count stops going stale.

### Item 5. P27 from W20

A writer answers W20-2 with a rule or a question, and writes the table that W20-6 asks for beside the `FixedVec` and `FixedStack` examples. The designer then adds P27 to the portfolio with its pages (`foundations/storage.md`, `classes/unions.md`, `foundations/language-support.md`), the rules it meets (STG-P2, STG-P3, UNI-13, UNI-14, ADD-10, LSUP-10) and a rating. P27 is a new form, so section 4.4 should find no program that becomes invalid.

### Item 6. The substitution boundary

I5-2 and I5-5 go to a writer session with probes. Then the designer reads P23b's rating again with X22-8 and Q-0411.

### Item 7. Pair exercise X14 with X23

Round 9 item 7, unchanged in scope. The pair asks what `new` does for a packed class whose members are under-aligned, now that ALN-4 states its second exception through STG-23.

### Item 8. A prototype hook in FR, then a P25 prototype

The FR design document names how a proposal prototype attaches to a page module behind a flag. The P25 prototype follows FR's stage for `new-delete`. X23 predicts 2 tests from `run` to `reject` and 3 programs unchanged through the non-allocating exemption.

### Item 9. An index of what a test of validity can see

Round 9 item 8, unchanged. 44 of 559 `reject` rules have an `accept` or `run` test that uses `requires`.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them.
- **P23b's exercise in the next phase.** It needs a matching rule, Q-0411 and Q-0412, and item 6.
- **P8b** until the maintainer decides rows 2 and 6.
- **Rules or `:since:` fields for later versions.** Row 7's probe makes an index only.
- **A source-reader designer** (Q-0116). Twenty-three exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2).
- **A model of object files or mixed implementations.** Only X21 needs one.
- **Full re-runs of old Propose reports.** Item 2's notes cover the changes that reach a design.
- **ILP32 runs as their own item.** X23-3's answer came from probes, and NEW-27 has `-m32` tests. The conversion tests can wait for a proposal that needs them.

## 4. Process changes

1. **The Propose prompt states its own read list** (item 1). The shared rules forbid what the Propose prompt asks for.
2. **A repair names the done designs its text change reaches.** Round 8 and 9 process change 2, still not taken. Phase 12 changed rows of 14 reports, and no commit names them. Item 2's tool change can print the list for the commit message.
3. **A `required` rule that states a rejection points to a `reject` rule.** TR-17 did this for `main`. G155 asks the same for NEW-29. A check could list `required` rules that say "not valid" with no `reject` rule cited in the same paragraph.
4. **Keep the exercise records current.** Section 4's prose names the latest round, and section 8 lists the Propose rows in order (X21, X22, X23).
5. **A commit message agrees with the text it adds.** 0f3f522f says the probe gives 16 as the default new alignment on every baseline compiler. NEW-27's table and the test `default-new-alignment-ilp32-clang.cpp` record 8 for Clang with `-m32`. AGENTS.md asks for a follow-up commit in such a case.
6. **The stop rule stays.** It counts X22 to X24.
7. **Keep the designer's view in round 11** in this form. Rerun the counts of sections 1.1, 1.3, 1.5 and 1.9, and the diff of changed rows against each done design.

## 5. The decision packet from a designer's view

The packet in `documentation/decisions/pending.md` gives each row's questions and an upper bound on the tests whose expectation changes. The order below follows the number of proposals that each row reaches.

| Order | Row | What it settles | Tests whose expectation changes | Proposals and done designs it reaches |
|---|---|---|---|---|
| 1 | 9, Q-0412 | which effects a standard attribute may have | 0 under either option (no test names Q-0412) | P22, P23a (X22), P23b and P24 choose a marker by it. X23 cites it. Under the kept answer, X22's `[[checked]]` adds required rejections and must become a keyword |
| 2 | 4, the NEW-P5 item and Q-0300 (4a) | whether `new` for a type that is not over-aligned adds an alignment argument on a second search | 0 if the maintainer follows the matrix. If the review, at most the 2 tests that name Q-0300 and the reject test `only-aligned-class-form-ordinary-type.cpp` | P25's mirror case (NEW-30). X23 says P25's own rules do not depend on it. NEW-22's sized form (RV5-B-5) is an Evidence note, outside the packet |
| 3 | 4a, Q-0407 | ENUM-10's type for an enumerator from another enumeration | 0 (no test names it) | P24, X20 |
| 4 | 7 | Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 tests name them; the probe changes no expectation | the version line of 17 Propose reports, and of P22, P24 and P26 |
| 5 | 1 | 54 questions, among them Q-0420 (DED-29 for an address) and Q-0421 | 0 (125 tests name them) | P23a and P23b through DED-29 (I5-1, I5-2) |
| 6 | 9, Q-0183 and Q-0186 | a cost model; judgments from outside Tidal | 0 | section 4 and the outside-knowledge line of every Propose report |
| 7 | 2 and 6 | Q-0322; Q-0343 and Q-0041 | 0 for row 2; at most 5 for row 6 | P8b (relocation by bytes cites CPY-19's trivially copyable list) |
| 8 | 10 | 15 questions Tidal answers as its own choice | 0 (27 tests name them) | Q-0020 (how tests record a compiler that lacks a feature), which X4 and X6 cite |

Rows 1, 4a and 10 each have an option under which no test's expectation changes, and they hold 84 open questions. Of their questions, Q-0420 and Q-0020 reach a design. I found no proposal that rows 3, 5 and 8 reach.

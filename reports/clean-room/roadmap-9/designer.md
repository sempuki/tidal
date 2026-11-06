# Roadmap, round 9: what to do after phase 11, from a language designer's view

Date: 2026-11-05. Decision 0029, with decisions 0037, 0039, 0057 and 0058. Role: clean-room reader, language-designer persona. Inputs: AGENTS.md and CLAUDE.md; my round 8 report `reports/clean-room/roadmap-8/designer.md` (the model for this form) and its scratch directory; `reports/clean-room/roadmap-7/portfolio.md`; `documentation/` (decisions 0058 and 0057, `decisions/pending.md` with its decision packet, the status page, the roadmap's phase 11 section, and sections 4, 6, 8 and 9 of `exercises.md`); `reports/clean-room/X23/report.md` in full, and X22's report and refresh note as read in round 8; the I4 report's findings and the P1 report's headings; the two 2026-11-05 gap reviews of phase 10 text in `reports/review/` (result, principle uses, questions, and the TR-P1, TR-4, LSUP-P2, CNS-P2, CPT-19 and DEP-P6 sections); `reports/clean-room/ledger.md`; `reports/clean-room/rust-frontend/conformance.tsv` for the program and `new-delete` rows; question files Q-0412, Q-0299, Q-0300, Q-0407, Q-0417, Q-0418, Q-0183, Q-0007, Q-0322, Q-0411 and Q-0383 (status and Waits for lines); targeted reads of TR-P1, NEW-21, NEW-22, NEW-29, ALN-4, ALN-5, STG-5 and CE-P4 under `specification/`; the change tables of X6, X7, X9, X10, X13, X14, X15, X21 and X22 at the rows that phase 11 changed; `tools/propose_check.py` with and without `--corpus` and `--strict`, on all reports and on each report alone; `tools/frontend_check.py`; and `git log 1e20bfa2..a15a877d` with every commit body. Counts come from read-only Python scripts in `reports/clean-room/roadmap-9/designer-scratch/scripts/`, run on this tree and on a copy of `1e20bfa2` made with `git archive`. I did not read the other round-9 reports, `inputs/` or `build/exercise-keys/`. I did not run `tools/similarity.py` or `tools/run_tests.py`. Statements from outside Tidal's files are marked "general knowledge". Commit read: `a15a877d`.

## 1. The state from a designer's view

### 1.1 Counts at a15a877d

| Measure | Round 8 (1e20bfa2) | Now (a15a877d) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1697 | 1697 |
| `required` / `reject` / `undefined` | 898 / 558 / 57 | 898 / 558 / 57 |
| `impl-defined` / `recommended` / `no-diagnostic` | 52 / 43 / 37 | 52 / 43 / 37 |
| `unspecified` / `conditionally-supported` / `withdrawn` | 32 / 5 / 15 | 32 / 5 / 15 |
| Rules and principles whose text changed in the phase (my diff, `:tests:` left out) | 49 (39 rules, 10 principles) | 26 (17 rules, 9 principles) |
| Test files with a `tidal-expect` line | 6681 | 6711 |
| Of these, `reject` | 2740 | 2745 |
| Of these, `link-fail` | 44 | 45 |
| Files under `tests/` that name `-m32` (of them, with `-m32` in `tidal-flags`) | 85 (76) | 85 (76) |
| Program corpus (`propose_check.py --corpus`) | 97 programs, 7 multi-unit, 118 files, 11438 lines | 103 programs (79 Write, 24 sketches), 8 multi-unit, 126 files, 12187 lines |
| Questions | 411 | 418 |
| Provisional / settled / compiler defects | 171 / 227 / 13 | 174 / 231 / 13 |
| Open questions whose `**Waits for:**` names a use of a principle | 3 | 0 |
| Open questions in the packet's rows 1 to 9 | 98 | 111 of 174 |
| Propose reports | 22 | 23 |
| `propose_check.py`: conflicts | 31 (10 withdrawn, 4 class, 17 new-id) | 31 (10 withdrawn, 4 class, 17 new-id) |
| `propose_check.py`: rows changed since their report's commit | 125 | 131 |
| Change-table rows whose rule text changed in the phase (my diff) | 26, in 14 reports | 17, in 11 reports |
| Front end (status page) | 6459 of 6681 | 6478 of 6711 |
| Program files the front end passes | 97 of 97 | 102 of 103 (`r8-tasks.cpp` is a false reject) |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 39 of 558 | 40 of 558 |
| Ledger rows (`reports/clean-room/ledger.md`) | none | 867 rows; 762 `commit`, 26 `planned`, 68 `open` |

Phase 11 added no rule ID and changed no class. It changed the text of 17 rules and 9 principles. The 6 new rows of `propose_check.py` are PACK-13 in X13 and five X23 rows. All five X23 rows changed in ee6085aa, X23's own repair commit. The tool does not say so.

`propose_check.py --strict` exits 1 on the whole set, and on X1, X2, X4 to X12 and X19 alone. X3, X13 to X18 and X20 to X23 exit 0. The 31 conflicts are the same as in round 8. The X19 class conflict on OPR-9 still prints, though the X19 refresh note answers it.

### 1.2 What phase 11 did for the designer

Phase 11 took round 8 item 1 in full and half of item 2. Decision 0058 deferred items 4 to 9 by name.

| Round-8 item | Phase 11 | State |
|---|---|---|
| 1. TR-P1 against TR-4, X22-6 and the X19-refresh gap on record | item F, 47f58739 | done. TR-P1 names `main` as the one requirement with a link-time diagnostic. X22-6 is Q-0412 (pending row 9). Q-0383 has a dated note. The class of TR-4 and the case of two definitions of `main` stay open (section 1.6) |
| 2. X23 on P25, X24 on P26, then the stop-rule count | item E, 5523f0e8 and ee6085aa | X23 done. X24 deferred by decision 0058. The count is interim (section 1.3) |
| 3. The later-version probe | none | waits on the maintainer (row 7). The plan names about 2640 `reject` tests and 7900 compilations. There are 2745 tests, so about 8235 compilations |
| 4. Refresh notes and a refresh check that reads them | none | deferred. `tools/propose_check.py` has not changed since d9c18bcc |
| 5. A shared impact scanner | none | deferred. X23 built two more scanners, so three are kept (X21, X22, X23) |
| 6. A front-end prototype of a done proposal | none | deferred |
| 7. An index of what a test of validity can see | none | deferred. 40 of 558 `reject` rules now |
| 8. Pair exercises | none | deferred |
| 9. ILP32 runs for conversion tests | none | deferred. 76 tests with `-m32` flags, as before |

Round 8's process changes:
- Taken: process change 1 became decision 0058's ledger rule. Every finding now has a ledger row, and a deferred finding gets a `planned` row. Process change 3: RV4 read TR-P1 and TR-4. Process change 5 in part: section 6 of `exercises.md` now points at section 8 and the phase table.
- Not taken: process change 2. Commit 8d7f3bb3 changed PACK-13, which X6 and X13 cite, and ee6085aa changed ALN-4, which X14 amends. Neither commit names those reports. Process change 4: section 9 still has no Propose prompt, and X23-7 asks for the same thing. Section 8 lists X23 above X22 above X21. Section 4's prose ends at "Round 7 ... with X22 on P23".

The `planned` ledger rows whose item ended (listed in a15a877d): I3-1, I3-5, I3-13, R10-4, R10-6, P1-3, P1-5, P1-7, P1-8, P1-9, I4-8 to I4-14, X23-3, X23-6 and X23-7. That is 20 rows. None has an answer. Each row still names a phase 11 item that is done, and the ledger has no state for "item ended, no answer". P1-1, P1-2 and I4-4 to I4-7 (6 rows) wait for front-end round 23, which moved to the next phase.

### 1.3 What X22 and X23 found, and what the repairs closed

| Exercise | Proposal | Findings | Closed in the specification | To a question | Planned, item ended | No action or out of scope |
|---|---|---|---|---|---|---|
| X22 | P23a, template-ids checked against constraints | 9 | 6 (X22-1 to X22-5, X22-7) | X22-3 (Q-0411), X22-6 (Q-0412) | none | X22-8 out of scope. X22-9 closed in `exercises.md` |
| X23 | P25, `new` and `delete` that honor every alignment | 7 | 4 (X23-1, X23-2, X23-4, X23-5) | X23-2 (Q-0417) | 3 (X23-3, X23-6, X23-7) | none |

Over X22 and X23, the text closed 10 of 16 findings. X23-1 is a "wrong" finding: ALN-4 promised alignment that NEW-29 let a class break. X22 had none. A second reading of `declarations/alignment.md` (12 commits after X14) found one wrong rule and two missing ones.

What X23 found about the text, by kind:
- **A promise with no class for its break.** ALN-4 now says that a class's own allocation function that NEW-29 calls without the alignment gets no promise, and "the program must make it return aligned storage". ALN-4 is a `required` rule. No rule gives a class to a program that breaks that duty. X23 proposed an `undefined` rule (X23:new-4) and a sanitizer test like its probe `w1.cpp`. The repair adopted neither. My scan finds ALN-4 to be the one `required` rule whose text says "the program must". TR-4 holds a rule on the program in the same way (section 1.6).
- **A compiler split now on record.** NEW-29 now says that a selected aligned form that is deleted or private makes the expression not valid. Clang and EDG reject, and GCC falls back (Q-0417, first review pending). This is the second case of X23:new-1. Phase 11 adopted that part of P25 as current text.
- **Three thresholds, untied (X23-3, planned).** NEW-28 uses the default new alignment, UNW-3 and NEW-22 use extended alignment, and ALN-4 uses the largest fundamental alignment. The ALN-5 table gives 8 for Clang with `-m32` and 16 for GCC with `-m32`.
- **Implicit objects in weakly aligned storage (X23-6, planned).** STG-P3's growable-array example and STG-8 do not say what happens when `::operator new(n * sizeof(T))` gives storage too weakly aligned for `T`. `roadmap/r4-smallvec.cpp` uses that pattern for a generic `T`.

X23's needs outside the specification are fewer than any report since X11. It raises no version line, no cost question and no mixed implementations. It is the first Propose report since X11 that does not raise version marking. P25 changes C++23 behavior only, so the version question does not bear on it (general knowledge: a later version may change the same fallback, which the probe of row 7 would show).

X22's done design met phase 11 text in two places:
- **Q-0412 against `[[checked]]`.** Q-0412's provisional answer limits a standard attribute to placement rules, warnings, promises and layout changes, and "a new attribute adds no required rejection". X22 chose the attribute `checked` for a feature that adds required rejections at the definition. Under that answer, P23a's marker must change to a keyword or a contextual keyword, as X12 and X18 chose. No note says so.
- **DEP-P6's new row.** RV4 changed the row for a template-id with independent arguments that fail its constraints to "need not diagnose; every baseline compiler rejects". P23a rejects a dependent template-id in a checked template. So under P23a, a checked template must diagnose a dependent template-id that its constraints do not cover, and need not diagnose an independent one that fails. X22's design should say whether the checked marker also requires the independent case.

The RV4 review of CPT-19 reads Q-0411 with a doubt, and says a later version may give EDG's answer. That is one more P23 question that row 7 would inform.

### 1.4 Pages the Propose exercises have reached

All 85 pages are reached by `:::{rule}` IDs cited in X1 to X23. X23 gave `declarations/alignment.md` its second reading. Four pages are reached by one exercise only:

| Page | Exercise | Commits to the page since that report | Proposal that aims at it |
|---|---|---|---|
| `declarations/namespaces.md` | X15 | 7 | none open |
| `expressions/comma.md` | X1 | 5 | P26 |
| `statements/loops.md` | X11 | 5 | P22 |
| `expressions/bitwise.md` | X17 | 1 | P24 |

### 1.5 The refresh check at a15a877d

`tools/propose_check.py` reports 23 reports, 31 conflicts, 131 changed rows and 0 warnings. With `--strict` it exits 1. The per-report changed rows are X13 14, X14 17, X15 19, X16 6, X17 24, X18 8, X19 12, X20 12, X21 10, X22 4 and X23 5. Run on X23 alone with `--strict`, the tool exits 0, as X23's header says it did on the table before the repair.

My diff of 1e20bfa2 against a15a877d finds 17 change-table rows in 11 reports whose text changed in phase 11. These reach a design:

| Report | Rows | Effect on the design |
|---|---|---|
| X22 (P23a) | DEP-P6 | The independent case is now "need not diagnose" (section 1.3). With Q-0412, the design needs a note |
| X21 (P15) | TR-P1 | TR-P1 now states the policy that X21-4 asked to move there: every link-time break except a missing `main` is `no-diagnostic`. X21's row cites TR-4 for that policy. X21:new-4 stays `no-diagnostic`, and its row should cite TR-P1 |
| X14 (P20) | ALN-4 | X14 amends ALN-4 with an exception for an under-aligned member. ALN-4 now has a second exception, a class allocation function that NEW-29 calls. The two compose, but X14's amendment names one exception only |
| X9 (P8) | OBJ-1 | OBJ-1 now says when the object of a delegating constructor counts as complete. That answers X9-6, which X9:new-6 needed for a delegating relocation constructor |
| X23 (P25) | NEW-29, NEW-P5, ALN-4, STG-5, NEW-21 | The repair took X23:new-1's second case, the STG-5 sentence of X23:new-2, and the ALN-4 split. P25 still withdraws NEW-29's retry and still changes 2 tests from `run` to `reject` |

The other rows add cases that the designs do not use: PACK-13 and PACK-P2 in X6, PACK-13 in X13, CEV-1 in X7 and X10, ORD-13 in X7, NAM-21 in X15, and OPR-P3 in X19.

The two limits that rounds 6 to 8 named remain. X1 to X12 name no commit, so the tool lists no changed row for them. The tool does not name the commit that changed a row, and does not read refresh notes.

### 1.6 What a designer still cannot state

**Which class a rule on the program gets when a `required` rule states it.** TR-P1 says a `required` rule is a fact about the implementation. TR-4 is `required` and says a program defines `main`, with a diagnostic. The RV4 review says the class of TR-4 and the class of that requirement differ. ALN-4 is `required` and says the program must make a class's function return aligned storage, with no class for a break. The RV4 review also says TR-P1 does not cover two definitions of `main` in two units. RV4's repair commit left these to the next review.

**What a standard attribute may do.** Q-0412 holds a provisional answer and waits on the maintainer (row 9). The answer fits X12 and X18 and does not fit X22.

**Whether a later version has taken the space.** 13 of the 15 reports from X9 to X23 raise it. X23 does not. The row 7 plan is costed. Its count is stale (section 1.2).

**What a change costs.** No change. Q-0183 keeps option 1.

**How much code a change touches.** Three kept scanners now use three methods: X21 and X22 scan text, and X23 adds a compiler run with an injected header of deleted declarations, beside a GCC warning. X23 4.4 says again that the corpus does not measure real use: none of the 103 programs declares a class-scope allocation function.

**Which programs change meaning without a diagnostic.** X23 4.3 argued by hand again, with 14 files that hold a test of validity with `new`. 40 of 558 `reject` rules have such a test.

**How two changes compose.** No pair exercise has run. X14 and X23 now both amend ALN-4.

**Validity on the other target.** X23-3 depends on the `-m32` values of ALN-5. The 76 tests with `-m32` flags are unchanged.

### 1.7 The portfolio

| # | Proposal | Round 8 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P25 | Aligned `new` and `delete` | ready | done (X23) | X23, repairs ee6085aa | X23-3 and X23-6 (planned, item ended). Q-0300 (row 4a) and Q-0417 (first review) do not change P25's rules |
| P26 | No overloaded comma, checked left operand | ready | ready | | The meaning of "has no value" for a class result. A fold over the comma operator: `r8-fmt.cpp` and `r7-registry/registry.h` use one, with calls as operands. The design must say what the "left operand" of a fold is |
| P24 | Flag enumerations | ready | ready | Q-0412 gives a provisional answer | The marker. Under Q-0412, the marker is a keyword or contextual keyword and must not clash with X12's `final`. ENUM-10 sits in row 4a (Q-0407) |
| P22 | Labeled `break` and `continue` | ready | ready | | The marker of the target. Version marking (row 7) |
| P23b | Expressions checked against requirements | partial | partial | Q-0263 settled | No matching rule (X22-8). Q-0411 (doubt). Q-0412 means its marker cannot be an attribute if it adds rejections |
| P8b | Relocation by bytes | partial | partial | | Row 2 (Q-0322) |

Totals now: 23 done, 3 ready (P22, P24, P26), 2 partial (P8b, P23b), 0 blocked. P26 is the one ready proposal that changes existing behavior. Its page has the most commits since a single reading among the ready proposals' pages.

Rough counts for P26 by `grep`: 7 files outside `tests/programs/` declare `operator,`, and no program does. Two programs use a fold over the comma operator, each with calls as operands.

### 1.8 Questions and principles

Phase 11 opened 7 questions (Q-0412 to Q-0418) and settled 4 (Q-0263, Q-0386, Q-0395, Q-0413). No open question waits for a use of a principle. Q-0263 settled through the use of CNS-P2. Q-0299's use of LSUP-P2 reads with a doubt, so Q-0299 waits for a review. The review says the reference makes such a declaration undefined, and Tidal chose `no-diagnostic`. Q-0299 bears on P25's migration, and X23 3 says P25 adds no requirement there.

Principles that proposals found weak. The findings I could parse (239 of the 269 X rows in the ledger) name 67 principles, and 75 findings name at least one. The most named are CE-P4 (4 findings), JMP-P2, TRY-P4, SINIT-P2 and CNS-P3 (3 each). Commits closed every finding on those five. The principles with a Propose finding that has no answer in the text:

| Principle | Finding | State |
|---|---|---|
| ATTR-P1, ATTR-P2, ATTR-P4 | X14-7, X22-6 | Q-0412, pending row 9. Bears on P22, P23a, P23b and P24 |
| STG-P3 | X23-6 | planned, item ended |
| NEW-P5 | X23-1 | text repaired. The program's duty has no class (section 1.6) |
| TR-P1 | X21-4 | repaired in item F. RV4 left two definitions of `main` and TR-4's class |

Three more principles have a review doubt that a designer meets. DEP-P6 now holds a "need not diagnose" row that X22's check makes asymmetric. CNS-P2 names one case of an atom that can never pass, and the RV4 review says other such atoms fall under the same reference rule. LSUP-P2 chose a class that the review says has no direct base in the reference.

## 2. Ranked items

Front-end round 23 (item FE) comes first in the next phase. The items below rank what a designer would put beside it. Item 4 runs inside FE's lane.

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Answer X23-3, X23-6 and X23-7, give ALN-4's duty a class, and settle TR-4's class | 1.2: 20 `planned` rows with an ended item; 1.3; 1.6 | 1 writer session, with probes | low | P25's record; the class policy that every proposal reads |
| 2 | A Propose prompt in `exercises.md` section 9, then X24 on P26, then the stop-rule count | 1.4: comma.md read once, 5 commits since; 1.3: 10 of 16 closed over X22 and X23; X23-7 | 1 short writer step, 1 reader session, 1 repair session | low | the second reading of `comma.md`; the stop rule's first full count |
| 3 | Refresh notes for X22, X21, X14, X9 and X23, and a refresh check that reads them and names the changing commit | 1.3, 1.5: 17 rows changed in phase 11, 5 of them by X23's own repair; Q-0412 against X22 | 1 reader session, a small tool change | low | reuse of the done designs; item 7 |
| 4 | A front-end prototype of P25, in front-end round 23's lane | 1.1: the front end has `MISSED-REJECT` on NEW-30's two tests; round 23 takes the two new NEW-29 tests | part of 1 front-end session | low | a measured check of X23's section 4 |
| 5 | A shared impact scanner from the X21, X22 and X23 scanners | 1.6: three kept tools, three methods | 1 writer or front-end session | low | section 4 of X24 and later reports |
| 6 | The later-version probe: a maintainer decision on row 7, with the plan's count updated | 1.6: 13 of 15 reports from X9 on raise it; Q-0411 doubt | one decision, then one session and one matrix run of about 8235 compilations | medium | the version line of P22, P23 and P24 |
| 7 | Pair exercises, starting with X14 and X23 | 1.5: both amend ALN-4 | 1 reader session per pair | low | an interaction section that can be checked |
| 8 | An index of what a test of validity can see | 1.6: 40 of 558 | 2 writer sessions | medium | section 4.3 of every proposal |
| 9 | ILP32 runs for the alignment and conversion tests | 1.3: X23-3 rests on `-m32` values; 76 tests | 1 writer session, a matrix run | low | the target line of a proposal |

### Item 1. Answer X23's planned rows, and the two class gaps

**What it is.** A writer session with five parts.
- X23-3: NEW-27 states the relation between the default new alignment and the largest fundamental alignment on each target in the ALN-5 table. UNW-3, NEW-22 and ALN-4 use NEW-28's threshold, or say why they differ. Probes on Clang and GCC with `-m32` decide the relation, since the ALN-5 table already shows 8 against 16.
- X23-6: STG-8 or STG-P3 says whether an implicit object can exist in storage too weakly aligned for its type. A sanitizer probe shows what each compiler reports.
- ALN-4's duty: the writer gives the break a class, with a rule that cites ALN-4, or moves the duty into a rule whose class states it. X23's probe `w1.cpp` becomes a sanitizer test.
- TR-4: the writer moves the requirement on `main` into a rule whose class matches TR-P1's table, or says in TR-P1 why a `required` rule holds it. TR-P1 says which rule governs two definitions of `main` in two units.
- X23-7: section 4 of `exercises.md` says that a proposal that changes existing behavior fills 4.3 and 4.4 in full, and names X23 in its prose. Section 8 lists X21, X22 and X23 in order.

The ledger then marks the five rows. Decision 0058 has the closing commit list `planned` rows whose item ended. The next phase decision should name an item for the other 15 (I3, R10, P1 and I4 rows).

**Evidence.** Sections 1.2, 1.3 and 1.6.

**Cost.** One writer session, with probes on two targets.

**Risk.** Low. The TR-4 change touches the class policy that every proposal reads, so the next gap review reads it.

**What it unblocks.** A complete record for P25. A class policy that X21 and later designs can cite without a caveat.

### Item 2. A Propose prompt, then X24 on P26, then the stop-rule count

**What it is.** Three steps.
- `exercises.md` section 9 gains a Propose prompt. Rounds 6 to 8 asked for one, and X23-7 shows the cost of its absence. The prompt says: run `propose_check.py` and `--corpus` first; read the refresh notes beside a report before relying on it; fill 4.3 and 4.4 in full for a change of existing behavior; state a version line, or say why none applies; save the change table where `--strict` can read it.
- X24 takes P26 on `expressions/comma.md`. The design states what "has no value" means for a class result, whether a call through a pointer or of an operator function counts as a call, and what the left operand is in a fold over the comma operator. Section 4 counts the 7 files that declare `operator,` and the 2 programs with a comma fold. P26 meets the overloaded operators that I3 modeled and the comma order of OPR-7.
- After X24, the designer view counts findings over X22 to X24, as round 7's stop rule asks.

P22 and P24 stay ready. P24 can use Q-0412's provisional answer and take a keyword marker. P22 waits for row 7 if its report is to have a version line.

**Evidence.** Sections 1.3, 1.4 and 1.7.

**Cost.** One short writer step, one reader session and one repair session.

**Risk.** Low for similarity. P26 changes existing behavior, so its section 4 needs a scan (item 5 helps).

**What it unblocks.** The second reading of `comma.md`. The stop rule's first full count.

### Item 3. Refresh notes where phase 11 text reaches a design, and a refresh check that reads them

**What it is.** Round 8 item 4, with a new order.
- X22 first. The note says that Q-0412's provisional answer rules out `[[checked]]` as an attribute that adds rejections, and names the replacement marker. It says whether the check also requires the independent case that DEP-P6 now lets go undiagnosed.
- X21: its TR-4 row now rests on TR-P1. The note also covers the SINIT-5 change of phase 10.
- X14: the second ALN-4 exception. X9: OBJ-1 now answers X9-6. X23: which of its rows the repair already took.
- Then the three old class conflicts (X4 CTOR-17, X7 CE-11, X9 STG-25).
- `tools/propose_check.py` gains three behaviors. It reads a refresh note beside a report and counts the conflicts and rows the note covers as "noted". It names the latest commit that changed each row, so a row changed by the report's own repair commit shows as such. It uses the commit that added a report when the report names none.

**Evidence.** Sections 1.3 and 1.5.

**Cost.** One reader session for the notes. A small tool change, with each value from `git log`.

**Risk.** Low. The check stays a warning without `--strict`.

**What it unblocks.** Item 7. Reuse of X22, X21 and X14 in later proposals. P23b, which builds on X22.

### Item 4. A front-end prototype of P25, in front-end round 23's lane

**What it is.** Round 8 item 6, with a new first case. Front-end round 23 already takes `aligned-deleted-form-beside-unaligned.cpp` and `aligned-private-form-beside-unaligned.cpp`. The front end shows `MISSED-REJECT` on NEW-30's two tests. P25 is the mirror of NEW-30 for over-aligned types. A prototype behind a flag removes NEW-29's retry, keeps the non-allocating exemption, and runs the tests and the 103 programs. X23 predicts 2 tests from `run` to `reject`, the 3 programs `W4/arena.cpp`, `W4/vector.cpp` and `W13/aligned.cpp` unchanged through the exemption, and 3 programs invalid without it. The prototype stays out of `main`'s front end.

**Evidence.** Section 1.1 and the `new-delete` rows of `conformance.tsv`.

**Cost.** Part of one front-end session, after round 23's NEW-29 and NEW-30 work.

**Risk.** Low.

**What it unblocks.** The first measured check of a Propose report's impact count.

### Item 5. A shared impact scanner

Round 8 item 5, with a third starting point. X23's `scan_one.sh` injects a header of deleted declarations and compiles each file. That method finds uses that a text scan misses, such as a placement form that the language picks. One tool under `tools/` takes the queries of X21, X22 and X23, honors `tidal-flags`, and reports `tests/programs/` apart from the rule tests. One session for the frame, and each later Propose report adds its query.

### Item 6. The later-version probe

Round 8 item 3, unchanged, with one record fix. The plan in `pending.md` should say about 2745 tests and 8235 compilations, and a tool should write that count. The probe now also informs Q-0411, where the RV4 review names a possible later change.

### Item 7. Pair exercises, starting with X14 and X23

Round 8 item 8, with a new first pair. X14 makes some members under-aligned, and X23 makes `new` honor every alignment. Both amend ALN-4. The pair asks what `new` does for a packed class whose members have alignments below their types'. X17 with X19, and X12 with P24, follow. One reader session per pair, after item 3's notes.

### Item 8. An index of what a test of validity can see

Round 6 item 2 to round 8 item 7, unchanged. 40 of 558 `reject` rules have an `accept` or `run` test that uses `requires`. X23 4.3 found 14 such files with `new` by hand. Two writer sessions, at medium risk.

### Item 9. ILP32 runs for the alignment and conversion tests

Round 8 item 9, extended. X23-3 rests on the `-m32` values of ALN-5. The item adds `-m32` variants of the NEW-27 and NEW-28 tests, beside the conversion tests of CONV-2 to CONV-11, CONV-19, CONV-21 and LIST-2. One writer session and a matrix run.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them.
- **P23b in the next phase.** It still needs a matching rule. Q-0411 holds a doubt, and Q-0412 affects its marker.
- **P8b** until the maintainer decides row 2.
- **Rules or `:since:` fields for later versions.** Item 6 makes an index only.
- **A source-reader designer** (Q-0116). Twenty-three exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2 in full).
- **A model of object files or mixed implementations.** Only X21 needs one.
- **Full re-runs of old Propose reports.** Item 3's notes cover the changes that reach a design.

## 4. Process changes

1. **A `planned` row names a live item.** When the item a `planned` row names ends without an answer, the closing commit moves the row to the next phase's item, or to `open`. Twenty rows now name an item that is done.
2. **A repair names the done designs its text change reaches.** Round 8 process change 2, not taken. Item 3's tool change lets the repair commit list them. Commits 8d7f3bb3 and ee6085aa show the need.
3. **A repair that states a program's duty gives the duty a class.** ALN-4 and TR-4 show a rule on the program inside a `required` rule. The check could flag `required` rules whose text says "the program must".
4. **A Propose prompt in `exercises.md` section 9** (item 2). Rounds 6, 7 and 8 asked for it.
5. **Keep the exercise records current.** Section 4's prose names the latest round, and section 8 lists the Propose rows in order.
6. **The stop rule stays.** It counts X22 to X24. X22 and X23 closed 10 of 16 findings in the text and found one wrong rule.
7. **Keep the designer's view in round 10** in this form. Rerun the counts of sections 1.1, 1.4, 1.5 and 1.8, and the diff of changed rows against each done design.

## 5. The decision packet from a designer's view

The packet in `documentation/decisions/pending.md` gives, for each row and option, the open questions it settles and the tests whose notes name them. A designer would settle these rows first. The order follows the number of proposals that each row reaches.

| Order | Row | What it settles | Tests whose expectation changes | Proposals and done designs it reaches |
|---|---|---|---|---|
| 1 | 9, Q-0412 | which effects a standard attribute may have | 0 under either option (no test names Q-0412) | P22, P23a (X22), P23b and P24 choose a marker by it. X12 and X18 reasoned from the same principles. Option 1 keeps ATTR-P2 and rules out X22's attribute. Option 2 changes ATTR-P2 and fits X22 |
| 2 | 4a | 18 questions where every newest family keeps the matrix's answer | 0 if the maintainer follows the compilers; 32 if the review | Q-0300 (P25's mirror rule NEW-30; X23 3 says no P25 rule depends on it), Q-0407 (ENUM-10, which P24 and X20 cite), Q-0181 (a later change) |
| 3 | 7 | Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 tests name them; the probe changes no expectation | the version line of 13 Propose reports, and of P22 and P24 |
| 4 | 9, Q-0183 and Q-0186 | a cost model; judgments from outside Tidal | 0 | section 4 and the outside-knowledge line of every Propose report |
| 5 | 2 | Q-0322 | 0 | P8b |

Rows 1 and 10 settle the most questions with no change of expectation (56 questions and 132 tests in row 1, 15 and 27 in row 10). Of row 1's questions, Q-0383 (X19's area) reaches a done design. I did not check row 10's questions against the designs. I found no proposal that rows 3, 5 and 8 reach. Row 6's Q-0343 says when a class is trivially copyable (CPY-19), which P8b's relocation by bytes would cite, by my reading.

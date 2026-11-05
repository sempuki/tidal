# Roadmap, round 7: what to do after phase 9, from a language designer's view

Date: 2026-11-05. Decision 0029, with decisions 0037, 0039, 0055 and 0056. Role: clean-room reader, language-designer persona. Inputs: AGENTS.md and CLAUDE.md; `reports/clean-room/roadmap-6/designer.md` (the model for this form) and `reports/clean-room/roadmap/designer.md` (the portfolio); `documentation/` (charter, roadmap, status, exercises, specification format, decisions 0039, 0055 and 0056, and `decisions/pending.md`); `reports/clean-room/X21/` in full and the headers of X17 to X20; the headers and findings lists of `I2/`, `R9/`, `PU1/`, `PU2/`, and `rust-frontend/report-20.md` and `report-21.md`; `questions/README.md` and the files Q-0175 and Q-0383; OPR-9, TR-P1 and the static-initialization rules under `specification/` through targeted searches; `reports/clean-room/rust-frontend/conformance.tsv` for the program rows; `tools/propose_check.py` and its output, with and without `--corpus`; and `git log 1b16115..HEAD` with the bodies of 2ebb5c3, 926831c, f2ac11c, 70e56be, d9c18bc, 32c89f7, 789c04e and 017b401. Commit read: `017b401`. Counts come from read-only Python scripts in my scratch directory, run on that tree. I did not read the other round-7 reports. I did not run `tools/similarity.py`. Statements from outside Tidal's files are marked "general knowledge".

## 1. The state from a designer's view

### 1.1 Counts at 017b401

| Measure | Round 6 (1b16115) | Now (017b401) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1696 | 1697 |
| `required` | 898 | 898 |
| `reject` | 558 | 558 |
| `undefined` | 57 | 57 |
| `impl-defined` | 52 | 52 |
| `recommended` | 44 | 43 |
| `no-diagnostic` | 36 | 37 |
| `unspecified` | 31 | 32 |
| `conditionally-supported` | 5 | 5 |
| `withdrawn` | 15 | 15 |
| Test files with a `tidal-expect` line | 6497 | 6619 |
| Of these, `reject` | 2684 | 2726 |
| Of these, `link-fail` | 44 | 44 |
| Of these, with `-m32` | 75 | 85 |
| Program corpus (`propose_check.py --corpus`) | 71 programs, 9185 lines (my count) | 85 programs (71 Write, 14 sketches), 5 multi-unit, 103 files, 10361 lines |
| Questions | 385 | 400 |
| Provisional | 221 | 172 |
| Settled | 151 | 215 |
| Provisional questions that wait for the next use of a named principle | about 60 | 9 |
| Propose reports | 20 | 21 |
| Change-table rows that `propose_check.py` reads | 971 | 1003 |
| Front end passes (status page) | 6183 of 6500 | 6362 of 6619 |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 32 | 37 |

My test count now matches the status page. Three rules changed class. ORD-17 is new (`required`). OPR-9 moved from `recommended` to `no-diagnostic`, and LAM-28 moved from `required` to `unspecified`, both in commit 1bf7812 (item RV2).

### 1.2 What phase 9 did for the designer

Phase 9 took one of round 6's eight items. Decision 0056 deferred three of them by name.

| Round-6 item | Phase 9 | State |
|---|---|---|
| 1. The later-version probe (`pending.md` row 7) | none | waits on the maintainer. The plan still names about 2640 `reject` tests, and there are 2726 |
| 2. An index of what a test of validity can see | none | deferred by decision 0056 |
| 3. A shared impact scanner | none | deferred by decision 0056. X21 saved its own scanner, `reports/clean-room/X21/scan_programs.py`. `propose_check.py --corpus` prints the corpus size only |
| 4. Propose X21 on P15 | item E, commits 70e56be and d9c18bc | done. 11 findings, 9 closed |
| 5. New proposals, then X22 to X24 | none | deferred by decision 0056 |
| 6. The refresh check for reports with no commit, and the commit that changed each row | none | not started. X1 to X12 still get no change list |
| 7. Pair exercises | none | deferred by decision 0056. Proposed in rounds 3 to 6 |
| 8. ILP32 runs for tests of conversion rules | none | not started. `-m32` tests went from 75 to 85 through other work |

Of round 6's seven process changes:
- Taken: the Propose table in `exercises.md` section 4 lists X17 to X21 (X21-11). X21 saved its probes and scanner in its report directory. This report keeps the view.
- Overtaken: decision 0056 defined a use of a principle. A use applies a principle to the question's own program, so a citation in a Propose report is not a use.
- Not taken: `exercises.md` section 9 has no Propose prompt. Section 8 gives finding counts with no count of what was closed. Section 8 still says 57 rule changes for X14 and X20, and the tool reads 56 and 58. The table still names P17 "opaque type aliases". Section 4's prose still names rounds 1 to 3 only, and section 6 still orders the items of decision 0055.

Section 8 has no row for PU2.

### 1.3 What X21 found, and what the repairs closed

| Exercise | Proposal | Findings | Closed in the specification | To a question | No action |
|---|---|---|---|---|---|
| X21 | P15, declared initialization order | 11 | 9 (X21-1 to X21-8, X21-11) | X21-1 cites Q-0175, which gains a dated note | X21-9, X21-10 |

Commit d9c18bc acted on all nine findings that a writer can act on. X13 to X16 closed 31 of 39, X17 to X20 closed 36 of 44, and X21 closed 9 of 11. A first designer reading of a page area still finds about nine gaps.

Two findings were wrong text on pages that Predict batches had passed:
- X21-1: SINIT-3 and SINIT-P2 said a read of another unit's variable gives "its final value or zero". For a member of a class object, OBJ-2 calls the read undefined.
- X21-2: STO-10 said the object whose initialization completed last is destroyed first. OBJ-4 places a constant-initialized object at its definition, and the OBJ-4 test shows the other order.

X17 to X21 together found seven such errors in five exercises.

X21-4 moved a class policy to where a designer looks for it. TR-P1 now says, after its class table, that no class requires a diagnostic when the units are linked.

The needs outside the specification, with X21 added:

| Need | Raised by X21 | Raised before |
|---|---|---|
| Version marking | X21-9 | X13 to X20 |
| Implementation cost (Q-0183) | X21-9 | X14, X15, X17 to X20 |
| Mixed implementations | X21-9 (a unit built without P15 has no entry, and the list silently gives today's order) | none |
| Target dependence | none | X17-11 |

The specification names no object file and no ABI. My search of `specification/` for either term finds no line.

### 1.4 Pages the Propose exercises have reached

All 85 pages are reached, by `:::{rule}` IDs cited in X1 to X21. X21 reached `declarations/inline.md` a second time. Six pages are reached by one exercise only:

| Page | Exercise | Commits to the page since that exercise read it, or since the report was added |
|---|---|---|
| `templates/constraints.md` | X8 | 15 |
| `declarations/alignment.md` | X14 | 10 |
| `declarations/namespaces.md` | X15 | 7 |
| `expressions/comma.md` | X1 | 5 |
| `statements/loops.md` | X11 | 5 |
| `expressions/bitwise.md` | X17 | 1 |

`templates/constraints.md` is still the page most changed since its one reading.

### 1.5 The refresh check at 017b401

`tools/propose_check.py` reports 21 reports, 31 conflicts (10 withdrawn, 4 class, 17 new-id, 0 unknown), 120 rows changed since their report's commit, and 0 warnings. The changed rows are X13 13, X14 16, X15 19, X16 6, X17 24, X18 8, X19 13, X20 12 and X21 10.

The fourth class conflict is new. X19's table lists OPR-9 as `recommended`, unchanged, and OPR-9 is now `no-diagnostic`. The change reaches X19's design. X19 section 4.3 counts a program that declares `operator""u8` as valid today, so that P1 changes its meaning with a warning. Under the present OPR-9 that program is not valid today, and no diagnostic is required. P1's change for that program moves from "valid programs that change meaning" toward "a program with no meaning gets one". Q-0383 is still provisional on whether such a literal calls the operator.

The two limits that round 6 named remain:
- X1 to X12 name no commit, so the tool lists no changed row for them.
- A change list does not say which commit changed a row. Of X21's 10 changed rows, all but at most one changed in d9c18bc, which acted on X21's own findings. The output does not show that.

### 1.6 What a designer still cannot state

**Whether a later version has taken the space.** X21-9 raises it for the place after a variable's declarator. So 12 of the 13 Propose reports from X9 on raise it. The plan in `pending.md` is written and costed, and Q-0007 is still provisional.

**What a change costs.** X21-9 names one entry per external variable with dynamic initialization and one progress state per unit, with no scale to put them on. Q-0183 keeps option 1.

**How much code a change touches.** X21 built a fifth impact tool. It is the first one kept in the repository, and it answers one query (namespace-scope variable definitions). The front end passes every program file (72 `pass`, 9 `pass-partial`, 4 `EDG-ONLY` in `conformance.tsv`), so a scanner or a prototype built on `tidal-fe` would read the whole corpus.

**Which programs change meaning without a diagnostic.** X21 4.3 argued by hand that no test of validity can see the clause. My scan finds 37 of the 558 `reject` rules cited by an `accept` or `run` test whose text contains `requires`. No index says which rejections such a test can see.

**What happens when units come from different implementations.** X21 is the first report to need this. P8 (relocation) and P20 (packed classes) touch layout and calling conventions too (general knowledge). Tidal has no model of object files.

**How two changes compose.** No exercise has written an interaction between two done proposals. The OPR-9 change in 1.5 shows that X19 now composes differently with the present text than it did with the text it read.

**Validity on the other target.** No change since round 6 beyond 10 more `-m32` tests.

### 1.7 The portfolio, updated

| # | Proposal | Round 6 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P15 | Declared initialization order | ready | done (X21) | | |
| P8b | Relocation by bytes | partial | partial | | `pending.md` row 2 |

Totals now: 21 done, 0 ready, 1 partial (P8b), 0 blocked. No proposal is left that a reader can take without a maintainer decision.

### 1.8 Questions and principles

Phase 9 settled 64 questions, and 27 of them through the principle-use batches PU1 and PU2. Nine provisional questions still wait for the next use of a named principle: Q-0018 (ENUM-P3), Q-0175 (SINIT-P2), Q-0217 and Q-0293 (ORD-P2), Q-0218 (ORD-P1), Q-0340 (UNI-P6), Q-0378 (COND-P2), Q-0383 (OPR-P3), and Q-0386 (LSUP-P1 and CMP-P5). Q-0175 waits on SINIT-P2 because the X21 repairs changed SINIT-P2. Q-0383 belongs to X19's area. 53 provisional questions cite `pending.md`.

PU2-1 bears on P2. The principles do not say whether `int` to `double` narrows when the target holds every `int` value, and X17 designed P2 against that same definition. Commit 32c89f7 answered PU2-1.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | A refreshed portfolio, then Propose X22 to X24 | 1.7: no ready proposal; 1.4: six pages read once; 1.3: about nine gaps per first reading | 1 portfolio session, 3 reader sessions, 1 repair session | low | the Propose lane; second readings of thin pages |
| 2 | The later-version probe: a maintainer decision on `pending.md` row 7 | 1.6: 12 of 13 reports from X9 on raise version marking | one decision; then one session, one matrix run, triage | medium | the version line of every proposal; Q-0007, Q-0087, Q-0181, Q-0188 |
| 3 | A shared impact scanner, started from X21's | 1.6: five scanners, one kept | 1 writer or front-end session | low | section 4.5 of every proposal |
| 4 | A front-end prototype of one done proposal | 1.6: the front end reads the whole corpus; no report's impact count has been checked | 1 front-end session | low | a measured check of a Propose report's impact section |
| 5 | Refresh notes for the class conflicts, and the refresh check for X1 to X12 | 1.5: 4 class conflicts, one of which reaches X19's design; X1 to X12 unlisted | 1 reader session and a small tool change | low | pair exercises; reuse of old reports |
| 6 | An index of what a test of validity can see | 1.6: 37 of 558 `reject` rules have such a test | 2 writer sessions | medium | section 4.3 of every proposal |
| 7 | Pair exercises, starting with X17 and X19 | 1.6: no exercise states how two changes compose | 1 reader session per pair | low | an interaction section that can be checked |
| 8 | ILP32 runs for tests of conversion rules | 1.6: 85 of 6619 tests run `-m32` | 1 writer session and a matrix run | low | the target line of a proposal that changes conversions |

### Item 1. A refreshed portfolio, then Propose X22 to X24

**What it is.** A designer session writes proposals P22 onward into a new portfolio file, rated as in round 3, and renames P17 to "explicit aliases" (X20-7). Each new proposal must be one a reader can take without a maintainer decision. The round-6 candidates still fit, in this order:

| # | Candidate | Pages it aims at | Changes existing behavior |
|---|---|---|---|
| P23 | Template bodies checked against their constraints | constraints, concepts, dependent-names, instantiation | yes |
| P25 | `new` and `delete` that honor every alignment, or reject it at compile time | new-delete, alignment, storage | yes |
| P22 | Labeled `break` and `continue` | loops, jumps, blocks | no |
| P26 | No overloaded comma, and a discarded left operand of `,` must have no value or be a call | comma, operators, evaluation-order | yes |
| P24 | Flag enumerations: a marked scoped enumeration gets `|`, `&`, `^` and `~` in its own type | bitwise, enumerations, overload-resolution | no |

X22 takes P23, because `templates/constraints.md` has 15 commits since its one reading. X23 takes P25, because `declarations/alignment.md` has 10. P25 must cite the answer that `pending.md` row 4 keeps for Q-0300. X24 takes P22 or P26. The motivation for each candidate is general knowledge, from other languages and common practice.

**Evidence.** Sections 1.3, 1.4 and 1.7. The portfolio is empty of ready proposals. Each first reading of a page area has found about nine gaps, and X17 to X21 found seven errors in text that Predict had passed.

**Cost.** One portfolio session, three reader sessions, one repair session.

**Risk.** Low for similarity. P22 and P23 are areas where a later version may act (general knowledge), which raises the value of item 2. P23 is large, and its first exercise may cover one form of check only.

**What it unblocks.** Second readings of six pages. A Propose lane that does not end with phase 9.

### Item 2. The later-version probe: a maintainer decision on `pending.md` row 7

**What it is.** Round 6 item 1, unchanged. The maintainer decides row 7. The plan's count of `reject` tests should read 2726.

**Evidence.** Section 1.6. Every Propose report from X13 on lists version marking as a need outside the specification.

**Cost.** One decision. If approved, one session for the runner option, one matrix run of about 8200 compilations, and triage.

**Risk.** Medium, in triage, as the plan says.

**What it unblocks.** The version line of every proposal, including those of item 1.

### Item 3. A shared impact scanner, started from X21's

**What it is.** One tool under `tools/` that a Propose reader runs to count constructs in `tests/` and `tests/programs/`. It starts from `reports/clean-room/X21/scan_programs.py` and adds the queries that X17 to X20 needed: implicit conversions by context and type, overriding declarations with and without `override`, literals by base and suffix, and alias declarations and scoped enumerations. It honors `tidal-flags`, so a `-m32` test counts on its own target, and it reports `tests/programs/` apart from the single-rule tests. The frame can read a compiler's syntax tree, as X17's did, or `tidal-fe`, which parses all 85 program files.

**Evidence.** Section 1.6. Five readers built five impact tools, and only X21's scanner is kept.

**Cost.** One session for the frame and five queries. Each later Propose report adds its query.

**Risk.** Low. A syntax tree from a compiler is implementation behavior. `tidal-fe` is Tidal's own code.

**What it unblocks.** Measured counts in section 4.5 of every proposal, and of item 1's three. A count of affected sites, which is one part of Q-0183's option 2.

### Item 4. A front-end prototype of one done proposal

**What it is.** The front-end reader adds one done proposal to `tidal-fe` behind an option, in a worktree that does not change the conformance baseline. The reader runs the suite and the programs with and without the option and compares the changed outcomes with the proposal's table. P2 is the best first case. X17 predicted 74 tests that change to `reject` and 3 program sites. P18 is the cheapest case, with a prediction of no outcome change and 23 `override` declarations.

**Evidence.** Section 1.6. No Propose report's impact count has been checked by an implementation of the proposal. The front end passes 6362 of 6619 tests and every program file. Round 5 item 8 asked for this probe, and round 6 kept it out of scope until the front end failed fewer tests.

**Cost.** One front-end session. Only tests that pass today count, so the front end's own failures do not enter the comparison.

**Risk.** Low. The prototype stays out of `main`'s front end.

**What it unblocks.** A measure of how far a Propose report's section 4 can be trusted. A first answer, from Tidal's own implementation, to "what a change costs".

### Item 5. Refresh notes for the class conflicts, and the refresh check for X1 to X12

**What it is.** Two parts.
- A reader writes a short refresh note for each report with a class conflict: X4 (CTOR-17), X7 (CE-11), X9 (STG-25) and X19 (OPR-9). Each note says whether the new class changes the report's section 4, and appends to the report directory without editing the report. X19 comes first, for the reason in 1.5.
- `tools/propose_check.py` uses the commit that added a report when the report names none, and says so. Each changed row names the latest commit that changed its directive text. Round 6 item 6 describes both changes.

**Evidence.** Section 1.5.

**Cost.** One reader session for the four notes. A small tool change, with both values from `git log`.

**Risk.** Low. The check stays a warning.

**What it unblocks.** Item 7, which reads X17 and X19. Reuse of X5 to X12 by later proposals.

### Item 6. An index of what a test of validity can see

**What it is.** Round 6 item 2, unchanged. For each `reject` rule, one value: visible (the error can occur where substitution or a requires expression sees it), fatal (the error always ends translation), or syntax (only the grammar rejects the form). A sample of the visible rules gets a test that observes the rejection from a requires expression.

**Evidence.** Section 1.6. X10, X12 and X16 to X21 each decided visibility by hand.

**Cost.** Two writer sessions.

**Risk.** Medium. The value of some rules depends on position.

**What it unblocks.** Section 4.3 of every proposal.

### Item 7. Pair exercises, starting with X17 and X19

**What it is.** An exercise takes two done proposals that touch the same rules and writes only their interaction, with a change table for the combined rules. X17 and X19 (narrowing everywhere and width-named suffixes) come first. Both read dad8bf8, and both amend conversions of small integer types. X17 and X20 come second.

**Evidence.** Proposed in rounds 3 to 6. Section 1.5 shows that the present text has moved under X19.

**Cost.** One reader session per pair, after item 5's X19 note.

**Risk.** Low.

**What it unblocks.** A way to state how two changes compose.

### Item 8. ILP32 runs for tests of conversion rules

**What it is.** Round 6 item 8, unchanged. `-m32` variants for the tests of CONV-2 to CONV-11, CONV-19, CONV-21 and LIST-2 whose result depends on the widths of TYP-1. `tidal-fe` has a `--m32` option, so item 3's scanner can count them on both targets.

**Evidence.** X17-11. 85 of 6619 tests run `-m32`.

**Cost.** One writer session and a matrix run over the changed directories.

**Risk.** Low.

**What it unblocks.** The target line of a proposal that changes conversions or integer types.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them.
- **A model of object files or of mixed implementations.** Only X21 needs one, and Tidal has no clean input for it. A proposal states the need in its outside-the-specification finding, as X21-9 does.
- **Relocation by bytes (P8b)** until the maintainer decides `pending.md` row 2.
- **Rules or `:since:` fields for later versions.** Item 2 makes an index only. Q-0185 stays provisional.
- **A source-reader designer** (Q-0116). Twenty-one exercises have not needed one.
- **The preprocessor beyond `#include`.** No proposal needs more.
- **A cost model with numbers** (Q-0183 option 2 in full). Items 3 and 4 give counts and one implementation's effort.
- **Full re-runs of old Propose reports.** Item 5's notes cover the conflicts that change a class. A full re-run repeats the design for a proposal that stays out of the specification.

## 4. Process changes

1. **What the Propose lane does now.** It writes a refreshed portfolio and runs three new exercises (item 1). It writes refresh notes only where a class conflict reaches a report's design (item 5). It does not re-run old reports in full, and it does not stop.
2. **A stop rule for the lane.** After X24, the designer view in round 8 counts the findings closed per exercise and the wrong-text findings. If a round of three exercises closes fewer than five findings each and finds no wrong text, the lane stops new proposals and keeps refresh notes and pair exercises only.
3. **A Propose prompt in `exercises.md` section 9.** Round 6's text still applies: run `tools/propose_check.py` first, name the commit read, use the table headers the tool reads, flush probes, quote the corpus from `--corpus`, run the impact scanner and add a query, and save probes and scanner in the report directory, as X21 did.
4. **Keep the exercise records current.** Section 8 takes rule-change counts from `propose_check.py`, gives closed, to-a-question and no-action counts, and gains a PU2 row. Section 4's prose names rounds 4 to 6 (X13 to X21). Section 6 orders the items of decision 0056 and its successor.
5. **A commit that changes a rule's class names the Propose reports that cite it.** `documentation/specification-format.md` asks this for a withdrawn rule. OPR-9's change in 1bf7812 reached X19 with no note. Running `propose_check.py` in the repair commit and listing the new class conflicts would show it.
6. **Keep the designer's view in round 8,** in this form, with the counts of sections 1.1, 1.4, 1.5 and 1.8 rerun.

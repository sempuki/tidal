# Roadmap, round 8: what to do after phase 10, from a language designer's view

Date: 2026-11-05. Decision 0029, with decisions 0037, 0039, 0056 and 0057. Role: clean-room reader, language-designer persona. Inputs: AGENTS.md and CLAUDE.md; `reports/clean-room/roadmap-7/designer.md` (the model for this form) and `reports/clean-room/roadmap-7/portfolio.md`; `documentation/` (charter, roadmap, status, exercises, specification format, decisions 0056 and 0057, and `decisions/pending.md`); `reports/clean-room/X22/` in full, with `X22/X19-refresh.md`; the headers and findings of `R10/`, `W18/`, `W19/`, `I3/`, `PU3/` and `rust-frontend/report-22.md`; the three 2026-11-05 gap reviews in `reports/review/` (result sections and the OPR-9, NEW-25 and ALN-P1 sections); the question files Q-0007, Q-0175, Q-0183, Q-0300, Q-0383 and Q-0411; targeted reads of TR-P1, TR-4, OPR-9, SINIT-4 to SINIT-6, SW-P1 and DEFARG-P2 under `specification/`; `reports/clean-room/rust-frontend/conformance.tsv` for the program rows; `tools/propose_check.py` and its output, with and without `--corpus`; and `git log f0f53319..1e20bfa2` with every commit body. Commit read: `1e20bfa2`. Counts come from read-only Python scripts in my scratch directory. I ran them on this tree and on copies of `017b401` and `dad8bf8` made with `git archive`. I did not read the other round-8 reports. I did not run `tools/similarity.py`. Statements from outside Tidal's files are marked "general knowledge".

## 1. The state from a designer's view

### 1.1 Counts at 1e20bfa2

| Measure | Round 7 (017b401) | Now (1e20bfa2) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1697 | 1697 |
| `required` | 898 | 898 |
| `reject` | 558 | 558 |
| `undefined` | 57 | 57 |
| `impl-defined` | 52 | 52 |
| `recommended` | 43 | 43 |
| `no-diagnostic` | 37 | 37 |
| `unspecified` | 32 | 32 |
| `conditionally-supported` | 5 | 5 |
| `withdrawn` | 15 | 15 |
| Rules and principles whose text changed since round 7 (my diff, `:tests:` left out) | | 49 (39 rules, 10 principles) |
| Test files with a `tidal-expect` line | 6619 | 6681 |
| Of these, `reject` | 2726 | 2740 |
| Of these, `link-fail` | 44 | 44 |
| Files under `tests/` that name `-m32` | 85 | 85 |
| Program corpus (`propose_check.py --corpus`) | 85 programs (71 Write, 14 sketches), 5 multi-unit, 103 files, 10361 lines | 97 programs (79 Write, 18 sketches), 7 multi-unit, 118 files, 11438 lines |
| Questions | 400 | 411 |
| Provisional | 172 | 171 |
| Settled | 215 | 227 |
| Open questions whose `**Waits for:**` field names a use of a principle | 9 (by my reading of the notes) | 3 (Q-0263, Q-0299, Q-0395) |
| Open questions held by a `pending.md` row | 53 cited the file | 98 (decision packet) |
| Propose reports | 21 | 22 |
| Change-table rows that `propose_check.py` reads | 1003 | 1039 |
| Change-table rows whose rule text changed in phase 10 | | 26, in 14 reports |
| Front end passes (status page) | 6362 of 6619 | 6459 of 6681 |
| Program files the front end passes (`conformance.tsv`) | 85 of 85 | 97 of 97 (80 `pass`, 11 `pass-partial`, 6 `EDG-ONLY`) |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 37 | 39 |

Phase 10 changed no rule's class and added no rule ID. It changed the text of 39 rules and 10 principles, mostly through RV3 and the repairs of I3, W18, W19 and X22. The same 85 files name `-m32` at both commits. My own `tidal-flags` scan finds 76 tests at both commits, so the other 9 files name `-m32` in a note or a second unit.

### 1.2 What phase 10 did for the designer

Phase 10 took part of two of round 7's eight items. Decision 0057 deferred five by name.

| Round-7 item | Phase 10 | State |
|---|---|---|
| 1. A refreshed portfolio, then X22 to X24 | item E, commits 3019417d and 1b05738b | portfolio and X22 done. X23 and X24 deferred by decision 0057 |
| 2. The later-version probe (`pending.md` row 7) | none | waits on the maintainer. The plan still names about 2640 `reject` tests, and there are 2740 |
| 3. A shared impact scanner | none | deferred by decision 0057. X22 kept its own scanner, `reports/clean-room/X22/scan_p23.py`, so two scanners are kept and none is shared |
| 4. A front-end prototype of a done proposal | none | deferred by decision 0057 |
| 5. Refresh notes for class conflicts, and the refresh check for X1 to X12 | item E, the X19 note only | X4, X7 and X9 have no note. `tools/propose_check.py` has not changed since d9c18bc |
| 6. An index of what a test of validity can see | none | deferred by decision 0057 |
| 7. Pair exercises | none | deferred by decision 0057 |
| 8. ILP32 runs for tests of conversion rules | none | deferred by decision 0057 |

Of round 7's six process changes:
- Taken: `exercises.md` section 4 names round 7 and lists X22, and its table calls P17 "explicit aliases". Section 6 orders the items of decision 0057. Section 8 has PU2 and PU3 rows, and its X14 and X20 rule-change counts (56 and 58) now match the tool. This report keeps the view.
- Taken in part: the lane ran one new exercise and one refresh note, as process change 1 asked. The stop rule (process change 2) counts X22 to X24, and only X22 ran. Section 1.3 gives the interim count.
- Not taken: section 9 has no Propose prompt. Section 8 gives no closed counts. Section 6 still opens with "the Propose rounds X1 to X21", and section 8 lists X22 above X21.
- Not tested: process change 5 asks that a commit that changes a rule's class name the Propose reports that cite the rule. Phase 10 changed no class. Text changes did reach done designs with no note (section 1.5).

### 1.3 What X22 found, and what the repairs closed

| Exercise | Proposal | Findings | Closed in the specification | To a question | Closed elsewhere | No action |
|---|---|---|---|---|---|---|
| X22 | P23a, template bodies checked against their constraints (form 1, template-ids) | 9 | 6 (X22-1 to X22-5, X22-7) | X22-3 also opens Q-0411 (EDG treats the two folds as one atom) | X22-9, in `exercises.md` | X22-6, X22-8 |

Commit 1b05738b acted on every finding that a writer can act on, with six new tests. X13 to X16 closed 31 of 39, X17 to X20 closed 36 of 44, X21 closed 9 of 11, and X22 closed 6 of 9 in the text. A designer reading of a page area still finds about nine gaps, on a first or a second reading.

X22 found no wrong text on a page that Predict had passed. X17 to X21 together found seven such errors in five exercises. X22's findings on the specification are four "unclear", two "missing" and one "hard to find". Five of the seven concern `templates/constraints.md` and its neighbors (CNS-14, CPT-19, DEP-P6, CNS-P4). That page had 15 commits since its one earlier reading (X8). A second reading of a much-changed page found as many gaps as a first reading usually does.

Two X22 findings have no record outside the report and a commit message:
- X22-6 asks which effects a standard attribute may have, and whether one may add a required rejection. Commit 1b05738b calls it "a design question for the maintainer". No question file and no `pending.md` row holds it. The answer bears on three designs that chose differently. X12 and X18 chose a contextual keyword, and X18 ruled out an attribute for a feature that only adds a rejection. X22 chose an attribute for a feature of the same kind. P24's marker, still to be designed, faces the same choice.
- The X19 refresh note names a gap. `literal-builtin-suffix-wins.cpp` expects a run result from a program that OPR-9 makes not valid, with no diagnostic required. Commit 1b05738b says the gap "goes with Q-0383 to the next review". Q-0383 has no note that says so, and the test is unchanged since 076aa546.

The needs outside the specification, with X22 added:

| Need | Raised by X22 | Raised before |
|---|---|---|
| Version marking | X22-8 (the name `checked`, and checks at the definition) | X13 to X21 |
| Implementation cost (Q-0183) | X22-8 (subsumption normal forms can grow fast, general knowledge) | X14, X15, X17 to X21 |
| Mixed implementations | none | X21-9 |
| Target dependence | none | X17-11 |
| What a standard attribute may do | X22-6 | X12, X14 and X18 each reasoned from ATTR-P2 |

### 1.4 Pages the Propose exercises have reached

All 85 pages are reached, by `:::{rule}` IDs cited in X1 to X22. X22 gave `templates/constraints.md` its second reading. Five pages are reached by one exercise only:

| Page | Exercise | Commits to the page since that exercise's report was added |
|---|---|---|
| `declarations/alignment.md` | X14 | 12 |
| `declarations/namespaces.md` | X15 | 7 |
| `expressions/comma.md` | X1 | 5 |
| `statements/loops.md` | X11 | 5 |
| `expressions/bitwise.md` | X17 | 1 |

`declarations/alignment.md` is now the page most changed since its one reading. P25 aims at it, and P26 aims at `expressions/comma.md`.

### 1.5 The refresh check at 1e20bfa2

`tools/propose_check.py` reports 22 reports, 31 conflicts (10 withdrawn, 4 class, 17 new-id, 0 unknown), 125 rows changed since their report's commit, and 0 warnings. The changed rows are X13 13, X14 17, X15 19, X16 6, X17 24, X18 8, X19 12, X20 12, X21 10 and X22 4. Round 7 gave 13 for X19. My own diff gives 12 at both 017b401 and 1e20bfa2. I did not trace the difference.

The X19 class conflict on OPR-9 still prints. The refresh note answers it, but the tool does not read refresh notes. All four of X22's changed rows (DEP-P6, CNS-14, CPT-19, DED-24) changed in 1b05738b, X22's own repair commit. The output does not show that.

My diff of 017b401 against 1e20bfa2 finds 26 change-table rows in 14 reports whose text changed in phase 10. These rows reach a design:

| Report | Rows | Effect on the design |
|---|---|---|
| X21 (P15) | SINIT-4, SINIT-5, SINIT-6, TR-4 | X21 amends SINIT-5, which now leaves out `inline` variables and uses that a static initialization makes. X21:new-4 rests on "TR-4's policy for a break that one unit cannot see". TR-4 now makes a program with no `main` not valid with a diagnostic at link time. A refresh note should say whether X21's cycle rule can now be a required diagnostic |
| X8 (P16) | CLS-13 | CLS-13 now covers a deduced explicit object parameter. Member call syntax for free functions meets that form directly (general knowledge) |
| X12 (P19) | SW-P1 | SW-P1 now says a `switch` with a case for every enumerator can still complete. That supports X12's motivation and needs no change |
| X13 (P6) | DEFARG-P2 | A default added outside the class must not make a special member. Named arguments add no defaults, so I see no effect |
| X20 (P17) | ENUM-10 | The type of an enumerator inside its list. Q-0407 holds it in `pending.md` row 4 |
| X19 (P1) | OPR-9, OPR-P3 | The new reserved form (a space, then `_` and an uppercase letter) does not touch width suffixes. The refresh note covers the class |

The other rows add cases that the designs do not use: DED-24 in five reports, LAM-15 in two, and INST-12, SB-P2, ORD-13, REF-8, NAM-21 and SINIT-4 (X10) in one each.

The TR-4 change left the translation page in conflict with itself:
- After its class table, TR-P1 says Tidal has no class that requires a diagnostic when the units are linked, and that such a rule is `no-diagnostic` (TR-4).
- TR-4, a `required` rule, now says that a program with no `main` is not valid with a diagnostic, and every baseline compiler gives that diagnostic at link time.
- The page's TR-4 note still says that TR-P1 and TR-4 send both cases to LNK-7.

X21-4 put the TR-P1 sentence where a designer looks for the class policy. A designer who reads TR-P1 now gets the wrong answer for one case. TR-4 also states a rule on the program inside a `required` rule.

The two limits that round 6 named remain. X1 to X12 name no commit, so the tool lists no changed row for them. A change list does not say which commit changed a row.

### 1.6 What a designer still cannot state

**Whether a later version has taken the space.** X22-8 raises it for the name `checked`. So 13 of the 14 Propose reports from X9 on raise it. The phase 9 gap reviews add a second form of the same gap. The classes review says VIRT-1 follows a later resolution of a defect, and row 4 marks Q-0181 and Q-0087 "matrix (a later change)". A designer cannot tell which C++23 answers rest on later fixes. The plan in `pending.md` is written and costed, and Q-0007 is still provisional.

**What a change costs.** X22 names one subsumption test per dependent template-id in a checked definition, with no scale. Q-0183 keeps option 1.

**How much code a change touches.** Six readers built impact tools, and two tools are kept (X21, X22). X22 4.4 says the corpus was written to exercise rules, so it does not measure how often real code uses a form. The front end passes all 97 program files, so a scanner or prototype built on `tidal-fe` reads the whole corpus.

**Which programs change meaning without a diagnostic.** X22 4.3 again argued by hand. My scan finds 39 of the 558 `reject` rules cited by an `accept` or `run` test whose text contains `requires`.

**What a standard attribute may do.** This gap is new in this round (X22-6). Three done designs chose a marker on opposite readings of ATTR-P2 and ATTR-P4.

**What happens when units come from different implementations.** No change since X21.

**How two changes compose.** No pair exercise has run. The X19 note is the first written record of a done design meeting changed text. Section 1.5 shows four more designs that met changed text with no note.

**Validity on the other target.** No change. 85 files name `-m32`, as in round 7.

### 1.7 The portfolio

| # | Proposal | Round 7 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P23a | Template-ids checked against constraints | new | done (X22) | X22, repairs 1b05738b | |
| P23b | Expressions checked against requirements | new | partial | | No matching rule (X22-8). Q-0411 on the identity of fold atoms. Q-0263 on when an atom that is never `bool` is diagnosed. X22-6 if it takes a new marker |
| P22 | Labeled `break` and `continue` | new | ready | | The marker of the target. Version marking |
| P24 | Flag enumerations | new | ready | | The marker, which must not clash with X12's `final` and faces X22-6. ENUM-10, which P24 cites, sits in row 4 (Q-0407) |
| P25 | Aligned `new` and `delete` | new | ready | | Q-0300 (row 4). PU3-4 adds Q-0299: LSUP-P2 gives no kind or underlying type for `std::align_val_t` |
| P26 | No overloaded comma, checked left operand | new | ready | | The meaning of "has no value" for a class result |
| P8b | Relocation by bytes | partial | partial | | `pending.md` row 2 (Q-0322) |

Totals now: 22 done, 4 ready (P22, P24, P25, P26), 2 partial (P8b, P23b), 0 blocked.

### 1.8 Questions and principles

Phase 10 opened 11 questions and settled 12, 10 through RV3 and 2 through PU3. Three open questions wait for a use of a principle: Q-0263 (CNS-P2), Q-0299 (LSUP-P2) and Q-0395 (SB-P2). Two of them bear on the portfolio. Q-0263 is in P23's area, and Q-0299 is in P25's.

Phase 10 decided no `pending.md` row. RV3 sent eight more questions to row 4, which now holds 37. The rows hold 98 open questions. Row 7 (versions) holds 5. Row 9 (process questions, including Q-0183 on cost and Q-0116 on a source-reader designer) holds 8.

With the Waits for field, a designer can see what holds each question in the areas of the done designs. Q-0175 (SINIT-P2, X21's area) and Q-0383 (OPR-P3, X19's area) now wait for a review that reads them without doubt, and Q-0411 waits for a first review.

I3 built overload ranking from the text alone and agreed with 1513 of 1532 decided assertions. Six failures came from the text, and the I3 repairs acted on them. P2, P24 and P26 change overload candidates or ranking. A designer can run a copy of `reports/clean-room/I3/ranking.py` to model that ranking.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Repair TR-P1 against TR-4, and give X22-6 and the X19-refresh gap a record | 1.5: three statements on one page disagree; 1.3: two findings live only in commit messages | 1 short writer session | low | X21's refresh note; P24's marker; Q-0383's next review |
| 2 | Propose X23 on P25 and X24 on P26, then the stop-rule count | 1.4: alignment has 12 commits since one reading, comma 5; 1.3: about nine gaps per reading | 2 reader sessions, 1 repair session | low | second readings of two thin pages; the stop rule |
| 3 | The later-version probe: a maintainer decision on `pending.md` row 7 | 1.6: 13 of 14 reports from X9 on raise version marking | one decision; then one session, one matrix run of about 8200 compilations, triage | medium | the version line of every proposal; Q-0007, Q-0087, Q-0181, Q-0188 |
| 4 | Refresh notes where changed text reaches a design (X21 first), and a refresh check that reads them | 1.5: 26 rows changed in phase 10 with no note; X19's conflict still prints | 1 reader session and a small tool change | low | reuse of X21 and X8; pair exercises |
| 5 | A shared impact scanner, started from the X21 and X22 scanners | 1.6: six tools, two kept, none shared | 1 writer or front-end session | low | section 4 of every proposal |
| 6 | A front-end prototype of one done proposal | 1.6: the front end passes all 97 program files and 6459 tests | 1 front-end session | low | a measured check of a Propose report's impact section |
| 7 | An index of what a test of validity can see | 1.6: 39 of 558 `reject` rules have such a test | 2 writer sessions | medium | section 4.3 of every proposal |
| 8 | Pair exercises, starting with X17 and X19 | 1.6: no exercise states how two changes compose | 1 reader session per pair | low | an interaction section that can be checked |
| 9 | ILP32 runs for tests of conversion rules | 1.6: 85 files name `-m32`, as in round 7 | 1 writer session and a matrix run | low | the target line of a proposal that changes conversions |

### Item 1. Repair TR-P1 against TR-4, and record two lost findings

**What it is.** Three small acts.
- A writer makes TR-P1, TR-4 and the TR-4 note agree. One option is for TR-P1 to name the one case that needs a diagnostic at link time (a program with no `main`). The other is to move that case out of TR-4 into a rule whose class states it. The writer decides which, from the matrix and the review that RV3 used. `main-no-definition.cpp` expects `link-fail` today and fits either option.
- X22-6 becomes a question, `Q-NEW-standard-attribute-effects`, with a provisional answer and a `**Waits for:**` field. If the maintainer must decide it, the question names a `pending.md` row. The question cites X12, X14, X18 and X22, which reasoned from ATTR-P2 and ATTR-P4.
- Q-0383 gains a dated note on `literal-builtin-suffix-wins.cpp`, as commit 1b05738b promised.

**Evidence.** Sections 1.3 and 1.5.

**Cost.** One short writer session. The TR fix may need one probe.

**Risk.** Low. The TR fix touches a principle that every proposal reads for the class policy, so the gap review of phase 10's text should read it (process change 3).

**What it unblocks.** Item 4's X21 note. P24's marker, and P23b's if it takes a new marker. The next review of Q-0383.

### Item 2. Propose X23 on P25 and X24 on P26, then the stop-rule count

**What it is.** Round 7 item 1, continued. X23 takes P25 on `declarations/alignment.md`, and X24 takes P26 on `expressions/comma.md`. Both change existing behavior, so both fill section 4.4 in full. P25 uses the answer that `pending.md` row 4 keeps for Q-0300 and the provisional answer of Q-0299, and says so. P26 meets the overloaded operators that I3 modeled. After X24, the designer view counts closed and wrong-text findings over X22 to X24, as round 7's stop rule says. P22 and P24 stay ready for a later round. P24 waits for item 1's attribute question.

**Evidence.** Sections 1.3, 1.4 and 1.7. X22 shows that a second reading of a changed page finds as many gaps as a first.

**Cost.** Two reader sessions, one repair session.

**Risk.** Low for similarity. P25 touches the library type `std::align_val_t`, and LSUP-P2 gives no shape for it (PU3-4). The exercise uses Tidal's provisional answer and records the gap.

**What it unblocks.** Second readings of the two most changed thin pages. The stop rule's first full count.

### Item 3. The later-version probe: a maintainer decision on `pending.md` row 7

**What it is.** Round 7 item 2, unchanged. The plan's count of `reject` tests should read 2740, about 8200 compilations on three compilers. Decision 0057 says that a tool writes the parts it can derive, and this count is one of them.

**Evidence.** Section 1.6. The probe would also show some `reject` tests that flip in a newest mode because of a defect fix. General knowledge: compilers apply many defect fixes in every mode, so the probe sees only some of them.

**Cost.** One decision. If approved, one session for the runner option, one matrix run, and triage.

**Risk.** Medium, in triage, as the plan says.

**What it unblocks.** The version line of every proposal, including X23 and X24.

### Item 4. Refresh notes where changed text reaches a design, and a refresh check that reads them

**What it is.** Two parts.
- A reader writes refresh notes in this order: X21 (SINIT-5 and TR-4, after item 1), X8 (CLS-13), then the three old class conflicts X4 (CTOR-17), X7 (CE-11) and X9 (STG-25). Each note says whether its report's section 4 changes, as the X19 note did, and goes beside the report.
- `tools/propose_check.py` gains three behaviors. It reads a refresh note beside a report and moves the conflicts and changed rows that the note covers to a "noted" count. It names the latest commit that changed each changed row, so that a row changed by the report's own repair commit shows as such. It uses the commit that added a report when the report names none (round 6 item 6).

**Evidence.** Section 1.5. Round 7's process change 5 named class changes only. Phase 10 changed no class and still reached X21's design.

**Cost.** One reader session for five notes. A small tool change, with each value from `git log`.

**Risk.** Low. The check stays a warning.

**What it unblocks.** Item 8, which reads X17 and X19. Reuse of X21 and X8 in later proposals.

### Item 5. A shared impact scanner, started from the X21 and X22 scanners

**What it is.** Round 7 item 3, with a second starting point. `reports/clean-room/X22/scan_p23.py` finds constrained templates and the template-ids inside other templates. One tool under `tools/` takes both scanners' queries and the queries that X17 to X20 needed. It honors `tidal-flags` and reports `tests/programs/` apart from the single-rule tests.

**Evidence.** Section 1.6.

**Cost.** One session for the frame and the two kept queries. Each later Propose report adds its query.

**Risk.** Low.

**What it unblocks.** Measured counts in section 4 of X23, X24 and later proposals.

### Item 6. A front-end prototype of one done proposal

**What it is.** Round 7 item 4, in the same form. The front end now passes every program file and the C++23 call forms, so its own failures add less noise to the comparison. P18 (required `override`) is the cheapest case. P2 has the most predicted change (74 tests, 3 program sites). For a proposal that changes ranking, a cheaper first step runs a copy of I3's `ranking.py` with the proposal's rule added, on the overloading grids.

**Evidence.** Section 1.6. No implementation of a proposal has checked a Propose report's impact count.

**Cost.** One front-end session.

**Risk.** Low. The prototype stays out of `main`'s front end.

**What it unblocks.** A measure of how far section 4 of a Propose report can be trusted.

### Item 7. An index of what a test of validity can see

This is round 6 item 2 and round 7 item 6, unchanged. 39 of 558 `reject` rules have an `accept` or `run` test that uses `requires`. It costs two writer sessions, at medium risk. It unblocks section 4.3 of every proposal.

### Item 8. Pair exercises, starting with X17 and X19

This is round 7 item 7, unchanged. X17 and X19 come first, and the X19 note they need now exists. A second pair is X12 with the P24 design from a later round, because both put a marker on a scoped enumeration. One reader session per pair, at low risk.

### Item 9. ILP32 runs for tests of conversion rules

This is round 7 item 8, unchanged. It adds `-m32` variants of the tests of CONV-2 to CONV-11, CONV-19, CONV-21 and LIST-2 whose result depends on the widths of TYP-1. One writer session and a matrix run, at low risk.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them. P25 needs the shape of one library type, which LSUP-P2 can state (Q-0299).
- **P23b in round 8.** It needs a matching rule that the design must supply, and it meets Q-0411, Q-0263 and item 1's attribute question. It waits until those questions settle or hold a provisional answer.
- **A model of object files or of mixed implementations.** Only X21 needs one.
- **Relocation by bytes (P8b)** until the maintainer decides `pending.md` row 2.
- **Rules or `:since:` fields for later versions.** Item 3 makes an index only.
- **A source-reader designer** (Q-0116). Twenty-two exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2 in full). Items 5 and 6 give counts and one implementation's effort.
- **Full re-runs of old Propose reports.** Item 4's notes cover the changes that reach a design.

## 4. Process changes

1. **A finding sent to the maintainer gets a record in the same commit.** A repair commit that leaves a finding for the maintainer files a question or a `pending.md` row for it. A commit that sends a gap to a review together with a question writes a dated note in that question. X22-6 and the X19-refresh gap show the need.
2. **A repair names the done designs that its text change reaches.** Round 7's process change 5 covered class changes. Extend it to any change of a rule or principle that a Propose change table marks "amended". Item 4's tool change lets the repair commit list them.
3. **The phase's gap review reads the principles that proposals cite for class policy.** The review of phase 10's text includes TR-P1 and TR-4 after item 1.
4. **A Propose prompt in `exercises.md` section 9.** Rounds 6 and 7 asked for it. Round 6's text still applies, with one addition: read the refresh notes beside a report before relying on the report.
5. **Keep the exercise records current.** Section 6 names X1 to X22. Section 8 lists X22 after X21 and gives closed, to-a-question and no-action counts.
6. **The stop rule stays.** It counts X22 to X24. X22 alone closed 6 of 9 findings in the text and found no wrong text.
7. **Keep the designer's view in round 9** in this form. Rerun the counts of sections 1.1, 1.4, 1.5 and 1.8, and the diff of changed rows against each done design.

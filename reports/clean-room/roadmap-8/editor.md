# Roadmap, round 8: what to do after phase 10, from an editor's view

Date: 2026-11-05. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 7 report. In `documentation/` it read the charter, roadmap, status, style guide, specification format, exercises, decisions 0056 and 0057, and `decisions/pending.md` with its decision packet. It also read `specification/`, the headers of `tests/`, and `questions/` with its index. The phase 10 reports it read were, in `reports/clean-room/`, R10, W18, W19, I3, PU3, X22 (with the X19 refresh note), `roadmap-7/portfolio.md`, and `rust-frontend/report-22.md` and `ledger.md`. In `reports/review/` it read the three 2026-11-05 gap reviews of the text phase 9 wrote. It read `git log f0f5331..1e20bfa2` with commit bodies (20 commits). It did not read the other round 8 reports. Read-only scripts made the counts from copies of the trees at f0f5331 (round 7) and 1e20bfa2, with one method for both. The method differs a little from the round 7 scripts, so a few round 7 figures below differ from the ones the round 7 report gave. Statements marked "general knowledge" do not come from Tidal's files. Commit read: `1e20bfa2`.

The page table uses the round 7 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 7 (f0f5331) | Now (1e20bfa2) |
|---|---|---|
| Live rules | 1682, plus 15 withdrawn | 1682, plus 15 withdrawn. No rule is new, and no rule changed class |
| Rule and principle text changed in the phase | 78 rules and 35 of 343 principles | 39 rules and 10 of 343 principles, leaving out `:tests:`, `:rules:` and `:exceptions:` lines. Templates hold 12 of the 49, classes 11, declarations 10. CTAD-16, ENUM-10, and TARG-27 changed only by a question link |
| Test files | 6619. 6534 cited by a rule, 85 program tests | 6681. 6584 cited by a rule, 97 program tests. 62 new, none removed, no expectation of an old test changed |
| Rules that rest on one test | 1 (CPY-28) | 0. CPY-28 gained `trivially-copyable-scalar-kinds.cpp` |
| Rules that rest only on tests with an override for a newest compiler | 28 | 28 |
| `check_specification.py` warnings | 29 | 28 (the 28 rules above). 0 problems |
| Tests with an override | 664 | 680 |
| Overrides by label | EDG 274, GCC 248, Clang 129, Clang 18 76, GCC 14 55, GCC 16 `-O2` 14, Clang 22 `-O2` 8, other 24 | EDG 281, GCC 256, Clang 130, Clang 18 76, GCC 14 57, GCC 16 `-O2` 14, Clang 22 `-O2` 8, other 24 |
| Tests with an override for exactly one newest family | 488 (Clang 104, GCC 186, EDG 198) | 502 (Clang 105, GCC 193, EDG 204) |
| Tests with an override for all three newest families | 1 | 1, the same test (`replace-base-old-pointer-constant.cpp`, Q-0307) |
| Tests with `tidal-error` patterns (this method) | 3037 | 3060. The status page gives 3064 by its own method |
| Questions | 400 | 411 |
| Open questions (all provisional) | 172 | 171 |
| Settled, compiler defect | 215, 13 | 227, 13 |
| Open questions held by `pending.md` (row 1's group and rows 2 to 9) | 67 (39%), counted from the notes | 98 (57%), counted from the `**Waits for:**` lines |
| Front end | 6362 of 6619 pass, 6414 pass their check | 6459 of 6681 pass, 6499 pass their check |
| Front end on page examples | 175 of 177 | 175 of 177 |
| Evidence lines, whole specification (this method) | 7590 of 43364 (17.5%) | 7769 of 43691 (17.8%) |
| `tests/RESULTS.md` | 6619 rows | 6681 rows |
| `question_index.py --check` | 0 problems | 0 problems |
| Pages whose only Predict batch showed test names | 1 | 0 (R10 on `foundations/objects`, 61 of 61) |
| `pending.md` | 9 rows. Row 1 one cell of 1399 characters | 9 rows, the same nine, none decided. Row 1 is 606 characters. A generated packet and a settlements table follow the rows |
| Front-end findings with no answer | G146 to G151 | G4, G13, G25, and G152, each with a ledger row in state `open` |

The pages grew by 327 lines, and Evidence grew by 179 of them. Phase 10 changed far less specification text than phase 9. Most of its work went into records, tools, and tests.

### What phase 10 did to the questions

Each count comes from each file's status at f0f5331 and 1e20bfa2, and from the `**Waits for:**` lines at R4 (264b504) and at 1e20bfa2.

- 11 new questions (Q-0401 to Q-0411). All are open. 8 of them went straight to `pending.md` row 4 (Q-0401, Q-0404 to Q-0410). Q-0402 waits for a review (doubt). Q-0403 and Q-0411 wait for a first review.
- 12 questions settled: 10 through RV3 (Q-0092, Q-0247, Q-0340, Q-0358, Q-0390, Q-0392, Q-0397 to Q-0400) and 2 through PU3 (Q-0370, Q-0376). No question reopened.
- Open questions went from 172 to 171. Phase 9 settled 61.
- 126 of the 171 open questions (74%) did not change in phase 10, apart from the `**Waits for:**` line that R4 added.

The `**Waits for:**` items, at R4 and now (a question can hold more than one item):

| Item | At R4 | Now |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 52 | 60 |
| newest families that agree (row 1's group) | 36 | 39 |
| a review (doubt) | 46 | 50 |
| a review (silent) | 14 | 15 |
| a review (against) | 5 | 9 |
| a review (part) | 1 | 1 |
| a use of a principle | 20 | 3 (Q-0263, Q-0299, Q-0395) |
| a first review | 6 | 2 (Q-0403, Q-0411) |
| a principle, a rule, child questions | 1 each | 1 each |

When each question goes in the first group that fits (maintainer, then row 1's group, then use, first review, against, silent, doubt, part), the 171 split this way:

| What holds the question | Questions |
|---|---|
| A row of `pending.md` (rows 2 to 9) | 60 |
| Two newest families differ (row 1's group) | 38 |
| A review that reads with doubt | 45 |
| A review that finds the reference silent | 14 |
| A review that reads against the answer, outside row 4 | 7 (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) |
| A use of a principle | 3 |
| A first review, a principle, child questions, part of a review | 1 each |

In phases 8 to 10, questions settled through two channels: a review with no doubt, and a principle use. Both channels now reach few of the open questions. 98 wait on the maintainer, and 67 more rest on a review that has already read them without full support. Of the 75 review-held questions, 48 did not change in phase 10. 29 of the 45 doubt-only ones were last read by a review dated 2026-11-03.

### Text phase 10 wrote that no review has read

The three 2026-11-05 gap reviews read the tree before RV3's repairs. They read the text of items G and T (ALN-8, BF-1, CMP-8, COND-16, INST-12, ORD-6, CLS-13, CONV-8, LAM-15, LSUP-2). Every later change is unread by a source reader:

- 34 rules and 10 principles changed in or after the RV3 repair commits (b1d89e7, 3aa9b4b, 4a193ea, 75e4fab, 1b05738):
  - from RV3 part A and W19: ACC-20, ALN-8, BF-1, CLS-26, CVF-7, DCMP-1, IF-25, LAM-15, OPR-9, OPR-P3, UNI-P4;
  - from RV3 parts B and C: CLS-2, CMP-8, CONV-P3, CTAD-16, DEFARG-P2, ENUM-10, LAM-6, LSUP-2, NAM-21, ORD-4, ORD-13, ORD-P1, SB-P2, SINIT-4, SINIT-5, SINIT-6, TARG-27, TR-4;
  - from I3 and W18: AUTO-19, CLS-9, CONV-9, ICS-28, ICS-45, ICS-P1, LAM-29, REF-8, RET-2, SW-P1;
  - from PU3: SB-P2 a second time;
  - from X22: CNS-14, CNS-P4, CPT-19, DED-24, DEP-P6.
- Q-0403 (W19-1) and Q-0411 (X22-3), which no review has read.
- 8 review-held questions whose rule or principle changed after the review: Q-0152, Q-0156, Q-0337 (ICS-28, ICS-45, ICS-P1), Q-0293 and Q-0396 (ORD-13, ORD-P1), Q-0383 (OPR-9, OPR-P3), Q-0386 and Q-0402 (LSUP-2).
- 62 new tests, among them the W18, W19, and `r7-` program tests.

These phase 10 findings ask for text, and no specification, question, test, or documentation file answers them:
- R10-1 to R10-7 and R10-9 (eight asks).
- I3-1 and I3-5. The commit says they need probes.
- PU3-3 and PU3-4. Q-0263 and Q-0299 hold them.
- X22-6. This is a design question that the X22 repair commit gives to the maintainer, and `pending.md` has no row for it.

Commit 7b9cd71 left two items from the round 7 programmer "for later" and says RV3 reads both pages. The first is a doubt on a placement `operator new` that is declared and not defined (LSUP-7 or ODR-8). The second is the LAM-18 note. No RV3 review names LSUP-7, ODR-8, or LAM-18.

### What phase 10 took from round 7

| Round 7 item | Outcome |
|---|---|
| 1. Review the text phase 9 wrote (RV3), then the repairs | Done. Three reviews read 117 rules and principles. 10 questions settled, 8 went to row 4. The repairs wrote 44 rule and principle changes that no review has read |
| 2. Repair the records, and generate the Split tests tables (R4) | Done. `tools/split_tests.py` writes the tables, the check fails on a stale table, the format has one definition, the grids moved, Q-0290 and the I1 and I2 headers are fixed. `exercises.md` section 6 is stale again (below) |
| 3. Hold each waiting question by a field, then PU3 | Done. All 172 questions got the line, and the check requires it. RV3's principle uses took 17 of the 20, so PU3 ran on 5 and settled 2 |
| 4. A decision packet for `pending.md` | In part. `question_index.py` writes the open questions of each row and the number of tests that name them. The packet gives neither the questions that each option settles nor the tests whose expectation each option changes. No row was decided |
| 5. Triage the review-held questions | In part. Item PK wrote the kind into each line (50 doubt, 15 silent, 9 against, 1 part). No second review ran, no rule of Tidal's own was proposed for the silent kind, and the 7 "against" questions outside row 4 stayed outside it |
| 6. Close the small open ends | Done. G147 to G151 answered, R10 ran on the objects page, CPY-28 has two tests. R10's nine findings have no answer |
| 7. Split the overloading principles | Not done. Decision 0057 defers it until I3 reports. I3 decided 1513 of 1532 assertions (98.8%) from the two pages. R/P is still 11.0 and 9.4 |
| Process: each phase names the review of its own text | Adopted in decision 0057. Phase 11 reads phase 10's text |
| Process: one reason field per open question, checked | Done |
| Process: tools select by the field | Done. `exercises.py principles` reads the line |
| Process: generated tables in pages | Done for Split tests, row 1's list, the packet, section 7's sentence, and the interpreter counts. `exercises.md` section 6, the status page's front-end paragraph, and the Divergences status tags are still written by hand, and each has drifted |
| Process: one definition per format part | Done for Split tests |
| Process: the closing commit gives the open-question change by group | Done in 1e20bfa2 |
| Process: a `pending.md` packet each phase | Done once. Decision 0057 does not make it a standing choice. The share of open questions in `pending.md` rose from 39% to 57% |

### Records that drift

These records disagree with the tree at 1e20bfa2, or with each other:

- **Divergences status tags.** 14 Divergences lines on 13 pages call a settled question "open":
  - Q-0340 (unions), Q-0390 (attributes), Q-0370 (default arguments), Q-0376 (structured bindings);
  - Q-0092 (lambdas), Q-0078 (constant evaluation), Q-0358 (ODR), Q-0400 (concepts);
  - Q-0392 (constraints and instantiation), Q-0399 (deduction), Q-0247 (dependent names), Q-0397 and Q-0398 (ordering).

  Thirteen come from phase 10's settlements. Q-0078 was already wrong at f0f5331. `check_specification.py` checks that each named question has a line, and no tool checks the line's tag against the question file.
- **Waits for: a use, after the use.** Q-0263 and Q-0299 wait for "a use of CNS-P2" and "a use of LSUP-P2". PU3 and RV3 both used these principles. Each use failed because the principle lacks a statement (PU3-3, PU3-4). Neither principle changed after PU3. `exercises.py principles` would select both questions again for a PU4 batch, on the same text. The README form `a principle (ID)` fits them better.
- **Settlements table.** In `pending.md`, the rows for RV3 (10 questions) and PU3 (2) have an empty Commit cell. They settled in b1d89e7 and 75e4fab.
- **Status page, front end.** The paragraph under "Known limits of the tooling" says that tests added after front-end round 21 have no row in `conformance.tsv`, and that round 22 takes them. Round 22 is done, and `conformance.tsv` has a row for each of the 6681 tests. The paragraph is written by hand.
- **`exercises.md` section 6** says that W1 to W17, X1 to X21, I1 and I2, and PU1 and PU2 are done. It lists PU3, I3, W18, W19, and X22 as the next exercises, and all five are done. Round 7 found the same drift one phase earlier.
- **A test against a reclassified rule.** `classes/operators/literal-builtin-suffix-wins.cpp` cites OPR-8. It expects `run` from a program that declares literal operators for suffixes that the language uses. OPR-9 now makes such a declaration not valid with no diagnostic required. The X19 refresh note names the gap, and the X22 repair commit sends it "with Q-0383 to the next review". The test's header does not name Q-0383.
- **Findings promised to an item.** The R10 commit says that the writer acts on R10-1 to R10-9 in RV3's repair pass. Neither RV3 repair commit names an R10 finding. The ledger of decision 0057 covers only front-end findings (G1 to G152), so no record shows these nine as open.
- **Q-0299's Opinions** are out of date order: the 2026-11-04 review, then 2026-11-05, then 2026-11-03, with the template comment between them. The PU3 note sits under "Provisional answer".
- **Stacked reasons.** 12 open questions still carry three or more "Kept open" lines (Q-0018 and Q-0378 seven, Q-0386 six). The `**Waits for:**` line now gives the current reason, so this drift costs reading time only.
- **The packet's promise.** Decision 0057, item PK, asks for "the tests whose expectation would change" for each option. The generated packet gives the number of tests whose notes name a row's questions.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | Other evidence |
|---|---|---|---|---|---|---|---|---|
| 1 | overloading/conversion-sequences | 47 / 133 | 13 | 225 | 27 | 178 / 1033 | 9.4 | No question settled in phases 9 and 10. 8 of the 13 wait for a review (doubt). ICS-28, ICS-45, and ICS-P1 changed after RV3, and Q-0152, Q-0156, and Q-0337 rest on them. ICS-35 and ICS-44 rest only on overridden tests |
| 2 | foundations/storage | 29 / 94 | 12 | 262 | 6 | 164 / 1018 | 3.6 | No rule changed and no question settled in phases 9 and 10. The 12 have six different `**Waits for:**` lines. Q-0307 is still the one test against all three families |
| 3 | overloading/overload-resolution | 44 / 185 | 10 | 333 | 30 | 205 / 1053 | 11.0 | Q-0405 is new and went to row 4. The highest R/P for a fifth round. I3 decided 98.8% of its assertions from the page |
| 4 | classes/members | 34 / 172 | 8 | 184 | 16 | 151 / 859 | 8.5 | Four rules changed (CLS-2, CLS-9, CLS-13, CLS-26), and three of them after RV3. Q-0406 is new in row 4. 3 questions wait for a review that finds the reference silent |
| 5 | foundations/constant-evaluation | 27 / 155 | 7 | 436 | 24 | 187 / 828 | 5.4 | The most report mentions. 5 of 7 wait for a review (doubt). Its Divergences line calls the settled Q-0078 open. CE-13 and CE-28 rest only on overridden tests |
| 6 | templates/class-template-deduction | 29 / 119 | 7 | 73 | 22 | 209 / 894 | 9.7 | 6 of its 7 open questions wait on the maintainer (rows 3 and 4). Q-0408 is new in row 4. The lowest F on the list |
| 7 | templates/ordering | 17 / 64 | 6 | 183 | 19 | 200 / 821 | 4.2 | ORD-4, ORD-13, and ORD-P1 changed after RV3. Q-0405 is new in row 4. Divergences lines call the settled Q-0397 and Q-0398 open |
| 8 | foundations/language-support | 13 / 68 | 6 | 160 | 17 | 164 / 693 | 4.3 | New on the list. LSUP-2 changed twice in phase 10, the second time after RV3. Q-0402 is new. Q-0299's line waits for a use that has already failed (PU3-4) |
| 9 | templates/constraints | 40 / 140 | 5 | 151 | 30 | 244 / 1052 | 8.0 | 30 overridden tests, tied with overload resolution for the most. CNS-14 and CNS-P4 changed in X22, unread. Q-0411 has no review. Q-0263's line waits for a use that has already failed (PU3-3). The Divergences line calls Q-0392 open |
| 10 | expressions/new-delete | 42 / 144 | 6 | 177 | 23 | 212 / 969 | 6.0 | New on the list. No rule changed and no question settled in phase 10. 3 wait for a review (doubt), 3 for the maintainer or row 1's group |

Two pages left the round 7 list. Instantiation went from 6 open questions to 5 (Q-0392 settled), and dependent names from 6 to 5 (Q-0247 settled).

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | **Review the text phase 10 wrote** (RV4), then the repairs | 34 rules and 10 principles changed after RV3 read the tree. Q-0403 and Q-0411 have no review. 8 review-held questions rest on changed text. Decision 0057 makes this review a standing choice | one source-reader session in two parts, one writer session | low | Q-0403, Q-0411, and the 8 questions. A use for Q-0395. Pages 1, 4, 7, 8, 9 |
| 2 | **A second packet for `pending.md`, by option** | 98 open questions (57%) wait on the maintainer. Row 4 has 37 questions, and in 17 of them every newest family keeps the matrix's answer | under one session, plus a tool change | low | Up to 98 settlements. Row 1 and the proposed row 4a alone hold 55 distinct questions |
| 3 | **A narrow second review of the doubt-held questions** | 45 questions are held only by a review's doubt. 29 were last read on 2026-11-03 by the cluster reviews | one source-reader session in two or three parts, one writer session | medium | For each of the 45: a settlement, a named missing step, or a move to a row |
| 4 | **One ledger for every exercise finding, then the open ones** | R10-1 to R10-9 have no answer after a commit promised one. I3-1, I3-5, PU3-3, PU3-4, X22-6, the LSUP-7 and LAM-18 notes, and G4, G13, G25, G152 are open | one tool session, one writer session | low | Pages 5, 8, 9. Q-0263 and Q-0299 get text to use |
| 5 | **Repair the records, and generate the parts that drifted** (R5) | Section 1, "Records that drift" | under one session, plus small tool changes | low | Divergences lines that match the question files. A status page and `exercises.md` that a check keeps true |
| 6 | **A principle-use batch PU4**, after RV4's repairs | Q-0395 waits for a use of SB-P2. RV4's repairs will add more | one reader batch, one writer pass | low | Q-0395 and the questions RV4 holds for a use |

### 2.1 Review the text phase 10 wrote

The review follows RV3's form, with a smaller part A:

- Part A reads the 34 rules and 10 principles in section 1, grouped by chapter. The largest groups are classes (ACC-20, BF-1, CLS-2, CLS-9, CLS-26, CVF-7, DCMP-1, OPR-9, OPR-P3, UNI-P4) and templates (CNS-14, CNS-P4, CPT-19, CTAD-16, DED-24, DEP-P6, ORD-4, ORD-13, ORD-P1, TARG-27). The three rules whose only change is a question link need no reading.
- Part B applies SB-P2, DEFARG-P2, ICS-P1, ORD-P1, and CONV-P3 to the programs of the open questions they hold. A use here meets decision 0055's condition, so Q-0395 can settle in RV4 without a PU4 batch.
- Part C reads Q-0403 and Q-0411 for the first time. It rereads Q-0152, Q-0156, Q-0293, Q-0337, Q-0383, Q-0386, Q-0396, and Q-0402 against the changed text. Part C also takes `literal-builtin-suffix-wins.cpp` with Q-0383.

Before the writer reads the review, `similarity.py` checks it (AGENTS.md rule 6). The writer gives each question a new `**Waits for:**` line or settles it.

### 2.2 A second packet for `pending.md`, by option

Phase 10's packet lists the questions of each row. A maintainer who reads it still has to work out what each answer would change. The editor proposes three changes to the packet, all generated by `question_index.py`:

| Row | Change | Questions |
|---|---|---|
| 1 | For the option "widen the rule to two differing families", list each question and the tests that name it. No expectation changes, because the answer stays | 39 |
| 4 | Split into 4a, where every newest family keeps the matrix's answer, and 4b, where one family follows the review. 4a asks one plain question: does Tidal follow every implementation against a review? | 4a: 17 (Q-0140, Q-0142, Q-0181, Q-0225, Q-0243, Q-0246, Q-0275, Q-0277, Q-0280, Q-0300, Q-0336, Q-0394, Q-0405, Q-0406, Q-0407, Q-0409, Q-0410). 4b: 20 |
| 10 (new) | A review finds the reference silent, so the answer is Tidal's own choice. The option is to record it as Tidal's choice in Evidence and settle | 15 |

For each row and option, the packet also gives the tests whose expectation would change. The tool can find these from the test notes that name each question. The seven "against" questions outside row 4 (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) join row 4 where the matrix keeps the answer, as item PK said they could. X22-6 becomes a question or a process row.

Row 1 and row 4a together hold 55 distinct open questions (Q-0142 is in both). Two answers from the maintainer would close about a third of the open count. Asking the maintainer is inside the repository's process, so the packet needs no wait before it is written.

### 2.3 A narrow second review of the doubt-held questions

Decision 0054 settles a question by review only when the review supports the answer with no doubt. 45 open questions are held by doubt alone. 29 of them were last read in the cluster reviews of 2026-11-03, which read many questions per section. A repeat of the same prompt would likely give the same reading.

The editor proposes a review with a narrower prompt:
- one question per section;
- the question's tests listed;
- a verdict from a fixed scale: supports; supports, with the missing step named; reads against; silent.

Each named step goes into the question's line. A step that a rule can state becomes a repair. A step that the reference does not decide moves the question to row 10 (item 2.2).

The risk is that the doubts persist. Then the gain is a named reason in each line, and row 10 grows.

### 2.4 One ledger for every exercise finding

Decision 0057 gave each front-end finding a ledger row, and the check fails on a finding with no row. The other exercises have no such record. R10's nine findings show the cost: a commit said they would go to RV3's repair pass, and nothing records that they did not.

The tool change has three parts:
- `check_specification.py` reads the finding IDs of every `reports/clean-room/*/report.md` (R, W, I, PU, X, D, and G), and fails when one has no row in a ledger.
- A finding can be in state `planned`, with the item that takes it.
- The close commit of a phase lists the `planned` rows whose item is done.

Then the writer answers the open findings:

- R10-1 to R10-7 and R10-9: notes and cross references on the objects page, CE-22, LSUP-11, OBJ-11, and the `auto` table. R10-3 (OBJ-1 against UNW-2) and R10-5 (what counts as a use of a `thread_local` object) may need a probe.
- I3-1 and I3-5: the probes the I3 repair commit names.
- PU3-3 and PU3-4: a statement in CNS-P2 on when an atom whose type is never `bool` is diagnosed, and a kind and underlying type for `std::align_val_t` in LSUP-P2. These give Q-0263 and Q-0299 a text to use.
- X22-6: a row or a question.
- The LSUP-7 or ODR-8 doubt and the LAM-18 note from 7b9cd71.
- G4, G13, G25, G152: a probe or a question each.

### 2.5 Repair the records, and generate the parts that drifted

- `check_specification.py` fails when a Divergences tag disagrees with the question's status, and `status_table.py` writes the tag. The 14 lines in section 1 change to "settled".
- `question_index.py` fills the Commit cell of the settlements table from `git log` for the questions a row names, or the table drops the column.
- `status_table.py` writes the front-end paragraph of the status page from `conformance.tsv` and `frontend_check.py`, or the paragraph loses the sentence that names a round.
- `exercises.md` section 6 points at the roadmap's current phase for the order, and `status_table.py` writes its list of done exercises from section 8. Then the next phase cannot leave the section stale.
- Q-0263 and Q-0299 change their line to `a principle (CNS-P2)` and `a principle (LSUP-P2)` until item 2.4 repairs the principles.
- `literal-builtin-suffix-wins.cpp` names Q-0383 in its note, and the OPR-8 Evidence says why the test keeps `run` until RV4 reads it.
- Q-0299's Opinions go in date order, and its PU3 note moves under Opinions.

### 2.6 A principle-use batch PU4

PU4 runs after RV4's repairs, on the questions whose line names a use. If RV4's part B uses SB-P2 on Q-0395 and supports it, PU4 may have nothing to take. In that case PU4 is skipped, and decision 0058 should say so. The batch is small either way.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phase 10 added no rule and no page |
| Splitting the overloading principles | Drop as an item | I3 built the ranking from the two pages and decided 98.8% of the assertions it could reach. Its failures came from missing steps, which item 2.4 and RV4 can add. Under decision 0055, a split would restart the wait of the 22 open questions on the two pages |
| The 28 rules that rest only on overridden tests | Leave as warnings | The count has not changed for three phases, and item TE probed each one in phase 8 |
| Settling questions where two newest families differ | Leave to the maintainer (row 1) | Item 2.2 states the option |
| New Propose rounds | One at most, from P22, P24, P25, or P26 | The refreshed portfolio has four ready proposals. A Propose round adds text that the next review must read |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md` |
| Port suites, guides on the site, Diagnose beyond D-classes, pair exercises, a shared impact scanner, ILP32 runs, labels for `unspecified` rules | Defer | Items 1 to 5 reach more open questions per session |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0057 say | 33 *(referenced)* markers name them, as in round 7. I3-9 and I3's note N12 ask for extended floating-point ranks and stay out |

Decision 0057 asks each phase to run an exercise that writes programs. A Write suite fits beside items 1 to 5. The editor has no view on its topic.

## 4. Process changes

- **Every exercise finding has a ledger row.** The front-end ledger of decision 0057 extends to every report in `reports/clean-room/`, with a `planned` state that names the item. A commit that says "the writer acts on these in item X" adds `planned` rows. The close commit lists the rows whose item ended without an answer.
- **A tag that restates a question's status comes from a tool.** People wrote the Divergences tags, the settlements table, and the front-end paragraph of the status page, and all three drifted in this phase. Decision 0057's choice "tools write the parts they can derive" names four parts. The editor proposes that it also name these three.
- **A use that fails without a repair changes the line.** When a principle use does not give the answer and the principle does not change in the same commit, the question's line names the principle as `a principle (ID)`. `question_index.py --check` fails when a question's last note records a failed use and its line still names a use of the same principle.
- **A `pending.md` packet by option each phase.** The close commit gives the share of open questions that `pending.md` holds. Phase 10's close gives the counts by row, and the share went from 39% to 57%. When the share rises, the next phase's first item is a packet that the maintainer can answer row by row.
- **The review of the previous phase's text stays first.** Phase 10 wrote 44 rule and principle changes after its own review read the tree. RV4 should run early in phase 11, so that RV5 in phase 12 reads RV4's repairs.
- **A phase aim by hold.** Decision 0056's aim was to end each phase with fewer open questions than at its start. Phase 10 met it by one question. The editor proposes an aim for each hold instead. For example: every doubt-only question is read by a review dated in the phase, and every row of the packet is put to the maintainer.

## 5. Files

The reader made the counts with read-only scripts in `/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/r8-editor/`. The scripts ran on copies made with `git archive`: the tree at f0f5331 (`t7/`, and `t7r/` for its reports), the tree at 1e20bfa2 (`t8/`), and `questions/` at 264b504 (`tR4/`).

- `count.py`: rules, principles, tests, overrides, questions, Evidence (output in `c_t7.json` and `c_t8.json`).
- `diff.py` and `smalldiff.py`: the rule, principle, and test changes of the phase, and the size of each text change (`diff.json`).
- `when.py`: the commit that changed each rule or principle (`when.json`).
- `q.py`: question movement, the `**Waits for:**` items at R4 and now, and the exclusive groups.
- `pages.py`: the page table (`pages8.json`).
- `divtags.py`: Divergences tags against question status.

Short inline scripts also counted row 4's items by family, the review dates of the review-held questions, and the open questions that rest on text written after RV3. The reader also ran `tools/check_specification.py` (1697 rules, 0 problems, 28 warnings) and `tools/question_index.py --check` (411 questions, 0 problems), which change no file.

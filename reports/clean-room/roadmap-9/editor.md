# Roadmap, round 9: what to do after phase 11, from an editor's view

Date: 2026-11-05. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 8 report with the scripts beside it. In `documentation/` it read decision 0058, `decisions/pending.md` with its packet, the status page, the roadmap's phase 11 part, and sections 4, 6, 7, and 8 of `exercises.md`. It read `git log 1e20bfa2..a15a877d` with every commit body (15 commits) and the `--stat` of each. In `reports/clean-room/` it read `ledger.md`, `rust-frontend/ledger.md`, and the finding headings of the I4, X23, and P1 reports. In `reports/review/` it read the two 2026-11-05 gap reviews of the text phase 10 wrote, and in `reports/similarity/` the summary line of the eight phase 11 reports. It read the question files that the counts below name, and the parts of `specification/` and the test headers that its scripts read. It did not read the other round 9 reports, `inputs/`, or `build/exercise-keys/`. It did not run `tools/run_tests.py` or `tools/frontend_check.py`, so the front-end counts come from the close commit. Read-only scripts made the counts from copies of the trees at 1e20bfa2 (round 8) and a15a877d, made with `git archive`, with one method for both. Statements marked "general knowledge" do not come from Tidal's files. Commit read: `a15a877d`.

The page table uses the round 8 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 8 (1e20bfa2) | Now (a15a877d) |
|---|---|---|
| Live rules | 1682, plus 15 withdrawn | 1682, plus 15 withdrawn. No rule is new, and no rule changed class |
| Principles | 343 | 343 |
| Rule and principle text changed in the phase | 39 rules and 10 principles | 17 rules and 9 principles, leaving out `:tests:`, `:rules:`, and `:exceptions:` lines. Foundations hold 8 of the 26, templates 6, expressions 5 |
| Test files | 6681. 6584 cited by a rule, 97 program tests | 6711. 6608 cited by a rule, 103 program tests. 30 new, none removed, no expectation of an old test changed |
| Rules that rest on one test | 0 | 0 |
| Rules that rest only on tests with an override for a newest compiler | 28 | 28, the same 28 |
| `check_specification.py` | 0 problems, 28 warnings | 1697 rules, 0 problems, 28 warnings (the 28 rules above) |
| Tests with an override | 680 | 688 |
| Overrides by label | EDG 281, GCC 256, Clang 130, Clang 18 76, GCC 14 57, other 80 | EDG 285, GCC 258, Clang 132, Clang 18 76, GCC 14 57, other 80 |
| Tests with an override for exactly one newest family | 502 (Clang 105, GCC 193, EDG 204) | 510 (Clang 107, GCC 195, EDG 208) |
| Tests with an override for all three newest families | 1 (`replace-base-old-pointer-constant.cpp`, Q-0307) | 1, the same test |
| Tests with `tidal-error` patterns (this method) | 3060 | 3066. The status page gives 3070 by its own method |
| Questions | 411 | 418 |
| Open questions (all provisional) | 171 | 174 |
| Settled, compiler defect | 227, 13 | 231, 13 |
| Open questions held by `pending.md` rows 1 to 9 | 98 (57%) | 111 (64%). With row 10, which phase 11 added, 124 (71%) |
| `pending.md` rows decided by the maintainer | none of 9 | none of 10 |
| Front end (close commit) | 6459 of 6681 pass, 6499 pass their check | 6478 of 6711 pass, 6520 pass their check. 11 of the 30 new tests do not pass |
| Front end on page examples (`examples.tsv`) | 175 of 177 | 175 of 177 |
| Evidence lines, whole specification | 7769 of 43691 (17.8%) | 7805 of 43780 (17.8%) |
| `tests/RESULTS.md` | 6681 rows | 6711 rows |
| `question_index.py --check` | 0 problems | 418 questions, 0 problems, 5 warnings (below) |
| Divergences tags that disagree with the question file | 14 lines on 13 pages | 0 |
| Exercise findings with a ledger row | front-end findings only (152) | 867 exercise findings and 152 front-end findings |

The pages grew by 89 lines, and Evidence grew by 36 of them. Phase 11 changed less specification text than phase 10, and most of its work went into records, tools, and tests.

### The read-only checks

- `tools/check_specification.py`: "1697 rules, 0 problems", exit 0, with the 28 warnings above.
- `tools/question_index.py --check`: "418 questions, 0 problems", exit 0. It gives 5 warnings from the family check of item R5. Q-0025 has a test with two newest families that differ, and its line does not name row 1's group. Q-0142, Q-0274, Q-0323, and Q-0341 name row 1's group, and no test that names them has two newest families that differ.
- `tools/checks.py --no-frontend`: similarity did not run (no reference corpus given); the other two checks give the lines above; exit 0. The script has no read-only mode beyond leaving out the front end and the corpus.
- `tools/divergence_tags.py` has no command line. `check_specification.py` calls it, and a direct call of `divergence_tags.current()` on every page finds 0 pages whose tags disagree with the question files.
- The eight phase 11 similarity reports (`2026-11-05-phase-11-f`, `-r5`, `-p`, `-p1`, `-rv4`, `-rv4r`, `-e-repairs`, `-close`) each report "0 flagged".

### What phase 11 did to the questions

Each count comes from each file's status and `**Waits for:**` line at 1e20bfa2 and at a15a877d.

- 7 new questions (Q-0412 to Q-0418). Q-0413 settled in the same phase. Q-0412 went to row 9, and Q-0416 to row 4a. Q-0414 and Q-0415 wait for a review (doubt), and Q-0417 and Q-0418 for a first review.
- 4 settled, all through RV4 (Q-0263, Q-0386, Q-0395, Q-0413). Two of them came from RV4's principle uses (CNS-P2, SB-P2). No question reopened.
- Open questions went from 171 to 174. Phase 10 went from 172 to 171.
- 138 of the 168 questions that were open at both ends did not change at all in phase 11, `**Waits for:**` line included.

The `**Waits for:**` items, at the start and the end of the phase (a question can hold more than one item):

| Item | Start | End |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 60 | 62 |
| newest families that agree (row 1's group) | 39 | 56 |
| a review (doubt) | 50 | 48 |
| a review (silent) | 15 | 15 |
| a review (against) | 9 | 9 |
| a review (part) | 1 | 1 |
| a use of a principle | 3 | 0 |
| a first review | 2 | 2 (Q-0417, Q-0418) |
| a principle, a rule, child questions | 1 each | 1 each |

These match the close commit. When each question goes in the first group that fits (maintainer, then row 1's group, then first review, against, silent, doubt, part), the 174 split this way:

| What holds the question | Questions |
|---|---|
| A row of `pending.md` (rows 2 to 9) | 62 |
| Two newest families differ (row 1's group) | 49 |
| A review that reads with doubt | 38 |
| A review that finds the reference silent (packet row 10) | 13 |
| A review that reads against the answer, outside row 4 | 7 (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364, as in round 8) |
| A first review | 2 |
| A principle (Q-0196), child questions (Q-0294), part of a review | 1 each |

Item PK2 moved 17 questions into row 1's group after the family check. The growth of row 1 explains most of the rise in the share that `pending.md` holds. 58 of the 73 review-held questions did not change in phase 11. By the latest date in each file, 27 of the 73 were last read on 2026-11-03, 11 on 2026-11-04, and 35 on 2026-11-05.

### Text phase 11 wrote that no review has read

RV4's two reviews read the tree after items F and P. They name every rule and principle that F and P changed (AUTO-P3, CNS-P2, DED-29, LSUP-11, LSUP-P2, OBJ-1, OBJ-11, TR-P1, ACC-10, CEV-1, PACK-13, PACK-P2). Two later commits changed 11 rules and 4 principles, and no source reader has read them:

- RV4's repairs (7cf4574b): CLS-26, CONV-9, ICS-45, NAM-21, OBJ-11, ORD-13, DEP-P6, OPR-P3.
- Item E's repairs from I4 and X23 (ee6085aa): ALN-4, COND-16, COND-P2, NEW-21, NEW-29, NEW-P5, STG-5.

Three questions rest on this text with no review that read it:

- Q-0417 (X23-2) and Q-0418 (I4-2) wait for a first review.
- Q-0415's line says "a review (doubt)". RV4's review read the old NAM-21 sentence on friend declarations as against the reference, and the writer then replaced the sentence. No review has read the new sentence or the question, and the review states no doubt about the reading the new sentence follows.

RV4's repair commit lists doubts it did not act on and says that "the next review reads them again": LAM-6's term "named capture", LAM-29, AUTO-P3 and RET-2 (the limits of JMP-10), SINIT-6, SW-P1's "needs", TR-P1 and TR-4, and DED-29. No ledger row or question records them, because review findings carry no IDs.

### What phase 11 took from round 8

| Round 8 item | Outcome |
|---|---|
| 1. Review the text phase 10 wrote (RV4), then the repairs | Done. Two reviews read 56 rules and principles and the records of 23 questions. 4 questions settled. The repairs changed 8 rules and principles, and item E's repairs 7 more, which no review has read |
| 2. A second packet for `pending.md`, by option | Done in part. The packet gives each option's questions, row 4 splits into 4a (18) and 4b (20), and row 10 holds the 15 silent ones. The "answers change" rows give the same test count as the "answers stay" rows, because the tool counts the tests whose notes name the questions. The seven "against" questions outside row 4 stayed outside. No row was decided |
| 3. A narrow second review of the doubt-held questions | Not done. Decision 0058 defers it. 38 questions are held by doubt alone |
| 4. One ledger for every exercise finding, then the open ones | Done. Item L wrote 837 rows and a check, and item F answered the R10, PU3, X22-6, G4, G13, G25, G152, LSUP-7, and LAM-18 asks. R10-4, R10-6, I3-1, I3-5, and I3-13 were deferred again |
| 5. Repair the records, and generate the parts that drifted (R5) | Done in large part. Divergences tags, settlements commits, and `exercises.md` section 6 now come from tools or point at a source. Q-0299's Opinions are still out of date order, and 10 questions still carry three or more "Kept open" lines |
| 6. A principle-use batch PU4 | Not needed. RV4's part B made the three uses, and no question waits for a use now |
| Process: every finding has a ledger row | Adopted (decision 0058) |
| Process: a tag that restates a status comes from a tool | Adopted for the Divergences tags, the settlements table, the front-end paragraph, and section 6 |
| Process: a failed use changes the line | Adopted as a choice. No check enforces it; `question_index.py` only accepts the form `a principle (ID)` |
| Process: a packet by option each phase | Adopted |
| Process: the review of the previous phase's text stays first | RV4 ran after F and P, as decision 0058 set. Its repairs and item E then wrote text after it |
| Process: a phase aim by hold | Not adopted |

### Decision 0058's standing choices in phase 11

**Every finding has a ledger row.** Held for the reports in `reports/clean-room/`. The ledger has 867 rows with no duplicates: 762 commit, 68 open, 26 planned, 5 text, 3 question, 3 out of scope. Every I4, X23, and P1 ID that any `.md` file in those directories names has a row. The close commit lists the 20 planned rows whose item ended without an answer, as the choice asks. Gaps:

- The 20 rows have no answer, and each still names an item that ended: I3-1, I3-5, I3-13, R10-4, and R10-6 name item RV4; P1-3, P1-5, P1-7, P1-8, P1-9, I4-8 to I4-14, X23-3, X23-6, and X23-7 name "item E repairs". The other 6 planned rows (P1-1, P1-2, I4-4 to I4-7) name item FE, which moved to phase 12.
- R10-4 and R10-6 were promised to RV3 in phase 10, then planned for F, then for RV4. I3-1, I3-5, and I3-13 were listed in item F and planned for RV4. Each needs a probe.
- The 68 open rows did not change in phase 11. Item L's script marked a row open when no commit names its ID. Some of them were answered under a collective name: commit eff1a372 (front-end round 22) says that every front-end finding of W18, W19, and I3 now passes, and W18-1 to W18-11 and W19-3, W19-4, and W19-6 are open rows.
- 7 rows in state `commit` give no commit: I4-1 to I4-3, X23-1, X23-2, X23-4, X23-5. Their Record says "decision 0058, item E repairs", because ee6085aa set them in its own commit.
- X23-7 asks for the X23 row in `exercises.md` section 4 and for a sentence on which report sections a proposal fills. 5523f0e8 added the row. The row stays `planned`.
- The gap reviews in `reports/review/` have no finding IDs, so RV4's nine doubts that "the next review reads again" have no row.

**A failed use names the principle.** No use failed in phase 11. RV4 used CNS-P2 on Q-0263, SB-P2 on Q-0395, and LSUP-P2 on Q-0299, and each use gave the provisional answer. Q-0299 stays open on the review's doubt, and its line says so. The only `a principle (ID)` line is Q-0196's, from before the phase. The choice held, and nothing tested it.

**A packet by option each phase.** PK2 wrote the packet. The close commit gives the change of each `**Waits for:**` item and does not state a share. The share rose from 57% to 64% for rows 1 to 9, and to 71% when row 10 counts. The packet's own sentence "The rows 1 to 9 hold 111 of the open questions" leaves out row 10, which PK2 added. Under the choice, a rise puts a packet among phase 12's first items. Phase 12 already has a packet, and no row of it was put to the maintainer as a request.

**Check results come from a tool.** After R5 wrote `tools/checks.py`, one commit has a Checks paragraph in another form: a2c246f8 ("check_specification.py reports 0 problems (exercises.md is not a specification page; status_table.py reports no change)"). Three commits ran the chain without the reference corpus, so similarity did not run (696e9977, 5523f0e8, 8d3a92b6). 5523f0e8 was committed after `check_specification.py` reported 2 problems, and its message says "At least one check failed". 8d3a92b6 explains the two false problems and fixes the check.

**Tools write the parts they can derive.** The Divergences tags agree with every question file. The settlements table has a commit in every row. `exercises.md` section 6 points at section 8 and the roadmap. The status page's front-end sentence comes from `conformance.tsv` and says "passes 6459 of 6681 tests", while the matrix line above it says 6711 tests and the close commit gives 6478 of 6711. The sentence is right for the file it reads, and it does not name the file.

### Phase 11 commit messages against their diffs

The reader compared each of the 15 commit bodies with its `--stat` and checked the numeric claims of L, R5, P, PK2, P1, RV4, E, and the close against the tree at each commit. These agree with the diffs:

- 5ab0686b's counts (755 commit, 68 open, 5 planned, 4 text, 3 question, 2 out of scope, 837 rows).
- 822cac53's 13 tags on 12 pages, and its tool changes.
- 47f58739's 3 new tests and one changed test, and its list of rule changes. The overload-resolution page changed only by a `:tests:` line and an agreement count, which the message does not name.
- 8d7f3bb3's 7 tests and 6 program tests; 696e9977's 17 question files; d859b0ad's P1 rows (7 planned, 1 text, 1 out of scope).
- 1e510736's four settlements, four Divergences tags, and changes in all 23 question files it names, Q-0146 and Q-0404 to Q-0410 among them.
- 7cf4574b's 6 tests and 2 questions; 5523f0e8's 21 planned rows; ee6085aa's 8 tests, 2 questions, and 7 ledger rows.
- a15a877d's open counts (171 to 174, 7 opened, 4 settled), its `**Waits for:**` changes, its 30 new tests, its 11 front-end names, and its list of 20 planned rows.

These messages misstate something, and each was corrected or is new here:

- 696e9977 says that the splits of Q-0142, Q-0274, Q-0323, and Q-0341 sit in tests that do not name them. d859b0ad corrects it: no test whose notes name them has two newest families that differ. Neither commit records the reading that the correction asks for, and the four warnings stay.
- 8d3a92b6 names the commit it corrects as 39d2b35. ee6085aa corrects it to 5523f0e.
- 5523f0e8's body says "similarity.py --fail-on-flag reports 0 flagged on both directories" for I4 and X23, and its Checks paragraph says similarity did not run. No file in `reports/similarity/` records the run on those two directories.
- a15a877d does not state the share that decision 0058 asks the closing commit to give.

### Records that drift

These records disagree with the tree at a15a877d, or with each other:

- **Planned rows that name an ended item.** 20 rows (above). No check reads a row's item against the roadmap.
- **Open ledger rows.** 68 rows say "no later commit or question names the finding". Some were answered under a range or a collective name (W18-1 to W18-11, W19-3, W19-4, W19-6 by eff1a372).
- **Commit rows with no commit.** 7 rows from ee6085aa.
- **X23-7** is half done and stays `planned`.
- **The status page's front-end sentence** gives `conformance.tsv`'s 6681 tests beside the matrix's 6711, and does not say which file it counts.
- **The packet's share sentence** counts rows 1 to 9 and leaves out row 10.
- **The packet's "answers change" column** gives the tests whose notes name the questions, the same count as "answers stay". The packet's own text says that "answers change" means "those tests change their expectation". Decision 0057's item PK asked for the tests whose expectation would change, and so did 0058's item PK2.
- **Q-0415's line** says "a review (doubt)". No review has read the question or the sentence that now answers it. "a first review" fits.
- **The five family warnings** (Q-0025, Q-0142, Q-0274, Q-0323, Q-0341) have no record that holds the reading d859b0ad asks for.
- **Q-0299's Opinions** are still out of date order (2026-11-04, 2026-11-05, 2026-11-03, 2026-11-05). Lines 56 and 66 open a code span with two backticks. Its PU3 note still sits under "Provisional answer".
- **Stacked reasons.** 10 open questions carry three or more "Kept open" lines: Q-0018 and Q-0378 seven, Q-0293 six, Q-0217, Q-0218, Q-0299, and Q-0383 five, Q-0175 four, Q-0337 and Q-0372 three.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | Other evidence |
|---|---|---|---|---|---|---|---|---|
| 1 | overloading/conversion-sequences | 47 / 133 | 13 | 243 | 27 | 178 / 1035 | 9.4 | The same 13 open questions as in round 8. No question settled in phases 9 to 11. 7 are in row 1's group, and RV4 read Q-0152 and Q-0156 with no doubt. ICS-45 changed after RV4. I3-1 (ICS-26), I3-5 (ICS-11), and I3-13 (ICS-25) are planned rows of an ended item |
| 2 | foundations/storage | 29 / 95 | 12 | 275 | 6 | 164 / 1020 | 3.6 | The same 12 open questions, with five different holds. STG-5 changed after RV4. X23-6 (STG-8, STG-P3) is planned with an ended item. Q-0307 is still the one test against all three families |
| 3 | overloading/overload-resolution | 44 / 186 | 10 | 353 | 30 | 205 / 1053 | 11.0 | The same 10. The highest R/P for a sixth round. 5 in row 1's group, 3 with the maintainer |
| 4 | classes/members | 34 / 173 | 8 | 187 | 16 | 151 / 859 | 8.5 | The same 8. CLS-26 changed after RV4. 3 wait for a review that finds the reference silent (row 10) |
| 5 | expressions/new-delete | 42 / 148 | 7 | 242 | 25 | 218 / 984 | 6.0 | F rose by 65, from X23. NEW-21, NEW-29, and NEW-P5 changed after RV4. Q-0417 has no review. X23-3 and X23-6 are planned with an ended item. P25 is proposed and not adopted |
| 6 | foundations/constant-evaluation | 27 / 155 | 7 | 439 | 24 | 187 / 828 | 5.4 | No change in phase 11. 5 of 7 are in row 1's group. CE-13 and CE-28 rest only on overridden tests |
| 7 | templates/class-template-deduction | 29 / 119 | 7 | 80 | 22 | 209 / 894 | 9.7 | No change in phase 11. 6 of its 7 wait on the maintainer (rows 3 and 4) |
| 8 | templates/ordering | 17 / 64 | 6 | 183 | 19 | 200 / 822 | 4.2 | ORD-13 changed after RV4. RV4 read Q-0293 and Q-0396 with a doubt. Q-0405 is in row 4a |
| 9 | expressions/conditional | 16 / 72 | 3 | 117 | 7 | 68 / 397 | 5.3 | New on the list. F went from 46 to 117. COND-16 changed in phases 10 and 11, and COND-16 and COND-P2 changed after RV4. Q-0418 has no review, and Q-0401 is in row 4b. I4-8 to I4-14 (COND-3, COND-8, COND-11, COND-16) are planned with an ended item. Ev is 17% of the page |
| 10 | foundations/language-support | 13 / 69 | 5 | 172 | 17 | 164 / 699 | 4.3 | Q-0386 settled. LSUP-11 and LSUP-P2 changed in F, and RV4 read both. Q-0299's records drift (above) |

Templates/constraints left the round 8 list (Q-0263 settled, and CNS-P2's repair has a review). Conditional joined it.

## 2. Ranked work items

Item FE (front-end round 23) comes first in phase 12, as decision 0058 moved it. Its inputs are the 11 tests of phase 11 that the front end does not pass and the planned rows P1-1, P1-2, and I4-4 to I4-7. The items below go with it.

| # | Item | Needs first | Evidence | Size |
|---|---|---|---|---|
| 1 | **Review the text phase 11 wrote after RV4** (RV5), then the repairs | nothing; runs beside FE | 11 rules and 4 principles changed after RV4 read the tree. Q-0417 and Q-0418 have no review. Q-0415's sentence is unread. RV4's nine doubts wait for "the next review" | one source-reader session in two parts, one writer session |
| 2 | **Answer the 20 planned rows of ended items** | the phase decision names the item in each row | I3-1, I3-5, I3-13, R10-4, R10-6 (each needs a probe; R10-4 and R10-6 deferred three times), I4-8 to I4-14, X23-3, X23-6, X23-7, P1-3, P1-5, P1-7, P1-8, P1-9 | one writer session with probes |
| 3 | **Put the packet to the maintainer, with the tests each option changes** | a tool change in `question_index.py` | 124 of 174 open questions (71%) sit in `pending.md`. Rows 1, 4a, and 10 hold 85 distinct questions, and each has an option where no answer changes. No row has been decided in two phases | under one session, plus the maintainer's answers |
| 4 | **Read the open ledger rows, and tighten the ledger check** | nothing | 68 open rows unchanged since item L, some answered under a collective name. 7 commit rows with no commit. No check reads a planned row's item | one session |
| 5 | **Repair the records that drift** (R6) | nothing; the Q-0415 line after item 1 | Section 1, "Records that drift" | under one session |
| 6 | **A narrow second review of the doubt-held questions** | item 1, so the review reads current text | 38 questions held by doubt alone. 27 review-held questions were last read on 2026-11-03. Decision 0058 deferred the item | one source-reader session in two or three parts, one writer session |

### 2.1 Review the text phase 11 wrote after RV4

RV4 read the tree after F and P. Then RV4's repairs and item E's repairs changed 15 rules and principles. The review has three parts:

- Part A reads CLS-26, CONV-9, ICS-45, NAM-21, OBJ-11, ORD-13, DEP-P6, and OPR-P3 (RV4's repairs), and ALN-4, COND-16, COND-P2, NEW-21, NEW-29, NEW-P5, and STG-5 (I4 and X23). COND-16 has changed in two phases running and has two splits (Q-0401, Q-0418).
- Part B reads Q-0417 and Q-0418 for the first time, and Q-0415 against the new NAM-21 sentence.
- Part C rereads the nine doubts that RV4's repair commit left: LAM-6, LAM-29, AUTO-P3 with RET-2, SINIT-6, SW-P1, TR-P1 with TR-4, and DED-29.

`similarity.py` checks the review before the writer reads it. If FE changes no specification text, RV5 can run at the start of the phase. Text that FE or item 2 writes would then wait for RV6.

### 2.2 Answer the 20 planned rows of ended items

The phase decision first re-points each row to a phase 12 item, so that the ledger names a live item. Then the writer answers:

- R10-4 (the return case of Q-0115) and R10-6 (a static `thread_local` data member under OBJ-11). Each needs a probe. OBJ-11 also changed in RV4's repairs, so the probe can share part A's tests.
- I3-1 (an ICS-26 example), I3-5 (ICS-11 and REF-1), and I3-13 (ICS-25), each after a probe. These fall on conversion-sequences, the weakest page.
- I4-8 to I4-14: COND-3 rows 3 and 4 have no type condition, COND-8 covers only an lvalue and a prvalue, COND-11 and OVL-11 disagree on the value category, COND-11 does not say which candidates to try, and three asks on knowledge the pages did not give. The repair should come before RV5 reads COND-16, or RV5 reads it twice.
- X23-3 (three alignment thresholds in NEW-28, NEW-22, and UNW-3), X23-6 (STG-8 and STG-P3 on implicit objects in allocated storage), and the second half of X23-7.
- P1-3, P1-5, P1-7, P1-8, P1-9: examples, a link, and the base flags on `index.md`.

Each answer sets the row to `commit`, `text`, `question`, or `out of scope`. A row that needs another phase gets a new item and a note that counts its deferrals.

### 2.3 Put the packet to the maintainer, with the tests each option changes

The share of open questions in `pending.md` rose for a second phase: 39%, then 57%, now 64% (71% with row 10). Decision 0058's choice puts a packet among the first items when the share rises. The packet exists, so the work is the part it still lacks and the request itself:

- `question_index.py` counts, for each option, the tests whose expectation the option changes. The tool can read the change from each test's expectation and overrides against the option's answer. If the tool cannot tell which tests a case covers, the column heading says "tests that name them", and the packet's text drops the claim that "those tests change their expectation".
- The share sentence counts rows 1 to 10 and gives a percentage. The close commit copies the sentence.
- The integrator asks the maintainer to answer rows 1, 4a, and 10 first. Each has an option where no answer changes, and together they hold 85 distinct open questions. Asking the maintainer is inside the repository's process.
- The seven "against" questions outside row 4 (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) join row 4a or 4b by the same family table, or the packet says why each stays out.

### 2.4 Read the open ledger rows, and tighten the ledger check

- A reading of the 68 open rows sets each one to its real state. W18-1 to W18-11 and W19-3, W19-4, W19-6 likely move to `commit` with eff1a372. The reading needs only Tidal's own commits and reports.
- `check_specification.py` fails on a `commit` row whose Record names no commit, unless the row was added in the working tree. The integrator fills the 7 rows of ee6085aa.
- `check_specification.py` warns on a `planned` row whose item the roadmap marks done or moved, so that the close commit's list comes from a tool.
- The gap reviews give each finding an ID (for example RV5-A3), and their findings get ledger rows. RV4's nine doubts get rows, planned for RV5.

### 2.5 Repair the records that drift

- The status page's front-end sentence names `conformance.tsv` and the round it records, or `status_table.py` writes both the baseline count and the current one.
- Q-0415's line changes to "a first review" until RV5 reads it.
- The five family warnings each get a dated note that states the reading, or a correction of the `**Waits for:**` line.
- Q-0299's Opinions go in date order, the two double backticks are fixed, and the PU3 note moves under Opinions.
- The ten questions with stacked "Kept open" lines keep only the last one. `git log` holds the others.

### 2.6 A narrow second review of the doubt-held questions

Round 8 proposed this item, and decision 0058 deferred it. The case is the same and the count is smaller: 38 questions are held by doubt alone, and 27 review-held questions were last read on 2026-11-03 in the cluster reviews. The prompt reads one question per section, lists its tests, and asks for a verdict on a fixed scale (supports; supports, with the missing step named; reads against; silent). A named step becomes a repair, and a "silent" verdict moves the question to row 10. Running the review after RV5 keeps it on current text.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phases 10 and 11 added no rule and no page |
| Splitting the overloading principles | Drop as an item | R/P is unchanged (11.0, 9.4). The failures on those pages are missing steps (I3-1, I3-5, I3-13), which item 2 answers. A split would restart the wait of the 22 open questions on the two pages |
| Adopting P25 (X23) | Leave to a later decision | P25 changes 33 rows and 2 test outcomes. Its findings that stand without it (X23-1, X23-2, X23-4, X23-5) are in the text. Adopting it adds text for RV6 to read |
| New Propose rounds, Diagnose batches | Defer | Items 1 to 4 reach more open questions and records per session |
| The 28 rules that rest only on overridden tests | Leave as warnings | The list has not changed for four phases |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md` |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0058 say | 33 *(referenced)* markers name them. I3-9 and P1-4 stay out of scope |

Decision 0057 asks each phase to run an exercise that writes programs. A Write suite (W20, which decision 0058 deferred) or Port suite P2 fits beside FE. The editor has no view on its topic.

## 4. Process changes

- **A planned row names a live item.** When an item ends, the close commit lists its planned rows (done in phase 11), and the next decision re-points each row to one of its items. A row deferred twice carries a count in its Record, and the decision says why it moves again.
- **A row's state comes from a reading.** Item L's script set `open` when no commit names an ID. A row stays `open` only after a writer reads the finding and the later commits. A commit that answers findings under a range names the range, so that the script can see it.
- **Review findings get IDs and ledger rows.** The gap reviews in `reports/review/` are the largest source of asks with no record. RV4's repair commit lists nine doubts for "the next review", and only the commit message holds them.
- **The share comes from a tool.** `question_index.py` prints the share of open questions that rows 1 to 10 hold. The close commit copies the line, and decision 0058's choice compares it with the previous phase's line.
- **The packet gives each option's effect, or says what it counts.** Two decisions asked for the tests whose expectation changes, and the packet gives the tests that name the questions.
- **A commit follows a passing chain.** 5523f0e8 went in with "At least one check failed" in its message. `tools/checks.py > FILE && git commit` is the chain that AGENTS.md describes. A commit that needs a failing check fixed first waits for the fix, as 8d3a92b6 then made.
- **The review of the previous phase's text reads the last repairs.** RV4 ran in the middle of phase 11, and 15 rules and principles changed after it. The phase decision names the text that each review takes, and the close commit lists the rules and principles that changed after the phase's review, for the next review to read.

## 5. Files

The reader's scripts are in `reports/clean-room/roadmap-9/editor-scratch/`: the round 8 scripts with the trees changed (`count.py`, `diff.py`, `when.py`, `q.py`, `pages.py`), and `share.py` (the `pending.md` share, unchanged questions, and review dates) and `pagewaits.py` (the holds on each page of the table). They ran on copies made with `git archive` in `/tmp/claude-1000/-home-ryanm-Code-tidal/939f1ac9-57f9-42e8-8e86-91384ffa67eb/scratchpad/r9e/`: the tree at 1e20bfa2 (`t8/`) and at a15a877d (`t9/`). Short inline scripts checked the ledger (duplicates, IDs with no row, commit rows with no commit), the packet's rows, and each commit's claims against `git show`. The reader also ran `tools/check_specification.py`, `tools/question_index.py --check`, `tools/checks.py --no-frontend`, and a direct call of `divergence_tags.current()`, which change no file.

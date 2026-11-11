# Roadmap, round 13: what to do after phase 15, from an editor's view

Date: 2026-11-11. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 12 report with the scripts beside it. In `documentation/` it read the status page, the roadmap's phase 14 and phase 15 parts, decisions 0062 and 0063, the deferral line of 0062, and `decisions/pending.md`. It read `git log 51bd5060..a6a2b16c` with every commit body (21 commits), the `--stat` of each, and the body of d25cfa62, which corrected the phase 14 records before phase 15 began. In `reports/clean-room/` it read `ledger.md`, `rust-frontend/ledger.md`, `rust-frontend/rewrite-marks.tsv`, `rust-frontend/conformance.tsv` (as data), the X25 and P4 reports, and `rust-frontend/report-26.md`. In `reports/review/` it read the four files dated 2026-11-11 that phase 15 added: the read of the RV7 repair diff, the two phase 15 gap reviews, and the read of the RV8 repair diff. In `reports/similarity/` it read the result line of the 15 phase 15 reports. In `frontend/` it read `DESIGN-rewrite.md` and sections 6 and 8 of `TEMPLATE-MODEL.md`, and no source file. It read the question files and the rule blocks that the counts below name. It did not read the other round 13 reports, `inputs/`, or `build/exercise-keys/`. It did not run `tools/run_tests.py` or `tools/frontend_check.py`, so the front-end counts come from report 26, `conformance.tsv`, and the commit messages. Read-only scripts made the counts from copies of the trees at 51bd5060 (round 12) and a6a2b16c, made with `git archive`, with the round 12 method. Commit read: `a6a2b16c`.

The page table uses the round 12 columns.

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.
- **FE**: tests in the page's test directory that fail their check at round 26 (`conformance.tsv`), with the tests of the marks file in parentheses.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 12 (51bd5060) | Now (a6a2b16c) |
|---|---|---|
| Live rules | 1685, plus 15 withdrawn | 1685, plus 15 withdrawn. No rule is new, and no rule changed class |
| Principles | 343 | 343 |
| Rule and principle text changed in the phase | 27 rules and 11 principles, plus 2 new rules | 28 rules and 8 principles, leaving out `:tests:`, `:rules:`, and `:exceptions:` lines. Templates hold 18 of the 36, classes 7, declarations 4, expressions 3, foundations 3, statements 1. Class template deduction alone holds 12 |
| Test files | 6947. 6809 cited by a rule, 138 program tests | 7015. 6863 cited by a rule, 152 program tests. 68 new, none removed, no expectation of an older test changed |
| Rules that rest on one test | 0 | 0 |
| `check_specification.py` | 1700 rules, 0 problems | 1700 rules, 0 problems |
| Tests with an override | 721 | 739 |
| Overrides by label | EDG 304, GCC 274, Clang 132, Clang 18 82, GCC 14 57, other 44 | EDG 313, GCC 286, Clang 132, Clang 18 83, GCC 14 57, other 44 |
| Tests with an override for exactly one newest family | 532 (Clang 107, GCC 205, EDG 220) | 545 (Clang 107, GCC 213, EDG 225) |
| Tests with an override for all three newest families | 1 (`replace-base-old-pointer-constant.cpp`, Q-0307) | 1, the same test |
| Tests with `tidal-error` patterns (this method) | 3145. The status page gave 3149 | 3173. The status page gives 3177 |
| Questions | 434 | 446 |
| Open questions (all provisional) | 185 | 197 |
| Settled, compiler defect | 236, 13 | 236, 13 |
| Share that `pending.md` holds (`question_index.py --share`) | Rows 1 to 9: 114 of 185 (62%). With row 10: 129 (70%) | Rows 1 to 9: 119 of 197 (60%). With row 10: 134 (68%) |
| `pending.md` rows decided by the maintainer | none of 10 | none of 10. The request of 2026-11-10 has no answer |
| Rows 1, 4a, and 10 | 88 questions and 16 rule items | 92 questions (58, 20, and 17, less three that two rows share) and 17 rule items. `question_index.py` now writes the sentence above the request, and the sentence matches |
| Marks file (`rewrite-marks.tsv`) | none | 32 tests. GC wrote 30, and round 26 added 2 |
| Front end (close commit) | 6804 of 6947 pass, 6831 pass their check | 6895 of 7015 pass, 6926 pass their check. `conformance.tsv` (round 26) gives 6892 of 7007 and 6921 |
| Completion line of decision 0060 | 107 tests fail their check, 3 of 138 program tests do not run, 9 out of scope | 80 tests, of which 32 are marked and 48 count toward the gate; 5 of 152 program tests; 9 out of scope. Round 26 ended at 77 (32 and 45) and 3 of 147 |
| Front end on page examples | 184 of 186, 2 unsupported | 190 of 192, 2 unsupported (round 26) |
| Evidence lines, whole specification | 8229 of 44811 (18.4%) | 8457 of 45284 (18.7%) |
| `tests/RESULTS.md` | 6947 rows | 7015 rows |
| `question_index.py --check` | 434 questions, 0 problems | 446 questions, 0 problems |
| Divergences tags that disagree with the question file | 0 | 0 |
| Exercise ledger | 1044 rows: 960 commit, 26 open, 33 text, 22 question, 3 out of scope | 1097 rows: 1004 commit, 16 open, 0 planned, 44 text, 30 question, 3 out of scope |
| Front-end ledger | 165 rows, 2 open (G164, G165), 4 out of scope | 165 rows, none open or planned: 156 commit, 4 text, 1 question, 4 out of scope. Every `commit` row names a commit, and every named hash exists |

The pages grew by 473 lines, and Evidence grew by 228 of them. Phase 15 changed 36 blocks, against 40 in phase 14. F5 changed 24, the RV8 repair (d6fd9cea) 18, and P4 (4431852f) 2. Eight blocks changed in two commits: CTAD-18, INST-5, OPR-8, OPR-10, OPR-16, OPR-P3, and STG-28 in F5 and the RV8 repair, and INST-1 in F5 and P4.

### The read-only checks

- `tools/check_specification.py`: "1700 rules, 0 problems", no warnings.
- `tools/question_index.py --check`: "446 questions, 0 problems", no warnings.
- `tools/question_index.py --share`: "Rows 1 to 9 hold 119 of the 197 open questions (60%). With row 10, the rows hold 134 (68%)." The line matches the packet and the close.
- A replay of `check_specification.py` and `question_index.py --check` on a `git archive` copy of each of the 21 phase 15 commits gives these results. `check_specification.py` gives 0 problems at 3a1ddc16 and one problem at the other 20, the stale-number check of the status page. A rerun of `status_table.py` on each of those 20 copies changes only the Similarity cells and the line that names the latest full similarity report, which a copy with no git history cannot order. `question_index.py --check` fails at four commits: 29e8ab25 and bcf30b72 (17 problems each), 76f9f670 (23), and 4431852f (24), and it passes at the other 17. Section "Phase 15 commit messages against their diffs" sets the failures against the messages.
- The 15 phase 15 similarity reports each report "0 flagged". They cover the T4, F5, RV8, and P4 text, the four review summaries, and the X25, P4, and round 26 reports, and the close adds a report of the three exercise reports together. No similarity output covers `frontend/DESIGN-rewrite.md`.

### What phase 15 did to the questions

Each count comes from each file's status and `**Waits for:**` line at 51bd5060 and at a6a2b16c.

- 12 new questions, all open. Q-0435 (OPR-8, OPR-10) waits for a use of OPR-P3. Q-0436 and Q-0440 (PRIM-2, OPR-8) wait for a principle, PRIM-P1. Q-0437 waits for CTAD-P1, Q-0443 for CTAD-P3, and Q-0444 for CTOR-P1. Q-0446 waits for OPR-P3 and for newest families that agree. Q-0438 and Q-0441 wait for a review (doubt) and for newest families that agree. Q-0445 waits for a review (doubt). Q-0439 (CTAD-2) and Q-0442 (ADD-14) are in `pending.md` row 4.
- No question settled, and none reopened.
- Open questions went from 185 to 197.
- 175 of the 185 questions that were open at both ends did not change at all. The ten that changed are Q-0020, Q-0045, Q-0225, Q-0226, Q-0243, Q-0277, Q-0304, Q-0429, Q-0430, and Q-0431. Each gained an Opinions entry or a dated note, and none changed its `**Waits for:**` line. The settled Q-0049 and Q-0231 gained entries too.

The `**Waits for:**` items, at the start and the end of the phase (a question can hold more than one item).

| Item | Start | End |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 65 | 67 |
| of which name row 4 (alone or with row 3 or 7) | 41 | 43 |
| of which hold the plain item "the maintainer (pending.md row 4)" | 37 | 39 |
| newest families that agree (row 1's group) | 55 | 58 |
| a review (doubt) | 53 | 56 |
| a review (silent) | 17 | 17 |
| a review (against) | 9 | 9 |
| a review (part) | 1 | 1 |
| a use of a principle | 3 (UNI-P3, DEP-P5, DEP-P2) | 4 (with OPR-P3) |
| a principle | 2 (Q-0196, Q-0429) | 8 (with Q-0436, Q-0437, Q-0440, Q-0443, Q-0444, Q-0446) |
| a rule, child questions | 1 each | 1 each |

When each question goes in the first group that fits (maintainer, then row 1's group, then a use, a first review, against, silent, doubt, part), the 197 split this way.

| What holds the question | Round 12 | Now |
|---|---|---|
| A row of `pending.md` (rows 2 to 9) | 65 | 67 (Q-0439 and Q-0442 joined) |
| Two newest families differ (row 1's group) | 49 | 52 (Q-0438, Q-0441, Q-0446 joined) |
| A review that reads with doubt | 42 | 43 (Q-0445 joined) |
| A review that finds the reference silent (packet row 10) | 15 | 15 |
| A review that reads against the answer, outside row 4 | 7 | 7, the same seven (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) |
| A use of a principle | 3 | 4 (Q-0424, Q-0427, Q-0432, Q-0435) |
| A principle | 2 | 7 (Q-0196, Q-0429, Q-0436, Q-0437, Q-0440, Q-0443, Q-0444) |
| Child questions (Q-0294), part of a review | 1 each | 1 each |

Of the 43 questions held by doubt alone, 19 were last read on 2026-11-03, 3 on 2026-11-04, 16 on 2026-11-05, and 5 on 2026-11-11. The seven "against" questions outside row 4 are the same seven as in rounds 8 to 12.

Phase 15 opened six questions that wait for a principle. At the start of the phase, two open questions waited for one. Each of the six records that its principle fails a step of the principle test of the style guide. The six wait for five principles (PRIM-P1, CTAD-P1, CTAD-P3, CTOR-P1, OPR-P3), all on pages that phase 15 changed, and CTAD-P1, CTAD-P3, and OPR-P3 themselves changed in phase 15.

### Text phase 15 wrote that no review has read

The two RV8 gap reviews read the tree at e7e972d6. They name every one of the 24 blocks that F5 changed, the 20 blocks of b5d925e0, and the three blocks that no RV7 table named (DED-20, CNS-13, CNS-P3). The read of the RV8 repair diff (76f9f670) read every hunk of d6fd9cea. 20 blocks changed after e7e972d6, and the close lists the same 20. Each row gives the words added and removed since e7e972d6, the commit, and the verdict of the read of d6fd9cea.

| Block | Page | Change | Commit | Verdict of the read of d6fd9cea |
|---|---|---|---|---|
| OPR-8 | classes/operators | +89 -18 | d6fd9cea | agrees with a doubt (RV9-R-5). Also +82 -17 in F5 |
| ADD-11 | expressions/additive | +32 | d6fd9cea | agrees (RM12-1) |
| STG-28 | foundations/storage | +24 -18 | d6fd9cea | reads against for element `k` above 0 (RV8-A-1, R-10). Also +59 -3 in F5 |
| INST-5 | templates/instantiation | +24 -6 | d6fd9cea | agrees. Also +44 in F5 |
| CTAD-18 | templates/class-template-deduction | +24 -3 | d6fd9cea | agrees. Also +59 -9 in F5 |
| FN-12 | declarations/functions/definitions | +23 -2 | 4431852f (P4) | no review |
| DEP-P1 | templates/dependent-names | +19 -3 | d6fd9cea | agrees |
| TYN-10 | declarations/type-names | +18 -1 | d6fd9cea | agrees with a doubt (RV9-R-4) |
| STG-16 | foundations/storage | +18 | d6fd9cea | reads against |
| OPR-P3 | classes/operators | +18 | d6fd9cea | agrees |
| INST-1 | templates/instantiation | +17 -1 | 4431852f (P4) | no review. The read of d6fd9cea saw only its `:tests:` line |
| DTOR-8 | classes/constructors | +15 -1 | d6fd9cea | agrees (RV9-R-1 withdraws RV8-R-1) |
| TYN-6 | declarations/type-names | +14 | d6fd9cea | agrees |
| DEP-40 | templates/dependent-names | +12 | d6fd9cea | agrees |
| OPR-16 | classes/operators | +10 | d6fd9cea | agrees |
| INST-2 | templates/instantiation | +10 -1 | d6fd9cea | agrees (RV9-R-2 withdraws RV8-R-4) |
| REF-2 | declarations/initialization/references | +7 | d6fd9cea | agrees. The doubt on a converting constructor stays |
| UNI-P3 | classes/unions | +6 -1 | d6fd9cea | agrees |
| OPR-10 | classes/operators | +4 -1 | d6fd9cea | agrees |
| ADD-14 | expressions/additive | +2 -1 | d6fd9cea | reads against (RV9-R-3, Q-0442) |

18 of the 20 blocks have a read on the record. FN-12 and INST-1 changed in P4's commit, after the read of d6fd9cea, and no review names FN-12 at all. P4 changed rule text inside an exercise item, and the read of the repair diff covered only the repair commit.

Text outside the blocks also changed after a review read it. The read of d6fd9cea covered the Notes, divergence lines, and examples of that commit. bdc27d44 then changed the Q-0444 line on the constructors page, the INST Note, and the notes of three tests, and P4 added an FN-12 Evidence note (13 lines) and an INST-1 note (11 lines on the instantiation page). No review has read these changes.

REF-2, UNI-P3, and DTOR-8 changed in three repairs running (RV6, RV7, and RV8). The read of d6fd9cea agrees with all three, so three repairs closed them, with a doubt left on REF-2 for a converting constructor (RV8-R-2) and a small doubt on UNI-P3's word "plain" (RV8-A-7). UNI-14 and DEP-P5 did not change in the RV8 repair. The RV8 review reads DEP-P5 against in the two rows that Q-0225 and Q-0226 hold (RV8-B-2).

ICS-16, which QR2 changed in phase 13, is in no review. RV6, RV7, and RV8 each left it out.

Five points stay against a review with the compilers' answer. Three have a question: RV8-A-1 with R-10 (Q-0430), the integer case of R-9 (Q-0442), and RV8-B-4 (Q-0439). Two are rule items of row 4a with no question: RV8-B-5 (CTAD-17) and RV8-B-6 (CTAD-16). The read of d6fd9cea finds that the reference accepts the CTAD-16 test that every compiler rejects. The RV6 and RV7 points of the same kind (RV6-A-4, RV6-A-12, RV6-B-2, RV7-B-4) still have no question.

### What phase 15 took from round 12

| Round 12 item | Outcome |
|---|---|
| 1. Answer the 26 open rows and G164 and G165 (F5), with a similarity record of the I7, P3, and round 25 reports first | Done. d25cfa62 made the similarity record. F5 answered 18 rows as `commit` and 4 as `question` (I7-6, I7-13, I7-14, P3-6). Round 26 answered P3-7 and P3-8. P3-9 and P3-10 stay open: their tests pass their check and stop in the run. G164 and G165 are `commit`. The three findings with no row became RM12-1 to RM12-3 (R9), and RV8 answered them |
| 2. Review the 20 blocks changed after RV7, the text outside them, and F5's text (RV8), with a recorded read of the RV7 repair diff, then the repairs, then a read of RV8's repair diff | Done in part. The read of b5d925e0 ran first (6a73ea89). Both gap reviews read the 20 blocks, the three blocks in no table, and the five blocks repaired twice as whole blocks. The read of d6fd9cea ran as its own part (76f9f670). Part C (whether a question should hold each point kept against) did not run. ICS-16 was not read |
| 3. Repair the records that drift (R9), with checks for review reads, settlement rows, and the sentence above the request | Done in part. `check_specification.py` fails when a done review item has no recorded read of its repair diff. `question_index.py` fails on a settlement row with no commit and writes the sentence above the request. d25cfa62 corrected the phase 14 messages. G164 names STG-28. The ledger header names the RV6 to RV9 forms. The request table has no line for the change of scope, and decision 0062 still defers interpreter output when every order agrees |
| 4. Put the gate's inputs in the tool | Done in part (GC). The marks file names 32 tests with their model sections, and the completion line gives them apart. Decision 0063 counts "few" without them. The tool prints no page table (report 26 built its table by hand), and the 29 tests that stop in a run stay off the line |
| 5. Route the rule items and the seven "against" questions, and date the change of the request's scope | Not done. The rule items went from 16 to 17. The seven "against" questions stayed outside row 4 for a sixth round. RV8-R-9 found that ADD-14 agrees for a base class subobject, so Q-0049 stays settled with no row 4 item |
| 6. A narrow second review of the doubt-held questions | Not done. Decision 0063 defers it, the fifth deferral. 43 questions are held by doubt alone |
| Process: the read of a review's repair diff is its own row | Adopted in part. Decision 0063 made it a standing choice and R9 made it a check. The reads ran as parts of RV8, not as rows of the phase table |
| Process: a third doubt on a block that two repairs changed opens a question | Not adopted. REF-2, UNI-P3, and DTOR-8 took a third repair, and the read agrees with each. DTOR-8's split gained Q-0444 from RM12-3 |
| Process: an exercise report passes similarity on the record | Adopted. The X25, P4, and round 26 commits each keep a similarity report, and the close keeps one for the three |
| Process: the close lists all text changed after the review | Not adopted. The close lists 20 blocks and gives a page count for the rest |
| Process: a finding in a commit message or a probe README gets a ledger row or a question in that commit | Not adopted. Two phase 15 findings have neither (section "Records that drift") |
| Process: a generated count stays generated | Adopted for the sentence above the request and the settlement commits |
| Process: the close copies its `**Waits for:**` change from a tool | Not adopted. The close's row 4 count takes the plain item, for the third phase running |

### Decision 0063's standing choices in phase 15

**The gate counts the template model apart.** Held. GC wrote the marks file, `frontend_check.py` checks each mark, and report 26 applied the gate with the marked tests apart: 45 of at most 50, no test made worse in rounds 25 and 26, and a named cause for each of the 45. Round 26 added two marks, so the round that tested the gate also moved two tests out of its count. With the marks of the round's start the count was 47. P4 then added three tests that fail, and the close's line gives 48. The gate holds by two tests.

**No rewrite code before the gate holds.** Held. RD (14ebbde6) adds `frontend/DESIGN-rewrite.md` and no other file.

**An item with a review ends after the read of its repair diff.** Held for RV8. The read of d6fd9cea (76f9f670) came before the roadmap marked RV8 done, and the R9 check passes. bdc27d44 then repaired the two readings that the read withdrew and changed notes on two pages and three tests. No read covers bdc27d44 or the rule text of 4431852f.

**Decision 0062's choices.** One completion count held. Report 26, the close, and the status page copy the line of `frontend_check.py`. Every `commit` row names its commit, in both ledgers. A front-end round ran after the last rule change of RV8, and round 26 took RV8's tests as they arrived. P4 changed FN-12 and INST-1 after round 26, and its 3 tests and 2 program tests do not pass. The close records them.

**Decision 0059's choices.** A `planned` row named a live item at every commit, and the close has no planned row. Every review point has a row: RV8-R 10, RV8-A 9, RV8-B 10, RV9-R 5. A front-end finding gets a test in the commit that records it: report 26 section 5 names a fault and the round records no finding, so the choice did not hold for that one fault. Seven commits were built in a worktree or from part of the tree (17077248, 3a1ddc16, 6a73ea89, 232ac618, 14ebbde6, 9f3bceb4, d6fd9cea). A script that lists lines one phase 15 commit adds and a later one removes finds only repairs of the same text, `:tests:` lines, generated counts, and the question template's comment lines, so no worktree commit dropped another item's work. T4, F5, and round 26 ran `frontend_check.py` on trees that held other items' work, and each message says so.

**Decision 0061's choices.** The host-only lists held in X25 and P4. P4-1 went to a probe grid on the matrix before FN-12 changed. The X25 claims wait in open rows. Report 26 ran no compiler. First tries became program tests (four r12 tries in T4, two P4 tries).

### Phase 15 commit messages against their diffs

The reader compared each of the 21 commit bodies with its `--stat`, the ledgers at each commit, the marks file at each commit, and the rule blocks at each commit. These statements agree with the diffs.

- The test counts. T4 adds 22 (13 tests and 9 program tests), F5 20 with 2 changed, d6fd9cea 18 (9 accept, 7 reject, 2 run) with 2 changed, P4 8 (3 tests and 5 program tests), and bdc27d44 changes 3. The total of 68 and the count of 7065 to 7133 `.cpp` files match the close.
- The rule changes. F5 changed 24 blocks, d6fd9cea 18, and P4 2 (FN-12, INST-1). No rule is new. The close's list of 20 blocks changed after e7e972d6 is the d6fd9cea set with FN-12 and INST-1.
- The marks file. 17077248 adds 30 rows by section (4, 5, 10, 4, 7), and a91ccc77 adds 2.
- The ledger moves. f6fccbaf sets 26 rows to planned. 6a73ea89 adds RV8-R-1 to R-10. 232ac618 sets 4 rows to `question`, and e7e972d6 sets 18 rows and G164 and G165 to `commit`. 13e3f3c0 adds RM12-1 to RM12-3. 29e8ab25 adds 19 points. bcf30b72 adds 8 open X25 rows and X25-8. d6fd9cea sets 9 rows to `text` and 3 to `question`. cb2c2a0d sets 23 rows to `commit` and P3-9 and P3-10 back to open. 76f9f670 adds RV9-R-1 to R-5. bdc27d44 and d233bde3 answer them. The close's 16 open rows are right.
- The open-question count and the share line in each message, the twelve new questions, and every `**Waits for:**` change in the close except the row 4 count below.
- bdc27d44's account of its own slip. 76f9f670 committed the read of the RV8 repair diff without its question records, and `question_index.py --check` reports 23 problems from 76f9f670 to bdc27d44.
- aa5c08e6's diff is the one change to the fallback text in `failure()` and a helper that strips a location at the start of each line. The reader ran no test, so the 12 host failures and the 392 tests of the check are not confirmed here.

These statements misstate something or leave something out, and no commit corrects them.

- The close gives "the maintainer (pending.md row 4) 37 to 39", "counting every question whose line names the item". Decision 0063's Consequences ask for "counting every question whose line names a row". 41 and then 43 questions name row 4. The phase 13 and phase 14 closes made the same count, and f940ea53 and d25cfa62 corrected them.
- 29e8ab25 commits the two RV8 gap reviews without their question records. `question_index.py --check` reports 17 problems at 29e8ab25 and at bcf30b72, until 9f3bceb4. None of the three messages gives the result of `question_index.py --check`.
- 4431852f says that `question_index.py --check` reports "24 problems that 76f9f670 left". 76f9f670 left 23. The 24th, a stale count in `pending.md`, appears first at 4431852f.
- The close says "Text outside rule blocks changed on 13 pages". After e7e972d6, 13 specification files changed in all. Text outside the blocks changed on 10 of them, and on two of those (members, language-support) only the generated agreement count changed. Notes, divergence lines, or examples changed on 8 pages: constructors, operators, function definitions, type-names, additive, storage, class template deduction, and instantiation. The close does not list them.
- 232ac618 leaves open "an out-of-range floating literal through the long double form (EDG rejects)". No ledger row or question holds the case.
- a91ccc77 says "No new findings; no ledger row" and names report 26 section 5's fault, a literal operator template called by name with explicit template arguments, which gives a wrong constant value. No test or front-end ledger row holds the fault.

### Records that drift

These records disagree with the tree at a6a2b16c, or with each other.

- **The close's row 4 count** departs from decision 0063's Consequences for the third phase (section above).
- **`question_index.py --check` failed at four commits, and the record names two.** bdc27d44 records 76f9f670 and 4431852f. No message records 29e8ab25 and bcf30b72. A commit that adds a review summary and leaves its records to a later commit fails the check in between.
- **Findings that only a commit message or a report holds.** The out-of-range floating literal through the `long double` form (232ac618) and report 26 section 5's fault on an explicit call of a literal operator template. The first touches OPR-8 and PRIM-2, where P4-7 is also open.
- **`frontend/TEMPLATE-MODEL.md` section 8.** The P3-6 row does not name Q-0441, which holds P3-6. The RV8 row says that "decision 0063's item RV8 reviews" the DEP and INST blocks, and RV8 is done. INST-1 changed again in P4, and section 8 does not say so.
- **`frontend/DESIGN-rewrite.md` section 10.** It says that the `forward_class` gap "has no ledger row", and RM12-2 is `commit` in d6fd9cea. It says that the three P3 tests join model section 3 "if a round marks them", and round 26 passed P3-7 and P3-8 with no mark. The document has no similarity record, and decision 0063 has the phase 16 rewrite start from it.
- **Ledger Records that name withdrawn items.** RV8-R-1's Record says that Q-0444 and its row 4 item hold "the read's lean", and RV8-R-4's says that row 4 gains "INST-1, INST-2". bdc27d44 removed both items (RV9-R-1, RV9-R-2). RV7-B-3's Record names the item "ADD-14, ADD-P6", which is now "ADD-14, Q-0442". Each Record states what its commit did, and none points to the row that changed it.
- **The request table of `pending.md`** has one line, dated 2026-11-10. Its scope grew from 86 questions and 15 rule items at the request to 92 and 17 now, and the table has no line for the change. Round 12 asked for one.
- **Decision 0062's deferral list** still names "interpreter output when every order agrees". Round 25 shipped it as P2-9's answer (b035efd4). Round 12 raised the same drift.
- **The roadmap's phase 15 part** has no outcome sentence. The phase 14 part gives the open-question change and the gate's result, and the phase 15 part gives only the table.
- **Decision 0063's status line** gives 45 tests toward the gate, from round 26. The close's completion line gives 48. Both are right at their commits, and the phase 16 decision needs the current one.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | FE | Other evidence |
|---|---|---|---|---|---|---|---|---|---|
| 1 | templates/class-template-deduction | 29 / 137 | 11 | 235 | 29 | 288 / 1048 | 9.7 | 5 (0) | New at the top. 12 blocks changed in phase 15, the most of any page, and CTAD-18 changed twice. RV8 read CTAD-2, CTAD-16, and CTAD-17 against (B-4, B-5, B-6), and the read of d6fd9cea kept all three against. Two of them are rule items with no question. Q 7 to 11, X 22 to 29, Evidence 209 to 288 lines |
| 2 | foundations/storage | 29 / 113 | 14 | 350 | 6 | 208 / 1118 | 3.6 | 0 | The most open questions with conversion sequences. STG-28 changed twice, and STG-16 now names an exception that both reads find against the reference (RV8-A-1, R-10). X25-2 and X25-6 are open on OBJ-1, STG-28, and STG-P2, and P4-5 on the STG-P2 example. Q-0307 is still the one test against all three families |
| 3 | classes/operators | 16 / 130 | 4 | 147 | 7 | 87 / 456 | 5.3 | 1 (0) | New on the list. Q 0 to 4. OPR-8, OPR-10, OPR-16, and OPR-P3 each changed in F5 and in the RV8 repair, and OPR-8 grew to 292 words. P4-7 is open on OPR-8, RV9-R-5 is a doubt, and report 26 section 5's fault has no test. Evidence doubled, 44 to 87 lines |
| 4 | templates/instantiation | 35 / 170 | 8 | 246 | 32 | 314 / 1098 | 7.0 | 10 (10) | INST-1 changed in F5 and P4, INST-5 in F5 and RV8. INST-1's P4 sentence has no review. A row 4 item for INST-1 and INST-2 came and went in one phase (RV8-R-4, RV9-R-2). Q-0441 is new. All 10 failures are marked |
| 5 | templates/dependent-names | 40 / 147 | 8 | 343 | 27 | 283 / 1167 | 6.7 | 12 (11) | First in rounds 11 and 12. Only DEP-P1 and DEP-40 changed. RV8-B-2 reads DEP-P5 against in the two rows of Q-0225 and Q-0226, with a DEP-25 doubt. Q-0427 and Q-0432 wait for uses of DEP-P5 and DEP-P2 for a second phase |
| 6 | overloading/conversion-sequences | 47 / 145 | 14 | 269 | 31 | 207 / 1083 | 9.4 | 5 (0) | The most open questions of any page, unchanged. ICS-16 changed in phase 13 and no review has named it. ICS-45 is a rule item with no question. Four of the five failures are out of scope |
| 7 | expressions/additive | 17 / 69 | 4 | 84 | 9 | 158 / 487 | 2.8 | 0 | Evidence is 32% of the page. ADD-11 gained 32 words (RM12-1). ADD-14 reads against for the integer case (RV9-R-3), and Q-0442 holds it in row 4a. RV8-R-9 closed the base class case |
| 8 | classes/unions | 28 / 179 | 7 | 263 | 18 | 204 / 965 | 4.7 | 0 | UNI-P3 took its third repair. X25-1 and X25-4 are open on UNI-14, and Q-0425 is the split that P27 needs. F rose from 214 to 263 with X25 |
| 9 | foundations/constant-evaluation | 27 / 163 | 9 | 484 | 30 | 202 / 843 | 5.4 | 2 (0) | Back on the list. The highest F of any page. X25-5 and X25-7 are open on CE-22 and CE-21. No block changed in phase 15 |
| 10 | declarations/functions/definitions | 16 / 65 | 0 | 96 | 13 | 112 / 410 | 3.2 | 0 | New on the list. FN-12 changed in an exercise commit and no review names it. P4-1 is open: the front end rejects the form that FN-12 now allows and accepts the form it rejects. The two tests have no row in `conformance.tsv` |

Templates/constraints, templates/deduction, and overloading/overload-resolution left the list. RV8-B read CNS-13, CNS-P3, and DED-20 with agreement or a doubt, and none of the three pages changed a question in phase 15. Overload resolution keeps the highest R/P (11.0) for a tenth round, and P3-10 is open on OVL-7.

## 2. Ranked work items

| # | Item | Needs first | Evidence | Size |
|---|---|---|---|---|
| 1 | **Answer the 16 open rows and the two findings with no row** (F6), with matrix probes of the X25 and P4 host-only claims first | ledger rows for the two findings | X25-1 to X25-7, X25-9, P4-1 to P4-5, P4-7, P3-9, P3-10. X25-1 to X25-7 fall on the object-model pages and the glossary | one writer session with probes |
| 2 | **Review the text no review has read** (RV9), with the points kept against, then the repairs, then a recorded read of the repair diff | item 1 | FN-12 and INST-1 from P4, bdc27d44's notes, ICS-16 for a fourth phase, two rule items against with no question, four questions that wait for a use | one source-reader session in two parts, one writer session, one short source-reader pass |
| 3 | **Start the rewrite with its records** (RW): the gate's current count, a takeover file the tool reads, and a similarity record of the design | the phase decision | 48 of 50 at the close. `DESIGN-rewrite.md` names three tool lines that do not exist and two open points that are stale | the implementer's item. The records are under one session |
| 4 | **Repair the records that drift** (R10), with a tool for the close's `**Waits for:**` change | nothing | Section 1, "Records that drift" | under one session |
| 5 | **Route the rule items and the seven "against" questions, and date the change of the request's scope** | nothing | 17 rule items with no question, one more in each of the last four phases. The seven are unchanged since round 8 | under one session |
| 6 | **A narrow second review of the doubt-held questions** | item 2, so the review reads current text | 43 questions held by doubt alone, 19 last read on 2026-11-03. Decisions 0058, 0059, 0061, 0062, and 0063 deferred it | one source-reader session in two or three parts, one writer session |

### 2.1 Answer the 16 open rows and the two findings with no row

The order puts the rows and probes first, then the rows that say a rule is wrong, then examples, then the front end. Each answer sets the row to `commit`, `text`, `question`, or `out of scope`.

| Rank | Rows | Work | Size |
|---|---|---|---|
| 1 | two new rows | Rows for the out-of-range floating literal through the `long double` form (232ac618) and for report 26 section 5's fault, with a test of an explicit call of a literal operator template (decision 0059) | small |
| 2 | X25-1, X25-4, X25-5, X25-7, P4-7 | Probe the host-only claims on the matrix: construction at an element of an inactive array member in constant evaluation and at run time, a trivial union copy with an element that has no value, a constant whose element an explicit destructor call ended, and the value that the `long double` form of a numeric literal receives | small |
| 3 | X25-2, X25-3 | OBJ-1 gains a row for an array, and the glossary entry for lifetime follows OBJ-1 on the end of a class object and the start of a scalar | small |
| 4 | X25-1, X25-4, X25-5, X25-7 | LSUP-10 names a member of a union whose lifetime began in the same evaluation, UNI-14 states the run-time reading of the member name, CE-22 states what a trivial union copy reads, and CE-21 gains an example with an ended element. A question holds each split | medium |
| 5 | P4-7 and the floating literal row | OPR-8 says which value the numeric form receives (CONV-4), and PRIM-2 or OPR-8 says what an out-of-range floating literal does through the `long double` form. A question holds EDG's rejection if the newest families split | small |
| 6 | P4-3, P4-4, P4-5, X25-6 | Examples: an adaptor over any sequence under RFOR-P1, a member call's result type without the library under CPT-28, the `Maybe` example with its move constructor and CPY-5, and a link from STG-P2 to the storage-form table | small |
| 7 | X25-9 | `text`: outside knowledge, and no rule rests on it | trivial |
| 8 | P3-9, P3-10, P4-1, P4-2 | The front-end round or the rewrite takes the tests (RFOR-2, OVL-7, FN-12, LSUP-1 with OPR-6). The rows stay `open` and name their tests until then. The phase decision says which front end owns each | front end |

### 2.2 Review the text no review has read

The read of d6fd9cea covered 18 of the 20 blocks changed after the RV8 reviews, so RV9 is smaller than RV8. The review has two parts.

- Part A reads FN-12 and INST-1 as changed by P4, with the P4-1 grid, and the notes that bdc27d44 and P4 changed (the Q-0444 line, the INST Note, the FN-12 and INST-1 Evidence notes, and the notes of `dtor-call-unqualified-base.cpp`, `class-decltype-consteval-call-not-instantiated.cpp`, and `integer-counterpart-one-past.cpp`). Part A also reads ICS-16, the text item 1 writes, and the questions item 1 opens. For Q-0424, Q-0427, Q-0432, and Q-0435, Part A reads UNI-P3, DEP-P5, DEP-P2, and OPR-P3 as uses.
- Part B reads the points kept against with a row 4 rule item and no question (RV8-B-5 on CTAD-17, RV8-B-6 on CTAD-16, RV6-A-4 and RV7-B-4 on UNW-3, RV6-A-12 on LSUP-11, RV6-B-2 on DEP-15 and DEP-P4) and says for each whether a question should hold it. Rounds 11 and 12 asked for this part as Part C, and decisions 0062 and 0063 left it out.

After the repairs, the short pass reads every commit that changes rule text, Notes, or test notes after the review's tree, from any item. The pass in phase 15 read only the repair commit, so P4's FN-12 change and bdc27d44's notes fell outside it.

### 2.3 Start the rewrite with its records

Item 3 belongs to the implementer's view, and the editor ranks only its records. Decision 0062 has the phase 16 decision add the rewrite once the gate holds, and round 26 found that the gate holds. These facts bear on the records.

- The gate held at 45 in round 26 and reads 48 at the close, against a limit of 50. Round 26 added two marks while it tested the gate. Items 1 and 2 add tests, and some will fail on the old front end. The phase decision says whether the gate is a test passed once, or a count that later rounds keep, and what happens to the rewrite item if the count passes 50.
- `DESIGN-rewrite.md` section 5 measures each step by `frontend_check.py` lines that the tool does not have: a shadow run of the new front end, a takeover list, and a parse count. A takeover file beside the marks file, read by the tool, keeps the owner of each test in one place. The tool fails when a test is both marked and owned, or owned and not passing on its owner.
- `tools/similarity.py` runs on `frontend/DESIGN-rewrite.md` and `frontend/TEMPLATE-MODEL.md`, and the output goes under `reports/similarity/`, before the implementer acts on them (AGENTS.md, decisions 0019 and 0029).
- `DESIGN-rewrite.md` section 10 and `TEMPLATE-MODEL.md` section 8 drop the stale points (section 1, "Records that drift").
- Until the takeover file names a test, the old front end's rounds keep answering its row. P3-9, P3-10, P4-1, and P4-2 need an owner.

### 2.4 Repair the records that drift

- A follow-up commit states what the phase 15 messages missed: the close's row 4 count (41 to 43 questions name row 4), the `question_index.py --check` failures at 29e8ab25 and bcf30b72, the 23 and 1 split of 4431852f's 24 problems, and the 8 pages whose Notes, lines, or examples changed after e7e972d6.
- `question_index.py` gains an option that prints the `**Waits for:**` change by item between two commits, counting every question whose line names a row. The close copies its output. Three closes running have counted the plain item by hand.
- Each commit message gives the result of both `check_specification.py` and `question_index.py --check`. The integrator chains both before `git commit` (AGENTS.md, "Shell habits").
- The two findings get ledger rows (section 2.1, rank 1).
- `TEMPLATE-MODEL.md` section 8 names Q-0441 for P3-6, marks RV8 done, and adds INST-1's P4 change. `DESIGN-rewrite.md` section 10 names RM12-2 and drops the P3 point.
- The Records of RV8-R-1, RV8-R-4, and RV7-B-3 name the row that later changed their item (RV9-R-1, RV9-R-2, RV8-R-9).
- The roadmap's phase 15 part gains its outcome sentence: open questions 185 to 197, 12 opened, none settled, and the gate's result.
- Decision 0062's deferral list drops interpreter output when every order agrees, and the phase 16 decision cites the completion line of the close.

### 2.5 Route the rule items and the seven "against" questions

- The 17 row 4a rule items each get a question, or the packet keeps them as rule items. CTAD-16 and CTAD-17 joined in phase 15. The read of d6fd9cea finds the CTAD-16 test valid by the reference, and every family rejects it. Item 2's Part B gives a reading for five of the items.
- The seven "against" questions outside row 4 (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) join row 4a or 4b by the family table, or the packet says why each stays out. Rounds 9 to 12 asked for this.
- The request table gains a dated line for the change of scope, from 86 questions and 15 rule items at the request to 92 and 17.
- The close records the maintainer's answer to the request of 2026-11-10, or the lack of one, as the phase 13 to 15 closes did.

### 2.6 A narrow second review of the doubt-held questions

Rounds 8 to 12 proposed this item, and decisions 0058, 0059, 0061, 0062, and 0063 deferred it. The count went from 42 to 43, and 19 of the 43 were last read in the 2026-11-03 cluster reviews. The prompt reads one question per section, lists its tests, and asks for a verdict on a fixed scale (supports; supports, with the missing step named; reads against; silent). A named step becomes a repair, and a "silent" verdict moves the question to row 10. Running the review after RV9 keeps it on current text. The rewrite does not wait on it, so the item can run beside item 3.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phases 10 to 15 added no page |
| Rewrite work on the old front end's marked tests | None | The marks file gives the 32 tests to the rewrite (decision 0063) |
| Propose X24 on P26, and a further Propose report | Defer | Each adds text for RV9 to read while the rewrite starts |
| Adopting P27's design (X25) | Keep as a design | X25 is a Propose report. Its findings X25-1 to X25-7 are about current text and go in item 1 |
| Implement I8 and a shared impact scanner | Defer | The implementer's time goes to the rewrite |
| Splitting the overloading principles | Keep off the list, as decisions 0059, 0061, 0062, and 0063 left it | R/P is unchanged (11.0, 9.4). The open rows on those pages are front-end rows |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md`. X25-9 names a later version and asks for no rule |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0063 say | 33 *(referenced)* markers name them |

## 4. Process changes

- **The read of a repair diff covers every later change to rule text and notes.** The pass reads each commit after the review's tree that changes a block, a Note, a divergence line, or a test note, from any item. P4 changed FN-12 and INST-1, and bdc27d44 changed notes, after the read of d6fd9cea.
- **An exercise item does not change rule text.** An exercise commit files a ledger row, and a writer item makes the change, so the review's set comes from the writer items. P4's FN-12 change is the one case in phase 15.
- **A review summary and its question records land in one commit.** If they cannot, the summary's commit says that `question_index.py --check` fails and names the commit that fixes it. Three phase 15 commits left the check failing, and the messages record one of them.
- **Every commit message gives both checks.** `check_specification.py` and `question_index.py --check` run before each commit, chained with `&&`, and the message gives both lines.
- **The close copies its `**Waits for:**` change from a tool.** Three closes running counted the row 4 item by a method that the phase decision did not ask for.
- **A "left open" line in a commit message becomes a ledger row in that commit.** The two phase 15 findings with no row each sit in a message line.
- **The gate's count names the marks of the round's start.** The round that tests the gate prints the line with the marks it found and with the marks it added. Round 26 gives 47 and 45.

## 5. Files

The reader's scripts are in `reports/clean-room/roadmap-13/editor-scratch/`: the round 12 scripts `count.py`, `pages.py`, `diff.py`, `when.py`, `q.py`, `groups.py`, `ledgers.py`, `hunks.py`, `repair_words.py`, `rows_1_4a_10.py`, `divtags.py`, `undone.py`, `replay.sh`, and `replay_status.sh`, with their paths changed; `named.py` (the phase 15 review files that name each block); `ledger_steps.py` (the moves of both ledgers at each commit of a range); and `fe.py` (the tests that fail their check at the last front-end round, by test directory, with the marked tests apart). They ran on copies made with `git archive` in `/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r13e/`: the tree at 51bd5060 (`t12/`), at a6a2b16c (`t13/`), and the `specification/` trees at e7e972d6 (`rv8a/`) and d6fd9cea (`rv8b/`). The reader also ran `tools/check_specification.py` and `tools/question_index.py --check` and `--share` on the worktree, which change no file.

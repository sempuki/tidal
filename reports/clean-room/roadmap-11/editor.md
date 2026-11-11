# Roadmap, round 11: what to do after phase 13, from an editor's view

Date: 2026-11-11. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 10 report with the scripts beside it. In `documentation/` it read decisions 0060 and 0061, `decisions/pending.md` with its packet, the status page, and the roadmap's phase 13 part. It read `git log c67b349b..3bd790ae` with every commit body (17 commits) and the `--stat` of each. In `reports/clean-room/` it read `ledger.md`, the I6 and P2 reports, `rust-frontend/report-24.md`, and `rust-frontend/ledger.md`. In `reports/review/` it read the two phase 13 gap reviews (`2026-11-10-gap-review-phase-13-*`), and in `reports/similarity/` the summary line of the six phase 13 reports. It read the question files and pending.md rows that the counts below name, and the parts of `specification/` and the test headers that its scripts read. It did not read the other round 11 reports, `inputs/`, `build/exercise-keys/`, or `frontend/`. It did not run `tools/run_tests.py` or `tools/frontend_check.py`, so the front-end counts come from report 24, `conformance.tsv`, and the close commit. Read-only scripts made the counts from copies of the trees at c67b349b (round 10) and 3bd790ae, made with `git archive`, with one method for both. Commit read: `3bd790ae`.

The page table uses the round 10 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.
- **FE**: tests in the page's test directory that fail their check at round 24 (report 24, section 9).

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 10 (c67b349b) | Now (3bd790ae) |
|---|---|---|
| Live rules | 1683, plus 15 withdrawn | 1683, plus 15 withdrawn. No rule is new, and no rule changed class |
| Principles | 343 | 343 |
| Rule and principle text changed in the phase | 32 rules and 4 principles, plus the new TR-17 | 34 rules and 8 principles, leaving out `:tests:`, `:rules:`, and `:exceptions:` lines. Templates hold 16 of the 42, classes 8, foundations 6, expressions 5, declarations 4, statements 2, overloading 1. Templates/deduction alone holds 10 |
| Test files | 6790. 6675 cited by a rule, 115 program tests | 6881. 6755 cited by a rule, 126 program tests. 91 new, none removed, no expectation of an older test changed |
| Rules that rest on one test | 0 | 0 |
| `check_specification.py` | 1698 rules, 0 problems, 0 warnings | 1698 rules, 0 problems, 0 warnings |
| Tests with an override | 699 | 712 |
| Overrides by label | EDG 291, GCC 267, Clang 132, Clang 18 76, GCC 14 57, other 42 | EDG 299, GCC 272, Clang 132, Clang 18 80, GCC 14 57, other 44. Round 10 gave "other 80" by another method |
| Tests with an override for exactly one newest family | 517 (Clang 107, GCC 200, EDG 210) | 525 (Clang 107, GCC 203, EDG 215) |
| Tests with an override for all three newest families | 1 (`replace-base-old-pointer-constant.cpp`, Q-0307) | 1, the same test |
| Tests with `tidal-error` patterns (this method) | 3090. The status page gave 3094 | 3128. The status page gives 3132 |
| Questions | 422 | 428 |
| Open questions (all provisional) | 176 | 180 |
| Settled, compiler defect | 233, 13 | 235, 13 |
| Share that `pending.md` holds (`question_index.py --share`) | Rows 1 to 9: 111 of 176 (63%). With row 10: 124 (70%) | Rows 1 to 9: 113 of 180 (63%). With row 10: 127 (71%) |
| `pending.md` rows decided by the maintainer | none of 10 | none of 10. The request of 2026-11-10 has no answer |
| Row 4 rule items with no question | 14, not counted by the packet | 15, counted by the packet. DEP-15 with DEP-P4 is new (RV6-B-2) |
| Front end (close commit) | 6586 of 6790 pass, 6626 pass their check | 6721 of 6881 pass, 6745 pass their check. `conformance.tsv` (round 24) gives 6718 of 6878 and 6742 |
| Completion line of decision 0060 | none. Report 23 counted 164 by the tool, 204 with the run stops | "127 tests fail their check, 2/126 program tests do not run to their result, 9 tests out of scope". CT's first count was 165 and 3 of 123 |
| Front end on page examples | 177 of 179 | 179 of 181, 2 unsupported |
| Evidence lines, whole specification | 7874 of 44046 (17.9%) | 8074 of 44408 (18.2%) |
| `tests/RESULTS.md` | 6790 rows | 6881 rows |
| `question_index.py --check` | 422 questions, 0 problems, 0 warnings | 428 questions, 0 problems, 0 warnings |
| Divergences tags that disagree with the question file | 0 | 0 |
| Exercise ledger | 925 rows: 878 commit, 17 open, 17 text, 10 question, 3 out of scope | 988 rows: 920 commit, 26 open, 0 planned, 27 text, 12 question, 3 out of scope |
| Front-end ledger | 157 rows, 5 open (G153 to G157) | 163 rows, 2 open (G161, G162), 4 out of scope (G42, G158 to G160, 9 tests) |

The pages grew by 362 lines, and Evidence grew by 200 of them. Phase 13 changed 42 blocks, against 37 in phase 12. F3 changed 31 of them and the RV6 repair 22, with 12 blocks in both.

### The read-only checks

- `tools/check_specification.py`: "1698 rules, 0 problems", exit 0, no warnings.
- `tools/question_index.py --check`: "428 questions, 0 problems", exit 0, no warnings.
- `tools/question_index.py --share`: "Rows 1 to 9 hold 113 of the 180 open questions (63%). With row 10, the rows hold 127 (71%)." The line matches the packet and the close commit.
- A replay of `check_specification.py` and `question_index.py --check` on a `git archive` copy of each of the 17 phase 13 commits gives 0 problems from `question_index.py` at every commit. `check_specification.py` gives 0 problems at 11 commits and one problem at 6 (c6c02908, 0ae4cdb5, 98efc40f, e2592575, 7dd6d7ca, 3bd790ae). At those six, a rerun of `status_table.py` on the copy changes only the similarity column of the 86 page rows and the line that names the latest full similarity report. `status_table.py` orders similarity reports by the time git first recorded them, and a copy has no git history. The live tree at 3bd790ae gives 0 problems. The replay finds no commit whose checks failed, apart from that one check that a copy cannot run.
- The six phase 13 similarity reports (`2026-11-10-phase-13-qr2`, `-t2`, `-f3`, `-rv6`, `2026-11-11-phase-13-rv6-repairs`, `-close`) each report "0 flagged", and each is under `reports/similarity/`. No phase 13 commit keeps its output under `build/`.

### What phase 13 did to the questions

Each count comes from each file's status and `**Waits for:**` line at c67b349b and at 3bd790ae.

- 6 new questions. Q-0423 (ICS-16, the mixed case split from Q-0419) waits for a review (silent), so it is in packet row 10. Q-0424 (UNI-9, an unnamed class in an anonymous union) waits for a use of UNI-P3. Q-0425 (UNI-14, a construction inside a member that is not active) waits for a review (doubt) and newest families that agree. Q-0426 (CNS-11 and CPY-15) is in `pending.md` row 4. Q-0427 (DEP-24) waits for a review (doubt). Q-0428 (INST-7) opened in F3 and settled in RV6.
- 2 settled under decision 0054: Q-0419 (QR2) and Q-0428 (RV6). No question reopened.
- Open questions went from 176 to 180.
- 170 of the 175 questions that were open at both ends did not change at all, `**Waits for:**` line included. The five that changed are Q-0410, Q-0416, Q-0418, Q-0421, and Q-0422.

The `**Waits for:**` items, at the start and the end of the phase (a question can hold more than one item):

| Item | Start | End |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 63 | 64 |
| of which name row 4 (alone or with row 3 or 7) | 39 | 40 |
| newest families that agree (row 1's group) | 54 | 55 |
| a review (doubt) | 50 | 51 |
| a review (silent) | 15 | 16 |
| a review (against) | 9 | 9 |
| a review (part) | 1 | 1 |
| a first review | 1 (Q-0422) | 0 |
| a use of a principle (JMP-P3, UNI-P3) | 0 | 2 (Q-0422, Q-0424) |
| a principle, a rule, child questions | 1 each | 1 each |

When each question goes in the first group that fits (maintainer, then row 1's group, then a use, a first review, against, silent, doubt, part), the 180 split this way:

| What holds the question | Round 10 | Now |
|---|---|---|
| A row of `pending.md` (rows 2 to 9) | 63 | 64 (Q-0426 joined) |
| Two newest families differ (row 1's group) | 48 | 49 (Q-0425 joined) |
| A review that reads with doubt | 41 | 41. Q-0419 left, Q-0427 joined |
| A review that finds the reference silent (packet row 10) | 13 | 14 (Q-0423 joined) |
| A review that reads against the answer, outside row 4 | 7 | 7, the same seven (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) |
| A use of a principle | 0 | 2 (Q-0422, Q-0424) |
| A first review | 1 | 0 |
| A principle (Q-0196), child questions (Q-0294), part of a review | 1 each | 1 each |

Of the 41 questions held by doubt alone, 20 were last read on 2026-11-03, 3 on 2026-11-04, 17 on 2026-11-05, and 1 (Q-0427) on 2026-11-10. 73 of the 77 review-held questions did not change in phase 13. The seven "against" questions outside row 4 are the same seven as in rounds 8, 9, and 10.

### Text phase 13 wrote that no review has read

RV6's two reviews read the tree at 98efc40f. They name all 20 blocks that changed after RV5 and all 31 blocks that F3 changed. They gave 34 points (RV6-A-1 to A-19, RV6-B-1 to B-15). The repair commit 9c009448 then changed 22 rules and principles, and no later commit changed rule text. The close commit lists the same 22. Each row gives the words that 9c009448 added and removed, whether a review table named the block, and the other changes to the block in phases 12 and 13:

| Block | Page | Repair | In a review table | Other changes |
|---|---|---|---|---|
| UNI-14 | classes/unions | +49 | yes, doubt (A-13) | F3 (0ae4cdb5). Q-0425 |
| DED-24 | templates/deduction | +40 -3 | yes, adds a condition (B-3) | F3 |
| REF-1 | declarations/initialization/references | +24 -11 | yes, doubt (A-1) | phase 12 (3b83788d, 3bed9cb5), F3. Four changes in two phases |
| JMP-P3 | statements/jumps | +32 | no. Q-0422 waits for a use of it | none |
| DEP-24 | templates/dependent-names | +30 | yes, doubt (B-9) | F3. Q-0427 |
| CONV-1 | foundations/conversions | +33 | yes, doubt (A-5) | phase 12 (03e578c4, 3bed9cb5) |
| RET-6 | declarations/functions/return-deduction | +27 | yes, doubt (A-2) | phase 12 (6c6f69f8, 3bed9cb5) |
| UNI-P1 | classes/unions | +27 -3 | yes, doubt (A-15) | F3 |
| REF-2 | declarations/initialization/references | +24 -3 | no. RV6-B-4 names it in its text | none. The glossary entry for a direct binding changed with it |
| VC-2 | foundations/values | +24 -1 | no | none |
| UNI-P3 | classes/unions | +22 | no. Q-0424 waits for a use of it | none |
| NEW-22 | expressions/new-delete | +16 | no. RV6-B-13 reads it through RV5-B-5 | phase 12 (F2). Clang 18 gains an override |
| DED-8 | templates/deduction | +14 -2 | yes, doubt (B-8) | F3 |
| DTOR-8 | classes/constructors | +14 | no | none |
| DTOR-6 | classes/constructors | +13 -1 | yes, doubt (A-11) | F3 |
| UNI-9 | classes/unions | +13 -1 | yes, adds a condition (A-17) | F3. Q-0424 |
| CV-6 | declarations/declarators | +9 -10 | yes, doubt (A-10) | F3 |
| DEP-P5 | templates/dependent-names | +7 | yes, doubt (B-9) | F3 |
| CPY-15 | classes/copy-move | +5 -1 | yes, against (A-18) | F3. Q-0426 |
| NAM-21 | foundations/names | +5 | yes, doubt (A-6) | phase 11, phase 12 (3bed9cb5). Q-0410 |
| UNI-6 | classes/unions | +4 -1 | yes, against (A-14) | F3. G163 |
| JMP-10 | statements/jumps | +2 | yes, doubt (A-9) | phase 12 (6c6f69f8, 3bed9cb5). Q-0422 |

Six of the 22 blocks were in no review table: JMP-P3, REF-2, VC-2, UNI-P3, NEW-22, and DTOR-8. Two questions (Q-0422, Q-0424) now wait for a use of a principle that changed after the review read it. Classes/unions holds five of the 22.

Text outside the rule blocks also changed after RV6 read the tree, and the close does not list it: the glossary entry for a direct binding, the STG-P2 table row for a union with an array member, and Evidence notes on UNI-14, TR-P1, NAM-21, LAM-6, ICS-11 with ICS-46, and DEP-15 with DEP-P4. QR2 (4045bbdd) changed ICS-16 by three words before RV6 read the tree, and neither review names ICS-16.

Two points that read against stay as `text` rows with a row 4 item and no question: RV6-A-4 (UNW-3) and RV6-A-12 (LSUP-11). RV6-B-2 (DEP-15, DEP-P4) is a `commit` row that names a new row 4 item with no question.

### What phase 13 took from round 10

| Round 10 item | Outcome |
|---|---|
| 1. Answer the 17 open rows and the 5 open front-end rows, with the front-end faults as tests first | Done. T2 wrote the tests first. F3 answered I5-1 to I5-8, I5-11, W20-1, and W20-4 to W20-6. Round 24 passed the tests of I5-9, I5-10, W20-2, and W20-3. G153 to G157 are `commit` |
| 2. Review the text phase 12 wrote after RV5, and the text of item 1 (RV6), then the repairs and a read of the repair diff | Done in part. The reviews read all 51 blocks and the five RV5 points kept with a note. The repairs changed 22 blocks. No file or commit records a read of the repair diff, and the roadmap marks the row done |
| 3. Front-end round 24, with the distance to decision 0060 from a tool | Done. `frontend_check.py` prints one completion line. The line went from 175 tests and 3 of 123 at the start of the round to 127 and 2 of 123. The out-of-scope tests are named in front-end ledger rows, which the tool reads |
| 4. Put the packet to the maintainer, on the record, with every row 4 item counted | Done in part. The request is dated in `pending.md`, and the packet counts the rule items. No row was decided. The seven "against" questions outside row 4 stayed outside for a fourth round. The close copies the share line and compares it with the start |
| 5. Repair the records that drift (R7), and extend the ledger check | Done. The close and the status page name the open rows of both ledgers. The ID form reads `RV5-A-3`. `check_specification.py` fails on a review point with no row and on an open or planned front-end row that names no test |
| 6. A narrow second review of the doubt-held questions | Not done. Decision 0061 defers it again, the third deferral. 41 questions are held by doubt alone, as in round 10 |
| Process: the review reads after the last item that writes rule text | Adopted for the order. RV6 read after F3. The short pass over the repair diff did not run, and 22 blocks changed after the review |
| Process: a point kept against the review gets a question | Not adopted. The packet counts rule items instead, and the items grew from 14 to 15 |
| Process: an exercise's front-end fault becomes a test in the exercise's commit | Adopted, with a check. I6-11 and I6-12 have tests in e2592575 |
| Process: the close names the open rows of both ledgers | Adopted |
| Process: the share comparison uses the previous close's tool line | Adopted. The close gives "63% and 70% for 176 open questions" at the start, which is the round 10 tool line |
| Process: the request to the maintainer is a dated record | Adopted |
| Process: decision 0060's distance comes from a tool | Adopted (item CT) |
| Process: similarity output goes under `reports/similarity/` | Adopted |

### Decision 0061's standing choices in phase 13

**One completion count.** Held. Report 24 (section 9), the close, and the status page copy the line of `frontend_check.py`. The status page's next step joins two lines. It gives "127 tests that fail their check, 2 of 126 program tests" as the count of round 24. Round 24's line reads 2 of 123, and 2 of 126 is the line after P2 added three program tests.

**A test that the front end leaves out has a row.** Held. G42 names four extended floating-point tests, and G158 to G160 name five more. The three tests that need an ILP32 bound or a size above the size type stay in scope.

**A first try is a program test.** Held for T2, which added `r10-list-first-try`, `r10-smart-ptr-first-try`, and `r10-tuple-second-try`. Each P2 first try equals its final version. The I6 programs are not Write or roadmap programs, and none became a program test.

**A probe the front end accepts against two compilers is a reject test.** Held. T2 made the UNI-9 probe a reject test. No later report gives such a probe.

**A reader report lists its host-only claims.** Not held. The I6 report ran its 14 programs on the host GCC and Clang only, and the P2 report names GCC 16.2.1 and Clang 22.1.8. Neither report has a list of host-only claims. I6-9 (p09), I6-13 (p13, p14), and P2-3 rest on host results. No prompt in `exercises.md` states the choice. A grep finds it only in decision 0061.

**Decision 0059's choices.** A `planned` row named a live item at every commit, and the close has no planned row. Each front-end finding got a test in the commit that records it: I6-11 and I6-12 in e2592575, and G161 and G162 name tests that exist. P2-9 names a program test, and the harness cannot see the wording that P2-9 is about. Every RV6 point has a row: 23 `commit`, 2 `question`, 9 `text`. A front-end round ran after the last rule change of the phase. Every Checks paragraph reports passing checks. Five commits ran the checks in a separate worktree (4e23b699, 2e588e25, e2592575, 7dd6d7ca, 9c009448), because other work was in progress in the main tree. 9c009448 ran `frontend_check.py` on a tree that also held round 24's work in progress, so its front-end line describes a tree that no commit holds.

### Phase 13 commit messages against their diffs

The reader compared each of the 17 commit bodies with its `--stat`, the ledger at each commit, and the rule blocks at each commit. These agree with the diffs:

- The test counts: afccbb13 adds 35 tests (27 tests, 8 program tests), 0ae4cdb5 37 with 2 changed, e2592575 2, 9c009448 14 with 1 changed, b2484ac8 3 program tests. The total is 91, as the close says.
- The rule changes: 4045bbdd (ICS-16), 0ae4cdb5 (31), 9c009448 (22). The close's list of 22 is the 9c009448 set.
- The ledger moves: c6c02908 17 open to planned, 98efc40f 13 to commit, e2592575 20 new open rows, 7dd6d7ca 34 new planned rows, 2eb7bf32 23 commit, 2 question, 9 text, bafaf6da 6 to commit, b2484ac8 8 new open rows and 1 text. The close's 26 open rows are right.
- The open-question count and the share line in each message, the six new questions, the two settlements, and the close's `**Waits for:**` changes.
- 2e588e25's nine out-of-scope tests (4, 2, 2, and 1) and six NO-DIAGNOSTIC-ALLOWED tests, which report 24's status table repeats.
- 53577f27's completion line: 127 is 69 missed rejects, 51 unsupported, 5 false rejects, and 2 EDG-AGREES.

One later commit records a correction, and it is right. 7dd6d7ca says Q-0422 and Q-0424 each wait for a principle. 9c009448 says each waits for a use of JMP-P3 or UNI-P3, because both principles changed in the phase.

These statements misstate something or leave something out, and no commit corrects them:

- 0ae4cdb5 says part A adds 17 tests and that "10 of the 38 new F3 tests" do not pass. The commit adds 37 tests, 16 of them in part A's directories. The same message's "76 new" (39 from T2's count plus 37) fits 37.
- The close and the roadmap mark RV6 done, and the roadmap row says "then repairs and a read of the repair diff". No commit or file records a read of 9c009448's diff. The close itself lists the 22 blocks "the next review reads".
- The close gives "pending.md row 4 35 to 36". That count takes only the plain item. 39 and then 40 questions name row 4, which matches the packet's 18 in row 4a and 22 in row 4b.
- The close lists the blocks changed after RV6 and leaves out the glossary entry, the STG-P2 table row, and the Evidence notes that 9c009448 also changed.

### Records that drift

These records disagree with the tree at 3bd790ae, or with each other:

- **RV6's read of the repair diff** is marked done in the roadmap row and in decision 0061's status line ("All items are done"). No record of the read exists.
- **The sentence above the request in `pending.md`** says rows 1, 4a, and 10 "hold 85 open questions and the 14 rule items of row 4a". The packet now gives 86 questions (55, 18, and 16, less three that two rows share) and 15 rule items. Q-0425 joined row 1 and DEP-15 with DEP-P4 joined row 4a after the request date, and the request row does not say so.
- **The settlements table of `pending.md`** has no commit for the QR2 row (4045bbdd) or the RV6 row (7dd6d7ca).
- **Front-end ledger commit rows with no commit.** G147, G149 to G152 (phase 12), and G153 to G157 (phase 13) are `commit` and name an item and no hash. The exercise ledger check fails on such a row, and nothing checks the front-end ledger. G157's Record still says that the front end misses two tests, which round 24 passes.
- **The host-only choice** is in decision 0061 and in no prompt, and the two reports that ran programs in phase 13 do not follow it.
- **Rows that name their own answer stay open.** I6-2's Record says "no text change". The P2 report says P2-7 needs no change. Both rows are `open`.
- **The status page's next step** credits round 24 with "2 of 126 program tests", a count from after round 24.
- **The close's list of text after RV6** names blocks only (above).
- **Exercise reports with no similarity record.** The I6, P2, and round 24 reports have no committed similarity output, and e2592575 wrote tests from I6's reductions in the same commit. The phase 12 I5, W20, and round 23 commits ran similarity on the default directories only, so the practice is older than phase 13. AGENTS.md asks that a clean-room report pass `tools/similarity.py` before a writer acts on it.
- **The ledger header** describes the ID forms of RV4 and RV5 and does not name RV6's.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | FE | Other evidence |
|---|---|---|---|---|---|---|---|---|---|
| 1 | templates/dependent-names | 39 / 138 | 6 | 319 | 23 | 237 / 1065 | 6.5 | 10 | New at the top. F rose from 183, from I6. 14 of the 18 open I6 rows name DEP rules. DEP-24 and DEP-P5 changed after RV6. DEP-15 with DEP-P4 is a row 4 item with no question (RV6-B-2). Q-0427 is new. The most front-end failures of any page. Report 24 says the DEP-24 failures need a model of the current instantiation, which it calls rewrite work |
| 2 | overloading/conversion-sequences | 47 / 144 | 14 | 249 | 31 | 207 / 1083 | 9.4 | 7 | Q-0419 settled and Q-0423 opened (row 10). ICS-16 changed in QR2 and no review named it. An Evidence note on ICS-11 with ICS-46 came after RV6. ICS-45 is a rule item with no question. The most open questions of any page |
| 3 | classes/unions | 28 / 178 | 7 | 205 | 18 | 204 / 961 | 4.7 | 5 | New on the list. Q 5 to 7 (Q-0424, Q-0425), X 13 to 18. Five blocks changed after RV6 (UNI-6, UNI-9, UNI-14, UNI-P1, UNI-P3), four of them also in F3. RV6 read UNI-6 against before the repair. Q-0424 waits for a use of UNI-P3 |
| 4 | foundations/storage | 29 / 105 | 12 | 296 | 6 | 163 / 1060 | 3.6 | 1 | Q-0425 joined. STG-P2 and STG-10 changed in F3, and the STG-P2 table changed again after RV6. STG-9 is a rule item. Q-0307 is still the one test against all three families |
| 5 | overloading/overload-resolution | 44 / 201 | 10 | 381 | 30 | 205 / 1054 | 11.0 | 4 | The same 10 questions. The highest R/P for an eighth round |
| 6 | foundations/constant-evaluation | 27 / 163 | 9 | 450 | 30 | 202 / 843 | 5.4 | 2 | Q-0425 joined. The highest F |
| 7 | templates/class-template-deduction | 29 / 120 | 7 | 84 | 22 | 209 / 894 | 9.7 | 9 | No rule change in phases 11 to 13. 6 of 7 questions wait on the maintainer. Round 24 left its failures, `r10-tuple-second-try`, and the T2 user-guide test |
| 8 | expressions/new-delete | 42 / 156 | 7 | 280 | 26 | 237 / 1016 | 6.0 | 4 | NEW-22 changed after RV6 and gained a Clang 18 override. NEW-25 and NEW-29 changed in F3. NEW-P5 is a rule item. G162 is open on LSUP-6 and NEW-31 |
| 9 | templates/deduction | 39 / 157 | 3 | 322 | 13 | 229 / 1086 | 7.8 | 4 | 10 blocks changed in phase 13, the most of any page, with 14 new tests. DED-8 and DED-24 changed after RV6. I6-14 asks for a DED-29 example, and RV6-B-15 keeps the RV5-B-10 doubt on DED-29 |
| 10 | templates/constraints | 40 / 145 | 5 | 181 | 34 | 257 / 1074 | 8.0 | 7 | New on the list. The most overrides of any page. RV6 read CNS-11's skip against (RV6-B-1), Q-0426 is in row 4b, and round 24 built the skip into the front end. I6-8 finds CNS-13 and the DEP-P6 row at two strengths |

Classes/members (Q 8, FE 1), templates/ordering (Q 6, FE 1), and expressions/conditional (Q 3, FE 1) left the list. None of them changed a question in phase 13.

## 2. Ranked work items

| # | Item | Needs first | Evidence | Size |
|---|---|---|---|---|
| 1 | **Answer the 26 open rows and G161 and G162** (F4), with matrix probes of the host-only claims first | the phase decision names the item in each row | I6-1 to I6-10, I6-13 to I6-20, P2-1 to P2-4, P2-6 to P2-9, G161, G162. 14 of the rows fall on the weakest page | one writer session with probes, two if the P2 examples go in |
| 2 | **Review the 22 blocks changed after RV6, the text outside them, and the text of item 1** (RV7), then the repairs, then a recorded read of the repair diff | item 1 | 22 blocks, 6 of them in no review table. REF-1 changed four times in two phases. Q-0422 and Q-0424 wait for a use of JMP-P3 and UNI-P3. The read of RV6's repair diff did not run | one source-reader session in two parts, one writer session, one short source-reader pass |
| 3 | **Repair the records that drift** (R8), with checks for the front-end ledger and the settlements table | nothing | Section 1, "Records that drift" | under one session |
| 4 | **Give decision 0060 a number for "few" and "stable"**, and the completion line by page from the tool | nothing | 127 tests fail their check after one round under the line. 57 of the 127 are template tests, and report 24 calls some of them rewrite work | under one session, plus the maintainer's view on the amendment |
| 5 | **Keep the packet current, and route the rule items and the seven "against" questions** | nothing | 15 rule items with no question. The seven "against" questions outside row 4 are unchanged since round 8. The request's sentence is stale | under one session |
| 6 | **A narrow second review of the doubt-held questions** | item 2, so the review reads current text | 41 questions held by doubt alone, 20 last read on 2026-11-03. Decisions 0058, 0059, and 0061 deferred it | one source-reader session in two or three parts, one writer session |

### 2.1 Answer the 26 open rows and G161 and G162

The order puts the host-only claims first, then the dependent-names rows that change rule text, then the rest. Each answer sets the row to `commit`, `text`, `question`, or `out of scope`.

| Rank | Rows | Work | Size |
|---|---|---|---|
| 1 | I6-9, I6-13, P2-3 | Probe the host-only claims on the matrix: the I6 programs p09, p13, and p14, and the P2 comparison of two `const char*` with equal text. Decision 0061 asks for this before a writer acts | small |
| 2 | I6-1, I6-9, I6-10 | DEP-5 says that an independent call uses argument-dependent lookup at the definition, and DEP-P1 gains a row. DEP-9 covers a qualified call with a dependent argument and a call whose name finds a member, and names "a point of instantiation" with INST-31 and INST-32. The I6 tests `independent-call-early` and `member-alias-independent` already decide I6-1 | medium |
| 3 | I6-5, I6-6, I6-19, I6-20 | DEP-2 on a member access through `this` and on `this` in a local class of a function template; DEP-4's last bullet leaves out the operands of `sizeof`, `alignof`, and `noexcept`; DEP-20 names a member function's return type. I6-5 and I6-6 have no test, so each needs a probe and a test | small to medium |
| 4 | I6-7, I6-8 | One strength for each check before instantiation: DEP-6 and DEP-12 against DEP-30, and CNS-13 against its DEP-P6 row. The rule kinds and the table must agree, and a question holds any split | small |
| 5 | I6-2, I6-3, I6-4, I6-17, I6-18 | DEP-P5 rows for `x.template ~T()` and for `typename T::template box b(1);`, a DEP-P5 sentence that the first phase only chooses a reading, a DEP-37 link to the operator arities, and comments in the example "The keywords". I6-2 can be `text` with no change | small |
| 6 | I6-13 to I6-16 | INST-1 gains an argument passed by value, INST-2 a call inside `decltype`, INST-5 the exception for INST-33, and INST-P2 a row for a friend defined in the class. DED-29 gains the I6-14 example, which RV7 then reads beside RV6-B-15 | small |
| 7 | G161, G162 | The loops page says how a loop goes on after a jump into its body, and what JMP-6 makes of a skipped init statement. LSUP-6 says the library defines `std::nothrow`. Round 24's tests already expect these readings | small |
| 8 | P2-1, P2-2, P2-7 | One informative example of an owning array with its copy functions deleted, and one of a counted shared handle, on the new-delete or copy-move page. A lookup that returns a pointer or `nullptr` covers P2-7. The examples run through `check_examples.py` | medium |
| 9 | P2-4, P2-6, P2-8 | A wrapping hash in `unsigned` on the multiplicative page that links CONV-2; LSUP-1 on the constructor's access and the spelling of the count type; a link to CE-3 from the array bound rule | small |
| 10 | P2-9 | Interpreter output when every explored order gives one result. Decision 0061 deferred the item, and rounds 7 to 10 raised it. The phase decision names an item for it or keeps the row open with a reason | small, in a front-end round |

### 2.2 Review the 22 blocks changed after RV6

RV6 kept the order that round 10 asked for and read after F3. The short pass over the repair diff did not run, so the 22 blocks wait for RV7. The review has three parts:

- Part A reads the 22 blocks of 9c009448, the six that no review table named first (JMP-P3, REF-2, VC-2, UNI-P3, NEW-22, DTOR-8). It also reads the glossary entry for a direct binding, the STG-P2 table, the six Evidence notes that 9c009448 added, and ICS-16.
- Part B reads the text item 1 writes, and gives a first reading of each question item 1 opens. It reads JMP-P3 and UNI-P3 as uses for Q-0422 and Q-0424.
- Part C reads the three points that stay against the review as row 4 items with no question (RV6-A-4, RV6-A-12, RV6-B-2), and says for each whether a question should hold it.

After the repairs, the short source-reader pass reads only the repair diff, and its result is a file under `reports/review/`. The close lists every block and every Evidence note that changed after that pass.

### 2.3 Repair the records that drift

- The roadmap's RV6 row and decision 0061's status line say that the read of the repair diff did not run, and point at RV7.
- `question_index.py` writes the sentence above the request in `pending.md` (the count of questions and rule items in rows 1, 4a, and 10). The request table gains a line when the scope of a pending request changes.
- The settlements table names 4045bbdd and 7dd6d7ca. `question_index.py --check` fails on a settlement row with a count and no commit.
- The front-end ledger's `commit` rows name a commit: G147, G149 to G157. `check_specification.py` applies the exercise ledger's hash check to the front-end ledger. G157's Record drops the claim that round 24 answered.
- I6-2 and P2-7 become `text`.
- The status page's next step gives each count with the commit whose line it copies.
- The ledger header names the RV6 form, or says that each review's own form is used.
- Each exercise commit runs `tools/similarity.py` on the report's directory and keeps the output under `reports/similarity/`.

### 2.4 Give decision 0060 a number for "few" and "stable"

Item 4 belongs to the front-end view, and the editor ranks only its records. Decision 0060 lets a roadmap propose the rewrite early "if the remaining failures are few and stable over two rounds". It gives no number for either word. The completion line now has one meaning, so a number can be tested:

- The amendment states "few" as a count of the completion line, and "stable" as the change of that count between two rounds with no change to the tests that the count reads.
- `frontend_check.py` prints the completion line by page, so a roadmap copies the page table and does not rebuild it from a report. Report 24, section 9 gives the table by hand. The table groups by test directory, so `declarations/initialization` holds several pages.
- Round 24 gives the first measurement under the line: 127 tests, of which templates hold 57 (dependent-names 10, class-template-deduction 9, constraints 7, instantiation 6). Round 25 gives the second.

### 2.5 Keep the packet current, and route the rule items and the seven "against" questions

- Each of the 15 row 4 rule items gets a question, or the packet keeps them as rule items and the sentence above the request counts them from the tool. Round 10 asked for one or the other, and phase 13 did the second.
- The seven "against" questions outside row 4 (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) join row 4a or 4b by the family table, or the packet says why each stays out. Rounds 9 and 10 asked for this.
- The close records the maintainer's answer to the request of 2026-11-10, or the lack of one, as the phase 13 close did.

### 2.6 A narrow second review of the doubt-held questions

Rounds 8 to 10 proposed this item, and decisions 0058, 0059, and 0061 deferred it. The count stayed at 41, and 20 of the 41 were last read in the 2026-11-03 cluster reviews. The prompt reads one question per section, lists its tests, and asks for a verdict on a fixed scale (supports; supports, with the missing step named; reads against; silent). A named step becomes a repair, and a "silent" verdict moves the question to row 10. Running it after RV7 keeps it on current text.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phases 10 to 13 added no page |
| The front-end rewrite (FR) | Wait for decision 0060's test, or for item 4's numbers and a second round under them | The completion line fell from 175 to 127 in one round, so it is not stable yet |
| Adopting P25 (X23), Propose X24 on P26 | Defer | Each adds text for RV7 to read |
| Interpreter output when every order agrees | Take it only as P2-9's answer in a front-end round | Decision 0061 deferred it, and P2-9 is its fifth raising |
| Splitting the overloading principles | Keep off the list, as decisions 0059 and 0061 left it | R/P is unchanged (11.0, 9.4). The open rows on those pages are missing steps |
| The exercise of decision 0057 | The editor has no view on its topic | I6 read dependent-names, which tops the list now. A blind build of a page that no build has read would add rows to item 1's successor |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md` |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0061 say | 33 *(referenced)* markers name them |

## 4. Process changes

- **The read of a review's repair diff is its own item.** The phase table gives it a row and a prerequisite, so the roadmap cannot mark it done inside the review item. The read leaves a file under `reports/review/`.
- **The close lists all text changed after the review.** Blocks, glossary entries, tables, and Evidence notes each count. A tool option can list them from the review's commit.
- **The host-only choice goes into the prompts.** Each prompt in `exercises.md` that runs programs asks for a section "Host-only claims", and the writer probes each claim on the matrix before acting.
- **The front-end ledger has the exercise ledger's checks.** A `commit` row names a commit.
- **A generated count stays generated.** The sentence above the request in `pending.md` and the settlement rows come from `question_index.py`, which fails `--check` when they are stale.
- **A row whose report answers "no change" goes in as `text`** in the exercise's commit.
- **Decision 0060 states its numbers.** A roadmap can then test the early proposal from the tool line.
- **An exercise report passes similarity on the record.** The exercise commit keeps the output under `reports/similarity/`.

## 5. Files

The reader's scripts are in `reports/clean-room/roadmap-11/editor-scratch/`: the round 10 scripts with the trees changed (`count.py`, `diff.py`, `when.py`, `q.py`, `share.py`, `pages.py`, `pagewaits.py`, `smalldiff.py`, `divtags.py`, `ledger_moves.py`, `replay.sh`), `groups.py` (the exclusive question groups with last-read dates), `repair_words.py` (the word changes of each block between two copies), `ledger_hash.py` (the states of both ledgers and the `commit` rows with no hash), `rows_1_4a_10.py` (the distinct questions of packet rows 1, 4a, and 10), and `tests_per_commit.sh` (tests added and changed by each commit). They ran on copies made with `git archive` in `/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r11e/`: the tree at c67b349b (`t10/`), at 3bd790ae (`t11/`), the specification before and after 9c009448 (`pre/`, `post/`), and before and after 4045bbdd (`q0/`, `q1/`). The reader also ran `tools/check_specification.py`, `tools/question_index.py --check` and `--share`, which change no file.

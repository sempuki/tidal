# Roadmap, round 14: what to do after phase 16, from an editor's view

Date: 2026-11-11. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 13 report with the scripts beside it. In `documentation/` it read the status page, the roadmap's phase 16 part, decision 0064 in full, the parts of decisions 0060 to 0063 that phase 16 changed, and `decisions/pending.md`. It read `git log b73a7d2d..HEAD` with every commit body (17 commits) and the `--stat` of each. In `reports/clean-room/` it read `ledger.md`, `rust-frontend/ledger.md`, `rust-frontend/rewrite-marks.tsv`, `rust-frontend/takeover.tsv`, `rust-frontend/conformance.tsv` (as data), the X24 report, `rust-frontend/report-rw1.md`, and `rust-frontend/report-27.md`. In `reports/review/` it read the two files that phase 16 added: the gap review of the phase 16 text (RV9) and the read of the RV9 repair diff. In `reports/similarity/` it read the result line of the 10 phase 16 reports. In `frontend/` it read `DESIGN-rewrite.md` and sections 6 and 8 of `TEMPLATE-MODEL.md`, and no source file. In `tools/` it read the takeover part of `frontend_check.py`. It read the question files and the rule blocks that the counts below name. It did not read the other round 14 reports, `inputs/`, or `build/exercise-keys/`. It did not run `tools/run_tests.py` or `tools/frontend_check.py`, so the front-end counts come from report 27, `conformance.tsv`, and the commit messages. Read-only scripts made the counts from copies of the trees at b73a7d2d (round 13) and f77bbe05, made with `git archive`, with the round 13 method. Commit read: `f77bbe05`.

The page table uses the round 13 columns.

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.
- **FE**: tests in the page's test directory that fail their check at round 27 (`conformance.tsv`), with the tests of the marks file in parentheses.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 13 (b73a7d2d) | Now (f77bbe05) |
|---|---|---|
| Live rules | 1685, plus 15 withdrawn | 1685, plus 15 withdrawn. No rule is new, and no rule changed class |
| Principles | 343 | 343 |
| Rule and principle text changed in the phase | 28 rules and 8 principles | 14 rules and 4 principles, leaving out `:tests:`, `:rules:`, and `:exceptions:` lines. Foundations holds 9 of the 18, templates 4, declarations 3, classes 2. F6 changed 11 and the RV9 repair 10, and CE-22, OBJ-1, and UNI-14 changed in both |
| Test files | 7015. 6863 cited by a rule, 152 program tests | 7073. 6907 cited by a rule, 166 program tests. 58 new, none removed, no expectation of an older test changed |
| Rules that rest on one test | 0 | 0 |
| `check_specification.py` | 1700 rules, 0 problems | 1700 rules, 0 problems |
| Tests with an override | 739 | 755 |
| Overrides by label | EDG 313, GCC 286, Clang 132, Clang 18 83, GCC 14 57, other 44 | EDG 326, GCC 289, Clang 132, Clang 18 83, GCC 14 59, other 44 |
| Tests with an override for exactly one newest family | 545 (Clang 107, GCC 213, EDG 225) | 557 (Clang 107, GCC 214, EDG 236) |
| Tests with an override for all three newest families | 1 (`replace-base-old-pointer-constant.cpp`, Q-0307) | 1, the same test |
| Tests with `tidal-error` patterns (this method) | 3173. The status page gave 3177 | 3197. The status page gives 3201 |
| Questions | 446 | 449 |
| Open questions (all provisional) | 197 | 200 |
| Settled, compiler defect | 236, 13 | 236, 13 |
| Share that `pending.md` holds (`question_index.py --share`) | Rows 1 to 9: 119 of 197 (60%). With row 10: 134 (68%) | Rows 1 to 9: 122 of 200 (61%). With row 10: 137 (68%) |
| `pending.md` rows decided by the maintainer | none of 10 | none of 10. The request of 2026-11-10 has no answer |
| Rows 1, 4a, and 10 | 92 questions and 17 rule items | 94 questions (60, 20, and 17, less three that two rows share) and 18 rule items. The sentence above the request gives 94 and 18 |
| Marks file (`rewrite-marks.tsv`) | 32 tests | 36 tests. Round 27 added 4: 2 to model section 3 (now 13) and 2 to section 4 (now 6) |
| Front end (close commit) | 6895 of 7015 pass, 6926 pass their check | 6967 of 7073 pass, 6990 pass their check. `conformance.tsv` (round 27) gives the same |
| Completion line of decision 0060 | 80 tests, of which 32 are marked and 48 count toward the gate; 5 of 152 program tests; 9 out of scope | 74 tests, of which 36 are marked and 38 count toward the gate's measure (42 with the 32 marks the round started with); 0 of 166 program tests; 9 out of scope |
| Front end on page examples | 190 of 192, 2 unsupported | 191 of 193, 2 unsupported |
| The rewrite (`--next`) | none | 88 unit tests pass; lex 7191 of 7191; parse 6475 of 6913; 0 tests owned |
| Evidence lines, whole specification | 8457 of 45284 (18.7%) | 8643 of 45646 (18.9%) |
| `tests/RESULTS.md` | 7015 rows | 7073 rows |
| `question_index.py --check` | 446 questions, 0 problems | 449 questions, 0 problems |
| Divergences tags that disagree with the question file | 0 | 0 |
| Exercise ledger | 1097 rows: 1004 commit, 16 open, 0 planned, 44 text, 30 question, 3 out of scope | 1129 rows: 1030 commit, 15 open, 0 planned, 48 text, 33 question, 3 out of scope |
| Front-end ledger | 165 rows, none open | 180 rows: 157 commit, 14 open (G166 to G179), 4 text, 1 question, 4 out of scope. Every `commit` row names a commit, and every named hash exists |

The pages grew by 362 lines, and Evidence grew by 186 of them. Phase 16 changed 18 blocks, against 36 in phase 15. F6 (b3f17197) changed 11 and the RV9 repair (8d33fcb1) 10. T5 (0681ae59) added 42 entries to the `:tests:` lines of 23 rules, F6 18 entries to 11 blocks, 66fc1016 one to TR-12, and 8d33fcb1 17 to 8 blocks. No commit removed a `:tests:` entry, so the hand merges of b3f17197 and 8d33fcb1 dropped no test.

### The read-only checks

- `tools/check_specification.py`: "1700 rules, 0 problems", no warnings.
- `tools/question_index.py --check`: "449 questions, 0 problems", no warnings.
- `tools/question_index.py --share`: "Rows 1 to 9 hold 122 of the 200 open questions (61%). With row 10, the rows hold 137 (68%)." The line matches the packet and the close.
- `tools/waits_change.py 5f0ffdd9 HEAD` gives the seven lines that the close copies, and the same lines from b73a7d2d.
- A replay of `check_specification.py` and `question_index.py --check` on a `git archive` copy of each of the 17 phase 16 commits gives these results. `question_index.py --check` passes at all 17. `check_specification.py` gives 0 problems at the six commits from b3f17197 to 35ced7de and one problem at the other 11, the stale-number check of the status page. A rerun of `status_table.py` on nine of those copies changes only the Similarity cells and the line that names the latest full similarity report, which a copy with no git history cannot order. So no phase 16 commit left either check failing.
- The 10 phase 16 similarity reports each report "0 flagged". They cover X24, F6, the RV9 records, repairs, read, and read records, the rewrite documents with report RW1, round 27 with the `frontend/` documents, the close, and the three exercise reports together.

### What phase 16 did to the questions

Each count comes from each file's status and `**Waits for:**` line at b73a7d2d and at f77bbe05.

- 3 new questions, all open. Q-0447 (CE-21, OBJ-2) waits for a review (doubt) and newest families that agree. Q-0448 (CE-22) waits for a review (doubt). Q-0449 (CE-22) waits for a review (doubt), newest families that agree, and a principle, CE-P3.
- No question settled, and none reopened.
- Open questions went from 197 to 200.
- 192 of the 197 questions that were open at both ends did not change at all. The five that changed are Q-0063, Q-0425, Q-0430, Q-0444, and Q-0445. One changed its `**Waits for:**` line. Q-0063 went from "a review (against)" to "the maintainer (pending.md row 4)" with the row 4 item "PRIM-15, Q-0063" (RV9-6).

The `**Waits for:**` items, at the start and the end of the phase (a question can hold more than one item).

| Item | Start | End |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 67 | 68 |
| of which name row 4 (alone or with row 3 or 7) | 43 | 44 |
| of which hold the plain item "the maintainer (pending.md row 4)" | 39 | 40 |
| newest families that agree (row 1's group) | 58 | 60 |
| a review (doubt) | 56 | 59 |
| a review (silent) | 17 | 17 |
| a review (against) | 9 | 8 |
| a review (part) | 1 | 1 |
| a use of a principle | 4 | 4, the same four |
| a principle | 8 | 9 (with Q-0449, CE-P3) |
| a rule, child questions | 1 each | 1 each |

When each question goes in the first group that fits (maintainer, then row 1's group, then a use, a first review, against, silent, doubt, part), the 200 split this way.

| What holds the question | Round 13 | Now |
|---|---|---|
| A row of `pending.md` (rows 2 to 9) | 67 | 68 (Q-0063 joined) |
| Two newest families differ (row 1's group) | 52 | 54 (Q-0447, Q-0449 joined) |
| A review that reads with doubt | 43 | 44 (Q-0448 joined) |
| A review that finds the reference silent (packet row 10) | 15 | 15 |
| A review that reads against the answer, outside row 4 | 7 | 6 (Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) |
| A use of a principle | 4 (Q-0424, Q-0427, Q-0432, Q-0435) | 4, the same |
| A principle | 7 | 7, the same |
| Child questions (Q-0294), part of a review | 1 each | 1 each |

Of the 44 questions held by doubt alone, 19 were last read on 2026-11-03, 3 on 2026-11-04, 16 on 2026-11-05, and 6 on 2026-11-11. Q-0063 left the "against" group because RV9-6 put it in row 4, and no item routed the other six. They are the six of rounds 8 to 13 less Q-0063.

Q-0424, Q-0427, and Q-0432 have waited for a use of a principle since phase 14, and Q-0435 since phase 15. No phase 16 review read UNI-P3, DEP-P5, DEP-P2, or OPR-P3 as a use.

### Text phase 16 wrote that no review has read

The RV9 gap review read the tree at 1cc1696e. It covers FN-12 and INST-1 as 4431852f changed them, every hunk of bdc27d44 with its three test notes, and the text of F6 (b3f17197), block by block and example by example. The read of the RV9 repair diff (b24e224f) read every hunk of 8d33fcb1 under `specification/`. After 1cc1696e, `specification/` changed only in 66fc1016 and 8d33fcb1. So every block that phase 16 changed has a recorded read. The read left these points, each an open ledger row.

| Point | Block | Kind | Ledger |
|---|---|---|---|
| RV10-R-3 | OBJ-2 | adds a condition. OBJ-2's new item and UNI-5 give two results for an assignment through the name of a scalar union member that a destructor call ended | open |
| RV10-R-1 | INST-30, INST-1, INST-P4 | doubt. INST-30's exception names 2 of the 5 places where the class is complete | open |
| RV10-R-2 | FN-12 | doubt. A nested class defined after its enclosing class | open |
| RV10-R-5 | CPT-28 | doubt. A variable template and an `if constexpr` condition | open |
| RV10-R-6 | LSUP-10, UNI-14, row 4 item | record. The Note leaves out "in the same evaluation" | open |
| RV10-R-7 | OBJ-1 | doubt. Vacuous initialization of an array | open |
| RV10-R-8 | OPR Note | record. The Note names a suffix that the test does not declare | open |
| RV10-R-4 | CE-22 | doubt, held by Q-0449 | question |

These texts changed in phase 16 and no review names them.

- TR-12's `:tests:` line and `string-literal-open-at-end-of-file.cpp` (66fc1016). The test is a grammar error with no wording of its own.
- The 20 rule tests of T5 and their 42 `:tests:` entries on 23 rules (0681ae59). Every compiler agrees on each, and the tests encode readings of TSP-14, CNS-3, CPY-4, OVL-24, and others that no review has checked.
- Q-0425's dated note of b3f17197 (X25-4), which records the run probe. RV9-8 read the UNI-14 Evidence note that gives the same probe.

Older text also stays unread. ICS-16 changed in phase 13 (QR2), and RV6, RV7, RV8, and RV9 each left it out. The last review that names it is the phase 12 gap review.

The read of the RV9 repair diff gives readings that hold no ID and no ledger row.

- Q-0449's three forms that no test holds. For a copy from a `constexpr` union whose active member is not `mutable`, the read finds the copy valid. Clang and GCC reject, EDG accepts, and CE-22 sets no limit, so the text and the review agree against two newest families with no test. For a struct with a `mutable` member and for a read of the `mutable` member, the read finds the evaluation not constant, as Clang and GCC give, and EDG accepts. Only the prose of Q-0449 holds the three forms.
- The INST Note's EDG split. The read places a `static constexpr` member's point after the class for a read in a default argument, so EDG's rejection reads against.

Points kept against a review with the compilers' answer and no question grew by one. RV9-7 added the rule item "LSUP-10, UNI-14" to row 4. The others are RV8-B-5 (CTAD-17), RV8-B-6 (CTAD-16), RV6-A-4 and RV7-B-4 (UNW-3), RV6-A-12 (LSUP-11), and RV6-B-2 (DEP-15, DEP-P4). Rounds 11 to 13 asked a review to say for each whether a question should hold it, and no review has.

Some blocks keep changing. INST-1 changed in five commits since phase 14 (1130ceb8, b5d925e0, 232ac618, 4431852f, 8d33fcb1). OPR-8 changed in three (232ac618, d6fd9cea, b3f17197), UNI-14 in three (b5d925e0, b3f17197, 8d33fcb1), and STG-P2 in three (1130ceb8, b5d925e0, b3f17197). FN-12, TYN-6, and TYN-10 changed in two each. Each read after the last change agrees or leaves a small doubt, and INST-1 and UNI-14 each carry an open point (RV10-R-1, RV10-R-3 through UNI-5).

### What phase 16 took from round 13

| Round 13 item | Outcome |
|---|---|
| 1. Answer the 16 open rows and the two findings with no row (F6), with matrix probes of the X25 and P4 host-only claims first | Done. F6-1 and F6-2 became rows. F6 ran probe grids, answered 9 rows as `commit` in 1cc1696e, X25-5 as `question` (Q-0448), and X25-9 as `text`, and sent P4-7, X25-7, and F6-2 to item FE. Round 27 answered those three and P3-9, P3-10, P4-1, and P4-2 (7ed491cf). No phase 16 row stays open |
| 2. Review the text no review has read (RV9), with the points kept against, then the repairs, then a recorded read of the repair diff | Done in part. RV9 read FN-12, INST-1, bdc27d44, and F6's text, and the read of 8d33fcb1 ran as part 3. Part B (whether a question should hold each point kept against) did not run, the third round that asked for it. ICS-16 and the four questions that wait for a use were not read |
| 3. Start the rewrite with its records (RW): the gate's current count, a takeover file the tool reads, and a similarity record of the design | Done. Decision 0064 made the gate's count a measure. `takeover.tsv` exists, and `frontend_check.py --next` reads it. 8f487785 and aa8c8e60 keep similarity records of the `frontend/` documents. The tool prints a problem in the takeover list and does not fail on it, and it has no check for a test that is both marked and owned |
| 4. Repair the records that drift (R10), with a tool for the close's `**Waits for:**` change | Done in part. `tools/waits_change.py` exists, and the close copies its lines. 5775d314 corrected the phase 15 records and gave the roadmap's phase 15 outcome. Decision 0062's deferral list names P2-9's answer. 8f487785 dropped the stale points of `DESIGN-rewrite.md` section 10 and named Q-0441 in `TEMPLATE-MODEL.md` section 8. The Records of RV8-R-1, RV8-R-4, and RV7-B-3 still name withdrawn items |
| 5. Route the rule items and the seven "against" questions, and date the change of the request's scope | Not done. The rule items went from 17 to 18. RV9-6 moved Q-0063 into row 4b, and the other six stay outside row 4 for a seventh round. The request table has one line |
| 6. A narrow second review of the doubt-held questions | Not done. Decision 0064 defers it, the sixth deferral. 44 questions are held by doubt alone |
| Process: the read of a repair diff covers every later change to rule text and notes | Not adopted, and not needed in phase 16. The only later change under `specification/` was 66fc1016's `:tests:` line |
| Process: an exercise item does not change rule text | Not adopted as a choice. It held in fact: X24, RW, and round 27 changed no file under `specification/` |
| Process: a review summary and its question records land in one commit | Adopted (decision 0064) and held in b310bf11 and b24e224f |
| Process: every commit message gives both checks | Adopted and held in 16 of 17 messages. f77bbe05 changes no file and gives none |
| Process: the close copies its `**Waits for:**` change from a tool | Adopted and held |
| Process: a "left open" line becomes a ledger row in that commit | Adopted. 8f487785 broke it for the old lexer's panic, and 66fc1016 added G180 and said so |
| Process: the gate's count names the marks of the round's start | Adopted. Report 27 gives 42 with the 32 starting marks and 38 with the 36 at the end |

### Decision 0064's standing choices in phase 16

**The old front end stays the front end of record.** Held. `takeover.tsv` has no row, and report 27 fixed no marked test in the old code. Round 27 changed `lexer.rs`, a file the rewrite moved, and aa8c8e60 made the same change in `frontend/next/`. Report 27 section 10 first named the change as left for others, and the commit message says the reader then made it.

**The gate's count becomes a measure.** Held. Report 27 gives the count with the starting marks (42) and with the marks at the end (38) and names a cause for each of the 38. Round 27 added four marks, one of them an RV9 test that arrived during the round.

**A review summary lands with its question records.** Held. b310bf11 holds the RV9 summary with six question records, and b24e224f holds the read of 8d33fcb1 with seven. `question_index.py --check` passes at every phase 16 commit, against four failing commits in phase 15.

**Every commit message gives the result of both checks.** Held in 16 of 17 messages. f77bbe05 is an empty commit that corrects a share figure of 58060b8f, and its tree is the tree of 58060b8f.

**A "left open" line becomes a ledger row in the same commit.** Held in part. 8f487785 names the old lexer's panic in report RW1 section 5 with no row, and 66fc1016 adds G180 and a test and states the miss. aa8c8e60 names two follow-ups of report 27 section 10 and does both. 8d33fcb1 says that "no test holds" two `mutable` forms, and the read then finds a third form with Tidal's text and the review against Clang and GCC. Only Q-0449's prose holds the three.

**The close's Waits-for lines come from `tools/waits_change.py`.** Held. The seven lines of 58060b8f match the tool's output for 5f0ffdd9 to HEAD, including "names pending.md row 4: 43 to 44". 54d51c9b's check of the tool on the phase 15 range gives "41 to 43", which the reader's rerun confirms.

**Decision 0063's choices.** An item with a review ended after the read of its repair diff: b24e224f precedes the roadmap's "Done" for RV9, and the R9 check passes.

**Decision 0062's choices.** One completion count held. Report 27, the close, and the status page copy the same line. Every `commit` row names its commit, in both ledgers. A front-end round ran after the last rule change of RV9 and took its tests as they arrived.

**Decision 0059's choices.** A `planned` row named a live item at every commit, and the close has no planned row. Every review point has a row: RV9 14, RV10-R 8. Six commits were built from an earlier head and applied onto a later one, or checked in a worktree with part of the tree (0681ae59, b3f17197, 8f487785, b310bf11, 8d33fcb1, aa8c8e60). A script that lists lines one phase 16 commit adds and a later one removes finds only repairs of the same text, `:tests:` lines, generated counts, ledger Records, question template comments, and the file counts of `DESIGN-current.md`. No commit dropped another item's work.

### Phase 16 commit messages against their diffs

The reader compared each of the 17 commit bodies with its `--stat`, the ledgers at each commit, the marks file, the questions, and the rule blocks at each commit. These statements agree with the diffs.

- The test counts. T5 adds 34 (20 rule tests: 5 reject, 15 run; and 14 program tests), F6 12 (6 accept, 4 reject, 2 run) with 3 changed, 66fc1016 1, and 8d33fcb1 11 (2 accept, 6 reject, 3 run) with 2 changed. The total of 58 and the count of 7133 to 7191 `.cpp` files match the close. The overrides that the messages name for F6 and 8d33fcb1 are on the tests, and the count of tests with an override grew by 16.
- The `:tests:` lines. T5 adds 42 entries for 20 tests to 23 rules, as its message says.
- The rule changes. F6 changed the 8 rules and 3 principles its message names, with `:tests:` lines only on OPR-1, PRIM-15, RFOR-2, RFOR-3, and CPT-28. 8d33fcb1 changed the 10 blocks that the close lists.
- The marks file. aa8c8e60 adds the four rows by section (3, 3, 4, 4).
- The ledger moves. 5f0ffdd9 sets 16 rows to planned. bc9c38f2 adds X24-1 to X24-8. b3f17197 sets X25-5 to `question`, X25-9 to `text`, and adds F6-1 and F6-2. 1cc1696e sets 9 rows to `commit` and P4-7, X25-7, and F6-2 to planned for item FE. 8f487785 adds G166 to G179, and 66fc1016 adds G180. b310bf11 adds RV9-1 to RV9-14. 8d33fcb1 sets RV9-3, RV9-5, and RV9-13 to `text` and RV9-4 to `question`. 35ced7de sets 10 rows to `commit`. b24e224f adds RV10-R-1 to R-8. 7ed491cf sets 7 rows and G180 to `commit`. The close's 15 open rows and 14 open G rows are right.
- The open-question count and the share line in each message, the three new questions, and every `**Waits for:**` change in the close.
- The pages count. The close says 13 pages changed after 1cc1696e. 12 pages and the glossary changed.

These statements misstate something or leave something out, and no commit corrects them.

- 8d33fcb1 says that the writer worked from 1cc1696e and that the integrator copied the changed files onto b310bf11 with "no file" overlapping the commits between. Six files that 8d33fcb1 changes also changed in b310bf11: `pending.md`, `status.md`, `ledger.md`, and Q-0063, Q-0447, and Q-0448. A replay finds no line of b310bf11 lost, so the commit built on part 1's records, and the message's account is wrong.
- The close says that the 10 blocks changed after RV9 read the tree and that "the next review reads them". The read of 8d33fcb1 already read all 10, and seven of its points are open rows. The close does not name that read for those blocks. It also calls INST-P4, a principle, a rule.
- Report 27 section 8 says the 38 tests are "19 missed rejects, 16 unsupported, 1 false reject, and 2 EDG-AGREES". Its own table and `conformance.tsv` give 17 missed rejects and 18 unsupported.
- F6-1's Record says that PRIM-15 "makes the literal conditionally supported". RV9-6 reads the form against, and row 4 now holds it. The Record states what b3f17197 did and does not point to RV9-6.

f77bbe05 corrects 58060b8f's start-of-phase share figure (67% to 68%), as AGENTS.md asks of a wrong message. b3f17197's line gave 67% for 199 open questions, and the close copied that figure for 197.

### Records that drift

These records disagree with the tree at f77bbe05, or with each other.

- **`frontend/DESIGN-rewrite.md` section 5** gives 32 marked tests and a build table of 17, 11, and 4 tests at steps 5, 6, and 7, with the marked count going from 32 to 15 to 4 to 0. The marks file has 36: step 6 takes 13 and step 7 takes 6. aa8c8e60 updated `TEMPLATE-MODEL.md` section 6 and not the design. Decision 0064's text gives the same 32, 17, 11, and 4.
- **`frontend/DESIGN-rewrite.md` section 10 and `TEMPLATE-MODEL.md` section 8** still list RV9 as an item that "reviews" FN-12, INST-1, and the hunks of bdc27d44. RV9 is done. INST-30 and INST-P4 changed in RV9, and RV10-R-1 is open on them. Section 8 names neither.
- **Findings that only a review's prose holds.** Q-0449's three untested forms and the INST Note's EDG reading (section "Text phase 16 wrote that no review has read").
- **Ledger Records that name withdrawn or stale states.** RV8-R-1 and RV8-R-4 name row 4 items that bdc27d44 removed, and RV7-B-3 names "ADD-14, ADD-P6". X25-5's Record says that the front end rejects `union-copy-part-with-no-value.cpp`, and round 27 passes it. F6-1's Record gives PRIM-15's class with no pointer to RV9-6.
- **The ledger header** names the forms RVn-A-n, RVn-B-n, and RVn-R-n, and RM12-n. It does not name the RV9-n form of the phase 16 gap review or the F6-n rows. RV9-n and RV9-R-n now name points of two different reviews, and a phase 17 review called RV10 would give RV10-n beside the existing RV10-R-n.
- **The request table of `pending.md`** has one line, dated 2026-11-10. Its scope grew from 86 questions and 15 rule items at the request to 94 and 18 now, and the table has no line for the change. Rounds 12 and 13 asked for one.
- **The status page's next steps** name "the 10 rules that changed after RV9 read the tree" as an input. Nine are rules and one a principle, the read of 8d33fcb1 covers them, and the open RV10 rows carry what remains.
- **`tools/frontend_check.py --next`** prints a problem of the takeover list and returns success. Once step 5 adds rows, a wrong row passes the check.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | FE | Other evidence |
|---|---|---|---|---|---|---|---|---|---|
| 1 | foundations/constant-evaluation | 27 / 170 | 12 | 500 | 35 | 241 / 908 | 5.4 | 2 (0) | First, from ninth. All three new questions name the page. CE-22 changed in F6 and in the RV9 repair, and CE-21 and CE-P3 in F6. Q-0449 waits for CE-P3, which fails step 3 of the principle test. Three Q-0449 forms have no test, one with the text against Clang and GCC. Q 9 to 12, X 30 to 35. The highest F of any page |
| 2 | foundations/storage | 29 / 113 | 14 | 351 | 6 | 217 / 1146 | 3.6 | 0 | The most open questions with conversion sequences. STG-6 and STG-P2 changed. Q-0430 and Q-0442 wait in row 4a. Q-0307 is still the one test against all three families |
| 3 | foundations/objects | 12 / 65 | 5 | 211 | 7 | 99 / 443 | 3.0 | 0 | New on the list. OBJ-1 changed in F6 and in the RV9 repair, and OBJ-2 in the repair. RV10-R-3 finds that OBJ-2's new item and UNI-5 give two results for one program, and RV10-R-7 is a doubt on OBJ-1. Q-0447 is new. X 5 to 7, F 193 to 211. X24-1 asks OBJ-3 for a statement's own scope |
| 4 | templates/instantiation | 35 / 176 | 8 | 255 | 32 | 330 / 1135 | 7.0 | 11 (11) | INST-1 changed in five commits since phase 14. INST-30 and INST-P4 changed in the RV9 repair, and RV10-R-1 is open on the reach of INST-30's exception. All 11 failures are marked, 13 tests in model section 3 |
| 5 | classes/unions | 28 / 179 | 7 | 268 | 18 | 214 / 981 | 4.7 | 0 | UNI-14 changed in F6 and in the RV9 repair, its third and fourth changes since phase 14. RV9-7 added the rule item "LSUP-10, UNI-14" to row 4, and RV10-R-6 says its record is too wide. UNI-5 meets OBJ-2 in RV10-R-3 |
| 6 | templates/class-template-deduction | 29 / 137 | 11 | 242 | 29 | 288 / 1048 | 9.7 | 4 (0) | First in round 13. No block changed in phase 16. CTAD-16 and CTAD-17 stay rule items with no question. Four unmarked failures (CTAD-1 sites and a constant parameter, CTAD-19) count toward the measure |
| 7 | overloading/conversion-sequences | 47 / 145 | 14 | 271 | 31 | 207 / 1083 | 9.4 | 5 (0) | The most open questions of any page, unchanged. ICS-16 changed in phase 13 and no review since has named it. ICS-45 is a rule item with no question. Four of the five failures are out of scope |
| 8 | templates/dependent-names | 40 / 147 | 8 | 360 | 27 | 283 / 1167 | 6.7 | 12 (12) | No block changed. Q-0427 and Q-0432 wait for uses of DEP-P5 and DEP-P2 for a third phase. G179 is a grammar gap that DEP-26 allows. All 12 failures are marked |
| 9 | statements/jumps | 17 / 69 | 2 | 194 | 4 | 51 / 414 | 5.7 | 0 | New on the list. Six X24 rows name the page (X24-1 to X24-3, X24-5 to X24-7), and F rose from 123 to 194. JMP-6 read literally rejects a `continue` before a later initialized declaration, which both host compilers accept. X24-3 is a host split with no question. Evidence is 12% of the page |
| 10 | expressions/casts | 19 / 64 | 1 | 73 | 1 | 56 / 391 | 4.8 | 0 | New on the list. G166: `cast_type` has no template-id, so `Box<int>{1}` is not derived, and 111 tests stop the new parser there. G167 (bases and member initializers, 75 tests) and G168 (`using namespace`, 59 tests) are the next two |

Classes/operators, expressions/additive, and declarations/functions/definitions left the list. OPR-8 changed a third time in F6, and RV9-6 moved its Q-0063 case into row 4. ADD-14 did not change. P4-1 is answered, and FN-12's read agrees with one small doubt (RV10-R-2). Overload resolution keeps the highest R/P (11.0) for an eleventh round.

## 2. Ranked work items

| # | Item | Needs first | Evidence | Size |
|---|---|---|---|---|
| 1 | **Answer the 15 open rows and the review's readings with no row** (F7), with matrix probes of the X24 host-only claims first | ledger rows for the readings | X24-1 to X24-8, RV10-R-1 to R-3, R-5 to R-8, Q-0449's three forms | one writer session with probes |
| 2 | **Close the grammar gaps G166 to G179** in the Syntax segments (SY) | nothing | 14 open front-end rows. 438 parse disagreements, 215 of them in tests that need a tree | one writer session |
| 3 | **Review the text that items 1 and 2 write, and the old gaps** (RV10), with the points kept against, then the repairs, then a recorded read of the repair diff | items 1 and 2 | ICS-16 for a fifth phase, four questions that wait for a use, six rule items against with no question | one source-reader session in two parts, one writer session, one short source-reader pass |
| 4 | **Repair the records that drift** (R11) | nothing | Section 1, "Records that drift" | under one session |
| 5 | **Keep the rewrite's records to the marks file** as step 3 starts | the phase decision | 36 marks against 32 in the design. The takeover check does not fail | the implementer's item. The records are under one session |
| 6 | **Route the rule items and the six "against" questions, and date the change of the request's scope** | nothing | 18 rule items with no question, one more in each of the last five phases. The six are unchanged since round 8 | under one session |
| 7 | **A narrow second review of the doubt-held questions** | item 3, so the review reads current text | 44 questions held by doubt alone, 19 last read on 2026-11-03. Decisions 0058, 0059, 0061, 0062, 0063, and 0064 deferred it | one source-reader session in two or three parts, one writer session |

### 2.1 Answer the 15 open rows and the readings with no row

The order puts new rows and probes first, then the rows where Tidal's text disagrees with itself, then the rows that say a rule is unclear, then examples and records. Each answer sets the row to `commit`, `text`, `question`, or `out of scope`.

| Rank | Rows | Work | Size |
|---|---|---|---|
| 1 | three new rows | Rows for Q-0449's three untested forms, and tests of each with the overrides the matrix gives. The form where CE-22 and the review allow the copy and Clang and GCC reject needs a question or a row 4 item | small |
| 2 | X24-1 to X24-4, X24-6, X24-7 | Probe the host-only claims of X24 on the matrix: the order in which a `goto` out of nested loops ends the statements' variables (`d1.cpp`), the `continue` before a later declaration (`k1.cpp`), a `goto` to a missing label in a template that is never instantiated (`t1.cpp`), `-Wreturn-type` after a `goto` out of a `while (true)` loop (`f1.cpp`), and the label and `break` forms of `l1.cpp` and `l2.cpp` | small |
| 3 | RV10-R-3 | OBJ-2's item gains the condition for a scalar union member named directly, which UNI-5 and UNI-14 make alive again, with a test of that form | small |
| 4 | X24-1, X24-2, X24-3 | OBJ-3 covers a statement's own scope and gives the order of its variables against the body's objects, and LOOP-3 says "after any jump". JMP-6 names the jumps it covers. JMP-14 or DEP-30 gives the class of a `goto` to a missing label in a templated context, and a question holds the host split if the matrix confirms it | medium |
| 5 | RV10-R-1, RV10-R-2, RV10-R-5, RV10-R-7 | INST-30 says what holds for a default template argument, a `noexcept` operand, and a default member initializer, with a Note or tests. FN-12 names a nested class defined after its enclosing class. CPT-28 states its reach for a variable template and `if constexpr`. OBJ-1's row names the default constructor. Each is small and can be `text` with a Note | small |
| 6 | X24-4, X24-5, X24-6, X24-7 | FN-7 excludes an inner `break` and names a `goto`. A glossary entry for "label". JMP-7 and JMP-14 name the member functions of a local class, as SW-3 does. An example and a test of one label name in a lambda body and its enclosing function | small |
| 7 | RV10-R-6, RV10-R-8 | The LSUP-10 Note and the row 4 item add "in the same evaluation". The OPR Note names `_ld` only | trivial |
| 8 | X24-8 | `text`: outside knowledge, and no rule rests on it | trivial |

### 2.2 Close the grammar gaps G166 to G179

The rewrite's parser follows the Syntax segments, and 14 forms that tests use are not derived. The old front end parses each from general knowledge, so no earlier round saw the gap. Each row is a gap in specification text, and the writer answers it. G166 (`cast_type` with a template-id, 111 tests), G167 (a base or member initializer as a template-id or `decltype`, 75 tests), and G168 (`using namespace` with an unqualified name, 59 tests) hold most of the 215 disagreements in tests that need a tree. G175 (`throw()`) touches Q-0077 in row 4b, and G179 touches Q-0226. A Syntax change is text, so item 3 reads it, and the rewrite's parse count measures it with no parser change beyond a function per new production. The 223 reject disagreements are not findings (report RW1 section 4) and need no text.

### 2.3 Review the text items 1 and 2 write, and the old gaps

The review has two parts.

- Part A reads the text that items 1 and 2 write, with the tests they add. It also reads ICS-16, unread since phase 12, and Q-0425's dated note. For Q-0424, Q-0427, Q-0432, and Q-0435, Part A reads UNI-P3, DEP-P5, DEP-P2, and OPR-P3 as uses.
- Part B reads the points kept against with a row 4 rule item and no question (RV8-B-5 on CTAD-17, RV8-B-6 on CTAD-16, RV6-A-4 and RV7-B-4 on UNW-3, RV6-A-12 on LSUP-11, RV6-B-2 on DEP-15 and DEP-P4, and RV9-7 on LSUP-10 and UNI-14) and says for each whether a question should hold it. Rounds 11 to 13 asked for this part, and decisions 0062 to 0064 left it out.

The review's points take an ID that no earlier review used. The read of its repair diff follows as its own part, as in phase 16.

### 2.4 Repair the records that drift

- A follow-up commit states what the phase 16 messages missed: the six files that 8d33fcb1 shares with b310bf11, the read of 8d33fcb1 for the 10 blocks the close lists, and report 27's split of 17 missed rejects and 18 unsupported.
- `DESIGN-rewrite.md` section 5 and decision 0064's successor give the marks by the marks file (36: 17 at step 5, 13 at step 6, 6 at step 7), or name the file and the tool line and give no count.
- `DESIGN-rewrite.md` section 10 and `TEMPLATE-MODEL.md` section 8 replace the RV9 row with RV10-R-1 (INST-30, INST-1, INST-P4) and RV10-R-2 (FN-12).
- The Records of RV8-R-1, RV8-R-4, RV7-B-3, X25-5, and F6-1 name the row or round that later changed their item (RV9-R-1, RV9-R-2, RV8-R-9, round 27, RV9-6).
- The ledger header names the RV9-n form and the F6-n rows, and gives the form for the next review's points.
- `frontend_check.py --next` fails on a problem of the takeover list, and on a test that is both marked and owned.

### 2.5 Keep the rewrite's records to the marks file

Item 5 belongs to the implementer's view, and the editor ranks only its records. The marks file grew from 32 to 36 in one round, while the design and decision 0064 kept 32. Step 3 adds the shadow run, which the tool does not print yet. Before step 5 adds the first takeover row, the takeover check fails on a bad row (section 2.4). Item 2 raises the parse count with no parser work, so the round report that measures step 3 says which parse disagreements item 2 closed.

### 2.6 Route the rule items and the six "against" questions

- The 18 row 4a rule items each get a question, or the packet keeps them as rule items. LSUP-10 and UNI-14 joined in phase 16. Item 3's Part B gives a reading for six of the items.
- The six "against" questions outside row 4 (Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) join row 4a or 4b by the family table, or the packet says why each stays out. Rounds 9 to 13 asked for this, and phase 16 moved Q-0063 only as part of RV9.
- The request table gains a dated line for the change of scope, from 86 questions and 15 rule items at the request to 94 and 18.
- The close records the maintainer's answer to the request of 2026-11-10, or the lack of one, as the phase 13 to 16 closes did.

### 2.7 A narrow second review of the doubt-held questions

Rounds 8 to 13 proposed this item, and decisions 0058, 0059, 0061, 0062, 0063, and 0064 deferred it. The count went from 43 to 44, and 19 of the 44 were last read in the 2026-11-03 cluster reviews. The prompt reads one question per section, lists its tests, and asks for a verdict on a fixed scale (supports; supports, with the missing step named; reads against; silent). A named step becomes a repair, and a "silent" verdict moves the question to row 10. Running the review after item 3 keeps it on current text. The rewrite does not wait on it, so the item can run beside item 5.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phases 10 to 16 added no page |
| Adopting P22's design (X24) | Keep as a design | X24 is a Propose report. Its findings X24-1 to X24-7 are about current text and go in item 1. The prototype waits for the rewrite's step 3 node point |
| Rewrite work on the old front end's marked tests | None | The marks file gives the 36 tests to the rewrite (decision 0064) |
| A further Propose report | Defer | Each adds text for item 3 to read while items 1 and 2 add text too |
| Implement I8 and a shared impact scanner | Defer | The implementer's time goes to the rewrite |
| Splitting the overloading principles | Keep off the list, as decisions 0059 and 0061 to 0064 left it | R/P is unchanged (11.0, 9.4). The failures on those pages are front-end rows or out of scope |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md`. X24-8 names later work and asks for no rule |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0064 say | 33 *(referenced)* markers name them |

## 4. Process changes

- **A reading with no test gets a ledger row.** A review or read that decides a form no test holds gives the form an ID, and the record commit adds a row. Q-0449's three forms sit only in a question's prose.
- **A message that applies a diff onto a later head lists the files that overlap.** The integrator runs a script that names the files the commit shares with the commits between its base and its parent, and the message copies the list. 8d33fcb1 says "no file" where six overlap.
- **A document that counts the marks reads them from the marks file.** The design and the phase decision give the file and the tool line, so a round that adds a mark leaves no stale count. Round 27 added four, and the design kept 32.
- **A block that changes in a third phase running opens a question or a principle check.** INST-1 changed in five commits over three phases, and UNI-14, OPR-8, and STG-P2 in three. Rounds 12 and 13 proposed a form of this, and no decision adopted it.
- **The close names the read that covers each block changed after the review.** The phase 16 close lists the 10 blocks as unread work for the next review, and the read of 8d33fcb1 already read them.
- **Review IDs carry the item that wrote them.** The next gap review and the read of its repair diff use IDs that no earlier review used, and the ledger header states the scheme. RV9-n and RV9-R-n already name points of two reviews.

## 5. Files

The reader's scripts are in `reports/clean-room/roadmap-14/editor-scratch/`: the round 13 scripts `count.py`, `pages.py`, `diff.py`, `when.py`, `q.py`, `groups.py`, `ledgers.py`, `ledger_steps.py`, `hunks.py`, `repair_words.py`, `rows_1_4a_10.py`, `divtags.py`, `undone.py`, `fe.py`, `replay.sh`, and `replay_status.sh`, with their paths changed; `named.py` (the phase 16 review files that name each block); `tests_lines.py` (the `:tests:` entries each commit adds and removes); `overlap.sh` (the files a commit shares with the commits between its stated base and its parent); and `fe_table.py` (the statuses in `conformance.tsv` of the tests that a round report's table names). They ran on copies made with `git archive` in `/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r14e/`: the tree at b73a7d2d (`t13/`), at f77bbe05 (`t14/`), at 1cc1696e (`rv9a/`), and at 8d33fcb1 (`rv9b/`). The reader also ran `tools/check_specification.py`, `tools/question_index.py --check` and `--share`, and `tools/waits_change.py` on the worktree, which change no file.

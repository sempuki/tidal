# Roadmap, round 15: what to do after phase 17, from an editor's view

Date: 2026-11-11. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 14 report with the scripts beside it. In `documentation/` it read the status page, the roadmap's phase 16 and 17 parts, decisions 0064 and 0065 in full, and `decisions/pending.md`. It did not reread decisions 0060 to 0063. It read `git log fb0bcf87..HEAD` with every commit body (19 commits) and the `--stat` of each. In `reports/clean-room/` it read `ledger.md`, `rust-frontend/ledger.md`, `rust-frontend/takeover.tsv`, `rust-frontend/rewrite-marks.tsv`, `rust-frontend/conformance.tsv` (as data), the X26 report, and `rust-frontend/report-rw2.md`, `report-28.md`, and `report-rw3.md`. In `reports/review/` it read the two files dated 2026-11-11 that phase 17 added: the gap review of the phase 17 text (RV10) and the read of the RV10 repair diff. In `reports/similarity/` it read the result line of the 13 phase 17 reports. In `frontend/` it read the opening and sections 5, 7, and 10 of `DESIGN-rewrite.md`, the section 8 row and the counts of `TEMPLATE-MODEL.md`, and the takeover lines of `README.md`, and no source file. In `tools/` it read the takeover and marks parts of `frontend_check.py` and `check_all.sh`. It read the question files and the rule blocks that the counts below name. It did not read the other round 15 reports, `inputs/`, or `build/exercise-keys/`. One script output listed some lines that 5f100f69 removed from files under `frontend/next/src/`. The reader then excluded `frontend/next/` and `frontend/src/` from that script, and nothing in this report rests on those lines. It did not run `tools/run_tests.py` or `tools/frontend_check.py`, so the front-end counts come from report 28, report RW3, `conformance.tsv`, and the commit messages. Read-only scripts made the counts from copies of the trees at fb0bcf87 (round 14), e3cb5a3c (the tree RV10 read), 88d14ea2 (the RV10 repairs), and 7a849708, made with `git archive` and the round 14 method. Commit read: `7a849708`.

The page table uses the round 14 columns.

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.
- **FE**: tests in the page's test directory that fail their check at round 28 (`conformance.tsv`), with the marked tests in parentheses. A "+n" counts the RV10 tests of 88d14ea2 that the old front end does not pass and that no round has taken.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 14 (fb0bcf87) | Now (7a849708) |
|---|---|---|
| Live rules | 1685, plus 15 withdrawn | 1685, plus 15 withdrawn. No rule is new, and no rule changed class |
| Principles | 343 | 343 |
| Rule and principle text changed in the phase | 14 rules and 4 principles | 23 rules and 3 principles, leaving out `:tests:`, `:rules:`, and `:exceptions:` lines. 31 changes in three commits: GR part 1 (1c223631) 3, F7 (b511838d) 19, and the RV10 repair (88d14ea2) 9. ATTR-2, CE-3, JMP-7, JMP-14, and OBJ-2 changed in both F7 and the repair. Statements holds 9 of the 26, declarations 7, foundations 5, templates 4, expressions 1 |
| Syntax nonterminals | 257 | 262. GR added `base_type`, `using_enum_decl`, and `exception_spec`; the RV10 repair added `decltype_expr` and `new_ptr` |
| Test files | 7073. 6907 cited by a rule, 166 program tests | 7148. 6970 cited by a rule, 178 program tests. 75 new, none removed, no expectation of an older test changed |
| Rules that rest on one test | 0 | 0 |
| `check_specification.py` | 1700 rules, 0 problems | 1700 rules, 0 problems |
| Tests with an override | 755 | 768 (GR 1, F7 10, RV10 repair 2) |
| Overrides by label | EDG 326, GCC 289, Clang 132, Clang 18 83, GCC 14 59, other 44 | EDG 330, GCC 298, Clang 134, Clang 18 84, GCC 14 59, other 46 |
| Tests with an override for exactly one newest family | 557 (Clang 107, GCC 214, EDG 236) | 565 (Clang 107, GCC 220, EDG 238) |
| Tests with an override for all three newest families | 1 (`replace-base-old-pointer-constant.cpp`, Q-0307) | 1, the same test |
| Tests with `tidal-error` patterns (this method) | 3197. The status page gave 3201 | 3223. The status page gives 3227 |
| Questions | 449 | 452 |
| Open questions (all provisional) | 200 | 202 |
| Settled, compiler defect | 236, 13 | 237 (Q-0450), 13 |
| Share that `pending.md` holds (`question_index.py --share`) | Rows 1 to 9: 122 of 200 (61%). With row 10: 137 (68%) | Rows 1 to 9: 123 of 202 (61%). With row 10: 138 (68%) |
| `pending.md` rows decided by the maintainer | none of 10. The request of 2026-11-10 had no answer | none of 10. The request still has no answer |
| Rows 1, 4a, and 10 | 94 questions and 18 rule items | 95 questions (61, 20, and 17, less three that two rows share) and 19 rule items. The sentence above the request gives 95 and 19 |
| Marks file (`rewrite-marks.tsv`) | 36 tests | 38 tests. Round 28 added 2 to model section 3 (now 15). No marked test passes |
| Takeover list (`takeover.tsv`) | no row | 1327 rows, all build step 3, all naming 5f100f69 |
| Old front end (close commit) | 6967 of 7073 pass, 6990 pass their check | 7049 of 7148 pass, 7069 pass their check. `conformance.tsv` (round 28) gives 7039 of 7131 and 7058 |
| Completion line of decision 0060 | 74 tests, 36 marked; 0 of 166 program tests; 9 out of scope | 70 tests, 38 marked and 32 toward the gate's measure; 1 of 178 program tests; 9 out of scope. Round 28 ended at 64 |
| Old front end on page examples | 191 of 193, 2 unsupported | 191 of 193, 2 unsupported |
| The rewrite (`--next`) | 88 unit tests; lex 7191 of 7191; parse 6475 of 6913; 0 owned | 126 unit tests; lex 7266 of 7266; parse 6828 of 6987; shadow step 3 plain 1602 of 1897; 1327 owned |
| Evidence lines, whole specification | 8643 of 45646 (18.9%) | 8839 of 45954 (19.2%) |
| `tests/RESULTS.md` | 7073 rows | 7148 rows |
| `question_index.py --check` | 449 questions, 0 problems | 452 questions, 0 problems |
| Divergence tags that disagree with the question file | 0 | 0 |
| Exercise ledger | 1129 rows: 1030 commit, 15 open, 0 planned, 48 text, 33 question, 3 out of scope | 1164 rows: 1054 commit, 13 open, 0 planned, 54 text, 40 question, 3 out of scope |
| Front-end ledger | 180 rows: 157 commit, 14 open | 183 rows: 172 commit, 2 open (G182, G183), 4 text, 1 question, 4 out of scope. Every `commit` row names a commit, and every named hash exists |

The pages grew by 308 lines, and Evidence grew by 196 of them. The `:tests:` lines gained 101 entries: GR 9 on 8 rules, T6 (3bb416e8) 27 on 15 rules, F7 38 on 19 rules, and the RV10 repair 27 on 13 rules. No commit removed a `:tests:` entry, so the hand merges of b511838d and 88d14ea2 dropped no test.

### The read-only checks

- `tools/check_specification.py`: "1700 rules, 0 problems", no warnings.
- `tools/question_index.py --check`: "452 questions, 0 problems", no warnings.
- `tools/question_index.py --share`: "Rows 1 to 9 hold 123 of the 202 open questions (61%). With row 10, the rows hold 138 (68%)." The line matches the close.
- `tools/waits_change.py 87d7219e HEAD` gives the four lines that the close copies. The same tool from fb0bcf87 gives the same four lines.
- A replay of `check_specification.py` and `question_index.py --check` on a `git archive` copy of each of the 19 phase 17 commits gives these results. `question_index.py --check` passes at all 19. `check_specification.py` gives 0 problems at 8 commits and one problem at the other 11, the stale-number check of the status page. A rerun of `status_table.py` on each of those 11 copies changes only the Similarity cells and the line that names the latest full similarity report, which a copy with no git history cannot order. So no phase 17 commit left either check failing.
- `tools/propose_check.py --strict` on the X20 to X26 reports gives 0 conflicts for each. It lists 12 of X24's 33 rows and 2 of X26's 39 rows as changed since their reports' commits (section "Records that drift").
- The 13 phase 17 similarity reports each report "0 flagged". 5d1a26bf gives a "0 flagged" result for the two `frontend/` documents with no saved report.

### What phase 17 did to the questions

Each count comes from each file's status and `**Waits for:**` line at fb0bcf87 and at 7a849708.

- 3 new questions. Q-0450 (DTY-3) opened in 1c223631 and settled in 80a70210. Q-0451 (JMP-14, JMP-7, DEP-30) waits for a review (doubt) and a principle, JMP-P1. Q-0452 (OBJ-3) waits for newest families that agree.
- 1 settled (Q-0450), none reopened.
- Open questions went from 200 to 202.
- 191 of the 200 questions open at both ends did not change at all. Q-0020, Q-0021, Q-0077, Q-0123, Q-0225, Q-0226, Q-0406, Q-0447, and Q-0449 gained notes or Opinions entries, and none changed its `**Waits for:**` line.
- Q-0451 entered `pending.md` row 4 in 18efd5eb and left it in 88d14ea2, when its answer changed. The close's line "no line that names a pending.md row changed in count" holds at the two ends.

The `**Waits for:**` items, at the start and the end of the phase (a question can hold more than one item).

| Item | Start | End |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 68 | 68 |
| of which name row 4 | 44 | 44 |
| of which hold the plain item "the maintainer (pending.md row 4)" | 40 | 40 |
| newest families that agree (row 1's group) | 60 | 61 (Q-0452) |
| a review (doubt) | 59 | 60 (Q-0451) |
| a review (silent) | 17 | 17 |
| a review (against) | 8 | 8 |
| a review (part) | 1 | 1 |
| a use of a principle | 4 | 4, the same four |
| a principle | 9 | 10 (Q-0451, JMP-P1) |
| a rule, child questions | 1 each | 1 each |

When each question goes in the first group that fits (maintainer, then row 1's group, then a use, a first review, against, silent, doubt, part), the 202 split this way.

| What holds the question | Round 14 | Now |
|---|---|---|
| A row of `pending.md` (rows 2 to 9) | 68 | 68 |
| Two newest families differ (row 1's group) | 54 | 55 (Q-0452 joined) |
| A review that reads with doubt | 44 | 45 (Q-0451 joined) |
| A review that finds the reference silent (packet row 10) | 15 | 15 |
| A review that reads against the answer, outside row 4 | 6 (Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) | 6, the same |
| A use of a principle | 4 (Q-0424, Q-0427, Q-0432, Q-0435) | 4, the same |
| A principle | 7 | 7, the same |
| Child questions (Q-0294), part of a review | 1 each | 1 each |

Of the 45 questions held by doubt alone, 19 were last read on 2026-11-03, 3 on 2026-11-04, 16 on 2026-11-05, and 7 on 2026-11-11. The six "against" questions were last read on 2026-11-04 (four) and 2026-11-05 (two). No item routed them, the eighth round running.

Q-0424, Q-0427, and Q-0432 have waited for a use of a principle since phase 14, and Q-0435 since phase 15. Neither RV10 nor the read of its repair diff names UNI-P3, DEP-P5, DEP-P2, or OPR-P3.

Q-0451 waits for "a principle (JMP-P1)". Its opening note says that no principle gives the answer, and the read of the RV10 repair diff says that the item "fits". The question does not say what sentence JMP-P1 needs.

### Text phase 17 wrote that no review has read

The RV10 gap review read the tree at e3cb5a3c. It covers every production and paragraph of GR part 1 (1c223631) with ATTR-19, IF-7, and DTY-3, the 16 rules, 3 principles, and glossary entry that F7 (b511838d) changed, and Q-0450 to Q-0452. After e3cb5a3c, `specification/` changed in 18efd5eb (one divergence line), 80a70210 (the same line), and 88d14ea2. The read of the RV10 repair diff (7f61141b) read every hunk of 88d14ea2 under `specification/` and the question records of 18efd5eb, 80a70210, and 88d14ea2. So every block that phase 17 changed has a recorded read. The read left these points, each a ledger row.

| Point | Block | Kind | Ledger |
|---|---|---|---|
| RV11-R-1 | TPL-4 | adds a condition. The sentence on two template heads in a class also rejects a friend declaration with two heads that the reference allows. No test holds the friend form | open |
| RV11-R-5 | OBJ-2, UNI-13 | doubt. The trivial default constructor must also not be deleted, and a scalar ended inside a living class member gets the plain reading. No test | open |
| RV11-R-7 | `template_arg`, TARG-16 | gap. The `braced_init` arm derives a braced list for a scalar parameter, every compiler accepts, and no rule names it | open |
| RV11-R-2 | casts paragraph, CTAD-1 | doubt. The paragraph leaves out the alias template that CTAD-14 deduces | open |
| RV11-R-3 | ATTR-19 | doubt. The table has no row for an attribute before a using-directive | open |
| RV11-R-8 | AUTO-11 | record. The place before `::` has no test | open |
| RV11-R-4 | `new_ptr` | the known split of Q-0123 | question |
| RV11-R-6 | JMP-14 | doubt. A discarded statement of a used template, held by Q-0451 | question |

These texts changed in phase 17 and no review names them.

- The 11 rule tests and 27 `:tests:` entries of T6 on 15 rules (3bb416e8), with the NAM-21 test of G181. Every compiler agrees on each, and the tests encode readings of LIST-12, CPT-26, CE-16, LAM-5, and TPL-11 that no review has checked.
- 18efd5eb's removal of the "open" tag from Q-0450's divergence line in `expressions/decltype.md` while Q-0450 stayed open. 80a70210 then wrote "settled on DTY-P2".

Older text also stays unread. ICS-16 changed in phase 13, and RV6, RV7, RV8, RV9, and RV10 each left it out. The last review that names it is the phase 12 gap review.

Points kept against a review with the compilers' answer and no question grew by one. RV10-6 added the rule item "TARG-16" to row 4. The others are RV9-7 (LSUP-10, UNI-14), RV8-B-5 (CTAD-17), RV8-B-6 (CTAD-16), RV6-A-4 and RV7-B-4 (UNW-3), RV6-A-12 (LSUP-11), and RV6-B-2 (DEP-15, DEP-P4). Rounds 11 to 14 asked a review to say for each whether a question should hold it, and no review has.

Some blocks keep changing. INST-1 changed in six commits over four phases (1130ceb8, b5d925e0, 232ac618, 4431852f, 8d33fcb1, b511838d). FN-12 changed in three phases (4431852f, 8d33fcb1, b511838d). OBJ-1, OBJ-2, and CE-22 changed in three commits over phases 16 and 17. DEP-30 changed a third time in 88d14ea2. JMP-7, JMP-14, CE-3, ATTR-2, and OBJ-2 changed twice within phase 17, in F7 and again in the repair of the review that read F7. Each read after the last change agrees or leaves a small doubt, and OBJ-2 carries an open point (RV11-R-5).

### What phase 17 took from round 14

| Round 14 item | Outcome |
|---|---|
| 1. Answer the 15 open rows and the review's readings with no row (F7), with matrix probes of the X24 host-only claims first | Done. F7 answered 11 rows as `commit` (e3cb5a3c), X24-3 as `question` (Q-0451), and X24-8 as `text`, and sent X24-4, RV10-R-1, and F7-1 to item FE, which answered them (7695e9e0). Q-0449's three forms got tests in b511838d and no rows. F7-2 is open for a Q-0447 table row with no test |
| 2. Close the grammar gaps G166 to G179 (GR) | Done. 1c223631 wrote the productions and 924fd6ce the parser functions. cb40b18b set the 14 rows and G181 to `commit`. G182 (a union partial specialization) is a grammar gap that GR found and left open |
| 3. Review the text that items 1 and 2 write, and the old gaps (RV10), with the points kept against, then the repairs, then a recorded read | Done in part. RV10 read GR, F7, and three questions, and the read of 88d14ea2 ran as part 3. Part B (whether a question should hold each point kept against) did not run, the fourth round that asked for it. ICS-16 and the four questions that wait for a use were not read |
| 4. Repair the records that drift (R11) | Done in part. 5d1a26bf corrected the phase 16 messages, the marks counts of `DESIGN-rewrite.md`, the RV9 row of section 10 and `TEMPLATE-MODEL.md` section 8, and the ledger header's RV9-n form. 5f100f69 made `frontend_check.py --next` fail on a takeover-list problem. No commit changed the Records of RV8-R-1, RV8-R-4, RV7-B-3, X25-5, or F6-1, although the roadmap marks R11 done with "stale ledger Records". The tool has no check for a test that is both marked and owned |
| 5. Keep the rewrite's records to the marks file | Done. `DESIGN-rewrite.md` section 5 and `TEMPLATE-MODEL.md` give 38 (17, 15, and 6), as the marks file holds |
| 6. Route the rule items and the six "against" questions, and date the change of the request's scope | Not done. The rule items went from 18 to 19. The six stay outside row 4. The request table has one line |
| 7. A narrow second review of the doubt-held questions | Not done. Decision 0065 defers it, the seventh deferral. 45 questions are held by doubt alone |
| Process: a reading with no test gets a ledger row | Not adopted as a choice. It held in fact for the read's points: RV11-R-1, R-5, and R-8 name forms with no test. F7 gave Q-0449's three forms tests in place of rows |
| Process: a message that applies a diff onto a later head lists the files that overlap | Not adopted. Six messages name their base and their target. 88d14ea2 names two merged files where eight overlap |
| Process: a document that counts the marks reads them from the marks file | Held in fact. Section 5 names the file as the list and dates its count to round 28 |
| Process: a block that changes in a third phase running opens a question or a principle check | Not adopted. INST-1 and FN-12 changed again |
| Process: the close names the read that covers each block changed after the review | Held. The close names 7f61141b for the nine rules of 88d14ea2 |
| Process: review IDs carry the item that wrote them | Not adopted. The phase 17 review used RV10-n beside the phase 16 read's RV10-R-n, and its read used RV11-R-n |

### Decisions 0064 and 0065's standing choices in phase 17

**The old front end stays the front end of record until the rewrite owns a test (0064).** The condition ended at 5f100f69, when the rewrite took 1327 tests. The status page and `frontend/README.md` say that the old front end stays the front end of record for every other test. Decision 0065 says that ownership moves test by test and does not state the rest. Round 28 kept "0 worse". No round fixed a marked test in the old code: each of the 36 starting marks still fails, and round 28 added two. Round 28 changed `interp/expr.rs` and `interp/mod.rs`, which the rewrite did not move, so the rule on moved files did not apply. c0461b86 says that build step 3 "copies them at this round's versions", and report RW3 says that step 3 wrote a new machine and took no change of c0461b86.

**The gate's count becomes a measure (0064).** Held. Report 28 gives 28 with the 36 starting marks and 26 with the 38 at the end, and names a cause for each. Its split (14 missed rejects, 11 unsupported, 1 false reject, 2 EDG-AGREES) matches `conformance.tsv`.

**A review summary lands with its question records (0064).** Held. 18efd5eb holds the RV10 summary with eight question records, and 7f61141b holds the read of 88d14ea2 with twelve. The settlement of Q-0450 went to its own commit (80a70210), because the settlements table names a commit and a commit cannot name its own hash. Its row names 18efd5eb, the commit of the reading. `question_index.py --check` passes at every phase 17 commit.

**Every commit message gives the result of both checks (0064).** Held in 19 of 19 messages.

**A "left open" line becomes a ledger row in the same commit (0064).** Held in part. 3bb416e8 added G181, 924fd6ce G182, b511838d F7-2, and c0461b86 G183, each in its own commit. 5f100f69 says that "X24's 19 prototype tests have no directory yet", and no row holds them. The rest of build step 3 (lambdas, structured bindings, linking, 122 reject checks) is the build's remainder, and the status page lists it.

**The close's Waits-for lines come from `tools/waits_change.py` (0064).** Held. The four lines of 7a849708 match the tool's output for 87d7219e to HEAD.

**Grammar text comes before parser code (0065).** Held. 1c223631 wrote the productions and 924fd6ce their functions. 88d14ea2 wrote `decltype_expr` and `new_ptr`, and 5f100f69 their functions. By the messages, the nonterminal unit test failed and `frontend_check.py --next` exited 1 at six commits (1c223631, 3a7e991c, 3bb416e8, 88d14ea2, 73ed0807, 7f61141b). No other check runs `--next`, so no check failed.

**Ownership moves test by test (0065).** Held in the list. Each of the 1327 rows names an existing test once, build step 3, and 5f100f69. No row is marked. Each test passes on the old front end (1285 `pass`, 36 `EDG-ONLY`, 6 `EDG-ALLOWED`). `frontend_check.py --next` fails on an owned test that the new front end does not pass. The default run of the tool does not read the list, and `tools/check_all.sh` runs the tool without `--next`. So a change that breaks an owned test on the new front end passes `check_all.sh`, and the "worse status" of decision 0065 shows only in a run by hand.

**Decision 0065's close.** Every step ran: the full matrix with `--report`, a whole-repository similarity report, a report of the four exercise reports, `frontend_check.py --next`, `status_table.py`, the roadmap rows, the Waits-for lines, the share line, the rules that changed after RV10 read the tree, and the status line.

**Decision 0063's choices.** An item with a review ended after the read of its repair diff: 7f61141b precedes the roadmap's "Done" for RV10.

**Decision 0062's choices.** One completion count held. Report RW3, the close, and the status page copy the same line. Every `commit` row names its commit, in both ledgers.

**Decision 0059's choices.** A `planned` row named a live item at every commit, and the close has no planned row. Every review point has a row: RV10 19, RV11-R 8. Seven commits were built from an earlier head and applied onto a later one (3bb416e8, 924fd6ce, b511838d, c0461b86, 18efd5eb, 88d14ea2, 5f100f69). A script that lists lines one phase 17 commit adds and a later one removes finds only repairs of the same text, generated counts and tables, ledger Records, the pending-row change of Q-0451, and the marks counts of `DESIGN-rewrite.md`. No commit dropped another item's work.

### Phase 17 commit messages against their diffs

The reader compared each of the 19 commit bodies with its `--stat`, the ledgers at each commit, the marks and takeover files, the questions, and the rule blocks at each commit.

**Statements that agree with the diffs**

- The test counts. GR adds 9 run tests, one with an override. T6 adds 24: 11 rule tests (1 reject, 10 run), 12 program tests, and the G181 test. F7 adds 25 (14 reject, 7 run, 4 accept), 10 with an override. 88d14ea2 adds 17 (8 reject, 4 run, 5 accept) and changes 6 headers. The total of 75 and the count of 7191 to 7266 `.cpp` files match the close.
- The `:tests:` lines. T6's "14 rules ... 26 entries" holds, with NAM-21's entry for G181 beside it. 88d14ea2 adds entries to exactly the 13 rules its message names, for its 17 new tests and the 5 jump tests that now cite DEP-30.
- The rule changes. GR changed ATTR-19, IF-7, and DTY-3. F7 changed the 16 rules and 3 principles it names. 88d14ea2 changed the 9 rules it names, and the close lists the same 9.
- The ledger moves. 87d7219e sets 15 rows to planned. 3a7e991c adds X26-1 to X26-6. 3bb416e8 adds G181, 924fd6ce G182, and c0461b86 G183. cb40b18b sets G166 to G181 to `commit`. b511838d sets X24-3 to `question`, X24-8 to `text`, and adds F7-1 and F7-2. e3cb5a3c and 7695e9e0 set 14 rows to `commit`. 18efd5eb adds RV10-1 to RV10-19. 88d14ea2 sets 4 to `question` and 5 to `text`, and 73ed0807 sets 10 to `commit`. 7f61141b adds RV11-R-1 to R-8. The close's 13 open rows and 2 open G rows are right.
- The marks file. c0461b86 adds the two rows in section 3.
- The takeover. 5f100f69 adds 1327 rows with "RW2" in the commit cell, and 6aa5a345 sets all 1327 to 5f100f69.
- The open-question count and the share line in each message, the three new questions, the settlement of Q-0450, and every `**Waits for:**` change in the close.

**Statements that misstate something or leave something out, with no commit that corrects them**

- 88d14ea2 says that the integrator applied part 2 onto 80a70210 "with no conflict (pending.md and the ledger merged)". The commit shares eight files with the commits after its base (e3cb5a3c): `pending.md`, `ledger.md`, `status.md`, `questions/README.md`, `expressions/decltype.md`, and Q-0021, Q-0123, Q-0449, and Q-0451. A replay finds no line of 18efd5eb or 80a70210 lost. 5d1a26bf corrected the same kind of statement in 8d33fcb1.
- c0461b86 says that build step 3 copies `interp/expr.rs` and `interp/mod.rs` at round 28's versions. Report RW3 says that step 3 copied no file of `frontend/src/interp/` and took no change of c0461b86. 5f100f69's message does not say so.
- 18efd5eb changes `expressions/decltype.md`, removing the "open" tag of Q-0450's line, and its message lists no change under `specification/`.
- The close says that round 28 ended at 64 and that the RV10 repairs "then added 7 tests that it does not pass". 64 and 7 make 71, and the line gives 70. One of the seven (`template-keyword-base-last-name.cpp`) passes its check and stops in the run.
- The roadmap's R11 row reads "Done: ... stale ledger Records", and the close counts R11 done. No phase 17 commit changes the five Records that round 14 named.
- 5f100f69 says that the owned tests' "status now comes from the new front end". That holds only for a run with `--next`. `conformance.tsv` and the default run still give the old front end's status for those tests.
- b511838d files Q-0449's three forms under RV10-R-4, whose Record is a different doubt, on the copy assignment.
- Report RW3 section 7 says "No row from G190 is needed". The front-end ledger ends at G183.

### Records that drift

These records disagree with the tree at 7a849708, or with each other.

- **`frontend/DESIGN-rewrite.md` section 7** says that the variables of a statement's own scope wait for X24-1, and its table marks OBJ-3, LOOP-3, and RFOR-4 "waits for X24-1". F7 answered X24-1 in b511838d. The same table gives DEP-30 as "unchanged" with the labeled jumps checked at a template's definition. 88d14ea2 put the jump conditions on DEP-30's list, so a template that no specialization uses needs no diagnostic. Report RW3 worked from cb40b18b, before both commits.
- **X24's and X26's change tables.** `propose_check.py --strict` lists 12 of X24's rows (JMP-P1, JMP-7, JMP-14, LOOP-3, OBJ-3, FN-7, ATTR-P1, JMP-P2, JMP-6, JMP-9, RFOR-4, DEP-30) and 2 of X26's (LOOP-3, DEP-30) as changed since their reports' commits. The P22 prototype reads X24's table, and no refresh note exists.
- **`frontend/DESIGN-rewrite.md` section 10 and `TEMPLATE-MODEL.md` section 8** name RV10-R-1 and RV10-R-3 as open. Both rows are `commit` (7695e9e0, e3cb5a3c). Section 8 does not name F7's changes to FN-12, INST-1, and INST-30, or RV10-16.
- **`frontend/DESIGN-rewrite.md` section 10** keeps the first-error target open at 77% (report 25). The takeover asks each owned reject test to name a cited or linked rule first, and report RW3 gives 2177 of 2805 for the old front end.
- **`frontend/DESIGN-rewrite.md` section 5**: the row for model section 4 has its last cell separator after the fourth test name, so two tests sit outside the table cell (5d1a26bf).
- **`takeover.tsv`'s header** says that the commit cell "RW2" stands for the commit of item RW2. Every row names 5f100f69.
- **Ledger Records that name withdrawn or stale states.** RV8-R-1 and RV8-R-4 name row 4 items that bdc27d44 removed, RV7-B-3 names "ADD-14, ADD-P6", X25-5 says that the front end rejects a test it passes, and F6-1 gives PRIM-15's class with no pointer to RV9-6. X24-3's Record says that JMP-14 checks its conditions in a template that no specialization uses, which 88d14ea2 changed.
- **The ledger header's review IDs.** RV10-n names points of the phase 17 review, and RV10-R-n names points of the phase 16 read. RV11-R-n names the read of RV10's repair, so a phase 18 review called RV11 would give RV11-n beside it.
- **The request table of `pending.md`** has one line, dated 2026-11-10. Its scope grew from 86 questions and 15 rule items at the request to 95 and 19 now, and the table has no line for the change. Rounds 12, 13, and 14 asked for one.
- **The takeover check** runs only with `--next`. `check_all.sh` and CI run no `--next`, and CI runs no front-end check.
- **X24's prototype tests.** X24 section 2 names 17 new and 2 changed tests for the prototype. `reports/clean-room/X24/tests/` does not exist, and no ledger row holds the gap.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | FE | Other evidence |
|---|---|---|---|---|---|---|---|---|---|
| 1 | statements/jumps | 17 / 81 | 3 | 200 | 8 | 81 / 463 | 5.7 | 0 | First, from ninth. JMP-7 and JMP-14 changed in F7 and again in the RV10 repair. Q-0451's answer changed within the phase, and the question entered row 4 and left it. RV11-R-6: JMP-14 picks one of two readings for a discarded statement. X24-3's Record states text that 88d14ea2 replaced, and X24's table has 12 changed rows. T 69 to 81, X 4 to 8 |
| 2 | foundations/objects | 12 / 72 | 6 | 215 | 9 | 135 / 488 | 3.0 | 0 | OBJ-1 and OBJ-2 changed in three commits over two phases, OBJ-2 twice in phase 17. RV11-R-5 and F7-2 are open forms with no test. Q-0447 is held by doubt, and Q-0452 is new (GCC 16 and EDG differ) |
| 3 | templates/declarations | 49 / 143 | 5 | 149 | 12 | 110 / 949 | 7.0 | 2 (0) +1 | New on the list. TPL-4's new sentence rejects a friend form that the reference allows (RV11-R-1), and no test holds that form. The old front end misses the reject of `two-template-heads-in-class.cpp`. Evidence is 12% of the page |
| 4 | foundations/constant-evaluation | 27 / 176 | 12 | 514 | 38 | 258 / 930 | 5.4 | 0 | CE-3 changed twice in the phase, and CE-22 a third time since phase 16. The highest F of any page. Q-0449 waits for a review, newest families, and CE-P3. X 35 to 38 |
| 5 | templates/arguments | 27 / 100 | 4 | 128 | 6 | 102 / 688 | 5.4 | 4 (1) | New on the list. RV10-6 reads TARG-16 and the `braced_init` arm against, and row 4a gained the rule item "TARG-16" with no question. RV11-R-7: the scalar case has a production and a compiler result and no rule |
| 6 | templates/dependent-names | 40 / 155 | 9 | 373 | 30 | 299 / 1188 | 6.7 | 12 (12) +1 | DEP-30 changed a third time since phase 14. Q-0451 joined the page, and RV10-10 reads Q-0226 against again. Q-0427 and Q-0432 wait for uses for a fourth phase. `template-keyword-base-last-name.cpp` stops in the run |
| 7 | templates/instantiation | 35 / 178 | 8 | 264 | 32 | 339 / 1146 | 7.0 | 12 (12) | INST-1 changed a sixth time since phase 14, and INST-30 changed. RV10-16 reads Q-0406 against again. Model section 3 holds 15 marked tests, 2 of them new |
| 8 | declarations/attributes | 20 / 96 | 4 | 107 | 20 | 130 / 534 | 5.0 | 0 +1 | New on the list. ATTR-2 changed in F7 and in the RV10 repair. RV11-R-3: ATTR-19's table has no row for an attribute before a using-directive. Q-0021 gained two rows in the phase, and Q-0123 now names `new_ptr` |
| 9 | overloading/conversion-sequences | 47 / 147 | 14 | 274 | 31 | 207 / 1083 | 9.4 | 5 (0) | The most open questions with storage, unchanged. ICS-16 changed in phase 13, and five reviews since left it out. ICS-45 is a rule item with no question. Four of the five failures are out of scope |
| 10 | declarations/auto | 22 / 69 | 0 | 53 | 0 | 14 / 415 | 4.4 | 0 +2 | New on the list. AUTO-11 gained an item with four places in the RV10 repair, and one place has no test (RV11-R-8). The old front end misses two of its new rejects. Evidence is 3.4% of the page, the lowest share of any page |

Foundations/storage, classes/unions, templates/class-template-deduction, and expressions/casts left the list. No block of the first three changed in phase 17, and their questions and rule items stay as round 14 gave them. GR closed G166, so `cast_type` derives the forms that 111 tests use, and RV11-R-2 is a small doubt. Overload resolution keeps the highest R/P (11.0) for a twelfth round.

## 2. Ranked work items

| # | Item | Needs first | Evidence | Size |
|---|---|---|---|---|
| 1 | **Answer the 13 open rows and the two front-end text rows** (F8) | nothing | X26-1 to X26-6, RV11-R-1 to R-3, R-5, R-7, R-8, F7-2, G182, G183 | one writer session with probes |
| 2 | **Hold the takeover in every check** | nothing | `check_all.sh` runs no `--next`. 1327 owned tests | under one session |
| 3 | **Review the text that item 1 writes, and the old gaps** (RV12), with the points kept against, then the repairs, then a recorded read of the repair diff | item 1 | ICS-16 for a sixth phase, four questions that wait for a use, eight points kept against with no question, T6's unread tests | one source-reader session in two parts, one writer session, one short source-reader pass |
| 4 | **Repair the records that drift** (R12) | nothing | Section 1, "Records that drift" | under one session |
| 5 | **Refresh X24 and X26 to the current text, and give X24's prototype tests a directory** | item 1 for X26-1 and X26-2 | 12 and 2 changed rows. 19 prototype tests with no directory | under one session |
| 6 | **Route the rule items and the six "against" questions, and date the change of the request's scope** | nothing | 19 rule items with no question, one more in each of the last six phases. The six unchanged since round 8 | under one session |
| 7 | **A narrow second review of the doubt-held questions** | item 3, so the review reads current text | 45 questions held by doubt alone, 19 last read on 2026-11-03. Decisions 0058, 0059, 0061, 0062, 0063, 0064, and 0065 deferred it | one source-reader session in two or three parts, one writer session |

### 2.1 Answer the 13 open rows and the two front-end text rows

The order puts the rows where a rule rejects a valid program or a rule is wrong first, then grammar gaps, then the rows that say a rule is unclear, then examples and records. Each answer sets the row to `commit`, `text`, `question`, or `out of scope`.

| Rank | Rows | Work | Size |
|---|---|---|---|
| 1 | RV11-R-1 | TPL-4 limits its sentence to a member declaration, with a run test of the friend form with two template heads and its matrix result | small |
| 2 | X26-1 | UNEV-P2 and UNEV-2 agree with COMMA-1 and its test on when a discarded class temporary ends, with a probe of the destruction order | small |
| 3 | G182 | `union_def` takes a template-id, as `class_def` does, with the parser function after it (decision 0065, "Grammar text comes before parser code") | small |
| 4 | G183 | LIST-4 says whether an object of another class that converts to the member's class keeps its braces, with the tests that round 28 named | small |
| 5 | RV11-R-5, F7-2, RV11-R-8 | OBJ-2 and UNI-13 add "not deleted", with a test. A test of the Q-0447 row that F7-2 names. A test of AUTO-11's place before `::` | small |
| 6 | RV11-R-7 | TARG-16 or a TARG Note names the braced list for a scalar parameter, and the row 4 item "TARG-16" follows | small |
| 7 | X26-2, X26-3, X26-4 | LOOP-3 says what becomes of the iteration expression's value. COMMA-5, UNEV-3, and STMT-4 share a term for an operand with no effect, or a Note says that each warning is the implementation's. A glossary entry for discarded-value expression, with a link from the comma page | small |
| 8 | RV11-R-2, RV11-R-3, X26-5 | The casts paragraph names the alias template (CTAD-14) and the dependent qualifier. ATTR-19 gains a row for the using-directive. A PRIM-28 example of a fold that calls an overloaded comma | trivial |
| 9 | X26-6 | `text`: outside knowledge, and no rule rests on it | trivial |

### 2.2 Hold the takeover in every check

Decision 0065 makes a failing owned test a worse status. The tool checks that only under `--next`, and `check_all.sh` runs the tool without it. Either `check_all.sh` runs `frontend_check.py --next` when cargo is present, or the default run reads `takeover.tsv` and reports an owned test whose new status is not passing. The tool also reports a test that is both marked and owned, which round 14 asked for. The phase decision states the front end of record for the tests that the takeover list does not name, as the status page and `frontend/README.md` do now. The rest of build step 3 is the implementer's item. Its round report gives the shadow count with the owned count, and names each owned test whose old status and new status differ.

### 2.3 Review the text item 1 writes, and the old gaps

The review has two parts.

- Part A reads the text that item 1 writes, with its tests. It also reads ICS-16, unread since phase 12, and the T6 rule tests of 3bb416e8 against the rules they cite. For Q-0424, Q-0427, Q-0432, and Q-0435, Part A reads UNI-P3, DEP-P5, DEP-P2, and OPR-P3 as uses.
- Part B reads the points kept against with a row 4 rule item and no question (RV10-6 on TARG-16, RV9-7 on LSUP-10 and UNI-14, RV8-B-5 on CTAD-17, RV8-B-6 on CTAD-16, RV6-A-4 and RV7-B-4 on UNW-3, RV6-A-12 on LSUP-11, and RV6-B-2 on DEP-15 and DEP-P4) and says for each whether a question should hold it. Rounds 11 to 14 asked for this part, and decisions 0062 to 0065 left it out.

The review and the read of its repair diff share one number that no earlier review used (section 4). The read follows as its own part, as in phases 16 and 17.

### 2.4 Repair the records that drift

- A follow-up commit states what the phase 17 messages missed: the eight files that 88d14ea2 shares with the commits after its base, the step 3 machine that took no change of c0461b86, the `decltype.md` change of 18efd5eb, the 64 and 6 of the close, and R11's Records that no commit changed.
- `DESIGN-rewrite.md` section 7 gives X24-1 as answered (b511838d) and DEP-30's current list. Section 10 and `TEMPLATE-MODEL.md` section 8 drop RV10-R-1 and RV10-R-3 and name the F7 changes to FN-12, INST-1, and INST-30. Section 10 gives the takeover's first-error condition in place of the 77% target. Section 5's table row for model section 4 gets its cell separator back.
- `takeover.tsv`'s header names the commit of each row.
- The Records of RV8-R-1, RV8-R-4, RV7-B-3, X25-5, F6-1, and X24-3 name the row, round, or commit that later changed their item (RV9-R-1, RV9-R-2, RV8-R-9, round 27, RV9-6, 88d14ea2).
- The ledger header gives the ID scheme of section 4 for the next review and its read.

### 2.5 Refresh X24 and X26, and give X24's prototype tests a directory

X24 is the design that the P22 prototype measures, and 12 of its 33 rows rest on text that F7 and RV10 changed. X26's LOOP-3 and DEP-30 rows changed too, and X26-1 and X26-2 may change COMMA-1, UNEV-2, and LOOP-3 again. A refresh note per design states each changed row's effect, and `propose_check.py --strict` then reports 0 changed rows for both. X24's 17 new and 2 changed tests go into `reports/clean-room/X24/tests/`, or a ledger row holds the gap. Decisions 0064 and 0065 deferred refresh notes for the done designs. The editor ranks only these two, because a prototype reads one of them.

### 2.6 Route the rule items and the six "against" questions

- The 19 row 4a rule items each get a question, or the packet keeps them as rule items. TARG-16 joined in phase 17. Item 3's Part B gives a reading for eight of the items.
- The six "against" questions outside row 4 (Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) join row 4a or 4b by the family table, or the packet says why each stays out. Rounds 9 to 14 asked for this.
- The request table gains a dated line for the change of scope, from 86 questions and 15 rule items at the request to 95 and 19.
- The close records the maintainer's answer to the request of 2026-11-10, or the lack of one, as the phase 13 to 17 closes did.

### 2.7 A narrow second review of the doubt-held questions

Rounds 8 to 14 proposed this item, and decisions 0058, 0059, 0061, 0062, 0063, 0064, and 0065 deferred it. The count went from 44 to 45, and 19 of the 45 were last read in the 2026-11-03 cluster reviews. The prompt reads one question per section, lists its tests, and asks for a verdict on a fixed scale (supports; supports, with the missing step named; reads against; silent). A named step becomes a repair, and a "silent" verdict moves the question to row 10. Running the review after item 3 keeps it on current text. The rewrite does not wait on it.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phases 10 to 17 added no page |
| Adopting P26's design (X26) | Keep as a design | X26 is a Propose report. Its findings X26-1 to X26-6 are about current text and go in item 1 |
| A Propose report on P30 | Defer | P30 is ready, and X24's refresh (item 5) comes first, since P30 builds on the same labels |
| Rewrite work on the old front end's marked tests | None | The marks file gives the 38 tests to the rewrite (decision 0064) |
| Implement I8 and a shared impact scanner | Defer | The implementer's time goes to the rest of build step 3 |
| Splitting the overloading principles | Keep off the list, as decisions 0059 and 0061 to 0065 left it | R/P is unchanged (11.0, 9.4). The failures on those pages are front-end rows or out of scope |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md`. X26-6 names outside knowledge and asks for no rule |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0065 say | 33 *(referenced)* markers name them |

## 4. Process changes

- **The default check reads the takeover list.** `check_all.sh` runs the takeover check, so an owned test that the new front end fails stops a push. Today only a run with `--next` sees it.
- **A message that applies a diff onto a later head lists the files that overlap.** The integrator runs `overlap.sh` (in the scratch directory) on the stated base and the commit, and the message copies the list. 8d33fcb1 and 88d14ea2 each named fewer files than overlap.
- **A roadmap row marked "Done" names the parts that did not happen.** R11's row says "stale ledger Records", and no commit changed them.
- **The close runs `propose_check.py --strict` on the done designs** and lists the rows that the phase changed. X24 has 12 such rows, and the P22 prototype reads its table.
- **A review and the read of its repair diff share one number.** The phase 18 review uses RV12-n and its read RV12-R-n, and no later review reuses a number. RV10-n and RV10-R-n now name points of two different items, and RV11-R-n names the read of RV10.
- **A message that states another item's later work names that item, and that item's message confirms or corrects it.** c0461b86 said that step 3 copies two `interp/` files, and 5f100f69 copied neither and does not say so.
- **A block that changes twice in one phase, or in a third phase running, opens a principle check.** JMP-7, JMP-14, CE-3, ATTR-2, and OBJ-2 changed twice in phase 17, and INST-1 and FN-12 changed in a fourth and third phase. Rounds 12 to 14 proposed a form of this, and no decision adopted it.

## 5. Files

The reader's scripts are in `reports/clean-room/roadmap-15/editor-scratch/`: the round 14 scripts `count.py`, `pages.py`, `diff.py`, `when.py`, `q.py`, `groups.py`, `ledgers.py`, `ledger_steps.py`, `hunks.py`, `repair_words.py`, `rows_1_4a_10.py`, `divtags.py`, `undone.py`, `fe.py`, `fe_table.py`, `tests_lines.py`, `overlap.sh`, `replay.sh`, and `replay_status.sh`, with their paths changed, `inputs/` left out of each copy, and `frontend/next/` and `frontend/src/` left out of `undone.py`; `tests_rules.py` (the rules whose `:tests:` line one commit changes); `newtests.py` (the new and changed tests of each commit, with outcome and override); `marks_status.py` (the statuses of the marked tests at two commits); and `takeover.py` (the checks of the takeover list against the tree, the marks file, and `conformance.tsv`). They ran on copies made with `git archive` in `/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r15e/`: the trees at fb0bcf87, e3cb5a3c, 88d14ea2, and 7a849708. The reader also ran `tools/check_specification.py`, `tools/question_index.py --check` and `--share`, `tools/waits_change.py`, and `tools/propose_check.py --strict` on the worktree, which change no file.

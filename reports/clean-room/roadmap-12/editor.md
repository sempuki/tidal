# Roadmap, round 12: what to do after phase 14, from an editor's view

Date: 2026-11-11. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 11 report with the scripts beside it. In `documentation/` it read the status page, the roadmap's phase 13 and phase 14 parts, decisions 0060 and 0062, the Decision part of 0061, and `decisions/pending.md`. It read `git log 45ee5e85..bd87b27c` with every commit body (14 commits), the `--stat` of each, and the body of f940ea53, which corrected the phase 13 records before phase 14 began. In `reports/clean-room/` it read `ledger.md`, `rust-frontend/ledger.md`, the I7 and P3 reports, `rust-frontend/report-25.md`, and one row of `roadmap-11/writer-probes/README.md`. In `reports/review/` it read the two phase 14 gap reviews (`2026-11-11-gap-review-phase-14-*`), and in `reports/similarity/` the result line of the six phase 14 reports. It read the question files and the rule blocks that the counts below name. It did not read the other round 12 reports, `inputs/`, `build/exercise-keys/`, or the `frontend/` sources. It did not run `tools/run_tests.py` or `tools/frontend_check.py`, so the front-end counts come from report 25, `conformance.tsv`, and the commit messages. Read-only scripts made the counts from copies of the trees at 45ee5e85 (round 11) and bd87b27c, made with `git archive`, with the round 11 method. Commit read: `bd87b27c`.

The page table uses the round 11 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.
- **FE**: tests in the page's test directory that fail their check at round 25 (report 25, section 7).

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 11 (45ee5e85) | Now (bd87b27c) |
|---|---|---|
| Live rules | 1683, plus 15 withdrawn | 1685, plus 15 withdrawn. DEP-40 and LOOP-9 are new and required. No rule changed class |
| Principles | 343 | 343 |
| Rule and principle text changed in the phase | 34 rules and 8 principles | 27 rules and 11 principles, plus the 2 new rules, leaving out `:tests:`, `:rules:`, and `:exceptions:` lines. Templates hold 24 of the 40, foundations 5, classes 4, statements 4, declarations 2, expressions 1. Dependent-names alone holds 14, instantiation 7 |
| Test files | 6881. 6755 cited by a rule, 126 program tests | 6947. 6809 cited by a rule, 138 program tests. 66 new, none removed, no expectation of an older test changed |
| Rules that rest on one test | 0 | 0 |
| `check_specification.py` | 1698 rules, 0 problems | 1700 rules, 0 problems, 0 warnings |
| Tests with an override | 712 | 721 |
| Overrides by label | EDG 299, GCC 272, Clang 132, Clang 18 80, GCC 14 57, other 44 | EDG 304, GCC 274, Clang 132, Clang 18 82, GCC 14 57, other 44 |
| Tests with an override for exactly one newest family | 525 (Clang 107, GCC 203, EDG 215) | 532 (Clang 107, GCC 205, EDG 220) |
| Tests with an override for all three newest families | 1 (`replace-base-old-pointer-constant.cpp`, Q-0307) | 1, the same test |
| Tests with `tidal-error` patterns (this method) | 3128. The status page gave 3132 | 3145. The status page gives 3149 |
| Questions | 428 | 434 |
| Open questions (all provisional) | 180 | 185 |
| Settled, compiler defect | 235, 13 | 236, 13 |
| Share that `pending.md` holds (`question_index.py --share`) | Rows 1 to 9: 113 of 180 (63%). With row 10: 127 (71%) | Rows 1 to 9: 114 of 185 (62%). With row 10: 129 (70%) |
| `pending.md` rows decided by the maintainer | none of 10 | none of 10. The request of 2026-11-10 has no answer |
| Row 4a rule items with no question | 15 | 16. ADD-14 with ADD-P6 is new (RV7-B-3) |
| Front end (close commit) | 6721 of 6881 pass, 6745 pass their check | 6804 of 6947 pass, 6831 pass their check. `conformance.tsv` (round 25) gives 6801 of 6938 and 6825 |
| Completion line of decision 0060 | 127 tests fail their check, 2 of 126 program tests do not run, 9 out of scope | 107 tests, 3 of 138 program tests, 9 out of scope. Round 25 ended at 104 tests and 1 of 133 |
| Front end on page examples | 179 of 181, 2 unsupported | 184 of 186, 2 unsupported (round 25) |
| Evidence lines, whole specification | 8074 of 44408 (18.2%) | 8229 of 44811 (18.4%) |
| `tests/RESULTS.md` | 6881 rows | 6947 rows |
| `question_index.py --check` | 428 questions, 0 problems | 434 questions, 0 problems |
| Divergences tags that disagree with the question file | 0 | 0 |
| Exercise ledger | 988 rows: 920 commit, 26 open, 27 text, 12 question, 3 out of scope | 1044 rows: 960 commit, 26 open, 0 planned, 33 text, 22 question, 3 out of scope |
| Front-end ledger | 163 rows, 2 open (G161, G162), 4 out of scope. 10 `commit` rows named no commit | 165 rows, 2 open (G164, G165), 4 out of scope. Every `commit` row names a commit, and every named hash exists |

The pages grew by 403 lines, and Evidence grew by 155 of them. Phase 14 changed 40 blocks, against 42 in phase 13. F4 changed 33 of them and the RV7 repair 20, with 13 blocks in both.

### The read-only checks

- `tools/check_specification.py`: "1700 rules, 0 problems", no warnings.
- `tools/question_index.py --check`: "434 questions, 0 problems", no warnings.
- `tools/question_index.py --share`: "Rows 1 to 9 hold 114 of the 185 open questions (62%). With row 10, the rows hold 129 (70%)." The line matches the packet and the close.
- A replay of `check_specification.py` and `question_index.py --check` on a `git archive` copy of each of the 14 phase 14 commits gives 0 problems from `question_index.py` at every commit. `check_specification.py` gives 0 problems at 3 commits (9359cc42, d0597264, 411ef68e) and one problem at the other 11, the stale-number check of the status page. A rerun of `status_table.py` on each of those 11 copies changes only the Similarity cells and the line that names the latest full similarity report. The tool orders similarity reports by the time git first recorded them, and a copy has no git history. The replay finds no commit whose checks failed, apart from that one check that a copy cannot run.
- The six phase 14 similarity reports (`2026-11-11-phase-14-rp`, `-t3`, `-f4`, `-rv7`, `-rv7-repairs`, `-close`) each report "0 flagged". No similarity output exists for the I7, P3, or round 25 reports (section "Records that drift").

### What phase 14 did to the questions

Each count comes from each file's status and `**Waits for:**` line at 45ee5e85 and at bd87b27c.

- 6 new questions. Q-0429 (DED-20, INST-5, PACK-1) waits for a principle, DED-P2, which no phase 14 item changed. Q-0430 (ADD-14, STG-28, STG-29) is in `pending.md` row 4a with two cases. Q-0431 (LSUP-1) waits for a review (silent), so it is in packet row 10. Q-0432 (DEP-4) waits for a use of DEP-P2. Q-0433 (INST-5) and Q-0434 (DEP-2) wait for a review (doubt).
- 1 settled under decision 0054: Q-0422, on JMP-P3 (RV7-A-11). No question reopened.
- Open questions went from 180 to 185.
- 172 of the 179 questions that were open at both ends did not change at all. The seven that changed are Q-0225, Q-0226, Q-0410, Q-0415, Q-0424, Q-0425, and Q-0427. Two of them changed their `**Waits for:**` line: Q-0424 now also waits for a review (doubt), and Q-0427 now waits for a use of DEP-P5.

The `**Waits for:**` items, at the start and the end of the phase (a question can hold more than one item):

| Item | Start | End |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 64 | 65 |
| of which name row 4 (alone or with row 3 or 7) | 40 | 41 |
| of which hold the plain item "the maintainer (pending.md row 4)" | 36 | 37 |
| newest families that agree (row 1's group) | 55 | 55 |
| a review (doubt) | 51 | 53 |
| a review (silent) | 16 | 17 |
| a review (against) | 9 | 9 |
| a review (part) | 1 | 1 |
| a use of a principle | 2 (JMP-P3, UNI-P3) | 3 (UNI-P3, DEP-P5, DEP-P2) |
| a principle | 1 (Q-0196) | 2 (Q-0196, Q-0429) |
| a rule, child questions | 1 each | 1 each |

When each question goes in the first group that fits (maintainer, then row 1's group, then a use, a first review, against, silent, doubt, part), the 185 split this way:

| What holds the question | Round 11 | Now |
|---|---|---|
| A row of `pending.md` (rows 2 to 9) | 64 | 65 (Q-0430 joined) |
| Two newest families differ (row 1's group) | 49 | 49 |
| A review that reads with doubt | 41 | 42. Q-0427 left, Q-0433 and Q-0434 joined |
| A review that finds the reference silent (packet row 10) | 14 | 15 (Q-0431 joined) |
| A review that reads against the answer, outside row 4 | 7 | 7, the same seven (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) |
| A use of a principle | 2 | 3 (Q-0424, Q-0427, Q-0432). Q-0422 left by settling |
| A principle | 1 | 2 (Q-0196, Q-0429) |
| Child questions (Q-0294), part of a review | 1 each | 1 each |

Of the 42 questions held by doubt alone, 20 were last read on 2026-11-03, 3 on 2026-11-04, 16 on 2026-11-05, and 3 (Q-0415, Q-0433, Q-0434) on 2026-11-11. The seven "against" questions outside row 4 are the same seven as in rounds 8 to 11.

### Text phase 14 wrote that no review has read

RV7's two reviews read the tree at 6afa79f9. They name all 22 blocks that changed after RV6 and all 33 blocks that F4 changed, and each records a read of the RV6 repair diff (9c009448) hunk by hunk. They gave 27 points (RV7-A-1 to A-11, RV7-B-1 to B-16). The repair commit b5d925e0 then changed 20 rules and principles, and no later commit changed rule text. The close lists the same 20. Each row gives the words that b5d925e0 added and removed, the RV7 verdict on the block before the repair, and the other changes to the block in phases 13 and 14:

| Block | Page | Repair | RV7 verdict before the repair | Other changes |
|---|---|---|---|---|
| LOOP-9 | statements/loops | +51 -3 | adds a condition (A-7) | new in F4 |
| REF-2 | declarations/initialization/references | +46 -28 | doubt (A-5) | RV6 repair (9c009448). Changed in both repairs |
| DEP-P6 | templates/dependent-names | +46 -6 | against, the CNS-13 row (B-2) | F4 |
| DED-20 | templates/deduction | +46 | in no table. RV7-B-16 names it in its text | none. Q-0429 |
| INST-2 | templates/instantiation | +35 -3 | adds a condition (B-6) | F4 |
| DEP-30 | templates/dependent-names | +31 | adds a condition (B-5) | F4 |
| DEP-40 | templates/dependent-names | +30 -5 | adds a condition (B-7) | new in F4 |
| UNI-P3 | classes/unions | +20 -3 | doubt (A-3) | RV6 repair. Changed in both repairs. Q-0424 waits for a use of it |
| DEP-P1 | templates/dependent-names | +19 -6 | doubt (B-13) | F4 |
| DEP-P5 | templates/dependent-names | +17 -5 | against (B-1), doubt (B-10) | F3, RV6 repair, F4. Four changes in two phases. Q-0427 waits for a use of it |
| CNS-13 | templates/constraints | +17 -7 | in no table. RV7-B-2 names it in its text | none |
| DTOR-8 | classes/constructors | +16 | doubt (A-1) | RV6 repair. Changed in both repairs |
| DEP-6 | templates/dependent-names | +16 | adds a condition (B-5) | F4 |
| LSUP-6 | foundations/language-support | +14 -7 | adds a condition (A-8) | F4 |
| CNS-P3 | templates/constraints | +14 -3 | in no table, and no point names it | none |
| INST-1 | templates/instantiation | +10 -2 | adds a condition (B-6) | F3, F4 |
| UNI-14 | classes/unions | +8 -2 | doubt (A-4) | F3, RV6 repair. Changed in both repairs |
| INST-P1 | templates/instantiation | +4 -1 | adds a condition (B-6) | F4 |
| DEP-4 | templates/dependent-names | +4 -2 | agrees, with typeid left out (B-12) | F4. Q-0432 |
| STG-P2 | foundations/storage | +2 -1 | against (A-10) | F3, F4. Q-0430 |

Three of the 20 blocks sat in no review table: DED-20, CNS-13, and CNS-P3. No review point names CNS-P3. Dependent-names holds 7 of the 20.

Five blocks changed in the repairs of both RV6 and RV7: REF-2, UNI-P3, DTOR-8, UNI-14, and DEP-P5. RV7 read each of the five with a doubt or against after the RV6 repair. The glossary entry for a direct binding also changed in both repairs. Two consecutive repairs have not closed these six texts.

Text outside the rule blocks also changed after RV7 read the tree, and the close does not list it: the glossary entry for a direct binding, the INST-33 reject example (RV7-B-15), and Evidence notes or divergence lines on 12 pages (dependent-names, storage, references, additive, instantiation, constraints, deduction, unions, new-delete, language-support, copy-move, constructors). ICS-16, which QR2 changed in phase 13, is still in no review: neither RV6 nor RV7 names it.

Three points that read against stay as `text` rows or settled questions with a row 4 item and no open question: RV7-A-2 (CPY-15, which Q-0426 holds), RV7-B-4 (NEW-22 and UNW-3 after a throw), and RV7-B-3 (ADD-14 with ADD-P6, where Q-0049 stays settled). The RV6 points of the same kind (RV6-A-4, RV6-A-12, RV6-B-2) still have no question.

### What phase 14 took from round 11

| Round 11 item | Outcome |
|---|---|
| 1. Answer the 26 open rows and G161 and G162 (F4), with matrix probes of the host-only claims first | Done. F4 reran the open behavior on the matrix with three new grids. 19 rows went to `commit` in F4, P2-9 to `commit` in round 25, 4 to `question`, and I6-2 and P2-7 to `text`. G161 and G162 are `commit` |
| 2. Review the 22 blocks changed after RV6, the text outside them, and item 1's text (RV7), then the repairs, then a recorded read of the repair diff | Done in part. Parts A and B ran, and each records a read of the RV6 repair diff. Part C (whether a question should hold the three RV6 points kept against) did not run. ICS-16 was not read. No file records a read of RV7's own repair diff (b5d925e0), and the roadmap marks the row done |
| 3. Repair the records that drift (R8), with checks for the front-end ledger and the settlements table | Done in part, by f940ea53 and R8. The front-end ledger names a commit in every `commit` row, and `check_specification.py` checks it. The settlements table has no check, and its RV7 row names no commit. The ledger header still describes only the RV4 and RV5 ID forms. Exercise commits still keep no similarity output for the report |
| 4. Give decision 0060 a number for "few" and "stable", and the completion line by page from the tool | Done in part. Decision 0062 sets the gate at 50 tests with no test made worse in two rounds. `frontend_check.py` prints no page table, and report 25 built its page table by hand |
| 5. Keep the packet current, and route the rule items and the seven "against" questions | Not done. The sentence above the request is stale again (86 questions and 15 rule items, against 88 and 16). The rule items grew from 15 to 16. The seven "against" questions stayed outside row 4 for a fifth round |
| 6. A narrow second review of the doubt-held questions | Not done. Decision 0062 defers it, the fourth deferral. 42 questions are held by doubt alone |
| Process: the read of a review's repair diff is its own item | Adopted as a standing choice of decision 0062, inside the RV7 row. The read of RV6's diff ran. The read of RV7's diff did not |
| Process: the close lists all text changed after the review | Not adopted. The close lists blocks only |
| Process: the host-only choice goes into the prompts | Adopted (R8). The I7 and P3 reports each have a "Host-only claims" section |
| Process: the front-end ledger has the exercise ledger's checks | Adopted (R8) |
| Process: a generated count stays generated | Not adopted. The sentence above the request and the settlement rows are written by hand |
| Process: a row whose report answers "no change" goes in as `text` in the exercise's commit | Adopted. I7-4 is `text` in 411ef68e |
| Process: decision 0060 states its numbers | Adopted (decision 0062) |
| Process: an exercise report passes similarity on the record | Not adopted. The I7, P3, and round 25 commits say "no similarity run" |

### Decision 0062's standing choices in phase 14

**The gate for an early rewrite.** Held as a test. Report 25, section 7 applies the gate. The count is 104 against the limit of 50, so the gate does not hold. The other two parts hold: no test moved from a passing status to a failing one in round 24 or 25, and each of the 104 tests has a named cause. The close and the status page copy the result.

**A repair diff gets a read.** Held for RV6's diff, which both RV7 reviews read hunk by hunk. Not held for RV7's diff. No file under `reports/review/` names b5d925e0, and the item ended with the roadmap row marked done.

**Every commit row names its commit.** Held. Both ledgers have no `commit` row without a hash, and every named hash is a commit. The `pending.md` settlements table is outside the choice, and its RV7 row names no commit.

**Decision 0061's choices.** One completion count held: report 25, P3, and the close copy the line of `frontend_check.py`. The out-of-scope rows held at 9 tests. First tries became program tests: `r11-expected-first-try` and `r11-signal-first-try` in T3, and `pipeline-first-try` and `events-first-try` in P3. The host-only lists held in I7 and P3. Report 25 ran no compiler.

**Decision 0059's choices.** A `planned` row named a live item at every commit, and the close has no planned row. Each front-end finding got a test in the commit that records it: I7-17 in 411ef68e, P3-7 to P3-10 in 8b10f65f, and I7-16, G164, and G165 name tests that exist. Every RV7 point has a row: 18 `commit`, 6 `question`, 3 `text`. A front-end round ran after the last rule change of the phase. Four commits were built in a worktree from HEAD with one item's files (d0597264, 411ef68e, 4dc57e81, b5d925e0). A script that lists lines one phase 14 commit adds and a later one removes finds only ledger moves, `**Waits for:**` lines, and generated counts, so no worktree commit dropped another item's work. 1130ceb8 and b5d925e0 ran `frontend_check.py` on a tree that held other items' uncommitted work and say so. Their front-end lines (140 and 110 tests) describe trees that no commit holds.

### Phase 14 commit messages against their diffs

The reader compared each of the 14 commit bodies with its `--stat`, the ledgers at each commit, and the rule blocks at each commit. These agree with the diffs:

- The test counts: d0597264 adds 27 (20 tests and 7 program tests), 411ef68e 1, 1130ceb8 19 with 4 changed, b5d925e0 10 with 4 changed, 8b10f65f 9 (4 tests and 5 program tests). The total is 66, as the close says.
- The rule changes: 1130ceb8 changed 33 blocks with DEP-40 and LOOP-9 new, and b5d925e0 changed 20. RP, T3, I7, and P3 changed only `:tests:` lines, generated counts, and grammar marks under `specification/`. The close's list of 20 is the b5d925e0 set.
- The ledger moves: bd97540b 24 open to planned and 2 to text, 6afa79f9 19 rows and G161 and G162 to commit, 87b127cb 18 commit, 6 question, 3 text, 18bb9b3a 3 to commit. The close's 26 open rows are right.
- The open-question count and the share line in each message, the six new questions, the one settlement, and every `**Waits for:**` change in the close except the row 4 count below.
- f4109f3a's hashes: a9c480b7, 47f58739, and afccbb13 each name the rows that the ledger gives them.
- I7's score (290 of 296, 117 tests, 90 grid cells, 14 tests elsewhere) and its 12 programs, of which GCC, Clang, and the front end agree on 9.
- b035efd4's completion line: 104 is 45 missed rejects, 50 unsupported, 4 false rejects, and 5 EDG-AGREES, and 30 rows of the table carry the rewrite mark.

These statements misstate something or leave something out, and no commit corrects them:

- The close gives "the maintainer (pending.md row 4) 36 to 37". That count takes only the plain item. 40 and then 41 questions name row 4, which matches the packet's 19 in row 4a and 22 in row 4b. f940ea53 corrected the same count in the phase 13 close.
- The close, the roadmap row, and decision 0062's status line mark RV7 done, and the row says "then repairs and a read of their diff". No commit or file records a read of b5d925e0's diff. The close itself lists the 20 blocks that "the next review reads".
- The close lists the blocks changed after RV7 and leaves out the glossary entry, the INST-33 example, and the Evidence notes that b5d925e0 also changed.
- b5d925e0 lists the text it changed and leaves out CNS-P3. Its ledger row RV7-B-2 names "the CNS principle row".
- The close says "P3 added the other 3 failing tests and 2 program tests that do not run". The two lines give 3 more tests that fail their check (6831 against 6825 with 9 new) and 2 more program tests that do not run. Report 25 counts its one stuck program test inside the 104, so the two first tries can be inside the 3. Read as written, the sentence counts five tests.
- d0597264 says a test for the ADD-11 doubt "waits for F4's text on typed pointers over bytes". 1130ceb8 does not mention ADD-11, and no ledger row or question holds the doubt.
- 9359cc42 gives a finding "for item F4 or a later phase": the `forward_class` shape allows attributes after the class key, and `elaborated_type` does not. F4 did not take it, and no ledger row or question holds it.

### Records that drift

These records disagree with the tree at bd87b27c, or with each other:

- **RV7's read of the repair diff** is marked done in the roadmap row and in decision 0062's status line ("All items are done"). No record of the read exists. Decision 0062's own standing choice asks for it before the item ends.
- **The sentence above the request in `pending.md`** says rows 1, 4a, and 10 "hold 86 open questions and the 15 rule items of row 4a". The packet now gives 88 questions (55, 19, and 17, less three that two rows share) and 16 rule items. Q-0430 joined row 4a, Q-0431 joined row 10, and ADD-14 with ADD-P6 joined the rule items after the request date. The request table has no line for the change of scope.
- **The settlements table of `pending.md`** has no commit for the RV7 row. Q-0422 settled in 4dc57e81.
- **Findings that only a commit message or a probe README holds.** The ADD-11 doubt (d0597264), the `forward_class` attributes (9359cc42), and a compiler split on an unqualified `d.~Base()` on a derived object, which GCC accepts and Clang and EDG reject (b5d925e0 and the probe README). AGENTS.md asks that a split be recorded as a question. The split falls under DTOR-8, which changed after RV7.
- **G164 names its own answer.** Its Record says that no rule gives the element that a `T*` cast from a byte offset into allocated storage points to. STG-28's second sentence, from 1130ceb8, gives it, and Q-0430 holds the case in row 4a with RV7-A-10 against it. The row does not name Q-0430.
- **Q-0049 is settled while row 4 holds its case.** RV7-B-3 reads ADD-14's one-past move against, and `pending.md` row 4 gained the rule item "ADD-14, ADD-P6" for it. The question file does not record that its case waits on the maintainer.
- **Exercise reports with no similarity record.** The I7, P3, and round 25 reports have no similarity output, and writers wrote 5 tests from the I7 and P3 reductions in the same commits. `similarity.py` checks `specification/`, `guides/`, and `questions/` by default, so "on the default directories" does not cover a report. AGENTS.md asks that a clean-room report pass `tools/similarity.py` before a writer acts on it (decisions 0019, 0029). Round 11 raised the same gap.
- **Decision 0062's deferral list** names "interpreter output when every order agrees" as later work. Round 25 shipped it as P2-9's answer (b035efd4), and neither the roadmap nor the decision records that.
- **The ledger header** describes the ID forms of RV4 and RV5 and does not name those of RV6 or RV7.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | FE | Other evidence |
|---|---|---|---|---|---|---|---|---|---|
| 1 | templates/dependent-names | 40 / 147 | 8 | 340 | 27 | 283 / 1164 | 6.7 | 12 | First for a second round. 14 blocks changed in phase 14, the most of any page, and 7 of the 20 changed after RV7. RV7 read DEP-P5 and DEP-P6 against before the repair. DEP-P5 changed four times in two phases. Q 6 to 8 (Q-0432, Q-0434), X 23 to 27. Q-0427 and Q-0432 wait for uses of DEP-P5 and DEP-P2. 11 of the 12 failures carry the rewrite mark |
| 2 | templates/instantiation | 35 / 163 | 7 | 234 | 30 | 287 / 1061 | 7.0 | 10 | New on the list. 7 blocks changed, 3 after RV7. Q 5 to 7 (Q-0429, Q-0433), X 26 to 30, 16 new tests. RV7-B-6 found the decltype exemption too wide in three blocks. P3-6 asks when a member's return type is made, and P3-7 and P3-8 are front-end faults on the same rule. 9 of the 10 failures carry the rewrite mark |
| 3 | templates/class-template-deduction | 29 / 123 | 7 | 218 | 22 | 209 / 894 | 9.7 | 8 | No rule change in phases 11 to 14. F rose from 84 to 218 with I7, and 14 open I7 rows name its rules. I7-1 to I7-3 rest on wrong assertions against tests. 6 of 7 questions wait on the maintainer |
| 4 | foundations/storage | 29 / 108 | 13 | 306 | 6 | 184 / 1088 | 3.6 | 0 | Q 12 to 13 (Q-0430, row 4a). RV7 read STG-28, STG-29, and STG-P2 against (A-10), and STG-P2 changed again after the review. G164 is open on STG-28. Q-0307 is still the one test against all three families |
| 5 | overloading/conversion-sequences | 47 / 145 | 14 | 266 | 31 | 207 / 1083 | 9.4 | 6 | The most open questions of any page, unchanged. ICS-16 changed in phase 13 and no review has named it. ICS-45 is a rule item with no question, and five of the six failures are ICS-45 tests |
| 6 | expressions/additive | 17 / 67 | 3 | 76 | 9 | 138 / 464 | 2.8 | 0 | New on the list. RV7-B-3 reads ADD-14 against twice: the F4 paragraph (Q-0430) and the older one-past sentence, a rule item with Q-0049 settled. Evidence grew from 119 to 138 lines after the review |
| 7 | classes/unions | 28 / 179 | 7 | 214 | 18 | 204 / 965 | 4.7 | 5 | UNI-P3 and UNI-14 changed in both repairs, and RV7 read both with a doubt. Q-0424 waits for a review and a use of UNI-P3. Q-0425 is in row 1's group |
| 8 | templates/constraints | 40 / 147 | 5 | 191 | 34 | 260 / 1078 | 8.0 | 7 | The most overrides of any page. CNS-13 and CNS-P3 changed after RV7, and no review table names either. Q-0426 is in row 4b |
| 9 | templates/deduction | 39 / 161 | 4 | 334 | 13 | 233 / 1109 | 7.8 | 4 | DED-20 gained 46 words after RV7, outside any review table. Q-0429 waits for DED-P2, which no item writes. All 4 failures carry the rewrite mark |
| 10 | overloading/overload-resolution | 44 / 206 | 10 | 407 | 30 | 205 / 1054 | 11.0 | 2 | The highest R/P for a ninth round. The same 10 questions. P3-10 is a front-end fault on a user `operator=` |

Foundations/constant-evaluation (Q 9, FE 2) and expressions/new-delete (Q 7, FE 4) left the list. Neither changed a question in phase 14, and RV7-B-4 on NEW-22 is a `text` row.

## 2. Ranked work items

| # | Item | Needs first | Evidence | Size |
|---|---|---|---|---|
| 1 | **Answer the 26 open rows and G164 and G165** (F5), with matrix probes of the host-only claims first | a similarity record of the I7, P3, and round 25 reports; the phase decision names the item in each row | I7-1 to I7-3, I7-5 to I7-15, P3-1 to P3-12, G164, G165. 14 of the rows fall on CTAD, whose rules no phase changed after phase 10 | one writer session with probes, two if the P3 examples go in |
| 2 | **Review the 20 blocks changed after RV7, the text outside them, and the text of item 1** (RV8), with a recorded read of the RV7 repair diff, then the repairs, then a read of RV8's repair diff as its own row | item 1 | 20 blocks, 3 of them in no review table. Five blocks and one glossary entry changed in two repairs running. ICS-16 has had no review for two phases. Three questions wait for uses of principles | one source-reader session in three parts, one writer session, one short source-reader pass |
| 3 | **Repair the records that drift** (R9), with checks for review reads, settlement rows, and the sentence above the request | nothing | Section 1, "Records that drift" | under one session |
| 4 | **Put the gate's inputs in the tool**: the completion line by page, a rewrite mark read from a file, and a provisional answer on whether the marked tests count toward "few" | nothing | 104 tests at round 25, 30 of them marked for the rewrite. The page table and the marks are in a report table only | under one session, plus the maintainer's view |
| 5 | **Route the rule items and the seven "against" questions, and date the change of the request's scope** | nothing | 16 rule items with no question, one more each phase. The seven are unchanged since round 8. Q-0049 is settled while row 4 holds its case | under one session |
| 6 | **A narrow second review of the doubt-held questions** | item 2, so the review reads current text | 42 questions held by doubt alone, 20 last read on 2026-11-03. Decisions 0058, 0059, 0061, and 0062 deferred it | one source-reader session in two or three parts, one writer session |

### 2.1 Answer the 26 open rows and G164 and G165

The order puts the record and the probes first, then the rows that rest on wrong assertions, then the rest. Each answer sets the row to `commit`, `text`, `question`, or `out of scope`.

| Rank | Rows | Work | Size |
|---|---|---|---|
| 1 | all | Run `tools/similarity.py` on `reports/clean-room/I7/`, `reports/clean-room/P3/`, and `rust-frontend/report-25.md`, and keep the output under `reports/similarity/` (AGENTS.md, decisions 0019, 0029) | small |
| 2 | P3-3, P3-5, P3-6, P3-11 | Probe the host-only claims on the matrix: a function template declared only and used in `decltype`, the dangling stage under the address sanitizer, a member definition whose return type is a specialization over an incomplete class, and copy-initialization through a converting constructor of a class with deleted copies | small |
| 3 | I7-1, I7-2, I7-3 | CTAD-10 says that a member whose type is not dependent follows LIST-4. CTAD-25 says that the constructor that runs can differ from the one that gave the guide, with the `result-then-initialize` example. CTAD-16 gains a note with DER-8 and CTAD-26, and `inherited-copy-candidate-user-guide` cites CTAD-26 and drops the sentence on the aggregate candidate | medium |
| 4 | I7-5, I7-6, I7-8, I7-9, I7-11, I7-13, I7-14 | Each asks what the text leaves open. Each needs a probe, and a question holds any split: implicit constructors and guides, the initializer-list guide and an empty list, an alias of an alias, the guides a derived class takes, OVL-14 for guides in copy initialization, CTAD-2's forms with no initializer, and when a conditional `explicit` is evaluated | medium |
| 5 | I7-7, I7-10, I7-12, I7-15 | A worked alias example under CTAD-14, a sentence in CTAD-P1 and CTAD-P3 on guides with no template head, "unlike ILC-3" in CTAD-18, and a link from CTAD-9 to the sequence of a braced list | small |
| 6 | P3-6 | INST-5 says that a member function definition's parameter and return types are made with the definition. The front-end tests of P3-7 and P3-8 already expect that reading | small |
| 7 | P3-1, P3-2, P3-3, P3-5 | Informative examples: an iterator over a computed sequence with a separate end type on the range-for page, the delegate form of LAM-P4, a declared-only function in `decltype` with ODR-3, and a member reference to a temporary under OBJ-9. The examples run through `check_examples.py` | medium |
| 8 | P3-4, P3-11, P3-12 | A note on the comparison page that links PRIM-9, a link from the initialization pages to VC-4, and glossary entries for closure type and range | small |
| 9 | G164, G165, and the three findings with no row | G164 becomes `text` on STG-28's second sentence, or `question` with Q-0430. VIRT-13 says whether the warning counts each hidden base function, with a test of one overload overridden. The ADD-11 doubt, the `forward_class` attributes, and the `d.~Base()` split each get a test, a rule change, or a question | small |
| 10 | P3-7 to P3-10 | Front-end round 26 takes the four tests. The rows stay `open` and name their tests until then | front end |

### 2.2 Review the 20 blocks changed after RV7

RV7 read after F4 and read the RV6 repair diff on the record, as round 11 asked. Its own repair diff has no read, so the 20 blocks wait for RV8. The review has three parts:

- Part A reads the 20 blocks of b5d925e0, the three that no review table named first (DED-20, CNS-13, CNS-P3). For the five blocks that both repairs changed (REF-2, UNI-P3, DTOR-8, UNI-14, DEP-P5) and the glossary entry for a direct binding, the verdict says whether a third repair or a question should hold the remaining doubt. Part A also reads the INST-33 reject example, the Evidence notes and divergence lines that b5d925e0 changed on 12 pages, and ICS-16. The read of b5d925e0 goes hunk by hunk, as RV7 read 9c009448.
- Part B reads the text item 1 writes and gives a first reading of each question item 1 opens. It reads UNI-P3, DEP-P5, and DEP-P2 as uses for Q-0424, Q-0427, and Q-0432.
- Part C reads the points kept against a review with no open question (RV6-A-4, RV6-A-12, RV6-B-2, RV7-B-3, RV7-B-4) and says for each whether a question should hold it. Round 11 asked for this part, and decision 0062 left it out.

After the repairs, the short source-reader pass reads only the repair diff. It is its own roadmap row, and its result is a file under `reports/review/`. The close lists every block, Evidence note, example, and glossary entry that changed after that pass.

### 2.3 Repair the records that drift

- The roadmap's RV7 row and decision 0062's status line say that the read of the RV7 repair diff did not run, and point at RV8.
- The settlements table's RV7 row names 4dc57e81. `question_index.py --check` fails on a settlement row with a count and no commit.
- `question_index.py` writes the sentence above the request in `pending.md` (88 questions and 16 rule items now), and `--check` fails when the sentence is stale. The request table gains a line for the change of scope: Q-0430 in row 4a, ADD-14 with ADD-P6 among the rule items, and Q-0431 in row 10.
- A follow-up commit states what the phase 14 messages missed: the close's row 4 count (40 to 41 questions name row 4), the five-test reading of the close's P3 sentence, CNS-P3 in b5d925e0, and the text outside blocks after RV7.
- The ADD-11 doubt, the `forward_class` attributes, and the `d.~Base()` split each get a ledger row, or a question for the split.
- G164's Record names STG-28's second sentence and Q-0430. Q-0049 gains a dated note that row 4 holds its case.
- The roadmap and decision 0062 record that round 25 answered P2-9 with the interpreter's first line.
- The ledger header says that each review gives its own ID form, or names the RV6 and RV7 forms.
- Each exercise commit runs `tools/similarity.py` on the report's directory and keeps the output under `reports/similarity/`.

### 2.4 Put the gate's inputs in the tool

Item 4 belongs to the front-end view, and the editor ranks only its records. The gate of decision 0062 is now testable, and round 25 tested it. These facts bear on the next test:

- 30 of the 104 tests carry the rewrite mark, so they pass only after the rewrite. The current design can take the other 74. The gate needs 54 of those 74 fixed with no new failures. In-scope failures went 167, 149, 127, and 104 over rounds 22 to 25, and phase 14 then added 3 more through P3.
- The rewrite mark and the page table live in report 25's table only. `frontend_check.py` reads the `out of scope` rows from the front-end ledger. A rewrite mark in the same ledger or in a file beside it would let the tool print the line with and without the marked tests, and by page.
- Decision 0062 does not say whether "few" counts the tests that only the rewrite can pass. The phase decision can give a provisional answer and ask the maintainer.
- The 24 tests that stop in a run (`run-unsupported`) pass their check, so the line leaves them out. The tool can print them on the same line.

### 2.5 Route the rule items and the seven "against" questions

- The 16 row 4a rule items each get a question, or the packet keeps them as rule items and the sentence above the request counts them from the tool. The count grew by one in each of the last three phases.
- The seven "against" questions outside row 4 (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) join row 4a or 4b by the family table, or the packet says why each stays out. Rounds 9 to 11 asked for this.
- Q-0049 either reopens and joins row 4 with ADD-14 and ADD-P6, or the row 4 item says why a settled question holds a case that a review reads against.
- The close records the maintainer's answer to the request of 2026-11-10, or the lack of one, as the phase 13 and 14 closes did.

### 2.6 A narrow second review of the doubt-held questions

Rounds 8 to 11 proposed this item, and decisions 0058, 0059, 0061, and 0062 deferred it. The count went from 41 to 42, and 20 of the 42 were last read in the 2026-11-03 cluster reviews. The prompt reads one question per section, lists its tests, and asks for a verdict on a fixed scale (supports; supports, with the missing step named; reads against; silent). A named step becomes a repair, and a "silent" verdict moves the question to row 10. Running it after RV8 keeps it on current text.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phases 10 to 14 added no page |
| The front-end rewrite (FR) | Wait for the gate, or for item 4's answer on the marked tests | Round 25 ended at 104 tests against a limit of 50 |
| Propose X24 on P26 | Defer | It adds text for RV8 to read |
| A lazy generator form (P3-1) | Keep coroutines out. Take P3-1's iterator example on the range-for page | The charter keeps coroutines out of scope (KW-1) |
| Interpreter output when every order agrees | Drop from the deferral list | Round 25 shipped it as P2-9's answer |
| Splitting the overloading principles | Keep off the list, as decisions 0059, 0061, and 0062 left it | R/P is unchanged (11.0, 9.4). The open rows on those pages are missing steps |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md` |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0062 say | 33 *(referenced)* markers name them |

## 4. Process changes

- **The read of a review's repair diff is its own row.** The phase table gives it a row with the repair as its prerequisite, and `check_specification.py` fails when the roadmap marks a review item done and no file under `reports/review/` names the repair commit. Decision 0062's choice inside the RV7 row did not hold.
- **A third doubt on a block that two repairs changed opens a question.** The question takes the case with a probe grid, in place of a third repair. Five blocks and one glossary entry are in that state now.
- **An exercise report passes similarity on the record.** The integrator runs `tools/similarity.py` on the report's directory before a writer acts on it, and keeps the output under `reports/similarity/`. A commit that says "no similarity run" names the report it left out.
- **The close lists all text changed after the review.** Blocks, glossary entries, examples, and Evidence notes each count. A tool option can list them from the review's commit.
- **A finding in a commit message or a probe README gets a ledger row or a question in that commit.** Three findings of phase 14 have neither.
- **A generated count stays generated.** The sentence above the request and the settlement rows come from `question_index.py`, which fails `--check` when they are stale.
- **The close copies its `**Waits for:**` change from a tool.** `question_index.py` can print the change by item between two commits, counting every question that names a row, so the row 4 count stops depending on the method.

## 5. Files

The reader's scripts are in `reports/clean-room/roadmap-12/editor-scratch/`: the round 11 scripts `count.py`, `pages.py`, `repair_words.py`, `groups.py`, and `rows_1_4a_10.py` with their paths changed; `diff.py` (blocks and tests changed between two trees); `when.py` (the commit that changed each block over a range); `q.py` (the question changes, `**Waits for:**` items, and exclusive groups between two trees); `ledgers.py` (the states and moves of both ledgers, `commit` rows with no hash, and hashes that are no commit); `hunks.py` (changed lines of each page, by block, Evidence, or other text); `divtags.py`; `undone.py` (lines that one commit of a range adds and a later one removes); `replay.sh` (the read-only checks at each commit); and `replay_status.sh` (the status-table rerun on a copy). They ran on copies made with `git archive` in `/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r12e/`: the tree at 45ee5e85 (`t11/`), at bd87b27c (`t12/`), at 6afa79f9 (`rv7a/`), at 4dc57e81 (`rv7rec/`), and at b5d925e0 (`rv7b/`). The reader also ran `tools/check_specification.py`, `tools/question_index.py --check` and `--share`, which change no file.

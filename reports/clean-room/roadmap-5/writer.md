# Roadmap, round 5: what to do after phase 7, from a clean-room writer's view

Date: 2026-11-03. Decision 0029, with decisions 0033, 0039, 0044, 0050, 0051, and 0054. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, exercises, specification format, the decisions named above, and `decisions/pending.md`), its own round-4 report `reports/clean-room/roadmap-4/writer.md`, the specification pages, the test headers, `questions/README.md` and the status and dated notes of every question file, the phase 7 reports (`R-overloading/`, `R-foundations/`, `R-exceptions/`, X13 to X16, front-end reports 15 to 17, `conformance.tsv`, `tests/programs/`), the 2026-11-03 gap reviews under `reports/review/`, and `git log 2798b37~1..HEAD` with commit bodies. It did not read the other round-5 reports or `inputs/`. All counts come from read-only `grep`, `git`, and short Python scripts kept outside the repository, and from the read-only runs of `tools/question_index.py --check`, `tools/check_specification.py`, and `tools/propose_check.py`, on the tree at commit 7d8c982. Comparisons with round 4 use the same scripts on commit 3d98c93. The report wrote no repository files.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 6 (3d98c93) | Now (7d8c982) |
|---|---|---|
| Rule entries | 1678 (15 withdrawn), 1663 live | 1691 (15 withdrawn), 1676 live |
| Words under `specification/` | 234,251 | 252,620 |
| Words on the 85 pages, without index and glossary | 227,178 | 245,475 |
| Evidence words on those pages | 53,963 (24%) | 64,937 (26%) |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6032 | 6337, of which 67 are program tests |
| Single-test rules | 3 | 8 |
| Test files with a per-compiler override | 523 (652 override lines) | 606 (763 override lines) |
| Test files with an EDG override | 214 | 251 |
| `undefined` rules with a run witness | 43 of 56 | 54 of 57; the other 3 (ADD-18, OBJ-12, VOL-6) have an Evidence note that no tool stops them |
| Tests with `tidal-interpreter: run-fail` | 4 | 14 |
| Question files | 352: 309 provisional, 1 open, 12 compiler defect, 30 settled | 376: 209 provisional, 0 open, 13 compiler defect, 154 settled |
| Open questions with no gap-review reading | 197 | 14: five process questions, Q-0312, and Q-0369 to Q-0376 |
| *(referenced)* markers | 29 | 32 |
| `tests/RESULTS.md` | 5609 of 6032 tests | 6337 of 6337 tests |
| Front end (`conformance.tsv`) | 5595 of 6032 pass | 6068 of 6337 pass (5929 pass, 139 differ from EDG only); 121 missed rejects, 78 unsupported, 65 run-unsupported, 4 false rejects, 1 wrong result |
| Committed similarity reports | 141 (8.6 MB) | 155 (13 MB); the 14 of phase 7 flag nothing, and 13 of them check every question |
| `question_index.py --check` warnings | 46 | 49 |

Phase 7 added 13 rules (ATTR-20, CINIT-3, CLS-35, CNS-40, COND-16, DEFARG-10, DEFARG-11, DEFARG-12, OBJ-12, ODR-15, PRIM-35, TR-16, UNW-11) and changed the class of 7 (CNS-39, TARG-22, STG-25, OBJ-11, LSUP-5, CTOR-17, DCMP-8). It added 305 tests: 238 rule tests and 67 program tests. It opened 24 questions and settled 124. The open count went from 310 to 209.

### 1.2 What phase 7 took from round 4

| Round 4 item | Phase 7 result |
|---|---|
| R1, close the phase 6 record | Done as item R1 (034f442) and the closing commit 7d8c982. `RESULTS.md` covers every test, similarity checks `questions/` by default, and `status_table.py` writes the front-end row of `exercises.md`. |
| GR2, gap reviews of the pages that never had one | Done as item GR (b6f488d, 7b7667a). Every page now has had a gap review, and every open question except the 14 above has a reading. |
| Q2, settle by divergence | Done in part. Item SR added the settling rule by review and the decision packet. The Divergence column was not built. Five open questions differ only on an older version (Q-0020, Q-0076, Q-0181, Q-0275, Q-0342). |
| E2, reader coverage | Done in part. Item PR ran Predict on the ten pages with no batch (839 of 840). Nobody has re-read the pages that phase 6 consolidated, and Diagnose still covers 4 pages. |
| U, a run check for every `undefined` rule | Done as item UW (78682a0). |
| FE15, front-end rounds | Done as rounds 15 to 17. |
| V, Evidence out of the overloading pages | Left. `conversion-sequences.md` grew from 11,520 to 12,398 words, and its Evidence from 7,197 to 7,936 words (64%). |
| L, test directories that match the pages | Left. Six pages still keep their tests under older names, such as `tests/foundations/overload-resolution` and `tests/expressions/sequencing`. |
| Process 1, check results written by a tool | Not built. Commit bodies still type their counts. No phase 7 commit exists only to correct an earlier message. |
| Process 2 and 3, one front-end binary per round, and status-to-status changes | Not built. See section 1.4. |
| Process 4, questions in every similarity run | Done. |
| Process 5, smaller per-commit similarity reports | Not done. Each phase 7 report checks the whole repository, at about 4,300 lines, for 56,481 lines in all. |
| Process 6, phase close as a checklist step | Done in 7d8c982. |

### 1.3 Where the writer's time went

Phase 7 had 22 commits. Seven were repair commits from reader and reviewer findings (a9906a8, b00cbf2, 573d849, 966aa1b, e0d06f0, 8c15b06, 2eb7c8d). Together they touched page files 107 times and added 189 tests. The other writer commits were item TP (21 pages, 37 tests), item UW (9 pages, 12 tests), item WP (67 program tests), and item SR.

Question files took a large share of the writing. Commit 0637b59 touched 90 question files, e0d06f0 touched 70, and 8c15b06 touched 120. Each settled question records the review, the principle, and the family that still differs, and the page's Evidence names it too.

Evidence took most of the growth in the pages. Page words rose by 18,297, and Evidence by 10,974 of them (60%). The pages that grew most were unwinding (+1,265 words), odr (+1,090), storage (+906), conversion sequences (+878), members (+816), and names (+781). Of the 64,937 Evidence words, 36,305 sit in paragraphs that name a question. The question file holds the same divergence, so the two records repeat each other.

### 1.4 Front-end rounds and the writer

Front-end rounds ran while the writer changed tests, as in phase 6. Report 17 records that the suite grew from 6286 to 6296 tests during the round and that three tests changed their expectation. The integrator wrote the round 15 and 16 baselines from a worktree at the previous commit. Commits e0d06f0 and 8c15b06 left the baseline as it was, because round 17 was running. Report 17 kept a round-16 build in a scratch directory to see changes between failing states, because `frontend_check.py` cannot see them. These are the workarounds that round 4 process items 2 and 3 named.

The front end now finds writer gaps at a steady rate. Findings G118 to G133 (16 findings over three rounds) gave 41 tests and 5 questions in 2eb7c8d. Thirteen of those tests do not pass on the front end yet, and the status page sends them to item FE.

### 1.5 Records that drift

- `documentation/exercises.md` section 6 still lists "W8 and W1" as the next exercises, and the roadmap's "Parallel lanes" section still names W8 and W1 and an override digest as lanes. Every Write suite is done, and phase 7 never ran those lanes.
- The status page's size estimate still counts "about 66 section-units". The specification has 85 pages. The charter names a size estimate on the status page as its third success criterion, so the estimate stays on the page and needs current numbers.
- `question_index.py --check` prints 49 warnings. Each names a question that a review file names and whose Opinions section does not name that review. Decision 0054 asks each commit that acts on a review to record the review's reading on every question it read.
- `propose_check.py` reads 0 rows from `reports/clean-room/X14/report.md`. The X14 table's header says "Rule ID" and "Kind of change", and the tool looks for "ID" and "Change", so it skips the table without a warning. The tool also reports 3 class conflicts. Two of them (CTOR-17 in X4, STG-25 in X9) come from phase 7 class changes.
- Eight rules have one test. Four are new in phase 7 (CINIT-3, UNW-11, OBJ-12, TR-16), and four are older template rules (INST-33, INST-36, TPL-48, DEP-37).

### 1.6 Questions

209 questions are open. Each has a dated note that says why it stays open, and a note can give more than one reason. Counted over the note text:

- 59 say that no principle on the page gives the answer. Most say "no principle on the page gives the answer" (36) or "states the case" (10). They spread over many pages. The largest groups are alignment (5), attributes (4), and new and delete (4).
- 48 mention a review that reads with doubt.
- 30 say that two newest families differ. Row 1 of `pending.md` lists 15 of them, which settle if the maintainer relaxes the rule.
- 29 record a change made on 2026-11-03 and stay open because the rule changed that day. Q-0025 is an example.
- 60 open questions appear somewhere in `pending.md`. Row 4 alone names 21 questions and 11 rules where the matrix agrees and a review reads the C++23 text the other way.

A script over the test notes maps each open question to the compiler families whose override names it. 168 have an override for a newest family: EDG alone 42, EDG and GCC 40, GCC alone 38, Clang alone 25, Clang and GCC 9, Clang and EDG 9, all three 5. Five have overrides only for older versions. 36 have no override. Two questions look ready now. Q-0076 is kept open because no principle gives the answer, yet Clang 22, GCC 16, and EDG 7.0 all reject both forms and TR-3 now says so, which the rule of decision 0051 accepts. Q-0342 waits on item TP, which is done (CLS-35).

### 1.7 Coverage by readers and reviewers

- **Gap reviews.** Every page has had one. No review has read the text that phase 7 wrote, which is the 13 new rules, the 7 class changes, the CLS-35 layout tables, and Q-0369 to Q-0376.
- **Predict.** Every page has had a batch. Item PR read its ten pages on 9f9e62a, before the gap-review repairs. Since then those ten pages have gained 974 lines and lost 282. The gap-review repairs also changed names, objects, and translation, which earlier batches read. The constant expressions page carries the most report mentions (362), and no reader has used it since phase 6 consolidated it.
- **Diagnose.** 4 pages (D-classes). 2916 tests carry `tidal-error` patterns. The pages with the most are constructors (120), names (100), instantiation (89), constant expressions (85), initialization (85), and function declarations (82).
- **Propose.** Every page has had one. The 39 findings of X13 to X16 are acted on.
- **Port.** None.

### 1.8 Similarity

Phase 7 commit bodies record two runs that flagged. The first run on the X13 to X16 reports flagged four 8-word runs of common phrasing, and the integrator reworded them (435a783). The first whole-repository run with questions flagged five question units, and the writer reworded them (034f442). Every flag was reworded before its commit. The source readers checked their summaries outside the repository first, as AGENTS.md asks. Volume is the cost. Each commit's report repeats about 4,300 `ok` lines, and `reports/similarity/` grew from 8.6 MB to 13 MB.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | R2. Close the phase 7 record | 49 index warnings; X14 unread by `propose_check.py`; stale order and lanes; size estimate at 66 units; 8 single-test rules | 0.5 | low | a clean base for the phase 8 decision |
| 2 | RV. A gap review of the text phase 7 wrote | 13 new rules, 7 class changes, CLS-35, 8 questions with no reading | 0.5-1 of repairs | medium | the same review level for new text as for old |
| 3 | Q3. Settle what the current rules allow, with a Divergence column | Q-0076 and Q-0342 ready; 29 questions changed on one day; round 4 Q2 column not built | 0.5-1 | low | a smaller open list without a new rule |
| 4 | PQ. Principles for the questions that no principle answers | 59 open questions say no principle answers them | 1.5-2 | medium | up to about 50 questions can settle under decision 0054 |
| 5 | E3. Predict on the repaired pages and the consolidated pages | 13 pages repaired after their last batch; constant expressions has 362 mentions and no reader since phase 6 | 1-1.5 of repairs | low | a test of the GR repairs and of item PQ's principles by use |
| 6 | V2. Evidence that does not repeat the questions | Evidence took 60% of phase 7 growth; 36,305 Evidence words sit in paragraphs that name a question | 1-1.5 | medium | readable pages; smaller similarity units |
| 7 | FE18. Front-end rounds 18 and later | 121 missed rejects (48 in templates), 78 unsupported, 65 run-unsupported, 13 tests from G118 to G133 | 0.5 per round | low | the regression signal; findings at 5 to 6 per round |
| 8 | D2. Diagnose batches on the pages with the most error patterns | 2916 tests with patterns; 4 pages read | 1-1.5 of repairs | low | `tidal-error` patterns and rule citations checked by readers |
| 9 | L. Test directories that match the pages | left by phase 7; six pages | 0.25 | none | simpler tools and batch scripts |

Total for items 1 to 7: about 6 to 8 writer sessions, plus the source-reader sessions of item 2 and the reader sessions of item 5. Items 8 and 9 add about 1.5.

### Item 1. R2. Close the phase 7 record

- **What.**
  - Record each review's reading in the Opinions section of the 49 questions that `question_index.py --check` warns about. Then make the warning an error, so that the next commit that acts on a review cannot leave one.
  - `propose_check.py` accepts the header names "Rule ID" and "Kind of change", or warns when a section 2 table has no column it can read. Record that the CTOR-17 and STG-25 conflicts come from phase 7 class changes, as the decision 0054 practice for withdrawals asks.
  - Rewrite `exercises.md` section 6 and the roadmap's "Parallel lanes" to state the present. Every Write suite is done.
  - Replace the status page's size estimate with counts that `status_table.py` writes: pages, live rules, and words, against the charter's third criterion.
  - Add a second test to each of the 8 single-test rules.
- **Evidence.** Section 1.5.
- **Cost.** 0.5 session. The second tests are the largest part.
- **Risk.** Low. The Opinions entries restate review readings in the writer's words and pass `similarity.py --fail-on-flag`.
- **Unblocks.** The phase 8 decision starts from records that tools check.

### Item 2. RV. A gap review of the text phase 7 wrote

- **What.** One source-reader gap review (decision 0019) of the 13 new rules, the 7 rules whose class changed, the CLS-35 layout tables, and Q-0369 to Q-0376. Then one repair pass.
- **Evidence.** Section 1.7. Phase 7 wrote these rules from findings and probes after the reviews of their pages had run. CLS-35 states the matrix layout in tables, and STG-25 and CLS-34 depend on it. ODR-15 and the DEFARG rules come from reader findings. The class changes (CTOR-17 and DCMP-8 to reject, CNS-39 and LSUP-5 to no-diagnostic) each change which programs a compiler may accept.
- **Cost.** 0.5 to 1 session of repairs, plus one source-reader session.
- **Risk.** Medium. CLS-35 and BF-9 state layout in dense terms. Keep the tables in the matrix's own terms and run the check on each repaired page.
- **Unblocks.** New text then has the same review as the pages it joined. The eight new questions get their first reading.

### Item 3. Q3. Settle what the current rules allow, with a Divergence column

- **What.**
  - `question_index.py` adds a Divergence column from the test notes: the families whose overrides name the question, and whether only an older version differs. The script of section 1.6 does this in about 40 lines.
  - Settle Q-0076 under decision 0051, and answer Q-0342 from CLS-35.
  - For the 29 questions whose rule changed on 2026-11-03, settle each one that meets the settling rule once a reader batch on its page has used the changed rule (item 5), and record the batch.
  - Write the waiting rule for a changed question into decision 0054's successor. A question does not settle in the commit that changes its rule.
- **Evidence.** Section 1.6.
- **Cost.** 0.5 to 1 session. The column is about 0.25.
- **Risk.** Low. Settling changes status lines and index rows.
- **Unblocks.** The open list shrinks without a new settling rule. The column shows each question's state at a glance and is a precondition for MSVC.

### Item 4. PQ. Principles for the questions that no principle answers

- **What.** For each of the 59 open questions whose note says that no principle on the page gives the answer, the writer decides whether the matrix shows a model that a principle can state. Where it does, the writer adds or widens a principle (decision 0024), with a test where the principle makes a claim that no test checks. Where it does not, the note says so and the question stays open.
- **Evidence.** Section 1.6. The no-principle reason is the most common in the notes. The questions spread over many pages, which suggests principles that state the general model and skip edge cases. Decision 0054's settling rule needs a principle, so these questions cannot settle by review alone.
- **Cost.** 1.5 to 2 sessions.
- **Risk.** Medium. A principle is prose, so it has more surface for similarity than a rule table. The larger risk is circular reasoning, because a principle written to settle a question proves nothing about that question. So a question does not settle on a principle written in the same phase until a reader or reviewer has used the principle (item 5, or item 2's reviewer if the timing fits).
- **Unblocks.** Up to about 50 settlements in a later step. Pages whose principles predict the cases that readers find hard.

### Item 5. E3. Predict on the repaired pages and the consolidated pages

- **What.**
  - Predict batches again for the pages that the GR repairs changed after their last batch: throw, handlers, unwinding, names, objects, storage, language support, translation, odr, types, keywords, and the two overloading pages.
  - Predict batches for constant expressions and evaluation order, which phase 6 consolidated and no reader has used since.
  - Run them after items 2 and 4, so that the batches read the repaired text and the new principles.
- **Evidence.** Section 1.7. Phase 7 batches gave 46 findings over 10 pages, mostly asks for Evidence and examples, with five real gaps.
- **Cost.** 1 to 1.5 writer sessions of repairs. The reader sessions are separate.
- **Risk.** Low.
- **Unblocks.** The settling of items 3 and 4. A measure of whether the GR repairs and the phase 6 consolidation made the pages easier to use.

### Item 6. V2. Evidence that does not repeat the questions

- **What.**
  - In Evidence, a settled or open question gets one line: the question, the answer the page follows, and the family that differs. The question file keeps the probe details and the compiler messages.
  - Move the generated probe grid tables of the two overloading pages into generated files beside the grids, as round 4 item V proposed. Each rule's Evidence keeps its source note (decision 0044).
  - `status_table.py` reports Evidence words per page, so growth shows in each commit.
- **Evidence.** Section 1.3. Item C reorganized the overloading Evidence and removed no fact. Phase 7 then added 10,974 Evidence words.
- **Cost.** 1 to 1.5 sessions, most of it on the overloading, odr, storage, and unwinding pages.
- **Risk.** Medium. A move must keep each rule's source note and the inventory notes from source readers. Similarity checks the moved text in its new place.
- **Unblocks.** Pages that a reader can read through. Smaller similarity units on the largest pages. Less work per settled question.

### Item 7. FE18. Front-end rounds 18 and later

- **What.** Answer the findings of each round. Round 18 starts with the 13 tests that 2eb7c8d left and the analyses that report 17 lists as missing (access after overload resolution, defaulted comparisons, consteval escalation, template checks).
- **Evidence.** Section 1.4. Rounds 15 to 17 gave 16 findings, each a precise gap such as the object of an early member call (G129) or the order of C names (G132).
- **Cost.** 0.5 session per round.
- **Risk.** Low.
- **Unblocks.** The interpreter for the remaining 65 run-unsupported tests. Program tests that the front end runs as a regression check.

### Item 8. D2. Diagnose batches on the pages with the most error patterns

- **What.** Diagnose batches for constructors, names, instantiation, constant expressions, initialization, and function declarations.
- **Evidence.** Section 1.7. Decision 0054 deferred Diagnose beyond D-classes. D-classes scored 143 of 143 and still gave 17 findings (DC-1 to DC-17).
- **Cost.** 1 to 1.5 sessions of repairs.
- **Risk.** Low. The batch shows a compiler message to the reader, and the writer adopts no wording from it.
- **Unblocks.** A reader check that each `tidal-error` pattern and its rule match how compilers report the error.

### Item 9. L. Test directories that match the pages

- **What.** Move the tests of six pages to directories named for their pages, as round 4 item L listed.
- **Evidence.** Left by phase 7. Commit a9906a8 ran the tests of the overloading pages from `tests/foundations/overload-resolution` and `tests/foundations/conversion-sequences`.
- **Cost.** 0.25 session, plus the full matrix run that closes the phase.
- **Risk.** None for similarity. `RESULTS.md`, `conformance.tsv`, and every `:tests:` line change in one commit, so the move belongs at a phase boundary.
- **Unblocks.** Tools and batch scripts can map a page to its directory without a table.

## 3. Out of scope

1. **No new topic.** The status page lists no in-scope topic without a page. The cost is in questions, Evidence, and review of new text.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. The 32 *(referenced)* markers state the boundary.
3. **MSVC stays deferred** until item 3's Divergence column exists. Overrides grew from 652 to 763 lines in phase 7, and a new family would add to the pages that carry the most (conversion sequences, overload resolution, and constraints, 27 each).
4. **A further settling rule from the writer.** Row 1 of `pending.md` already lists the 15 questions that settle if two differing families are allowed. That choice belongs to the maintainer. Items 3 and 4 work within the current rules.
5. **Row 4 of `pending.md`.** The 21 questions and 11 rules where the matrix agrees and a review reads the C++23 text the other way wait on the maintainer. The writer keeps the matrix answer and adds no rows without a review that reads against the matrix.
6. **Port suites, guides on the site, and the index of later-version acceptances** stay as decision 0054 left them. From the writer's view, item 8 gives more per session than a Port suite, because Diagnose scores automatically.

## 4. Process changes

Ranked by writer time saved.

1. **One front-end binary per round, and status-to-status changes in `frontend_check.py`.** A round builds into its own target directory, and `frontend_check.py` takes the binary path and the test commit the round started from. The check reports a test that moves from one failing status to another. Phase 7 ran three rounds beside writer commits and worked around the gap with worktrees and a scratch build each time (section 1.4).
2. **Smaller per-commit similarity reports.** A commit's report lists the units of the files it changed, with the flag count. The phase-close report checks the whole repository. Phase 7 added 56,481 report lines, nearly all repeated `ok` lines.
3. **A missing review reading fails the check.** After item 1, `question_index.py --check` fails when a question that a review names has no Opinions entry for that review.
4. **No settling in the commit that changes the rule.** Item 3 writes this down. Phase 7 already kept 29 such questions open by habit.
5. **Evidence words in the status table.** Item 6 adds the column, so a commit that grows Evidence shows it.
6. **Check results written by a tool.** A script runs the check chain and writes the Checks paragraph with the counts and the commit. Phase 7 had no correction commits, so this ranks lower than in round 4, but every commit body still types its counts.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, and scripts run on commit 7d8c982, with the round 4 comparisons run on commit 3d98c93.

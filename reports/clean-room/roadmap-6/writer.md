# Roadmap, round 6: what to do after phase 8, from a clean-room writer's view

Date: 2026-11-04. Decision 0029, with decisions 0033, 0039, 0044, 0050, 0051, 0054, and 0055. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, style guide, specification format, exercises, the decisions named above, and `decisions/pending.md`), its own round-5 report `reports/clean-room/roadmap-5/writer.md`, the specification pages, the test headers, `questions/README.md` and the notes of the open question files, the phase 8 reports (R8, I1, W16, W17, X17 to X20, front-end reports 18 and 19, `conformance.tsv`, `tests/programs/`), the two 2026-11-04 gap reviews under `reports/review/`, and `git log dc90663~1..HEAD` with commit bodies. It did not read the other round-6 reports or `inputs/`. All counts come from read-only `grep`, `git`, and short Python scripts kept outside the repository, and from read-only runs of `tools/check_specification.py`, `tools/question_index.py --check`, and `tools/propose_check.py`, on the tree at commit 1b16115. Comparisons with round 5 use the same scripts on commit 7d8c982, extracted with `git archive`. The report wrote no repository files.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 7 (7d8c982) | Now (1b16115) |
|---|---|---|
| Rule entries | 1691 (15 withdrawn), 1676 live | 1696 (15 withdrawn), 1681 live |
| Principles | 343 | 343, of which item PQ widened 52 |
| Words under `specification/` | 252,620 | 252,225 |
| Words on the 85 pages, without index and glossary | 245,475 | 244,746 |
| Evidence words on those pages | 64,937 (26%) | 54,522 (22%) |
| Evidence words in paragraphs that name a question | 36,305 | 34,099 |
| Words outside Evidence on those pages | 180,538 | 190,224 |
| `cpp` example blocks | 181, none compiled by a tool | 197, all compiled by `check_examples.py`: 27 fragment, 109 reject, 7 diverges, 54 compile alone |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6337, of which 67 are program tests | 6500, of which 71 are program tests |
| Single-test rules | 8 | 0 |
| Rules that rest only on tests with an override for a newest compiler | 44 (count from item TE's commit) | 29, each a warning of `check_specification.py` |
| Test files with a per-compiler override | 606 (763 override lines) | 634 (797 override lines) |
| Test files with an EDG override | 251 | 268 |
| Tests with `tidal-interpreter: run-fail` | 14 | 19 |
| Tests with `tidal-error` patterns | 2916 | 2981 |
| Question files | 376: 209 provisional, 13 compiler defect, 154 settled | 385: 221 provisional, 13 compiler defect, 151 settled |
| Open questions with no review reading | 14 | 17: eight process questions and Q-0377 to Q-0385 |
| *(referenced)* markers | 32 | 33 |
| Front end (`conformance.tsv`) | 6068 of 6337 pass, judged with the EDG override first | 6183 of 6500 pass, judged by the base expectation (item HN); 135 missed rejects, 85 unsupported, 57 run-unsupported, 21 EDG-AGREES, 12 false rejects, 7 wrong results |
| Committed similarity reports | 155 (13.2 MB, 191,021 lines) | 165 (17.0 MB, 237,619 lines) |
| `question_index.py --check` | 49 warnings | 0 problems; a missing review reading now fails |
| `check_specification.py` | 0 problems | 0 problems; 29 override-only warnings and one warning that 68 pages keep the older Evidence form |

Phase 8 added 5 rules (ACC-20, CONV-21, DEFARG-13, LSP-8, PFX-22) and changed the class of none. It added 163 tests: 8 in R2, 10 in RT, 44 in RV, 17 in PQ and TE, 69 in the repairs from X17 to X20, I1, and G136 to G141, 14 with W16 and W17, and 1 from R8. It opened 9 questions (Q-0377 to Q-0385), settled 5, and reopened 8. The open count went from 209 to 221. Decision 0055 aimed to end below 209.

The pages lost 729 words in all. Evidence lost 10,415 words, and the text outside Evidence gained 9,686. The widened principles, the new CLS-35 rows, and the examples took most of that gain.

### 1.2 What phase 8 took from round 5

| Round 5 item | Phase 8 result |
|---|---|
| R2, close the phase 7 record | Done as item R2 (8f59cc7). The 49 readings are recorded and a missing reading fails the check. `propose_check.py` reads the X14 table (56 rows). The size estimate comes from `status_table.py`. Each single-test rule has a second test. |
| RV, a gap review of the text phase 7 wrote | Done as item RV (7a68692): 44 tests, 2 new rules, 2 new questions, 4 settled, and 7 more items in `pending.md` row 4. |
| Q3, settle what the current rules allow, with a Divergence column | Done in part. Q-0076 settled. The Divergence column was not built. Q-0342 is still open, and its note still says that it waits on item TP, which phase 7 finished. |
| PQ, principles for the questions no principle answers | Done as item PQ (e11f565). The style guide has the principle test. 52 principles were widened. 8 settled questions failed the test and reopened. 60 open questions now wait for a reader or reviewer to use a widened principle. |
| E3, Predict on the repaired and consolidated pages | Done in part as R8 (0949de9): 8 pages, 979 of 982. Throw, handlers, objects, language support, translation, odr, types, keywords, and the two overloading pages had no new batch. |
| V2, Evidence that does not repeat the questions | Done in part as item V (79f6d82). The format is written and checked. 10 pages are converted, and their Evidence fell from 28,280 to 13,581 words. 68 pages keep the older form. |
| FE18, front-end rounds | Done as rounds 18 and 19. |
| D2, Diagnose batches | Not done. Decision 0055 deferred Diagnose again. |
| L, test directories that match the pages | Done (ef19840). |
| Process 1, one front-end binary per round, and status-to-status changes | Half done. `frontend_check.py` now fails on a move from one failing status to another (item HN). Round 19 still built the old front end in a scratch directory, and commits 7a68692 and e11f565 skipped `frontend_check.py` because the round was rebuilding `tidal-fe`. |
| Process 2, smaller per-commit similarity reports | Not done. Each phase 8 report checks the whole repository at about 4,650 lines. |
| Process 3, a missing review reading fails the check | Done in R2. |
| Process 4, no settling in the commit that changes the rule | Adopted by decision 0055. |
| Process 5, Evidence words in the status table | Done in item V. |
| Process 6, check results written by a tool | Not built. Commit dad8bf8 exists only to correct a count in e11f565. |

### 1.3 Where the writer's time went

Phase 8 had 18 commits. Nine were writer commits. The writer commits touched page files 262 times and question files 281 times. Item RT touched 79 pages, mostly to add example markers. Item R2 touched 81 question files, PQ and TE 71, item L 47, and item V 42.

Repairs from findings took most of the new tests. The one repair commit 076aa54 acted on 44 Propose findings, 13 Implement findings, and 6 front-end findings, and added 69 tests and 2 rules. Exercises by use gave the most precise findings per writer hour. I1 named 17 layout steps that the text did not give, and each repair was a sentence or a table row with a test. The Predict batches R8 scored 979 of 982 and gave 39 findings, mostly asks for examples and Evidence lines. Two of the 39 were wrong predictions, and one of those came from a wrong entry in the undefined-rule index.

Question files took a large share again. Each PQ check wrote a dated note in the question file and a matching line in the page's Evidence. Under the older Evidence form, the page line repeats much of the question note.

### 1.4 Questions

221 questions are open. A script over the notes and over `pending.md` sorts them by what holds each one open. A question can fall in more than one group.

- 60 wait for a reader or reviewer to use a principle that item PQ widened (decision 0055). 48 of them are outside `pending.md` and name at most one newest family in their test overrides, so they can settle under decision 0054 once the use is recorded.
- 73 appear in `pending.md` and wait on the maintainer. Row 4 grew from 21 to 26 questions in phase 8.
- 34 notes say that two newest families differ.
- 68 notes mention a review that reads with doubt.
- 17 have no review reading: the eight process questions and the nine questions phase 8 opened.
- 4 wait because item RV changed their rule (Q-0370, Q-0372, Q-0374, Q-0376). No batch or review has read those rules since.

Over the test overrides, the open questions split by the newest families that differ: EDG alone 46, EDG and GCC 41, GCC alone 41, Clang alone 25, Clang and EDG 11, Clang and GCC 8, all three 5. Six have overrides only for older versions, and 38 have none.

The rule "a reader or reviewer has used the principle" has no definition of use. The R8 readers read three pages whose principles PQ widened (CNS-P2, INST-P1, INST-P3, SEQ-P1). Their 982 answers name a principle 4 times, all CE-P1. So no record shows that any widened principle was used, and no question settled on one. The style guide's principle test (section 3) already describes the use. A reader takes the question's program, applies the principle's model without the rule text, and reaches the answer. No exercise runs that test.

### 1.5 Evidence

Item V cut the Evidence of the 10 converted pages by 14,699 words and moved the facts into question files and `tables.md` files. The other 68 pages hold 39,759 Evidence words, and 28,543 of them sit in paragraphs that name a question. The largest are instantiation (2,176 Evidence words), class template deduction (2,164), deduction (1,955), new and delete (1,699), specialization (1,366), language support (1,242), concepts (1,232), packs (1,167), arguments (1,153), and members (1,115). On new and delete, language support, and members, more than 85% of the Evidence words sit in paragraphs that name a question.

`check_specification.py` warns once for all 68 pages. A page that the writer converts can go back to the older form without an error.

### 1.6 Coverage by readers and reviewers

- **Gap reviews.** No review has read the text phase 8 wrote: the 5 new rules, the 52 widened principles, the TR-4 policy for link-time diagnostics, the CLS-35 rows from I1 (data size, union layout, reference and member-pointer sizes), the X17 to X20 repairs (CONV-20, CONV-21, KW-3, OPR-8, OPR-9, VIRT-1, VIRT-3, VIRT-7, VIRT-13, LNK-1, ODR-10, STG-23), the G136 to G141 repairs (CTOR-17, VOL-2, STG-29, OVL-38, RFOR-7), and Q-0377 to Q-0385.
- **Predict.** Phase 8 changed 82 of the 85 pages. R8 read 8 of them after item PQ. `exercises.md` section 7 still says that 18 pages have only a batch that showed test names. R8 re-read six of those pages (constant evaluation, evaluation order, names, constraints, instantiation, dependent names), so 12 remain: translation, objects, values, conversions, class template deduction, deduction, arguments, packs, concepts, ordering, declarations, and specialization.
- **Implement.** One exercise (I1), on class layout. It scored 241 of 243 and still found 13 gaps.
- **Diagnose.** 4 pages (D-classes). 2981 tests carry `tidal-error` patterns.
- **Propose.** Every proposal that round 5 rated ready is done. `propose_check.py` lists 104 rows of the 20 reports whose rule changed after the report's commit.
- **Port.** None.

### 1.7 Front end and the writer

Item HN made the counts honest, and the pass count fell before it rose again. 57 of the 163 tests that phase 8 added do not pass on the front end: 18 missed rejects, 11 false rejects, 11 EDG-AGREES, 7 unsupported, 6 wrong results, and 4 run-unsupported. Most arrived after round 19 started. Round 19 also left 10 EDG-AGREES tests with reasons, 54 run-unsupported tests, and `char-named-escape.cpp`. Six front-end findings (G136 to G141) each named a sentence that a rule or the format lacked, and each became a change to that rule or to the format.

### 1.8 Similarity

Every flag in phase 8 was reworded before its commit: five runs of common phrasing in X17 to X20, one phrase in W17 twice, and PRIM-P1 and IF-P4 in item PQ. Volume is still the cost. Phase 8 added 10 reports and 46,598 lines, nearly all `ok` lines, and `reports/similarity/` grew from 13.2 MB to 17.0 MB.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | PU. A principle-use batch for the 60 waiting questions | 60 questions wait on use; 48 can settle then; R8 answers name a principle 4 times in 982 | 0.5-1 | low | an open count below the 209 at the end of phase 7 |
| 2 | R3. Close the phase 8 record | stale section 7 count; Q-0342 note; Divergence column not built; no definition of use | 0.25-0.5 | low | item PU; MSVC later |
| 3 | RV2. A gap review of the text phase 8 wrote | 5 new rules, 52 widened principles, TR-4, CLS-35 rows, X and G repairs, 9 questions with no reading | 0.5-1 of repairs | medium | a review level for new text equal to the old; a second use of the principles |
| 4 | V3. The Evidence format on the other 68 pages | 39,759 Evidence words, 28,543 in question paragraphs; item V cut 52% on its 10 pages | 1.5-2 | medium | readable template and allocation pages; less writing per question |
| 5 | FE20. Front-end rounds 20 and later | 57 phase 8 tests fail; 10 EDG-AGREES and 54 run-unsupported left | 0.5 per round | low | the regression signal for the new tests |
| 6 | I2. A second Implement exercise, on overload ranking | I1 found 13 gaps at 241 of 243; Predict finds few at 99.7% | 1 of repairs | low | a check that the overloading pages stand without the probe tables that item V moved |
| 7 | E4. Predict on the pages read only with test names | 12 pages; 82 pages changed in phase 8 | 1 of repairs | low | coverage without test names on every page |
| 8 | D2. Diagnose batches | 2981 tests with patterns; 4 pages read | 1-1.5 of repairs | low | a reader check of patterns and rule citations |

Total for items 1 to 6: about 5 to 7 writer sessions, plus the reader sessions of items 1 and 6 and the source-reader session of item 3. Items 7 and 8 add about 2 to 2.5.

### Item 1. PU. A principle-use batch for the 60 waiting questions

- **What.**
  - A clean-room reader runs the style guide's principle test as an exercise. For each waiting question, the batch gives the question's program (from its tests, with no expectation line) and the principles of the affected page and of the pages they cite. Rule text, Evidence, `:tests:` lines, and question files are removed, as the I1 filter did.
  - The reader gives an outcome for each program and names the principle and the part of its model that decides it. `exercises.py score` scores the outcome against the provisional answer.
  - The writer settles each question where the reader reaches the answer, where the principle decides it, and where decision 0054's family condition holds. The dated note names the batch.
  - Where the reader misses, the question stays open, and the miss is a finding against the widened principle.
- **Evidence.** Section 1.4. The 60 questions cannot settle until a use is recorded, and no current exercise records principle use.
- **Cost.** 0.5 to 1 writer session: building the batch from the tests, then the settling and the repairs for misses. The reader session is separate.
- **Risk.** Low. Settling changes status lines, notes, and index rows. A principle repaired after a miss gives no answer in the same phase, so a repaired question waits again.
- **Unblocks.** Up to 48 settlements. The open count would fall to about 173. The batch also tests the 52 widened principles by use, which item PQ could not do for itself.

### Item 2. R3. Close the phase 8 record

- **What.**
  - Define "used" in the style guide, section 3: a reader batch or a review whose record names the principle and the question's program. A Predict answer gains an optional `principles` field, and `exercises.py score` counts it.
  - `exercises.md` section 7 gives the 12 pages that have only a batch with test names, and names the R8 batches.
  - Update Q-0342. Item TP, CLS-35, and I1 have run, so the note gives the current reason or the question settles.
  - Build the Divergence column of `question_index.py` from the test overrides, as round 5 item Q3 proposed. The script of section 1.4 does the work in about 40 lines.
  - Record that commits 7a68692 and e11f565 ran without `frontend_check.py`, and that round 19 added their baseline rows.
- **Evidence.** Sections 1.2, 1.4, and 1.6.
- **Cost.** 0.25 to 0.5 session.
- **Risk.** Low.
- **Unblocks.** Item 1 needs the definition of use. The Divergence column shows each question's state at a glance and is a precondition for MSVC.

### Item 3. RV2. A gap review of the text phase 8 wrote

- **What.** One source-reader gap review (decision 0019) of the 5 new rules, the 52 widened principles, TR-4, the CLS-35 rows that I1 added, the X17 to X20 and G136 to G141 repairs, and Q-0377 to Q-0385. Then one repair pass.
- **Evidence.** Section 1.6. Phase 8 wrote this text from findings and probes after RV read phase 7's text. The widened principles state cases that rules held before, so a review can find a principle that now says more than the matrix shows. CONV-21 moved the narrowing table out of LIST-2 and now serves ten rules.
- **Cost.** 0.5 to 1 session of repairs, plus one source-reader session.
- **Risk.** Medium. A principle is prose and has more surface for similarity than a rule table. Each repaired page runs `similarity.py --fail-on-flag`.
- **Unblocks.** The new text gets the same review as the old. A review reading of a widened principle counts as a second use under item 2's definition. The nine new questions get their first reading.

### Item 4. V3. The Evidence format on the other 68 pages

- **What.**
  - Convert the 68 pages to the format of `specification-format.md`, largest first: instantiation, class template deduction, deduction, new and delete, specialization, language support, concepts, packs, arguments, and members. A divergence becomes one line, and the question file keeps the probes and messages.
  - After each page converts, `check_specification.py` fails if that page goes back to the older form. When the last page converts, the warning becomes an error.
- **Evidence.** Section 1.5. On its 10 pages, item V cut Evidence from 28,280 to 13,581 words and removed no fact.
- **Cost.** 1.5 to 2 sessions. The ten largest pages take about half.
- **Risk.** Medium. A move must keep each rule's source note (decision 0044) and the inventory notes from source readers. Facts that only a page holds move to the question file first, as item V did.
- **Unblocks.** Pages that a reader can read through. Each later settlement then changes one Evidence line.

### Item 5. FE20. Front-end rounds 20 and later

- **What.** Answer the findings of each round. Round 20 starts with the 57 phase 8 tests that do not pass, the 10 EDG-AGREES tests that report 19 lists, and the largest run-unsupported groups (`new` forms, constructor calls, jumps into a nested statement).
- **Evidence.** Section 1.7.
- **Cost.** 0.5 session per round.
- **Risk.** Low.
- **Unblocks.** The regression signal covers the tests phase 8 added. Findings like G136 to G141 keep finding sentences the rules lack.

### Item 6. I2. A second Implement exercise, on overload ranking

- **What.** A clean-room reader builds the ranking of implicit conversion sequences and the choice of the best viable function from the two overloading pages alone, with Evidence and `:tests:` lines removed. The reader scores the component on the tests under `tests/overloading/` and the grid files, as I1 did on the layout tests.
- **Evidence.** Section 1.3. I1 found 13 gaps that Predict could not, on pages that Predict scores near 100%. Item V moved the probe tables of both overloading pages into `tables.md` files, and the pages now hold 1,916 and 1,909 Evidence words. No reader has used them since. The class-property lists of the copy-move page, scored on `tests/classes/copy-move/grids/`, are the second choice.
- **Cost.** 1 writer session of repairs. The reader session is separate.
- **Risk.** Low. The reader writes code, and the writer adopts no wording from it.
- **Unblocks.** Decision 0055's exercise by use for phase 9. A measure of whether the overloading rules state every step without the moved tables.

### Item 7. E4. Predict on the pages read only with test names

- **What.** Predict batches for the 12 pages of section 1.6, with test names hidden. Answers use the `principles` field of item 2.
- **Evidence.** Section 1.6.
- **Cost.** 1 writer session of repairs.
- **Risk.** Low.
- **Unblocks.** Every page has a batch without test names. Principle use recorded on the eight templates pages, where item PQ widened DED-P3, CPT-P3, PACK-P2, ORD-P1, and CTAD-P1.

### Item 8. D2. Diagnose batches

- **What.** Diagnose batches for constructors, names, instantiation, constant evaluation, initialization, and function declarations, as round 5 item 8 proposed.
- **Evidence.** Section 1.6. Decisions 0054 and 0055 both deferred it.
- **Cost.** 1 to 1.5 sessions of repairs.
- **Risk.** Low. The batch shows a compiler message to the reader, and the writer adopts no wording from it.
- **Unblocks.** A reader check that each `tidal-error` pattern and its rule match how compilers report the error.

## 3. Out of scope

1. **No new topic.** The status page lists no in-scope topic without a page. Phase 8 added 5 rules and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. The 33 *(referenced)* markers state the boundary.
3. **The proposals of X17 to X20** (narrowing as an error everywhere, required override, width-named literal suffixes, opaque type aliases) stay out of the specification, as commit 076aa54 says.
4. **MSVC stays deferred** until item 2's Divergence column exists. Override lines grew from 763 to 797 in phase 8.
5. **A further settling rule from the writer.** Row 1 of `pending.md` lists 15 questions that settle if two differing families are allowed. That choice belongs to the maintainer. Item 1 works within the current rules.
6. **Rows 4 and 7 of `pending.md`.** Row 4 now holds 26 questions and several rules where the matrix agrees and a review reads the C++23 text the other way. Row 7 holds the later-version probe. Both wait on the maintainer.
7. **Port suites and guides on the site** stay as decision 0055 left them. From the writer's view, an Implement exercise gives more precise findings per session than a Port suite.

## 4. Process changes

Ranked by writer time saved.

1. **A definition of use, recorded where it happens.** A reader answer or a review reading counts as a use of a principle when it names the principle and the program. The question's dated note names the batch or review. Without the definition, decision 0055's waiting rule holds 60 questions with no way to release them.
2. **Smaller per-commit similarity reports.** A commit's report lists the units of the files it changed, with the flag count. The phase-close report checks the whole repository. Phase 8 added 46,598 report lines.
3. **A fixed front-end binary for writer commits.** Each round leaves its `tidal-fe` at a path that the next rebuild does not touch, and `frontend_check.py` takes the binary path. Writer commits during a round then still run the check. Two phase 8 commits skipped it.
4. **A ratchet on the Evidence form.** A converted page cannot go back to the older form (item 4).
5. **An open-count aim by cause.** Decision 0055 aimed below 209 and ended at 221, because PQ reopened 8 and held 60. The next decision states the aim per group of section 1.4, so the record shows which group moved.
6. **Check results written by a tool.** A script runs the check chain and writes the Checks paragraph with the counts and the commit. Phase 8 had one correction commit (dad8bf8).

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, and scripts run on commit 1b16115, with the round 5 comparisons run on commit 7d8c982.

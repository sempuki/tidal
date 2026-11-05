# Roadmap, round 7: what to do after phase 9, from a clean-room writer's view

Date: 2026-11-05. Decision 0029, with decisions 0033, 0039, 0044, 0050, 0051, 0054, 0055, and 0056. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, style guide, specification format, exercises, the decisions named above, and `decisions/pending.md`), its own round 6 report `reports/clean-room/roadmap-6/writer.md`, the specification pages, the test headers, `questions/README.md` and the notes of the open question files, the phase 9 reports (I2, X21, R9, PU1, PU2, front-end reports 20 and 21, `conformance.tsv`, `examples.tsv`), the three 2026-11-04 gap reviews of the text phase 8 wrote under `reports/review/`, and `git log 1b16115..HEAD` with commit bodies. It did not read the other round 7 reports, `inputs/`, or `build/exercise-keys/`. All counts come from read-only `grep`, `git`, and short Python scripts kept outside the repository, and from read-only runs of `tools/check_specification.py`, `tools/question_index.py --check`, and `tools/propose_check.py`. Comparisons with round 6 read commit 1b16115 with `git show` and `git ls-tree`. Four probes under `reports/clean-room/roadmap-7/writer-probes/` ran on Clang 22 and GCC 16 on the host and on Clang 18, Clang 22, GCC 14, and EDG 7.0 in their containers. Commit read: `017b401`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 8 (1b16115) | Now (017b401) |
|---|---|---|
| Rule entries | 1696 (15 withdrawn), 1681 live | 1697 (15 withdrawn), 1682 live |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 111 rules and 66 principles (phase 8) | 78 rules and 35 principles on 45 pages, and 1 new rule (ORD-17) |
| Words on the 85 pages, without index and glossary | 244,746 | 262,940 |
| Evidence words on those pages | 54,522 (22%) | 66,692 (25%) |
| Words outside Evidence on those pages | 190,224 | 196,248 |
| `cpp` example blocks | 197 | 219; the front end passes 175 of 177 that it runs |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6500, of which 71 are program tests | 6619, of which 85 are program tests |
| Live rules with one test | 0 | 1 (CPY-28, since item R3 removed a `:tests:` entry) |
| Live rules with exactly two tests | not counted | 686 |
| Rules that rest only on tests with an override for a newest compiler | 29 | 28, each a warning of `check_specification.py` |
| Test files with a per-compiler override | 634 (797 override lines) | 668 (828 override lines) |
| Test files with an override for a newest family | not counted | 564: EDG alone 198, GCC alone 180, Clang alone 100, EDG and GCC 57, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Test files with an EDG override | 268 | 278 |
| Tests with `tidal-allows-edg` | 0 | 5 |
| Tests with `tidal-interpreter: run-fail` | 19 | 19 |
| Tests with `tidal-error` patterns | 2981 | 3041 |
| Question files | 385: 221 provisional, 13 compiler defect, 151 settled | 400: 172 provisional, 13 compiler defect, 215 settled |
| *(referenced)* markers | 33 | 33 |
| Front end (`conformance.tsv`) | 6183 of 6500 pass; 135 missed rejects, 85 unsupported, 57 run-unsupported, 21 EDG-AGREES, 12 false rejects, 7 wrong results | 6362 of 6619 pass; 118 missed rejects, 79 unsupported, 52 run-unsupported, 6 false rejects, 2 EDG-AGREES, 0 wrong results |
| Committed similarity reports | 165 (17.0 MB, 237,619 lines) | 173 (20 MB, 276,724 lines) |
| `question_index.py --check` | 0 problems | 0 problems |
| `check_specification.py` | 0 problems, 29 override-only warnings, 1 older-form warning | 0 problems, 28 override-only warnings, 1 one-test warning |

Phase 9 added 1 rule (ORD-17) and 119 tests, and changed the class of two rules (OPR-9, from recommended to no-diagnostic, and LAM-28, from required to unspecified). It opened 15 questions (Q-0386 to Q-0400) and settled 64. The open count went from 221 to 172, below the aim of decision 0056.

Evidence grew by 12,170 words. Item V3 accounts for 4,799 of them on its 68 pages (45,776 to 50,575), because the new Split tests tables are longer than the settlement tables they replaced. The rest came from the repairs.

### 1.2 What phase 9 took from round 6

| Round 6 item | Phase 9 result |
|---|---|
| PU, a principle-use batch | Done as PU1 (29 questions, 66 programs, 20 settled) and PU2 (15 questions, 44 programs, 7 settled), with the `exercises.py principles` tool (6f3d108). |
| R3, close the phase 8 record | Done in 2ebb5c3. A use is defined in style guide section 3, and Predict answers carry `principles`. Q-0342 settled. The Divergence column of `question_index.py` was not built. |
| RV2, a gap review of the text phase 8 wrote | Done in 1bf7812: three reviews, 58 tests, 36 questions settled, 9 opened. |
| V3, the Evidence format on the other 68 pages | Done in 789c04e. The check now fails on a missing Divergences part. Evidence on those pages grew by 10%. |
| FE20, front-end rounds | Done as rounds 20 and 21. |
| I2, a second Implement exercise on overload ranking | Done as I2 on partial ordering instead (decision 0056). 140 of 140 checks, 17 findings that name 22 missing steps. Overload ranking has no Implement exercise. |
| E4, Predict on the pages read only with test names | Done as R9 on 11 of 12 pages, 1055 of 1055. The objects page waited for a tool fix that came in 32c89f7. |
| D2, Diagnose batches | Not done. Decision 0056 deferred Diagnose a third time. |
| Process 1, a definition of use | Done in R3. |
| Process 2, smaller per-commit similarity reports | Not done. Phase 9 added 8 reports and 39,105 lines. |
| Process 3, a fixed front-end binary for writer commits | Not done. Commits 1bf7812 and d9c18bc skipped `frontend_check.py` because a round was rebuilding `tidal-fe`. |
| Process 4, a ratchet on the Evidence form | Done in V3. |
| Process 5, an open-count aim by cause | Done. The closing commit 017b401 gives the change by cause. |
| Process 6, check results written by a tool | Not done. Commit 9b1963e exists to record what 32c89f7 did and did not say. The body of 1bf7812 repeats two paragraphs and gives the reason for the skipped `frontend_check.py` twice, in two forms. |

### 1.3 Where the writer's time went

Phase 9 had 17 commits. Ten were writer or integrator commits. Four were repair commits: 1bf7812 (RV2, 58 new tests), b7e0adf (PU1 and G142 to G145, 8 tests), d9c18bc (I2 and X21, 20 tests and ORD-17), and 32c89f7 (PU2 and R9, 8 tests). V3 was the largest single edit (68 pages, no rule text).

Settling came from three sources. RV2 settled 36, PU1 20, PU2 7, and R3 one. The two principle-use batches settled 27 questions from 110 programs, about one question per four programs.

The ordering page took the most rule work. I2's 17 findings, two front-end faults, R9D-1 to R9D-3, and PU2-2 changed ORD-1, ORD-3, ORD-4, ORD-6, ORD-9, ORD-11 to ORD-14, ORD-P1, ORD-P2, and ORD-P4, and added ORD-17. Three new questions (Q-0396 to Q-0398) came from that work.

R9 scored 1055 of 1055 and gave 33 findings, mostly asks for an example, a table row, or one more sentence. Five of the 1055 answers name a principle.

### 1.4 Questions

172 questions are open. A script over the notes, the test overrides, and `pending.md` sorts them by what holds each one open. A question can fall in more than one group.

- 67 appear in `pending.md` and wait on the maintainer. Row 4 ("Matrix against review") holds 43 items. In 17 of them one newest family follows the review, and in 26 every newest family keeps the matrix's answer.
- 21 have a latest note that says a rule or principle changed and that the question waits for a reader or review to use the change. `exercises.py principles` selects only 6 of them (Q-0217, Q-0218, Q-0293, Q-0340, Q-0378, Q-0386). The other 15 use wordings that its pattern `WAITS_FOR_USE` does not match, such as "a question settles only after a reader or a review has used the changed rule" (Q-0247, Q-0263, Q-0269, Q-0299, Q-0358, all from 2026-11-03) or "no reader or review has used the changed rule" (Q-0370, Q-0376). Neither PU batch saw these 15. Of the 21, 14 are outside `pending.md` and name at most one newest family in their test overrides.
- 42 are outside `pending.md`, name at most one newest family, and are held because the latest review reads with doubt, finds the reference unclear or silent, or reads part of the question against the answer. 13 of them have a rule in their Affects line that changed after phase 8 began: Q-0018 (ENUM-5), Q-0175 (SINIT-3), Q-0184 and Q-0328 (INIT-1), Q-0295 (TSP-14), Q-0318 (CE-9), Q-0325 (CAST-6), Q-0357 and Q-0359 (STG-12), Q-0372 (BF-9), and Q-0361, Q-0381, and Q-0384 (CLS-35). Their reviews read the older text.
- 13 have no review reading: the eight process questions and Q-0396 to Q-0400, which phase 9 opened after RV2. Four of the five name at most one newest family.
- Six questions were held again in phase 9 after a use, because the repair of the use's finding changed the principle: Q-0175, Q-0217, Q-0218, and Q-0293 in d9c18bc, and Q-0217, Q-0218, Q-0293, Q-0378, and Q-0386 in 32c89f7.

Over the test overrides, the open questions split by the newest families that differ: EDG and GCC 40, EDG alone 30, Clang alone 23, GCC alone 18, Clang and EDG 11, Clang and GCC 8, all three 3. 39 have no newest-family override, and 15 of those have no test.

The question notes and the overrides can drift apart. The V3 commit records that the Q-0290 note names GCC 16 and EDG 7.0 as differing while its test has an override only for Clang. A crude script that compares the families named in each latest "Kept open" note with the override families finds two more candidates, Q-0025 and Q-0221. No tool checks this, because the Divergence column that rounds 5 and 6 proposed was not built.

### 1.5 Evidence

Every page now uses the format of `specification-format.md`. The Split tests part is a hand-made copy. A script in the writer's scratch space printed the 64 tables of V3, and no tool regenerates or checks them. At commit 017b401 those 64 tables match the overrides of their pages. Seven pages carry tests with an override for a baseline compiler and have no Split tests part: constraints (28 tests), constant evaluation (24), translation (24), dependent names (23), ordering (19), odr (13), and assignment (1). Six of the seven are pages that phase 8 item V converted before the Split tests part existed. The format says the table has one row for each such test.

The largest Evidence segments are deduction (2,370 words), instantiation (2,356), class template deduction (2,282), overload resolution (2,027), new and delete (2,016), conversion sequences (1,918), and translation (1,854).

### 1.6 Coverage by readers and reviewers

- **Gap reviews.** No review has read the text phase 9 wrote. That text is 78 changed rules, 35 changed principles, and ORD-17, on 45 pages. 43 of the rules (with ORD-17) and 20 of the principles, on 28 pages, changed after the RV2 commit, from the findings of PU1, PU2, R9, I2, X21, and G142 to G145. The rest are RV2's own repairs, which the writer wrote from the reviews and which no review has read in their new form. The ordering page and the static initialization page (SINIT-3 to SINIT-7, SINIT-P2, STO-10, STO-P1, TR-P1, TR-4) changed most.
- **Predict.** Every page except objects has a batch without test names. The ordering page changed in 32c89f7, after its R9 batch.
- **Principle use.** PU1 and PU2 are the only records of use. R9 readers named a principle in 5 of 1055 answers.
- **Implement.** I1 (class layout) and I2 (partial ordering). Each scored at or near full marks and still found more than a dozen missing steps.
- **Diagnose.** D-classes only, 4 pages. 3041 tests carry `tidal-error` patterns.
- **Propose.** X21 used the last ready proposal. `propose_check.py` lists 21 reports and 120 rows whose rule changed after the report's commit.
- **Port.** None.

### 1.7 Front end and the writer

Rounds 20 and 21 removed every wrong run result and cut EDG-AGREES from 21 to 2 and false rejects from 12 to 6. 118 missed rejects, 79 unsupported, and 52 run-unsupported tests remain.

Report 21 gives six findings, G146 to G151. R9D-1 answered G146 in 32c89f7 (ORD-9 now says how `T&&` takes `A&` inside the pair). G147 to G151 have no answer in the tree. The writer probes give these results.

| Finding | Probe | Clang 18, Clang 22 | GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|---|---|
| G147, ORD-6 with members of two classes | `g147-using-member-ordering.cpp`: a base member template brought in by a using declaration against a derived member template | accept; the run chooses the base's `T*` form | accept; the same choice | accept |
| G148, COND-16 and direct binding to a class prvalue | `g148-cond-derived-prvalue.cpp`: a `Derived` prvalue against a `Base` xvalue | the result is a prvalue of `Base` | the result is an xvalue of `Base` | the result is a prvalue of `Base` |
| G150, CMP-8 and a member of a base subobject | `g150-base-member-compare.cpp`: `&x.b < &x.d` in constant evaluation | not a constant | a constant | not a constant |
| G151, BF-1 and an explicit conversion function | `g151-explicit-conversion-width.cpp`: one explicit `operator int` and one `operator long` | accept | reject | accept |

In G148, G150, and G151, GCC is the one family that differs. G150 has the split of Q-0387. G147 shows no split, so a test and one sentence in ORD-6 can answer it.

### 1.8 Similarity

Every flag in phase 9 was reworded before its commit. V3 reworded two Divergences lines, and the integrator removed a document number from one RV2 review. Volume is still the cost. `reports/similarity/` holds 173 files and 20 MB.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | RV3. A gap review of the text phase 9 wrote, and of the held questions | 79 rules and 35 principles unread; Q-0396 to Q-0400 unread; 13 doubt-held questions read against older text | 0.75 to 1 of repairs | medium | first readings; review uses that can settle held questions |
| 2 | R4. Close the phase 9 record | 15 held questions that the PU tool does not select; 7 pages without Split tests; no Divergence column; Q-0290 | 0.5 | low | item PU3; MSVC later |
| 3 | G. Answers for G147 to G151 | five unanswered findings; probes in section 1.7 | 0.25 | low | front-end round 22 |
| 4 | PU3. A principle-use batch after the repairs of RV3 | 14 settleable candidates among 21 held | 0.5 | low | settlements, after the last principle change of the phase |
| 5 | FE22. Front-end round 22 | 118 missed rejects, 79 unsupported, 52 run-unsupported | 0.5 per round | low | the regression signal for phase 9 tests |
| 6 | I3. Implement on overload ranking | I2 found 22 steps at 140 of 140; overloading pages hold the fourth and sixth largest Evidence | 1 of repairs | low | a check of the overloading rules by use |
| 7 | D2. Diagnose batches | deferred three times; 3041 tests with patterns, 4 pages read | 1 to 1.5 of repairs | low | a reader check of patterns and rule citations |

Items 1 to 5 cost about 2.5 to 3 writer sessions, plus the source-reader session of item 1 and the reader sessions of items 4 and 5. Items 6 and 7 add about 2 to 2.5.

### Item 1. RV3. A gap review of the text phase 9 wrote, and of the held questions

- **What.**
  - One source-reader gap review (decision 0019) in three parts, as RV2 had. The review reads ORD-17, the 78 changed rules, and the 35 changed principles of section 1.6. The ordering page and the static initialization rules get a full reading.
  - The same review reads Q-0386 to Q-0400 and the 13 doubt-held questions of section 1.4 against the current text.
  - Where a held question waits on a principle, the reading names the principle and the question's program, so the reading counts as a use under decision 0056.
  - Then one repair pass by the writer, with probes on the three newest compilers.
- **Evidence.** Sections 1.4 and 1.6. RV2 settled 36 questions in one pass, more than either PU batch. Five questions have no reading of any kind. The 13 doubt-held questions have reviews of text that has since changed.
- **Cost.** 0.75 to 1 writer session of repairs, plus one source-reader session.
- **Risk.** Medium. The ordering page has dense prose with new tables, and a repair there has more surface for similarity than a rule row. Each repaired page runs `similarity.py --fail-on-flag`.
- **Unblocks.** First readings for Q-0396 to Q-0400. Settlements where the review supports the answer with no doubt and at most one newest family differs. A review use for the 21 held questions, which can settle some of them without a batch.

### Item 2. R4. Close the phase 9 record

- **What.**
  - One form for the dated note of a held question, for example "Waits for a use of PRINCIPLE-ID." `question_index.py --check` fails on a provisional question whose latest note says that a rule changed and that does not use the form. The 15 notes of section 1.4 get the form.
  - `tools/status_table.py` writes the Split tests table of each page from the test headers, as it writes the agreement sentence, and `check_specification.py` fails when a table is stale or missing. The seven pages of section 1.5 gain the table.
  - The Divergence column of `question_index.py`, from the test overrides, and a check that each latest "Kept open" note names the families that its overrides name. Q-0290, Q-0025, and Q-0221 get a correct note first.
  - The Predict batch of the objects page, now that `exercises.py predict` handles the included header.
  - CPY-28 gets a second test.
- **Evidence.** Sections 1.1, 1.4, and 1.5. The V3 commit itself records the Q-0290 mismatch and leaves it for a later pass.
- **Cost.** 0.5 session, plus the reader session of the objects batch.
- **Risk.** Low. The edits touch notes, tables, and tools.
- **Unblocks.** Item 4 needs the note form, or it misses 15 questions again. The Divergence column shows each question's state at a glance and is a precondition for MSVC.

### Item 3. G. Answers for G147 to G151

- **What.** One sentence and one test for each finding.
  - G147: ORD-6 says which class the object parameter names when a using declaration brings in a base member template. Every compiler agrees on the probe.
  - G148: COND-16 step 2 says whether a reference binds directly to a class prvalue. Clang and EDG give a prvalue, and GCC gives an xvalue. A new question holds the provisional answer.
  - G150: CMP-8 says whether a member of a base subobject counts. The probe joins Q-0387, where GCC already differs.
  - G151: BF-1 and ALN-8 say whether an explicit conversion function counts. Clang and EDG accept, and GCC rejects. A new question holds the provisional answer.
  - G149: INST-12 says what a substitution failure in a waiting requires clause gives at a use, with a test that calls the member. The writer probes it the same way.
- **Evidence.** Section 1.7. Findings of this kind in phases 8 and 9 (G136 to G145) each became a sentence and a test.
- **Cost.** 0.25 session.
- **Risk.** Low.
- **Unblocks.** The front end can implement the five cases in round 22.

### Item 4. PU3. A principle-use batch after the repairs of RV3

- **What.** A principle-use batch, built by `exercises.py principles` after item 2 fixes the note form, for every question that still waits on a use after item 1's repairs. The batch runs after the phase's last change to a principle, so a repair does not reset the questions it just served.
- **Evidence.** Section 1.4. 14 of the 21 held questions could settle under decision 0054 once a use supports them and a review reads them with no doubt. Six questions were held again in phase 9 because a repair came after their use.
- **Cost.** 0.5 writer session for the settling and the repairs. The reader session is separate.
- **Risk.** Low.
- **Unblocks.** Up to 14 settlements, fewer where item 1 already gives the use.

### Item 5. FE22. Front-end round 22

- **What.** A front-end round after items 1 and 3. The round starts with the five G answers, the RV3 repair tests, CPY-27 and CNS-32 (the last EDG-AGREES tests), the four older false rejects, and the run-unsupported tests of two-unit programs (`across-units-class-member.cpp`).
- **Evidence.** Section 1.7.
- **Cost.** 0.5 writer session per round, to answer findings.
- **Risk.** Low.
- **Unblocks.** The regression signal covers the phase 9 and phase 10 tests.

### Item 6. I3. Implement on overload ranking

- **What.** A clean-room reader builds the ranking of implicit conversion sequences and the choice of the best viable function from the two overloading pages alone, with Evidence and `:tests:` lines removed, and scores the component on `tests/overloading/` and its grids. Round 6 ranked this procedure, and decision 0056 ran partial ordering first.
- **Evidence.** I1 and I2 each scored near full marks and still found 13 and 22 missing steps. Overload resolution and conversion sequences hold 2,027 and 1,918 Evidence words. OVL-38 and OVL-41 changed in phase 9. Their Predict batches ran before item V moved their probe tables.
- **Cost.** 1 writer session of repairs. The reader session is separate.
- **Risk.** Low. The reader writes code, and the writer adopts no wording from it.
- **Unblocks.** The exercise by use that each phase runs (decision 0055), on the procedure that most other pages cite.

### Item 7. D2. Diagnose batches

- **What.** Diagnose batches for constructors, names, instantiation, constant evaluation, initialization, and function declarations, as rounds 5 and 6 proposed.
- **Evidence.** Section 1.6. Decisions 0054, 0055, and 0056 each deferred Diagnose.
- **Cost.** 1 to 1.5 writer sessions of repairs.
- **Risk.** Low. The batch shows a compiler message to the reader, and the writer adopts no wording from it.
- **Unblocks.** A reader check that each `tidal-error` pattern and its rule match how compilers report the error.

## 3. Out of scope

1. **No new topic.** The status page lists no in-scope topic without a page. Phase 9 added one rule and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. The 33 *(referenced)* markers state the boundary.
3. **The X21 proposal** (a declared initialization order across translation units) stays out of the specification, as d9c18bc says.
4. **MSVC stays deferred** until item 2's Divergence column exists. Override lines grew from 797 to 828 in phase 9.
5. **A further settling rule from the writer.** Row 1 of `pending.md` lists 15 questions that settle if two differing families are allowed. That choice belongs to the maintainer.
6. **Rows 4 and 7 of `pending.md`.** Row 4 holds 43 items, and 26 of them keep the matrix's answer on every newest family. Row 7 holds the later-version probe. Both wait on the maintainer.
7. **Port suites, guides on the site, and new Propose proposals** stay where decision 0056 left them. The Propose portfolio has no ready proposal.

## 4. Process changes

Ranked by writer time saved.

1. **One form for a held note, checked by a tool.** A question that waits on a use states "Waits for a use of ID" in its latest note, and `question_index.py --check` fails on any other wording for the same state. In phase 9, 15 held questions used other wordings and no batch saw them.
2. **A repair restarts the wait only where the change reaches the question's programs.** The dated note of a repair names which of the question's programs the changed principle decides differently. A change that decides none of them leaves the use in force. Six questions were held again in phase 9 after a use. The next decision can adopt this as a provisional choice.
3. **Generated Evidence parts come from tools and are checked.** The Split tests table joins the agreement sentence in `status_table.py`. Seven pages lack the table, and the 64 hand-made tables have no check.
4. **A fixed front-end binary for writer commits.** Each round leaves its `tidal-fe` at a path that the next rebuild does not touch, and `frontend_check.py` takes the binary path. Two phase 9 commits skipped the check, as two phase 8 commits did.
5. **Smaller per-commit similarity reports.** A commit's report lists the units of the files it changed, with the flag count, and the phase-close report checks the whole repository. Phase 9 added 39,105 report lines.
6. **Check results written by a tool.** A script runs the check chain and writes the Checks paragraph with the counts and the commit. Commit 9b1963e exists to record what 32c89f7 left out, and 1bf7812 states its skipped check twice in two forms.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commit 017b401, and the four writer probes. The round 6 comparisons come from the same scripts on commit 1b16115.

# Roadmap, round 5: what to do after phase 7, from an editor's view

Date: 2026-11-03. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and the round-4 editor report. In `documentation/` it read the charter, roadmap, status, style guide, specification format, exercises, decisions 0033, 0039, 0044, 0050, 0051 and 0054, and `decisions/pending.md`. It also read `specification/`, the headers of `tests/`, and `questions/` with its index. The phase 7 reports it read were, in `reports/clean-room/`, R-overloading, R-foundations reports C and D, R-exceptions, X13 to X16, rust-frontend report-15 to report-17, and `tests/programs/`. In `reports/review/` it read the 2026-11-03 gap reviews of the exceptions chapter, names, the object model, program structure, and the four question groups. It also read `git log --oneline 2798b37~1..HEAD` (22 commits). The reader did not read the other round-5 reports. Read-only scripts made the counts from the tree at commit 7d8c982. The phase 7 baseline is 1ab2175, whose specification, tests and questions match 3d98c93, the commit round 4 counted. Statements marked "general knowledge" do not come from Tidal's files.

The page table uses the round-4 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines in `reports/clean-room/` that cite a rule on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 4 (3d98c93) | Now (7d8c982) |
|---|---|---|
| Live rules | 1663, plus 15 withdrawn | 1676, plus 15 withdrawn. 13 rules are new: ATTR-20, CINIT-3, CLS-35, CNS-40, COND-16, DEFARG-10 to DEFARG-12, OBJ-12, ODR-15, PRIM-35, TR-16, UNW-11 |
| Test files | 6032, every one cited by a rule | 6337. 6270 are cited by a rule, and 67 are program tests under `tests/programs/` that no rule lists |
| Rules that rest on one test | 3 (TPL-48, INST-33, INST-36) | 8 (TR-16, OBJ-12, CINIT-3, UNW-11, DEP-37, TPL-48, INST-33, INST-36) |
| Tests with an override | 523 | 606 (6 of them program tests) |
| Overrides by label | EDG 211, GCC 198, Clang 108, Clang 18 64, GCC 14 36 | EDG 247, GCC 237, Clang 114, Clang 18 73, GCC 14 46, GCC 16 `-O2` 14, Clang 22 `-O2` 8, other labels 24 |
| Tests whose overrides name only an older version | 59 | 70 |
| Rules whose every test carries an override | 43 | 46 |
| Tests with `tidal-error` patterns | 2798 | 2916 |
| Questions | 352 | 376 |
| Open questions (provisional or open) | 310 | 209 |
| Settled, compiler defect | 30, 12 | 154, 13 |
| Front end | 5595 of 6032 pass | 6068 of 6337 pass |
| Evidence lines, whole specification | 5259 of 38199 (14%) | 6297 of 40041 (16%) |
| `tests/RESULTS.md` | 5609 rows, 426 tests missing | 6337 rows, none missing, none stale |
| `question_index.py --check` warnings | not run | 9 (46 when item R1 added the warning) |

Two of the 13 new rules rest on one test (OBJ-12 and UNW-11), and so do two more that also rest only on overridden tests (CINIT-3 and TR-16). Five of the new rules rest only on overridden tests (ATTR-20, CINIT-3, CNS-40, DEFARG-12, PRIM-35). DEP-37 fell to one test when Q-0247 moved a case to DEP-33. ADD-17 and MUL-12 left the override-only list.

The Evidence segments grew by 1038 lines, and the pages grew by 1842. So 56% of the new page text is Evidence.

### What phase 7 did to the questions

Each count below comes from each file's status at 1ab2175 and at 7d8c982.

- 24 new questions (Q-0353 to Q-0376). 21 are still open, 2 are settled, and 1 (Q-0356) is a compiler defect.
- 122 open questions settled. Q-0312 went from "open" to "provisional". No question reopened.
- Open questions went from 310 to 209, below the target of decision 0054.

The 124 settlements came from five commits:

| Commit | Settled |
|---|---|
| 0637b59, item SR | 34 |
| b00cbf2, repairs from the exceptions and names reviews | 1 |
| 573d849, repairs from the object-model and program-structure reviews | 13 |
| e0d06f0, question reviews of overloading and templates | 36 |
| 8c15b06, question reviews of classes, statements, expressions, and declarations | 40 |

Each settled question says "Settled on 2026-11-03 under decision 0054". Row 1 of `pending.md` asks the maintainer to confirm "the 110 questions settled" and counts only the three item commits. The 14 settlements in the two repair commits are missing from that row.

A script sorted the 209 open questions by the note that says why each stays open. One note can give several reasons, so the counts are approximate.

| Reason in the question's note | Questions |
|---|---|
| The review reads with doubt, is silent or unclear, or reads against the answer | 69 |
| No principle on the page gives the answer, or states the case | 53 |
| Two newest families differ from the answer in some form | 30 |
| Waits on the maintainer | 15 |
| The rule changed on the same day | 4 |
| Other (Q-0198) | 1 |
| A note that says only "stays open" (Q-0165, Q-0168, Q-0179, Q-0205, Q-0247, Q-0248, Q-0263, Q-0269, Q-0336) | 9 |
| No note in either form | 28 |

The 28 with no note are these:

- 8 process questions: Q-0006, Q-0008, Q-0116, Q-0119, Q-0182, Q-0183, Q-0186, Q-0312.
- Q-0007.
- Q-0129, Q-0299, Q-0334, and Q-0358, which a 2026-11-03 review contradicts.
- the 15 questions Q-0362 to Q-0376, which no review has read.

The 53 questions held by the principle condition sit on many pages. Alignment has 5, new-delete 4, and enumerations, attributes, and unions have 3 each. Another 15 pages have 2 or 1.

The principle condition is applied unevenly:

- 10 questions stay open because no principle "states the case".
- 9 questions settled on a principle that states only the general model: Q-0242, Q-0244, and Q-0352 on CTAD-P1; Q-0259 on CPT-P3; Q-0055 on CPY-P3; Q-0064 on PRIM-P1; Q-0067 on TYN-P1; Q-0111 on DEFARG-P2; and Q-0319 on SEQ-P2. The commits say so.
- Q-0329 settled on CE-P2, a principle of another page.

The splits grew:

- In 77 tests (63 in round 4), Tidal sides with one newest family against the other two. Clang is that family in 53, GCC in 15, and EDG in 9.
- Every one of the 77 names a question except `vendor-attribute-not-applied.cpp` (ATTR-20), whose note explains an implementation-defined choice.
- `replace-base-old-pointer-constant.cpp` (STG-19, Q-0307) still goes against all three families.

### Exercises in phase 7

The ten pages that no batch had reached got Predict batches without test names. They scored 839 of 840 and gave 46 findings (RPA to RPE). Gap reviews read the eleven pages that had none, and four more reviews read the 187 open questions that had none. Propose X13 to X16 gave 39 findings. The front end ran rounds 15 to 17 and recorded G118 to G133.

18 pages still have only a Predict batch with test names, from 2026-10-31 and 2026-11-01. They are the 7 Foundations pages of the first batch (translation, objects, values, names, conversions, evaluation order, constant evaluation) and the 11 templates pages. Their rows in `exercises.md` section 8 do not say "without test names", and the rows of every later batch do. Commit 9f9e62a says "Every page now has a Predict batch without test names". That sentence holds for "a Predict batch" and does not hold with "without test names".

### What phase 7 took from round 4

| Round-4 item | Outcome |
|---|---|
| 1. Settle by review | Done as item SR, then applied in item GR. Open questions went from 310 to 209 |
| 2. Repair the drifting records | Done as item R1. The six test directories were not moved |
| 3. One decision packet | Done (`pending.md`, 9 rows) |
| 4. Gap reviews for unreviewed questions | Done for every question that existed when item GR ran. 15 questions opened later have no review |
| 5. Predict on pages with no name-free batch | 10 of 28 pages done. The 18 above remain |
| 6. Write programs as tests | Done (67 programs) |
| 7. Rework the overloading pages | Not done. R/P is still 11.0 and 9.4. Evidence grew from 676 to 779 lines (32% of the chapter). Split tables replaced some bullets |
| 8. Thin evidence | Not done. Override-only rules rose from 43 to 46, and single-test rules from 3 to 8 |
| Process: check warnings for override-only rules and the 0044 proxy | Not done. This is the fourth round to ask |
| Process: a reading on every reviewed question | Done as a warning in `question_index.py --check`. 9 warnings remain |
| Process: a target for open questions | Done. Each settling commit states the counts before and after |
| Process: each new question names its finding | Not done. Q-0373 to Q-0376 name no source. Q-0357 came from a Predict finding (commit a9906a8) and names only a later review |

### Records that drift

These records disagree with the tree at 7d8c982, or with each other:

- `pending.md` row 1 says 110 settled questions, and 124 settled under decision 0054.
- `pending.md` row 4 is one table cell of about 2500 characters. It holds 21 questions and about a dozen rules with no question. The EDG report already has its own section, and row 4 does not.
- The status lines of decisions 0051 and 0054 do not say that their items are done. The line of 0050 does.
- The template in `documentation/decisions/README.md` lists only "Accepted" and "Superseded by". Three decisions are "Provisional", and the README does not mention `pending.md`.
- `specification-format.md` says tests mirror the `specification/` layout. Six pages still keep their tests elsewhere: both overloading pages under `tests/foundations/`, and constant evaluation, conversions, evaluation order, and values under `tests/expressions/`.
- `specification-format.md` says the Evidence segment has an agreement sentence, bullets, and an `Inputs:` line. Pages now also carry subsections and tables. "Split tests" appears on 4 pages, and "Sources", "Divergences", "Results where every compiler agrees", and "Probe grids" each appear on 2. The format describes none of them.
- `question_index.py --check` warns for 9 questions whose Opinions do not name a review that names them: Q-0007, Q-0021, Q-0081, Q-0146, Q-0181, Q-0188, Q-0203, Q-0307, Q-0319.
- 37 open questions lack a dated reason in the common form (the 28 and the 9 in the table above).
- The `exercises.md` rows for the first Foundations batch and the templates batches do not say whether the batch hid test names (above).

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | Other evidence |
|---|---|---|---|---|---|---|---|---|
| 1 | overloading/conversion-sequences | 47 / 133 | 13 | 123 | 27 | 397 / 1242 | 9.4 | The longest page, with 8 one-family tests (tied for the most). ICS-35 and ICS-44 rest only on overridden tests. Tests sit under `tests/foundations/` |
| 2 | overloading/overload-resolution | 44 / 176 | 11 | 232 | 27 | 382 / 1204 | 11.0 | The highest R/P. 7 one-family tests. 10 Evidence subsections. Q-0140 and Q-0142 are kept against the review |
| 3 | foundations/storage | 29 / 86 | 12 | 198 | 6 | 166 / 948 | 3.6 | The one test against all three families (Q-0307). Q-0357 and Q-0359 are new from Predict, and Q-0372 reaches STG-11 |
| 4 | expressions/new-delete | 42 / 142 | 10 | 144 | 22 | 167 / 902 | 6.0 | 4 questions held only by the principle condition. NEW-35 and NEW-36 rest only on overridden tests |
| 5 | classes/unions | 28 / 152 | 10 | 128 | 13 | 103 / 748 | 4.7 | New on this list. 3 questions held by the principle condition. Q-0372 is new and has no review |
| 6 | templates/constraints | 40 / 126 | 5 | 48 | 27 | 228 / 958 | 8.0 | CNS-18, CNS-32, CNS-38, and the new CNS-40 rest only on overridden tests. No batch without test names |
| 7 | templates/instantiation | 35 / 134 | 6 | 62 | 23 | 255 / 880 | 7.0 | INST-33 and INST-36 rest on one test. 6 one-family tests. No batch without test names |
| 8 | templates/dependent-names | 39 / 123 | 6 | 124 | 22 | 261 / 1048 | 6.5 | DEP-37 is newly single-test. DEP-27 and DEP-34 rest only on overridden tests. 6 `no-diagnostic` rules |
| 9 | foundations/constant-evaluation | 27 / 153 | 7 | 328 | 24 | 147 / 728 | 5.4 | 8 one-family tests (tied for the most). Tests sit under `tests/expressions/constant-expressions/`. No batch without test names |
| 10 | declarations/alignment | 8 / 48 | 5 | 46 | 22 | 56 / 261 | 2.7 | 46% of its tests have overrides, the highest share. 5 questions held by the principle condition, the most of any page |

Close behind:

- classes/members has Q 8 and R/P 8.5.
- templates/class-template-deduction has R/P 9.7 and F 43.
- templates/packs has R/P 10.3.
- declarations/attributes has Q 6, and ATTR-13, ATTR-18, and ATTR-20 rest only on overridden tests.

Since round 4, reference binding fell from 12 open questions to 4 and leaves the list. Open questions fell on constraints (15 to 5), instantiation (17 to 6), class template argument deduction (13 to 6), and dependent names (14 to 6). The two overloading pages stay at the top, and their R/P has not changed in two rounds.

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | **Make the principle condition one test, then close the principle gaps.** Define in the style guide when a principle "gives the answer", apply the definition to the 19 questions on either side, then write or extend principles for the rest | 53 open questions held by the principle condition. 9 questions settled on a general principle while 10 stay open for lack of one that states the case | two sessions | medium | Up to about 50 settlements. Pages 4, 5, and 10 |
| 2 | **Repair the records.** Fix the `pending.md` count, split row 4 into a section, update the decision status lines and the README template, mark the old Predict rows, clear the 9 warnings, and give the 37 open questions a dated reason | Section 1, "Records that drift" | under one session | low | A packet the maintainer can act on. Every later count |
| 3 | **Predict on the 18 pages with only named batches**, first constraints, instantiation, dependent names, and constant evaluation | 18 pages, about 1870 tests. Their batches date from before phases 6 and 7 changed the pages | three sessions | low | Findings on pages 6 to 9 |
| 4 | **Thin evidence and the two check warnings.** Give each override-only rule a test that every compiler agrees on, or a note, and give each single-test rule a second test. Add the warnings, and make a new rule fail the check when it has only one test or only overridden tests | 46 override-only rules and 8 single-test rules, 7 of them new or changed in phase 7. Fourth round to ask for the warnings | one session | low | Rules that do not rest on a disputed test |
| 5 | **Review the 15 unreviewed questions and the 13 new rules** | Q-0362 to Q-0376 have no review reading. The new rules came after the gap reviews | one source-reader session, one writer session | low | The settling rule on these questions |
| 6 | **Write the Evidence format down, then slim the overloading Evidence.** Describe the subsections and split tables in `specification-format.md`, use one layout on every page, and replace each settled-question bullet with one line | 56% of page growth was Evidence. Five subsection layouts appear on 4 or fewer pages. Overloading Evidence is 32% of the chapter | one session | low | Readable overloading pages. Item 7 |
| 7 | **Rework the overloading pages and move the six test directories** | R/P 11.0 and 9.4, unchanged since round 4. Six directories break the stated layout | one to two sessions | medium | A reader can find a tie-breaker. The Predict batches of those pages stay useful |

### 2.1 Make the principle condition one test, then close the principle gaps

Decision 0054 settles a question when the review supports the answer with no doubt "and a principle on the page gives the same answer". The writers read that condition in two ways:

- In e0d06f0 and 8c15b06, a principle that states the general model was enough for 9 questions, and a principle of another page was enough for Q-0329.
- In 10 other questions the note says that no principle "states the case".

The editor proposes one test, stated in the style guide's section on principles. A principle gives the answer when a reader who applies the principle's model to the question's program reaches the provisional answer without the rule text. A principle on another page counts when the affected page cites it. The writer then rechecks the 19 questions under that test. Each one either settles or reopens, and a note records the test.

After that, the 53 held questions fall into two kinds:

- Some have a principle that misses the case because the model has a hole. There the writer extends the principle, and the question settles if the extended model gives the answer.
- Some sit on rules that are exceptions to their principle, such as TPL-37 for Q-0196. There the page should name the exception in the principle's `:exceptions:` line, and the question stays open.

The risk is that a writer bends a principle to settle a question. A source reader's gap review of the changed principles keeps the risk down, and a contributor can reopen any settled question.

### 2.2 Repair the records

Each step below fixes one record in section 1:

- In `pending.md` row 1, say 124 settled questions and name the two repair commits. Better, have `tools/question_index.py` write the count.
- Move row 4 into a section like the EDG report, with one row per question or rule, its side, and the review that reads it. Row 4 in the main table then becomes one line.
- Add "All items are done" to the status lines of decisions 0051 and 0054. Add "Provisional" to the README template and one sentence on `pending.md`.
- In `exercises.md` section 8, add "with test names" to the rows of the first Foundations batch and the three templates rows.
- Record the review reading on the 9 questions that `question_index.py --check` names.
- Give each of the 37 open questions with no dated reason a note in the common form. "Kept open on DATE under decision 0054, because ..." is the form most files use.

### 2.3 Predict on the 18 pages with only named batches

The 11 templates pages hold four of the ten weakest pages. The first Foundations batch read the specification under its older paths, `spec/` and `docs/`. On the phase 7 rate of 46 findings in 840 answers, about 1870 tests would give about 100 findings. That number is a projection. Constraints, instantiation, dependent names, and constant evaluation go first, because they are on the weak list.

### 2.4 Thin evidence and the two check warnings

The 46 override-only rules are CE-13, CE-18, CE-28, PRIM-15, PRIM-35, DTY-5, UNEV-4, PFX-15, NEW-35, NEW-36, TYN-5, TYN-9, ALN-7, REF-15, ATTR-13, ATTR-18, ATTR-20, DEFARG-12, EXS-5, EXS-6, CINIT-3, CLS-26, CTOR-14, CTOR-17, CTOR-18, DCMP-8, ICS-35, ICS-44, TPL-42, TPL-48, TARG-22, TSP-17, TSP-21, ORD-7, DEP-27, DEP-34, CPT-3, CPT-15, CNS-18, CNS-32, CNS-38, CNS-40, PACK-30, CTAD-13, TRY-12, and TRY-13. Some of them (ATTR-20, TRY-12) are implementation-defined or recommended, and an override is how each test records the choice. Each such rule needs a note that says so. Each of the others needs a test where every compiler agrees.

Phase 7 shows the cause. Rules changed by a review or a question (CINIT-3, CNS-40, DCMP-8, CTOR-17, TARG-22) arrive with the test that showed the split and no other. Two changes to `check_specification.py` would catch this at the commit:

- a warning for every override-only or single-test rule,
- a failure when a rule added or reclassified in the commit has either property.

### 2.5 Review the 15 unreviewed questions and the 13 new rules

Q-0362 to Q-0376 came from the gap-review repairs, X13, and the front-end findings G118 to G133. Most of them go back to a review, but none carries a review reading. The 13 new rules have no review of their own text. One source-reader session can read both, grouped as phase 7 grouped its question reviews. The settling rule then applies.

### 2.6 Write the Evidence format down, then slim the overloading Evidence

The Predict findings asked for tables per split, and the writers added them (commit a9906a8). The format did not follow. The editor proposes these parts for the Evidence segment in `specification-format.md`, in this order:

1. the agreement sentence,
2. "Divergences", one line per open question with its sides,
3. "Split tests", a table of test, rule, and each family's result,
4. "Probe grids", when a page has them,
5. `Inputs:`.

A settled question keeps one line under Divergences that names the question and the family that still differs. The question file keeps the probes and the reasoning. `tools/check_specification.py` can check the order of the subsection headings.

### 2.7 Rework the overloading pages and move the six test directories

This item carries round 4's item 7 forward unchanged:

- Split the principles so that none covers more than about six rules.
- Move `tests/foundations/conversion-sequences/` and `tests/foundations/overload-resolution/` to `tests/overloading/`. Move the four other directories to their page's path.

A move changes the cache keys and the front-end baseline (general knowledge about caches keyed by path). So the move runs in a phase that already plans a full matrix run, and item 3's batches on these pages run after the rework.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phase 7 added 13 rules and no page |
| A third settling rule for questions where two newest families differ | Leave it to the maintainer (`pending.md` row 1 already asks) | 30 open questions are of this kind. A clean-room writer should not widen the rule on its own |
| Language after C++23 and the index of later acceptances (`pending.md` row 7) | Keep out | Still 209 open C++23 answers. Q-0087, Q-0181, and Q-0188 show later changes reaching C++23 answers |
| Diagnose batches beyond D-classes, and Port suites | Defer until item 3 is done | Predict still gives about one finding per 18 answers |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 and 0054 say | Nothing in phase 7 argues for them. 32 *(referenced)* markers name them |
| Library traits and builtin queries in tests | Keep out | Q-0343 waits on the maintainer (`pending.md` row 6) |

## 4. Process changes

- **One test for "a principle gives the answer"**, in the style guide (item 1). Every settling note names the principle and says whether it states the case or the model.
- **A required reason line for open questions.** A header field such as `**Open because:**` with a date replaces the free notes. `question_index.py --check` fails when a provisional question has no such line, or when the line is older than the latest review that names the question.
- **Each new question names its source.** A `**Source:**` header line names the finding, review, or report. `question_index.py --check` fails when the line is missing. Round 4 asked for this, and phase 7 opened 24 questions without it.
- **Packet counts come from tools.** `question_index.py` writes the counts in `pending.md` (questions settled under each decision, questions that wait), so a row cannot drift from the files.
- **Check warnings become failures for new rules** (item 4). This is the fourth round to ask.
- **Review warnings fail at a phase's end.** `question_index.py --check` already warns. The closing commit of a phase should run it in a mode that fails on any warning.
- **Coverage claims come from a tool.** `tools/exercises.py` can list the pages with no Predict batch without test names, and a closing commit cites its output instead of a sentence. The same tool can mark each `exercises.md` row as named or name-free.
- **A phase's closing commit updates its decision's status line**, and `check_specification.py` can fail when the roadmap marks every row of a phase done and the decision's status line does not say so.

## 5. Files

The reader made the counts with read-only scripts in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/rm5/editor/`: `count.py` for rules, tests, overrides, and Evidence; `q.py` for question movement and review coverage; `op.py` and an inline script for the reasons questions stay open; `pages.py` for the page table; and the lists `kept.txt` and `noprinciple.txt`.

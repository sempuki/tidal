# Roadmap, round 6: what to do after phase 8, from an editor's view

Date: 2026-11-04. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and the round-5 editor report. In `documentation/` it read the charter, roadmap, status, style guide, specification format, exercises, decisions 0033, 0039, 0044, 0050, 0051, 0054 and 0055, and `decisions/pending.md`. It also read `specification/`, the headers of `tests/`, and `questions/` with its index. The phase 8 reports it read were, in `reports/clean-room/`, R8 (reports a to d), I1, W16, W17, X17 to X20, rust-frontend report-18 and report-19, and `tests/programs/`. In `reports/review/` it read the two 2026-11-04 gap reviews of the text phase 7 wrote. It also read `git log --oneline dc90663~1..HEAD` (18 commits). The reader did not read the other round-6 reports. Read-only scripts made the counts from the tree at commit 1b16115. The phase 8 baseline is 229d192, whose specification, tests and questions match 7d8c982, the commit round 5 counted. Statements marked "general knowledge" do not come from Tidal's files.

The page table uses the round-5 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines in `reports/clean-room/` that cite a rule on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 5 (7d8c982) | Now (1b16115) |
|---|---|---|
| Live rules | 1676, plus 15 withdrawn | 1681, plus 15 withdrawn. 5 rules are new: ACC-20, CONV-21, DEFARG-13, LSP-8, PFX-22. No rule changed class |
| Rule and principle text changed in the phase | not counted | 111 rules and 66 of 343 principles changed their prose, leaving out `:tests:`, `:rules:` and `:exceptions:` lines |
| Test files | 6337 (67 program tests) | 6500. 6429 are cited by a rule, and 71 are program tests |
| Rules that rest on one test | 8 | 0 |
| Rules that rest only on tests with an override for a newest compiler | 46 (any override) | 29 by the check's definition. 16 of them have a note that their class allows the split. 13 have no program that the newest compilers agree on |
| Tests with an override | 606 | 634 (6 of them program tests) |
| Overrides by label | EDG 247, GCC 237, Clang 114, Clang 18 73, GCC 14 46, GCC 16 `-O2` 14, Clang 22 `-O2` 8, other 24 | EDG 264, GCC 244, Clang 120, Clang 18 74, GCC 14 49, GCC 16 `-O2` 14, Clang 22 `-O2` 8, other 24 |
| Tests whose overrides name only an older version | 70 | 74 |
| Tests with `tidal-error` patterns | 2916 | 2981 |
| Questions | 376 | 385 |
| Open questions (all provisional) | 209 | 221 |
| Settled, compiler defect | 154, 13 | 151, 13 |
| Front end | 6068 of 6337 pass (the old harness) | 6183 of 6500 pass, and 6247 pass their check (the harness of item HN) |
| Evidence lines, whole specification | 6297 of 40041 (16%) | 5997 of 41006 (15%) |
| `tests/RESULTS.md` | 6337 rows, none missing | 6500 rows, none missing, none stale |
| `question_index.py --check` | 9 warnings | 0 problems. A missing review reading now fails the check |
| `check_specification.py` warnings | not counted | 30: one for each of the 29 thin rules, and one line for 68 pages in the older Evidence form |
| Page examples (`check_examples.py`) | no tool | 197 blocks on 8 compilers, 0 problems |

The pages grew by 965 lines, and Evidence fell by 300 lines.

Splits grew a little. In 83 tests (77 in round 5), Tidal sides with two newest families against the third. Clang is the odd family in 57, GCC in 17, and EDG in 9. Every one names a question except two whose notes explain an allowed result (`vendor-attribute-not-applied.cpp` under ATTR-20, and `override-independent-base-unused.cpp` under DEP-30, a no-diagnostic rule). `replace-base-old-pointer-constant.cpp` (STG-19, Q-0307) is still the one test that goes against all three families.

### What phase 8 did to the questions

Each count comes from each file's status at 229d192 and at 1b16115.

- 9 new questions (Q-0377 to Q-0385). All 9 are open, and each has a `**Source:**` line.
- Item PQ reopened 8 settled questions under the principle test: Q-0064, Q-0067, Q-0111, Q-0242, Q-0259, Q-0319, Q-0329, and Q-0373. Q-0373 had settled earlier in the phase.
- 5 questions settled, so 4 settled at the end: Q-0076, Q-0363, Q-0365, and Q-0367.
- Open questions went from 209 to 221. Decision 0055 aimed below 209. The roadmap's phase 8 section states the miss and its causes.

A script sorted the 221 open questions by what holds each one. A question goes in the first group that fits, so the counts are approximate.

| What holds the question | Questions |
|---|---|
| A principle that phase 8 widened, which no reader or reviewer has used since | 59 |
| A row of `pending.md` (the maintainer) | 54 |
| Two newest families differ from the answer | 16 |
| The 2026-11-04 gap review reads against the answer, or is unclear | 10 |
| New in phase 8, with no review | 9 |
| An older review reading with doubt, silence, or a reading against the answer | 73 |

The 59 questions sit on 51 principles. Alignment holds 5, new-delete 4, and unions and attributes 3 each. The R8 batches ran after item PQ and read three of the widened principles (CNS-P2, INST-P1, INST-P3). Q-0235, Q-0253, and Q-0264 wait on those three. Q-0235 also waits on `pending.md` row 5.

Q-0337 is open and appears in no file of `reports/review/`. The nine new questions also appear in none.

### Exercises in phase 8

- R8 ran Predict without test names on eight pages (constant evaluation, evaluation order, constraints, instantiation, dependent names, storage, unwinding, names). The batches scored 979 of 982 and gave 39 findings (R8A to R8D).
- I1 built class layout from the rules alone. It matched 241 of 243 `static_assert` checks, 24 of 24 run checks, and 22 of 22 checks of older-version and EDG variants.
- W16 and W17 wrote four larger programs. X17 to X20 proposed changes on P1, P2, P17, and P18. The front end ran rounds 18 and 19.

12 pages still have only a Predict batch with test names. They are 4 Foundations pages (translation, objects, values, conversions) and 8 templates pages (arguments, class template deduction, concepts, declarations, deduction, ordering, packs, specialization). They cite 1025 tests.

### What phase 8 took from round 5

| Round-5 item | Outcome |
|---|---|
| 1. One test for the principle condition, then close the gaps | Done as item PQ. The style guide has the test. 8 questions reopened, and 52 principles were widened. The 59 questions on them wait for a reader or reviewer |
| 2. Repair the records | Done as item R2, except two parts. `pending.md` row 4 is still one table cell, now about 3400 characters (about 2500 in round 5). 36 open questions have no reason in the common dated form |
| 3. Predict on the 18 pages with only named batches | 6 of 18 done in R8. 12 remain |
| 4. Thin evidence and the two check warnings | Done as item TE. No rule rests on one test. The check fails when a commit adds or reclassifies a thin rule |
| 5. Review the unreviewed questions and the new rules | Done as item RV, with two gap reviews. The text phase 8 wrote has no review |
| 6. Write the Evidence format down | Done as item V. 10 pages use the format. 68 pages keep the older form, and the check warns once for them |
| 7. Rework the overloading pages and move the test directories | The six directories moved (item L). The overloading Evidence fell from 7962 to 1911 words and from 5257 to 1909 words. The principles were not split. R/P is still 11.0 and 9.4, now for a third round |
| Process: a reason line for open questions | Not done. Reasons are still free notes |
| Process: each new question names its source | Done. 9 of 9 new questions have the line |
| Process: packet counts from tools | Done. `question_index.py` writes the row 1 total and fails on a wrong one |
| Process: warnings fail for new rules, and review warnings fail | Done |
| Process: coverage claims from a tool | Not done. The drift below shows the cost |
| Process: a closing commit updates the decision's status line | Done. Decisions 0050, 0051, 0054, and 0055 say "All items are done" |

### Records that drift

These records disagree with the tree at 1b16115:

- `exercises.md` section 7 says "18 pages have only a batch that showed test names". The rows of section 8 give 12.
- `pending.md` row 4 holds about 30 questions and rules in one cell, and grew in phase 8 by the RV items.
- 36 open questions carry no reason in the forms "Kept open on DATE", "Reopened as provisional on DATE", or "stays provisional until". 27 of them carry an older form ("Kept on", "Changed on"). The 9 new questions, Q-0377 to Q-0385, carry no reason at all.
- `reports/clean-room/I1/report.md` says in its header that the message is "the only copy of the report". The report is committed.
- The 2026-11-03 notes "no principle on the page gives the answer" still stand in about 48 question files. A later 2026-11-04 note in the same files says that item PQ widened the principle. A reader who reads the first note alone gets the old state.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | Other evidence |
|---|---|---|---|---|---|---|---|---|
| 1 | overloading/conversion-sequences | 47 / 133 | 13 | 137 | 27 | 178 / 1024 | 9.4 | ICS-35 and ICS-44 still rest only on overridden tests. OVL-P2 and OVL-P4 were widened and hold 2 questions. New Evidence form |
| 2 | foundations/storage | 29 / 93 | 12 | 248 | 6 | 158 / 1012 | 3.6 | Q-0307, the one test against all three families. New Evidence form |
| 3 | overloading/overload-resolution | 44 / 179 | 11 | 253 | 28 | 189 / 1029 | 11.0 | The highest R/P, unchanged for three rounds |
| 4 | expressions/new-delete | 42 / 144 | 10 | 158 | 23 | 183 / 938 | 6.0 | NEW-P1 to NEW-P4 widened, holding 4 questions. NEW-36 rests only on overridden tests. Older Evidence form |
| 5 | classes/members | 34 / 161 | 10 | 151 | 15 | 126 / 792 | 8.5 | New on this list. Q-0381 from I1. Older Evidence form |
| 6 | classes/unions | 28 / 158 | 10 | 156 | 13 | 110 / 796 | 4.7 | UNI-P1, UNI-P4, and UNI-P6 widened, holding 3 questions. Q-0384 from I1. Older Evidence form |
| 7 | foundations/constant-evaluation | 27 / 155 | 7 | 359 | 24 | 146 / 774 | 5.4 | The most report mentions. R8A gave 10 findings on it and evaluation order |
| 8 | templates/class-template-deduction | 29 / 117 | 7 | 46 | 21 | 208 / 843 | 9.7 | No batch without test names. The lowest F of the list. Older Evidence form |
| 9 | templates/constraints | 40 / 131 | 6 | 75 | 28 | 183 / 985 | 8.0 | CNS-18 and CNS-38 rest only on overridden tests |
| 10 | declarations/alignment | 8 / 49 | 5 | 56 | 23 | 63 / 291 | 2.7 | 47% of its tests carry an override. All 5 of its open questions wait on ALN-P1 or ALN-P3 |

Close behind are templates/instantiation (Q 6, Ev 259 of 991, the largest Evidence share outside the list) and templates/packs (R/P 10.3).

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | **Review the text phase 8 wrote, and use the widened principles.** A source-reader gap review of the 66 changed principles, the 5 new rules, the 111 changed rules, and Q-0377 to Q-0385. Then the writer applies the settling rule | 59 open questions wait on a principle that no one has used. No review has read phase 8's text | one source-reader session, one writer session | low | Up to 59 settlements. Pages 4, 6, and 10. A review reading on the 9 new questions |
| 2 | **Repair the records, and generate the claims that drift** | Section 1, "Records that drift" | under one session | low | Counts that hold. Item 1's notes in one form |
| 3 | **Move the 68 pages to the Evidence format**, then make the warning a failure | 68 of 85 pages keep the older form. Pages 4, 5, 6, 8, and 10 among them | two sessions | low | One layout for readers. The divergence lines that item 1 adds go in one place |
| 4 | **Predict on the 12 pages with only named batches**, first class template deduction | 12 pages, 1025 tests. Their batches predate phases 6 to 8 | two sessions | low | Findings on page 8 and the templates chapter |
| 5 | **Split the overloading principles** | R/P 11.0 and 9.4, the third round that asks | one session | medium | A reader can find a tie-breaker. Runs after item 1 |

### 2.1 Review the text phase 8 wrote, and use the widened principles

Decision 0055 holds a question that rests on a phase 8 principle until "a reader or reviewer has used that principle". The status page lists this as its second next step and names no exercise that does it. One gap review can do both jobs:

- The review reads each of the 51 widened principles that hold a question against the case of each question it holds. A principle used this way meets the decision's condition.
- The review reads the 5 new rules (ACC-20, CONV-21, DEFARG-13, LSP-8, PFX-22) and the 111 rules whose prose changed, as RV did for phase 7.
- The review reads Q-0337 and Q-0377 to Q-0385, which no review names.

The writer then settles each question whose principle passes the style guide's test and whose review supports the answer. Q-0235, Q-0253, and Q-0264 can be checked first against the R8 answers, because R8 read CNS-P2, INST-P1, and INST-P3 after item PQ. A phase 9 target of open questions below 209 is reachable only through this item, since 59 of the 221 wait on it.

The risk is a review that finds a widened principle wrong. In that case the principle gets a question of its own, and its held questions stay open.

### 2.2 Repair the records, and generate the claims that drift

Each step fixes one record in section 1:

- Make `tools/exercises.py` (or `status_table.py`) write the count of pages with only named batches into `exercises.md` section 7, and fail the check when the sentence is stale. Today the sentence says 18 and the table gives 12.
- Move `pending.md` row 4 into a section like the EDG report, one row per question or rule, with the side each family takes and the review that reads it. Round 5 asked for this too.
- Give each of the 36 open questions without a dated reason one line in the common form. Give Q-0377 to Q-0385 their first line after item 1's review.
- Mark the 2026-11-03 "no principle" notes in those files as replaced by the 2026-11-04 note, or move the older note under a dated history heading.
- Remove the "only copy" sentence from the I1 report header, or add a line that says where the report is saved.

### 2.3 Move the 68 pages to the Evidence format

Item V wrote the format and converted 10 pages. On those pages Evidence fell from 28280 to 13581 words (commit 79f6d82). The other 68 pages name questions among their note bullets. The work is mechanical, page by page:

1. Put each question the page names in one Divergences line with its rules, status, answer, and the family that differs.
2. Move each split test into the Split tests table.
3. Keep the probes and reasoning in the question file.

When the last page moves, `check_specification.py` turns the warning into a failure. Item 1's settling notes should land in the new layout, so item 3 runs on a page before or with item 1's writer pass on that page.

### 2.4 Predict on the 12 pages with only named batches

Class template deduction goes first, because it is on the weak list and has the lowest F there. The other templates pages follow, then the four Foundations pages. The first Foundations batch read the specification under its older paths. R8 scored 979 of 982 with 39 findings, so 1025 tests would give about 40 findings. That number is a projection.

### 2.5 Split the overloading principles

Phase 8 moved the overloading tests and cut their Evidence from 13219 to 3820 words. The principles stay as they were: 5 principles for 47 rules, and 4 for 44. The editor proposes no principle that covers more than about six rules. OVL-P2 and OVL-P4 were widened in phase 8 and hold 2 questions. Under decision 0055 a rewritten principle restarts the wait for its questions, so this item runs after item 1 has used OVL-P2 and OVL-P4.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phase 8 added 5 rules and no page |
| The 13 thin rules with no program that the newest compilers agree on | Leave as warnings | Item TE probed each one and named the forms in Evidence. A further probe would repeat that work |
| Settling questions where two newest families differ | Leave to the maintainer (`pending.md` row 1) | 16 open questions are of this kind. A clean-room writer should not widen the rule on its own |
| Later language versions | Keep out until `pending.md` row 7 is decided | The plan is costed and waits on the maintainer |
| Diagnose batches beyond D-classes, Port suites, and guides on the site | Defer | Items 1 to 4 give more findings per session. Decision 0055 already lists them as later work |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051, 0054, and 0055 say | Nothing in phase 8 argues for them. 33 *(referenced)* markers name them |

## 4. Process changes

- **A settling exercise for each phase's principles.** A phase that widens or writes principles names, in its decision, the reader batch or review that will use them. Phase 8 widened 52 principles and named no such step, so 59 questions wait.
- **A required reason line for open questions.** A header field such as `**Open because:**`, with a date, replaces the free notes. `question_index.py --check` fails when a provisional question has none, or when the line is older than the latest review or note that names the question. Round 5 asked for this, and 36 open questions still lack the common form.
- **Coverage claims come from a tool.** `exercises.md` section 7's count and the list of pages with only named batches come from a script, and the check fails on a stale count. Round 5 asked for this, and phase 8 left a stale "18" behind.
- **A format warning becomes a failure on a date.** The 68-page Evidence warning gets an end phase in the roadmap. A new page or a page that a commit converts must use the new form.
- **`pending.md` rows stay short.** A row longer than a few hundred characters moves to its own section. `question_index.py --check` can warn on a long cell.
- **The open-question target counts what holds each question.** A phase decision states the target together with the number of questions that its items can reach, so a missed target is visible at the start.

## 5. Files

The reader made the counts with read-only scripts in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/rm6/editor/`: `count.py` for rules, tests, overrides, and Evidence; `q.py` for question movement and review coverage; `pages.py` for the page table; inline scripts for the split tests, the rule and principle changes since 229d192, what holds each open question, and the pages with only named batches; and the lists `thin.txt` and `waitpq.txt`.

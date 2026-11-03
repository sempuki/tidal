# Roadmap, round 4: what to do after phase 6, from a clean-room writer's view

Date: 2026-11-03. Decision 0029, with decisions 0033, 0039, 0044, 0050, and 0051. A clean-room agent in the writer's role prepared this report. It read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, style guide, specification format, exercises, pipeline, and the decisions named above, with 0052 and 0053), `reports/clean-room/roadmap-3/writer.md`, the specification page list and segment sizes, the test headers, `questions/README.md` and the status lines of every question file, `tools/`, the phase 6 reports under `reports/clean-room/` (front-end reports 10 to 14 and the exercise status rows for R-*, W1 to W15, and X9 to X12), the headings and result sections of the 2026-11-01 and 2026-11-03 files under `reports/review/`, the unit names in the committed similarity reports, and `git log 39e9322..HEAD` with commit bodies. It did not read the other round-4 reports or `inputs/`. All counts come from read-only `grep`, `awk`, `git`, and short Python scripts kept outside the repository, run on the tree at commit 3d98c93. The report wrote no repository files.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 5 (round 3) | Now (3d98c93) |
|---|---|---|
| Rule entries | 1633 (1 withdrawn) | 1678 (15 withdrawn), so 1663 live |
| Specification words | 205,081 | 234,251 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 5408 | 6032 |
| Single-test rules | 297 | 3 |
| Test files with a per-compiler override | 461 | 523 (652 override lines) |
| Test files with an EDG override | 172 | 214 |
| `undefined` rules with a `run-fail` test | not counted | 43 of 56 |
| Tests with `tidal-interpreter: run-fail` | none | 4 |
| Question files | 321 | 352: 309 provisional, 1 open, 12 compiler defect, 30 settled |
| Index answers of the form "option N" | 65 | 0 (`check_specification.py` fails on one) |
| *(referenced)* markers | 21 | 29 (library 17) |
| Front end (`conformance.tsv`) | 4439 of 5408 pass | 5595 of 6032 pass (212 missed rejects, 122 differ from EDG only, 112 run-unsupported, 106 unsupported, 6 false rejects, 1 missed undefined run) |
| Committed similarity reports | 110 | 141 (31 in phase 6, all "0 flagged") |

Phase 6 added 31 live rules, 624 tests, and 31 question files. The writers acted on about 350 reader findings: Write W1 to W14 (107), Predict REA to RCD (158), Propose X9 to X12 (49), front end G85 to G117 (33), and the designer's batch. They also acted on six question-cluster gap reviews and the item O gap review.

### 1.2 What phase 6 took from round 3

| Round 3 item | Phase 6 result |
|---|---|
| R0, records from tools | Done (4d804bd). `status_table.py` writes the prose numbers, `question_index.py` builds the index, `number_questions.py` numbers `Q-NEW` placeholders, `check_all.sh` runs similarity with `--fail-on-flag` and no pipe, and Predict scores the cited rule. |
| Q, retire questions | Done as six clusters. Settled questions rose from 12 to 30. Open questions still rose from 289 to 310. |
| E, reader coverage | Done for Write (W1 to W14) and for Predict on 57 more pages. Diagnose stays at 4 pages. |
| GR, gap reviews of pages that never had one | Left. Only question clusters and item O had gap reviews. |
| H, hotspot pages | Done (aae6c9e). It merged 12 rules into others, and item C withdrew 2 more (bf171c6). |
| T2, a second test for every rule | Done (4a83577, 275 new tests). 3 single-test rules remain, all in templates. |
| FE10 | Done as rounds 10 to 14. |
| X2, digest the overrides | Left. Phase 6 added 138 override lines and removed 11. |
| O, class properties | Done (3249100, 645fac2). |
| Tool list items 1 to 7 | Done. Item 8 (list-order check) and item 9 (one results file per chapter) were left. |

### 1.3 Where the writer's time went

Phase 6 had 79 commits. Thirteen were repair commits from reader findings. Together they touched 190 specification files and added 218 tests. The largest were d58799c (Write W2 to W12: 29 pages, 52 tests), 0bf1a54 (Write W1 to W14: 29 pages, 47 tests), and bddd2a0 (Propose X9 to X12: 25 pages, 38 tests). The question clusters touched 10 to 35 question files per commit. Item C's consistency pass touched 82 pages. Repairs remain the main cost, and they are productive.

Four commits exist only to correct an earlier commit message: 0cc80a1, 76bd2a0, d7e90d0, and 0480c72. Each corrects a number or a check result that a person typed: a test count (40 for 38), a probe count, a site build result, and a front-end check whose failure a pipe into `tail` hid. AGENTS.md forbids that pipe.

Two commits record that the integrator's `frontend_check.py` run could not finish or gave a wrong result because a front-end round was rebuilding `tidal-fe` at the same time (ce617b7, 0cc80a1). Front-end report 14 records that the test set grew from 5998 to 6026 during its round.

### 1.4 Records that drift again

R0 made the status page's numbers come from tools. Three records still drift.

- **The full-matrix results are stale.** `tests/RESULTS.md` was last written at bddd2a0. It lists 5609 tests. The tree has 6032. 426 tests are missing from the file, and 3 rows name tests that no longer exist. The status page says "5609 of 5609 tests pass on each", which `status_table.py` reads correctly from a stale file, so `check_specification.py` passes. Each commit after bddd2a0 ran only its changed directories, which is the right habit, but no step regenerates the file. Phase 5 ended with a full run (57b828e). Phase 6 did not.
- **Hand-kept rows outside the status page.** The front-end row of `documentation/exercises.md` still says "round 10 done: 4918/5480 pass". In `documentation/roadmap.md`, the phase 6 rows R0, G, W, P, and D are not marked done, though their commits are in the log. The size estimate on the status page still counts 66 section-units, and the specification has 85 pages.
- **No committed similarity report checks a question file.** Phase 6 committed 31 reports. Each holds the specification and the guides, and none holds a unit from `questions/`. Only two earlier reports do, both dated 2026-10-31. Commit bodies say that the questions were checked, for example 3d98c93: "flags no passage in the changed pages and questions". The committed report of that commit holds no question unit. Of the five flags that phase 6 commit bodies mention, three were in question files (Q-0254, Q-0255, Q-0260 in b84b480 and ba19f11). 122 question files now record a source-reader gap review's reading in the writer's words. Questions are now the text with the most review-derived content and the least similarity record.

### 1.5 Questions

- 310 questions are open. A script over the test notes maps each open question to the compilers whose override names it:
  - 255 have a test where the newest version of some family differs from the expectation. GCC alone: 75. EDG alone: 65. EDG and GCC: 37. Clang alone: 34. Clang and EDG: 22. Clang and GCC: 12. All three: 10.
  - 10 have overrides only for Clang 18 or GCC 14 (Q-0020, Q-0055, Q-0067, Q-0076, Q-0081, Q-0181, Q-0228, Q-0275, Q-0297, Q-0342). Under decision 0051 these can settle, except Q-0020 (a policy) and Q-0181 and Q-0275 (on the maintainer's list).
  - 45 have no override that names them. These are classification questions, where every compiler accepts or rejects alike and the question is about the rule's class or wording. Decision 0051's settling rule does not cover them.
- 197 provisional questions mention no gap review. By page, the largest groups are reference binding (11), raw storage (9), new and delete (9), overload resolution (7), implicit conversion sequences (7), and explicit and partial specialization (7).
- The status page lists seven groups of questions that wait on the maintainer, including the EDG report (30 questions of cluster 5) and seven questions where a gap review reads the text against every compiler.

### 1.6 Coverage by readers and reviewers

- **Gap reviews.** By the headings of `reports/review/`, no source-reader gap review has read the three exceptions pages (41 rules) or eight Foundations pages: names (42), storage (29), use and the one-definition rule (19), translation (15), fundamental types (14), language-support types (13), objects (11), and keywords (4). That is 188 rules, about 11% of the live rules, and it includes raw storage, which has more `undefined` rules (7) than any other page.
- **Predict.** 75 of 85 pages have had a batch. The ten without one are the three exceptions pages, the two overloading pages, and five Foundations pages (types, language-support, keywords, storage, odr). Phase 6 batches scored 3262 of 3272 on outcome and 3186 on cited rule, and gave 158 findings.
- **Diagnose.** 4 of 85 pages (virtual, handlers, unions, conversion functions).
- **Port.** No Port suite has run.

### 1.7 Hotspots after consolidation

Lines in phase 6 reports (roadmap reports excluded) that name a page's rules:

| Page | Lines | Rules |
|---|---|---|
| constant expressions | 182 | 32 |
| raw storage | 121 | 29 |
| constructors and destructors | 106 | 34 |
| language-support types | 92 | 13 |
| copy and move | 85 | 28 |
| unwinding | 82 | 10 |
| new and delete | 81 | 42 |

Most of these lines come from reports written before the consolidation commit aae6c9e. No reader has used the consolidated constant expressions, evaluation order, or unwinding pages since.

### 1.8 Size and Evidence

The two overloading pages are the largest (11,520 and 9,484 words). Item C reorganized their Evidence into four parts and removed no fact, and both pages grew slightly. Evidence holds 7,199 of the 11,520 words on `conversion-sequences.md`. Across the specification, Evidence holds 54,133 of 234,251 words (23%).

### 1.9 Similarity

Similarity is still cheap where it runs. Phase 6 commit bodies record five flagged units, each reworded before the commit. The cost is in the record (section 1.4) and in volume. Each of the 31 phase 6 reports checks the whole specification at about 2,400 lines, for 71,913 lines in phase 6 and 8.6 MB under `reports/similarity/`.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | R1. Close the phase 6 record | 426 tests outside `RESULTS.md`; no committed report checks a question; hand-kept rows on three pages | 0.5-1 | none, unless the first question report flags | an honest base for the phase 7 decision |
| 2 | GR2. Gap reviews of the 11 pages that never had one, with their questions | 188 rules unreviewed; round 3 rank 4, left by phase 6 | 2-3 of repairs | medium | the same review level on every page; questions on those pages can settle |
| 3 | Q2. Settle by divergence, with a generated divergence column | 310 open; 10 old-version-only; 45 classification questions with no rule for settling | 1.5-2 | low | fewer "depends" answers; MSVC later |
| 4 | E2. Reader coverage of the last pages and of the consolidated pages | 10 pages with no Predict batch; 81 with no Diagnose batch; no reader on the consolidated hotspots | 1.5-2 of repairs | low | evidence for or against the consolidation; item GR2's repairs get tested by use |
| 5 | U. A run check for every `undefined` rule | 13 of 56 `undefined` rules have no run-fail and no interpreter test | 0.5-1 | none | decision 0053 used beyond 4 tests; the interpreter's stops checked |
| 6 | FE15. Front-end rounds 15 and later | 212 missed rejects, 112 run-unsupported, 106 unsupported | 0.5-1 per round | low | the regression signal; findings at 4 to 18 per round |
| 7 | V. Evidence out of the overloading pages | Evidence is 62% of `conversion-sequences.md` | 0.5-1 | medium | readable procedure pages; smaller similarity units |
| 8 | L. Test directories that match the pages | 6 pages keep tests under old directory names | 0.25 | none | simpler tools and batches |

Total for items 1 to 6: about 7 to 10 writer sessions, plus the source-reader sessions for item 2. Items 7 and 8 add about 1.

### Item 1. R1. Close the phase 6 record

- **What.**
  - Run the full matrix with `--report` and commit `tests/RESULTS.md`, as phase 5 did in 57b828e.
  - `status_table.py` compares the tests in `RESULTS.md` with the test files, and the status sentence gives both numbers when they differ. `check_specification.py` fails when the file names a missing test. This keeps the cheap per-directory habit and makes the gap visible.
  - Optionally, one results file per chapter (round 3, tool item 9). A writer can then regenerate the results of the chapters it ran, and a full run is no longer the only way to refresh the file.
  - Add `questions/` to the default `--spec` set of `tools/similarity.py`, and commit one whole-repository report that covers the specification, the guides, and every question.
  - Generate the front-end row of `exercises.md` from `conformance.tsv`, or replace its numbers with a pointer to the status page. Mark R0, G, W, P, and D done on the roadmap. Replace the size estimate on the status page with the page count, or remove it.
- **Evidence.** Section 1.4.
- **Cost.** 0.5 to 1 session, most of it the full matrix run (the cache holds the unchanged tests).
- **Risk.** None for the specification. The first question report may flag passages in the 122 questions that record a gap review's reading. Rewording them could add 0.25 to 0.5 session.
- **Unblocks.** The phase 7 decision starts from counts that tools wrote. The charter's second success criterion stays true for every text the writer produces.

### Item 2. GR2. Gap reviews of the pages that never had one

- **What.** A source-reader gap review (decision 0019) for each group, followed by a repair pass:
  - exceptions: throw, handlers, unwinding (41 rules),
  - lookup: names (42 rules),
  - the object model: objects, storage, language-support types (53 rules),
  - program structure: translation, use and the one-definition rule, types, keywords (52 rules).
  Each review also reads the open questions that cite the group's pages and have no gap-review reading. Raw storage alone has 9 such questions.
- **Evidence.** Section 1.6. Round 3 ranked this item fourth, and phase 6 spent its gap reviews on question clusters and item O. Each past gap review led to 0.5 to 1 session of repairs. The item O review listed eight gaps in its nine rules.
- **Cost.** 2 to 3 writer sessions of repairs, plus four source-reader sessions.
- **Risk.** Medium. Storage and lookup use dense terms of art, and the storage page already carries 6 library markers. Keep one condition per rule and run the similarity check on each repaired page and question.
- **Unblocks.** Every page then has had a review. Questions on these pages can settle under decision 0051. Item E2's Predict batches for these pages test the repaired text.

### Item 3. Q2. Settle by divergence

- **What.**
  - `question_index.py` adds a Divergence column, generated from the test notes: the compiler families whose overrides name the question, and whether only an older version differs. This is the round 3 override digest (X2) done as a tool, and it shows each question's state at a glance.
  - Settle the questions where only Clang 18 or GCC 14 differs (about 7 of the 10 in section 1.5).
  - Propose a decision for classification questions (45 with no override). One possible rule is that such a question settles when a gap review agrees with the provisional answer and no family differs. The maintainer decides.
  - Group the maintainer's waiting items into one sheet with one line per question and the choice it needs, so the maintainer can answer in one pass.
- **Evidence.** Section 1.5. Open questions grew by 21 in phase 6 though six clusters settled 18. Most open questions (255) stay open under the current rule because a newest version differs. Closing them needs the maintainer's EDG report or a decision on each.
- **Cost.** 1.5 to 2 sessions. The Divergence column is about 0.25.
- **Risk.** Low. Settling changes status lines and index rows. Question text already passes through similarity under item 1.
- **Unblocks.** Fewer "depends" answers in Predict. A clean base before any MSVC work.

### Item 4. E2. Reader coverage of the last pages and the consolidated pages

- **What.**
  - Predict batches for the ten pages without one (section 1.6), after item 2 repairs them.
  - Predict batches again for the pages that item C consolidated: constant expressions, evaluation order, unwinding, and the overloading pages.
  - Diagnose batches for the pages with the most `tidal-error` tests and no batch yet. Diagnose has reached 4 pages.
- **Evidence.** Phase 6 Predict batches gave 158 findings over 14 batch groups, about 11 per group, and the cited-rule score (3186 of 3272) now separates readers. The consolidated pages carry the most report lines (section 1.7) and no reader has used them since.
- **Cost.** 1.5 to 2 writer sessions of repairs. The reader sessions are separate.
- **Risk.** Low. Findings that a reader fills from outside knowledge need a probe before the writer adopts them.
- **Unblocks.** A measure of whether consolidation lowered the finding rate. That measure decides whether further pages need item C's treatment.

### Item 5. U. A run check for every `undefined` rule

- **What.** For each of the 13 `undefined` rules with no `run-fail` test and no `tidal-interpreter` test (CTOR-12, VOL-6, TRY-11, MUL-12, MUL-14, ADD-11, ADD-14, ADD-17, ADD-18, NEW-3, SEQ-4, OBJ-10, INIT-1), probe whether a sanitizer configuration or the interpreter stops the run. Add a `run-fail` test or a `tidal-interpreter: run-fail` test. Where neither stops it, the rule's Evidence says so.
- **Evidence.** Section 1.1. Decision 0053 exists and four tests use it.
- **Cost.** 0.5 to 1 session.
- **Risk.** None for similarity. An interpreter stop only adds a check. The rule's class still rests on compiler evidence (decision 0053).
- **Unblocks.** Every `undefined` rule has a run that a tool checks. The front end gets direct targets for its interpreter.

### Item 6. FE15. Front-end rounds

- **What.** Answer the findings of each later round.
- **Evidence.** Missed rejects cluster on declarations (65), templates (49), classes (34), and expressions (30). Run-unsupported tests cluster on classes (38) and expressions (32). Rounds 10 to 14 gave 4 to 18 findings each.
- **Cost.** 0.5 to 1 session per round.
- **Risk.** Low.
- **Unblocks.** The interpreter for more run tests, and item 5's interpreter stops.

### Item 7. V. Evidence out of the overloading pages

- **What.** Move the generated probe grid tables of the two overloading pages out of the page body, into generated pages under the site or files beside the grids under `tests/.../grids/`. The Evidence segment keeps the source table, the divergences, and a link to each grid.
- **Evidence.** Section 1.8. Item C kept every fact in the page.
- **Cost.** 0.5 to 1 session, mostly in `build_site.py` and `status_table.py`.
- **Risk.** Medium. Evidence holds the inventory notes from source readers. A move must not drop the note that names each rule's source (decision 0044). Similarity checks the moved text in its new place.
- **Unblocks.** Procedure pages that a reader can read through. Smaller similarity units on the two largest pages.

### Item 8. L. Test directories that match the pages

- **What.** Move the tests of six pages to directories named for the pages:
  - `expressions/constant-expressions` to `foundations/constant-evaluation`,
  - `expressions/value-categories` to `foundations/values`,
  - `expressions/conversions` to `foundations/conversions`,
  - `expressions/sequencing` to `foundations/evaluation-order`,
  - `foundations/conversion-sequences` and `foundations/overload-resolution` to `overloading/`.
- **Evidence.** The test directories still follow page names from before phases 2 and 6. Commit ce617b7 left the overloading tests under `tests/foundations/`.
- **Cost.** 0.25 session, plus a full matrix run if the cache key holds the path.
- **Risk.** None for similarity. `RESULTS.md`, `conformance.tsv`, and every `:tests:` line change in one commit, so the commit belongs at a phase boundary, together with item 1's full run.
- **Unblocks.** Tools and batch scripts can map a page to its directory without a table.

## 3. Out of scope

1. **No new topic in phase 7.** The status page lists no in-scope topic without a page. The cost is in review coverage, questions, and records.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. The 29 *(referenced)* markers state the boundary.
3. **MSVC stays deferred** until item 3's Divergence column exists. A new family would add overrides to the pages that already carry the most (overload resolution 27, conversion sequences 27, constraints 26).
4. **The launder operation, `typeid`, and the bit copy stay referenced** (Q-0322), as decision 0051 chose.
5. **A probe index of later-language acceptances** (Q-0007) waits on the maintainer. From the writer's view it carries no similarity risk and adds no rules, but it adds tests with notes that the Divergence column would have to separate from C++23 divergences.
6. **The list-order check** has been asked for in three rounds and never built. Item O wrote its condition lists without it, and the item O gap review compared those lists with the source. A gap review covers the same risk at the cost of one reviewer session. Drop the check from the tool list, and name the gap review as the control for list rules in decision 0044.

## 4. Process changes

Ranked by writer time saved.

1. **Check results from tools in commit messages.** A script runs the check chain (`run_tests.py` on the changed directories, `check_specification.py`, `question_index.py --check`, `similarity.py --fail-on-flag`, `frontend_check.py`, `build_site.py --strict`) and writes a Checks paragraph to a file, with the counts each tool printed and the commit it ran at. The writer adds the file to the commit message. Four phase 6 commits exist only to correct typed numbers (section 1.3).
2. **One front-end binary per round.** A front-end round builds into its own target directory, and `frontend_check.py` takes the path of the binary it checks. The integrator's check then never runs against a binary that another agent is rebuilding (ce617b7, 0cc80a1). A round also records the test commit it started from, so that tests a writer adds during the round count as new.
3. **`frontend_check.py` reports changes between failing states.** Report 14 says the check cannot see a test that moves from one failing status to another, and the round built a second front end to compare. The check can compare each test's status with its baseline status.
4. **Questions in every similarity run.** Covered by item 1. A question that records a gap review's reading passes `--fail-on-flag` before the commit, and the committed report shows it.
5. **Smaller per-commit similarity reports.** A commit's report lists the units of the files it changed, plus the flag count. One whole-repository report closes each phase. `status_table.py` already handles a report that checks only some units. Phase 6 added 71,913 report lines, nearly all of them repeated `ok` lines.
6. **Phase close as a checklist step.** The last step of each phase runs the full matrix with `--report`, the whole-repository similarity report, `frontend_check.py --update` if the round asks for it, and `status_table.py`, and marks the roadmap rows. Phase 6's last commit (3d98c93) closed the phase without a full matrix run or a whole-repository similarity report.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, and scripts run on commit 3d98c93.

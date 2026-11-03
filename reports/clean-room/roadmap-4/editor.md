# Roadmap, round 4: what to do after phase 6, from an editor's view

Date: 2026-11-03. Decision 0029. A clean-room reader in the role of the editor who owns the specification's quality (weakest pages, questions, evidence, consistency) wrote this report. The reader read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, style guide, specification format, exercises, and decisions 0033, 0039, 0044, 0050, 0051), the round-3 editor report, `specification/`, the headers of `tests/`, `questions/` with its index, the phase 6 reports (`reports/clean-room/` R-expressions, R-statements, R-declarations, R-classes, W1 to W15, X9 to X12, rust-frontend report-10 to report-14; `reports/review/` the gap reviews of clusters 1 to 6 and of item O, and the class-properties inventory), and `git log --oneline 39e9322..HEAD` (79 commits). The reader did not read the other round-4 reports. Read-only scripts made the counts from the tree at commit 3d98c93. Statements marked "general knowledge" do not come from Tidal's files.

Columns are the same as in round 3. **R/T** is live rules (withdrawn rules left out) and distinct tests cited. **Q** is open questions (provisional or open) whose `Affects:` line names a rule on the page. **F** is lines in `reports/clean-room/` that cite a rule on the page, without the roadmap reports (the status page's "Report mentions"). **X** is cited tests with a per-compiler override. **Ev** is the lines of the Evidence segment against the page's lines. **R/P** is live rules per principle.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 3 (5eff7cd) | Now (3d98c93) |
|---|---|---|
| Live rules | 1633 | 1663, plus 15 withdrawn rules that point to the rule that replaced them |
| Test files | 5408 | 6032, every one cited by a rule |
| Rules that rest on one test | 297 (18%) | 3 (TPL-48, INST-33, INST-36) |
| Tests with an override | 436 (8%) | 523 (9%) |
| Overrides by label | EDG 170, GCC 159, Clang 80, Clang 18 alone 55, GCC 14 alone 33 | EDG 211, GCC 198, Clang 108, Clang 18 64, GCC 14 36 |
| Tests whose overrides name only an older version | 55 | 59 |
| Rules whose every test carries an override | 51 | 43 |
| Tests with `tidal-error` patterns | 1451 | 2798 |
| Questions | 321 | 352 |
| Open questions (provisional or open) | 289 (290 at 39e9322, when phase 6 began) | 310 |
| Settled, compiler defect | 12, 20 | 30, 12 |
| Front end | 4439 of 5408 pass | 5595 of 6032 pass |
| Evidence lines, whole specification | not counted | 5259 of 38199 (14%) |

Phase 6 moved questions this way, counted from each file's status at 39e9322 and at 3d98c93:

- 28 new open questions (Q-0323 to Q-0352, less Q-0332 and Q-0333, which are compiler defects).
- 9 open questions settled (Q-0010, Q-0090, Q-0169, Q-0261, Q-0270, Q-0271, Q-0288, Q-0289, Q-0291).
- 9 compiler-defect questions settled, because the newest versions fixed the defect.
- 1 compiler-defect question reopened (Q-0181). Q-0300 settled and reopened within the phase, so its status did not change.

Open questions rose from 290 to 310. Clusters 2, 3, 5, and 6 held 68 distinct questions, and the standing rule of decision 0051 settled none of them, because the newest versions split on every one. Cluster 4 settled 1 of 14. Cluster 1 settled 19 of 26, and its gap review reopened 2 of those (Q-0181, Q-0300).

The gap reviews gave a reading on 85 questions. 54 of the 78 that stay provisional carry a reading of one option with no doubt. In at least 5 of those 54 (Q-0041, Q-0243, Q-0275, Q-0277, Q-0280) the reading goes against Tidal's answer, and the status page sends them to the maintainer. About 49 open questions therefore have a source reader's reading that supports the current answer, and they stay open only because one family differs. 217 open questions appear in no phase 6 gap review.

Tidal sides with one newest family against the other two in 63 tests. Clang is the one family in 40 of them, GCC in 16, and EDG in 7. Every one of the 63 names a question. In one test Tidal goes against all three newest families on a point that no implementation choice explains (`replace-base-old-pointer-constant.cpp`, STG-19, Q-0307).

The Predict batches without test names reached 57 pages and scored 3262 of 3272. 3186 answers cited a rule of the test. The score stays near full without the names, so the findings carry the information. The four Predict chapters gave 158 findings, the Write suites W1 to W14 gave 107, X9 to X12 gave 49, and the front end gave G85 to G117. Across the Predict and Write reports, a grep of the finding kinds gives about 47 "unclear", 43 "missing", 35 "needs example", 35 "hard to find", 14 "outside knowledge", and 5 "wrong".

### What phase 6 took from round 3

| Round-3 item | Outcome |
|---|---|
| 1. Settle the backlog | Clusters 1 to 6 ran, each with a gap review. Open questions rose by 20 (above) |
| 2. Write suites for unexercised pages | All of W1 to W14 ran. The second half of decision 0051's item W did not happen. None of the 62 Write programs (6626 lines) is a test under `tests/` |
| 3. Predict format, then four chapters | Done. 57 pages have a batch without test names. 28 pages have none: Foundations (12), Overloading (2), templates (11), exceptions (3) |
| 4. Overloading chapter, slimmer Evidence | The chapter exists. Evidence is now in four parts with the grids collapsed, and it grew from 603 to 676 lines (29% of the chapter). The tests stay under `tests/foundations/` |
| 5. Thicken thin evidence | Done for single-test rules (297 to 3). Override-only rules fell from 51 to 43 |
| 6. Consistency pass | Done. "is an error" fell from 12 uses to 2. The glossary has "substitution failure" and "deduction failure". Concepts now has 6 principles |
| 7. Implementation limits | Done (TR-13, TR-15, Q-0350, Q-0351) |
| 8. Class properties | Done as item O, with a gap review and repairs |
| Process: tools write the status numbers | Done in R0. The baseline pass count still comes from `tests/RESULTS.md`, which is stale (item 2) |
| Process: two warnings in `check_specification.py` | Not done. The check prints no warning for override-only rules or for the decision 0044 proxy |

### Records that drift

These records disagree with the tree at 3d98c93:

- `tests/RESULTS.md` was last written at bddd2a0, 39 commits back. It has 5609 rows. 426 test files in the tree have no row, and 3 rows name removed tests. The status page's "5609 of 5609 tests pass" describes that record and not the current tests.
- The phase 6 table of `documentation/roadmap.md` marks Q, C, O, and FE as done and leaves R0, G, W, P, and D without a state. The status line of decision 0051 does not say which items are done. Decision 0050's line does.
- The front-end row of `documentation/exercises.md` section 8 says round 10 and 4918 of 5480. The front end is at round 14 and 5595 of 6032.
- Q-0312 has the status "open" and a provisional answer (TR-12). It is the only "open" question.
- The commit that acted on the cluster 2 review recorded the review's reading only for the questions it changed or kept with doubt. Q-0137, Q-0139, Q-0144, Q-0145, Q-0147, and Q-0187 do not mention the review, although the review agrees with each. Clusters 3 to 6 record a reading on every question.
- Six pages keep their tests under another path: `overloading/` under `tests/foundations/`, and constant-evaluation, conversions, evaluation-order, and values under `tests/expressions/`.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | Other evidence |
|---|---|---|---|---|---|---|---|---|
| 1 | overloading/overload-resolution | 44 / 171 | 17 | 176 | 27 | 344 / 1147 | 11.0 | The highest R/P in the tree. No Predict batch ever. 7 tests where Tidal sides with one family. Front end 83% |
| 2 | overloading/conversion-sequences | 47 / 129 | 16 | 92 | 27 | 332 / 1156 | 9.4 | The longest page. ICS-35 and ICS-44 rest only on overridden tests. 8 one-family tests, the most of any directory. Q-0294 and its splits have no gap review |
| 3 | templates/instantiation | 35 / 134 | 17 | 60 | 23 | 233 / 858 | 7.0 | Q rose from 15. 2 of the 3 single-test rules in the tree. 6 one-family tests. 5 of its questions have no gap review |
| 4 | declarations/initialization/references | 13 / 64 | 12 | 84 | 15 | 80 / 405 | 4.3 | 0.92 questions per rule, the highest. No phase 6 cluster took it, so 10 of its 12 questions have no gap review |
| 5 | foundations/storage | 29 / 83 | 12 | 160 | 6 | 109 / 860 | 3.6 | New in this list. Phase 6 opened 7 of its questions (Q-0323 to Q-0325, Q-0327, Q-0328, Q-0336, Q-0350), mostly from Write suites and item C. 9 have no gap review. No Predict batch. The interpreter now has byte memory to run its tests |
| 6 | templates/constraints | 39 / 125 | 15 | 46 | 26 | 206 / 920 | 7.8 | Q fell from 18. CNS-18, CNS-32, and CNS-38 still rest only on overridden tests. Front end 83% |
| 7 | templates/dependent-names | 39 / 121 | 14 | 106 | 21 | 238 / 1024 | 6.5 | 6 `no-diagnostic` rules. 8 questions with no gap review. DEP-27 and DEP-34 rest only on overridden tests |
| 8 | expressions/new-delete | 42 / 140 | 12 | 127 | 22 | 146 / 877 | 6.0 | 8 questions with no gap review (Q-0035, Q-0036, Q-0091, Q-0221, Q-0222, Q-0300, Q-0301, Q-0302). Front end 75% |
| 9 | templates/class-template-deduction | 29 / 116 | 13 | 43 | 21 | 188 / 813 | 9.7 | Three principles for 29 rules. 6 questions with no gap review |
| 10 | declarations/alignment | 8 / 37 | 6 | 13 | 16 | 37 / 227 | 2.7 | 43% of its tests have overrides, the highest share. Its GCC questions (Q-0069 to Q-0071, Q-0073) have no gap review |

Close behind:
- templates/ordering: 11 of 42 tests have overrides (26%), and Evidence is 147 of 501 lines (29%).
- templates/specialization: Q 10 and F 21, the lowest F in the templates chapter.
- foundations/constant-evaluation: X 22, and 7 one-family tests.
- declarations/linkage-specifications: the front end passes 21 of 33 tests (64%), the lowest page.
- templates/packs: 10.3 rules per principle.

Round 3's list and this one share nine pages. Every page on both lists gained tests, and only constraints and alignment lost questions.

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | **Settle by review.** A second standing rule that lets a gap review's reading settle a question when one family differs. Apply it to the 85 reviewed questions | Open questions rose from 290 to 310. The rule of 0051 settled 1 of 82 distinct questions in clusters 2 to 6. About 49 open questions have a reading that supports the answer with no doubt | one session, plus a maintainer decision | medium | A falling question count. Pages 1 to 9. The EDG report |
| 2 | **Repair the drifting records.** Rerun the full matrix, mark the phase 6 rows, fix the stale rows and statuses listed in section 1 | Section 1, "Records that drift" | under one session, plus a full matrix run | low | A status page that matches the tree. Every later count |
| 3 | **One decision packet for the maintainer.** One file that lists every waiting item with its options and the answer Tidal keeps by default | Six items wait on the status page. Some date from decision 0024 | under one session | low | About 40 questions, the EDG report, and the choices of decision 0051 |
| 4 | **Gap reviews for the 217 unreviewed open questions**, in four clusters | 217 open questions appear in no phase 6 review. Pages 2, 4, 5, 8, and 10 hold most of theirs | four sessions of a source reader, two of a writer | medium | Item 1's rule on the rest of the backlog |
| 5 | **Predict on the 28 pages with no name-free batch**, starting with overloading and storage | Pages 1, 2, and 5 have never had a Predict batch. Templates had batches only with test names | three to four sessions | low | Findings on the weakest pages |
| 6 | **Turn the Write programs into tests.** The second half of decision 0051's item W | 62 programs, 6626 lines, no test under `tests/` | one session | low | Evidence from use in the matrix, and run tests for the interpreter |
| 7 | **Rework the overloading pages.** More principles, Evidence that points to questions, tests under `tests/overloading/` | R/P 11.0 and 9.4. Evidence 29% of the chapter and growing | one to two sessions | medium | A reader can find a tie-breaker. Item 5 on these pages |
| 8 | **Thin evidence that remains.** A test that every compiler agrees on for each of the 43 override-only rules, and the two check warnings | 43 rules. The warnings were asked for in rounds 2 and 3 | one session | low | Rules that do not rest only on a disputed test |

### 2.1 Settle by review

The rule of decision 0051 asks every newest family to agree. That condition holds mostly for questions about older versions, and phase 6 used up that supply in cluster 1. The other clusters are made of splits by construction, so they cannot close under the rule.

Under the proposed rule, a question also settles when its gap review supports the provisional answer with no doubt, and a principle on the page gives the same answer. The question records the differing family's result as a deviation, and the test keeps its override. A family that is alone in a settled question becomes a "compiler defect" entry, as Q-0002 and Q-0003 already are. A question whose review reads against Tidal, or against every compiler, goes to the maintainer (item 3). A review "with doubt" or "unclear" keeps the question open.

Applied to the 85 reviewed questions, this rule settles about 49 at once. Cluster 5 alone gives 23 of them, all with EDG as the one family, and they join the EDG report that already waits on the maintainer.

The risk is that one source reader's reading becomes final. Two checks keep the risk down. The rule asks for a principle on the page as well as the review. And a contributor can reopen a settled question with a pull request, as `questions/README.md` already allows.

### 2.2 Repair the drifting records

These steps fix each record in section 1:

- Run `tools/run_tests.py --compilers all --report` on the whole tree, so `tests/RESULTS.md` covers all 6032 tests. Then rerun `tools/status_table.py`.
- Mark R0, G, P, and D done in the phase 6 table of the roadmap. Mark W done with the Write programs left over (item 6), or leave W open until item 6 lands. Update the status line of decision 0051.
- Rewrite the front-end row of `exercises.md` section 8 from `conformance.tsv`.
- Change Q-0312 to "provisional", since it has an answer.
- Add the cluster 2 review's reading to Q-0137, Q-0139, Q-0144, Q-0145, Q-0147, and Q-0187.

Moving the six test directories goes in item 7, because it touches the cache and the front-end baseline.

### 2.3 One decision packet for the maintainer

The status page has six items under "Waiting on the maintainer". They cover the EDG report (Q-0002, Q-0003, and 30 template questions), seven questions where the matrix agrees and a review reads the text the other way (Q-0300, Q-0181, Q-0243, Q-0275, Q-0277, Q-0280, Q-0225), the variable form of Q-0235, Q-0343 and Q-0041, the choices of decision 0051, and the probe of later language versions. The request for an outside reader dates from decision 0024.

The editor proposes one file, such as `documentation/decisions/pending.md`, with one row per item. Each row gives the options, the answer Tidal keeps until the maintainer decides, and the questions that the decision closes. Item 1's new rule goes in the packet as its first row. A short packet costs the maintainer less time than six items spread over prose.

### 2.4 Gap reviews for the 217 unreviewed open questions

Grouped by the first rule in each question's `Affects:` line:

| Cluster | Questions | Count | Pages |
|---|---|---|---|
| 7 | Overloading and references: Q-0039, Q-0040, Q-0042, Q-0043, Q-0109, Q-0110, Q-0132, Q-0134 to Q-0136, Q-0140 to Q-0143, Q-0149 to Q-0151, Q-0154, Q-0155, Q-0157, Q-0164 to Q-0167, Q-0180, Q-0184, Q-0294, Q-0331, Q-0337 | 29 | pages 1, 2, and 4 |
| 8 | Objects and storage: the STG, NEW, ALN, VOL, LSUP, and enumeration-bytes questions, among them Q-0069 to Q-0073, Q-0221, Q-0222, Q-0301, Q-0302, Q-0304, Q-0306, Q-0322 to Q-0328, Q-0350 | 42 | pages 5, 8, and 10 |
| 9 | The rest of templates: specialization, dependent names, CTAD, instantiation, declarations, concepts, constraints, packs | 42 | pages 3, 6, 7, and 9 |
| 10 | Classes, expressions, declarations, and statements | 96 | split by chapter into two reviews |

8 questions name no rule (Q-0006, Q-0008, Q-0116, Q-0119, Q-0182, Q-0183, Q-0186, Q-0312). They need the maintainer, and they go in item 3.

Each cluster follows the pattern of phase 6. A writer probes the cells that no test covers, a source reader reviews, and a writer records a reading on every question and applies item 1's rule. Phase 6's review results suggest that about 60% of a cluster can settle this way (54 of 85). The share for these clusters is a projection, not a count.

### 2.5 Predict on the 28 pages with no name-free batch

The pages are the 12 Foundations pages, the 2 Overloading pages, the 11 templates pages, and the 3 exceptions pages. Overloading and storage come first, because they are on the weak list and have never had a batch. Templates come next. Their phase 5 batches scored 1089 of 1092 with the test names visible, so those scores say little about the text.

Phase 6 batches gave about one finding per 20 answers (158 of 3272). On the same rate, 28 pages with about 2600 tests would give about 130 findings. That estimate is a projection.

### 2.6 Turn the Write programs into tests

Decision 0051 asked that every Write program run in the interpreter and become a run test. The reports show that each program builds and runs on GCC and Clang, and the front end runs the W7 and W10 programs. No program is in `tests/`.

The editor proposes to put each program under `tests/programs/<suite>/` with `tidal-expect: run`, and cite the rules that its report names. A program test is evidence from use. It should not count toward decision 0044's test that one step alone decides, and `check_specification.py` can leave the directory out of that proxy. The matrix then catches a later rule change that breaks a program that a reader wrote from the text.

### 2.7 Rework the overloading pages

The two pages are still the longest, with the most rules per principle. Phase 6 gave the Evidence a structure and kept every fact. The Divergences part now lists a bullet per split, and each split also has a question file.

Proposals:
- Split the four principles of overload-resolution and the five of conversion-sequences, so that each principle covers at most about six rules. The style guide's principle-first rule (decision 0024) asks a principle to explain its rules, and eleven rules per principle make that hard.
- Replace each Divergences bullet that has a question with one line that names the question and its answer. The question file keeps the probe results.
- Move `tests/foundations/conversion-sequences/` and `tests/foundations/overload-resolution/` to `tests/overloading/`, and the four other mismatched directories to their page's path. A move changes the cache keys and the front-end baseline (general knowledge about caches keyed by path).

Item 5's batch on these pages runs after the rework, so that its findings apply to the new text.

### 2.8 Thin evidence that remains

43 rules rest only on tests with an override: DCMP-8, CTOR-14, CTOR-17, CTOR-18, CLS-26, ALN-7, ATTR-13, ATTR-18, EXS-5, EXS-6, REF-15, TYN-5, TYN-9, TRY-12, TRY-13, ADD-17, DTY-5, UNEV-4, MUL-12, NEW-35, NEW-36, PFX-15, PRIM-15, CE-13, CE-18, CE-28, ICS-35, ICS-44, TARG-22, CTAD-13, CPT-3, CPT-15, CNS-18, CNS-32, CNS-38, TPL-42, TPL-48, DEP-27, DEP-34, ORD-7, PACK-30, TSP-17, and TSP-21. Some of these rules are `impl-defined` or `unspecified`, and an override is how the test records each choice. The rest need a test where every compiler agrees, or a note on why no such test exists.

The two warnings that rounds 2 and 3 asked for go in `check_specification.py` with this item. One warns for a rule whose every test has an override. The other warns, on the procedure pages of decision 0044, for a step rule with no test that cites it alone.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. The cost sits in questions and records |
| Language after C++23, and the index of later rejections (designer item 4, Q-0007) | Keep out until the open count falls below the count at the start of phase 6 (290) | 310 C++23 answers are still open. Q-0188 and Q-0181 already show later changes leaking into C++23 answers |
| Diagnose batches beyond D-classes, and the Port suites | Defer to the phase after items 4 and 5 | Predict findings still arrive at about one per 20 answers. A Diagnose batch measures messages, which matter less while the answers are open |
| Extended floating-point types, coroutines, modules, threads and the memory model, the general library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decision 0051 say | Nothing new in phase 6 argues for them. 29 *(referenced)* markers name them |
| Library traits and builtin queries in tests | Keep out, as decision 0051 says | Item O worked through behavior and probes. Q-0343 depends on the builtin queries and goes to the maintainer |
| A list of token sequences that no production accepts | Keep out | TR-12 answers Q-0312 |

## 4. Process changes

- **A second standing rule for settling**, as item 1 describes. Decision 0051's rule is provisional, so the next phase decision can extend it.
- **Each phase decision states a target for open questions.** Phase 6 commits stated opened and closed counts, and the phase still ended 20 higher. The next phase decision should state the open count at its start and the count it aims for. A phase that ends above its target says why in its closing commit.
- **Every act-on-review commit records a reading on every question in the review.** `tools/question_index.py --check` can flag a question that a file in `reports/review/` names and that has no Opinions entry dated after the review.
- **A tool marks plan rows done.** `tools/status_table.py` or a new check can fail when a phase table row has no state, and can write the front-end row of `exercises.md` from `conformance.tsv`.
- **The status page states what the matrix record covers.** `status_table.py` can count test files with no row in `tests/RESULTS.md`, and `check_specification.py` can fail when the count is above zero at a phase's end. Commits during a phase may run subsets, as they do now.
- **The two check warnings of item 8.** Round 4 is the third round that asks for them.
- **Questions keep the status "open" only while they have no provisional answer.** `question_index.py --check` can enforce the status.
- **Each new question names the finding that raised it.** Phase 6 opened 28 questions out of about 350 findings. A link from question to finding lets the editor see which exercises raise questions.

## 5. Files

The reader made the counts with read-only scripts in its scratch directory outside the repository (`count.py`, `count2.py`, `qdiff.py`, and the lists `gap.txt` and `reviewed.txt`).

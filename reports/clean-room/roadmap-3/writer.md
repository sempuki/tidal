# Roadmap, round 3: what to do after phase 5, from a clean-room writer's view

Date: 2026-11-01. Decisions 0029, 0044, 0047, 0050. A clean-room agent in the writer's role prepared this report. It read AGENTS.md, CLAUDE.md, `documentation/` (with decisions 0044, 0047, 0048, and 0050), `reports/clean-room/roadmap-2/`, `tools/`, `questions/README.md`, the specification page list, the front-end baseline `conformance.tsv`, and `git log` with commit bodies. It listed the file names under `reports/similarity/` and `reports/review/` and read the last line of each similarity report, which holds only a flag count. It did not read the bodies of the review summaries, and it did not read `inputs/`. All counts come from read-only `grep`, `awk`, and `git log`. The report wrote no repository files.

## Baseline numbers

| Measure | End of phase 4 (round 2) | Now |
|---|---|---|
| Rules | 1541 | 1633 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 4946 | 5408 |
| Single-test rules | 446 (29%) | 297 (18%) |
| Tests with a per-compiler override | about 386 | 461 (this count includes the `-O2` report overrides) |
| Question files | 270 | 321: 288 provisional, 1 open, 20 compiler defect, 12 settled |
| Index rows whose answer is "option 1" | 74 | 65 |
| Front end (`conformance.tsv`) | 3444 of 4348 | 4439 of 5408 (347 missed rejects, 312 unsupported, 223 run-unsupported, 74 false rejects) |
| Probe grid files | 40 | 38 committed under `tests/*/grids/` |

Phase 5 added 92 rules and 462 tests. The writers also acted on 163 reader findings: front-end G68 to G84 (17), Diagnose DC-1 to DC-17 (17), W15 (9), Predict RA, RB, and RC (56), and Propose X5 to X8 (64). The gap review of item P added 38 more.

## 1. What costs the most now

The writer's time in phase 5 went mostly to repairs. New pages (F1, F2) were two commits. Repairs from findings were at least eight commits, and each touched 18 to 136 files. The cost now sits in five places.

### 1.1 Repairs from reader findings

- Commits a877d20, 97a31fb, 3c569c2, b456a30, ef3408d, 008bc71, d4d3b68, and c33f49b are repair commits. Together they changed about 550 files.
- Each Propose report gave 12 to 20 findings, and each one touched many rules. X6 alone lists 75 rule changes.
- The repairs are productive. Phase 5 repairs added rules such as CE-29, CE-30, FN-15, NAM-37, NAM-38, PFX-20, PFX-21, UNI-14, and CNS-39. Most of these are rules any reader of the page needed.
- The repairs cluster on a few pages. `constant-evaluation.md` changed in four phase 5 commits and gained CE-29, CE-30, and the CE-P4 table piecemeal. The status table gives constant expressions 192 report mentions on 30 rules.

### 1.2 Integration overhead

- 23 commit bodies say that the integrator renumbered a writer's questions (for example "filed it as Q-0350, and the integrator renumbered it", in ef3408d). Each parallel writer picks a private range (Q-0340, Q-0350, Q-0360), and the integrator rewrites every reference.
- Two commits record edits that landed in the neighbour commit. 33ef46e says the question edits "went into the previous commit". 7819b7c says "The glossary entries go with the next commit".
- `questions/README.md` is edited by hand. Phase 5 commits list its new rows one by one (a011ece, 33ef46e).

### 1.3 Hand-kept numbers drift

`documentation/status.md` has four prose numbers that no tool writes, and all four are wrong after phase 5:

| Status page says | Files say |
|---|---|
| 4946/4946 tests pass on each compiler | 5408/5408 (`tests/RESULTS.md`, commit 57b828e) |
| 283 questions open | 289 (288 provisional, 1 open) |
| 16 compiler-defect questions | 20 |
| 13 *(referenced)* markers: library 5, extended floating point 3, preprocessor 1 | 21 markers: library 11, extended floating point 4, preprocessor 2 |

`tools/status_table.py` still prints the literal "0 flagged" in the Similarity column. Round 2 asked for that fix. Commit dbd4096 added the templates chapter and left the constant in place. The Report mentions column counts every Markdown file under `reports/clean-room/`, roadmap reports included, so this report raises the counts of every rule it names.

### 1.4 The similarity record lags the text

- The last committed similarity report is `reports/similarity/2026-10-31-types.txt`, from commit a3df519 at the end of phase 4. It contains no STG rule, no new LSUP rule, and no CE-30. Phase 5 commits say "similarity --fail-on-flag: 0 flagged" and commit no report. Pipeline step 8 says to commit the report.
- `tools/check_all.sh` runs `similarity.py | tail -1` under `set -e`, without `--fail-on-flag`. A flag can never fail the script. AGENTS.md forbids both the pipe and the missing option. Commit 95c9542 fixed the rule in AGENTS.md after commit dc5b4ca went through with a flag. The script was not fixed.
- Similarity itself is cheap. 19 of 110 reports flagged one or two units, and 14 rewrite reports exist. The cost is the record and the script.

### 1.5 Questions grow faster than they close

- Question files grew from 270 to 321 in phase 5. 11 questions were settled or merged by C1 and lane U.
- 65 index rows still answer "option 1", which a reader of the index cannot interpret.
- The status page lists 7 compiler-defect candidates (Q-0015, Q-0023, Q-0026, Q-0044, Q-0125, Q-0181, Q-0206) that wait for a step no phase item holds.

### 1.6 What does not cost much

- The full matrix is cached (`build/cache`, 59,223 entries, 232 MB). Most phase 5 commits run only the changed directories, and a full run closes each phase (57b828e).
- Parallel writers on disjoint pages worked well. Commit 3c569c2 had two writers on 625 and 1076 tests with no conflict except question numbers.
- Probe grids and decision 0044's controls were needed only in phases 3 and 4.

## 2. Work items, ranked

| Rank | Item | Writer sessions | Similarity risk | Questions |
|---|---|---|---|---|
| 1 | R0. Make the record current (tools) | 0.5-1 | none | 0 |
| 2 | Q. Retire questions | 1.5-2 | none | closes 20-40 |
| 3 | E. Reader coverage of the untouched chapters | 3-4 of repairs | low | 5-15 new |
| 4 | GR. Gap reviews of pages that never had one | 1.5-2.5 of repairs | medium | 5-10 new |
| 5 | H. Consolidate the hotspot pages | 1.5-2 | medium | settles a few |
| 6 | T2. A second test for every rule | 2 | low | 0-5 |
| 7 | FE10. Front-end round 10 findings | 1-2 per round | low | 2-5 |
| 8 | X2. Digest the overrides on lookup, overload resolution, and templates | 1-1.5 | none | closes several |
| 9 | O. Observation-channel decision, then class properties and `typeid` | 0.25, then 1-1.5 | high | 2-4 |

Total for items 1 to 8: about 12 to 17 writer sessions. Item 9 adds about 1.5.

### 1. R0. Make the record current

- **What.** Tool fixes that end the drift in section 1.3 and 1.4:
  - `status_table.py` writes the prose numbers (tests passing, open and defect questions, markers per topic) or the prose stops giving numbers and points at the table,
  - the Similarity column reads the latest committed report for each page,
  - Report mentions counts finding reports only, leaving out `roadmap*/`,
  - `check_all.sh` runs similarity with `--fail-on-flag` and no pipe,
  - one whole-specification similarity report is committed.
- **Evidence.** Section 1.3 lists four wrong numbers. Section 1.4 shows that no committed report covers `storage.md`.
- **Unblocks.** Honest counts for the phase 6 decision and for every later step. The whole-specification report is the charter's second success criterion, kept current.
- **Size.** 0.5 to 1 session.
- **Risk.** None for similarity. A first full report may flag phase 5 text such as STG rules. Rewrites would then add about 0.25 session.

### 2. Q. Retire questions

- **What.**
  - Write out the 65 "option 1" answers in the index.
  - Decide the 7 compiler-defect candidates on the status page.
  - Close each compiler-defect question where every current baseline version agrees (the Q-0270 pattern from round 2).
  - Generate the index from the question files, so the index cannot drift from the files.
- **Evidence.** 289 open questions against 12 settled. Phase 5 added 51 question files. Q-0004 shows a writer can settle a question when every configuration fits one option (commit 33ef46e).
- **Unblocks.** Fewer "depends" answers in Predict batches. Lane X2. A smaller list for contributors to read.
- **Size.** 1.5 to 2 sessions. Rewording "option 1" rows is about 0.25 of that.
- **Risk.** None for similarity. Settling needs a rule for when a writer may settle a question. Decision 0010 should say so, so the step does not look like a writer deciding by fiat.

### 3. E. Reader coverage of the untouched chapters

- **What.** Run the exercises that have never touched most pages:
  - Predict batches for the statements, expressions, declarations, classes, and exceptions pages. Only the Foundations and templates pages have had Predict batches.
  - Write suites W1 to W14. Only W0 and W15 have run.
  - Diagnose batches beyond the four classes and exceptions pages done in phase 5.
- **Evidence.** `documentation/exercises.md` lists W1 to W14 as planned. Phase 5 Predict batches gave 14 to 25 findings each (RA, RB, RC), and the Diagnose batch gave 17. These rates come from template pages. Older pages have had more repair passes, so expect fewer findings per batch there.
- **Unblocks.** The first reader evidence on about 50 pages. Item H uses the findings to choose which pages to rewrite.
- **Size.** 3 to 4 writer sessions of repairs, spread over the phase. The reader sessions are separate.
- **Risk.** Low. Watch for findings that a reader fills from outside knowledge. Each one needs a probe before the writer adopts it.
- **Note on scoring.** Predict scores were 1320 of 1323 across the eleven phase 5 batches, and the batches still gave 56 findings. The findings came from the readers' reasons. `tools/exercises.py score` checks only the outcome, not the rules cited. Scoring the cited rules (as `score_diagnose` already does) would make the number useful (section 3).

### 4. GR. Gap reviews of pages that never had one

- **What.** A source-reader gap review (decision 0019) for the pages that, by commit subjects and the `reports/review/` file names, have had none: the exceptions chapter (throw, handlers, unwinding), name lookup (item H), use and the one-definition rule (items I and J), fundamental types, keywords, language-support types, and raw storage (F1, F2).
- **Evidence.** `reports/review/` holds gap reviews for the statements core, operators, declarations, classes, L, M, N1, N2, O, R, and P. F1 and F2 had inventories only. Each past gap review led to a repair pass of 0.5 to 1 session (P had 38 items and 66 new tests).
- **Unblocks.** The same review level on every page. The storage and language-support pages are new and carry many `undefined` rules.
- **Size.** 1.5 to 2.5 writer sessions of repairs, plus one source-reader session per group.
- **Risk.** Medium. Gap items on storage and lookup touch dense terms of art. One condition per rule keeps the risk down.

### 5. H. Consolidate the hotspot pages

- **What.** Rewrite the pages with the most findings per rule around their principles. Merge rules that repairs added one at a time, and check that each principle still holds the page's model.
- **Evidence.** Report mentions per rule (status table, inflated by roadmap reports, so treat as an order only):

  | Page | Rules | Tests | Mentions | Per rule |
  |---|---|---|---|---|
  | evaluation order | 10 | 33 | 90 | 9.0 |
  | range-based for | 7 | 29 | 49 | 7.0 |
  | constant expressions | 30 | 103 | 192 | 6.4 |
  | constexpr, consteval, constinit | 12 | 68 | 70 | 5.8 |
  | unwinding and termination | 9 | 34 | 46 | 5.1 |
  | objects, storage, lifetime | 11 | 40 | 54 | 4.9 |
  | reference binding | 14 | 62 | 68 | 4.9 |

  Constant expressions gained CE-29, CE-30, and CE-P4 in phase 5 repairs, and CE-3, CV-8, and ODR-3 now share a new glossary term.
- **Unblocks.** Fewer findings in item E on these pages. A stable base for the interpreter's constant evaluator (decision 0048).
- **Size.** 1.5 to 2 sessions. Constant expressions is about half of it.
- **Risk.** Medium. Evaluation order and constant evaluation use terms of art. A rewrite should define Tidal terms and keep one condition per rule. Run the similarity check on each rewritten page.

### 6. T2. A second test for every rule

- **What.** Do for the other chapters what C2 did for templates. Every rule gets at least two tests.
- **Evidence.** 297 single-test rules remain. The largest counts:

  | Page | Rules | Single-test rules |
  |---|---|---|
  | lambdas | 28 | 14 |
  | implicit conversion sequences | 47 | 12 |
  | storage duration and linkage | 23 | 11 |
  | new and delete | 39 | 11 |
  | class members | 27 | 10 |
  | conditional operator | 15 | 9 |
  | constant expressions | 30 | 9 |

  C2 added 117 tests in one commit (d4d3b68) and closed 105 single-test rules.
- **Unblocks.** Decisive tests for item E's Predict batches and for the front end.
- **Size.** 2 sessions.
- **Risk.** Low. Tests are written fresh and are outside the similarity corpus. A second test sometimes shows that a rule fuses two conditions, which splits the rule.

### 7. FE10. Front-end round 10 findings

- **What.** Answer the findings of the next front-end round.
- **Evidence.** Missed rejects cluster on classes (119: virtual 28, unions 17, conversion functions 15, comparisons 15, derived 13, access 12) and handlers (19). Templates have 47 false rejects. 223 run tests are unsupported by the interpreter. Rounds 8 and 9 gave 8 and 9 findings.
- **Unblocks.** The interpreter for classes and exceptions, and the regression signal.
- **Size.** 1 to 2 sessions per round.
- **Risk.** Low.

### 8. X2. Digest the overrides on lookup, overload resolution, and templates

- **What.** Lane X for the pages round 2 named and phase 5 left out. Confirm that each override names a question or a choice, group them by cause, and close compiler-defect cases that every current version fixes.
- **Evidence.** Phase 5 lane X covered alignment, attributes, and exception specifications, and removed no override (0578c65). The largest counts now are overload resolution 26, constraints 26, implicit conversion sequences 23, dependent names 20, class template argument deduction 19, new and delete 18, instantiation 18, and concepts 17.
- **Unblocks.** Item Q. A clean baseline before any MSVC work.
- **Size.** 1 to 1.5 sessions.
- **Risk.** None for similarity.

### 9. O. Observation-channel decision, then class properties and `typeid`

- **What.** Decide how a test observes a property that only the library names. Then complete the trivially-copyable and standard-layout conditions (2 *class properties* markers) and decide whether `typeid` gets a shape entry.
- **Evidence.** 2 class-property markers and 11 library markers remain. F2 left `typeid`, the new handler, and launder as referenced. The round 2 host probe showed a user-declared `std::type_info` makes `typeid` comparisons compile on Clang 22 and GCC 16.
- **Unblocks.** CLS-19 loses its marker. The roadmap's Deferred list shrinks.
- **Size.** 0.25 session for the decision, 1 to 1.5 after it.
- **Risk.** High for class properties. CLS-19 was flagged once, and condition lists are the riskiest form. Write one rule per condition with an indirect test for each. This item can move to phase 7 if phase 6 runs long.

The status page's follow-ups (a `-fsanitize=type` configuration for STG-18 and STG-23, the copy-move triviality split, the class-and-variable rule in `names.md`) are small. They fit into items Q and T2.

## 3. Tool and process changes

Ranked by writer time saved.

1. **Question numbers.** A writer subagent files a question under a placeholder ID (for example `Q-NEW-a`). A script, `tools/number_questions.py`, gives each placeholder the next free number and rewrites every reference in pages, tests, and questions. This removes the step that 23 commits describe.
2. **A generated question index.** `questions/README.md` is built from each file's status and provisional-answer lines. `check_specification.py` fails on an answer of the form "option N".
3. **Status numbers from tools** (item R0). No number in `status.md` prose is typed by hand.
4. **`check_all.sh` fails on a flag.** Use `--fail-on-flag` and drop the `| tail -1`. Commit a whole-specification similarity report at the end of each phase.
5. **Predict scoring checks the cited rules.** `exercises.py score` already does so for Diagnose batches. For Predict, count an answer right when the outcome matches and one cited rule is a rule the test cites. Report both numbers.
6. **A per-writer file list.** Each writer subagent returns the list of files it changed. The integrator stages from the list and checks `git show --stat`. AGENTS.md asks for the stat check already. The list prevents the split commits in 33ef46e and 7819b7c.
7. **Small fixes in `check_specification.py`.** The guide-link check runs twice (in `check_markers` and in `main`), so one fault prints two errors. The second `HEADING` branch after the old-heading error can never run. `status_table.py` assigns `TESTS` twice.
8. **The list-order check.** Round 1 (item 5) and round 2 (item 2) both asked for it, and it still does not exist. Item O needs it before class property lists are written.
9. **`tests/RESULTS.md` size.** The file has 26,776 lines, and a full-matrix commit adds about 1,500 changed lines (57b828e). Writing one results file per chapter would keep diffs readable. This change is optional.

## 4. Scope decisions

1. **No new topic in phase 6.** Phase 5 showed that consolidation and reader findings now cost more than new pages. Every topic the charter lists has a page. Phase 6 should consolidate and get reader coverage, and leave topics alone.
2. **Coroutines stay out.** Round 2's reasons hold. Run tests would need the `<coroutine>` header, the transformation is a procedure that needs all of decision 0044's controls, and the interpreter has no suspension model.
3. **Extended floating-point types stay out.** Clang 22 rejected all five suffixes in the round 2 probe. Every rule would be `conditionally-supported` with evidence from one family. The 4 markers state the boundary.
4. **The preprocessor beyond `#include "file"`, modules, threads, and the memory model stay out**, as the charter says.
5. **MSVC stays deferred** until item X2 is done. A new implementation would add overrides to the pages that already carry the most.
6. **Language version stays C++23** (Q-0007).
7. **Class properties and `typeid` get a decision in phase 6** (item O). The writing can wait for phase 7 if needed.

General knowledge used in this report: the real coroutine handle resumes through compiler built-ins, so a coroutine run test needs the header. Everything else comes from Tidal's files, its commit messages, and the round 2 host probes.

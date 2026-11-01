# Roadmap, round 2: what to do after phase 4, from an editor's view

Date: 2026-10-31. Decision 0029. A clean-room reader in the role of the editor who owns the specification's quality wrote this report. The reader read AGENTS.md, CLAUDE.md, `documentation/` (with decisions), `specification/`, `tests/`, `questions/`, `reports/clean-room/` (with the round-1 roadmaps), `frontend/` conformance, and `tools/`. The reader counted the evidence with read-only scripts. The counts come from the working tree. The tree holds uncommitted repairs from the R and P gap reviews (modified front-end sources, new tests under `tests/templates/`), so a few counts will move when those repairs land. Statements marked "general knowledge" do not come from Tidal's files.

Columns: **R/T** rules and distinct tests cited; **Q** questions whose `Affects:` line names a rule on the page; **F** lines in `reports/clean-room/` that cite a rule ID on the page; **X** cited tests with a per-compiler override; **1t** rules backed by exactly one test.

Repo-wide counts:
- 1541 rules. 446 of them (29%) rest on one test.
- 4939 test files on disk. 117 of them are cited by no rule, and all 117 are under `tests/templates/`: concepts 38, class-template-deduction 29, constraints 29, packs 21.
- 270 questions: 253 provisional, 16 "compiler defect", 1 settled.

The templates chapter has 352 rules, 116 of them single-test (33%). It has about 80 questions (0.23 per rule, against 0.15 for the rest) and 97 override tests. F is 0 on every templates page.

## 1. The ten weakest pages

| # | Page | R/T | Q | F | X | 1t | Other evidence |
|---|---|---|---|---|---|---|---|
| 1 | declarations/initialization/references | 14 / 61 | 11 | 55 | 14 | 2 | 0.79 questions per rule, the highest in the tree. Most questions are about one braced element binding a reference to an array (Q-0149, Q-0164 to Q-0166) |
| 2 | foundations/overload-resolution | 44 / 149 | 18 | 47 | 25 | 6 | 304 lines of Evidence; 22 links forward into the templates chapter; OVL-41 restates ORD-13 with fewer conditions |
| 3 | templates/dependent-names | 38 / 95 | 14 | 0 | 16 | 13 | 6 `no-diagnostic` rules, the most on any page; 188 lines of Evidence |
| 4 | templates/instantiation | 35 / 119 | 14 | 0 | 17 | 6 | INST-15 classes an endless instantiation chain as `undefined`, but its only test expects all three families to reject it |
| 5 | foundations/conversion-sequences | 46 / 110 | 14 | 52 | 22 | 12 | 290 lines of Evidence; shares the braced-list cluster with page 1 |
| 6 | templates/ordering | 16 / 36 | 7 | 0 | 9 | 3 | 25% of its tests have overrides; 2 principles cover 16 rules; the constraint tie-breaker is split with OVL-30 and OVL-41 |
| 7 | templates/constraints | 32 / 73 | 10 | 0 | 12 | 9 | 29 template tests cite none of its rules yet |
| 8 | declarations/alignment | 7 / 31 | 5 | 1 | 13 | 2 | 42% of its tests have overrides, the same as in round 1. Lane X has not run |
| 9 | templates/packs | 25 / 33 | 1 | 0 | 1 | 17 | 68% of its rules are single-test, the highest share; 21 tests are not cited yet |
| 10 | templates/concepts | 35 / 67 | 4 | 0 | 7 | 17 | 49% single-test; 38 tests are not cited yet; one example block |

Close behind:
- templates/specialization: Q 9, X 9.
- templates/class-template-deduction: Q 6, X 9, 29 uncited tests.
- foundations/constant-evaluation: Q 8, X 13, F 120.
- exception-specifications: 5 questions on 10 rules.
- multiplicative and additive: 9 single-test rules each. Lane U has not run.
- logical: 8 of 14 rules single-test. Lane V has not run.

## 2. The 270 questions: which clusters to settle first

Of the 270 provisional answers, 175 rest on "two of three implementations agree" or similar wording. One question has an outside opinion. 74 rows of `questions/README.md` give only "option 1" as the provisional answer, so the index does not say what the specification does. 11 questions affect process or the charter, and no page.

| Order | Cluster | Questions | Why first | Cost |
|---|---|---|---|---|
| 1 | Housekeeping | Duplicates Q-0045 and Q-0057 (STO-11 and UNI-7 state one rule twice), Q-0041 and Q-0097, Q-0024 and Q-0112. Moot now that phase 4 is drafted: Q-0113, Q-0118. The 16 "compiler defect" questions | Closes about 25 questions with no new evidence | under one session |
| 2 | Constraints as a tie-breaker | Q-0142, Q-0146, Q-0160, Q-0162, Q-0210, Q-0265 | One model spread over OVL-19, OVL-30, OVL-41, and ORD-13, and the wordings have drifted | one session |
| 3 | A braced list binding a reference or an array | Q-0081, Q-0132, Q-0135, Q-0136, Q-0149, Q-0150, Q-0154, Q-0155, Q-0157, Q-0164 to Q-0167, Q-0188 (15) | Covers the weakest page (references) and the fifth weakest (conversion-sequences) | one to two sessions |
| 4 | Must an implementation diagnose? | Q-0053, Q-0062, Q-0066, Q-0079, Q-0084, Q-0128, Q-0172, Q-0173, Q-0195, Q-0197, Q-0223, Q-0228, Q-0241, Q-0247, Q-0257, Q-0264 (16) | A single policy for `no-diagnostic` against `reject` settles all 16, and dependent-names leans on it most | one decision, then one session |
| 5 | Partial ordering details | Q-0147, Q-0148, Q-0152, Q-0187, Q-0211, Q-0214, Q-0216 to Q-0218 | The object parameter and rewritten candidates drive 9 overrides on ordering | one session |
| 6 | The edge of constant evaluation | 27 questions mention it, among them Q-0001 to Q-0005, Q-0090, Q-0117, Q-0192, Q-0193 | Includes the round-1 lane U questions. Interpreter work (decision 0048) needs these answers | two sessions |
| 7 | Template syntax corners | Explicit instantiation (Q-0233 to Q-0237, Q-0254 to Q-0256); `template` and `typename` keywords (Q-0224 to Q-0227, Q-0245) | Low reader value. Settle them together under a standing rule | under one session |

The maintainer should also decide one standing rule: "where the newest version of each baseline family agrees, the provisional answer becomes settled unless a gap review objects". About 100 questions would close under that rule. The risk is that a majority of implementations can share a defect. The gap reviews by a source reader are the check on that risk.

## 3. Ranked work items

| # | Item | Evidence | Unblocks | Size | Risk |
|---|---|---|---|---|---|
| 1 | **Fix the editor's instruments.** `tools/status_table.py` lists six sections and leaves out `templates`. The status table therefore has no row for 11 pages and 352 rules. Status also says EDG has per-compiler expectations on 61 tests, and 158 tests carry one. `exercises.md` still says the front end is at round 5 with 3161/3776; the tool now records 3500/4348. Cite the 117 uncited tests when the R and P repairs land | counts above | Every other item, because they rank by these columns | under one session | low |
| 2 | **Settle question clusters 1 to 4** (section 2). Rewrite the 74 "option 1" index rows so each states the answer | section 2 | Pages 1, 2, 5, 6, and 3 | two to three sessions, plus maintainer time | medium: settling by majority can lock in a shared defect |
| 3 | **A consistency pass over the templates chapter and the pages it shares models with.** Give each cross-page model one home. The constraint tie-breaker sits in ORD-13 and is restated in OVL-30 and OVL-41 with fewer conditions. The anonymous-union rule sits in both STO-11 and UNI-7. Add "substitution failure" to the glossary. The term is used on deduction, constraints, packs, and CTAD, alongside "deduction fails", "substitution fails", and "deduction failed". Bring concepts, constraints, packs, and CTAD (one example block each) up to the three or four blocks the other template pages have. Split ordering's two principles | `grep` counts in section 1 | Predict batches on templates, the front end's round 8 | one to two sessions | low |
| 4 | **Run clean-room exercises on the templates chapter.** F is 0 on all 11 pages. The front end passes 58 of 379 template tests (141 missed rejects, 168 unsupported). Run an R-templates Predict batch. Add a Write suite for generic code; the Haskell persona (type classes) maps onto concepts | `conformance.tsv`, `exercises.md` section 8 | Real F counts for pages 3, 4, 6, 7, 9, 10 | two sessions per batch | medium: the first templates batch will produce many findings at once |
| 5 | **Thicken the single-test template rules.** 116 of 352 template rules have one test. Start with packs (17), concepts (17), deduction (14), dependent-names (13), and declarations (13). The R and P repairs on disk cover part of packs and concepts | section 1 | Item 4 scores that mean something | two sessions | low |
| 6 | **Restructure the site around overloading.** Foundations has 13 pages and 258 rules. conversion-sequences and overload-resolution make up 90 of those rules and 32 of the questions. overload-resolution links 22 times into the templates chapter, which comes later, and it comes before `names` in the order even though lookup comes first. Move conversion-sequences and overload-resolution into an "Overloading" chapter between classes and templates, and put `names` before them. Move rule-like sentences out of their 304- and 290-line Evidence segments into rules or questions | `foundations/index.md`, link counts | A reading order that matches the dependencies; smaller Evidence segments | one to two sessions | medium: published URLs change. Rule IDs stay the same |
| 7 | **Finish the round-1 lanes.** U: multiplicative, additive, and comparison still have 9 single-test rules each and 5, 2, and 7 questions. V: logical 8 of 14 rules single-test, initialization/static 12 tests, blocks 15. X: alignment and attributes have 13 overrides each, exception-specifications 6 | section 1, round-1 editor report | W1 and W2 write suites | three to four sessions | low |
| 8 | **Scope: add implementation limits** (see section 4) | INST-15, Q-0012, DCL rule on the largest array | Correct classes for compile-time limits | one session | medium |
| 9 | **Scope: add class properties in a narrow form** (see section 4) | 2 *(class properties referenced)* markers (members.md, copy-move.md), Q-0266, deferred items S and T | Triviality rules that the switch, jump, and union pages and the constraints chapter already use | two sessions | medium-high |
| 10 | **Watch dependent-names and instantiation after cluster 4.** Re-measure their 6 and 3 `no-diagnostic` rules and their 16 and 17 overrides. Move each rule that the policy turns into `reject` | section 1 | The front end's missed rejects (141 on templates) | one session | low |

## 4. Scope decisions

| Topic | Proposal | Case for | Case against |
|---|---|---|---|
| Implementation limits (instantiation depth, constant-evaluation steps, the largest array) | Add. One `impl-defined` rule per kind of limit, no minimums, on the translation page | INST-15 calls an endless chain `undefined`, yet every baseline family rejects it with a depth message. Q-0012 already treats the array limit as implementation-defined. Templates and `constexpr` code reach these limits often | The limits are tunable with compiler flags. A test at a fixed depth is slow and can break when defaults change (general knowledge) |
| Class properties (trivial, trivially copyable, standard-layout) | Add the definitions the language itself uses. Keep the library traits referenced | JMP-6, SW-5, UNI-5 and Q-0266 use triviality. Generic code observes these properties through the traits. Two markers remain | Observing the traits needs library headers or vendor built-ins. Deferred items S and T wait on that decision |
| Extended floating-point types | Keep out. Keep the three markers (conversions.md, conversion-sequences.md twice) | They are part of C++23 and change conversion ranks | They need `<stdfloat>`. Support differs between Clang and GCC (general knowledge). They add rows to the hardest page (conversion-sequences) |
| Language after C++23 (Q-0007) | Keep out until the backlog is settled | Pack indexing and other new template features land in C++26 (general knowledge) | 253 provisional answers for C++23 are still open |
| Coroutines, modules, threads and the memory model, the general library, the preprocessor beyond `#include` | Keep out, as the charter says | Nothing new. The front end rejects valid `#include` tests, but the tooling can fix that with a stub | Each is a chapter as large as templates, and each carries a high similarity risk |

## 5. Process changes

- `tools/status_table.py` should read the section list from `specification/index.md`. A hard-coded list leaves out a chapter without any error.
- `tools/check_specification.py` should fail when two questions share an `Affects:` rule and their titles match, and when a rule restates another page's rule ID with different conditions. The second check can only flag such rules for review.
- `questions/README.md` should be generated from each question's "Provisional answer" section, so no row reads "option 1".
- Status should gain a "templates" row group and a per-chapter line with total rules, single-test share, and questions per rule. The comparisons in this report were made by hand.

## 6. Files

Scratch scripts for these counts (read-only, outside the repository):
- `/tmp/claude-1000/-home-ryanm-Code-tidal/f142cdb8-5f85-4266-b9df-c7416ea6d3a0/scratchpad/stats.py`
- `/tmp/claude-1000/-home-ryanm-Code-tidal/f142cdb8-5f85-4266-b9df-c7416ea6d3a0/scratchpad/qs.py`
- `/tmp/claude-1000/-home-ryanm-Code-tidal/f142cdb8-5f85-4266-b9df-c7416ea6d3a0/scratchpad/rej.py`

This report is for `/home/ryanm/Code/tidal/reports/clean-room/roadmap/` (round 2, editor). The integrator saves it.

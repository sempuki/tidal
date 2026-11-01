# Roadmap, round 3: what to do after phase 5, from an editor's view

Date: 2026-11-01. Decision 0029. A clean-room reader in the role of the editor who owns the specification's quality wrote this report. The reader read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, style guide, specification format, and decisions 0033, 0039, 0044, 0050), `reports/clean-room/roadmap-2/`, `specification/`, the headers of `tests/`, `questions/`, and the phase 5 exercise reports (R-templates, D-classes, W15, X5 to X8, front-end round 9). The reader did not read the other round-3 reports. Read-only scripts made the counts from the tree at commit 5eff7cd. Statements marked "general knowledge" do not come from Tidal's files.

Columns are the same as in round 2, with one change. **R/T** is rules and distinct tests cited. **Q** is open questions whose `Affects:` line names a rule on the page. **F** is lines in `reports/clean-room/` that cite a rule ID on the page. F now leaves out the roadmap reports, because those reports cite rules only to rank them. **X** is cited tests with a per-compiler override. **1t** is rules backed by exactly one test.

Repo-wide counts:
- 1633 rules (1541 in round 2). 297 of them (18%) rest on one test, down from 446 (29%). The templates chapter has 2 such rules out of 374.
- 5408 tests. Every test is cited by a rule (117 were uncited in round 2).
- 436 tests (8%) carry an override: EDG 170, GCC 159, Clang 80, Clang 18 alone 55, GCC 14 alone 33. 55 tests have overrides only for an older version.
- 51 rules rest only on tests that carry an override.
- 321 questions: 288 provisional, 20 "compiler defect", 12 settled, 1 open. Phase 5 opened 51 (Q-0271 to Q-0321) and closed 10.
- Predict batches have matched 1320 of 1323 answers over 18 pages. Phase 5 exercises gave 155 findings: 56 Predict, 17 Diagnose, 9 Write, 64 Propose, and 9 front end.

The templates chapter has 104 open questions on 374 rules (0.28 per rule). The other chapters have at most 0.17. Templates have 150 override tests (13% of their tests, against 7% elsewhere). Evidence takes 18% of the chapter's lines, and 16% of Foundations.

## 1. The ten weakest pages

| # | Page | R/T | Q | F | X | 1t | Other evidence |
|---|---|---|---|---|---|---|---|
| 1 | foundations/overload-resolution | 44 / 164 | 16 | 129 | 26 | 5 | 308 of 1108 lines are Evidence. Links into the templates chapter rose from 22 to 25. 12 of its questions are tie-breaker splits, and three of them split all three families (Q-0148, Q-0152, Q-0153). X5 and X8 found 9 gaps on it |
| 2 | foundations/conversion-sequences | 47 / 114 | 15 | 78 | 23 | 12 | 295 of 1103 lines are Evidence. ICS-35, ICS-44, and ICS-46 rest only on overridden tests. Q-0294 groups the braced-list questions, and its 14 child splits stay open |
| 3 | templates/constraints | 39 / 122 | 18 | 43 | 26 | 0 | The most questions on any page (10 in round 2). CNS-18, CNS-32, and CNS-38 rest only on overridden tests. RA-8 to RA-11 found unclear wording on CNS-9, CNS-10, and CNS-22 |
| 4 | declarations/initialization/references | 14 / 62 | 12 | 60 | 14 | 2 | 0.86 questions per rule, still among the highest in the tree. REF-P3 now holds the braced-list model, but the splits under Q-0294 are unsettled. REF-15 rests only on overridden tests |
| 5 | templates/dependent-names | 39 / 117 | 14 | 72 | 20 | 0 | 6 `no-diagnostic` rules, the most on any page. 965 lines, the third longest page. Overrides rose from 16 to 20. X5-9, X6-8, and X8-5 found cases missing from DEP-2's list of dependent names. 18 front-end missed rejects |
| 6 | templates/instantiation | 36 / 130 | 15 | 54 | 18 | 1 | INST-15 still classes an endless instantiation chain as `undefined`, and every family still rejects it. The explicit-instantiation questions (Q-0233 to Q-0237, Q-0254 to Q-0256) are unsettled. X6-6 and X6-12 found rules hard to combine |
| 7 | templates/ordering | 16 / 41 | 7 | 25 | 11 | 0 | 27% of its tests have overrides (25% in round 2). 132 of 474 lines are Evidence. Two principles still cover 16 rules. Q-0216 to Q-0218 split all three families. RA-12 found a winner hard to read, and RA-13 found duplicate Evidence bullets |
| 8 | templates/class-template-deduction | 29 / 113 | 12 | 32 | 19 | 0 | 171 lines of Evidence. 8 Predict findings (RB-1 to RB-8), the most on one page. Q-0276 to Q-0280 are all EDG-only splits |
| 9 | expressions/new-delete | 39 / 116 | 12 | 46 | 18 | 11 | 740 lines. 7 rules have no test that cites them alone. NEW-35 and NEW-36 rest only on overridden tests (Q-0301, Q-0302) |
| 10 | declarations/alignment | 8 / 35 | 7 | 2 | 15 | 2 | 0.88 questions per rule and 43% override share, both the highest. Every override names a question, so lane X digested the page, but none of its GCC questions (Q-0069 to Q-0071) is settled. Only W15 has reached the page |

Close behind:
- templates/specialization: Q 11, X 13, and F 15, the lowest F in the templates chapter.
- templates/concepts: Q 11, X 17, and 12.3 rules per principle, the highest ratio.
- foundations/language-support: 12 of 42 tests have overrides (29%). 5 rules have no test of their own. The front end passes 50% of its tests.
- Single-test rules outside templates: lambdas 14 of 28, storage-linkage 11 of 23, new-delete 11, members 10, conditional 9 of 15.
- Nine pages that no exercise other than the front end has reached: inline, linkage-specifications, enumerations, namespaces, type-names, bitwise, types, blocks, and loops.

## 2. The question clusters to settle first

The backlog grows faster than it closes. Phase 5 opened 51 questions and closed 10. The index column gives these bases for the open answers:
- 66 rows say only "option 1" or "option 2". All of them are Q-0038 to Q-0111, so the index does not say what the specification does for them.
- 77 rows rest on two families against one. GCC is the one in 32, EDG in 28, Clang in 17.
- 30 rows have all three families differing.
- 15 rows say that the current versions agree and an older version differs.
- 11 of the 20 "compiler defect" questions say that a newer version fixed the defect (Q-0015, Q-0023, Q-0026, Q-0044, Q-0072, Q-0101, Q-0125, Q-0177, Q-0181, Q-0206, Q-0308).
- 9 open questions affect no page (Q-0006, Q-0007, Q-0008, Q-0116, Q-0119, Q-0182, Q-0183, Q-0185, Q-0186). Q-0312 is the only question with the status "open".

| Order | Cluster | Questions | Why first | Cost |
|---|---|---|---|---|
| 1 | Housekeeping under a standing rule | The 11 fixed compiler defects above. The 8 questions where only Clang 18 or GCC 14 differs (Q-0169, Q-0261, Q-0270, Q-0288, Q-0289, Q-0291, Q-0297, Q-0319). About 7 where every compiler agrees with the answer (Q-0010, Q-0243, Q-0257, Q-0271, Q-0275, Q-0300, Q-0305). Rewrite the 66 "option N" rows | Closes about 26 questions and fixes 66 rows with no new evidence | one session |
| 2 | Overload tie-breakers | Q-0133, Q-0137, Q-0138, Q-0139, Q-0144, Q-0145, Q-0147, Q-0148, Q-0152, Q-0153, Q-0156, Q-0159, Q-0161, Q-0163, Q-0187, Q-0188, Q-0189 (17) | Covers pages 1 and 2. Three of them split all three families, so a majority cannot decide them | two sessions, and a source-reader inventory of the tie-breaker order |
| 3 | Partial ordering | Q-0211, Q-0214, Q-0216, Q-0217, Q-0218, Q-0219, Q-0220, Q-0285, Q-0293 (9) | Round 2 ranked this cluster fifth, and phase 5 did not take it. Q-0216 to Q-0218 split all three families. Covers page 7 | one session |
| 4 | Constant evaluation and undefined behavior | Must constant evaluation reject the behavior: Q-0002, Q-0003, Q-0013, Q-0212, Q-0307, Q-0309, Q-0318, Q-0320. The edge of a constant expression: Q-0001, Q-0090, Q-0117, Q-0192, Q-0193, Q-0303 | The interpreter (decision 0048) and the front end need these answers. EDG or GCC accepts in most of them | one to two sessions |
| 5 | EDG alone on templates | 28 rows where Clang and GCC agree and EDG differs, among them Q-0224, Q-0229, Q-0231, Q-0235, Q-0237, Q-0245, Q-0256, Q-0260, Q-0266 to Q-0268, Q-0272, Q-0277, Q-0278, Q-0280, Q-0285, Q-0286, Q-0292 | One decision covers all 28. The maintainer already has the EDG report for Q-0002 and Q-0003 waiting. These questions can go in the same report | one session, plus maintainer time |
| 6 | Template syntax corners | Explicit instantiation (Q-0233 to Q-0237, Q-0254 to Q-0256). The `template` and `typename` keywords (Q-0224 to Q-0227, Q-0245) | Round 2 ranked this cluster seventh, and phase 5 did not take it. Covers page 6. Low reader value | under one session |
| 7 | Process questions | The 9 questions with no page, and Q-0312 | They need the maintainer. Q-0312 has no provisional status yet | maintainer time |

Round 2 asked for a standing rule, and no decision has adopted one. The editor asks again. "Where the newest version of each baseline family agrees, the provisional answer becomes settled unless a gap review objects." Under that rule, a question where only Clang 18 or GCC 14 differs closes at once. A two-against-one question stays open. The risk is that the newest versions can share a defect. A source reader's gap review of each settled cluster is the check on that risk (decision 0019).

## 3. Ranked work items

| # | Item | Evidence | Unblocks | Size | Risk |
|---|---|---|---|---|---|
| 1 | **Settle the backlog.** Adopt the standing rule in a decision. Run clusters 1 to 6 of section 2 in order, with a source-reader gap review after clusters 2 and 3. Mark each settled question in the page's Evidence | 289 open questions. 51 opened against 10 closed in phase 5. 66 index rows give no answer | Pages 1 to 8 of section 1. The interpreter's answers on constant evaluation. An outside review, which needs settled answers to review | three to four sessions, plus maintainer time | medium, because settling by agreement can lock in a shared defect |
| 2 | **Run the Write suites that reach the unexercised pages.** W14 (inline, namespaces, type-names, storage-linkage), W13 (linkage-specifications, alignment, volatile), W3 (blocks, loops), W11 (enumerations), and W1 with `foundations/types` added to its pages | 9 pages reached by no exercise. Of the 15 Write suites, only W0 and W15 have run. `exercises.md` section 7 says a page with no Write suite and no finding gets one before the catalog is done | Real F counts for 9 pages. The coverage rule of the catalog | one session per suite, five sessions | low |
| 3 | **Fix the Predict format, then run Predict on the four chapters with no batch.** Give the Predict reader a copy of the specification without `:tests:` lines. Then build batches for expressions, statements, declarations, and classes | Readers matched 1320 of 1323 answers, so the score no longer tells the editor anything. RB-1 drew its answer from a test named `no-initializer.cpp`. 570 of the 5408 test names hint at the outcome ("reject", "ambiguous", "undefined", and so on). 920 rules in four chapters have no Predict batch | Findings on the chapters that phase 5 did not exercise. A score that means something | one session for the format, then about two sessions per chapter | medium, because the first batches will bring many findings at once |
| 4 | **Make an Overloading chapter, and slim the Evidence.** Move conversion-sequences and overload-resolution into a chapter between classes and templates, and put `names` before them. Split each long Evidence segment into sources, divergences, and generated grids | The two pages hold 2211 lines and 603 lines of Evidence. overload-resolution links 25 times into the later templates chapter. Round 2 ranked this item sixth, and phase 5 did not take it | A reading order that follows the dependencies. Evidence that a reader can scan | one to two sessions | medium, because published URLs change. Rule IDs stay the same |
| 5 | **Thicken the thin evidence outside templates.** Add a second test to the single-test rules, starting with lambdas (14), conversion-sequences (12), storage-linkage (11), new-delete (11), members (10), and conditional (9). Add a test where every compiler agrees for the 51 rules that rest only on overridden tests | Section 1 counts. Decision 0044 asks for a test that only one step or tie-breaker decides. By a rough proxy (a test that cites only the rule), ICS-4, ICS-40, ICS-41, and ORD-2 have none | Item 3 scores on these chapters. Cluster 2 of section 2 | two sessions | low |
| 6 | **A consistency pass.** Add "substitution failure" to the glossary (round 2 asked for this; the page text uses "substitution failure", "substitution fails", "deduction fails", and "deduction failed"). Replace "is an error" (12 uses on 9 pages) with the words of the word list. Split ordering's two principles. Split the principles of concepts (12.3 rules per principle) and templates/declarations (12.2). Merge the duplicate Evidence bullets of RA-13. Update the status line of decision 0033, which is still provisional on Q-0113 although Q-0113 is settled | `grep` counts. RA-13 | Predict batches on templates that read more easily | one session | low |
| 7 | **Scope: implementation limits** (section 4) | INST-15 is still `undefined` while every family rejects with a depth message. CE-10 already sets limits for constant evaluation only. Q-0012 | A correct class for INST-15. One model for every compile-time limit | one session | medium |
| 8 | **Scope: class properties in a narrow form** (section 4) | 2 *(class properties referenced)* markers. Q-0266. The status follow-up on the copy-move triviality split | The triviality rules that the switch, jump, union, and constraints pages already use | two sessions | medium-high |

## 4. Scope decisions

| Topic | Proposal | Case for | Case against |
|---|---|---|---|
| Implementation limits (instantiation depth, the largest array, the largest object) | Add. One `impl-defined` rule per kind of limit, with no minimums, on the translation page. Reclass INST-15 | INST-15 contradicts its own tests. CE-10 sets the pattern. Q-0012 and Q-0308 already treat sizes as limits | Flags can tune the limits. A test at a fixed depth is slow and can break when defaults change (general knowledge) |
| Class properties (trivial, trivially copyable, standard-layout) | Add the definitions that the language itself uses. Keep the library traits referenced | Round 2 made the same case, and the deferred items S and T still wait. Q-0266 and the copy-move follow-up need the definitions | Tests can observe the traits only through library headers or vendor built-ins. Tests allow neither |
| The launder operation | Keep it referenced. Add a rule on the storage page that the pointer a placement `new` returns is the way back to the object | W15-2 found no usable path back to an object in raw storage. 11 of the 21 markers are library markers | A declared `std::launder` without a header fails to link (W15-2), so no test can use it |
| Extended floating-point types | Keep out. Keep the 4 markers | They are part of C++23 and change conversion ranks | They need `<stdfloat>`. They would add rows to page 2, the second weakest page |
| Language after C++23 (Q-0007) | Keep out until cluster 1 to 6 settle | Proposals X5 to X8 each name a need outside C++23 (X5-19, X6-15, X7-12, X8-17) | 289 provisional answers for C++23 are still open |
| Lexical structure beyond G1 (Q-0312) | Keep out. Settle Q-0312 as "no", so the grammar is the only statement of free token sequences | X6-13 asked for a list of free forms such as `for constexpr` | A list of invalid token sequences has no end |
| Coroutines, modules, threads and the memory model, the general library, the preprocessor beyond `#include`, MSVC | Keep out or deferred, as the charter and the roadmap say | Nothing new since round 2 | Each is a chapter as large as templates, and each has a high similarity risk |

## 5. Process changes

- **Let tools write every number on the status page.** Commit 5eff7cd fixed four hand-written numbers. The EDG row still says "per-compiler expectations on 61 tests", and 172 tests carry an EDG override (round 2 found 158). `tools/status_table.py` should write the test count, the question counts by status, the marker counts, and the override counts. `tools/check_specification.py` should fail when a number is stale.
- **Make the status columns measure what they claim.** The Similarity column is the literal text "0 flagged" in `status_table.py`, for every page. Read the column from the latest similarity report, or drop it. The "Report mentions" column counts the roadmap reports, so each roadmap round raises the counts of the pages it ranks. Leave out `reports/clean-room/roadmap*`.
- **Generate `questions/README.md`** from each question's status and "Provisional answer" section. Round 2 asked for this. The 66 "option N" rows remain.
- **Count questions opened and closed per phase.** Each phase decision should state both numbers. A phase that opens more than it closes should say why.
- **Take test names out of the Predict reader's copy.** `tools/exercises.py predict` should build a copy of `specification/` without `:tests:` lines, so the score measures the text. Until then, report Predict results by findings and not by score.
- **Add two warnings to `tools/check_specification.py`.** One warns for a rule whose every test carries an override (51 rules). The other warns, on the procedure pages of decision 0044, for a step or tie-breaker rule with no test that cites it alone. The second warning is a proxy, so it should not fail the check.
- **Pair each settled cluster with a gap review.** A source reader's review after each cluster of section 2 (decision 0019) is the only check on a majority that shares a defect.

## 6. Files

The reader made these counts with read-only scratch scripts outside the repository.

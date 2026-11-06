# Roadmap, round 8: what to do after phase 10, from a clean-room writer's view

Date: 2026-11-05. Decision 0029, with decisions 0033, 0039, 0044, 0050, 0051, 0054, 0055, 0056, and 0057. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, style guide, specification format, exercises, decisions 0056 and 0057, and `decisions/pending.md`), its own round 7 report `reports/clean-room/roadmap-7/writer.md`, the phase 10 reports (R10, W18, W19, I3 with its notes, PU3, X22 with `X22/X19-refresh.md`, `roadmap-7/portfolio.md`, `rust-frontend/report-22.md` and `ledger.md`), the three 2026-11-05 gap reviews under `reports/review/`, the headers of the question files and tests, and `git log f0f5331..1e20bfa2` with commit bodies. It did not read the other round 8 reports, `inputs/`, or `build/exercise-keys/`. All counts come from read-only `grep`, `git`, and short Python scripts kept outside the repository (`scripts/` beside this report), and from read-only runs of `tools/check_specification.py`, `tools/question_index.py --check`, and `tools/propose_check.py`. Comparisons with round 7 read commit 017b401 (the tree that round 7 read) with `git show` and `git ls-tree`. Two probe grids ran with `tools/probe_grid.py` on the baseline compilers, and one probe ran on the host compilers (`probes/` beside this report). Commit read: `1e20bfa2`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 9 (017b401) | Now (1e20bfa2) |
|---|---|---|
| Rule entries | 1697 (15 withdrawn), 1682 live | 1697 (15 withdrawn), 1682 live |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 78 rules and 35 principles on 45 pages, and 1 new rule (phase 9) | 39 rules and 10 principles on 32 pages; no new rule and no class change |
| Words on the 85 pages, without index and glossary | 262,940 | 265,585 |
| Evidence words on those pages (this report's script) | 66,522 | 67,700 |
| `cpp` example blocks | 219; the front end passes 175 of 177 that it runs | 220; the front end passes 175 of 177 that it runs |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6619, of which 85 are program tests | 6681, of which 97 are program tests |
| Live rules with one test | 1 (CPY-28) | 0 |
| Live rules with exactly two tests | 686 | 677 |
| Rules that rest only on tests with an override for a newest compiler | 28 | 28, each a warning of `check_specification.py` |
| Test files with a per-compiler override | 668 (828 override lines) | 684 (846 override lines) |
| Test files with an override for a newest family (host overrides counted) | 574: EDG alone 198, GCC alone 186, Clang alone 104, EDG and GCC 57, Clang and EDG 18, Clang and GCC 10, all three 1 | 589: EDG alone 204, GCC alone 193, Clang alone 105, EDG and GCC 58, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Test files with an EDG override (any configuration) | 278 | 285 |
| Tests with `tidal-allows-edg` | 5 | 5 |
| Tests with `tidal-interpreter: run-fail` | 19 | 19 |
| Tests with `tidal-error` patterns | 3041 | 3064 |
| Pages with a Split tests part | 64, made by hand | 77, written by `tools/split_tests.py` and checked |
| Question files | 400: 172 provisional, 13 compiler defect, 215 settled | 411: 171 provisional, 13 compiler defect, 227 settled |
| *(referenced)* markers | 33 | 33 |
| Front end (`conformance.tsv`) | 6362 of 6619 pass; 118 missed rejects, 79 unsupported, 52 run-unsupported, 6 false rejects, 2 EDG-AGREES, 0 wrong results | 6459 of 6681 pass; 116 missed rejects, 60 unsupported, 40 run-unsupported, 4 false rejects, 2 EDG-AGREES, 0 wrong results |
| Front-end findings with a ledger row | none (no ledger) | 152: 145 commit, 4 open, 1 question, 1 text, 1 out of scope |
| Committed similarity reports | 173 (20 MB, 276,724 lines) | 184 (24 MB, 321,238 lines) |
| `question_index.py --check` | 0 problems | 0 problems |
| `check_specification.py` | 0 problems, 28 override-only warnings, 1 one-test warning | 0 problems, 28 override-only warnings |

Round 7 gave 66,692 Evidence words and 564 newest-family override files for commit 017b401. This report's scripts give 66,522 and 574 for the same commit. Both columns above use this report's scripts.

Phase 10 added 62 tests and no rule. It opened 11 questions (Q-0401 to Q-0411) and settled 12. The open count went from 172 to 171, one below the aim of decisions 0056 and 0057.

### 1.2 What phase 10 took from round 7

| Round 7 item | Phase 10 result |
|---|---|
| RV3, a gap review of the text phase 9 wrote, and of the held questions | Done in b1d89e7 and 3aa9b4b. Three source-reader reviews read 117 rules and principles, made 21 principle uses, and read the held questions. 19 tests. 10 questions settled. Seven new questions (Q-0404 to Q-0410) went to `pending.md` row 4, and RV3 sent Q-0401 there too. |
| R4, close the phase 9 record | Done in 264b504, 0e5a8cc, bfe3a88, 7e4ddf7, and a9c480b. The note form became a `**Waits for:**` field that `question_index.py --check` requires. `split_tests.py` writes the Split tests tables, and seven pages gained one. Q-0290 has a correct note. Predict R10 on the objects page scored 61 of 61. CPY-28 has a second test. The Divergence column was not built. The integrator checked the three notes of round 7 by hand. |
| G, answers for G147 to G151 | Done in a9c480b, with a ledger of all 151 findings and a check for a finding with no row. The answers match the round 7 probes. G148 became Q-0401, G150 joined Q-0387, and G151 joined Q-0314. |
| PU3, a principle-use batch after RV3 | Done in 75e4fab. The field selected 5 questions with 13 programs, because RV3's uses had settled most of the 20. 5 of 13 answers match, and 2 questions settled. |
| FE22, front-end round 22 | Done in eff1a37. 97 more passes, 0 worse. |
| I3, Implement on overload ranking | Done in 4915c31: 1513 of 1532 decided assertions, 22 missing steps, findings I3-1 to I3-18. |
| D2, Diagnose batches | Not done. Decision 0057 deferred Diagnose a fourth time. |
| Process 1, one form for a held note | Done as the `**Waits for:**` field, which is stronger. |
| Process 2, a repair restarts the wait only where it reaches the question | Adopted in decision 0057. Q-0370 and Q-0376 failed their RV3 use, the writer repaired DEFARG-P2 and SB-P2, and PU3 then settled both. |
| Process 3, generated Evidence parts | Done. `split_tests.py` writes the tables, and `status_table.py` writes the interpreter counts and the Write rows of `exercises.md`. |
| Process 4, a fixed front-end binary for writer commits | Not done. Commit 1b05738 (X22 repairs, 6 tests) records no `frontend_check.py` result. Front-end round 22 was running, and its report counts those 6 tests as arrivals during the round. |
| Process 5, smaller per-commit similarity reports | Not done. Phase 10 added 10 reports and 44,414 lines (phase 9: 8 and 39,105). |
| Process 6, check results written by a tool | Not done. Commit 0e5a8cc says `check_specification.py` reports 0 problems; it reported 1, because its output went through `tail` (f2c8a15 repairs this). Commit b1d89e7 says 14 new tests; it added 11 (3aa9b4b corrects this). Commit 7e4ddf7 ran no full check because the tree held uncommitted edits of item G. |

### 1.3 Where the writer's time went

Phase 10 had 20 commits. Thirteen were writer or integrator commits, five saved reader exercises (R10, W18 and W19, I3, X22, FE22), and two were the decision and the close. Six repair commits added the 62 tests.

| Commit | Source | Tests | Rules and principles changed |
|---|---|---|---|
| a9c480b | G and R4 | 8 | ALN-8, BF-1, CMP-8, COND-16, INST-12, ORD-6 |
| 7b9cd71 | T | 18 | CLS-13, CONV-8, LAM-15, LSUP-2 |
| b1d89e7 | RV3 part A, W19 | 11 | 9 rules and OPR-P3, UNI-P4 |
| 3aa9b4b | RV3 parts B and C | 8 | 14 rules and CONV-P3, DEFARG-P2, ORD-P1, SB-P2 |
| 4a193ea | I3, W18 | 11 (8 of them W18 and W19 programs) | 8 rules and ICS-P1, SW-P1 |
| 75e4fab | PU3 | 0 | SB-P2 |
| 1b05738 | X22 | 6 | CNS-14, CPT-19, DED-24, CNS-P4, DEP-P6 |

RV3 was the largest repair source (23 rules and 6 principles). It also settled 10 questions, and PU3 settled 2. PU3 cost a reader session for 13 programs and gave about one settlement per 6.5 programs. PU1 and PU2 gave one per four.

R4 was the largest record edit. The `**Waits for:**` field touched 172 question files, and the Split tests tool rewrote 77 tables.

The writer did not act on every finding it received. Section 1.7 lists what is left.

### 1.4 Questions

171 questions are open. By the `**Waits for:**` field (a question can hold more than one item):

| Item | At R4 (264b504) | Now |
|---|---|---|
| the maintainer (`pending.md` rows) | 52 | 60 |
| a review | 66 (46 doubt, 14 silent, 5 against, 1 part) | 75 (50 doubt, 15 silent, 9 against, 1 part) |
| newest families that agree | 36 | 39 |
| a use of a principle | 20 | 3 (Q-0263, Q-0299, Q-0395) |
| a first review | 6 | 2 (Q-0403, Q-0411) |
| a principle, a rule, child questions | 1 each | 1 each |

By the whole field, 56 questions wait only on the maintainer, 45 only on a review with doubt, 33 only on newest families that agree, 14 only on a silent review, and 7 only on a review against the answer.

The writer can reach few of these. Two kinds can settle without the maintainer: a use or first review that gives the answer, and a review that supports the answer with no doubt. The writer's pool for phase 11 is about 13 questions:

- the 3 that wait on a use (Q-0263 on CNS-P2, Q-0299 on LSUP-P2, Q-0395 on SB-P2). PU3 found that CNS-P2 and LSUP-P2 do not decide their programs (PU3-3, PU3-4), and no commit has repaired them. SB-P2 changed after its use.
- the 2 that wait on a first review (Q-0403, Q-0411).
- 8 doubt-held questions whose Affects line names a rule or principle that changed in phase 10: Q-0152 (ICS-28), Q-0156 and Q-0337 (ICS-45), Q-0293 (ORD-13), Q-0378 (COND-16), Q-0383 (OPR-9, OPR-P3), Q-0386 and Q-0402 (LSUP-2).

Row 4 grew from 29 questions (e9b55af) to 37. Eight of the 11 questions that phase 10 opened went there. In five of them (Q-0405, Q-0406, Q-0407, Q-0409, Q-0410) every newest family keeps the matrix's answer against the review. 98 open questions now wait on a `pending.md` row.

The field and the test overrides can disagree, and no tool compares them. A crude script takes, for each open question, the tests that its Affects line names or whose notes name it, and the most newest families that differ in any one of them. It finds 16 candidates:

- 4 whose field says "newest families that agree" while no test has more than one newest family that differs: Q-0142, Q-0274, Q-0323, Q-0341.
- 12 held by a review, with no "newest families" item, where some test has two newest families that differ: Q-0025, Q-0033, Q-0134, Q-0152, Q-0156, Q-0161, Q-0192, Q-0193, Q-0337, Q-0346, Q-0349, Q-0383.

A test can carry an override for another question, so some of these are false hits. Each needs a reading of the test notes.

### 1.5 Evidence

Every page that carries a test with an override for a baseline compiler has a Split tests part, and a tool writes it (77 pages). The five template pages lost their grid tables to `tests/<page>/grids/tables.md` (13 such files now). Evidence grew by 1,178 words, mostly from the seven new Split tests parts and six new Divergences lines. The largest Evidence segments are translation (2,335 words), constraints (2,293), instantiation (2,095), overload resolution (2,069), new and delete (2,014), dependent names (1,965), and class template argument deduction (1,939).

### 1.6 Coverage by readers and reviewers

- **Gap reviews.** RV3 read the text phase 9 wrote and the G and T text before its repairs. No review has read 34 rules and 10 principles, on 30 pages, in the form they took in or after commit b1d89e7. They are ACC-20, ALN-8, AUTO-19, BF-1, CLS-2, CLS-9, CLS-26, CMP-8, CNS-14, CONV-9, CPT-19, CTAD-16, CVF-7, DCMP-1, DED-24, ENUM-10, ICS-28, ICS-45, IF-25, LAM-6, LAM-15, LAM-29, LSUP-2, NAM-21, OPR-9, ORD-4, ORD-13, REF-8, RET-2, SINIT-4, SINIT-5, SINIT-6, TARG-27, TR-4, and the principles CNS-P4, CONV-P3, DEFARG-P2, DEP-P6, ICS-P1, OPR-P3, ORD-P1, SB-P2, SW-P1, UNI-P4. Decision 0057 says phase 11 reads them.
- **Predict.** Every page has a batch without test names. R10 closed the objects page at 61 of 61.
- **Principle use.** PU1, PU2, PU3, and the 21 uses inside RV3.
- **Implement.** I1 (class layout), I2 (partial ordering), and I3 (overload ranking). Each scored at or near full marks and found 13, 22, and 22 missing steps.
- **Write.** W18 and W19 gave 21 findings, 8 program tests, and 10 front-end faults. The `r7-` programs of item T add four more program tests.
- **Diagnose.** D-classes only, 4 pages. 3064 tests carry `tidal-error` patterns. In front-end round 22, the first error of a passing reject named a cited or linked rule in 2004 of 2609 cases.
- **Propose.** X22 on P23a. `propose_check.py` lists 22 reports, 125 rows whose rule changed after the report's commit, and 31 conflicts in the older reports (10 withdrawn, 4 class, 17 new-id). The portfolio has four ready proposals (P22, P24, P25, P26).
- **Port.** None. Decision 0057 asks each phase for a Write or Port session.

### 1.7 Findings with no answer

Decision 0057 gave front-end findings a ledger. Findings of other exercises have none, and some were dropped.

| Finding | State at 1e20bfa2 |
|---|---|
| R10-1 to R10-7, R10-9 | Commit 7e4ddf7 says the writer acts on them in RV3's repair pass. No later commit names them, and OBJ-1, OBJ-2, OBJ-11, CE-22, and RET-6 did not change in phase 10. Eight asks: sanitizer notes for a read after a block ends and after `delete`, when the object of a delegating constructor counts as complete, the return of a local with an element that has no value, what counts as a use of a `thread_local` object, the reject rule for a read of storage with no object, and a `decltype(auto)` row for a parenthesized name. |
| I3-1, I3-5, I3-13 | 4a193ea says they need probes of their own. No record holds them. |
| PU3-3 (Q-0263), PU3-4 (Q-0299) | Dated notes only. CNS-P2 and LSUP-P2 are unchanged. |
| X19 refresh, section 4 | `literal-builtin-suffix-wins.cpp` is a `run` test of OPR-8 whose program declares `operator"" u` and other suffixes without `_`, which OPR-9 makes not valid with no diagnostic required. 1b05738 sends it with Q-0383 to the next review. |
| G152 | Ledger row, state open. |
| T leftovers | 7b9cd71 left the placement `new` that is declared and not defined (LSUP-7 or ODR-8) and the LAM-18 note on Clang's unused-capture warning for RV3. RV3's reviews do not name either. |
| G4, G13, G25 | Ledger rows open since reports 2 and 3. G13 asks for TYP-2, which the types page now has. G25 asks for overload ranking, which the overloading pages now give. G4 asks about CE-23's literal-type list, which now has 11 tests, among them `literal-one-constexpr-constructor.cpp`. |

The writer probes give these results (`probes/README.md`).

| Finding | Probe | Clang 18, Clang 22 | GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|---|---|
| G152 | `g152-candidate-body.json`: a plain call operator against a generic one whose body is not valid for the argument; deduced, trailing `decltype`, and declared results; a run and a `static_assert`; 15 cells | accept all | accept all | accept all |
| PU3-4, Q-0299 | `pu3-4-align-val-t-shape.json`: four shapes of `std::align_val_t`, with no use, with a `new` of an over-aligned type, and with a cast | accept every shape | reject every shape but a scoped enumeration of `size_t`, with no use; accept a cast with no declaration | reject the same shapes as GCC; reject a cast with no declaration |
| T leftover, placement `new` | `placement-new-declared-only.cpp`, host only | builds and runs with no use; link fails with a use | the same | not run |

G152 shows no split, so one sentence and one test can answer it. The answer for PU3-4 matches Q-0299's tests and adds one fact: GCC declares `std::align_val_t` in every unit.

### 1.8 Front end and the writer

Round 22 passed every C++23 call form of item T and every phase 10 rule change that a new test checks. It took 36 old tests to a pass. Of the 222 tests that do not pass, the old groups that round 22 did not target hold most of the missed rejects: access (6), DEFARG-13, LIST-13, LIST-16, PACK-8, MPTR-11, DCMP-12, and DCMP-13 (17), and the nine instantiation rules (9). One phase 10 test fails (Q-0406, CLS-2).

The ledger makes front-end findings visible. Its four open rows are G152 and three stale rows from rounds 2 and 3.

### 1.9 Similarity

Four flags in phase 10, each reworded before its commit. Three of them were in reader text that the integrator saved (PU3-1, X22 section 1.2, report 22). One was a writer sentence in SB-P2 (3aa9b4b). Five reader reports came back as messages because the tool refused the reader's write (R10, W18, I3, X22, report 22), and the integrator saved each. Volume is still the cost. `reports/similarity/` holds 184 files and 24 MB.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | F. Answer the phase 10 findings that no commit acted on | 8 R10 asks, I3-1, I3-5, I3-13, PU3-3, PU3-4, the X19 gap, G152, 2 T leftovers, 3 stale ledger rows; probes in section 1.7 | 0.5 to 0.75 | low | RV4 reads repaired text; uses for Q-0263 and Q-0299 |
| 2 | RV4. A gap review of the text phase 10 and item F wrote, with the 13 questions in reach | 34 rules and 10 principles unread in their final form; Q-0403, Q-0411; 3 uses; 8 doubt-held questions with changed rules | 0.5 to 0.75 of repairs | medium | up to 13 settlements; the first reading of phase 10's text |
| 3 | R5. Close the phase 10 record | dropped R10 findings; 16 field and override candidates; three wrong check claims | 0.5 | low | item F's ledger stays complete; MSVC later |
| 4 | I4. Implement on constraint normalization and subsumption | constraints names 21 questions, the most of any page; X22-3, PU3-3; Implement found 13 to 22 steps each time | 1 of repairs | low | a check by use of the constraints rules |
| 5 | P1. The first Port suite | decision 0057's choice; no Port suite has run; W18 and W19 gave 21 findings | 0.5 of repairs | low | program tests from another language's idioms |
| 6 | FE23. Front-end round 23 | 116 missed rejects, 60 unsupported, 40 run-unsupported; untargeted groups of 32 missed rejects | 0.25 to 0.5 | low | the regression signal for reject tests |
| 7 | D2. Diagnose batches | deferred four times; 605 passing rejects whose first front-end error names another rule | 1 to 1.5 of repairs | low | a reader check of patterns and rule citations |

Items 1 to 3 cost about 1.5 to 2 writer sessions, plus the source-reader session of item 2. Items 4 to 7 add about 2.75 to 3.5.

### Item 1. F. Answer the phase 10 findings that no commit acted on

- **What.** One sentence, row, or test for each finding of section 1.7, probed on the three newest compilers.
  - R10-1 to R10-7 and R10-9 on the objects, constant evaluation, static initialization, and `auto` pages. The sanitizer notes of R10-1 and R10-2 need a run on the matrix's sanitizer builds first.
  - I3-1: an ICS-26 example that splits `Derived*` to `const Base*` into two steps. I3-5: a row in ICS-11 for a non-class prvalue, and the qualification-adjustment binding in REF-1. I3-13: a probe of two user-defined element conversions through different functions, then a sentence in ICS-25.
  - PU3-3: CNS-P2 says when an atom whose type does not depend on a parameter and is not `bool` is diagnosed, as CNS-39 already decides. PU3-4: LSUP-P2 gives the shape of `std::align_val_t` (a scoped enumeration with underlying type `size_t`), and says that GCC and EDG check a declaration with no use. A test for GCC's implicit declaration of the name joins Q-0299.
  - The X19 gap: `literal-builtin-suffix-wins.cpp` cites OPR-9 beside OPR-8, and its note says that the program breaks OPR-9 and records what every compiler does. Q-0383 gains a dated note.
  - G152: DED-29 or LAM-15 says that forming a candidate does not instantiate its body, and a test has a generic lambda whose body is not valid for the argument. Every compiler agrees.
  - The placement `new` declared and not defined: a link-fail test for the used form, and a run test for the unused one.
  - The LAM-18 note on Clang's unused-capture warning.
  - G4, G13, and G25 move to state `text` with the rule that decides each.
- **Evidence.** Section 1.7. 7e4ddf7 promised the R10 repairs to RV3, and they did not happen.
- **Cost.** 0.5 to 0.75 session.
- **Risk.** Low. Each change is a sentence, a row, or a note.
- **Unblocks.** RV4 then reads the repaired CNS-P2 and LSUP-P2, so the uses for Q-0263 and Q-0299 can give an answer.

### Item 2. RV4. A gap review of the text phase 10 and item F wrote

- **What.**
  - One source-reader gap review, in two parts. It reads the 34 rules and 10 principles of section 1.6 and the text of item F.
  - The same review uses CNS-P2, LSUP-P2, and SB-P2 on the programs of Q-0263, Q-0299, and Q-0395, and gives the first reading of Q-0403 and Q-0411.
  - It rereads the 8 doubt-held questions whose rule changed in phase 10.
  - Then one repair pass by the writer, with probes on the three newest compilers.
- **Evidence.** Sections 1.4 and 1.6. RV3 settled 10 questions in one pass. PU3 settled 2 with a reader session of its own. Three uses fit inside a review at no extra session.
- **Cost.** 0.5 to 0.75 writer session of repairs, plus one source-reader session. The text to read is about 40% of RV3's.
- **Risk.** Medium. The repairs touch the overloading pages (ICS-P1, ICS-28, ICS-45), where prose is dense.
- **Unblocks.** Up to 13 settlements. A reading of the phase 10 text before phase 11 changes it again.

### Item 3. R5. Close the phase 10 record

- **What.**
  - A findings ledger for every exercise (R, W, I, PU, X, D), in the form of the front-end ledger, with a check that fails when a report names a finding with no row. Item F's answers fill the first rows.
  - `question_index.py --check` compares the item "newest families that agree" with the newest families that differ in the question's tests, and fails on a mismatch that has no note. The 16 candidates of section 1.4 get a reading first.
  - A script that runs the check chain and prints the Checks paragraph with the counts, the new tests, and the commit.
- **Evidence.** Sections 1.2, 1.4, and 1.7.
- **Cost.** 0.5 session.
- **Risk.** Low. The edits touch tools and records.
- **Unblocks.** The ledger keeps item F from drifting again. The family check is the step that rounds 5 to 7 asked for before MSVC.

### Item 4. I4. Implement on constraint normalization and subsumption

- **What.** A clean-room reader builds normalization, satisfaction, and the subsumption test from the concepts and constraints pages alone, with Evidence and `:tests:` lines removed. The reader scores the component on `tests/templates/concepts/`, `tests/templates/constraints/`, and a probe grid of pairs of constrained overloads. Partial ordering is a callback, as constraints were for I3.
- **Evidence.** The constraints page names 21 questions, the most of any page, and holds 2,293 Evidence words. X22-3 found that compilers differ on the identity of an atom (Q-0411). PU3-3 found a gap in CNS-P2. I1, I2, and I3 each found more than a dozen missing steps at near full marks.
- **Cost.** 1 writer session of repairs. The reader session is separate.
- **Risk.** Low. The reader writes code, and the writer adopts no wording from it.
- **Unblocks.** A check by use of the procedure that the concepts, constraints, ordering, and overload resolution pages share.

### Item 5. P1. The first Port suite

- **What.** One Port suite as `exercises.md` section 3 describes it: three short programs in a persona's language, and a C++ program with the same behavior, with the interpreter step.
- **Evidence.** Decision 0057 asks each phase for a Write or Port session. W1 to W19 have run and no Port suite has. W18 and W19 gave 21 findings and 8 program tests.
- **Cost.** 0.5 writer session of repairs and program tests.
- **Risk.** Low.
- **Unblocks.** Program tests written from idioms outside C++, which the Write suites do not reach.

### Item 6. FE23. Front-end round 23

- **What.** A front-end round after items 1 and 2. It starts with the phase 11 tests and G152, then the untargeted missed-reject groups (access 6, the DEFARG, LIST, PACK, MPTR, and DCMP group 17, instantiation 9), then Q-0406's default template argument.
- **Evidence.** Section 1.8. 116 of the 222 failures are missed rejects.
- **Cost.** 0.25 to 0.5 writer session to answer findings.
- **Risk.** Low.
- **Unblocks.** The front end checks more of the reject tests that the writer adds.

### Item 7. D2. Diagnose batches

- **What.** Diagnose batches on six pages, as rounds 5 to 7 proposed. The batch picks programs first from the passing rejects whose first front-end error names a rule the test does not cite.
- **Evidence.** Section 1.6. Decisions 0054 to 0057 each deferred Diagnose. The front end's 605 such rejects are a list that costs nothing to make.
- **Cost.** 1 to 1.5 writer sessions of repairs.
- **Risk.** Low. The batch shows a compiler message to the reader, and the writer adopts no wording from it.
- **Unblocks.** A reader check that each `tidal-error` pattern and its rule match how compilers report the error.

## 3. Out of scope

1. **No new topic.** The status page lists no in-scope topic without a page. Phase 10 added no rule and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. The 33 *(referenced)* markers state the boundary. I3-9 asked for extended floating-point ranks and stays out.
3. **The maintainer's rows.** 98 open questions wait on `pending.md`, 37 of them on row 4. The decision packet lists them with their tests. The writer does not choose for the maintainer, and no item here widens a settling rule.
4. **A large drop in the open count.** Section 1.4 puts about 13 questions in the writer's reach. Decision 0057's aim of fewer open questions holds. A larger drop needs a decision on rows 1 and 4.
5. **MSVC stays deferred** until item 3's family check exists. Override lines grew from 828 to 846 in phase 10.
6. **New Propose rounds** (X23 on P22, P24, P25, or P26). X22's repairs cost a commit and opened Q-0411. The portfolio keeps its ready proposals for a phase with fewer open repairs.
7. **Splitting the overloading principles.** I3's text findings were local (ICS-P1, ICS-28, ICS-45, REF-8, CONV-9). Nothing in I3 asks for a split.
8. **Later versions** wait on row 7.

## 4. Process changes

Ranked by writer time saved.

1. **Every exercise finding has a ledger row.** The front-end ledger works, and the R10 findings show what happens without one. A commit that defers a finding names the item that takes it. The check fails when a phase closes with the row still open and no item named.
2. **Check results written by a tool.** This is the third round to propose it. Phase 10 had three commit messages that claimed more or less than the checks did (0e5a8cc, b1d89e7, 7e4ddf7). The `tail` case broke a written shell habit, which a script prevents.
3. **The family check for the `**Waits for:**` field.** The field is checked for form and rows. It is not checked against the overrides, and section 1.4 finds 16 candidates.
4. **A save step for reader reports.** Five reader reports came back as messages, and three of the phase's four similarity flags were in reader text. A script that writes the report, runs `similarity.py --fail-on-flag` on it, and prints the flagged runs makes the integrator's save one step.
5. **A fixed front-end binary for writer commits.** 1b05738 added 6 tests while round 22 ran and recorded no front-end result. Each round leaves its `tidal-fe` at a path that the next rebuild does not touch, and `frontend_check.py` takes the binary path.
6. **Smaller per-commit similarity reports.** A commit's report lists the units of the files it changed, with the flag count, and the phase-close report checks the whole repository. Phase 10 added 44,414 lines.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commit 1e20bfa2, and the three writer probes. The round 7 comparisons come from the same scripts on commit 017b401.

# Roadmap, round 9: what to do after phase 11, from a clean-room writer's view

Date: 2026-11-05. Decision 0029, with decisions 0033, 0039, 0050, 0051, 0054, 0055, 0056, 0057, and 0058. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, its own round 8 report `reports/clean-room/roadmap-8/writer.md` with the probes and scripts beside it, `documentation/status.md`, decision 0058, `git log 1e20bfa2..a15a877d` with every commit body, the phase 11 reports (`reports/clean-room/I4/report.md`, `P1/report.md`, `X23/report.md`, `ledger.md`, `rust-frontend/ledger.md`), the two 2026-11-05 gap reviews of phase 10 text in `reports/review/`, the question files that its view needs (Q-0025, Q-0115, Q-0142, Q-0196, Q-0198, Q-0274, Q-0294, Q-0299, Q-0323, Q-0341, Q-0402 to Q-0418), and the rules and tests that its items name. It did not read the other round 9 reports, `inputs/`, `build/exercise-keys/`, or `frontend/`. Counts come from read-only `git`, `grep`, and short Python scripts in `writer-scratch/` beside this report, and from read-only runs of `tools/check_specification.py` and `tools/question_index.py --check`. Comparisons with round 8 read commit 1e20bfa2 through `git archive` into a scratch directory. Ten probe grids ran with `tools/probe_grid.py` on the baseline compilers, and five probes ran on the host compilers (`writer-probes/README.md`). Commit read: `a15a877d`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 10 (1e20bfa2) | Now (a15a877d) |
|---|---|---|
| Rule entries | 1697 (15 withdrawn), 1682 live | 1697 (15 withdrawn), 1682 live |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 39 rules and 10 principles on 32 pages | 17 rules and 9 principles on 20 pages; no new rule and no class change |
| Words on the 85 pages, without index and glossary | 265,585 | 266,614 |
| Evidence words on those pages | 67,700 | 68,087 |
| `cpp` example blocks | 220 | 220 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6681, of which 97 are program tests | 6711, of which 103 are program tests |
| Live rules with one test | 0 | 0 |
| Live rules with exactly two tests | 677 | 672 |
| Rules that rest only on tests with an override for a newest compiler | 28 | 28, the same 28 rules |
| Test files with a per-compiler override | 684 (846 override lines) | 692 (854 override lines) |
| Test files with an override for a newest family | 589 | 597: EDG alone 208, GCC alone 195, Clang alone 107, EDG and GCC 58, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Test files with an EDG override | 285 | 289 |
| Tests with `tidal-error` patterns | 3064 | 3070 |
| Question files | 411: 171 provisional, 13 compiler defect, 227 settled | 418: 174 provisional, 13 compiler defect, 231 settled |
| Front end | 6459 of 6681 pass (round 22) | 6478 of 6711 pass (`frontend_check.py` at the close; no round ran); 11 of the 30 new tests do not pass |
| Front-end ledger | 152 rows, 4 open | 152 rows: 146 commit, 4 text, 1 question, 1 out of scope, 0 open |
| Exercise ledger | none | 867 rows: 762 commit, 68 open, 26 planned, 5 text, 3 question, 3 out of scope |
| Committed similarity reports | 184 (24 MB, 321,238 lines) | 193 (27 MB, 363,607 lines) |
| `check_specification.py` | 0 problems, 28 override-only warnings | 0 problems, 28 override-only warnings |
| `question_index.py --check` | 0 problems | 0 problems, 5 family-check warnings |

Phase 11 added 30 tests and no rule. It opened 7 questions (Q-0412 to Q-0418) and settled 4 (Q-0263, Q-0386, Q-0395, Q-0413). The open count went from 171 to 174.

### 1.2 What phase 11 took from round 8

| Round 8 item | Phase 11 result |
|---|---|
| 1. F, the phase 10 findings that no commit acted on | Done in 47f58739, with five exceptions. R10-4, R10-6, I3-1, I3-5, and I3-13 went to RV4 as `planned` rows, because each "needs a probe". RV4's commits do not name them, and the close lists them as planned rows whose item ended. G152, PU3-3, PU3-4, the X19 gap, the placement and LAM-18 leftovers, and G4, G13, G25 are answered. |
| 2. RV4, a gap review of phase 10 text | Done in 1e510736 and 7cf4574b. Two reviews read 56 rules and principles. Four questions settled. Seven rules changed. The repair commit lists eight review points it did not act on: LAM-6, LAM-29, AUTO-P3 with RET-2, SINIT-6, SW-P1, TR-P1 with TR-4, and DED-29 on an address with a target type. Section 1.6 shows that probes decide three of them. |
| 3. R5, close the phase 10 record | Done in 5ab0686b (item L, the ledger) and 822cac53 (item R5: `divergence_tags.py`, `checks.py`, and the family check). |
| 4. I4 on constraint normalization | Not done in that form. Decision 0058 took COND-16 for I4. I4 scored 85 of 85 and gave 14 findings. |
| 5. P1, the first Port suite | Done in d859b0ad: three Rust programs, 9 findings. |
| 6. FE23 | Moved to the next phase by the close. |
| 7. D2, Diagnose | Not done. Decision 0058 deferred it a fifth time. |
| Process 1, a ledger row for every finding | Done (item L). |
| Process 2, check results written by a tool | Done as `tools/checks.py`. Commit 5523f0e8 still went in with the line "At least one check failed" in its own Checks paragraph, because the commit was not chained to the script's exit code. 8d3a92b6 repairs the false problem and records the miss. |
| Process 3, the family check | Done. 22 warnings at R5, 17 read as real by PK2, 5 left. |
| Process 4, a save step for reader reports | Not done. The tool refused the writes of the P1, I4, and X23 reports, and the integrator saved each from the transcript. |
| Process 5, a fixed front-end binary | No front-end round ran, so no case arose. |
| Process 6, smaller similarity reports | Not done. Phase 11 added 9 reports and 42,369 lines. |

The "planned" rows whose item ended: 20, as the close lists them. None has an answer at a15a877d. They are I3-1, I3-5, I3-13, R10-4, R10-6 (item RV4); P1-3, P1-5, P1-7, P1-8, P1-9, I4-8 to I4-14, X23-3, X23-6, X23-7 (item E). Six more (P1-1, P1-2, I4-4 to I4-7) wait for front-end round 23, which has not run. X23-7 is half done: `exercises.md` section 4 has the X23 row, and no sentence says which of sections 4.3 and 4.4 a proposal fills.

R10-4, R10-6, I3-1, I3-5, and I3-13 have now been deferred twice (item F to item RV4, then RV4 to the close). The writer probes of this report answer each of them (section 1.5).

### 1.3 Where the writer's time went

Phase 11 had 15 commits. Seven were writer or integrator commits with text or tests, three saved reader or review work (1e510736, 5523f0e8, d859b0ad), two were tools and records (5ab0686b, 822cac53), one was the packet (696e9977), and two were the decision and the close.

| Commit | Source | New tests | Rules and principles changed |
|---|---|---|---|
| 47f58739 | F | 3 | DED-29, LSUP-11, OBJ-1, OBJ-11, AUTO-P3, CNS-P2, LSUP-P2, TR-P1, and notes |
| 8d7f3bb3 | P | 13 (6 program tests) | PACK-13, PACK-P2, ACC-10, CEV-1 |
| a2c246f8 | E, prompt | 0 | none (`exercises.md`) |
| 7cf4574b | RV4 | 6 | CLS-26, CONV-9, ICS-45, NAM-21, OBJ-11, ORD-13, DEP-P6, OPR-P3 |
| ee6085aa | I4, X23 | 8 | COND-16, COND-P2, NEW-P5, ALN-4, NEW-29, NEW-21, STG-5 |

The I4 and X23 repairs took 7 of their 21 findings. The other 14 stay planned.

### 1.4 Questions

174 questions are open. By the `**Waits for:**` field (a question can hold more than one item):

| Item | At the start of phase 11 | Now |
|---|---|---|
| the maintainer (`pending.md` rows) | 60 | 62 |
| newest families that agree | 39 | 56 |
| a review | 75 | 73 (48 doubt, 15 silent, 9 against, 1 part) |
| a use of a principle | 3 | 0 |
| a first review | 2 | 2 (Q-0417, Q-0418) |
| a principle, a rule, child questions | 1 each | 1 each (Q-0196, Q-0198, Q-0294) |

By the whole field, 52 questions wait only on the maintainer, 38 only on a review with doubt, 37 only on newest families that agree, 13 only on a silent review, 10 on a doubt and newest families, and 7 only on a review against the answer.

The writer can reach few of these directly. The pool for phase 12:

- **Q-0323.** Its family item says two newest families differ "in one form", an assignment to an `int` element before `construct_at` in constant evaluation. No test of the repository holds that form, which is why `question_index.py` warns. The probe `q-0323-assign-before-construction.json` finds Clang alone rejecting the form (GCC and EDG accept). The question's own tests have no newest override. Moved to a question of its own, the assignment form leaves Q-0323 with a review that supports the answer and no family that differs.
- **Q-0142 and Q-0274.** Their family items are stale. In Q-0142's tests, only EDG among the newest compilers differs; Q-0142 still waits on row 4. Q-0274's answer is no-diagnostic, both results conform, and only Clang overrides; it still waits on a review with doubt.
- **Q-0341.** The split sits in `tests/classes/members/grids/standard-layout.json`, which the family check does not read. The warning is a false hit.
- **Q-0025.** The rule needs no diagnostic, so every result conforms. The warning is a false hit.
- **Q-0417 and Q-0418** wait for a first review by a source reader.
- **Q-0403, Q-0152, Q-0156, Q-0383** read with no doubt in RV4 and stay open because two newest families differ. No writer step reaches them.

Row 4 holds 36 questions (4a 17, 4b 20 in the packet of 696e9977). 110 of the 174 open questions wait on a row of `pending.md` (63%), up from 57%. Decision 0058 says the next phase's first items include a packet when the share rises. PK2 already wrote one by option, so the maintainer can answer it now.

### 1.5 Findings with no answer, and the writer probes

| Finding | Probe | Clang 18, 22 | GCC 14, 16 | EDG 7.0 | What the text needs |
|---|---|---|---|---|---|
| R10-4 | `r10-4-return-local-unset.json`: return of a local `S` with an unset member; the caller reads the set member | reject every return of a named, parenthesized, or conditional local | accept every form | reject as Clang | A test of the named-local return under CE-22 and Q-0115 with a GCC override, and a run of a prvalue return (accepted everywhere when only the set member is read) with no override. The Q-0115 split-test table then lists the return case of R10-4. |
| R10-6 | `r10-6-static-thread-local-member.cpp`, host | defers the member and constructs it with the other `thread_local` of the unit | the same | runs no program | One OBJ-11 sentence: a `static thread_local` data member, defined at namespace scope, counts. A run test in which the use of a namespace-scope variable starts the member, beside `thread-local-member-init-point.cpp`. SINIT-6's RV4 doubt (it names only the variable itself) takes the same sentence. |
| I3-13 | `i3-13-list-worst-element.json`: arrays and `std::initializer_list` of `int` against `long`, elements that convert by different functions | choose by the first element of the list: `{P, Q}` gives `int`, `{Q, P}` gives `long` | every such call is ambiguous, also `{P, P}`, where one function serves both lists | as Clang | ICS-16 and ICS-17 say which element conversion a list sequence holds when several elements share the worst kind. A new question for GCC's ambiguity (GCC alone among the newest families). Four tests from the grid. |
| I3-1 | `i3-1-pointer-then-qualification.cpp`, host | `Middle*` wins over `const Middle*`; `const Middle*` wins over `Base*` | the same | not run (the grid of I3 covers EDG) | The ICS-26 example of I3-1. No question. |
| I3-5 | `i3-5-qualification-direct-binding.cpp`, host | an `int*` lvalue binds `const int* const&` to the same object; a prvalue binds through a temporary, and `const int*&&` wins | the same | not run | The ICS-11 row and the REF-1 case that I3-5 suggests. `rvalue-reference-before-qualification.cpp` already tests the prvalue case. |
| DED-29 (RV4 doubt) | `ded29-address-target.json`: `int (*p)(const Circle&) = g;` beside a template `g` with a deduced result whose body is not valid for `Circle` | reject (the body is instantiated) | accept | accept | DED-29 says what happens when an address with a target type forms the candidate. A new question: Clang follows the review's reading, GCC and EDG accept. Two tests. With a trailing `decltype` or a declared result, every compiler accepts. |
| AUTO-P3, RET-2 (RV4 doubt) | `autop3-ret2-implicit-move-limits.json` | `volatile int&` for a local `volatile int`, `int&` for a local `int&` | the same | the same | JMP-10 leaves out a `volatile` local and a local of lvalue reference type. Every compiler agrees, so the repair is a sentence and three tests with no override. |
| TR-P1, TR-4 (RV4 doubt) | `two-mains-a.cpp`, `two-mains-b.cpp`, host | the link fails | the link fails | not run | TR-P1 names two definitions of `main` beside the missing one, with a two-unit link-fail test. The class of TR-4 is a separate wording point. |
| X23-3 | `x23-3-thresholds.json` | default new alignment 16, `alignof(long double)` 16 | the same | 16 and 16; EDG ILP32: 16 and 4 | X23-3 says the thresholds agree on every matrix target. On EDG ILP32 the default new alignment (16) is above the alignment of every fundamental type the grid read (at most 8). NEW-27 can state that the default new alignment is at least the largest fundamental alignment on every matrix target, and UNW-3, NEW-22, and ALN-4 can use NEW-28's threshold. |
| X23-6 | `x23-6-underaligned-implicit-object.cpp`, host, UBSan | reports a misaligned member access | reports the same | not run | The sanitizer cannot tell STG-18 (no object) from STG-23 (an object at a misaligned address). The writer adopts X23-6's suggestion for STG-8 as provisional and opens a question. |
| Q-0323 | `q-0323-assign-before-construction.json` | reject | accept | accept | See section 1.4. |

The other planned rows need no probe:

- I4-8, I4-11, I4-12, I4-13, I4-14: sentences in COND-3, COND-11, COND-16, and OVL-P4, as the I4 report suggests. I4-9 and I4-10 also have reader programs that both compilers agree on (`I4/programs/p9-lvalue-xvalue-deleted-move.cpp`, `p3-two-int-refs.cpp`, `p11-step3-after-step1.cpp`). None of them is a test yet.
- P1-3, P1-5, P1-8: examples (a two-alternative class, a link to the `Owner` example from `virtual.md`, a library-free requirement). P1-7: `specification/index.md`'s Baseline compilers section names no base flags; `-std=c++23 -pedantic-errors` appears only in `documentation/`. P1-9: one TR-16 sentence on exact results.
- X23-7: one sentence in `exercises.md` section 4 on sections 4.3 and 4.4.

The RV4 points with no probe stay doubts: LAM-6's term "named capture", LAM-29's inference on finality, SW-P1's "needs", and the class of TR-4. A second review reads them.

### 1.6 Rules that rest on tests with an override for a newest compiler

The 28 rules are the same 28 as at 1e20bfa2. No phase 10 or phase 11 commit changed the set. By class and by the question behind them:

| Group | Rules | Count |
|---|---|---|
| The class allows both results (no-diagnostic, recommended, implementation-defined, conditionally supported) | CINIT-3, CNS-38, CTOR-14, EXS-6, PACK-30, TSP-17, TYN-5 (no-diagnostic); REF-15, TRY-12, TRY-13, UNEV-4 (recommended; GCC gives no warning); ATTR-20, EXS-5 (impl-defined); PRIM-15, PRIM-35 (conditionally supported) | 15 |
| Reject or required, on a settled or compiler-defect question | ALN-7 (Q-0073), CE-28 (Q-0002), CNS-18 (Q-0260), DEFARG-12 (Q-0370), DEP-34 (Q-0245), DTY-5 (Q-0065), NEW-36 (Q-0302), TPL-42 (Q-0199) | 8 |
| Reject or required, on an open question | CE-13 (Q-0033), CTOR-18 (Q-0083), DEP-27 (Q-0227), ICS-35 (Q-0152) | 4 |
| Required, with no question | ICS-44 (Clang; extended floating-point types referenced) | 1 |

By the override: EDG alone 9, GCC alone 10, Clang alone 3, EDG and GCC 3, Clang and EDG 1, Clang and GCC 1, all three 1 (ATTR-20).

For the 15 rules whose class allows both results, an override is the content of the rule, and no test can avoid it. For the 13 reject and required rules, the writer probed neighbour forms of four (CE-28 in seven constant contexts, ALN-7 on an opaque enumeration declaration, DTY-5 on a nested class definition, DEP-34 with a qualified destructor name). Each split the same way as the rule's tests. The warning has no action for these rules today. ICS-44 is the one rule with no question that names its split.

### 1.7 Coverage by readers and reviewers

- **Gap reviews.** RV4 read the text phase 10, F, and P wrote. No review has read 11 rules and 4 principles in the form that the RV4 repairs and the I4 and X23 repairs gave them: ALN-4, CLS-26, COND-16, CONV-9, ICS-45, NAM-21, NEW-21, NEW-29, OBJ-11, ORD-13, STG-5, and COND-P2, DEP-P6, NEW-P5, OPR-P3.
- **Implement.** I1 to I4. I4 found three wrong or missing rules at full marks, the fewest of the four.
- **Port.** P1 gave 9 findings; two are front-end faults.
- **Propose.** X23 on P25. `exercises.md` section 4 lists X23.
- **Diagnose.** D-classes only. 61 tests have a family with no pattern.
- **Review findings have no ledger.** The eight RV4 points that the repair commit did not act on live only in its commit body.

### 1.8 Front end and the writer

`frontend_check.py` at the close: 6478 of 6711 pass, 0 worse. Eleven phase 11 tests do not pass. Three come from I4 (`class-cv-lvalues-convert-to-int`, `class-derived-prvalue-const-base-lvalue`, `class-derived-xvalue-const-base-lvalue`), two from X23 (`aligned-deleted-form-beside-unaligned`, `aligned-private-form-beside-unaligned`), and six from item P (the four pack-expansion tests, `braced-constructor-template`, `r8-tasks`). Six ledger rows wait for the round (P1-1, P1-2, I4-4 to I4-7).

The front end builds the conditional operator from the same COND text that I4-8 to I4-14 find incomplete. Front-end round 23 is the next implementer of COND-16 after I4.

### 1.9 Similarity

No flag reached a commit in phase 11. One 10-word run in an option of Q-0413 was flagged and reworded before 8d7f3bb3. Three reader reports came back as messages (P1, I4, X23). Volume is the cost: 193 reports, 27 MB.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | FE23. Front-end round 23 | the 11 phase 11 tests, P1-1, P1-2, I4-4 to I4-7; decision 0058 | 0.25 to 0.5 | low | the 6 FE ledger rows; the regression signal for the COND, NEW, and PACK tests |
| 2 | F2. Answer the 20 planned rows whose item ended, with the probes of this report | section 1.5; five of the rows deferred twice | 0.75 to 1 | low | COND text that FE23 and RV5 read; 2 or 3 new questions with tests |
| 3 | RD. The RV4 points that a probe decides | DED-29 address case, JMP-10 limits, TR-P1 two mains, SINIT-6 | 0.25 | low | text that RV5 reads |
| 4 | QR. Question records the writer can repair | Q-0323 split, Q-0142 and Q-0274 family items, 3 false warnings | 0.25 | low | a likely settlement of Q-0323; 0 family warnings |
| 5 | RV5. A gap review of the text phases 11 and 12 wrote, with Q-0417, Q-0418, and the new questions | 11 rules and 4 principles unread in final form; items 2 and 3 | 0.5 of repairs | medium | first reviews; the doubts of LAM-6, LAM-29, SW-P1, TR-4 |
| 6 | OV. A recorded reason for each rule that rests on an override | 28 warnings, the same set for two phases | 0.25 | low | a warning that names only new cases |
| 7 | LO. Triage of the 68 open ledger rows | W1 to W19, RPC, X5, X6 | 0.5 | low | a ledger with no row in state `open` |

Items 2 to 4 cost about 1.25 to 1.5 writer sessions. With item 1, the phase starts with about 1.5 to 2.

### Item 1. FE23. Front-end round 23

- **What.** Decision 0058's FE item: the 11 phase 11 tests that do not pass, the reductions of P1-1, P1-2, and I4-4 to I4-7, then the backlog groups of round 8 (access, the DEFARG, LIST, PACK, MPTR, and DCMP group, instantiation).
- **Needs first.** The COND part of item 2 (I4-8, I4-9, I4-11, I4-13). The front end implements COND-16 from the text that I4 found incomplete, and three of the 11 tests are conditional tests. If item 2 cannot come first, the writer answers the round's COND findings against the I4 report.
- **Evidence.** Section 1.8. The close moved the item here.
- **Cost.** 0.25 to 0.5 writer session to answer findings.
- **Risk.** Low.
- **Unblocks.** Six ledger rows. Front-end results for the 30 phase 11 tests.

### Item 2. F2. Answer the planned rows whose item ended

- **What.**
  - R10-4: a test of the named-local return with a GCC override (Q-0115), and a prvalue-return test with no override. The Q-0115 table lists both.
  - R10-6: one OBJ-11 sentence on a `static thread_local` data member, and a run test.
  - I3-1, I3-5: the ICS-26 example and the ICS-11 and REF-1 rows.
  - I3-13: a sentence in ICS-16 and ICS-17 on the element that a list sequence holds when elements tie, a new question for GCC's ambiguity, and four tests from the grid.
  - I4-8 to I4-14: the COND-3, COND-8, COND-11, COND-16, and OVL-P4 sentences, and tests from the reader programs p9, p3, and p11.
  - X23-3: NEW-27 states the order of the two thresholds, and UNW-3, NEW-22, and ALN-4 use NEW-28's threshold. One `-m32` test reads both values on EDG.
  - X23-6: an STG-8 sentence, adopted as provisional, and a question.
  - X23-7: one sentence in `exercises.md` section 4.
  - P1-3, P1-5, P1-8: three examples. P1-7: the base flags in `index.md`. P1-9: one TR-16 sentence.
- **Needs first.** Nothing. The probes of section 1.5 cover every row that "needs a probe".
- **Evidence.** Section 1.2 and 1.5. The close lists all 20 rows.
- **Cost.** 0.75 to 1 session.
- **Risk.** Low. Each change is a sentence, a row, an example, or a test. ICS-16 and ICS-17 are on the dense overloading page.
- **Unblocks.** Item 1 reads complete COND text. Item 5 reads the repairs. The ledger has no planned row from phase 11 left.

### Item 3. RD. The RV4 points that a probe decides

- **What.**
  - DED-29: say what happens for an address with a target type. Clang instantiates the losing template's body and rejects; GCC and EDG accept. A new question and two tests.
  - JMP-10: a `volatile` local and a local of lvalue reference type stay lvalues in a `return`. Every compiler agrees. Three tests from the grid, no override. AUTO-P3 and RET-2 cite the change.
  - TR-P1: two definitions of `main` beside the missing one. A two-unit link-fail test.
  - SINIT-6: point at OBJ-11's wider trigger, with item 2's R10-6 test.
- **Needs first.** Nothing.
- **Evidence.** The RV4 repair commit 7cf4574b lists these as "not acted on". Section 1.5 gives the probes.
- **Cost.** 0.25 session.
- **Risk.** Low.
- **Unblocks.** Item 5 reads the changed text with the doubts answered.

### Item 4. QR. Question records the writer can repair

- **What.**
  - Q-0323: a new question for the assignment before `construct_at` in constant evaluation, with the grid's forms as tests (Clang rejects, GCC and EDG accept). Q-0323 drops the family item and settles under decision 0054 if its review support holds.
  - Q-0142 and Q-0274: drop "newest families that agree" with a dated note that names the tests.
  - Q-0341 and Q-0025: a note that the family check cannot see the split (a grid; a no-diagnostic rule), or a check that reads grids and skips no-diagnostic answers.
- **Needs first.** Nothing.
- **Evidence.** Section 1.4 and the 5 warnings of `question_index.py --check`.
- **Cost.** 0.25 session.
- **Risk.** Low.
- **Unblocks.** One likely settlement. The family check reports nothing.

### Item 5. RV5. A gap review of the text phases 11 and 12 wrote

- **What.** One source-reader review of the 11 rules and 4 principles of section 1.7, the text of items 2 and 3, and the first reading of Q-0417, Q-0418, and the new questions of items 2 to 4. It rereads LAM-6, LAM-29, SW-P1, and TR-4, whose doubts no probe decides. Then one repair pass.
- **Needs first.** Items 2 and 3.
- **Evidence.** Section 1.7. RV4 settled 4 questions and gave 7 repairs.
- **Cost.** 0.5 writer session of repairs, plus one source-reader session.
- **Risk.** Medium. COND-16 and NEW-29 are long rules.
- **Unblocks.** Up to 4 first reviews. A reading of COND-16 after I4 and FE23.

### Item 6. OV. A recorded reason for each rule that rests on an override

- **What.** Each of the 28 rules gets a reason that `check_specification.py` reads: the class allows both results (15), or a named question holds the split (12). ICS-44 gains a question for Clang's result. The check then warns only on a rule with no reason.
- **Needs first.** Nothing.
- **Evidence.** Section 1.6. The warning list has not changed in two phases, and probes of neighbour forms found no agreeing form.
- **Cost.** 0.25 session.
- **Risk.** Low.
- **Unblocks.** A warning that shows a new thin rule when one appears.

### Item 7. LO. Triage of the 68 open ledger rows

- **What.** A reading of each `open` row (RPC-4 to RPC-10, W1, W3, W4, W8, W13, W14, W18, W19, X5-18, X5-19, X6-2, X6-14, X6-15) against today's text. Most will move to `text`, because later phases rewrote the pages they name. The rest get a planned item.
- **Needs first.** Nothing.
- **Evidence.** The ledger, 68 rows in state `open`, set by a script in item L.
- **Cost.** 0.5 session.
- **Risk.** Low.
- **Unblocks.** A ledger in which `open` means a new finding.

### What the rules need before the next exercises

- **A Write suite on library-shaped code (W20).** P1-3 (a two-alternative class) and P1-8 (a library-free requirement) are the examples such a suite would look for first. Item 2 gives both.
- **An Implement on deduction and the substitution boundary.** DED-29's address case (item 3) and the I3-13 list sequence (item 2) are steps such a component needs.
- **Diagnose.** 61 tests have a family with no `tidal-error` pattern. A Diagnose batch shows patterns, so it needs no rule change first.
- **Propose X24.** Items 2 and 3 change no rule that the portfolio's ready proposals (P22, P24, P26) name, except ICS-16 and ICS-17 for any overloading proposal.

## 3. Out of scope

1. **No new topic.** The status page lists no in-scope topic without a page. Phase 11 added no rule.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. ICS-44 refers to extended floating-point types and stays as it is.
3. **The maintainer's rows.** 110 of 174 open questions wait on `pending.md`. PK2's packet gives each option's effect. The writer does not choose for the maintainer.
4. **A large drop in the open count.** About 5 questions are in the writer's reach (Q-0323 and the first reviews). Items 2 and 3 open 3 or 4 new questions.
5. **A narrow second review of the doubt-held questions.** 38 questions wait only on a doubt. Item 5 rereads the doubts that RV4 raised; the older doubts wait for a later phase, as decision 0058 lists.
6. **MSVC** stays deferred. Override lines grew from 846 to 854.
7. **Changing the 28 override-only tests.** The probes found no form that removes an override. Item 6 records the reasons.
8. **Later versions** wait on row 7.

## 4. Process changes

Ranked by writer time saved.

1. **A planned row names an item that runs in the same phase, and a deferral to a second item needs a probe file.** R10-4, R10-6, I3-1, I3-5, and I3-13 went from F to RV4 and from RV4 to the close with the same reason ("needs a probe"). Each probe in section 1.5 took one grid or one host run. The rule: a row deferred for a probe carries the probe's path when it is deferred again.
2. **Review points that a repair commit leaves get ledger rows.** RV4's eight leftovers live only in a commit body. Three of them are decided by a probe now. The ledger takes review points under an `RV4-n` prefix, with the same states.
3. **The commit is chained to `checks.py`.** 5523f0e8 printed "At least one check failed" in its own message and went in. `checks.py` can exit non-zero on a failed check, and the commit runs after it with `&&`, as AGENTS.md's shell habits say.
4. **The family check reads grids and knows the no-diagnostic case.** Two of its five warnings are false hits for these reasons (Q-0341, Q-0025).
5. **A save step for reader reports.** Three more reports came back as messages in phase 11.
6. **Smaller per-commit similarity reports.** Phase 11 added 42,369 lines in 9 reports.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commit a15a877d and 1e20bfa2, and the writer probes in `writer-probes/`.

# Roadmap, round 7: what to do after phase 9, from an editor's view

Date: 2026-11-05. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and the round 6 editor report. In `documentation/` it read the charter, roadmap, status, style guide, specification format, exercises, decisions 0033, 0039, 0044, 0050, 0051, 0054, 0055 and 0056, and `decisions/pending.md`. It also read `specification/`, the headers of `tests/`, and `questions/` with its index. The phase 9 reports it read were, in `reports/clean-room/`, I2, X21, R9 (reports a to d), PU1, PU2, and rust-frontend report-20 and report-21. In `reports/review/` it read the three 2026-11-04 gap reviews of the text phase 8 wrote. It also read `git log 1b16115..HEAD` with commit bodies (17 commits). The reader did not read the other round 7 reports. Read-only scripts made the counts from the trees at 1b16115 (round 6) and 017b401, with one method for both, so a few round 6 figures differ from the ones the round 6 report gave. Statements marked "general knowledge" do not come from Tidal's files. Commit read: `017b401`.

The page table uses the round 6 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 6 (1b16115) | Now (017b401) |
|---|---|---|
| Live rules | 1681, plus 15 withdrawn | 1682, plus 15 withdrawn. ORD-17 is new. LAM-28 moved from required to unspecified, and OPR-9 from recommended to no-diagnostic |
| Rule and principle text changed in the phase | 111 rules, 66 of 343 principles | 78 rules and 35 of 343 principles, leaving out `:tests:`, `:rules:` and `:exceptions:` lines. Templates hold 25 of the 78 rules, and ordering alone holds 9 rules and 3 principles |
| Test files | 6500. 6429 cited by a rule, 71 program tests | 6619. 6534 cited by a rule, 85 program tests. 119 new, none removed |
| Rules that rest on one test | 0 | 1, CPY-28. Item R3 removed one of its two `:tests:` entries |
| Rules that rest only on tests with an override for a newest compiler | 29 | 28 |
| `check_specification.py` warnings | 30 | 29 (the 28 rules above and CPY-28). 0 problems |
| Tests with an override | 634 | 668 |
| Overrides by label | EDG 264, GCC 244, Clang 120, Clang 18 74, GCC 14 49, GCC 16 `-O2` 14, Clang 22 `-O2` 8, other 24 | EDG 274, GCC 248, Clang 129, Clang 18 76, GCC 14 55, GCC 16 `-O2` 14, Clang 22 `-O2` 8, other 24 |
| Tests with an override for exactly one newest family | 460 | 488 (Clang 104, GCC 186, EDG 198) |
| Tests with an override for all three newest families | 4 | 1, `replace-base-old-pointer-constant.cpp` (STG-19, Q-0307). Item R3 gave the other three a base line that one family keeps |
| Tests with `tidal-error` patterns | 2981 | 3041 |
| Questions | 385 | 400 |
| Open questions (all provisional) | 221 | 172 |
| Settled, compiler defect | 151, 13 | 215, 13 |
| Front end | 6183 of 6500 pass, 6247 pass their check | 6362 of 6619 pass, 6414 pass their check |
| Front end on page examples | 156 of 163 at the start of the phase (2ebb5c3) | 175 of 177 |
| Evidence lines, whole specification | 5912 of 41091 (14%) | 7505 of 43449 (17%) |
| `tests/RESULTS.md` | 6500 rows | 6619 rows |
| `question_index.py --check` | 0 problems | 0 problems |
| Pages whose only Predict batch showed test names | 12 | 1 (`foundations/objects`) |
| `pending.md` | 9 rows. Row 4 was one cell of 3435 characters | 9 rows, the same nine. Row 4 points to a section of 43 rows. Row 1 is one cell of 1399 characters (1188 in round 6) |

The pages grew by 2358 lines, and Evidence grew by 1593 of them. Item V3 states the cause. The Split tests tables that V3 printed on 64 pages are longer than the settlement tables they replace, and the Evidence of the 68 pages went from 45776 to 50575 words.

### What phase 9 did to the questions

Each count comes from each file's status at 1b16115 and at 017b401.

- 15 new questions (Q-0386 to Q-0400). Each has a `**Source:**` line. 12 are open. Q-0387, Q-0388, and Q-0391 settled.
- 61 questions that were open at 1b16115 settled. No question reopened.
- Open questions went from 221 to 172. RV2 settled 36, PU1 20, PU2 7, and R3 one (Q-0342).
- 108 of the 172 open questions have no change in any phase 9 commit.

A script sorted the 172 open questions by the last "Kept open" line in each file and by the questions that `pending.md` names. A question goes in the first group that fits.

| What holds the question | Questions |
|---|---|
| `pending.md` row 1: two newest families differ (the row's list of 15) | 15 |
| Another row or section of `pending.md` | 52 |
| A rule or principle changed, and the question waits for a use (decision 0055) | 21 |
| Two newest families differ, and row 1's list does not name the question | 16 |
| A review reads with doubt, finds the reference silent or unclear, or reads against the answer | 63 |
| No "Kept open" line (Q-0396 to Q-0400) | 5 |

So 67 open questions (39%) are named in `pending.md` and wait on the maintainer. 47 of the 63 questions in the review group had no change in phase 9.

Only 3 of the 21 questions that wait for a use match the phrases that `tools/exercises.py principles` selects on ("waits for the next use" and three others). The other 18 say the same thing in other words, for example "a question settles only after a reader or a review has used the changed rule" (Q-0129, Q-0247, Q-0263, Q-0358) or "no reader or review has used the changed rule" (Q-0370, Q-0376). Nine of them (Q-0092, Q-0129, Q-0247, Q-0263, Q-0269, Q-0299, Q-0358, Q-0370, Q-0376) are in neither PU batch and in no phase 8 review.

### Text phase 9 wrote that no review has read

The three 2026-11-04 gap reviews read the text at 1b16115. Every change after that is unread by a source reader:

- ORD-17, and the class changes of LAM-28 and OPR-9.
- 78 rules and 35 principles whose prose changed. The repairs from I2 and R9 changed ORD-1, ORD-3, ORD-4, ORD-6, ORD-9, ORD-11 to ORD-14, ORD-P1, ORD-P2, and ORD-P4. The repairs from X21 and R9 changed SINIT-3 to SINIT-7, SINIT-P2, STO-10, STO-P1, TR-4, TR-13, and TR-P1.
- Q-0396 to Q-0400, from I2 and R9. No file of `reports/review/` names them, and none has a "Kept open" line.
- Q-0389, Q-0390, Q-0392, Q-0393, Q-0394, and Q-0395, which RV2 opened from the reviews. A review read each case, and no review has read the provisional answer as written.
- Q-0337, which appears in no review file for the third round.
- The Divergences lines that V3 wrote on 68 pages. Each line restates its question's answer and the family that differs. A tool checks that each named question has a line, and no tool checks the line against the question file.

Front-end findings G146 to G151 (report-21) on ORD-9, ORD-6, COND-16, INST-12, CMP-8, BF-1, and ALN-8 have no answer in any specification, question, test, or documentation file.

### What phase 9 took from round 6

| Round 6 item | Outcome |
|---|---|
| 1. Review the text phase 8 wrote, and use the widened principles | Done as RV2, PU1, and PU2. 63 questions settled. 21 open questions still wait for a use, most of them on text that phase 9 changed |
| 2. Repair the records, and generate the claims that drift | Done as R3, except the I1 header. Section 7's sentence is generated, row 4 is a section, 41 questions got a dated reason, 48 old notes are marked replaced |
| 3. Move the 68 pages to the Evidence format | Done as V3. The 10 pages of item V kept the phase 8 version of the format (below) |
| 4. Predict on the 12 pages with only named batches | 11 done in R9, 1055 of 1055. `foundations/objects` waits. The tool fix it needed came in 32c89f7 |
| 5. Split the overloading principles | Not done. Decision 0056 defers it. R/P is still 11.0 and 9.4, now for a fourth round |
| Process: a settling exercise for each phase's principles | Done for phase 8's principles (PU). Decision 0056 names no review for the text phase 9 changes |
| Process: a required reason line for open questions | In part. R3 wrote the lines. No check fails on a missing line, and 5 new questions lack one |
| Process: coverage claims from a tool | Done for section 7's sentence. Section 8's rows are still written by hand |
| Process: a format warning becomes a failure | Done. The older-form warning is gone |
| Process: `pending.md` rows stay short | Row 4 done. Row 1 grew by 211 characters |
| Process: the open-question target counts what holds each question | In part. Decision 0056 asks for the change by group (held for a principle use, in `pending.md`, the rest). The closing commit gives the change by item |

### Records that drift

These records disagree with the tree at 017b401, or with each other:

- **Split tests.** Six item V pages have no Split tests part, though their cited tests carry overrides: constant evaluation (24 tests), translation (24), constraints (28), dependent names (23), ordering (19), and ODR (18). Assignment has 1 such test and no part. Four other pages each lack one row (instantiation, unwinding, inline, virtual). No tool writes or checks the tables.
- **Two definitions of Split tests.** The parts table of `specification-format.md` says the table holds "each test of the page on which the newest compilers differ". The Split tests section of the same file says one row "for each test of the page that carries an override for a baseline compiler". The V3 tables follow the second.
- **Probe grids.** The format puts grid tables in `tests/<page>/grids/tables.md` and a link on the page. Eight grid directories have no `tables.md` (copy-move, members, arguments, specialization, deduction, instantiation, packs, class template deduction). Instantiation, class template deduction, deduction, specialization, and arguments keep 114 lines of grid tables on the page. The V3 commit says `tests/` was out of reach for that step.
- **Q-0290.** Its last "Kept open" line says GCC 16 and EDG 7.0 differ from the answer. The answer is GCC and EDG's option, and the test overrides only Clang. The V3 commit names this and leaves it.
- **`pending.md` row 1.** The row lists 15 questions that settle if two newest families may differ. 16 more open questions give two differing families as their last reason, and the list does not name them. Q-0045, Q-0062, Q-0080, and Q-0368 among them have a review that reads the answer. The row's cell also holds a list of commits that grows with each settling item.
- **`exercises.md` section 6** still says "Decision 0055, item E, orders the next exercises" and lists phase 8's exercises. It names Propose rounds X1 to X16; X17 to X21 are done.
- **`exercises.md` section 8** has a PU1 row and no PU2 row.
- **Decision 0056** says "All items are done". Its item E lists Predict on 12 pages, and 11 ran. The roadmap's phase 9 row says so; the decision does not.
- **Report headers.** `reports/clean-room/I1/report.md` and `reports/clean-room/I2/report.md` each say the message is "the only copy" of the report. Both reports are committed.
- **Stacked reasons.** 13 open questions carry three or more "Kept open" lines. Q-0018 and Q-0378 carry six, all dated 2026-11-04. A reader has to find the last one to learn the current reason.
- **CPY-28** rests on one test since R3 removed an entry. The check warns, and decision 0055's rule fails a commit only when it adds or reclassifies a thin rule.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | Other evidence |
|---|---|---|---|---|---|---|---|---|
| 1 | overloading/conversion-sequences | 47 / 133 | 13 | 152 | 27 | 177 / 1025 | 9.4 | No question settled and no rule changed in phase 9. ICS-35 and ICS-44 rest only on overridden tests. Three split tests name no question |
| 2 | foundations/storage | 29 / 93 | 12 | 261 | 6 | 163 / 1019 | 3.6 | No question settled and no rule changed in phase 9. Q-0307 is the one test against all three families |
| 3 | overloading/overload-resolution | 44 / 181 | 9 | 287 | 29 | 199 / 1049 | 11.0 | The most overridden tests of any page, and the highest R/P for a fourth round. Q-0140 and Q-0142 wait in `pending.md` |
| 4 | templates/ordering | 17 / 62 | 7 | 179 | 19 | 161 / 772 | 4.2 | New on the list. 13 of its rules and principles changed in phase 9, and no review has read them. Q-0396 to Q-0398 have no reason line. No Split tests part. G146 and G147 have no answer |
| 5 | foundations/constant-evaluation | 27 / 155 | 7 | 423 | 24 | 150 / 793 | 5.4 | The most report mentions. No Split tests part. CE-13 and CE-28 rest only on overridden tests |
| 6 | templates/instantiation | 35 / 140 | 6 | 118 | 24 | 272 / 1013 | 7.0 | The largest Evidence share (27%), with 33 lines of grid tables on the page. Q-0235 waits on row 5. Q-0392 is new. G149 has no answer |
| 7 | templates/constraints | 40 / 132 | 4 | 93 | 28 | 189 / 993 | 8.0 | 28 overridden tests and no Split tests part. CNS-18 and CNS-38 rest only on overridden tests |
| 8 | templates/dependent-names | 39 / 127 | 6 | 161 | 23 | 185 / 1004 | 6.5 | The same 6 open questions as in round 6. No Split tests part. DEP-27 and DEP-34 rest only on overridden tests |
| 9 | templates/class-template-deduction | 29 / 117 | 6 | 66 | 21 | 214 / 900 | 9.7 | The lowest F on the list. 5 of its 6 open questions wait in `pending.md`. Grid tables on the page |
| 10 | classes/members | 34 / 163 | 7 | 161 | 16 | 148 / 820 | 8.5 | Q-0381 rests on a review that finds the reference silent. Two grids and no `tables.md` |

Three pages left the round 6 list. New and delete and unions each went from 10 open questions to 6, and attributes from 6 to 4. Alignment went from 5 to 0.

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | **Review the text phase 9 wrote** (RV3), then the repairs | 78 rules, 35 principles, ORD-17, 2 class changes, 12 new open questions, and Q-0337. No source reader has read any of it | one source-reader session in two or three parts, one writer session | low | Reason lines for Q-0396 to Q-0400. Review readings for the six RV2 questions. Pages 4, 6, and 9 |
| 2 | **Repair the records, and generate the Split tests tables** (R4) | Section 1, "Records that drift" | one session, plus a tool | low | One Evidence layout on all 85 pages. Counts that a check keeps true |
| 3 | **Hold each waiting question by a field, then a third principle-use batch** (PU3) | 21 questions wait for a use, and the tool selects 3 of them | one tool session, one reader batch, one writer pass | low | Up to 21 settlements |
| 4 | **A decision packet for `pending.md`** | 67 open questions (39%) wait on the maintainer. No row was decided in phase 9 | under one session | low | Up to 67 settlements, and 16 more if row 1 widens |
| 5 | **Triage the questions that a review reads with doubt or silence** | 63 questions. 47 had no change in phase 9 | one writer session | medium | A path for each of them, by a second review, a principle, or a row |
| 6 | **Close the small open ends**: G146 to G151, Predict on objects, CPY-28 | Six findings with no answer, one page with only a named batch, one thin rule | under one session each | low | Pages 4, 6, and 10. Section 7's count reaches 0 |
| 7 | **Split the overloading principles** | R/P 11.0 and 9.4 for a fourth round. No question on the two pages settled in phase 9 | one session | medium | A reader can find a tie-breaker. Runs after item 1 |

### 2.1 Review the text phase 9 wrote

Decision 0056 made a review of phase 8's text an item, and RV2 settled 36 questions from it. Phase 9 wrote less text than phase 8, and more of it sits on one procedure. The ordering page changed in 13 rules and principles, and three of its new questions (Q-0396 to Q-0398) record a split with no review reading.

The review can follow RV2's form:

- Part A reads ORD-17 and the 78 changed rules against the reference, grouped by chapter. Templates (25 rules) and declarations (20) are the largest groups.
- Part B reads each of the 35 changed principles against the questions it holds. A principle used this way meets decision 0055's condition, so this part also serves item 3.
- Part C reads Q-0337, Q-0389, Q-0390, Q-0392 to Q-0400, and the answers of Q-0293 and Q-0386, which PU2 did not support.

The writer then gives each question a reason line or settles it, and acts on the Part A gaps. Before the review starts, the writer runs `similarity.py` on the review as AGENTS.md rule 6 requires.

The risk is a review that reads ORD-17 or ORD-P2 against the matrix. Then the question goes to the "Matrix against review" section of `pending.md` with the others.

### 2.2 Repair the records, and generate the Split tests tables

The largest part is a tool. `status_table.py` (or a new script) prints each page's Split tests table from the test headers and the matrix results, writes it into the page, and makes `check_specification.py` fail when a table is stale. V3's writer already had such a script in scratch space. With the tool in the repository:

1. The six item V pages and assignment get their tables (137 tests), and the four missing rows appear.
2. `specification-format.md` keeps one definition of the table. The V3 tables use "an override for a baseline compiler". The editor proposes keeping that one and changing the parts table to match.
3. The grid tables of five pages move to `tests/<page>/grids/tables.md` with `tools/probe_grid.py`, and the Probe grids parts link them. The eight grid directories get their `tables.md`.

The other repairs are small, one record each:

- Q-0290's last line names Clang as the family that differs.
- `tools/question_index.py` writes the row 1 list of questions that settle if two families may differ, from the questions' last reason lines, and fails `--check` when the list is stale. The list of commits in row 1 moves to a short table below the rows.
- `exercises.md` section 6 names decision 0056's exercises and the Propose rounds to X21. Section 8 gains a PU2 row. Section 8 rows could come from a list of report directories, so a missing row fails a check.
- The I1 and I2 headers lose the "only copy" sentence.
- Decision 0056's status line says that the objects page's Predict batch waits.

### 2.3 Hold each waiting question by a field, then PU3

`tools/exercises.py principles` selects a question by four phrases in its last dated note. 18 of the 21 questions that wait for a use say it another way, so the tool skips them, and nine have waited since phase 7 or phase 8. A header field fixes the selection:

```text
**Waits for:** a use of ORD-P2 (decision 0055), since 2026-11-04
```

`question_index.py --check` fails when a provisional question has no `**Waits for:**` or `Kept open` reason, which also covers Q-0396 to Q-0400. The tool selects by the field. The writer fills the field for the 21 questions, and PU3 runs on them after item 1, so that RV3's Part B and PU3 together count as the use.

### 2.4 A decision packet for `pending.md`

The nine rows have not changed in phase 9, and the open questions they hold went from 54 to 67 of the open count (from 24% to 39%). The share grows because the other groups settle. The editor cannot decide a row. The editor can put each row in a form the maintainer can answer in one reading:

| Row | What it decides | Open questions it closes |
|---|---|---|
| 1 | Settling by review while two newest families differ | 15 listed, up to 31 with the 16 unlisted |
| 2 | Settling when every newest family agrees | Q-0322 |
| 3 | A report of EDG behavior to the EDG project | none directly. 12 open questions appear in its section |
| 4 | Follow a review against a compiler that differs | 29 open questions and 14 rules in 43 rows |
| 5 | The variable form of Q-0235 | Q-0235 |
| 6 | Q-0343 and Q-0041 | 2 |
| 7 | The later-version probe | Q-0007, and Q-0087, Q-0181, Q-0188 |
| 8 | An outside reader | none |
| 9 | Process questions | 8 |

The packet is a section of `pending.md` (or a page beside it) that gives, for each row, the option Tidal keeps, the questions each option settles, and the tests whose expectation would change. Row 1 and row 4 together hold 44 to 60 of the 67 questions. Asking the maintainer to answer is an action inside the repository's process, so AGENTS.md does not require a wait before the packet is written.

### 2.5 Triage the questions that a review reads with doubt or silence

63 open questions sit on a review reading that does not support the answer. 47 of them had no change in phase 9, and no phase item names them. Decision 0054 settles a question by review only when the review supports the answer with no doubt, so another round of the same review would give the same reading. The question files give three kinds:

- **The review finds the reference silent or the question outside it** (for example Q-0020, Q-0079, Q-0179, Q-0180, Q-0184, Q-0185, Q-0321, Q-0325, Q-0349, Q-0361, Q-0381, Q-0384). The answer is then Tidal's own choice. Evidence could say so, as it already does for VIRT-13 and ATTR-6, and a new rule could settle such a question. That rule is the maintainer's to adopt, so it would go to `pending.md` as row 10.
- **The review reads with doubt or calls the question unclear** (for example Q-0001, Q-0032, Q-0033, Q-0124, Q-0154, Q-0200). A second review with a narrower prompt, one question per section, can resolve the doubt or state it.
- **The review reads against the answer** (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344). These belong in the "Matrix against review" section of `pending.md` when the matrix keeps the answer.

The writer sorts the 63 into these kinds in one pass and writes the kind into each question's reason field (item 2.3).

### 2.6 Close the small open ends

- G146 to G151 name gaps in ORD-9, ORD-6, COND-16, INST-12, CMP-8, BF-1, and ALN-8. The writer probes each one and answers in the rules or in a question. RV3 can read the same rules, so these fit beside item 1.
- `tools/exercises.py predict foundations/objects` now builds its 61 programs. One Predict batch without test names takes section 7's count to 0.
- CPY-28 gets a second test, or its Evidence says why none can exist. The page's Evidence says that no core-language check gives a yes or no for the property, so the second path may be the only one.

### 2.7 Split the overloading principles

Conversion sequences and overload resolution hold 22 open questions, and none settled in phase 9. Their principles cover 9.4 and 11.0 rules each. Round 6 proposed principles of about six rules or fewer. Under decision 0055 a rewritten principle restarts the wait of its questions, so this item runs after RV3 has read the two pages' principles, and a PU batch follows it.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phase 9 added one rule and no page |
| New Propose proposals | Defer | X21 used the last ready proposal (P15). Decision 0056 defers new ones |
| The 28 rules that rest only on overridden tests | Leave as warnings | Item TE probed each one in phase 8. No new form appeared |
| Settling questions where two newest families differ | Leave to the maintainer (row 1) | Item 2.4 prepares the decision |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md` |
| Port suites, guides on the site, Diagnose beyond D-classes, pair exercises, a shared impact scanner | Defer | Items 1 to 5 reach more open questions per session. Decision 0056 lists them as later work |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0056 say | 33 *(referenced)* markers name them, the same as in round 6 |

## 4. Process changes

- **Each phase names the review of its own text.** A phase decision names the gap review that reads the rules and principles its items change, as decision 0056 did for phase 8's text. Phase 9 changed 113 rules and principles and named no such review.
- **One reason field per open question, checked.** A `**Waits for:**` (or `**Open because:**`) header line replaces the stack of "Kept open" lines as the current reason, and the dated lines stay below as history. `question_index.py --check` fails on a provisional question without the field. Round 5 and round 6 asked for this, and 5 new questions lack a reason.
- **Tools select by the field.** `exercises.py principles` reads the field, so a change of wording in a note cannot drop a question from a batch.
- **Generated tables in pages.** Split tests tables, row 1's list of questions, and section 8's rows come from tools, and the check fails on a stale one. A table printed once from a scratch script drifts at the next override change.
- **The format document has one definition per part.** A change to `specification-format.md` updates both the parts table and the part's section in the same commit.
- **The closing commit gives the open-question change by group.** Decision 0056 asked for this. A script like the one in section 1 can print the groups at the start and end of a phase.
- **A `pending.md` packet each phase.** The closing commit of a phase counts the open questions that `pending.md` holds. When the share rises, the next phase's first item is a decision packet for the maintainer.

## 5. Files

The reader made the counts with read-only scripts in `/tmp/claude-1000/-home-ryanm-Code-tidal/8431ee71-ca68-4ee4-8d89-674ba4995171/scratchpad/rm7/`, on copies of the trees at 1b16115 and 017b401 made with `git archive`: `count.py` for rules, tests, overrides, questions, and Evidence; `diff.py` for the rule, principle, and test changes; `q.py` for question movement and review coverage; `hold2.py` for what holds each open question (output in `hold2.json`); `pages.py` for the page table; `split.py` for the family splits; and `splitcheck.py` for the Split tests tables against the test headers. The reader also ran `tools/check_specification.py` and `tools/question_index.py --check`, which change no file.

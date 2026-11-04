# Roadmap, round 5: what to do after phase 7, from a language designer's view

Date: 2026-11-03. Decision 0029, with decisions 0037, 0039 and 0054. Role: clean-room reader, language-designer persona. Inputs: AGENTS.md and CLAUDE.md; `reports/clean-room/roadmap-4/designer.md` (the model for this form); `documentation/` (charter, roadmap, status, exercises, specification format, decisions 0039 and 0054, and `decisions/pending.md`); `reports/clean-room/X13/` to `X16/` (headers, impact sections and findings in full); the head of `rust-frontend/report-17.md` and `conformance.tsv`; `questions/README.md` and the status lines of Q-0007, Q-0029, Q-0183, Q-0185, Q-0188, Q-0322 and Q-0369 to Q-0371; `tools/propose_check.py` and its output; rule text under `specification/` through targeted searches; and `git log --oneline 2798b37~1..HEAD` with `git show --stat` for commits d375f6c, 435a783, 966aa1b and 2daa800. Counts come from read-only Python scripts in my scratch directory, run on the tree at commit 7d8c982. I did not read the other round-5 reports. I did not run `tools/similarity.py`. Statements from outside Tidal's files are marked "general knowledge".

## 1. The state from a designer's view

### 1.1 Counts at 7d8c982

| Measure | Round 4 | Now |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1678 | 1691 |
| `required` | 892 | 895 |
| `reject` | 553 | 557 |
| `undefined` | 56 | 57 |
| `impl-defined` | 51 | 52 |
| `recommended` | 44 | 44 |
| `no-diagnostic` | 33 | 35 |
| `unspecified` | 30 | 31 |
| `conditionally-supported` | 4 | 5 |
| `withdrawn` | 15 | 15 |
| Test files that expect `reject` | 2564 | 2633 |
| Test files that expect `link-fail` | 39 | 44 |
| Questions open for opinions | 310 | 209 (all provisional) |
| Settled questions | 30 | 154 |
| Front end passes | 5595 of 6032 | 6068 of 6337 |

My script counts 6334 test files with a `tidal-expect` line: 2633 `reject`, 2107 `accept`, 1455 `run`, 95 `run-fail` and 44 `link-fail`. The status page gives 6337 tests and says the matrix passes all of them on each baseline compiler. I did not trace the difference of three. The question index has 376 questions: 209 provisional, 13 compiler defects and 154 settled.

### 1.2 What phase 7 did for the designer

Phase 7 took two of round 4's eight items, and one more in part.

| Round-4 item | Phase 7 | State |
|---|---|---|
| 1. A refresh check for Propose reports | item X, commit d375f6c | done, with a gap (section 1.5) |
| 2. Propose round 4, X13 to X16 | item X, commits 435a783 and 966aa1b | done. 39 findings |
| 3. Pair exercises | none | not started |
| 4. Probe of later-version space | none | waits on the maintainer, row 7 of `pending.md` |
| 5. An index of what a test of validity can see | none | not started |
| 6. A corpus manifest and a construct count | item WP, commit 2daa800 | half done. `tests/programs/` holds the 67 Write programs as run tests with `tidal-rules` lines. No tool counts constructs over them |
| 7. A rule class for link-time diagnostics (D30) | none | open |
| 8. A front-end impact probe | none | not started |

Of round 4's six process changes, four were taken:
- Propose reports name the commit they read. `documentation/exercises.md` section 4 says so, and X13 to X16 each name 573d849.
- A reader runs the refresh check before reading an older report. X14 and X16 record that they did.
- A change that withdraws a rule lists the Propose reports that cite it in its commit message (`specification-format.md`, per commit d375f6c).
- This report keeps the view.

Two were not taken. The status rows in `exercises.md` section 8 still give finding counts only, with no count of what was closed. The Propose prompt does not say that probes flush before they abort, although X14 and X16 each say their probes did.

Phase 7 also changed the designer's inputs in ways no designer item asked for. Item SR and item GR settled 110 questions, and `pending.md` collects every item that waits on the maintainer in one file. Three of its rows matter to proposals: row 2 (Q-0322, which holds P8b), row 7 (the later-version probe, Q-0007), and row 9 (Q-0183, the cost model).

### 1.3 What X13 to X16 found, and what the repairs closed

| Exercise | Proposal | Findings | Closed in the specification | To a question | No action |
|---|---|---|---|---|---|
| X13 | P6, named arguments | 13 | 11 (X13-1 to X13-11) | X13-5, Q-0370 | X13-12, X13-13 |
| X14 | P20, packed classes | 10 | 8 (X14-1 to X14-8) | X14-4 joins Q-0069 | X14-9, X14-10 |
| X15 | P21, names private to a namespace | 9 | 7 (X15-1 to X15-7) | X15-1, Q-0369; X15-2, Q-0371 | X15-8, X15-9 |
| X16 | P4, chained comparisons | 7 | 5 (X16-1 to X16-5) | Q-0029 gains rows | X16-6, X16-7 |

Commit 966aa1b closed 31 of 39 findings and added 39 tests. A finding sent to a question also got its tests and rule text, so I count it as closed. The 16 new tests that the front end got wrong at 966aa1b all pass in `conformance.tsv` at 7d8c982.

Each of the eight findings with no action is either a list of outside knowledge or a list of needs outside the specification. The second kind repeats the standing needs:

| Need | Raised by |
|---|---|
| Version marking | X13-13, X14-10, X15-9, X16-7 |
| Implementation cost | X14-10, X15-9 |
| A corpus of real programs | X16-7 |
| A library chapter (parameter names) | X13-13 |
| The preprocessor (`#if` expressions) | X16-7 |

The rate fell. X9 to X12 gave 49 findings, 45 closed. X13 to X16 gave 39, 31 closed. The fall fits the targets. X13 to X15 went to five small declarations pages, and X16 to a page that phase 6 had just repaired for P4. The first designer reading of a page still finds about eight gaps that a writer can close.

Four findings were wrong or unclear rules that a Predict batch had not caught: X13-1 (DEFARG-P2 against OPR-2), X13-9 (the DEFARG Evidence counted the wrong tests), X15-3 (NS-P2 against NS-13), and X15-4 (NS-1 and argument-dependent lookup). R-default-arguments and R-namespaces ran in earlier phases and scored 220/220 and 132/132. A Predict batch scores outcomes, so a principle that misstates a rule while every test still passes stays invisible to it. A Propose exercise reads the principles as a designer must.

### 1.4 Pages the Propose exercises have reached

I mapped every rule ID cited in X1 to X16 to its page. 84 of 85 pages are reached. Commit 435a783 says every page has now had a Propose exercise. My method counts cited IDs only, and `expressions/bitwise.md` is reached by name in X1 with no BIT rule cited. So the two counts differ by that one page.

Twelve pages are reached by one exercise only:

| Page | Exercise |
|---|---|
| `expressions/additive.md`, `multiplicative.md`, `shift.md`, `comma.md` | X1 |
| `expressions/conditional.md` | X4 |
| `statements/loops.md` | X11 |
| `declarations/enumerations.md`, `type-names.md` | X12 |
| `declarations/alignment.md`, `foundations/types.md` | X14 |
| `declarations/inline.md`, `namespaces.md` | X15 |

X1 is the oldest Propose report (commit a606e10). Since then, 12 commits have changed `additive.md`, 11 `multiplicative.md`, 5 `shift.md` and 4 `comma.md`. Section 1.3 shows that a first reading finds rules that a Predict batch passes. A second reading of the arithmetic pages is due.

### 1.5 The refresh check has two gaps

`tools/propose_check.py` at 7d8c982 reports 16 reports and 30 conflicts: 10 withdrawn, 3 class, 17 new-id, 0 unknown. Round 4 counted 10, 1 and 16. The new rows:
- CTOR-17 in X4 (`no-diagnostic` then, `reject` now),
- STG-25 in X9 (`impl-defined` then, `unspecified` now, from item TP),
- EXS-P5 in X2, a principle the tool now reads.

The tool has two gaps.

**It reads X14 as 0 rows.** X14's change table has the header `Rule ID | Kind of change | Old class`. The tool looks for columns named `id`, `change` and `old class`, and skips a table without them. X13 uses `ID | Change`, so its 59 rows are read. X14's 57 rows are never checked, and the tool prints "0 rows, 0 conflicts" with no warning. A reader who trusts the output sees a clean report.

**It does not see a changed rule.** It flags a withdrawn rule and a changed class. Commit 966aa1b rewrote rules that X13 to X16 cite and keep in their tables: DEFARG-2, DEFARG-5, DEFARG-9, ALN-1, ALN-4, NS-1, NS-15, NS-P2, LSP-4, CMP-14 and PRIM-27, among others. The tool reports "0 conflicts, read 573d849, 5 later commits change specification/" for each of the four. The commit count is the only sign. A row that says "unchanged but must state its effect" can now be wrong in its effect, and nothing points to it.

### 1.6 What a designer still cannot state

**Whether a later version has taken the space.** All four of X13 to X16 raise this again. Of the eight Propose reports since X9, seven raise it as a finding or answer it with their own probe. X11 does not mention versions. Q-0007 is still provisional. `pending.md` row 7 holds the probe for the maintainer. Row 4 of the same file names a question (Q-0087) where Clang and GCC follow a later change and EDG follows C++23. My search finds 30 lines in the 2026-11-03 gap reviews that mention a later version or a newer change.

**What a change costs.** Q-0183 keeps option 1 (cost in words), and `pending.md` row 9 lists it. X14-10 and X15-9 each give a cost they cannot state.

**How much code a change touches.** `tests/programs/` now holds 67 whole programs, 7610 lines, each citing the rules its report names. X16 scanned all 6335 test files, which include these programs, for chained comparisons and found only the four tests written to test chains. X16-7 still says Tidal has no corpus. The programs are reader-written and few, and no tool counts constructs over them. So the corpus exists in form and gives no numbers yet.

**Which programs change meaning without a diagnostic.** X16 4.3 found that a chain inside a dependent requirement changes its answer under P4 with no diagnostic on either compiler, and the repairs added `chained-in-concept-no-warning.cpp`. This is the same channel X10 and X12 found. My scan finds 28 of the 557 `reject` rules cited by an `accept` or `run` test whose text contains `requires`. Round 4 gave 3 by a narrower method, so the two numbers do not compare. Neither count says which rejections a test of validity can see. No index of that exists.

**Link-time diagnostics (D30).** Open since round 1. The TR-P1 class table still has no class for a program that a correct implementation rejects only at link time. The 44 `link-fail` tests cite 33 `no-diagnostic` rules and 28 `required` rules, and no `reject` rule. So a test outcome of "fails to link" maps to two classes, and neither says that the link must fail. P15 waits on this.

### 1.7 The portfolio, updated

| # | Proposal | Round 4 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P1 | Width-named literal suffixes | ready | ready | | |
| P2 | Narrowing is an error everywhere | ready | ready | | |
| P3 | Left-to-right evaluation | done | done | | |
| P4 | Chained comparisons | ready | done (X16) | | |
| P5 | Automatic objects start at zero | done | done | | |
| P6 | Named arguments | ready | done (X13) | | |
| P7 | Temporaries through reference chains | done | done | | |
| P8 | Relocation by a relocation function | done | done | | |
| P8b | Relocation by bytes | partial | partial | | Q-0322, `pending.md` row 2 |
| P9 | Overload sets as values | done | done | | Q-0142 is in `pending.md` row 4 |
| P10 | Expansion over elements | done | done | | |
| P11 | Exceptions inside constant evaluation | done | done | | |
| P12 | Value-specialized parameters | done | done | | |
| P13 | Checked `noexcept` | done | done | | |
| P14 | Scope-exit blocks | done | done | | |
| P15 | Declared initialization order | partial | partial | | D30 |
| P16 | Member call syntax | done | done | | |
| P17 | Opaque type aliases | ready | ready | | Class-template and argument equivalence still need a check |
| P18 | Required `override` | ready | ready | | |
| P19 | Exhaustive `switch` over a marked enumeration | done | done | | |
| P20 | Packed classes | ready | done (X14) | ALN-1, ALN-4, STG-23 and ATTR-20 repaired (X14-1 to X14-7) | |
| P21 | Names private to a namespace | ready | done (X15) | NS-1, NS-15, NS-P2 and LSP-4 repaired | |

Totals now: 16 done, 4 ready (P1, P2, P17, P18), 2 partial (P8b, P15), 0 blocked. P8b and P15 wait on the maintainer and on D30, so the four ready proposals are the last ones a reader can take without a decision.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Close the two gaps in the refresh check | 1.5: X14's 57 rows go unchecked; rules rewritten by 966aa1b are not flagged | small tool | low | trustworthy inputs for items 2 and 5 |
| 2 | The later-version probe, put to the maintainer as a costed plan | 1.6: seven of X9 to X16 raise version marking; `pending.md` row 7 | one plan now; a runner option and one matrix run if approved | medium | section 1.1 of every proposal; Q-0007, Q-0087, Q-0181, Q-0188 |
| 3 | A rule class for link-time diagnostics | D30, open since round 1; 44 `link-fail` tests over two classes | 1 writer session | low | P15 |
| 4 | An index of what a test of validity can see | 1.6: X10, X12 and X16 each found the silent channel by hand | 2 writer sessions | medium | section 4.3 of every proposal |
| 5 | Propose round 5: X17 to X20 | 1.3: about 8 closable findings per exercise; 1.4: 12 pages read once; 1.7: 4 ready proposals | 4 reader sessions, then 1 repair session | low | the last ready proposals; a second reading of the arithmetic pages |
| 6 | A construct count over `tests/programs/` | 1.6: 67 programs exist, no counts | small tool | low | Q-0183 option 2 in part |
| 7 | Pair exercises | round 4 item 3, not taken | 2 reader sessions | low | a way to state how two changes compose |
| 8 | A front-end impact probe that changes one rule | front end passes 6068 of 6337; X16 scanned 6335 files by hand | large | medium | measured Tests columns |

### Item 1. Close the two gaps in the refresh check

**What it is.** Two changes to `tools/propose_check.py`.
- The tool reads the column names that the reports use (`ID` or `Rule ID`, `Change` or `Kind of change`), and prints a warning when a report has a section 2 and yields no rows.
- For a report that names its commit, the tool lists each row whose rule's directive text changed between that commit and HEAD. A `git diff` of the rule's directive block is enough. The line says "changed since 573d849", with no judgment about whether the row still holds.

**Evidence.** Section 1.5. X14 is read as 0 rows today. Commit 966aa1b rewrote at least eleven rules that X13 to X16 keep in their tables, and the tool reports 0 conflicts for all four.

**Cost.** Small. The rule text and the commit ID are both already at hand.

**Risk.** Low. The check stays a warning, because a report is a record of its date. The "changed" list will be long for old reports, so it should print only for reports that name a commit.

**What it unblocks.** Items 5 and 7 read old reports and need to know which rows still hold.

### Item 2. The later-version probe, put to the maintainer as a costed plan

**What it is.** The probe is unchanged from round 4 item 4. Compile every test that expects `reject` (2633 now) under the newest `-std` mode of each baseline compiler, and publish the tests whose outcome flips as an index with no rules and no `:since:` field. The item for phase 8 is to put row 7 of `pending.md` to the maintainer with that plan, its cost, and the list of questions that name a later change, so the decision can be made in one step.

**Evidence.** Section 1.6. Version marking is the need that the Propose reports repeat most often, in seven of the last eight. X10 showed that a hand probe under the next version's mode finds real overlap.

**Cost.** One short plan now. If approved, a runner option and one matrix run, then triage.

**Risk.** Medium. Triage is the effort, and a reader who triages may recall later-version material. Q-0116 keeps the source-reader designer as the channel for any explanation. The index must name compiler behavior only.

**What it unblocks.** The version line of every proposal. Data for Q-0181 and Q-0188, and for Q-0087 in `pending.md` row 4.

### Item 3. A rule class for link-time diagnostics

**What it is.** Unchanged from round 4 item 7. A row in the TR-P1 class table, or a stated policy, for a program that a correct implementation must reject only at link time. Then a pass over the 61 rule citations of the 44 `link-fail` tests, so each one cites a rule of that class or says why the link failure is only one possible result.

**Evidence.** D30, open since round 1. Section 1.6: the `link-fail` tests cite 33 `no-diagnostic` and 28 `required` rules, and no class says that the link must fail.

**Cost.** One writer session.

**Risk.** Low. The tests exist, and the matrix already passes them.

**What it unblocks.** P15, the last proposal held by a specification gap. A designer can then say whether a proposal's new error belongs at compile time or link time.

### Item 4. An index of what a test of validity can see

**What it is.** Round 4 item 5, unchanged in form. For each `reject` rule, one value:
- **visible:** the error can occur where substitution (DED-24) or a requires expression (CPT-26) sees it,
- **fatal:** the error is always fatal (DED-29, or outside any template),
- **syntax:** only the grammar rejects the form (TR-12).

A rule that is visible in one position and fatal in another gets both values with a note. Add a note for the constant-initialization channel (SINIT-1, SINIT-4) where the rule concerns constant evaluation. Add a test for a sample of the "visible" rules that observes the rejection from inside a requires expression.

**Evidence.** Section 1.6. X10, X12 and X16 each worked out visibility by hand. X16 found that a dependent requirement changes its answer with no diagnostic on either compiler, so no warning flag can find the affected code.

**Cost.** Two writer sessions.

**Risk.** Medium. The value for some rules depends on position, and some rules will need a probe.

**What it unblocks.** Section 4.3 of every proposal. A check on item 8's results.

### Item 5. Propose round 5: X17 to X20

**What it is.** Four Propose exercises on the four ready proposals. Each is aimed at pages that one exercise has read, or none.

| Exercise | Proposal | Pages it aims at | Changes existing behavior |
|---|---|---|---|
| X17 | P2, narrowing is an error everywhere | additive, multiplicative, shift, bitwise (through compound assignment and the usual arithmetic conversions), assignment | yes |
| X18 | P18, required `override` | virtual, derived, keywords | yes |
| X19 | P1, width-named literal suffixes | primary, types, keywords | no |
| X20 | P17, opaque type aliases | type-names, templates arguments, enumerations | no |

The page lists are aims. Each reader states which pages the design reached. X17 and X18 change the meaning or validity of existing programs, so their impact sections exercise item 4 and item 6.

**Evidence.** Sections 1.3, 1.4 and 1.7. The arithmetic pages were last read by X1, and 32 commits have changed them since. A first reading finds wrong principles that Predict batches pass.

**Cost.** One reader session each, then one writer session for repairs.

**Risk.** Low for similarity. X19 touches lexical structure, which stays deferred beyond the slice in PRIM-26 and the keyword tables. The brief should keep it to PRIM-26.

**What it unblocks.** After this round every proposal in the portfolio is done or held by a decision. The designer then needs new proposals, or a different exercise kind (item 7, pair exercises).

### Item 6. A construct count over `tests/programs/`

**What it is.** The second half of round 4 item 6. A small tool counts, over the 67 program tests:
- the rules each program cites (from its `tidal-rules` line),
- named constructs that a proposal can ask about, through the front end's parse (chained comparisons, narrowing conversions, `override`, default arguments, and so on).

A Propose report then quotes the count in its impact section, with the caveat that readers wrote the programs.

**Evidence.** Section 1.6. The programs exist and pass on every baseline compiler, and in the front end 63 of 67 pass and 4 differ from EDG only. X16 counted chains over the whole suite by hand with three scripts.

**Cost.** Small, if the front end can print what it parsed. A token scan is the fallback.

**Risk.** Low. 7610 lines by clean-room readers give only rough frequency. Reports must say so.

**What it unblocks.** Q-0183 option 2 in part. A base for item 8 that is not made of single-rule tests.

### Item 7. Pair exercises

**What it is.** Unchanged from round 4 item 3. An exercise takes two done proposals that touch the same rules and writes only their interaction section, with a change table for the combined rules.
- **X9 and X11** (relocation and scope-exit blocks). Both amend OBJ-3 and the block-exit rules.
- **X10 and X12** (exceptions in constant evaluation and exhaustive `switch`). Both add termination points.

A third pair is now possible. X14 (packed classes) and X9 (relocation) both depend on layout and on what a byte copy of a member means.

**Evidence.** Round 3 and round 4 both proposed this. It was not taken.

**Cost.** One session per pair.

**Risk.** Low. Both early pairs cite rules that phases 6 and 7 rewrote, so item 1 must run first.

**What it unblocks.** A way to state how two changes compose, which no exercise tests today.

### Item 8. A front-end impact probe that changes one rule

**What it is.** Unchanged from round 4 item 8. A front-end mode changes one rule's class or result, runs the suite and `tests/programs/`, and lists the tests whose outcome moves. The mode reports only over tests that the front end passes today.

**Evidence.** The front end passes 6068 of 6337 tests, up from 5595 of 6032. `conformance.tsv` still lists 121 missed rejects. X16 filled its Tests column from two compilers' warnings and a token scan.

**Cost.** Large.

**Risk.** Medium. Outcomes for the tests the front end fails stay unknown.

**What it unblocks.** Measured Tests columns in change tables.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them. X15 kept clear of modules, and X13 left library parameter names unspecified.
- **Relocation by bytes (P8b)** until the maintainer decides `pending.md` row 2 (Q-0322).
- **Rules or `:since:` fields for later versions.** Item 2 makes an index only. Q-0185 stays provisional.
- **A source-reader designer** (Q-0116). The clean-room designer has not needed one in sixteen exercises.
- **The preprocessor beyond `#include`.** X16-7 notes that P4 cannot speak about `#if`, and no proposal needs more.
- **Vendor attributes beyond ATTR-20.** ATTR-20 states that a recognized attribute has an implementation-defined effect. A list of vendor attributes would be outside knowledge.

## 4. Process changes

1. **A refresh check that reads every report.** Item 1. Until then, `exercises.md` section 4 should name the column headers a change table uses, so new reports match the tool.
2. **Rate repairs in the exercise status.** `exercises.md` section 8 still gives finding counts only. Section 1.3 shows the form: closed, to a question, no action. Rounds 3 and 4 asked for this.
3. **Probes flush before they abort, in the prompt.** X14 and X16 did this because round 4 asked. The Propose prompt in `exercises.md` does not say so, and the next reader may not read round 4.
4. **Propose prompts point to `tests/programs/`.** X16 wrote "Tidal has no corpus" while scanning the corpus. The prompt should name the program tests as the corpus, with their size and origin.
5. **Keep the designer's view in round 6,** in this form, with the counts of sections 1.1, 1.4 and 1.5 rerun.

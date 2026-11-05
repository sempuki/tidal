# Roadmap, round 6: what to do after phase 8, from a language designer's view

Date: 2026-11-04. Decision 0029, with decisions 0037, 0039 and 0055. Role: clean-room reader, language-designer persona. Inputs: AGENTS.md and CLAUDE.md; `reports/clean-room/roadmap-5/designer.md` (the model for this form); `documentation/` (charter, roadmap, status, exercises, specification format, decisions 0039, 0054 and 0055, and `decisions/pending.md`); `reports/clean-room/X17/` to `X20/` (sections 4 and 5 in full, headers of the rest); the headers and findings lists of `I1/`, `W16/`, `W17/`, `R8/` and `rust-frontend/report-18.md` and `report-19.md`; `questions/README.md` and the files Q-0064 and Q-0380 to Q-0383; TR-4 and its Evidence in `specification/foundations/translation.md`; `tools/propose_check.py` and its output; rule text under `specification/` through targeted searches; and `git log --oneline dc90663~1..HEAD` with the commit messages of 076aa54, e11f565 and 6007252. Counts come from read-only Python scripts in my scratch directory, run on the tree at commit 1b16115. I did not read the other round-6 reports. I did not run `tools/similarity.py`. Statements from outside Tidal's files are marked "general knowledge".

## 1. The state from a designer's view

### 1.1 Counts at 1b16115

| Measure | Round 5 (7d8c982) | Now (1b16115) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1691 | 1696 |
| `required` | 895 | 898 |
| `reject` | 557 | 558 |
| `undefined` | 57 | 57 |
| `impl-defined` | 52 | 52 |
| `recommended` | 44 | 44 |
| `no-diagnostic` | 35 | 36 |
| `unspecified` | 31 | 31 |
| `conditionally-supported` | 5 | 5 |
| `withdrawn` | 15 | 15 |
| Test files with a `tidal-expect` line | 6334 | 6497 |
| Of these, `reject` | 2633 | 2684 |
| Of these, `link-fail` | 44 | 44 |
| Program tests under `tests/programs/` | 67 | 71 (89 files, 9185 lines) |
| Questions | 376 | 385 |
| Provisional | 209 | 221 |
| Settled | 154 | 151 |
| Propose reports | 16 | 20 |
| Change-table rows that `propose_check.py` reads | 797 (X14 read as 0 then) | 971 |
| Front end passes (status page) | 6068 of 6337 | 6183 of 6500 |

The status page counts 6500 tests, and my script counts 6497 files with a `tidal-expect` line. I did not trace the difference of three. The two front-end counts do not compare, because item HN changed how the harness judges a test. W16 and W17 add 4 program tests and 1473 lines. My line count includes headers and the files of multi-unit programs, so it is higher than the 7712 lines that X19 and X20 give for the 67 earlier programs.

### 1.2 What phase 8 did for the designer

Phase 8 took four of round 5's eight items. It took two of round 5's five process changes, and a third in practice only.

| Round-5 item | Phase 8 | State |
|---|---|---|
| 1. Close the two gaps in the refresh check | item R2 | done. The tool reads both header forms, warns on a table it cannot read, and lists rows changed since a named commit. A gap remains for reports that name no commit (section 1.5) |
| 2. The later-version probe as a costed plan | item R2 | done as a plan. `pending.md` row 7 holds a four-step plan with costs. It still waits on the maintainer |
| 3. A rule class for link-time diagnostics (D30) | item TE | done as a policy. TR-4 says that no class requires a diagnostic at link time, and each of the 44 `link-fail` tests cites a `no-diagnostic` rule |
| 4. An index of what a test of validity can see | none | deferred by decision 0055 |
| 5. Propose round 5, X17 to X20 | item E, commits 1a4c9e2 and 076aa54 | done. 44 findings |
| 6. A construct count over `tests/programs/` | none | deferred by decision 0055 |
| 7. Pair exercises | none | deferred by decision 0055. Proposed in rounds 3, 4 and 5 |
| 8. A front-end impact probe | none | not started |

Of round 5's process changes:
- Taken: `exercises.md` section 4 names the column headers the tool reads, and this report keeps the view.
- Taken in practice only: X17 to X20 used `tests/programs/` as the corpus and gave its size and origin.
- Not taken: `exercises.md` section 8 still gives finding counts with no count of what was closed. The rows for X17 to X20 now also give how many tests change outcome, which is new. `exercises.md` has no Propose prompt in section 9, so nothing in the repository tells a reader to flush probes or to use `tests/programs/`. X17 to X20 used the corpus, and X18 and X20 say that their probes flushed, because their briefs asked.

Section 4 of `exercises.md` has drifted. Its exercise table stops at X16, and its prose names rounds 1 to 3 only. Section 6 says X17 to X20 are done.

### 1.3 What X17 to X20 found, and what the repairs closed

| Exercise | Proposal | Findings | Closed in the specification | To a question | No action |
|---|---|---|---|---|---|
| X17 | P2, narrowing is an error everywhere | 11 | 9 (X17-1 to X17-9) | X17-7 adds a note to Q-0124 | X17-10, X17-11 |
| X18 | P18, required `override` | 11 | 9 (X18-1 to X18-9) | X18-3, Q-0382 | X18-10, X18-11 |
| X19 | P1, width-named literal suffixes | 11 | 9 (X19-1 to X19-9) | X19-1, Q-0383; X19-4, Q-0380; X19-6 recorded in Q-0064 | X19-10, X19-11 |
| X20 | P17, opaque type aliases | 11 | 9 (X20-1 to X20-9) | none | X20-10, X20-11 |

Commit 076aa54 acted on all 36 findings that a writer can act on, in one commit. A finding sent to a question also got rule text or tests, so I count it as closed. X13 to X16 gave 39 findings and 31 closed. X17 to X20 gave 44 and 36 closed. A first designer reading of a page area still finds about nine gaps.

Five findings were wrong or contradictory text on pages that Predict batches had passed:
- X17-5: CMP-P1 said a negative `int` compared with an `unsigned` becomes a large unsigned value. The sentence was false for `<=>`.
- X17-6: BF-7 and LIST-2 contradicted each other on the width of a bit-field.
- X18-1: KW-3 gave `override` a meaning after a class name, and cited the wrong rule.
- X18-9: the Syntax segment of `virtual.md` defined a `virtual_decl` that suggested `override` needs `virtual`.
- X19-7: UN-3 did not say that the unsigned negation applies to the promoted operand, so a reader computed 255 for a value of -1.

The eight findings with no action are lists of outside knowledge (X17-10 to X20-10) and lists of needs outside the specification (X17-11 to X20-11). The second kind repeats the standing needs:

| Need | Raised by X17 to X20 | Raised before |
|---|---|---|
| Version marking | X17-11, X18-11, X19-11, X20-11 | X13 to X16 |
| Implementation cost (Q-0183) | X17-11, X18-11, X19-11, X20-11 | X14, X15 |
| Target dependence | X17-11 (validity under P2 follows TYP-1 on ILP32) | none |
| The preprocessor (`#if 1u8`) | X19-11 | X16 |
| A library chapter (stream output of `unsigned char`) | X19-10 | X13 |

Each of X17 to X20 also built its own impact tool, and none is in the repository. X17 used a scanner over a compiler's syntax tree with five stated limits. X18 used Clang's override warnings over all 76 program files and 251 test files. X19 and X20 used text scans over `tests/programs/`. Each report directory holds only `report.md`.

### 1.4 Pages the Propose exercises have reached

I mapped every rule ID cited in X1 to X20 to its page. All 85 pages are reached. My method counts `:::{rule}` IDs only, and not principle IDs, so its list of thinly read pages can differ from round 5's.

Seven pages are reached by one exercise only:

| Page | Exercise | Commits to the page since the commit that exercise read, or since the report was added |
|---|---|---|
| `expressions/comma.md` | X1 | 5 |
| `templates/constraints.md` | X8 | 14 |
| `statements/loops.md` | X11 | 4 |
| `declarations/alignment.md` | X14 | 5 |
| `declarations/inline.md`, `namespaces.md` | X15 | 2 and 5 |
| `expressions/bitwise.md` | X17 | 1 |

Round 5 listed twelve such pages. X17 reached additive, multiplicative and shift a second time, X19 reached types, and X20 reached type-names and enumerations. `templates/constraints.md` is the page most changed since its one reading.

### 1.5 The refresh check at 1b16115

`tools/propose_check.py` reports 20 reports, 30 conflicts (10 withdrawn, 3 class, 17 new-id, 0 unknown), 104 rows changed since their report's commit, and 0 warnings. It now reads X14 (56 rows). The changed rows are X13 12, X14 16, X15 19, X16 6, X17 22, X18 8, X19 12 and X20 9.

Two limits remain.

**Reports that name no commit get no change list.** X1 to X12 name no commit, so the tool cannot say which of their rows changed. As a test, I gave the tool's own functions the commit that added each report as a stand-in for the commit it read. X5 to X12 then show 268 changed rows of 492. X1 to X4 show 0, because the tool finds no directive text at their commit a606e10. The stand-in commit can be later than the commit the reader read, so 268 is a lower bound. Pair exercises (item 7) read X9 to X12, which show 134 changed rows among them.

**A change list does not separate a report's own repairs from other changes.** 22 of X17's rows changed. Most of these changes came from commit 076aa54, which acted on X17's own findings. A reader cannot tell from the output which changes answer the report and which came from other work.

### 1.6 What a designer still cannot state

**Whether a later version has taken the space.** All four of X17 to X20 raise this. So 11 of the 12 Propose reports from X9 on raise it, as a finding or with their own probe. `pending.md` row 7 now holds a plan for the probe with costs (a runner option, one matrix run of about 7900 compilations, triage, and an index with no rules). The plan names about 2640 `reject` tests, and there are 2684 now. Q-0007 is still provisional.

**What a change costs.** All four of X17 to X20 give costs they cannot state. Q-0183 keeps option 1, and `pending.md` row 9 lists it. X20-11 lists three kinds of cost (a candidate family in overload resolution, an identity rule for template-ids, and a lost optimization) with no scale to put them on.

**How much code a change touches.** X17 counted 74 tests that change to `reject` and 3 sites in the 67 programs. X18 counted 23 `override` declarations and no missing ones. X19 counted 3413 integer literals and no width suffix. Each count came from a tool that the next reader cannot rerun.

**Which programs change meaning without a diagnostic.** X17 4.3 found that requirements such as `take_int(t)` change their answer for `double` under P2 with no diagnostic, and it named round 5's item 4 as the index it would have used. X18, X19 and X20 each argued in their section 4.3 that no detection code changes its answer, by hand. My scan finds 32 of the 558 `reject` rules cited by an `accept` or `run` test whose text contains `requires`. Round 5 found 28 by the same method. No index says which rejections a test of validity can see.

**Validity on the other target.** Under P2, whether a conversion narrows depends on the widths of TYP-1, such as `long` on ILP32 and the signedness of plain `char`. 75 of 6497 test files carry `-m32`. X17's scanner read every test as LP64.

### 1.7 The portfolio, updated

Only the rows that moved are shown. The others are as in round 5.

| # | Proposal | Round 5 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P1 | Width-named literal suffixes | ready | done (X19) | | |
| P2 | Narrowing is an error everywhere | ready | done (X17) | | |
| P8b | Relocation by bytes | partial | partial | Q-0322 records the EDG 7.0 results (item R2) | `pending.md` row 2 |
| P15 | Declared initialization order | partial | ready | TR-4 states that no class requires a link-time diagnostic (item TE) | The proposal must add the class it needs, or make a cycle `no-diagnostic` |
| P17 | Opaque type aliases | ready | done (X20) | | |
| P18 | Required `override` | ready | done (X18) | | |

Totals now: 20 done, 1 ready (P15), 1 partial (P8b), 0 blocked. P15 is the last proposal a reader can take without a maintainer decision. The portfolio needs new proposals.

P17's name now uses a word with another meaning. X20-7 notes that "opaque" names an enumeration declaration without enumerators, and the glossary now has an entry for that term. X20 called its feature an "explicit alias".

### 1.8 Questions that wait on a principle a reader has used

Decision 0055 holds back a question that would settle on a principle widened in phase 8 until a reader or reviewer uses that principle. 60 provisional questions carry this note. X18, X19 and X20 cite seven of the widened principles: ATTR-P1 (X18), OPR-P3 and PRIM-P1 (X19), and DED-P3, ENUM-P2, OVL-P4 and TYN-P1 (X20). Only Q-0064 records the use (X19-6), and it says PRIM-P1 passes the test of the style guide in that use. The other questions do not say whether a citation in a Propose report counts as a use. The pages with the most such questions are `alignment.md` (5), `new-delete.md` (4), and `enumerations.md`, `attributes.md` and `unions.md` (3 each).

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | The later-version probe: a maintainer decision on `pending.md` row 7 | 1.6: 11 of 12 reports from X9 on raise version marking; the plan is written and costed | one decision; then one session, one matrix run, and triage | medium | the version line of every proposal; Q-0007, Q-0087, Q-0181, Q-0188 |
| 2 | An index of what a test of validity can see | 1.6: seven reports worked out visibility by hand; X17 asked for this index by name | 2 writer sessions | medium | section 4.3 of every proposal |
| 3 | A shared impact scanner over `tests/` and `tests/programs/` | 1.3 and 1.6: four readers built four tools, none kept | 1 writer session | low | the Tests columns and section 4.5 of every proposal; Q-0183 option 2 in part |
| 4 | Propose X21 on P15 | 1.7: P15 is ready; TR-4 is new and no reader has used it | 1 reader session, then repairs | low | the portfolio's last ready proposal; a reading of TR-4, SINIT and LNK |
| 5 | New proposals, then Propose X22 to X24 | 1.4: seven pages read once; 1.7: no ready proposal after P15; 1.8: 60 questions wait on a use | 1 portfolio session, 3 reader sessions, 1 repair session | low | uses of widened principles; second readings of thin pages |
| 6 | The refresh check for reports with no commit, and the commit that changed each row | 1.5: 268 or more changed rows in X5 to X12 go unlisted | small tool | low | items 7 and 4 read old reports |
| 7 | Pair exercises | proposed in rounds 3, 4 and 5; new pairs from X17 to X20 | 2 reader sessions | low | a way to state how two changes compose |
| 8 | ILP32 runs for tests of conversion rules | 1.6: 75 of 6497 tests run `-m32`; validity under P2 follows TYP-1 | 1 writer session and a matrix run | low | the target line of a proposal that changes conversions |

### Item 1. The later-version probe: a maintainer decision on `pending.md` row 7

**What it is.** Phase 8 wrote the plan that round 5 asked for. The plan is in `pending.md` under "Later-version probe". The item for phase 9 is the maintainer's decision on row 7, then the four steps of the plan if the maintainer approves. The plan's count of `reject` tests should read 2684.

**Evidence.** Section 1.6. Every Propose report since X13 raises version marking, and four of them name it as the first need outside the specification.

**Cost.** One decision. If approved, one session for the runner option, one matrix run, and one triage session for each few hundred flips.

**Risk.** Medium, and in triage, as the plan says. A writer who triages can recall later-version material. The index names compiler behavior only.

**What it unblocks.** The version line of every proposal. Data for Q-0007, Q-0087, Q-0181 and Q-0188.

### Item 2. An index of what a test of validity can see

**What it is.** Round 5 item 4, unchanged in form. For each `reject` rule, one value:
- **visible:** the error can occur where substitution (DED-24) or a requires expression (CPT-26) sees it,
- **fatal:** the error is always fatal (DED-29, or outside any template),
- **syntax:** only the grammar rejects the form (TR-12).

A rule visible in one position and fatal in another gets both values with a note. A test for a sample of the "visible" rules observes the rejection from inside a requires expression. X17-8 added one such test, for LIST-2.

**Evidence.** Section 1.6. X10, X12, X16, X17, X18, X19 and X20 each decided visibility by hand. X17 found a silent change of answer for P2. X19 found that an invalid literal is fatal even inside a requires expression, which is a "fatal" value for PRIM-16 that no page states.

**Cost.** Two writer sessions.

**Risk.** Medium. The value of some rules depends on position, and some rules need a probe.

**What it unblocks.** Section 4.3 of every proposal. A check of item 3's counts for changes that no compiler warns about.

### Item 3. A shared impact scanner over `tests/` and `tests/programs/`

**What it is.** One tool under `tools/` that a Propose reader runs to count constructs. It replaces round 5 item 6 (the construct count) and round 5 item 8 (the front-end impact probe) for now. It reads each test with the compiler's syntax tree, as X17's scanner did, and honors `tidal-flags` so that `-m32` tests count on their own target. A first set of queries comes from the reports:
- implicit conversions by context and by source and target type (X17),
- overriding declarations with and without `override` (X18),
- literals by base and suffix (X19),
- alias declarations, alias templates and scoped enumerations (X20).

The output separates `tests/programs/` from the single-rule tests, because the two give very different rates (X17 4.4: 74 tests against 3 program sites).

**Evidence.** Section 1.3. Four readers wrote four tools, and the next reader cannot rerun any of them. X17 listed five limits of its scanner, such as reading every test as LP64.

**Cost.** One writer session for the frame and the four queries. Each later Propose report adds its query.

**Risk.** Low. A syntax tree from a compiler is behavior of an implementation, which a clean-room writer may use. The program tests remain reader-written, and reports must say so.

**What it unblocks.** Measured counts in section 4.5 of every proposal. Q-0183 option 2 in part, since a count of affected sites is one part of a cost.

### Item 4. Propose X21 on P15

**What it is.** A Propose exercise on P15, declared initialization order across translation units. The design must choose between a cycle that is `no-diagnostic` and a new class for a diagnostic at link time, against TR-4's stated policy. The page list is an aim: `storage-linkage.md`, `static.md` under initialization, `odr.md`, `translation.md` and `inline.md`.

**Evidence.** Section 1.7. P15 has been partial since round 1, on D30 alone. TR-4 now answers D30, and no reader has used the new TR-4 text. `inline.md` is read once (1.4).

**Cost.** One reader session, then a share of a repair session.

**Risk.** Low. Multi-unit tests exist (decision 0014), and 44 `link-fail` tests show the current form.

**What it unblocks.** The last ready proposal. A designer reading of TR-4 and of the SINIT rules, which the Predict batches score by outcome only.

### Item 5. New proposals, then Propose X22 to X24

**What it is.** A designer session adds proposals P22 onward to the portfolio, rated as in round 3. Each is aimed at a page read once (1.4) or at a page whose widened principles wait on a use (1.8). Candidates, each my own and each a first pass:

| # | Candidate | Pages it aims at | Changes existing behavior |
|---|---|---|---|
| P22 | Labeled `break` and `continue` | loops, jumps, blocks | no |
| P23 | Template bodies checked against their constraints | constraints, concepts, dependent-names, instantiation | yes |
| P24 | Flag enumerations: a marked scoped enumeration gets `|`, `&`, `^` and `~` in its own type | bitwise, enumerations, overload-resolution | no |
| P25 | `new` and `delete` that honor every alignment, or reject it at compile time | new-delete, alignment, storage | yes |
| P26 | No overloaded comma, and a discarded left operand of `,` must have no value or be a call | comma, operators, evaluation-order | yes |

Then X22 to X24 take three of them. P23 reaches the most changed once-read page. P25 reaches the two pages with the most questions that wait on a use (9 between them). The motivation for each candidate is general knowledge, from other languages and common practice.

**Evidence.** Sections 1.4, 1.7 and 1.8. After X21 the portfolio has no ready proposal. Each first reading of a page area has found about nine gaps, five of them in X17 to X20 on text that Predict had passed.

**Cost.** One portfolio session, three reader sessions, one repair session.

**Risk.** Low for similarity. P22 and P23 are areas where a later version may act, which makes item 1 more useful (general knowledge). P23 is large and may need its scope cut to one form of check.

**What it unblocks.** Uses of the widened principles that 60 questions wait on, which lets them settle under decision 0055. Second readings of seven pages.

### Item 6. The refresh check for reports with no commit, and the commit that changed each row

**What it is.** Two small changes to `tools/propose_check.py`.
- For a report that names no commit, the tool uses the commit that added the report and says so ("added at 85583a8, no commit named"). The note keeps the result a lower bound.
- Each "changed since" line names the latest commit that changed the rule's directive text. A reader then sees that 076aa54 is the repair commit for X17 and can set those rows apart.

The tool must also read the directive text at a606e10, or say that it cannot, so X1 to X4 do not show a silent 0.

**Evidence.** Section 1.5. With the stand-in commit, X5 to X12 show 268 changed rows that the tool does not list today.

**Cost.** Small. Both values come from `git log`.

**Risk.** Low. The check stays a warning, as `exercises.md` section 4 says.

**What it unblocks.** Item 7, which reads X9 to X12, and item 2, which cites old reports.

### Item 7. Pair exercises

**What it is.** Round 5 item 7. An exercise takes two done proposals that touch the same rules and writes only their interaction section, with a change table for the combined rules. Pairs, in the order I would take them:
- **X17 and X19** (narrowing everywhere and width-named suffixes). X19's probe showed that `signed char s{U8{200}}` narrows. Both amend conversions of small integer types.
- **X17 and X20** (narrowing everywhere and explicit aliases). An explicit type over an integer must say which P2 conversions apply.
- **X9 and X11** (relocation and scope-exit blocks), and **X14 and X9** (packed classes and relocation), from earlier rounds.

**Evidence.** Proposed in rounds 3, 4 and 5. X17 to X20 add two pairs whose reports read the same commit, dad8bf8, so no refresh is needed for them.

**Cost.** One session per pair.

**Risk.** Low. The older pairs need item 6 first.

**What it unblocks.** A way to state how two changes compose, which no exercise tests.

### Item 8. ILP32 runs for tests of conversion rules

**What it is.** A writer adds `-m32` variants for the tests of CONV-2 to CONV-11, CONV-19, CONV-21 and LIST-2 where the result depends on the widths of TYP-1, and lists the variants in the TYP-1 Evidence. The matrix already has EDG 7.0 with ILP32 and the GCC and Clang `-m32` runs.

**Evidence.** Section 1.6. X17-11 found that validity under P2 depends on `long` and plain `char`. 75 of 6497 test files run `-m32`.

**Cost.** One writer session and one matrix run over the changed directories.

**Risk.** Low.

**What it unblocks.** The target line of any proposal that changes conversions or integer types, such as P1, P2 and P24.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them. X19-10 touched stream output and left it as outside knowledge.
- **Relocation by bytes (P8b)** until the maintainer decides `pending.md` row 2.
- **Rules or `:since:` fields for later versions.** Item 1 makes an index only. Q-0185 stays provisional.
- **A source-reader designer** (Q-0116). The clean-room designer has not needed one in twenty exercises.
- **The preprocessor beyond `#include`.** X16-7 and X19-11 note that P4 and P1 cannot speak about `#if`. No proposal needs more.
- **A front-end impact probe that changes one rule.** Item 3 gives the counts that the probe was for, and the front end still fails 317 of 6500 tests. Return to it after item 3.
- **A cost model with numbers** (Q-0183 option 2 in full). Item 3 gives counts of affected sites. A scale for implementation effort needs implementation experience that Tidal does not have.

## 4. Process changes

1. **A Propose prompt in `exercises.md` section 9.** It says: run `tools/propose_check.py` first; name the commit read; use the change-table headers the tool reads; flush probes before they abort; use `tests/programs/` as the corpus, with its size and origin; run the impact scanner (item 3) and add a query to it; and list each widened principle that the design uses, with the question that waits on it.
2. **Keep the Propose table current.** `exercises.md` section 4 should list X17 to X20 and later exercises, and say which round each belongs to. The rule-change counts in section 8 should come from `propose_check.py`. Today section 8 says 57 for X14 and X20, and the tool reads 56 and 58.
3. **Rate repairs in the exercise status.** Section 8 still gives finding counts only. Section 1.3 shows the form: closed, to a question, no action. Rounds 3, 4 and 5 asked for this.
4. **Record uses of widened principles.** When a Propose report cites a principle that a question waits on, the repair commit applies the test of the style guide to that use and records the result in the question, as Q-0064 does for X19. Section 1.8 lists six principles that X18 to X20 cite with no record.
5. **Keep the impact scripts.** A Propose reader saves its scanner and probe scripts in its report directory, as I1 saved `layout.py` and `score.py`.
6. **Rename P17 in the portfolio** to "explicit aliases", so "opaque" keeps one meaning (X20-7).
7. **Keep the designer's view in round 7,** in this form, with the counts of sections 1.1, 1.4, 1.5 and 1.8 rerun.

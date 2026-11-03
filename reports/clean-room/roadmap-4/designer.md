# Roadmap, round 4: what to do after phase 6, from a language designer's view

Date: 2026-11-03. Decision 0029, with decisions 0037, 0039 and 0051. Role: clean-room reader, language-designer persona. Inputs: AGENTS.md and CLAUDE.md; `reports/clean-room/roadmap-3/designer.md` (the model for this form); `documentation/` (charter, roadmap, status, exercises, decisions 0039, 0044, 0050 and 0051); `reports/clean-room/X9/` to `X12/` (design summaries, interactions, impact sections and findings in full; change tables by script); the head of `rust-frontend/report-14.md`; the heads of the 2026-11-03 gap reviews and the class-property inventory in `reports/review/`; `questions/README.md` and Q-0007, Q-0183 and Q-0312; rule text under `specification/` through targeted searches; and `git log --oneline 39e9322..HEAD` with `git show --stat` for commits 90ab3e9, bddd2a0 and aae6c9e. Counts come from read-only Python scripts in my scratch directory, run on the tree at commit 3d98c93. I did not read the other round-4 reports. I did not run `tools/similarity.py`. Statements from outside Tidal's files are marked "general knowledge".

## 1. The state from a designer's view

### 1.1 Counts at 3d98c93

| Measure | Round 3 | Now |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs | 1633 | 1678 |
| `required` | 875 | 892 |
| `reject` | 547 | 553 |
| `undefined` | 56 | 56 |
| `impl-defined` | 43 | 51 |
| `recommended` | 43 | 44 |
| `no-diagnostic` | 35 | 33 |
| `unspecified` | 29 | 30 |
| `conditionally-supported` | 4 | 4 |
| `withdrawn` | 1 | 15 |
| Test files that expect `reject` | 2355 | 2564 |
| Test files that expect `link-fail` | 36 | 39 |
| Questions open for opinions | 283 | 310 (309 provisional, 1 open) |

My script counts 6029 test files with a `tidal-expect` line: 2564 `reject`, 2019 `accept`, 1323 `run`, 84 `run-fail` and 39 `link-fail`. The status page says the front end passes 5595 of 6032 tests and the matrix passes 5609 of 5609. I did not trace the difference between these totals and mine. The question index has 352 questions: 310 open for opinions, 12 compiler defects and 30 settled.

### 1.2 What phase 6 did for the designer

Phase 6 took three of round 3's eight items and two of its five process changes.

| Round-3 item | Phase 6 | State |
|---|---|---|
| 1. Repair batch for open designer findings | item D, commit 90ab3e9 | done. PRIM-26 (D3), four CONV-20 rows (D4, D6), CMP-14 (D10), SEQ-11 (D11), FN-16 and the FN-P1 row (D15), THR-13 (D25), CE-31, since merged into CE-8 (D27), the two CE-P4 rows, the FN-P1 blank line, and the coroutine note at OBJ-3 and UNW-P1 |
| 2. Propose round 3, X9 to X12 | item D, commits 85583a8 and bddd2a0 | done. 49 findings |
| 3. Free-space index with a visibility column | none | not started |
| 4. Probe of later-version space | none | waiting on the maintainer (status page, last item) |
| 5. Corpus of whole programs | item W ran every Write suite | programs exist, no corpus |
| 6. Front-end impact probe | none | not started |
| 7. Rule class for link-time diagnostics (D30) | none | open |
| 8. Class properties beyond S1 | item O | done. CPY-19 and CLS-19 are complete, and both *(class properties referenced)* markers are gone |

Process change 1 (proposal IDs in their own space, such as `X9:new-1`) is now in `documentation/exercises.md` section 4, and X9 to X12 use it. Process change 5 (keep this view) is this report. Process changes 2 (a refresh check), 3 (a pair exercise) and 4 (rate partial repairs) were not taken.

Of the round-1 findings, only D30 is still open.

### 1.3 What X9 to X12 found, and what the repairs closed

| Exercise | Findings | Closed in the specification | Elsewhere | No action |
|---|---|---|---|---|
| X9 | 15 | 12 (X9-2 to X9-13) | X9-1 by item O | X9-14, X9-15 |
| X10 | 12 | 12 | | |
| X11 | 9 | 9 | | |
| X12 | 13 | 11 (X12-1 to X12-11) | | X12-12, X12-13 |

45 of 49 findings are closed. Two of the four with no action, X9-15 and X12-13, raise the same two standing needs as every earlier round, version marking and implementation cost. X9-14 and X12-12 list outside knowledge. The repair commit says that X11-3 was partly wrong, because its probe printed with `puts` and then aborted, so the buffered output was lost.

The rate holds steady. X5 to X8 gave 64 findings over four exercises, and X9 to X12 gave 49. Each round still finds about a dozen gaps per exercise that a writer can close.

### 1.4 Pages no Propose exercise has reached

I mapped every rule and principle ID cited in X1 to X12 to its page. 80 of 85 pages are reached. X9 to X12 reached 49 pages, seven of them for the first time: storage, types, virtual, enumerations, type-names, primary and blocks. Five pages are still unreached, all in the declarations chapter:

- `declarations/alignment.md` (ALN, 8 rules),
- `declarations/functions/default-arguments.md` (DEFARG, 9 rules),
- `declarations/inline.md` (INL, 8 rules),
- `declarations/linkage-specifications.md` (LSP, 7 rules),
- `declarations/namespaces.md` (NS, 17 rules).

My method counts IDs only. Round 3 counted 74 pages for X1 to X8, and my script gives 73 for the same reports, so the two methods differ by about one page.

### 1.5 Propose reports go stale

A script of mine reads the change table of each Propose report and checks each row against the rules at 3d98c93. A row conflicts when it names a rule that is now withdrawn, when its old class differs from the current class, or when its "new" ID now names a real rule.

| Conflict | Rows | Where |
|---|---|---|
| The row amends or relies on a rule that is now withdrawn | 10 | SEQ-3 (X1); INST-14 (X6, X7); UNW-9 (X9, X10, X11); CE-31 (X10, X11, X12); CX-6 (X11) |
| The old class differs from today's class | 1 | CE-11 (X7: `required` then, `reject` now) |
| A "new" row's ID now names a real rule | 16 | X2 (2), X5 (3), X6 (3), X7 (2), X8 (6) |

Commit aae6c9e (item C, hotspot pages) withdrew 12 rules by merging them into others. CE-31 is one of them. X10's design rests on CE-31 carrying the termination cases, and three reports dated 2026-11-03 cite it. So a Propose report can go stale within days of being written. Each withdrawn rule says where it went ("merged into CE-8"), so a tool can point to the new target. No Propose report names the commit it read.

The 16 ID collisions are all from reports written before the `X9:new-1` form. Round 3 counted 14. CLS-29 and CLS-30, added by item O, account for the rise.

### 1.6 What a designer still cannot state

**Whether a later version has taken the space.** Q-0007 still tracks C++23, and `tools/compilers.json` sets only `-std=c++23` (16 entries). X10 ran its own probes under `-std=c++2c` and found that GCC 16 accepts each form that P11 makes valid and rejects each form that P11 keeps rejected. So GCC's next-version mode already uses that space with the same design. X12-13 raises the need again. Six questions cite Q-0007, and the 2026-11-03 gap reviews mention later changes in 15 places.

**What a change costs.** Q-0183 keeps option 1 (cost in words). X9 to X12 each say "Tidal has no corpus of real programs". Phase 6 item W ran all fourteen planned suites. The reports under `reports/clean-room/W*/` and `java-programmer/` now hold 65 top-level `.cpp` files, plus W14's four multi-unit programs in subdirectories, about 7200 lines in all. Some of these files are probes rather than task programs (W3 has three). No manifest lists the task programs, and no tool counts constructs over them.

**Which programs change meaning.** X10 4.3 and 4.4 found three channels that turn a newly valid constant evaluation into a change in meaning: substitution (DED-24), requires expressions (CPT-26), and the choice between constant and dynamic initialization (SINIT-1, SINIT-4). The third channel is new to the designer's list. It narrows an unspecified choice and changes the value both compilers give today. X12 4.3 found the opposite case. A `switch` is a statement, so no test of validity can see it. Each report again worked this out by hand. My scan finds 3 of the 553 `reject` rules cited by an `accept` or `run` test that contains a requires expression. So the suite almost never shows how a rejection looks from inside a test of validity.

**Forms that only the grammar rejects.** Item G made TR-12 the general rule, and 7 tests cite it. Q-0312, the one question still `open`, leaves to the maintainer whether the roadmap keeps a list of such forms (option 3).

**Link-time diagnostics (D30).** The TR-P1 class table has no class for a program that a correct implementation rejects only at link time. 39 tests expect `link-fail`. P15 still waits on this.

### 1.7 The portfolio, updated

| # | Proposal | Round 3 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P1 | Width-named literal suffixes | ready | ready, complete | PRIM-26 (D3) | |
| P2 | Narrowing is an error everywhere | ready | ready, complete | CONV-20 rows (D4, D6) | |
| P3 | Left-to-right evaluation | done | done | | |
| P4 | Chained comparisons | ready | ready, complete | CMP-14 (D10), SEQ-11 (D11) | |
| P5 | Automatic objects start at zero | done | done | | |
| P6 | Named arguments | partial | ready | FN-16 and the FN-P1 row (D15) | |
| P7 | Temporaries through reference chains | done | done | | |
| P8 | Relocation by a relocation function | partial | done (X9) | | |
| P8b | Relocation by bytes | (part of P8) | partial | CPY-19 and CLS-19 complete (item O) | STG-8 creates objects from a byte copy only through library operations, which stay referenced (Q-0322). A rule for a class whose bytes can move needs a new property (X9-1) |
| P9 | Overload sets as values | done | done | | Q-0142 is still provisional. EDG differs, so decision 0051 keeps it open |
| P10 | Expansion over elements | done | done | | |
| P11 | Exceptions inside constant evaluation | ready | done (X10) | | |
| P12 | Value-specialized parameters | done | done | | |
| P13 | Checked `noexcept` | done | done | | |
| P14 | Scope-exit blocks | ready | done (X11) | | |
| P15 | Declared initialization order | partial | partial | | D30 |
| P16 | Member call syntax | done | done | | |
| P17 | Opaque type aliases | partial | ready | CPY-19 complete (D21) | Class-template and argument equivalence still need a check |
| P18 | Required `override` | ready | ready | | |
| P19 | Exhaustive `switch` over a marked enumeration | ready | done (X12) | | |
| P20 | Packed classes: a class marker that gives each member alignment 1 | new | ready | ALN-1 to ALN-8, CLS-19, STG-2, member pointers | Must say what a reference or pointer to a member that is not aligned means |
| P21 | Names private to a namespace: a section of a namespace definition whose names lookup finds only inside definitions of that namespace | new | ready | NS-1 to NS-17, INL-6, INL-7, LSP-4, ODR rules | Must stay apart from modules, which are out of scope. Must say what happens to a C name (LSP-4 says a C name is known by its spelling alone) |

Totals now: 12 done, 8 ready, 2 partial (P8b, P15), 0 blocked.

P20 and P21 are my own designs, aimed at the unreached pages. General knowledge: several compilers offer a packed attribute as an extension, and several languages have names private to a module. I have not checked either design rule by rule.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | A refresh check for Propose reports | 1.5: 10 rows cite withdrawn rules, 1 class change, 16 ID collisions; reports name no commit | small tool | low | current inputs for item 3; X1 to X12 stay usable |
| 2 | Propose round 4: X13 to X16 | 1.4: 5 unreached pages; 1.3: about a dozen findings per exercise | 4 reader sessions, then 1 repair session | low | first designer use of the five declarations pages |
| 3 | Pair exercises | X9-4 predicted that P14 touches the same block-exit rules as P8; X10 and X12 both amend the termination list (UNW-5) | 2 reader sessions | low | findings where two changes cannot both be stated |
| 4 | Probe of later-version space | 1.6; X10's hand probe; X9 and X12 raise the need again, after eight of eight earlier reports | small tool, 1 matrix run | medium | section 1.1 of every proposal; Q-0007 option 2 |
| 5 | An index of what a test of validity can see | 1.6: 3 of 553 `reject` rules have such a test; X10 4.3 adds the SINIT channel | 2 writer sessions | medium | section 4.3 of every proposal |
| 6 | A corpus manifest and a construct count | 1.6: 65 Write files, no manifest; Q-0183 | 1 session and a small tool | low | Q-0183 option 2 |
| 7 | A rule class for link-time diagnostics | D30, open since round 1; 39 `link-fail` tests | 1 writer session | low | P15 |
| 8 | A front-end impact probe that changes one rule | front end passes 5595 of 6032; every change table's Tests column is filled by hand | large | medium | measured impact sections |

### Item 1. A refresh check for Propose reports

**What it is.** A script reads the change table of each report under `reports/clean-room/X*/` and checks each row against the specification. A row is flagged when:
- its rule is withdrawn (the flag names the rule that the merge note gives),
- its old class differs from today's class,
- its "new" ID is a real rule and does not use the `X<n>:new-<k>` form.

The check runs as a warning in `tools/check_specification.py` or as its own tool. Propose reports also name the commit they read, so the check can report how far each report has drifted.

**Evidence.** Section 1.5. My scratch script does this in about 30 lines.

**Cost.** Small.

**Risk.** Low. The check must not fail a commit, because a report is a record of its date.

**What it unblocks.** The pair exercises (item 3) read old reports and need to know which rows still hold.

### Item 2. Propose round 4: X13 to X16

**What it is.** Four Propose exercises chosen to reach the five unreached pages:

| Exercise | Proposal | Unreached pages it reaches | Changes existing behavior |
|---|---|---|---|
| X13 | P6, named arguments | default-arguments | no |
| X14 | P20, packed classes | alignment | no |
| X15 | P21, names private to a namespace | namespaces, inline, linkage-specifications | no |
| X16 | P4, chained comparisons | none (comparison, evaluation-order) | yes |

X16 is the one that changes the meaning of valid programs. P4 has been ready since round 1, and its impact section exercises item 5's question.

**Evidence.** Section 1.4. X9 to X12 reached seven pages for the first time and found 49 gaps.

**Cost.** One reader session each, then one writer session for repairs.

**Risk.** Low for similarity. X15 risks drifting toward modules, so its brief should say that modules stay out (decision 0051).

**What it unblocks.** Every page then has a designer's reading. P6 tests FN-16, the newest rule from the designer batch.

### Item 3. Pair exercises

**What it is.** An exercise that takes two done proposals that touch the same rules and writes only the interaction section, with a change table for the combined rules. Two pairs:
- **X9 and X11** (relocation and scope-exit blocks). Both amend OBJ-3 and the block-exit rules (X9-4). Can a scope-exit block name a relocated variable? Which runs first at the end of the block?
- **X10 and X12** (exceptions in constant evaluation and exhaustive `switch`). Both add or move termination points (UNW-5, the former CE-31). Does a final enumeration's out-of-range `switch` end a constant evaluation the same way as an uncaught exception?

**Evidence.** Round 3 process change 3, not taken. X9-4 named the X11 overlap before X11 ran.

**Cost.** One session each.

**Risk.** Low. Both pairs use rules that phase 6 just repaired, so findings are likely to be few and small.

**What it unblocks.** A way to state how two changes compose, which no exercise tests today.

### Item 4. Probe of later-version space

**What it is.** Unchanged from round 3 item 4. Compile every test that expects `reject` (2564) under the newest `-std` mode of each baseline compiler. Publish the list of tests whose outcome flips, as an index with no rules and no `:since:` field.

**Evidence.** Section 1.6. X10 shows that the hand probe works and that it finds real overlap. The status page lists the decision as waiting on the maintainer.

**Cost.** Small: a runner option and one matrix run, then triage.

**Risk.** Medium. Triage is the effort. A reader who triages may be drawn to recall later-version material. Q-0116 keeps the source-reader designer as the channel for any explanation.

**What it unblocks.** Every proposal can fill its version line from an index. Q-0181 and Q-0188, which ask whether Tidal follows a change the newest compilers apply, gain data.

### Item 5. An index of what a test of validity can see

**What it is.** Round 3 item 3, narrowed. For each `reject` rule, one value:
- **visible:** the error can occur where substitution (DED-24) or a requires expression (CPT-26) sees it,
- **fatal:** the error is always fatal (DED-29, or outside any template),
- **syntax:** only the grammar rejects the form (TR-12).

Add a note per rule for the constant-initialization channel (SINIT-1, SINIT-4) where the rule concerns constant evaluation. Add a test for a sample of the "visible" rules that observes the rejection from inside a requires expression. Drop the position column that round 3 proposed. The reports did not use it.

**Evidence.** Section 1.6. X10 4.3 and X12 4.3 each worked out visibility by hand.

**Cost.** Two writer sessions.

**Risk.** Medium. Some rules are visible in one position and fatal in another, so a rule may need two values.

**What it unblocks.** Section 4.3 of every proposal. A check on item 8's results.

### Item 6. A corpus manifest and a construct count

**What it is.** A manifest that lists the task programs among the Write files (not the probes), with their suite and task. The front end then counts named constructs over the listed programs.

**Evidence.** Section 1.6. Q-0183 option 2 waits for "a corpus of whole programs".

**Cost.** One session for the manifest and a small counting mode.

**Risk.** Low. About 60 programs written by readers give only rough frequency. Reports must say so.

**What it unblocks.** Q-0183 option 2. A base for item 8 that is not made of fragments.

### Item 7. A rule class for link-time diagnostics

**What it is.** Unchanged from round 3 item 7. A row in the TR-P1 table, or a stated policy, for a program that a correct implementation rejects only at link time, and a pass over the rules that the `link-fail` tests cite.

**Evidence.** D30, open since round 1. 39 tests expect `link-fail`.

**Cost.** One writer session.

**Risk.** Low.

**What it unblocks.** P15, the last proposal held by a specification gap.

### Item 8. A front-end impact probe that changes one rule

**What it is.** Unchanged from round 3 item 6. A front-end mode changes one rule's class, runs the suite and the corpus, and lists the tests whose outcome moves. The mode reports only over tests that the front end passes today.

**Evidence.** The front end passes 5595 of 6032 tests, up from 4439 of 5408 at round 3. Report-14 lists 205 missed rejects among the baseline tests.

**Cost.** Large.

**Risk.** Medium. Outcomes for the tests that the front end fails stay unknown.

**What it unblocks.** Measured Tests columns in change tables.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them. X15 must not grow into modules.
- **Relocation by bytes (P8b)** until Q-0322 changes. The byte copy that would create the object stays referenced.
- **Rules or `:since:` fields for later versions.** Item 4 is an index only. Q-0185 stays deferred.
- **A source-reader designer** (Q-0116). The clean-room designer has not needed one.
- **Lexical structure beyond PRIM-26.** P1 can be written against PRIM-26 alone.

## 4. Process changes

1. **Propose reports name the commit they read,** in the header paragraph, as roadmap reports do.
2. **The refresh check runs before any exercise reads an older Propose report** (item 1).
3. **A consolidation that withdraws a rule lists the Propose reports that cite it** in its commit message. Commit aae6c9e withdrew CE-31 on the day three Propose reports cited it.
4. **Rate partial repairs in the exercise status.** `documentation/exercises.md` section 8 still gives finding counts only. A count of findings closed, sent elsewhere, sent to a question and needing no action shows the two standing needs at once. Round 3 asked for this, and it was not taken.
5. **Probes flush before they abort.** X11-3 drew a wrong conclusion from output lost in a buffer. The Propose prompt should say so.
6. **Keep the designer's view in round 5,** in this form, with the counts of sections 1.1, 1.4 and 1.5 rerun.

# Roadmap, round 15: what to do after phase 17, from an implementer's view

Date: 2026-11-11. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone and uses the specification as the design input. The evidence is the two front ends (`frontend/src/` and `frontend/next/`), their reports, ledger, baseline, marks file, and takeover list in `reports/clean-room/rust-frontend/`, the design of the rewrite (`frontend/DESIGN-rewrite.md`), and the phase 17 exercise and review records. Commit read: `7a849708`.

Inputs:
- AGENTS.md, CLAUDE.md
- `reports/clean-room/roadmap-14/implementer.md`, the model for this report's form
- `documentation/`: status, roadmap (the phase 17 table), decisions 0060, 0064, and 0065, and `decisions/pending.md`
- `git log fb0bcf87..HEAD` with every commit body (19 commits)
- `frontend/DESIGN-current.md`, `frontend/DESIGN-rewrite.md`, `frontend/TEMPLATE-MODEL.md`, and `frontend/README.md` (the rewrite part)
- `reports/clean-room/rust-frontend/`: `report-rw2.md`, `report-28.md`, and `report-rw3.md` in full, `rewrite-marks.tsv`, `takeover.tsv`, `ledger.md`, and `conformance.tsv`
- `reports/clean-room/X26/report.md` (sections 3 to 5 and the host-only claims)
- `reports/clean-room/ledger.md` (the 13 open rows)
- `reports/review/2026-11-11-gap-review-phase-17.md` (the headings and the Result) and `2026-11-11-read-of-rv10-repair-diff.md` (the Result and the RV11-R points)
- `frontend/next/src/` (the layout, the line counts, `machine/`, `sem/`, and the lambda check), `tools/frontend_check.py`, `tools/check_all.sh`, and `frontend/src/` by line counts only
- `tests/`: the headers of every test, and the source of the tests that the scripts below sort

I did not read the other round 15 reports, `inputs/`, or `build/exercise-keys/`. I ran no compiler, so this report makes no host-only claim.

**Measurements.** I ran `tools/frontend_check.py --next` once under the run lock, with no `--update`. Its lines match the phase 17 close commit:

```
frontend: 7049/7148 pass, 7069/7148 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 17 new (7 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 70 tests fail their check, of which 38 need the template model (marks file) and 32 count toward the gate of decisions 0062 and 0063; 1/178 program tests do not run to their result; 9 tests out of scope
frontend: 2177/2805 passing rejects name a cited rule (1988) or a linked rule (189) in their first error; `tidal-fe conform` gives the table per page
next: unit: 126 tests pass
next: lex: 7266/7266 files give the old front end's tokens
next: parse: 6828/6987 tests parse as their outcome needs
next: parse: 142 of the 159 disagreements are reject tests that stop on the line of the old front end's first error
next: shadow: step 3 plain: 1602/1897 pass their check, 1600 of the old front end's 1892
next: shadow: step 3: 1795/3832 pass their check, 1792 of the old front end's 3816; 3 pass on the new front end only
next: shadow: all: 1915/7148 pass their check; 4954 unsupported
next: owned: 1327 tests (takeover list)
```

I then ran `tidal-next conform tests -v` and `tidal-fe conform tests -v` once each under the same lock, to have both statuses for every test. The scripts and their output are in `reports/clean-room/roadmap-15/implementer-scratch/`:

- `check-next-output.txt` holds the tool's whole output. `conform-next.txt` and `conform-old.txt` hold the two harness outputs.
- `scopes.py` sorts every test into a scope by the tool's own rules (the page prefixes of `DESIGN-rewrite.md` section 3 and the tool's keyword pattern) and adds two step 4 scopes. It writes `scopes.tsv`. Its old statuses skip the tool's `no-diagnostic` adjustment, so its step 3 plain count of old passes is 1891 where the tool prints 1892.
- `step3_rest.py` sorts the step 3 plain tests that the old front end passes and the new one does not. Its patterns are my judgment. It finds 291 of the tool's 292 and splits them close to report RW3's table.
- `causes70.py` sorts the 70 failing tests of the completion line: the 38 marked ones by model section, and the 32 others into cause groups from report 28 section 8 and the RV10 repair commit (`88d14ea2`), by my judgment.
- `step4_parts.py` splits the step 4 plain scope into three parts that a staged step 4 could take.

## 1. Where the specification and the front end stand

| Measure | Round 14 (7073 tests, `f77bbe05`) | Now (7148 tests, `7a849708`) |
|---|---|---|
| Full pass, old front end | 6967 (98.5%) | 7049 (98.6%) |
| Check pass, old front end | 6990 | 7069 |
| Completion line (decision 0060) | 74 tests (36 marked, 38 unmarked), 0 of 166 program tests, 9 out of scope | 70 tests (38 marked, 32 unmarked), 1 of 178 program tests, 9 out of scope |
| `MISSED-REJECT` in scope | 38 (21 marked, 17 unmarked) | 40 (21 marked, 19 unmarked) |
| `unsupported` in scope | 26 (8 marked, 18 unmarked) | 19 (10 marked, 9 unmarked) |
| `FALSE-REJECT` in scope | 4 (3 marked, 1 unmarked) | 5 (3 marked, 2 unmarked) |
| `EDG-AGREES` (a failure) | 6 (4 marked, 2 unmarked) | 6 (4 marked, 2 unmarked) |
| `run-unsupported` | 23 | 20 |
| Old front end: passing rejects whose first error names a cited or linked rule | 2154 of 2782 (77%) | 2177 of 2805 (78%) |
| New front end: the same share | none | 610 of 997 (61%) |
| Rules | 1700 | 1700 (no new rule) |
| Questions open | 200 | 202 |
| Front-end findings (G) | G1 to G180, 14 open | G1 to G183, 2 open (G182, G183) |
| Exercise ledger rows open | 15 | 13 (X26-1 to X26-6, RV11-R-1 to RV11-R-3, RV11-R-5, RV11-R-7, RV11-R-8, F7-2) |
| Old front-end source | 52954 lines, 28 files; 7 over 2000 lines | 53293 lines, 28 files; 7 over 2000 lines |
| New front-end source | 10197 lines, 208 files | 22512 lines, 302 files; the longest is `expressions/primary/lexical.rs` (857) |
| Tests the new front end owns | 0 | 1327 |

### What phase 17 took from round 14

| Round 14 item | Phase 17 result |
|---|---|
| 1. The grammar gaps G166 to G179 | Done in GR (`1c223631`, `924fd6ce`). Every test of each form parses. The tree side of the parse count went from 216 disagreements to 6. G182 (`union_def` with a template-id) and G183 (LIST-4) are new and open. |
| 2. The rewrite, step 3 | Done in part in RW2 (`5f100f69`). Measures below. |
| 3. Front-end round 28 | Done (`c0461b86`). The machine and constant-evaluation groups closed before step 3, and step 3 then wrote its machine new and copied none of them. |
| 4. Text for the X24 and RV10 rows | Done in F7 (`b511838d`). |
| 5. Gap review RV10 with the read of its repair diff | Done (`18efd5eb`, `88d14ea2`, `7f61141b`). The read opened RV11-R-1 to RV11-R-8. |
| 6. The P22 prototype | Done in RW2. Exactly the two JMP-17 tests change over a base of 1819 tests. X24's 19 prototype tests have no directory yet. |

Item 2's measures against round 14's list:

| Measure of round 14 item 2 | Now |
|---|---|
| The step 3 plain shadow line passes each test the old front end passes, or each difference has a cause | 1600 of 1892. Report RW3 section 2 gives a cause for each of the 292 others. |
| The step 3 shadow line gives a count, sorted by cause | 1795 of 3832. Report RW3 sorts only the plain scope. |
| The 3 plain program tests run to their result | 0 of 3. `r4-hello.cpp` calls `puts`, which has no definition in the unit. `r4-bits.cpp` stops as undefined at PFX-4, because the machine has no byte view of a `float` (STG-26). `r4-lambda-generic.cpp` holds a lambda. |
| `next: unit` passes, with unit tests in each new module | 126 pass. |
| No file over about 2000 lines; no module names a rule of another page | The longest file has 857 lines. 64 rule IDs belong to another page (`DESIGN-rewrite.md` section 10). |
| `next: parse` loses no test | Holds. |
| "0 worse" on every commit | Holds. |
| The takeover list holds each candidate the new front end passes | None of the 10 candidates passes on the new front end. Round 28 fixed 5 of them in the old code. The list holds 1327 other tests. |

Of round 14's process changes:
- Change 1 is not in place. `DESIGN-rewrite.md` section 5 copies the marks file's counts (38, then 17, 15, and 6), and its build table copies them again (38 to 21 to 6 to 0).
- Change 2 is not in place. The parse line still counts the reject disagreements.
- Change 3 holds for GR, whose text and parser came in consecutive commits. The RV10 repairs changed the Syntax segments in `88d14ea2`, and the parser followed only in `5f100f69`. In the three commits between them, `next: unit` failed one test, and `frontend_check.py --next` exited 1.
- Change 4 is not in place. Section 8 has no commit column, and it still calls the machine, `types.rs`, `ast.rs`, `conform.rs`, and `consteval.rs` "moved", where step 3 wrote `machine/` and `sem/` new.
- Change 5 holds in part. The tool prints the shadow lines by scope. No shadow baseline exists.
- Change 6 holds through decision 0065.
- Change 7 holds for RW2 and RW3. Report 28 did not run `--next`, and its commit message gives the `next:` lines.
- Change 8 holds in part. An owned reject test must name a cited or linked rule first. No target covers the other reject tests.

### The 70 tests of the completion line

| Measure | Round 25 | Round 26 | Round 27 | Round 28 | Now |
|---|---|---|---|---|---|
| In-scope tests that fail their check | 104 | 77 | 74 | 64 | 70 |
| Of these, marked for the template model | 30 | 32 | 36 | 38 | 38 |
| Of these, unmarked | 74 | 45 | 38 | 26 | 32 |

The 6 tests added since round 28 come from the RV10 repairs (`88d14ea2`). The old front end has not run a round on them.

The 38 marked tests, by the section of `TEMPLATE-MODEL.md` that the marks file names (`causes70.py`):

| Model section | Step | Tests | Status now |
|---|---|---|---|
| 1, dependence marks | 5 | 4 | 1 missed reject, 1 unsupported, 2 EDG-AGREES |
| 2, the current instantiation | 5 | 6 | 3 false rejects, 2 missed rejects, 1 unsupported |
| 5, checks at the definition | 5 | 7 | 7 missed rejects |
| 3, a specialization | 6 | 15 | 8 missed rejects, 5 unsupported, 2 EDG-AGREES |
| 4, the immediate context | 7 | 6 | 3 missed rejects, 3 unsupported |

The 32 unmarked tests, with the build step whose code writes the cause again and the new front end's status:

| Cause | Tests | Rules | Step | New front end |
|---|---|---|---|---|
| Template checks outside the marks (missed rejects) | 11 | TARG-15, TARG-7, CTAD-19 with CTAD-23, CPT-37, CNS-13, TPL-31, TPL-7, ORD-3, TSP-23; and from RV10, TPL-4 and ENUM-20 | 5 to 7 | unsupported |
| Template constructs outside the marks (unsupported) | 7 | TARG-14, CTAD-1, CPT-1, CPT-20, CNS-30, PACK-3, PACK-5 | 5 to 7 | unsupported |
| The RV10 grammar forms | 4 | ATTR-2 (`maybe_unused` before a using-directive), AUTO-11 (`decltype(auto)` after `~` and as a cast type), NEW-1 (`new_ptr`) | 3 | 3 pass; `new-type-pointer-forms.cpp` is unsupported |
| Lookup and complete-class checks | 3 | UNEV-1, PFX-11, NAM-8 | 3 and 4 | unsupported |
| Lambdas | 2 | CEV-1, LAM-18 | 3 and 4 | unsupported |
| Overload sets as arguments | 2 | ICS-47, OVL-20 | 4 and 5 | unsupported |
| EDG-AGREES in class and constraint code | 2 | CPY-27, CNS-32 | 4 and 5 | unsupported |
| A structured binding with a member `get` template | 1 | SB-5 | 3 | unsupported |
| **total** | 32 | | | |

The new front end passes 3 of the 70. Each of the 3 is a reject test whose first error on the new front end names neither a cited nor a linked rule, so none is owned (report RW3 section 1). 18 of the 32 unmarked tests sit in code that steps 5 to 7 write again. 8 sit in step 3's remaining groups or in step 4.

### Run stops

20 tests pass their check and stop in a run: report 28's 19 and `template-keyword-base-last-name.cpp` from RV10. By `scopes.py`: 3 sit in the step 3 plain scope, 6 in the step 4 plain scope, 1 in step 4 with a template, and 10 need the templates pages. No program test stops. The new front end runs 2 of the 3 plain ones: `reference-constant-initialized.cpp` (SINIT-1) and `integer-pointer-no-object-sanitizer.cpp` (UN-7, STG-15).

### Where the rewrite stands

**The crate.** `frontend/next/` holds the step 1 and 2 core, and from step 3 `sem` (types, the typed tree, entities, scopes, the `Check` context, 1734 lines), `target.rs`, a new `machine/` (1240 lines), `conform/`, 48 `check.rs` parts and 20 `run.rs` parts on the step 3 pages, the non-class parts of the two overloading pages, the node-check and `Ext` node points, and `proposals/p22.rs`. The classes, templates, and exceptions pages have only their `syntax.rs` parts. The type table has no `DepType`, and the typed tree has no dependence marks.

**The tests by scope.** `scopes.py` gives this split. "Step 4 plain" is a test whose rules sit on the step 3 or step 4 pages and whose code uses no keyword of the templates pages and no `std` or `#include`.

| Scope | Tests | Old front end passes the check | New front end passes the check | Both | Owned | Program tests |
|---|---|---|---|---|---|---|
| step 3 plain | 1897 | 1891 | 1602 | 1600 | 1327 | 3 |
| step 4 plain, step 3 rules only (a class, `new`, `throw`, ... in the code) | 1349 | 1346 | 171 | | 0 | |
| step 4 plain, a step 4 rule | 1474 | 1473 | 54 | | 0 | |
| step 3 or 4 rules, a template or `std` in the code | 993 | 976 | 23 | 23 | 0 | 69 |
| any templates page | 1435 | 1377 | 65 | 62 | 0 | 77 |

The two step 4 plain rows hold 2823 tests and 29 program tests. `step4_parts.py` splits them further:

| Part of step 4 | Tests | Old front end passes the check | New front end | Program tests |
|---|---|---|---|---|
| a. Classes: definitions, members, access, special members, derived classes, unions, `new` and `delete`, `this`, lambdas | 1784 | 1781 | 167 | 15 |
| b. Overloading with class operands, operator functions, virtual functions, conversion functions, comparisons | 765 | 764 | 38 | 11 |
| c. Exceptions | 274 | 274 | 20 | 3 |

**The 292 step 3 plain tests that the old front end passes and the new one does not** (report RW3 section 2; `step3_rest.py` gives the same groups within a few tests, with 75 lambdas where the report has 67):

| Group | Tests | What the new front end needs |
|---|---|---|
| A missing check (`MISSED-REJECT`) | 122 | a check in a page that exists. By the first cited rule: ATTR 17, DEFARG 9, FN 8, RET 8, CMP 8, NS 6, AUTO 5, ENUM 5, UNEV 5, and 26 prefixes with 1 to 4 each |
| A missing check where EDG agrees | 12 | the same |
| A lambda expression | 67 | a closure class, which is step 4's |
| `if consteval` | 12 | a statement form and the immediate-function context |
| A `noexcept` specifier with an operand, and the `noexcept` operator | 13 | an unevaluated operand and a function type with its exception specification |
| A structured binding declaration | 13 | the array and tuple-like forms that need no class keyword |
| A braced list bound to a reference to an array | 9 | one initialization form |
| An assumption `[[assume(e)]]` | 6 | an expression tree for the attribute's tokens |
| `long double` arithmetic | 5 | the 80-bit arithmetic of the old `interp/f80.rs` (360 lines) |
| Other unsupported constructs | 32 | sorted in the round report |
| A false reject (named escape, G159, out of scope) | 1 | none |

Of the 1602 tests whose check passes, about 85 do not run to their result. 45 are programs of several translation units, because linking has not moved. The other 40 are run stops, missed stops at undefined behavior, 7 unexpected stops, and 3 wrong results.

**Verdicts on constructs that the new front end does not model.** Decision 0060 keeps `unsupported` for a construct that the rewrite does not implement. Across all 7148 tests the new front end gives 113 false rejects. 2 sit in the step 3 plain scope, 89 in the step 4 plain scope and the step 3 and 4 tests with a template, and 22 in the tests of the templates pages. Some samples from the step 4 plain scope follow.
- A `friend` declaration gives TR-12 ("the declaration declares nothing").
- A member function defined outside its class with a `const` qualifier gives DCL-21.
- A class with a static `operator()` or `operator[]` gives COMMA-6 or PFX-2.
- A bitwise operator on an enumeration with a user-declared operator function gives BIT-3.

The run side gives 8 unexpected stops at undefined behavior and 4 wrong results, among them `r4-bits.cpp`. No owned test is touched, because an owned test must pass. Each is a verdict where `unsupported` is the honest answer, and each is a place where step 4's code will meet a check that already fires.

**Ownership in the tool.** `takeover.tsv` names 1327 tests, each with step 3 and `5f100f69`. The RW2 commit message (`5f100f69`) says that their status now comes from the new front end. The tool gates owned tests in a narrower way.
- `conformance.tsv` still holds the old front end's status for every owned test, and `--update` writes the old status.
- The "0 worse" line and the completion line count the old front end's status on owned tests.
- Only `--next` reads the takeover list and fails on an owned test that the new front end does not pass. `tools/check_all.sh` runs the tool without `--next`.

So the old front end's statuses for owned tests still gate every commit, and the new front end's statuses gate only a run with `--next`.

**The takeover gap in the step 3 plain scope.** 1600 tests pass their check on both front ends, and 1327 are owned. Of the other 273, 85 do not pass in full on the new front end (a run status), 185 are reject tests whose first error on the new front end names another rule, 1 is an accept test that F7 added after the list was chosen, and 2 pass in full on the new front end and stop in a run on the old one. The new front end passes 2 tests that the old one fails, both RV10 reject tests, and their first errors name another rule too.

**Stale records.**
- `DESIGN-rewrite.md` section 5 copies the marks counts, and its build table gives step 4 "none" in the Takes over column, while decision 0065 moves ownership test by test.
- Section 8 calls the machine, `types.rs`, `ast.rs`, `conform.rs`, and `consteval.rs` moved. Step 3 wrote `machine/` and `sem/` new and named no rule in `machine/`.
- Section 10 and `TEMPLATE-MODEL.md` section 8 list RV10-R-1 and RV10-R-3 as open. F7 answered both, and round 28 passes RV10-R-1's test.
- The RV10 repairs changed DEP-30, whose list now takes the jump conditions of JMP-7 and JMP-14. Model section 5's list of definition checks cites DEP-30, and section 8 does not name the change.
- `TEMPLATE-MODEL.md` lines 5 and 10 now give 38, as the round 28 commit says.

### The X26 and RV11 findings

| Rows | Who acts | Why the front end cares |
|---|---|---|
| X26-1 (UNEV-P2, UNEV-2, COMMA-1) | text | The end of a discarded class temporary. The machine ends temporaries at the full expression today. Step 4 writes class temporaries in the machine. |
| X26-2 (LOOP-3, UNEV-2) | text | The value of a `for` iteration expression. A volatile read there is a step 3 run case. |
| X26-3, X26-4, X26-5 (COMMA-5, UNEV-3, STMT-4, PRIM-28) | text, glossary, example | No front-end work, except that X26-3 decides which warnings a recommended rule asks the checks for. |
| X26-6 | none | Outside knowledge. |
| X26 section 3, the prototype hook | design | A P26 prototype needs three things that `DESIGN-rewrite.md` section 7 does not give: whether a node check knows that its node sits in an unevaluated operand, how a node check's result at substitution is classed, and a check on a declaration for the removal of `operator,` (OPR-2). |
| RV11-R-1 (TPL-4) | text, a test | TPL-4's new sentence also rejects a friend declaration with two template heads. RV10's `two-template-heads-in-class.cpp` is a missed reject of the old front end, so the check that round 29 or step 5 writes should follow the answer. |
| RV11-R-7 (TARG-16) | text | `braced_init` in `template_arg` also derives a list for a scalar parameter. The new parser derives it already. |
| RV11-R-2, RV11-R-3, RV11-R-5, RV11-R-8 (CTAD-1, CTAD-14, ATTR-19, OBJ-2, UNI-13, AUTO-11) | text, tests | Small records. RV11-R-8 names a place of AUTO-11 that no test holds, and the new parser takes `decltype_expr` there. |
| F7-2 (Q-0447) | a test | A union row with no test. |
| G182 (`union_def`) | Syntax text, then the parser | `programs/roadmap/r10-variant.cpp` does not parse. The program is a step 5 test, so the gap blocks no step 3 or 4 measure. |
| G183 (LIST-4) | text | Two `r14-fields` programs depend on whether a value of another class with a conversion function initializes an aggregate member whole. Step 4 writes LIST-4 for class values. |

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | The rewrite: finish build step 3, grow the takeover list, and make the new front end's status the status of record for owned tests | 292 plain tests left with a cause each; 273 plain tests pass on both front ends and are not owned; owned tests are gated only under `--next`; 113 false rejects | medium to large | none |
| 2 | The rewrite: build step 4, part a (class types, the alternation of parser and analysis, members, special members, derived classes, unions, `new` and `delete`, closures) | 1784 tests in part a, 1781 old passes; the 67 lambda tests of step 3 need closure classes; step 5 and the 38 marked tests wait for step 4 | large | item 1's tool change; item 1's step 3 groups for the shadow line |
| 3 | Front-end round 29 on the old code: phase 18's new tests and the twin changes, and nothing more | the old front end still judges 5821 tests; 6 RV10 tests have no round; 18 of the 32 unmarked failures sit in code that steps 5 to 7 write again | small | the phase's text items for their tests |
| 4 | Text for the 13 open rows and the two open G rows | X26-1, G182, G183, and RV11-R-1 reach both front ends | small each | none |
| 5 | Gap review RV11 of the phase 18 text, with the read of its repair diff | item 4 changes text and a Syntax production that no source reader has read | medium | item 4 for its text |
| 6 | Records: the design documents to the present, X26's points in the prototype hook, and X24's prototype tests | section 1, "Stale records"; X26 section 3; report RW3 section 4 | small | none |

### 1. The rewrite: finish build step 3, grow the takeover list, and make the new front end's status the status of record for owned tests

**What it is.** The clean-room writer in the implementer role finishes build step 3 of `DESIGN-rewrite.md` section 5 in `frontend/next/`, under decisions 0029, 0035, 0060, 0064, and 0065, and changes `tools/frontend_check.py` so that the takeover list decides which front end's status counts.

**Lambdas move to step 4.** A closure type is a class with a call operator and captures as members (LAM rules). The new front end gives `unsupported` for each lambda and names build step 4 in the message. Step 3 ends without the 67 lambda tests, and item 2 takes them first. The measure below names them apart.

**The order of the remaining groups.** Faults come first, then the groups with no new structure, then the run side, then the new constructs:

| Part | Group | Tests | Why in this place |
|---|---|---|---|
| a | The verdicts on unmodelled constructs: the 8 unexpected stops, the 4 wrong results, and the false rejects of the step 3 and step 4 scopes. A construct that a later step writes gives `unsupported`. `r4-bits.cpp` stops as unsupported until the byte view exists. | 12 run verdicts; 113 false rejects | A wrong verdict hides a fault. Step 4 starts on code whose checks fire only on what they model. |
| b | The missing checks, page by page, with the largest pages first: `declarations/functions` (24), `declarations/attributes` (17), `expressions/comparison` (8), `declarations/namespaces` (6), `expressions/decltype` (6), then the rest | 122 and 12 EDG-AGREES | Each is a check in a page that exists. They are the largest group, and each passing test is a takeover candidate. |
| c | The first error of the 185 passing reject tests that name another rule, together with the 64 rule IDs that name another page. A caller passes the rule of its context, as `copy_init` takes its rule (section 10). | 185 | The same change brings both counts down. Each test then joins the takeover list. |
| d | Linking several translation units, and the calls of functions with no definition in the unit (`puts` in `r4-hello.cpp`). The link part of the old `interp/run.rs` (538 lines) is the input. | 45 programs and part of 22 run stops | The largest run group. A program test runs only with it. |
| e | `long double` from the old `interp/f80.rs`, the byte view of objects (STG-26), and the missed stops at undefined behavior | 5, part of 22, 8 | The old code has each one. |
| f | `if consteval` (12), the `noexcept` operator and specifier (13), structured bindings without a class (13), a braced list bound to a reference to an array (9), assumptions (6), and the 32 other constructs | 85 | New constructs on step 3 pages. |

**The measure that ends step 3.**
- `next: shadow: step 3 plain` passes each of the old front end's 1892 tests, except the lambda tests and the 2 tests of G159. The round report names each other difference with a cause and a G row or a step.
- The 2 plain program tests without a lambda (`r4-hello.cpp`, `r4-bits.cpp`) and the 45 programs of several units in the plain scope run to their result.
- On the full shadow run, the new front end gives no `FALSE-REJECT`, no `RUN-UNEXPECTED-UB`, and no `RUN-WRONG-RESULT` on a test whose construct a later step writes. Each one left has a G row.
- The new front end's first error names a cited or linked rule in each passing reject test of the plain scope, or the round report gives the cause. The share is at least the old front end's on the same tests.
- No module names a rule ID of another page.
- `next: unit` passes, no file passes about 2000 lines, `next: parse` loses no test, and the old front end's line stays at "0 worse".

**How the takeover list grows.** Decision 0065 moves ownership test by test. A test joins when it cites only step 3 pages, passes in full on the new front end, and, for a reject, names a cited or linked rule first. I propose three changes to the way rows join.
- Rows join at the commit of each part, with that commit's hash, so no placeholder cell needs a later commit.
- A test that the new front end passes and the old one fails joins by the same conditions. Decision 0065 does not ask the old front end to pass. `reference-constant-initialized.cpp` (SINIT-1), `integer-pointer-no-object-sanitizer.cpp` (UN-7, STG-15), and `flow-off-end-inner-break-no-warning.cpp` (FN-7) can join now. After part c, `maybe-unused-on-using-directive.cpp` and `decltype-auto-functional-cast.cpp` can join too.
- The scope widens with item 2. A test of the step 4 part a scope joins at the end of item 2 by the same conditions, with step 4 in its row.

With the step 3 plain scope done, the list holds about 1800 tests (1892 old passes, less about 75 lambda tests and the run stops that stay). Item 2 adds part a's tests.

**When the old front end's statuses for owned tests stop mattering.** They stop mattering at the commit that changes `tools/frontend_check.py` in these four ways. The change comes first in the item.
1. Without `--next`, the tool reads the takeover list, runs `tidal-next conform`, and takes the new front end's status for each owned test. "0 worse" compares that status with the baseline.
2. `--update` writes the new front end's status for an owned test into `conformance.tsv`, with a third column naming the front end.
3. The completion line leaves owned tests out and adds "O owned by the new front end".
4. `tools/check_all.sh` keeps calling the tool, which now covers owned tests.

From that commit on, a change to the old front end that makes an owned test fail on the old code changes no result, as `DESIGN-rewrite.md` section 10 already plans. Rounds fix owned tests in the new code only. Until that commit, the old front end's status for an owned test still gates every run of `check_all.sh`. A report-only line can still print the old front end's passes on owned tests, as a canary for a shared fault. A shadow baseline (`conformance-next.tsv`, round 14 change 5) records the new front end's status on tests it does not own, so that a regression shows before a test is owned.

**Evidence.** Section 1, "Where the rewrite stands". Report RW3 gives a cause for each of the 292 tests. The tool's ownership is narrower than the RW2 commit message and decision 0065 describe.

**Size.** Medium to large. Parts b and c are many small checks. Part d moves about 540 lines. Part f is new code on six pages.

**Prerequisites.** None.

**Risk.** Medium. Part a can lower the count of passing tests where a guess happened to be right. Part c touches the error path of every page, and the takeover list catches a regression on an owned test at once.

### 2. The rewrite: build step 4, part a

**What it is.** The first part of build step 4: class types and class entities in `sem`, class layout in `target`, the alternation of parser and analysis that section 2 describes and section 10 defers to the class pages, member lookup, the deferred parts of a class (NAM-6), the special member functions, derived classes without virtual functions, unions, `new` and `delete`, `this`, class objects with constructors and destructors in the machine, and closures. Overloading with class operands, virtual functions, operator functions, conversion functions, comparisons, and exceptions are parts b and c, for phase 19.

**Step 4 starts in phase 18.** The reasons are these.
- The 67 lambda tests of step 3 need closure classes.
- 89 false rejects outside the templates pages show that step 3's checks already meet class constructs, and a class entity in `sem` is the place to report them as unsupported or check them.
- Step 5, and with it the 17 marked tests of model sections 1, 2, and 5, waits for step 4, because nearly every marked test declares a class template.
- Part a has the largest share of the step 4 tests (1784 of 2823), and parts b and c build on its class entities.

**Scope in phase 18.** Part a in full, after item 1's part a and its tool change. The old code for it is `class.rs` (1774 lines), `special.rs` (1097), the class checks in `decl2.rs` and `expr.rs`, the class part of `layout.rs`, and the object parts of `interp/`. Each is rewritten (section 8).

**The measure that ends the part.**
- A shadow line for part a: `next: shadow: step 4a: P/1784 pass their check, Q of the old front end's 1781`. The tool takes the scope from `step4_parts.py`'s rules or from a list beside the takeover list.
- The round report sorts each test that the old front end passes and the new one does not, by part b or c, by a G row, or by a fault.
- The 15 program tests of part a run to their result.
- The lambda tests of step 3 pass, and they join the takeover list with step 4.
- The parser and analysis alternate, and the step 2 oracle's points that analysis answers (report RW2's RW2-2, and DEP-24's member template through an object of known type) parse.
- The other measures of item 1 hold for the new modules.

**Evidence.** Section 1, the tables of the tests by scope and of the parts of step 4.

**Size.** Large. `class.rs` and `special.rs` alone hold 2871 lines, and the class checks of `decl2.rs` and `expr.rs` come on top. The machine gains objects with subobjects, constructors, and destructors.

**Prerequisites.** Item 1's tool change, so that part a's takeover counts. Item 1's part a, so that no class construct gives a guessed verdict when part a starts.

**Risk.** High. The alternation changes the order in which the step 3 pages see a unit. The 1327 owned tests guard the change, and the parse count guards the oracle's answers.

**Alternative.** Step 4 in full in phase 18, measured on the 2823 step 4 plain tests (2819 old passes). This brings step 5 to phase 19. I rank part a first, because phase 17 did most of step 3 and not all of it, and part a alone is about as large as step 3.

### 3. Front-end round 29 on the old code

**What it is.** A round in the form of report 28, with the `next:` lines in its section 1. Decision 0064 keeps the old front end as the front end of record for the tests the takeover list does not name. The groups:
1. **Phase 18's new tests,** as items 4 and 5 add them. Each failing new test is marked, handed to the new front end when it sits in a step that item 1 or 2 finishes, or fixed in the old code where the fix is small.
2. **The 6 RV10 tests.** The 3 that pass on the new front end go to item 1's takeover after part c. `new-type-pointer-forms.cpp` (NEW-1) is a false reject of the old parser. `using-enum-dependent.cpp` (ENUM-20) and `two-template-heads-in-class.cpp` (TPL-4) are small checks. The round takes the last three where the fix is small, and TPL-4's check waits for RV11-R-1's answer.
3. **Twin changes.** A change to `lexer.rs`, `diag.rs`, `main.rs`, or `conform.rs` is made in both crates.

**What the old front end needs in phase 18, under decision 0064.**
- It stays the front end of record for 5821 tests and keeps "0 worse" on them.
- No fix of the 38 marked tests.
- No fix of the 18 template tests outside the marks, unless the fix is small. Steps 5 to 7 write `templ.rs`, `cons.rs`, and `ctad.rs` again.
- No fix for the 8 unmarked tests in step 3's and step 4's groups (CEV-1, LAM-18, SB-5, UNEV-1, PFX-11, NAM-8, ICS-47, OVL-20) or the 2 EDG-AGREES tests (CPY-27, CNS-32). Items 1 and 2 write their code.
- No machine fix before a copy. Step 3 wrote the machine new, so round 28's reason for that order has gone.
- No refactoring.

**Evidence.** Section 1, "The 70 tests of the completion line".

**Size.** Small.

**Prerequisites.** Items 4 and 5 for their tests.

### 4. Text for the 13 open rows and the two open G rows

**What it is.** The clean-room writer acts on each open row, in this order:
1. **G182.** `union_def` takes `[nested_name] (class_name | template_id) ["final"]`, as `class_head` does. The parser function follows in the same item.
2. **G183 (LIST-4).** A sentence on a value of another class whose conversion function, or conversion function template, gives the member's class, with a test where the compilers agree. Both `r14-fields` programs depend on it.
3. **X26-1 and X26-2 (UNEV-P2, UNEV-2, COMMA-1, LOOP-3).** The end of a discarded class temporary, and the iteration expression as a discarded-value expression.
4. **RV11-R-1 (TPL-4).** The friend declaration with two template heads, with a test.
5. **RV11-R-2, RV11-R-3, RV11-R-5, RV11-R-7, RV11-R-8, F7-2.** Small text changes and the tests that RV11-R-8 and F7-2 name.
6. **X26-3, X26-4, X26-5.** One rule for the no-effect warning, a glossary entry, an example.
7. **X26-6.** A check of the library headers in scope for `operator,`, within `documentation/clean-room.md`.

**Evidence.** Section 1, "The X26 and RV11 findings". X26's host-only claims need probes on the matrix before X26-1 and X26-3 get tests.

**Size.** Small each.

**Prerequisites.** None.

### 5. Gap review RV11 of the phase 18 text, with the read of its repair diff

**What it is.** A source-reader gap review of item 4's text, G182's production, and LIST-4's sentence for G183. The item ends with the recorded read of its repair diff (decision 0063).

**Evidence.** No source reader has read item 4's text. The parser follows a Syntax change literally (G182), and LIST-4 decides two program tests on both front ends (G183).

**Size.** Medium.

**Prerequisites.** Item 4 for its text.

### 6. Records

**What it is.**
- `DESIGN-rewrite.md` section 5 names the marks file and gives the step of each model section with no count. Its build table gives step 3's and step 4's takeover by decision 0065, splits step 4 into parts a, b, and c with their scopes, and moves lambdas to step 4.
- Section 8 says which old files step 3 rewrote (the machine, `sem`) and which it moved, with a commit column.
- Section 10 and `TEMPLATE-MODEL.md` section 8 drop RV10-R-1 and RV10-R-3 and name DEP-30's change for model section 5.
- Section 7 gains X26 section 3's three points: the context a node check sees, the class of a node check's result at substitution, and a check on a declaration.
- `reports/clean-room/X24/tests/` holds X24's 19 prototype tests, which the impact diff of P22 can then run.
- The commit that lands item 1's tool change says what the RW2 commit message (`5f100f69`) claimed about owned statuses.

**Size.** Small.

**Prerequisites.** None. The step 4 split follows item 2's scope.

### Dependency sketch

```
1 step 3: a verdicts ─> b checks ─> c first errors ─> d linking ─> e long double, bytes ─> f constructs
  tool change (owned status of record) ──┐       └─> takeover rows at each part
                                         v
2 step 4a: class types, alternation, members, special members, closures ──> lambda tests ──> step 4b, 4c (phase 19)
                                                                                     └──> step 5 (marked tests)
3 round 29 (new tests, RV10 tests) <── 4 text ──> 5 RV11
6 records (any time; the step 4 split with item 2)
```

Items 1, 3, 4, and 6 can start together. Item 2 waits for item 1's tool change and part a. Item 5 waits for item 4's text.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| `<cstdarg>`, named escapes, `#include <...>`, extended floating-point types | the library, a name table, real headers, *(referenced)* | G42 and G158 to G160 hold 9 tests; the step 3 measure leaves out the 2 named-escape tests | Keep the rows |
| ILP32 | in the matrix (EDG ILP32) | `target.rs` has both tables from TYP-1 and TYP-2 | Keep in scope |
| `no-diagnostic` rules | the class needs no diagnostic | The shadow run gives `MISSED-REJECT` where the old harness gives `NO-DIAGNOSTIC-ALLOWED` | Keep the status; the tool applies the same adjustment to the new front end |
| Coroutines, threads, the memory model, modules | outside the language goal | No test or front-end path needs them | Keep out |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | P22 runs only under its flag | Keep out |
| MSVC | not in the matrix | A second ABI would double `target`'s tables | Keep out |
| Prototypes of Propose designs | not part of the rules | P22 is in place; P26 needs step 4b for OPR-2 and step 5 for X26:new-2 and X26:new-3 | P26's prototype after step 5; its hook points in item 6 |
| Implement I8 on instantiation | deferred by decisions 0062 to 0065 | Step 6 writes model section 3 | Keep deferred until step 5 starts |

## 4. Process changes

1. **The takeover list decides the status of record in every run.** Item 1's tool change. Without it, `check_all.sh` guards owned tests through the old front end only.
2. **An unmodelled construct gives `unsupported`.** The shadow line "all" adds the new front end's false rejects, unexpected stops, and wrong results, and a round report gives a G row or a step for each.
3. **A Syntax repair lands with its parser function.** Round 14's change 3 covers GR. The RV10 repairs left `next: unit` failing for three commits. The commit that changes a Syntax segment, or the next commit, adds the function.
4. **Takeover rows carry the commit of the part that took them.** The placeholder cell and its follow-up commit (`6aa5a345`) go.
5. **The design copies no count.** Round 14's change 1 again. Section 5 and the build table name the marks file and the takeover list and give no number.
6. **A first-error target.** The owned reject tests meet it by their condition. For the rest of the plain scope the new front end names a cited or linked rule first in 565 of 750 passing reject tests (75%). The target at the end of step 3 is the old front end's share on the same tests.
7. **Rule IDs stay on their page before step 4.** 64 IDs name another page now. Each step 4 page would add more through the same contexts.
8. **A shadow baseline.** `conformance-next.tsv` records the new front end's status on every test it does not own, and a move to a failing status prints.

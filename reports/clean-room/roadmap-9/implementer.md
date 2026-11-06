# Roadmap, round 9: what to do after phase 11, from an implementer's view

Date: 2026-11-05. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The evidence is the clean-room front end in `frontend/`, its reports and ledger in `reports/clean-room/rust-frontend/`, and the phase 11 exercise records. Commit read: `a15a877d`.

Inputs:
- AGENTS.md, CLAUDE.md
- `documentation/`: status, roadmap (phase 11 table), exercises (the Implement prompt and the done table), decisions 0057 and 0058, and `decisions/pending.md` (the table and the settlements)
- `reports/clean-room/roadmap-8/implementer.md`, the model for this report's form
- `reports/clean-room/rust-frontend/`: `ledger.md`, `conformance.tsv`, `examples.tsv`, report 22 (sections 2 to 5), and the G152 passage of `frontend/README.md`
- `reports/clean-room/ledger.md` (the rows in state `planned` and the counts by state)
- the phase 11 exercise records: I4 (`report.md`, `notes.md`, the reductions), P1 (findings P1-1 and P1-2, the reductions), and X23 (findings X23-3, X23-6, X23-7)
- `reports/review/`: the two 2026-11-05 gap reviews of the text phase 10 wrote (Result sections, and the DED-29, CPT-19, CNS-P2, CNS-14, and Q-0411 sections)
- the list of phase 11 similarity reports
- `specification/`: `expressions/conditional.md` (through I4's filter), DED-P2 and DED-29, OVL-11, CONV-1, CONV-19, ADD-18, LAM-19, and the rule kinds of every page
- `questions/`: status and `**Waits for:**` lines of Q-0373, Q-0378, Q-0401, Q-0406, Q-0411, Q-0417, Q-0418
- `tests/`: the headers of the failing tests, and the two DED-29 tests in full
- `git log 1e20bfa2..a15a877d` with commit bodies (15 commits)

I did not read the other four round 9 reports, `inputs/`, or `build/exercise-keys/`. I did not read `frontend/src/` except by a grep for rule IDs.

**Measurements.** Every count comes from commit `a15a877d`, with a clean tree. I built the front end with `cargo build --release --locked --target-dir` into my session scratch directory, so `frontend/target` was not touched. `tools/frontend_check.py` has no target-directory option and builds into `frontend/target`, so I did not run it. I ran `tidal-fe conform tests -v` once. Its counts match the phase 11 close commit exactly: 6478 of 6711 pass, 6520 pass their check, 11 new tests do not pass. I compared the result with `conformance.tsv` at `1e20bfa2`. The file did not change in phase 11. I also ran `tidal-fe run` on the six I4 reductions and the three P1 reductions. The counting script and the failure list are in `reports/clean-room/roadmap-9/implementer-scratch/`. Other counts come from grep, so they are approximate. Statements from general knowledge say so.

## 1. Where the specification and the front end stand

| Measure | Round 8 (6681 tests, `1e20bfa2`) | Now (6711 tests, `a15a877d`) |
|---|---|---|
| Full pass | 6459: 6174 `pass`, 264 `EDG-ONLY`, 10 `EDG-ALLOWED`, 11 `pass-partial` | 6478: 6189 `pass`, 268 `EDG-ONLY`, 10 `EDG-ALLOWED`, 11 `pass-partial` (6478 of 6711, 96.5%) |
| Check pass | 6499 | 6520 |
| `EDG-AGREES` (a failure) | 2 (CPY-27, CNS-32) | 2, the same |
| False rejects | 4 | 9 |
| Missed rejects | 116 | 118 |
| Unsupported (check) | 60 | 62 |
| Run tests | 1696: 1651 pass (11 partial), 40 unsupported, 5 not run | 1715: 1662 pass (11 partial), 0 wrong, 0 missed or unexpected UB, 42 unsupported, 11 not run because the check fails |
| Interpreter-only expectations | 19, all met | 19, all met |
| Passing rejects whose first error names a cited or linked rule | 2004 of 2609 (77%) | 2005 of 2612 (77%): 1828 cited, 177 linked, 606 another rule, 1 no rule |
| `reject` rules absent from the front end's source | 103 of 558 | 103 of 558 |
| `unspecified` rules named in the front end's source | 13 of 32 | 13 of 32 |
| Page examples (`examples.tsv`) | 175 of 177 pass; 2 unsupported | unchanged; I did not run `check_examples.py` |
| Rules by kind | 1697 | 1697, the same split |
| Questions | 411: 171 provisional, 13 compiler defect, 227 settled | 418: 174 provisional, 13 compiler defect, 231 settled |
| Front-end findings (G) | G1 to G152, 4 open | G1 to G152, 0 open: 146 commit, 4 text, 1 question, 1 out of scope |
| Exercise findings (`ledger.md`) | no ledger | 867 rows: 762 commit, 68 open, 26 planned, 5 text, 3 question, 3 out of scope |
| Front-end source (Rust lines, approximate) | 50160 | 50160; `frontend/` did not change in phase 11 |

The status page still gives the round 22 baseline (6459 of 6681, and 1696 run tests). The close commit gives 6478 of 6711. The two counts differ because no round updated `conformance.tsv` in phase 11.

### What phase 11 took from round 8

| Round 8 item | Phase 11 result |
|---|---|
| 1. One ledger for every finding; G152, G4, G13, G25, R10 | Done. Item L (`5ab0686b`) gave every exercise finding a row and a check. Item F (`47f58739`) answered G152 in DED-29, moved G4, G13, and G25 to `text`, and acted on R10-1 to R10-3, R10-5, R10-7, and R10-9. R10-4 and R10-6 stay `planned`. |
| 2. Implement I4 (COND-16) | Done (`5523f0e8`). 85 of 85 assertions, 16 unstated steps in `notes.md`, findings I4-1 to I4-14. The writer acted on I4-1, I4-2, and I4-3 (`ee6085aa`). |
| 3. Implement I5 (deduction and the substitution boundary) | Not taken. |
| 4. Front-end round 23 | Not done. Moved to the next phase by the close commit. |
| 5. Rule boundaries on the lowest pages | Deferred by decision 0058. The rate stays at 77% for a third phase. |
| 6. Library surface and `va_list` | Deferred by decision 0058. The two PFX-22 false rejects date from round 6. |
| 7. Labels for the `unspecified` rules | Deferred by decision 0058. |
| 8. A front-end prototype of P23 | Deferred by decision 0058 ("a front-end prototype of a done proposal"). |

Of round 8's process changes, phase 11 took the ledger for every finding and one Implement exercise. Process change 4 asked that a phase take or drop each item deferred twice. Decision 0058 deferred items 5, 6, and 7 a third time, in a list with no reasons. Process change 5 asked for one definition of the run counts. The status page and the harness still give the run counts from two sources.

### The `planned` rows whose item ended

The close commit lists 20 `planned` rows whose item ended without an answer: I3-1, I3-5, I3-13, R10-4, R10-6, P1-3, P1-5, P1-7, P1-8, P1-9, I4-8 to I4-14, X23-3, X23-6, and X23-7. None of them has an answer at `a15a877d`. Each Record cell still names an item that ended (item RV4 or "item E repairs"). Six more rows wait for front-end round 23: P1-1, P1-2, and I4-4 to I4-7. The ledger has no rule for a `planned` row whose item ended, so these 20 rows now read the same as rows with a live plan.

### The failures that front-end round 23 takes

The front end fails 233 tests. 222 of them are the failures of round 8, each with the same status. Phase 11 fixed none, because no round ran. The other 11 are phase 11 tests:

| Group | Tests | Status on the front end | Cause |
|---|---|---|---|
| COND-16 base row and step 1 (I4-1, I4-2) | 2 | `FALSE-REJECT` | The front end still applies the old base row ("the base class with the cv-qualifiers of X") and rejects with COND-5, "Base and const Base". |
| COND-11 for cv-differing class lvalues (I4-3) | 1 | `run-unsupported` | "class conditional" |
| PACK-13 and PACK-14: packs in initializer lists and template-id base packs | 4 | 2 `FALSE-REJECT`, 2 `unsupported` | A parse error at `...` in a member or base initializer; PACK-8 for a pack in a parenthesized initializer; CTOR-15 "initialized two times" for `leaf<I>...` |
| `programs/roadmap/r8-tasks.cpp` | 1 | `FALSE-REJECT` | the CTOR-15 fault above |
| ILC-1 braced initialization through a constructor template | 1 | `run-unsupported` | "constructor call" |
| NEW-29 deleted or private aligned form (X23-2, Q-0417) | 2 | `MISSED-REJECT` | The front end falls back to the unaligned form, as GCC does. |

Six reductions from phase 11 stop the front end and have no test:

| Finding | Reduction | Front end now |
|---|---|---|
| I4-4 | `I4/reductions/r1-conv-to-int-ref-lvalue.cpp` | rejects with ASN-2 "assignment to an rvalue"; the pages give an `int` lvalue |
| I4-5 | `I4/reductions/r3-explicit-conversion.cpp` | "unsupported: class conditional"; the pages reject |
| I4-6 | `I4/reductions/r4-ambiguous-base.cpp` | runs, result 0; the pages reject (CONV-17) |
| I4-7 | `I4/reductions/r5-private-base.cpp` | runs, result 0; the pages reject (ACC-12) |
| P1-1 | `P1/reductions/shapes-unknown-bound-converting.cpp` | "unsupported: array of unknown bound" |
| P1-2 | `P1/reductions/parse-class-converts-to-const-ref*.cpp` (2 files) | rejects with OVL-35; the pages bind a `const R&` through the converting constructor |

P1-2 matters most of these. A `const` reference parameter that takes an argument through a converting constructor is a common form (general knowledge), and the front end rejects it.

### Failures by page

The failure rate per test directory (every status other than a full pass):

| Directory | Failing | Rate | Round 8 | Largest causes |
|---|---|---|---|---|
| templates/packs | 12 of 73 | 16.4% | 11.6% | the 4 new PACK-13 and PACK-14 tests; PACK-8 (3 missed) |
| expressions/conditional | 8 of 72 | 11.1% | 7.4% | the 3 new COND tests; COND-8, COND-16, COND-11 (3 missed) |
| classes/conversion-functions | 7 of 63 | 11.1% | 11.1% | 3 run stops at an overload set |
| expressions/new-delete | 14 of 132 | 10.6% | 9.4% | `new` in a constant (5) and in a run (5); NEW-29 and NEW-30 (4 missed) |
| overloading/conversion-sequences | 12 of 130 | 9.2% | 9.2% | ICS-45 inside a requires-expression (5) |
| classes/access | 6 of 72 | 8.3% | 8.5% | ACC-3, ACC-15, ACC-18, ACC-19 (all missed) |
| templates/dependent-names | 10 of 122 | 8.2% | 8.2% | DEP-37 (2) |
| foundations/language-support | 4 of 49 | 8.2% | 8.2% | |
| templates/instantiation | 11 of 140 | 7.9% | 7.9% | 9 missed rejects on 8 INST rules |
| classes/comparisons | 6 of 82 | 7.3% | 7.3% | DCMP-12 and DCMP-13 (3 missed) |

By chapter, overloading has 18 of 299 (6.0%) and templates 74 of 1197 (6.2%). Templates hold 43 of the 118 missed rejects:

| Page | Missed | Rules |
|---|---|---|
| instantiation | 9 | INST-6, INST-11, INST-12 with INST-6 (2), INST-15 with TR-13, INST-21, INST-22, INST-29, INST-33 |
| packs | 5 | PACK-8 (3), PACK-2, PACK-6 with PACK-26 |
| dependent-names | 5 | DEP-37 (2), DEP-30, DEP-31, DEP-33 |
| constraints | 5 | CNS-29 (2), CNS-13, CNS-24 with CNS-27, CNS-26 with CNS-27 |
| arguments | 4 | TARG-24 (2), TARG-7, TARG-15 |
| class-template-deduction | 4 | CTAD-2 (2), CTAD-12, CTAD-19 with CTAD-23 |
| declarations, specialization | 3 each | TPL-7, TPL-16, TPL-31; TSP-17, TSP-23, TSP-31 |
| deduction, concepts | 2 each | DED-29 (2); CPT-15, CPT-37 |
| ordering | 1 | ORD-3 with ORD-15 |

The check-side unsupported causes did not change: the `same` idiom with variable templates (16), a concept in `static_assert` (8), `new` in a constant (6), the `noexcept` operator naming a later member (4), the BF-9 layout of classes with bit-fields (4), conversion functions in a constant (4), and pointer arithmetic in a constant (4).

### G152 has an answer, and the front end still misses its two tests

Item F answered G152 in DED-29. Forming a candidate substitutes into the function type only, and a candidate's body is instantiated only when the call selects it. The front end already behaves that way (`frontend/README.md`). DED-29's list still says that an error in the body of a lambda in the function type, or in a template parameter declaration, is a hard error. `substitution-lambda-body.cpp` and `substitution-lambda-default.cpp` test that list item, and the front end misses both. These two tests are now front-end work only.

Two points in DED-29 still stop an implementer:

1. The second list item says that substitution instantiates a body "for example to find its deduced result type". The new paragraph says that a candidate's body is instantiated only when the call selects it, "also when its result type is deduced". An implementer cannot tell from the text when a deduced result type is needed before the selection. One case is a call inside `decltype` or `sizeof` that names a candidate with a deduced result type.
2. The 2026-11-05 expressions and templates review reads DED-29 with a doubt. When the address of an overloaded name is taken with a target type, both candidates have their bodies instantiated, because their result types must match the target. DED-29 says "only when the call selects it". Commit `7cf4574b` lists this case as not acted on. DED-30 and OVL-18 cover the address case, and neither says when the body is instantiated.

### The steps of the conditional operator that the text still does not give

The writer acted on I4-1 to I4-3. I4-4 to I4-7 are front-end faults. I4-8 to I4-14 ask for text, and none of them changed the page. I read `conditional.md`, OVL-11, and CONV-1 at `a15a877d` against each:

| Finding | Step the text does not give | State of the text now |
|---|---|---|
| I4-8 | the type condition of COND-3 rows 3 and 4 ("Two xvalues", "An lvalue and an xvalue"); taken literally, row 4 matches an `int` lvalue and a `long` xvalue | unchanged |
| I4-9 | the copy or move when a class glvalue becomes a prvalue: COND-8 names only an lvalue and a prvalue; the xvalue case and the step 3 copies (`Box` to `const Box`, `Derived` to `Base`) have no rule | unchanged |
| I4-10 | the value category after a built-in candidate wins: OVL-11 sends the operands back to the operator's own rules, which give an lvalue for two `int&` results; COND-11 says a prvalue | unchanged |
| I4-11 | which `(T, T)` pointer candidates COND-11 tries, and how `int*` and `const int*` meet | unchanged |
| I4-12 | that steps 1 and 2 are a copy context, so `explicit` conversion functions take no part | unchanged; the reader also used outside knowledge for the character-type promotions, as I3's reader did (I3-12) |
| I4-13 | that "binds directly" in steps 1 and 2 excludes every prvalue result, also of a non-class conversion function; and that step 3's "Otherwise" means "if step 1 or 2 does not bind" | unchanged |
| I4-14 | how COND-11 picks one conversion function per operand and compares two built-in candidates (ICS-7, ICS-25, ICS-37, CVF-4 on two pages) | unchanged |

Three steps from `notes.md` have no finding of their own. Step 1 says nothing on access, while REF-1 asks for an accessible base (notes item 5; I4-7 proposes one sentence). The page does not say which expressions are null pointer constants for rows 8 and 9 (notes item 13). When both operands convert and the step 3 sequence is also ambiguous, the page names two rules and does not say which one a diagnostic cites (notes item 14).

The front end's state on the conditional operator: 8 of 72 tests fail, I4-4 to I4-7 add four faults, and every run test that cites COND-16 fails. I4's frozen component (`reports/clean-room/I4/conditional.py`) scores 85 of 85 and has a type model of the same rules. A front-end round can use it as a second reading of the pages.

### The rules the interpreter still cannot run

701 rules have at least one run test outside `tests/programs/`. For 14 of them, no run test passes on the front end:

| Rule | Kind | Why |
|---|---|---|
| COND-16 | required | the two new tests: a false reject and a "class conditional" stop |
| PACK-13, PACK-14 | required | the new initializer tests: parse errors and false rejects |
| PFX-22 | required | `va_list` from `<cstdarg>`, since round 6 |
| IF-12 | required | a jump into a nested statement (2 tests) |
| JMP-16 | required | a variable of an enclosing function |
| SB-12 | required | a structured binding or init capture |
| OVL-42 | required | class assignment |
| ORD-9 | required | a `new` expression |
| DEP-20, DEP-25 | required | a dependent name in a run |
| TARG-12 | required | a call of an unknown callee |
| TSP-8 | reject | a missing argument |
| LAM-19 | undefined | a `new` expression |

Of the 57 `undefined` rules, 55 have a `run-fail` or interpreter-only test that passes. LAM-19's one test stops at `new`. ADD-18 has no run test, and its constant-evaluation tests on ILP32 are unsupported ("constant initializer: pointer arithmetic").

The 42 run stops by construct: `new` forms (7), constructor calls (5, one of them new), jumps into a nested statement (3), overload sets (3), variables of an enclosing function (3), variables with no definition in the unit (3), and 18 single causes.

X23-6 names one run case that the text does not decide. Implicit objects created in storage whose alignment is too weak for the type have no rule. An interpreter cannot tell whether to stop at the creation (STG-18) or at the first access (STG-23).

## 2. Ranked items

Item FE comes first by the close commit and decision 0058. The other items go with it.

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Front-end round 23 (item FE), in four groups | 233 failures, 222 unchanged since round 8; 11 phase 11 tests; 6 reductions; 0 open G rows | large | 2 for the reductions |
| 2 | Tests for the six front-end reductions | I4-4 to I4-7, P1-1, P1-2 have no test, so no baseline row can hold them | small | none |
| 3 | Answers for I4-8 to I4-14 and the other planned rows whose item ended | 20 rows with an ended item; 7 on one page; I3-12 and I4-12 point at the same gap | small | none |
| 4 | Implement I5: deduction from a call and the substitution boundary, blind | DED-29's two points; the RV4 doubt on the address case; 43 template missed rejects, 11 of them on deduction and instantiation | medium | none |
| 5 | Take or drop the three items deferred three times | rule boundaries (77% for three phases), the library surface (2 false rejects since round 6), labels for the `unspecified` rules (19 of 32 not named) | small each | none |
| 6 | The rules the interpreter cannot run | 14 rules with no passing run test; ADD-18 with none; X23-6 | small (text), medium (interpreter) | 1 |
| 7 | A front-end prototype of P23 | deferred by decisions 0057 and 0058 | medium | 1 for the constraint groups |

### 1. Front-end round 23 (item FE), in four groups

**What it is.** A front-end round in the form of report 22. The groups, in order:

1. **The phase 11 tests (11) and the six reductions.** The COND-16 base row and step 1 (2), COND-11 for cv-differing class lvalues (1), PACK-13 and PACK-14 in initializer lists (4 and `r8-tasks.cpp`), ILC-1 through a constructor template (1), and NEW-29 for a deleted or private aligned form (2). Then I4-4 to I4-7, P1-1, and P1-2, once item 2 gives them tests. P1-2 first, because it is a common form.
2. **The conditional operator as one component.** The 5 older conditional failures (COND-8 with a private copy, COND-16 both ways, COND-11 with several conversions, and 2 `same` idiom tests) with group 1's conditional tests. The round can compare the front end with I4's component on the same inputs.
3. **The template missed rejects (43).** DED-29 (2) first, since the text now decides them. Then PACK-8 (3), DEP-37 (2), CTAD-2 (2), CNS-29 (2), TARG-24 (2), the 9 instantiation tests, and the single rules in the table of section 1. The CLS-2 test (Q-0406) stays with them.
4. **The backlog groups of round 8.** The `same` idiom (16), concepts in `static_assert` (8), `new` in a constant (6) and in a run (7), constructor calls in a run (4 old), the `noexcept` operator naming a later member (4), BF-9 (4), conversion functions in a constant (4), pointer arithmetic in a constant (4), and the older missed-reject groups: ACC-3, ACC-15, ACC-18, ACC-19 (6), DEFARG-13 (4), LIST-13 and LIST-16 (4), DCMP-11 to DCMP-13 (5), NEW-30 (2), MPTR-11 (3), and OVL-9 and OVL-20 (4).

**Evidence.** Section 1. The round 22 rate was 36 old failures per round. At that rate the 222 old failures need about six rounds. Every G row has an answer, so the round starts with no text question open on the front end's side.

**Size.** Large. Groups 1 and 2 are small to medium. Group 3 is medium. Group 4 is the rest.

**Prerequisites.** Item 2 for the reductions in group 1. Item 4 should report before group 3 changes the deduction and instantiation code, or group 3 should take the DED-29 tests only.

**Risk.** Low. Each group that the front end cannot close from the text becomes a G finding with a ledger row.

### 2. Tests for the six front-end reductions

**What it is.** The clean-room writer turns the I4-4 to I4-7 and P1-1 and P1-2 reductions into tests and runs them on the matrix. I4-4 asks for a conversion to `int&` against an `int` lvalue. I4-5, I4-6, and I4-7 are reject tests (COND-5, CONV-17, ACC-12). P1-1 and P1-2 are run tests (DCL-5 and DCL-19, REF-2 and OVL-35).

**Evidence.** Section 1. The ledger holds these six rows as `planned` for item FE, and `tools/frontend_check.py` reports only tests. Without tests, a round can fix them and no baseline row records the fix.

**Size.** Small. Each reduction has one to four lines.

**Prerequisites.** None.

### 3. Answers for I4-8 to I4-14 and the other planned rows whose item ended

**What it is.**
- The writer acts on I4-8 to I4-14, or records why not. Section 1 gives the step each one asks for. The three `notes.md` steps without a finding (access in step 1, null pointer constants, the rule a diagnostic cites) go with them.
- For the character-type promotions that I3 and I4 both took from outside knowledge, CONV-1 or a matrix table gives each character type's promotion and the widths that decide signed against unsigned. The ledger has I3-12 as `text`, and a second reader still missed it, so the text is hard to find.
- The writer answers I3-1, I3-5, I3-13, R10-4, and R10-6 with the probes they wait for, and acts on P1-3, P1-5, P1-7 to P1-9, X23-3, X23-6, and X23-7.

**Evidence.** Section 1. 20 rows name an item that ended. Seven are on one page, and the front end's conditional work in item 1 reads that page.

**Size.** Small. Most findings ask for one sentence, a row condition, or an example.

**Prerequisites.** None. Item 1's group 2 goes better after I4-8 to I4-14.

### 4. Implement I5: deduction from a call and the substitution boundary, blind

**What it is.**
- A fresh clean-room reader builds a component from the deduction page (DED-P1, DED-P2, DED-P3 and their rules), DED-24 to DED-30, and the instantiation rules DED-29 relies on (INST-6, INST-7). Evidence and `:tests:` lines are filtered out, with a filter that the reader checks as I4's reader did.
- The component takes a function template's parameter types and a call's argument types and categories, or an address with a target type. It returns the deduced arguments, a deduction failure, a substitution failure, or a hard error, and it names the step that decided. It also says which bodies it instantiated.
- The scoring phase uses the deduction tests (123 files) and the grids in `tests/templates/deduction/grids/`. Constraints and partial ordering stay callbacks, as in I3.
- The prompt names the two DED-29 points of section 1 and asks the reader to log them if the text does not decide them.

**Evidence.**
- DED-29's second list item and its new paragraph, and the RV4 doubt on the address case (section 1). Commit `7cf4574b` left the address case for the next review.
- Templates hold 43 of the 118 missed rejects. Deduction and instantiation hold 11 of them, and 16 failures in all.
- I1 to I4 each found more than a dozen unstated steps. I4 found three wrong results that its 85 passing assertions did not show, through 13 programs. No blind build has read a template page other than ordering.

**Size.** Medium. The type model needs templates, packs, and dependent types. I2's model covers part of this.

**Prerequisites.** None. Round 8 ranked G152 as a prerequisite, and item F answered it.

**Alternative.** Constraint normalization and subsumption (CNS-P1, CPT-19). Q-0411 waits on a review with a doubt, and the review says the C++26 text may give EDG's answer. Templates/constraints and concepts hold 13 failures. I rank deduction higher because DED-29's text changed in phase 11 and no reader has built from it.

### 5. Take or drop the three items deferred three times

**What it is.** A phase 12 decision adopts each of these items or drops it with a reason:
- **Rule boundaries on the lowest pages.** The lowest pages by first error are foundations/values (0 of 9), expressions/decltype (2 of 12), foundations/conversions (7 of 21), expressions/member-pointers (11 of 29), foundations/names (47 of 95), and templates/instantiation (36 of 71). All are unchanged from round 8.
- **The library surface and `va_list`.** The seven C functions are listed only in `frontend/README.md`. The two PFX-22 tests stop at `va_list`.
- **Labels for the `unspecified` rules.** 19 of 32 are not named in the front end's source: ADD-13, ASN-11, CE-27, CPY-12, INST-11, JMP-11, LAM-20, LAM-28, MUL-13, MUL-17, NEW-14, NEW-18, NEW-34, OPR-12, SINIT-4, STG-24, TR-16, VC-15, VC-7. P1-9 asks for a word on TR-16 and exact results.

**Evidence.** Rounds 6, 7, and 8 ranked these items. Decisions 0056, 0057, and 0058 deferred them in a list.

**Size.** Small each, if adopted. A drop costs one sentence.

**Prerequisites.** None.

### 6. The rules the interpreter cannot run

**What it is.**
- The writer adds a run test for ADD-18, or records that no test on the matrix targets can reach the case.
- The writer answers X23-6 in STG-8, so that an interpreter knows where a misaligned implicit object stops a run.
- Item 1 covers the run stops by construct. The jump into a nested statement (IF-12, SW-1) and variables of an enclosing function (JMP-16, SW-5, UNI-7) are the groups where one fix frees whole rules.

**Evidence.** Section 1. 14 rules have no passing run test. 55 of 57 `undefined` rules have one.

**Size.** Small for the text. Medium for the interpreter work, which item 1 holds.

**Prerequisites.** Item 1 for the interpreter work.

### 7. A front-end prototype of P23

Round 8 item 8, unchanged. Decisions 0057 and 0058 deferred it. I rank it last, because item 1 has the constraint groups to close first, and P23 is not part of Tidal's rules. Item 5's rule applies here too: phase 12 should adopt it or drop it.

### Dependency sketch

```
2 tests for reductions ──> 1 FE group 1 (I4-4 to I4-7, P1-1, P1-2)
3 I4-8 to I4-14 ─────────> 1 FE group 2 (conditional)
4 Implement I5 ──────────> 1 FE group 3 (deduction and instantiation)
1 FE groups 3 and 4 ─────> 6 interpreter work, 7 P23 prototype
5 take or drop (independent, small)
```

Items 2, 3, and 5 are small and can start with item 1. Item 4 reads different pages from item 3 and can run beside it.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| C library functions in tests | the general library | The interpreter knows seven by a front-end choice. A second implementer has to guess. | Keep the library out. State the seven as a test convention, or drop the item with a reason (item 5). |
| `<cstdarg>` and `va_list` | the general library, and no standard headers | Two PFX-22 tests need them since round 6. | Keep out. Rewrite the tests, or list the shape under decision 0041. |
| Extended floating-point types | *(referenced)* only | Some `same` idiom tests and 2 conversion-sequences tests stop on them. | Keep out. The front end may leave those tests unsupported. |
| Named escapes `\N{...}` | lexical structure | Both named-escape tests need a Unicode name table. `char-named-escape.cpp` is a false reject (PRIM-5), and `char-named-escape-unknown.cpp` is unsupported. | Keep the tests. The front end may leave them unsupported. |
| Preprocessor beyond `#include "file"` | real headers come with the library | `include-missing-file-angle.cpp` is unsupported by design. | Keep out. |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them. | Keep out. |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | The CPT-19 review points at a C++26 change. The front end's work would not change. | Keep out. |
| MSVC | not in the matrix | A second ABI would double the layout tables. | Keep out. |
| Propose designs (P23, P25) | not part of the rules | A prototype tests whether a design can be built. | Allow on a branch, outside the baseline, or drop (item 7). |

## 4. Process changes

1. **Re-plan each `planned` row whose item ended.** The decision that adopts phase 12 names a phase 12 item for each of the 20 rows, or sets the row to `open`. `check_specification.py` can fail on a `planned` row whose Record names an item that a decision marks done.
2. **Run a front-end round in every phase that changes rules.** Phase 11 changed COND-16 and PACK-13, and the front end now rejects programs that the pages accept. A phase with no round leaves every new text fault and every reduction waiting a full phase.
3. **Turn each Implement or Port reduction into a test in the same phase.** The front-end harness sees only tests. I4 and P1 left six reductions with no test.
4. **Ask for programs in every Implement exercise.** I4's 85 passing assertions missed three wrong results. Its 13 programs found them. The prompt in `exercises.md` already asks for programs; the scoring section of the report should count the findings from programs apart from the score.
5. **Take or drop items deferred three times.** Rounds 6 to 8 ranked three items, and three decisions deferred them in a list with no reasons.
6. **Give the front-end counts one source.** The status page gives the round 22 baseline (6459 of 6681, 1696 run tests). The close commit gives 6478 of 6711 from a run. One tool should write both, from the same run, and say which tests arrived after the last round.
7. **Keep the front-end report form.** Report 22's tables by group, before and after, worked. Round 23 should keep them and add a row for each reduction that item 2 turns into a test.

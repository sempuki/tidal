# Roadmap, round 6: what to do after phase 8, from an implementer's view

Date: 2026-11-04. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The evidence is the clean-room front end in `frontend/`, its reports in `reports/clean-room/rust-frontend/`, and the Implement exercise I1 in `reports/clean-room/I1/`.

Inputs:
- AGENTS.md, CLAUDE.md
- `documentation/`: charter, roadmap, status, style guide, specification format, exercises, decisions 0033, 0039, 0044, 0050, 0051, 0054, 0055, and `decisions/pending.md`
- `reports/clean-room/roadmap-5/implementer.md`, the model for this report's form
- `reports/clean-room/rust-frontend/` reports 18 and 19, and `conformance.tsv`
- `reports/clean-room/I1/` (report, notes, score output)
- the phase 8 exercise records: R8, W16, W17, and X17 to X20 (from the repair commit messages and the exercise status table), and `tests/programs/`
- `reports/review/`: the two 2026-11-04 gap reviews (headings and results)
- `specification/` (the rules cited below), the headers of `tests/`, and `questions/README.md`
- `git log --oneline dc90663~1..HEAD` (18 commits), and the messages of commits `74f2cfc`, `1f81776`, `e4276f1`, `076aa54`, `26f05fc`, `6007252`, `ef19840`, and `1b16115`

**Measurements.** Every count comes from commit `1b16115`. I built the front end from that commit into my scratch directory with `cargo build --release --locked --target-dir`, so the repository's build directory was not touched. I ran `tidal-fe conform ../tests -v`. Its status counts match `conformance.tsv` exactly. I ran `tidal-fe check` on each of the 2521 single-unit passing `reject` tests to read the rules that the diagnostics name, and `tidal-fe run` on each of the 1514 single-unit `run` and `run-fail` tests to read the choice points. Counts come from grep and short Python scripts in my scratch directory, so they are approximate. Statements from general knowledge say so.

## 1. Where the specification and the front end stand

| Measure | Round 5 (6337 tests, `7d8c982`) | Now (6500 tests, `1b16115`) |
|---|---|---|
| Harness | judged by the EDG override first | judged by the base expectation (item HN) |
| Full pass | 6068 (99 of them by agreeing with EDG against the base) | 6183: 5947 `pass`, 221 `EDG-ONLY`, 9 `EDG-ALLOWED`, 6 `pass-partial` |
| Check pass | 6134 (6035 by the base expectation alone) | 6247 |
| `EDG-AGREES` (agrees with EDG against the base, a failure) | not measured (99) | 21 |
| False rejects | 4 | 12 |
| Missed rejects | 121 | 135 |
| Unsupported (check) | 78 | 85 |
| Run tests | 1564: 1491 pass, 1 wrong, 69 unsupported, 3 not run | 1620: 1546 pass (6 partial), 7 wrong, 0 other outcome, 0 missed or unexpected UB, 57 unsupported, 10 not run |
| Interpreter-only expectations | 14 | 19, all met |
| Rejects that name one of the test's rules (harness count) | 1655 of 2505 (66%) | 1748 |
| The same, by my script on single-unit tests | first diagnostic: 1516 name a test rule, 315 name no rule | any error: 1763 of 2521 (70%); first error: 1753; first error with no rule: 33 |
| `reject` rules never named first on a passing test | 176 | 144 of 558 |
| `reject` rules absent from the front end's source | 125 | 107 |
| Runs that pass a choice point | 56 | 57 single-unit runs (SEQ-8 36, SEQ-1 15, CMP-8 2, MPTR-7 2, PRIM-9 2, CMP-20 1); 6 partial |
| Rules by kind | 1691 | 1696: 898 required, 558 reject, 57 undefined, 52 impl-defined, 44 recommended, 36 no-diagnostic, 31 unspecified, 5 conditionally-supported, 15 withdrawn |
| `undefined` rules the interpreter names | 45, plus 8 through a general rule | 51 in the interpreter's source; ADD-14, MUL-14, OBJ-10, PRIM-8, and RET-6 through a general rule; ADD-18 not stopped |
| `unspecified` rules with a choice point or a fixed order | 9 of 31 | 9 of 31; the other 22 have no label |
| Questions | 376: 209 provisional, 13 compiler defect, 154 settled | 385: 221 provisional, 13 compiler defect, 151 settled |
| Front-end findings | G118 to G133 | G134 to G141; G136 to G141 acted on in `076aa54`; G134 and G135 not acted on |

### What phase 8 took from round 5

| Round-5 item | Phase 8 result |
|---|---|
| 1. A harness that judges by the specification | Done (item HN). Three parts were left: the list of allowed results per `unspecified` rule, the labels for the 22 `unspecified` rules with no choice point, and SEQ-8, which still has two options per operator. |
| 2. Front-end round 18 | Done in rounds 18 and 19. 74 of the 84 `EDG-AGREES` tests now give the base answer. CONV-11 follows its table. VOL-6 and OBJ-12 stop the run. The interpreter knows a closed list of C functions. |
| 3. Rule-ID scoring per page | Not taken. The harness still prints one total. |
| 4. An Implement exercise | Done (I1). 241 of 243 `static_assert` checks, 24 of 24 run checks, 13 findings, all repaired in `076aa54`. |
| 5. The run-time library surface | In part. The interpreter's list (`_Exit`, `exit`, `abort`, `malloc`, `free`, `puts`, `putchar`) is a front-end choice in `frontend/README.md`. The specification names none of these functions. |
| 6. Check-side round on missed rejects | Not taken. Missed rejects rose from 121 to 135. |

### The repairs went in after the last front-end round

Round 19 ran beside items RV, PQ, TE, and I1. The repairs from I1, X17 to X20, W16 and W17, R8, and G136 to G141 landed after it ended. 84 tests are new since `1f81776`, and 24 of them fail on the front end. Each commit's message said these failures "go to item FE", and item FE was already done.

| Source of the repair | Failing tests | Status |
|---|---|---|
| I1 (CLS-35 rows, the empty-class definition, the union rows) | `empty-class-cases.cpp`, `layout-union-size.cpp` | false reject: the front end's layout differs from the repaired table |
| G136 (CTOR-17 applies only through a constructor that uses the initializer) | `reference-default-member-unused.cpp`, `reference-default-member-aggregate-value.cpp` | false reject: the front end applies the old reading |
| G139 (STG-29 covers a member byte array) | `member-byte-array-cast-then-construct.cpp`, `member-byte-array-cast-then-placement.cpp` | wrong result 1: the front end kept the exception that G139 asked the text to remove |
| G140 (OVL-38 compares expressions) | `default-from-two-scopes-same-value.cpp` | `EDG-AGREES` |
| G141 (RFOR-7 member form) | `data-member-begin-not-callable-class.cpp`; `data-member-function-pointers.cpp` | `EDG-AGREES`; run-unsupported |
| X18 (VIRT-3 at most once, VIRT-7, VIRT-13) | `final-twice.cpp`, `override-twice.cpp`, `destructor-final-implicit-override.cpp`, `missing-override-inconsistent-warns.cpp`; `override-independent-base-unused.cpp` | 4 missed rejects; 1 `EDG-AGREES` |
| X19 (OPR-8, OPR-9, the `z` suffix) | `literal-reserved-suffix-used.cpp`, `int-size-suffix-hex-unsigned.cpp`; `literal-reserved-suffix-u8.cpp` | 2 false rejects; 1 `EDG-AGREES` |
| X20 (PFX-22, LNK-1) | `ellipsis-argument-conversions.cpp`; `class-unnamed-namespace-internal.cpp` | false reject at `va_list` (the test includes `<cstdarg>`); wrong result 1 |
| X17 (CONV-20) | `implicit-conversion-member-and-new-contexts.cpp` | unsupported |
| W16, W17 (ACC-20, TPL-23, programs) | `friend-specialization-only.cpp`; `member-outside-clause-missing.cpp`; `W16/evaluator.cpp`, `W17/units/main.cpp` | false reject; `EDG-AGREES`; 2 programs run-unsupported (class assignment, an overloaded operator) |

In four of these rows the front end now disagrees with text that its own finding changed: I1 (by way of the layout code), G136, G139, and G141. The specification moved, and the implementation that asked for the move did not follow it.

### Failures that are older than phase 8

- **`EDG-AGREES`, 21 tests.** 10 of them are left over from round 19, and report 19 gives a reason for each: partial ordering of conversion function templates (3 ORD tests), CNS-25, CNS-32, CTAD-18, DED-11 (`noexcept(B)`), INST-6, CPY-27, and the G134 case `function-point-constexpr-later.cpp`. The other 11 are the six that arrived during round 19 and the five new rows in the table above.
- **Wrong run results, 7 tests.** The three from phase 8 repairs are in the table above. The other four:
  - `return-local-destructor-throws-caught-inside.cpp` and `return-local-destructor-throws-result-order.cpp` (UNW-1). Tidal follows the review against Clang (`pending.md` row 4). The interpreter gives 10 and 2.
  - `thread-local-constant-variable-starts-nothing.cpp` (OBJ-11, `pending.md` row 4). The interpreter starts the deferred construction and gives 1.
  - `c-variable-and-global-variable-two-units.cpp` (LSP-8). The interpreter does not link a C variable to the global variable of the other unit.
- **Run-unsupported, 57 tests.** The largest groups are `new` forms (8), constructor calls (6), a variable declared in another unit (4), jumps into a nested statement (3), range-based `for` over a class (3), variables of an enclosing function (3), and destructor function try blocks (2).
- **Check-side unsupported, 85 tests.** 27 stop at a variable template with arguments the front end cannot compute (the `same` idiom; 23 in round 5). Then concepts in `static_assert` (8), conversion to a function reference (12 across two contexts), conversion functions in constants (10), `new` in a constant (6), and the `noexcept` operator (4).
- **Missed rejects, 135 tests.** Templates have 47, classes 24, expressions 22, declarations 21, and overloading 9. `expressions/conditional` alone has 10.

### Places where the text or the record still stops the implementer

1. **Two front-end findings have no answer.** G134 asks for a header line that names the rule that allows EDG's answer, because a test that cites one `impl-defined` rule and one `required` rule is judged `EDG-AGREES`. G135 names three tests with no base `tidal-expect` line (`array-too-large.cpp`, `floating-overflow-constant.cpp`, `floating-narrowing-out-of-range.cpp`), which the harness passes with either answer. No commit, question, or document cites G134 or G135. `function-point-constexpr-later.cpp` is still `EDG-AGREES` for this reason. `floating-overflow-constant.cpp` still passes with either answer, although by CE-9 the specification's answer is reject.

2. **Rule IDs on rejects.**
   - 1753 of 2521 passing single-unit rejects name a rule of the test in their first error (70%). The rate rose from round 5, and errors with no rule fell from 315 to 33.
   - The lowest pages by first error:

     | Page | Named | What the front end names instead |
     |---|---|---|
     | foundations/keywords | 0 of 94 | TR-12 on 79 |
     | foundations/values | 0 of 9 | UN-9, ASN-2, DTOR-3, REF-10, PFX-20 |
     | templates/ordering | 0 of 10 | OVL-36 |
     | foundations/types | 1 of 20 | no rule on 19 (TYP-7 specifier combinations) |
     | expressions/decltype | 2 of 12 | TR-12, DTOR-3, DEL-1, CE-12 |
     | foundations/conversions | 7 of 20 | |
     | expressions/conditional | 5 of 13 | |
     | foundations/odr | 8 of 19 | |
     | templates/instantiation | 30 of 67 | |

   - Most of these pairs are a general rule and a specific rule that both reject the program, for example VC-12 against ASN-2 and ORD-15 against OVL-36. Decision 0052 lets a run stop match a general `undefined` rule through a link. The harness has no such match for a rejection. This is the third round that asks for one.
   - The TYP-7 case is a front-end gap: the diagnostic exists and carries no rule.

3. **The library surface is a front-end choice.** The interpreter's list of seven C functions is in `frontend/README.md` under decision 0035. Ten tests declare such functions with C linkage. A second implementer has to read the front end's README or the tests to learn which functions a test may call. `ellipsis-argument-conversions.cpp` (PFX-22) is the only test that includes a standard header (`<cstdarg>`). The front end reads no standard header, so the test is a false reject at `va_list`. Decision 0041 lists the language-support names. It lists neither `va_list` nor the C functions.

4. **The `unspecified` rules are half done.** Item HN judges every explored run against the one expected outcome. 22 of the 31 `unspecified` rules have no choice point and no label that says whether a test can observe them (ADD-13, ASN-11, CE-27, CLS-34, CPY-12, ENUM-13, INST-3, INST-11, JMP-11, LAM-20, MUL-13, MUL-17, NEW-14, NEW-18, NEW-34, OPR-12, SINIT-4, STG-24, STG-25, TR-16, VC-7, VC-15). Six program runs are partial. Three stop at the bound of 64, and three settle a line.

5. **I1 worked, and it covered one procedure.** I1 found 13 gaps in a page that 17 front-end rounds had already passed. The front end's failures show where the next blind build would help most: partial ordering (0 of 10 rejects named, 3 `EDG-AGREES`), the conditional operator (10 missed rejects, 6 unsupported), and the ranking of conversion sequences (11 unsupported on that page).

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | What it unblocks |
|---|---|---|---|---|---|
| 1 | Front-end round 20: catch up with the phase 8 repairs | 24 of 84 new tests fail; 4 of them contradict text the front end's own findings changed; 21 `EDG-AGREES`; 7 wrong results | medium | low | every phase 8 repair checked by an implementation |
| 2 | Close G134 and G135, and give each repair a front-end check | G134 and G135 have no answer; 3 tests with no base line; repairs landed after item FE ended | small | low | a harness with no test that passes on either answer; repairs that do not wait a phase for a check |
| 3 | Rule-ID scoring per page, with links for rejections | 70% rate; keywords 0 of 94, values 0 of 9, ordering 0 of 10, types 1 of 20; asked for in rounds 3, 4, and 5 | small | low | a short list of rule boundaries to review |
| 4 | Implement I2: partial ordering of function templates, blind | I1: 13 findings on a page that had passed; ordering has 0 of 10 rules named and 3 `EDG-AGREES` | medium | medium | a measure of the second procedure page; findings before the front end patches around them |
| 5 | The library surface in the specification | 7 C functions in a README; 10 tests declare them; `<cstdarg>` in one test | small | low | a stated boundary for a second implementer; PFX-22's run test |
| 6 | The `unspecified` rules: a label for each of the 22, then allowed results | 22 rules unlabelled; SEQ-8 has two options; 6 partial runs | small (labels), medium (results) | low | the results that decisions 0047 and 0048 promise |
| 7 | Check-side round on missed rejects and unsupported constructs | 135 missed rejects, 85 unsupported, 27 at the `same` idiom | large | low | a front end that rejects what the text rejects |

### 1. Front-end round 20: catch up with the phase 8 repairs

**What it is.**
- The 24 failing tests from the repairs after round 19 (section 1, first table). Start with the four where the front end's own finding changed the text:
  - the I1 changes to CLS-35 in the front end's layout code (empty class, union layout, data size, empty `[[no_unique_address]]` member),
  - CTOR-17 applied only through a constructor that uses the default member initializer (G136),
  - STG-29 on member byte arrays (G139), which also lets `W15/optional.cpp` be checked under the stricter reading,
  - the member form of RFOR-7 (G141).
- The X18 checks on `override` and `final` (VIRT-3, VIRT-7, VIRT-13), the X19 literal rules (OPR-8, OPR-9, the `z` suffix), LNK-1 across units, and ACC-20.
- The 10 `EDG-AGREES` tests left from round 19, and the 6 that arrived during it.
- The four older wrong results: LSP-8 across units, OBJ-11, and the two UNW-1 tests. UNW-1 and OBJ-11 follow `pending.md` row 4. The interpreter should give the answer that Tidal keeps until the maintainer decides.
- The two run-unsupported programs, `W16/evaluator.cpp` (class assignment) and `W17/units/main.cpp` (an overloaded operator across units).

**Evidence.** Section 1. The repair commits `076aa54`, `26f05fc`, and `6007252` each say that the tests that fail go to item FE. Round 19 ended before them.

**What it unblocks.** Every phase 8 repair has an implementation that follows it. The four contradictions are the clearest test of whether a repair says what the finding asked, because the finding's author is the implementer.

**Cost.** Medium. Most items are local checks. Layout and partial ordering are the larger parts.

**Risk.** Low. If the front end cannot follow a repaired rule from its text alone, that is a new G finding and worth having.

### 2. Close G134 and G135, and give each repair a front-end check

**What it is.**
- G134: a header line on an EDG override that names the rule that allows EDG's answer, or says that EDG's answer is a defect. The harness then judges `EDG-ALLOWED` by that rule alone.
- G135: a base `tidal-expect` line for the three tests, or a note that each rule allows both answers and a harness status that says so. `floating-overflow-constant.cpp` waits on Q-0001. The test should say what Tidal keeps until Q-0001 settles.
- A repair commit runs `tools/frontend_check.py` on the tests it adds, as `076aa54` already did, and lists each failing test with the rule it cites. When a phase's front-end item is already closed, the failures go to a named next round in `status.md`, and not to the closed item.

**Evidence.** Section 1, place 1, and the "go to item FE" lines of the three repair commits after `1f81776`.

**What it unblocks.** No test passes on either answer. The status page shows which tests wait on the front end.

**Cost.** Small.

**Risk.** Low.

### 3. Rule-ID scoring per page, with links for rejections

**What it is.**
- `tidal-fe conform` prints, per page, the rate of passing rejects whose first error names one of the test's rules, and lists the passing rejects whose first error names another rule.
- A rejection that names a rule linked from the test's rule counts as a match, as decision 0052 does for run stops. The other choice is for the tests to cite both rules under decision 0042.
- The 144 `reject` rules that are never the first rule on a passing test, each marked as a front-end shortcut or a rule that another rule always reaches first.
- The front end gives the TYP-7 diagnostic a rule.

**Evidence.** Section 1, place 2. Rounds 3, 4, and 5 asked for this, and no phase took it.

**What it unblocks.** A short list of candidate rule-boundary findings (decision 0038). Keywords, values, ordering, and types come first.

**Cost.** Small.

**Risk.** Low. A front end can raise the rate by naming rules loosely, so the rate should prompt review and should not be a target.

### 4. Implement I2: partial ordering of function templates, blind

**What it is.** A second Implement exercise under the prompt in `exercises.md` section 9. A fresh clean-room reader builds partial ordering from the ordering page and the parts of the deduction page it uses, with Evidence and `:tests:` lines removed, and without reading `tests/` or `frontend/`. The component takes two function template signatures and a call context and returns which is more specialized, or neither. The scoring phase turns the ordering tests and grids into calls. A second candidate is the ranking of implicit conversion sequences on the conversion-sequences page.

**Evidence.**
- I1 found 13 gaps in CLS-35, BF-9, and the glossary. Front-end rounds 15 to 17 had already passed the layout tests, because the front end learned some of its choices from the tests (G119, G123 to G127).
- On the ordering page, the front end names none of the 10 test rules on its rejects (it names OVL-36). Three ordering tests are `EDG-AGREES`, and report 19 says the front end lacks partial ordering of conversion function templates.
- Partial ordering is a procedure with several steps, and Predict batches check single outcomes only.

**What it unblocks.** A measure of how far the text alone carries an implementer on a templates procedure, and findings that a front-end round would otherwise patch around.

**Cost.** Medium. Partial ordering needs a small model of types and deduction, which layout did not.

**Risk.** Medium. General knowledge: partial ordering is one of the procedures that implementations are known to differ on, so a reader may fill gaps from memory. The prompt should ask the reader to mark each step the text did not give, as I1's `notes.md` did.

### 5. The library surface in the specification

**What it is.**
- The language-support page, or a test-convention section of `specification-format.md`, lists the C functions that tests may declare (`_Exit`, `exit`, `abort`, `malloc`, `free`, `puts`, `putchar`), with one sentence each on the effect a test relies on. The list holds no rules.
- For `va_list`, one of two changes. The PFX-22 run test observes the conversions some other way, or decision 0041 lists the variadic-argument facilities as a language-support shape.

**Evidence.** Section 1, place 3. The interpreter's list exists only as a front-end choice. `ellipsis-argument-conversions.cpp` is a false reject because of `<cstdarg>`.

**What it unblocks.** A second implementer knows which library effects a test can rely on without reading the first implementer's README.

**Cost.** Small.

**Risk.** Low. Decision 0041 should say whether a test convention widens the scope.

### 6. The `unspecified` rules: a label for each of the 22, then allowed results

**What it is.**
- Each of the 22 rules gets one label: the interpreter explores it, a matrix table fixes it, or no test can observe it.
- SEQ-8 explores more than two options where an operator has more than two side effects.
- Where a rule gives a finite set of results, the test lists that set, and the harness compares the explored set with it.

**Evidence.** Section 1, place 4. Report 18, section 3, left these items.

**What it unblocks.** The result sets that decisions 0047 and 0048 make the test of an `unspecified` rule.

**Cost.** Small for the labels, medium for the result lists.

**Risk.** Low. More exploration means more partial runs, and the harness already reports them.

### 7. Check-side round on missed rejects and unsupported constructs

**What it is.** Close the 135 missed rejects and the 85 check-side unsupported tests, grouped by analysis. The groups are the conditional operator (10 missed, 6 unsupported), templates (47 missed), access after overload resolution, the `same` idiom with variable templates (27), conversion to a function reference (12), and the `noexcept` operator (4).

**Evidence.** Section 1. Missed rejects went from 121 to 135 because no phase 8 item took them.

**What it unblocks.** A front end that rejects what the text rejects on every page.

**Cost.** Large, for a modest gain per test.

**Risk.** Low. Each group the front end cannot close from the text becomes a G finding.

### Dependency sketch

```
2 G134, G135, repair checks ──> 1 front-end round 20 ──> 7 check-side round
3 rule-ID scoring (independent, small)
4 Implement I2 (independent; best before round 20 works on ordering)
5 library surface ──> 1 (PFX-22 run test)
6 unspecified labels (independent; result lists after the labels)
```

Items 2, 3, 5, and the labels of item 6 are small and can start at once. Item 4 should run before item 1 touches partial ordering, so the blind reader's findings come first.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| C library functions in tests | the general library | The interpreter knows seven by a front-end choice. A second implementer has to guess. | Keep the library out. State the seven as a test convention (item 5). |
| `<cstdarg>` and `va_list` | the general library, and no standard headers | One test needs them for PFX-22. | Keep out. Rewrite the test, or list the shape under decision 0041. |
| Named escapes `\N{...}` | lexical structure beyond G1 | One test needs a Unicode name table, which is data. | Keep the test. The front end may leave it unsupported. |
| Lexical structure beyond G1 | similarity risk | No phase 8 finding needed a new lexical rule. X19 changed only the suffix rules. | Keep out. |
| Preprocessor beyond `#include "file"` | real headers come with the library | Multi-unit tests and test headers work. | Keep out. |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them. OBJ-11 touches `thread_local` and needs no thread model. | Keep out. |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | An index of flips would not change what the front end must do. | Keep out of the front end's work. |
| MSVC | not in the matrix | A second ABI would double the CLS-35 and BF-9 tables, which I1 just repaired. | Keep out. |
| `typeid` | needs a library shape | One test uses it. | Keep out. |

## 4. Process changes

1. **A repair names its front-end round.** When a repair adds tests that fail on the front end and the phase's front-end item is closed, the commit names the next round, and `status.md` lists the tests (item 2). Phase 8 sent 24 tests to an item that had already ended.
2. **Report the run side in `status.md`.** Rounds 4 and 5 asked for this. The status page gives 6183 full passes and 6247 check passes, and no run counts. Add one line: 1546 of 1620 runs pass (6 partial), 7 give a wrong result, 57 are unsupported, and 10 are not run because the check failed.
3. **Keep a ledger of front-end findings.** G134 and G135 have no answer and no question. A table of G findings with their state (acted on in a commit, recorded as a question, or open) would show them. `question_index.py` could warn on a G finding that is in no ledger row.
4. **Run an Implement exercise each phase.** Decision 0055 counts Implement as an exercise by use. I1 found 13 gaps that 17 test-driven rounds had missed. One procedure per phase, picked by the lowest rule-ID rate or the most `EDG-AGREES` tests, keeps that check going.
5. **Report the named-rule rate per page** (item 3). This is the fourth round that asks.
6. **Keep the front-end report form.** Rounds 18 and 19 gave before-and-after counts on one copy of the tests, and that made the HN correction readable. Round 20 should keep the form and add a table of the phase 8 repair tests with their status before and after.

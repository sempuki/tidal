# Roadmap, round 4: what to do after phase 6, from an implementer's view

Date: 2026-11-03. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The clean-room front end in `frontend/` is the evidence.

Inputs:
- AGENTS.md, CLAUDE.md
- `documentation/`: charter, roadmap, status, style guide, specification format, exercises, and decisions 0033, 0039, 0044, 0050, 0051, 0052, 0053
- `reports/clean-room/roadmap-3/implementer.md`, which is the model for this report's form
- `reports/clean-room/rust-frontend/` reports 10, 10b, 11, 12, 13, and 14, and `conformance.tsv`
- the phase 6 exercise reports: R-expressions, R-statements, R-declarations, R-classes, W1 to W15 (the programs and the reports), X9 to X12 (headings only)
- `reports/review/`: the 2026-11-03 gap reviews of question clusters 2 to 6 and of item O, and the class-property inventory (headings and results)
- `specification/` (translation, keywords, conversions, storage, members, unions, and the rules cited below), the headers of `tests/`, `questions/README.md`, Q-0342
- `git log --oneline 39e9322..HEAD` (79 commits) and the messages of the repair commits

**Measurements.** All counts are from commit `3d98c93`. I ran the committed binary (`frontend/target/release/tidal-fe conform ../tests -v`) read-only, with its output in my scratch directory. Its statuses match `conformance.tsv` on all 6032 tests, so the numbers below are the committed baseline. I also ran `tidal-fe run` on each of the 1407 `run` and `run-fail` tests to count choice points, ran `tidal-fe check` on each of the 2283 passing `reject` tests to find the rule each diagnostic names, and ran the interpreter on the 63 finished Write programs under `reports/clean-room/W*/`. Counts come from grep and short Python scripts, so they are approximate. Statements from general knowledge say so.

| Measure | Round 3 (5408 tests) | Now (6032 tests) |
|---|---|---|
| Check pass | 4675 (86%) | 5708 (95%): 5586 pass, 122 differ from EDG only |
| False rejects | 74 | 6 |
| Missed rejects | 347 | 212 (187 cite a `reject` rule) |
| Unsupported (check) | 312 | 106 |
| Check, `templates/` | 925 of 1104 (84%) | 1051 of 1126 (93%) |
| Check, by expectation | accept 1576/1802, reject 1874/2355, link-fail 36/36 | accept 1926/2019, reject 2341/2564, link-fail 39/39, run 1315/1323, run-fail 84/84 |
| Rejects that name one of the test's rules | 1277 of 1874 (68%) | 1537 of 2283 (67%) |
| Run tests checked by the interpreter | 1212: 946 pass, 6 wrong, 3 missed UB, 4 unexpected UB, 225 unsupported, 28 not run | 1411 (1323 `run`, 84 `run-fail`, 4 interpreter-only): 1286 pass, 0 wrong, 1 missed UB, 0 unexpected UB, 115 unsupported, 9 not run |
| Runs that pass a choice point | 37, one fixed choice | 45; 44 explore the other choices, and every other choice gives the same result |
| Rules by kind | 1633 | 1678: 892 required, 553 reject, 56 undefined, 51 impl-defined, 44 recommended, 33 no-diagnostic, 30 unspecified, 4 conditionally-supported, 15 withdrawn |
| `undefined` rules with no run witness | 15 | 13 |
| Questions | 321: 288 provisional, 20 compiler defect, 12 settled, 1 open | 352: 309 provisional, 12 compiler defect, 30 settled, 1 open |
| Front-end findings | G76 to G84 | G85 to G117, all repaired by `3d98c93` |

## 1. Where the specification still stops an implementer

Phase 6 took every item of my round-3 list except the launder operation, which decision 0051 keeps referenced.

| Round-3 item | Phase 6 result |
|---|---|
| 1. Byte-level memory | Done (report-10b). The interpreter keeps bytes for scalars, arrays, and simple classes. The four STG-23 tests stop with STG-23. The bytes of a class with bases, bit-fields, or a pointer for virtual calls are still missing. |
| 2. Grammar-validity rule | Done. TR-12 is the rule, and 152 passing rejects now name it. Q-0312 is still the one `open` question. |
| 3. Undefined-rule overlap and an interpreter expectation | Done (decisions 0052, 0053). Four tests carry `tidal-interpreter: run-fail`. |
| 4. Interpreter coverage | Mostly done. Wrong results went from 6 to 0, and run-unsupported from 225 to 115. |
| 5. Check-side round | Done in rounds 10 to 14. False rejects went from 74 to 6. |
| 6. Predict triage | Done as item P for the expressions, statements, declarations, and classes chapters, scored by cited rule. |
| 7. Choice exploration and SEQ-4 | Done in part. The interpreter explores choices and detects SEQ-4 on the operands of built-in binary operators. |
| 8. Class-property lists and layout | Done as item O (CPY-19, CPY-28, CLS-19, CLS-29 to CLS-34, UNI-15, UNI-16). |
| 9. The launder operation | Kept out (decision 0051, Q-0322). |

### Failures by page

The page is the page of the first rule that the test cites. "Run unsupported" counts `run` and `run-fail` tests whose check passed and whose run the interpreter cannot finish.

| Page | False reject | Missed reject | Unsupported | Run unsupported | Rules most cited |
|---|---|---|---|---|---|
| expressions/new-delete | 0 | 4 | 5 | 21 | NEW-2, NEW-28, NEW-21, NEW-29 |
| overloading/overload-resolution | 0 | 7 | 10 | 2 | OVL-20, OVL-29, OVL-9, OVL-36 |
| classes/unions | 0 | 7 | 6 | 5 | BF-9, UNI-7, BF-11, UNI-2 |
| classes/members | 0 | 0 | 11 | 4 | CLS-33, CLS-34, CLS-10, CLS-17 |
| declarations/structured-bindings | 0 | 4 | 8 | 2 | SB-5, SB-17, SB-14, SB-12 |
| expressions/conditional | 0 | 6 | 2 | 4 | COND-3, COND-9, COND-5, COND-8 |
| templates/dependent-names | 1 | 6 | 3 | 2 | DEP-37, DEP-12, DEP-30, DEP-20 |
| classes/conversion-functions | 0 | 3 | 1 | 7 | CVF-2, CVF-1, CVF-9, CVF-4 |
| declarations/initialization/list | 0 | 7 | 2 | 2 | LIST-14, LIST-13, LIST-16 |
| overloading/conversion-sequences | 0 | 1 | 10 | 0 | ICS-45, ICS-44, ICS-35, ICS-47 |
| templates/instantiation | 0 | 9 | 1 | 1 | INST-6, INST-12, INST-30 |
| foundations/names | 3 | 7 | 0 | 0 | NAM-41, NAM-39, NAM-42 |

No page has more than 30 failures. Round 3 had eight pages with more than 30.

### Front-end gaps (the text is enough)

- **Class layout in the interpreter.** Item O and the W13 repairs now give the matrix layout: BF-9 gives the bit-field placement on both targets, and the CLS-31 and CLS-34 Evidence gives a table for empty bases, the pointer for virtual calls, and the reuse of end padding (Q-0342). The front end does not use them yet.
  - 15 `static_assert` tests stop on "sizeof of a class with a layout the front end does not compute" (CLS-33, CLS-34, BF-9, STG-3, DER-2).
  - 4 run tests stop on "bytes of a type whose layout the front end does not compute" (STG-25, BF-9, two `new` tests), and 5 on "class layout the front end does not model" (anonymous unions, UNI-7, CTOR-24).
  - Two Write programs stop there too (`W13/header.cpp`, `W8/tagged.cpp`).
- **Run-time constructs.**

  | Construct | Run tests |
  |---|---|
  | `new` and `delete` with class-specific or replaced allocation functions, over-aligned types, and calls of `operator new` by name | about 22 |
  | virtual bases | 9 |
  | constructor calls the check does not build | 6 |
  | variables of an enclosing function in a local class or lambda | 5 |
  | block-scope `extern` and static declarations used before their declaration ran (LNK-9, STO-7) | 4 |
  | jumps into a nested statement (IF-12, SW-1) | 3 |
  | overloaded `operator[]`, `operator->`, and several-argument `operator[]` | 7 |

- **Check-side unsupported constructs.** The `same` idiom with arguments the front end cannot compute is 28 tests (34 in round 3). Then concepts in `static_assert` (7), template-parameter values (7), conversion to a function reference (12), conversion functions (8), and the `noexcept` operator (4).
- **Missed rejects.** 212 remain, spread thinly. 187 cite a `reject` rule, 17 only `recommended` rules, and 7 only `no-diagnostic` rules. The phase 6 Predict batches scored 3262 of 3272 on the same chapters, so the text decides most of these outcomes.
- **False rejects.** Six remain. Three are new NAM-39 and NAM-40 tests (a class and a variable that share a name). One is `current-instantiation-partial-alias.cpp`, which the repair of G116 added in `3d98c93`. One needs the Unicode name table (`char-named-escape.cpp`). One needs substitution into a generic lambda's trailing return type (DED-29).
- **Write programs.** Of the 63 finished Write programs, 47 run to result 0 in the interpreter (9 of them with fixed choices), 14 stop as unsupported, and 2 are rejected. GCC and Clang build and run both rejected programs:
  - `W15/optional.cpp`: DEL-1, "the copy constructor of 'Buffer' is deleted".
  - `W15/largest.cpp`: CE-16 on `static_assert(largest(1, 2.5, 'c') == 99.0)`.

  Both are front-end false rejects that no test catches.

### Places where the text still stops the implementer

1. **Some `impl-defined` rules give no matrix choice.** An interpreter that must match the matrix needs the choice. A grep of rule bodies for "matrix", "x86-64", "-m32", or a table finds 17 `impl-defined` rules without one. Some of these get their choice from another rule (MUL-11 from MUL-20, ADD-12 from LSUP-4, SINIT-5 names Clang and GCC). Others do not:
   - CONV-11: a floating value above the largest finite value converts to the largest finite value or to infinity, and "the implementation defines which one". `floating-narrowing-out-of-range-run.cpp` stops the interpreter for this reason. The test accepts either result, but an interpreter must pick one.
   - CONV-7: the rounding of a value between two floating values.
   - CAST-12: whether `reinterpret_cast` converts between function and object pointers.

   Decision 0036 asks for per-target tables. An audit of the 51 `impl-defined` rules would show which still lack one.

2. **The matrix layout sits in Evidence, and STG-25 disagrees with it.**
   - CLS-34 makes the place of the pointer for virtual calls, the padding, and the reuse of a base's end padding `unspecified`. The matrix choices are in the page's Evidence table only.
   - The tests assert exact sizes. `layout-base-tail-padding.cpp` expects `sizeof(Next<Plain>) == 12` and `sizeof(Next<Initialized>) == 8`. An implementer who reads only the rules cannot derive these values.
   - STG-25 says that in the matrix a derived member can use the end padding of a base "whose constructor is user-provided". The CLS-34 Evidence table lists more cases: a default member initializer, any user-provided special member, or a `private` or `protected` data member. The two statements should agree, and the rule should hold the full condition.
   - General knowledge: the matrix targets follow a platform ABI that fixes these choices. Stating them as matrix facts carries little similarity risk.

3. **`undefined` rules without a run witness.**
   - 13 `undefined` rules have no `run-fail` test and no interpreter-only test: ADD-11, ADD-14, ADD-17, ADD-18, CTOR-12, INIT-1, MUL-12, MUL-14, NEW-3, OBJ-10, SEQ-4, TRY-11, VOL-6.
   - The interpreter detects SEQ-4 (report-10b), but no test cites it at run time.
   - `scope-end-other-type-exception.cpp` (STG-21) is an `accept` test that the interpreter stops with STG-21, and it lacks the `tidal-interpreter: run-fail` line.
   - `nonvolatile-access-to-volatile-constant.cpp` (VOL-6) runs to result 0 in the interpreter. The interpreter does not track access to a volatile object through a non-volatile glvalue.
   - The new `implicit-objects-no-set-sanitizer.cpp` (STG-18) is the one RUN-MISSED-UB. The interpreter does not detect a byte array where no set of implicit objects gives defined behavior.

4. **Rule IDs on rejects.**
   - 1537 of 2283 passing rejects name one of the test's rules (67%). 286 name no rule at all.
   - The 92 keyword tests cite KW-1, and the front end names TR-12. KW-1 says that "the grammar derives no name from a keyword (TR-12)". The harness lets a run stop match a general or a specific `undefined` rule (decision 0052). It has no such match for a rejection, so these 92 correct rejects count as naming the wrong rule.
   - By my count, which reads only the rule in brackets, the lowest named-rule rates after keywords are types (0 of 18), names (19 of 88), conversions (5 of 19), member pointers (9 of 28), instantiation (25 of 65), and dependent names (18 of 44). A low rate points either at a front-end shortcut or at a rule boundary that the text leaves unclear.

5. **Choice points are explored, but the result set is not checked.**
   - 45 run tests pass a choice point: SEQ-8 (27), SEQ-1 (11), PRIM-9, MPTR-7, CMP-8 (2 each), SINIT-3, CMP-20 (1 each). Every explored alternative gives the same result, and no run reached the bound.
   - SEQ-1 tries two argument orders, not every permutation. The SEQ-4 check does not cover overloaded operators or the placement arguments of `new` (NEW-18).
   - The harness does not list a test whose explored runs differ. Decision 0047 makes the set of results the test of an `unspecified` rule, and nothing compares that set with the matrix results.

6. **Pages that no reader has used except through the front end.** Ten pages have no Predict batch: overloading (conversion sequences, overload resolution), exceptions (throw, handlers, unwinding), and foundations (types, keywords, language support, the one-definition rule, storage). The overloading pages are new as a chapter in phase 6 (item C), and they have 30 front-end failures. The storage page holds most of the byte-level rules that the interpreter implements.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | What it unblocks |
|---|---|---|---|---|---|
| 1 | Matrix target profile: a choice for every `impl-defined` rule, and the layout table as rules | 17 rules without a matrix choice by grep; layout only in Evidence; STG-25 against CLS-34 | small to medium (3 to 6 rules or tables, an audit) | low | items 2 and 4; a deterministic interpreter |
| 2 | Front-end round 15: class layout, allocation functions, virtual bases, and the six false rejects | 24 layout tests, about 22 `new` run tests, 13 virtual-base tests, 2 rejected Write programs | medium to large | low | about 60 tests; the storage and members pages at run time |
| 3 | Write programs as run tests | 63 programs, 47 run, 2 false rejects no test catches; decision 0051 asked for it | small | medium | whole-program regression tests for the interpreter |
| 4 | Run witnesses for the remaining `undefined` rules | 13 rules without one; SEQ-4 detected and untested; STG-21 and VOL-6 tests | small | medium | decision 0053 applied to every `undefined` rule |
| 5 | Predict batches for the ten pages without one | overloading 30 front-end failures; storage and language support used only by the front end | medium | low | a text check of the last pages; a source of findings for item 2 |
| 6 | Rule-ID scoring per page, and a reject overlap for TR-12 | 67% named-rule rate; 286 rejects with no rule; 92 KW-1 tests | small | low | a list of rule boundaries to check |
| 7 | Result sets for choice points | 45 tests, sets not compared; SEQ-1 two orders only | medium | medium | a check of each `unspecified` rule against the matrix |
| 8 | Check-side round on the missed rejects | 212 missed rejects, 106 unsupported | large | low | a front end that rejects what the text rejects |

### 1. Matrix target profile

**What it is.**
- An audit of the 51 `impl-defined` rules. Each rule gets the choice of the matrix targets (x86-64 and `-m32`), in the rule or in a table that the rule links (decision 0036). CONV-7, CONV-11, and CAST-12 are the first cases.
- The CLS-34 Evidence table moves into an `impl-defined` table that the matrix choices fill, in the way BF-9 does now. CLS-34 can stay `unspecified` for the language, with the table stating what the matrix does.
- STG-25 takes the full end-padding condition, or links to the table.
- `unspecified` rules (30) get a list of the allowed results where the text has a finite set, for item 7.

**Evidence.**
- 17 `impl-defined` rules match no grep for a matrix choice. Several of them get the choice from another rule, so the audit is a reading task, and the real count is lower.
- `floating-narrowing-out-of-range-run.cpp` stops the interpreter at CONV-11.
- The 15 `sizeof` tests and `layout-base-tail-padding.cpp` assert values that only the Evidence states.
- STG-25 and the CLS-34 Evidence give different conditions for the reuse of end padding.

**What it unblocks.** Item 2's layout code works from rules. The interpreter becomes deterministic on every `impl-defined` point, so a wrong result is a front-end bug or a text gap, never a guess.

**Cost.** Small to medium: an audit, three to six rule changes, one layout table, and the tests stay as they are.

**Risk.** Low. The facts come from the matrix compilers' behavior, which a writer can probe. General knowledge: these choices come from a published platform ABI and not from the ISO text, so the similarity risk is low. The gap review of item O warned that labelling all layout as implementation-defined promises documentation that the reference does not require. A table that says "the matrix does this" avoids that promise.

### 2. Front-end round 15

**What it is.**
- Class layout from BF-9, CLS-31, CLS-33, CLS-34, and item 1's table: bases, empty bases, bit-fields, the pointer for virtual calls, and end-padding reuse. Bytes of such classes in the interpreter.
- `new` and `delete` with class-specific and replaced allocation functions, over-aligned types (NEW-28, NEW-29), and calls of `operator new` and `operator delete` by name (NEW-40, NEW-41, STG-28).
- Virtual bases at run time and in constant evaluation.
- Anonymous unions (UNI-7), jumps into nested statements (IF-12, SW-1), block-scope `extern` before its declaration ran (LNK-9, STO-7), and the remaining overloaded `operator[]` and `operator->` forms.
- The six false rejects and the two rejected Write programs.
- STG-18 detection for the new sanitizer test, and VOL-6 detection.

**Evidence.** 115 run-unsupported tests, of which about 22 are `new` forms, 9 are virtual bases, and 9 are layout. Four more check-side tests stop at a virtual base. 24 check and run tests stop on layout (section 1). `W15/optional.cpp` and `W15/largest.cpp` are false rejects of programs that GCC and Clang run.

**What it unblocks.** About 60 tests. The storage, members, unions, and new-delete pages at run time. Write programs W4, W8, W9, and W13 in the interpreter.

**Cost.** Medium to large. Report-10b already names the layout as the main construct still missing.

**Risk.** Low. The round should end with G findings in the usual form. Any layout fact the text leaves open becomes a finding against item 1.

### 3. Write programs as run tests

**What it is.**
- Each finished Write program becomes a `run` test. A new directory, for example `tests/programs/`, holds them, so that they do not count against one page.
- `tidal-rules` lists the rules the program's report cites as deciding. The front-end harness runs them as it runs other `run` tests.
- Probe and mistake programs stay in the reports. The mistake programs that a compiler rejects for a stated reason can become `reject` tests on their pages.

**Evidence.**
- Decision 0051 says that every Write program also runs in the interpreter and becomes a run test. The programs ran in the interpreter (the reports say so), and none became a test. 44 test notes name a Write finding. No test holds a whole program.
- I ran the 63 finished programs. 47 give result 0, 14 stop as unsupported, and 2 are false rejects that only this run shows.

**What it unblocks.** Regression tests that combine many rules, which single-rule tests do not give. A front-end change that breaks a whole program shows up in `frontend_check.py`.

**Cost.** Small. The programs exist and pass on GCC and Clang.

**Risk.** Medium. A whole program cites many rules, and the per-rule counts in `status_table.py` would grow. The decision should say whether these tests count toward a rule's tests or sit apart. The programs are clean-room work, so the similarity risk is low.

### 4. Run witnesses for the remaining `undefined` rules

**What it is.**
- For each of the 13 rules, a `run-fail` test if a sanitizer stops it, or an `accept` test with `tidal-interpreter: run-fail` if only the interpreter does.
- SEQ-4 first, because the interpreter already detects it and no sanitizer does (general knowledge).
- `scope-end-other-type-exception.cpp` gains the line.
- Each rule that no tool can stop gets a note on its page that says so, so the gap is recorded and not open-ended.

**Evidence.** Section 1, place 3. Decision 0053's consequences ask that other `accept` tests of `undefined` rules gain the line when the interpreter stops them. Only 4 tests carry it now.

**What it unblocks.** Decision 0048's harness rule applied to every `undefined` rule. A list of `undefined` rules that the interpreter does not model (VOL-6, STG-18 now).

**Cost.** Small: about 15 tests and a few lines of front-end work.

**Risk.** Medium. Decision 0053 says that an interpreter stop shows only that the program reaches the named operation. The rule's class still rests on compiler evidence. Tests that only the interpreter judges should stay a small, labelled set.

### 5. Predict batches for the ten pages without one

**What it is.** Predict batches, scored by cited rule, for conversion sequences, overload resolution, throw, handlers, unwinding, types, keywords, language support, the one-definition rule, and storage. `tools/exercises.py` builds them.

**Evidence.**
- The phase 6 Predict batches covered 57 pages and scored 3262 of 3272, with findings that the repairs acted on.
- The overloading pages became a chapter in phase 6 with slimmer Evidence, and they hold 30 front-end failures, the second most of any chapter.
- The storage and language-support pages define what the interpreter models, and only the front end has used them.

**What it unblocks.** A text check of every page by a reader. The overloading failures split into text gaps and front-end gaps without the front-end author's judgment.

**Cost.** Medium: three or four batches.

**Risk.** Low. Keywords is a page of 92 near-identical tests, so its batch can sample.

### 6. Rule-ID scoring per page, and a reject overlap for TR-12

**What it is.**
- `tidal-fe conform` reports the named-rule rate per page, and lists passing rejects that name no rule.
- Extend decision 0052's matching to `reject` rules whose text links a general rule: a diagnostic that names TR-12 matches a test of KW-1. Or the keyword tests cite TR-12 as well, under decision 0042.

**Evidence.** Section 1, place 4. Round 3 asked for the per-page rate (process change 1), and the harness still prints one total.

**What it unblocks.** A short list of pages where a correct rejection comes with the wrong rule. Those are candidate rule-boundary findings (decision 0038).

**Cost.** Small.

**Risk.** Low. A front end can raise the rate by naming rules loosely. The rate is a signal for review, not a target.

### 7. Result sets for choice points

**What it is.**
- SEQ-1 explores every argument order up to the bound.
- `tidal-fe conform` lists each test whose explored runs give more than one outcome, with the set.
- For a test with a choice point, the harness compares the set with the matrix results in `tests/RESULTS.md`. A matrix result outside the set is a finding.
- The SEQ-4 check covers overloaded operators and the arguments of `new`.

**Evidence.** Section 1, place 5. Decisions 0047 and 0048 promise the set. Now all 44 explored runs agree, so the current tests do not exercise a set with two members.

**What it unblocks.** An executable check of the `unspecified` rules. Tests with two allowed outcomes, such as the GCC and Clang argument orders, become checkable (general knowledge: the two compilers evaluate function arguments in different orders on x86-64).

**Cost.** Medium.

**Risk.** Medium. Paths grow fast, so the bound must stay and the output must say when the bound stopped a run.

### 8. Check-side round on the missed rejects

**What it is.** Close the 212 missed rejects and the 106 check-side unsupported tests, grouped by rule: list initialization (LIST-13, LIST-14), conditional operator (COND-3, COND-9), instantiation (INST-6, INST-12), access (ACC-15, ACC-3), linkage specifications (8 tests), the `same` idiom (28), and conversion to a function reference (12).

**Evidence.** The failures by page table. No page has more than 9 missed rejects, so the work is a long tail.

**What it unblocks.** A front end that follows the text on every finished page, which makes item 5's split cleaner.

**Cost.** Large, for a modest gain per test.

**Risk.** Low. Each group that the front end cannot close from the text becomes a G finding.

### Dependency sketch

```
1 target profile ──> 2 front-end round 15 ──> 3 Write programs as tests
                 └──> 7 result sets
4 run witnesses (independent; needs 2 for VOL-6 and STG-18)
5 Predict batches ──> 8 check-side round
6 rule-ID scoring (independent, small)
```

Items 1, 4, and 6 are small or medium and can start at once. Item 3 can start at once for the 47 programs that already run, and add the rest as item 2 closes them.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| The launder operation and `<new>` | the general library | `W15/optional.cpp` keeps the pointer that `new` returns, so the Write suites did not need the operation. STG-19 keeps one way out that no test exercises. | Keep out, as decision 0051 says. |
| Named escapes `\N{...}` | lexical structure beyond G1 | One test needs a Unicode name table. The table is data, not rules. | Keep the test. The front end may leave it unsupported. |
| Lexical structure beyond G1 | similarity risk | TR-12 works with the current terminals. No phase 6 finding needed a new lexical rule. | Keep out. |
| Preprocessor beyond `#include "file"` | the library comes with real headers | The front end handles the test headers and multi-unit tests. | Keep out. |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them. | Keep out. |
| Extended floating-point types | little value | `long double` now runs as the x87 format. No test needs the others. | Keep out. |
| `typeid` | needs a library shape | One test uses it. | Keep out. |
| Later language versions (Q-0007) | the charter fixes C++23 | An index of later changes would not change what the front end must do. | Keep out of the front end's work. |
| MSVC | not in the matrix | A second target ABI would double item 1's tables. | Keep out until item 1 is done. |

## 4. Process changes

1. **Fail on new tests that the front end fails, or list them.**
   - `frontend_check.py` passes when a new test fails, and `--update` writes the failure into the baseline. The repair commit `3d98c93` added `current-instantiation-partial-alias.cpp`, which is a false reject at HEAD, and `da2285f` added `implicit-objects-no-set-sanitizer.cpp`, which the interpreter misses.
   - The check could print the new failing tests in its summary line, and `status.md` could list them. Each one is either a front-end ticket or a text finding, and the commit that adds the test should say which.
2. **Keep `exercises.md` current.** Its front-end row says "round 10 done: 4918/5480 pass". `status.md` has the current numbers (5595 of 6032, 5708 check). `status_table.py` could write both.
3. **Report the named-rule rate per page** (item 6). Round 3 asked for this, and the harness still prints one total.
4. **Report the run side in one line.** `status.md` gives the full-pass count (5595) and the check count (5708), and no run counts. Add the run line: 1286 of 1411 runs pass, 115 are unsupported.
5. **Record what decision 0051 promised and phase 6 did not do.** The Write programs did not become run tests. The roadmap marks items R0, G, W, P, and D without "Done", while the commit log shows each done. The roadmap table should match the log, and the next decision should carry item 3 forward or drop it.
6. **Run finished Write programs in the front-end check.** Two false rejects appeared only when I ran the programs by hand. Until item 3 lands, the front-end report can list each program's interpreter result, as report-14 does for W7 and W10 only.
7. **Keep the front-end report form.** Each round ends with G findings and the conformance tables. A table of `undefined` rules with their witnesses, as report-9 had, would let the next round measure item 4 directly.

# Front end, round 18: the harness judges by the specification

Date: 2026-11-03. Decisions 0029, 0035, 0047, 0048, 0055 (phase 8, item HN).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `frontend/`, and `tools/`, `reports/clean-room/roadmap-5/implementer.md` (section 1 place 1, item 1, process change 1), decisions 0047 and 0048, and report 17. No compiler was run, and no crates were added. The changed files are `frontend/src/conform.rs`, `frontend/src/interp/expr.rs`, `frontend/src/interp/run.rs`, `frontend/src/main.rs`, `frontend/README.md`, and `tools/frontend_check.py`. No file was added.

A clean-room writer added and changed tests during the round. The committed baseline `conformance.tsv` has 6337 tests. The suite had 6347 tests at the measured run: 11 tests are new, and `exceptions/unwinding/return-local-destructor-throws-caught-inside.cpp` was removed. Four of the new tests are missed rejects under both front ends (section 1). The suite changed again after the measured run, so a later run can count a few tests more or fewer.

## 1. Conformance

### Method

The front end of commit `dc90663` was built in a scratch directory. Both front ends ran `conform -v` on one copy of `tests/` and `specification/`, taken at one time, so the before and after counts cover the same 6347 tests. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, or `pass-partial`. A passed check is a full pass or a `run-` status.

### Counts

| Measure | Baseline `conformance.tsv` (6337 tests) | Before, same 6347 tests | After, 6347 tests |
|---|---|---|---|
| Full pass | 6068 | 6074 | 5986 |
| Check pass | 6134 | 6140 | 6056 |

Status by test, before and after, on the 6347 tests:

| Status | Before | After |
|---|---|---|
| pass | 5935 | 5832 |
| EDG-ONLY (differs from EDG only, a pass) | 139 | 135 |
| EDG-ALLOWED (agrees with EDG, the rules allow it, a pass) | | 15 |
| pass-partial (a pass whose explored set is partial) | | 4 |
| EDG-AGREES (agrees with EDG against the base, a failure) | | 84 |
| RUN-OTHER-OUTCOME | | 0 |
| FALSE-REJECT | 4 | 4 |
| MISSED-REJECT | 125 | 125 |
| unsupported | 78 | 78 |
| run-unsupported | 65 | 69 |
| RUN-WRONG-RESULT | 1 | 1 |

Full passes fall by 88 and check passes by 84. The check table of `conform`:

| Directory | Tests | Pass | False reject | Missed reject | Unsupported | Differs from EDG only | Agrees with EDG only | EDG, allowed |
|---|---|---|---|---|---|---|---|---|
| classes | 1102 | 1047 | 0 | 19 | 6 | 9 | 21 | 0 |
| declarations | 1183 | 1112 | 2 | 16 | 9 | 25 | 13 | 6 |
| exceptions | 216 | 214 | 0 | 2 | 0 | 0 | 0 | 0 |
| expressions | 1380 | 1295 | 2 | 20 | 17 | 36 | 6 | 4 |
| foundations | 1010 | 932 | 0 | 15 | 23 | 20 | 15 | 5 |
| programs | 67 | 63 | 0 | 0 | 0 | 4 | 0 | 0 |
| statements | 253 | 244 | 0 | 3 | 0 | 2 | 4 | 0 |
| templates | 1136 | 995 | 0 | 50 | 23 | 43 | 25 | 0 |
| **total** | 6347 | 5902 | 4 | 125 | 78 | 139 | 84 | 15 |

The run table:

| Directory | Tests | Pass | Pass, partial | Other outcome | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|---|---|
| classes | 328 | 313 | 0 | 0 | 0 | 0 | 0 | 15 | 0 |
| declarations | 241 | 232 | 0 | 0 | 0 | 0 | 0 | 8 | 1 |
| exceptions | 151 | 139 | 0 | 0 | 1 | 0 | 0 | 11 | 0 |
| expressions | 363 | 350 | 0 | 0 | 0 | 0 | 0 | 13 | 0 |
| foundations | 203 | 197 | 0 | 0 | 0 | 0 | 0 | 6 | 0 |
| programs | 67 | 67 | 4 | 0 | 0 | 0 | 0 | 0 | 0 |
| statements | 103 | 96 | 0 | 0 | 0 | 0 | 0 | 7 | 0 |
| templates | 110 | 99 | 0 | 0 | 0 | 0 | 0 | 9 | 2 |
| **total** | 1566 | 1493 | 4 | 0 | 1 | 0 | 0 | 69 | 3 |

"Pass, partial" tests are in "Pass" too.

The count of passing rejects whose diagnostic names one of the test's rules is now 1680, and was 1655. The count now takes the passing rejects by the base expectation. It adds the `EDG-ONLY` rejects and leaves out the rejects that only agree with EDG.

### The check against the baseline

`python3 -I tools/frontend_check.py` (without `--update`), against the committed baseline (6337 tests), on the live tree:

```
frontend: 5985/6345 pass, 6055/6345 pass their check, 88 worse, 0 better, 19 moved between passing statuses, 8 new (3 not passing)
frontend: 14/14 interpreter-only expectations met
```

The check exits 1. The 88 are the 84 `EDG-AGREES` tests and the 4 `run-unsupported` tests below. The 19 are the 15 `EDG-ALLOWED` tests and the 4 `pass-partial` programs. The tree had 6345 tests at this run because the other writer was still changing tests. The integrator runs `python3 -I tools/frontend_check.py --update` to record the new statuses. This round does not change `conformance.tsv`.

### Tests that change status

Every change, on the 6347 tests:

| Change | Tests |
|---|---|
| pass, now EDG-AGREES | 84 |
| pass, now EDG-ALLOWED | 15 |
| EDG-ONLY, now run-unsupported | 4 |
| pass, now pass-partial | 4 |

**pass, now EDG-AGREES, base `reject` (60).** The front end accepts, as EDG does. Each is a missed reject by the specification's answer:
`classes/access/using-reduces-access.cpp` (ACC-2), `classes/comparisons/out-of-class-reference-member-would-delete.cpp` (DCMP-8), `classes/comparisons/out-of-class-would-delete.cpp` (DCMP-8), `classes/constructors/base-init-calls-member-of-member-constant.cpp` (CTOR-12 OBJ-2), `classes/constructors/const-object-only-anonymous-unions.cpp` (CTOR-24), `classes/constructors/delete-array-deleted-dtor.cpp` (DTOR-3), `classes/constructors/double-destroy-constant.cpp` (DTOR-7), `classes/constructors/double-destroy-trivial-constant.cpp` (DTOR-7), `classes/constructors/nested-default-member-init-before-complete.cpp` (CTOR-18), `classes/constructors/nested-default-member-init-read-in-default-argument.cpp` (CTOR-18), `classes/constructors/parameter-destroyed-by-callee-constant.cpp` (DTOR-7 VC-10), `classes/constructors/reference-default-member-binds-temporary.cpp` (CTOR-17), `classes/constructors/reference-member-binds-converted-parameter.cpp` (CTOR-17), `classes/constructors/reference-member-binds-temporary.cpp` (CTOR-17), `classes/copy-move/trivially-copyable-member-template-selected.cpp` (CPY-27 UNI-11), `classes/members/explicit-object-default-argument.cpp` (CLS-14), `classes/unions/bitfield-width-scoped-enumeration.cpp` (BF-2), `classes/unions/user-assignment-not-activating-constant.cpp` (UNI-6), `classes/virtual/explicit-object-overrides.cpp` (VIRT-19), `declarations/alignment/enum.cpp` (ALN-7), `declarations/alignment/operand-scoped-enumeration.cpp` (ALN-8), `declarations/alignment/scoped-enum.cpp` (ALN-7), `declarations/exception-specifications/narrowing-operand.cpp` (EXS-7), `declarations/exception-specifications/pointer-operand.cpp` (EXS-7), `declarations/initialization/list/paren-no-brace-elision.cpp` (LIST-9), `declarations/initialization/references/aggregate-member-paren-not-extended.cpp` (REF-5), `declarations/structured-bindings/constrained.cpp` (SB-15), `declarations/volatile/static-const-volatile-in-class.cpp` (CLS-23), `expressions/constant-expressions/allocator-result-other-type.cpp` (CE-4), `expressions/constant-expressions/constant-initializer-trivial-class-block-static.cpp` (CE-24 CE-21 CINIT-1), `expressions/constant-expressions/constant-initializer-zero-trivial-class.cpp` (CE-24 CE-21 CINIT-1), `expressions/constant-expressions/explicit-condition-narrowing.cpp` (CE-19), `foundations/objects/scalar-write-in-own-initializer-constant.cpp` (OBJ-1 OBJ-2), `foundations/objects/scalar-write-through-pointer-in-own-initializer-constant.cpp` (OBJ-1 OBJ-2), `foundations/odr/class-constant-copy-in-lambda.cpp` (ODR-2 ODR-3), `foundations/odr/local-class-binds-constant.cpp` (ODR-2), `foundations/odr/member-read-beside-mutable-in-lambda.cpp` (ODR-2 ODR-3), `foundations/odr/mutable-member-read-in-lambda.cpp` (ODR-2), `foundations/overload-resolution/default-from-two-scopes.cpp` (OVL-38), `foundations/overload-resolution/tie-constraints-ellipsis.cpp` (OVL-30 OVL-36), `foundations/translation/main-wrong-parameter-types.cpp` (TR-3 FN-6), `statements/range-for/data-member-begin-counts.cpp` (RFOR-7), `templates/concepts/placeholder-structured-binding.cpp` (CPT-37), `templates/constraints/friend-distinct-from-namespace.cpp` (CNS-32 CNS-24), `templates/constraints/friend-template-declaration.cpp` (CNS-31), `templates/constraints/template-template-more-constrained.cpp` (CNS-25), `templates/dependent-names/template-keyword-destructor.cpp` (DEP-34), `templates/dependent-names/template-keyword-destructor-dot.cpp` (DEP-34), `templates/dependent-names/template-keyword-in-declarator.cpp` (DEP-27), `templates/dependent-names/template-keyword-in-declarator-static.cpp` (DEP-27), `templates/instantiation/explicit-array-bound-differs.cpp` (INST-22), `templates/instantiation/explicit-declaration-internal-class.cpp` (INST-29), `templates/instantiation/explicit-declaration-internal-variable.cpp` (INST-29), `templates/instantiation/explicit-unqualified-outside-namespace-variable.cpp` (INST-21), `templates/instantiation/function-unevaluated-braced-untaken.cpp` (INST-7 INST-6), `templates/instantiation/member-partial-specializations-collide.cpp` (INST-6), `templates/ordering/constraints-conversion-results-differ.cpp` (ORD-13 ORD-15), `templates/ordering/member-object-rvalue.cpp` (ORD-6 ORD-15), `templates/packs/expansion-to-concept-parameters.cpp` (PACK-11), `templates/specialization/partial-reordered.cpp` (TSP-11).

**pass, now EDG-AGREES, base `accept` or `run` (24).** The front end rejects, as EDG does. Each is a false reject by the specification's answer:
`classes/members/standard-layout-address-constant.cpp` (CLS-30), `classes/unions/anonymous-union-unnamed-namespace.cpp` (UNI-9), `declarations/initialization/list/narrowing-bit-field-fits.cpp` (LIST-2), `declarations/initialization/list/narrowing-unscoped-enum-values-fit.cpp` (LIST-2), `declarations/storage-linkage/anonymous-union-unnamed-namespace.cpp` (STO-11), `expressions/new-delete/constant-evaluation-nothrow.cpp` (NEW-37 LSUP-6), `expressions/new-delete/throw-placement-deleted-function-and-template.cpp` (NEW-24 DED-39), `foundations/conversion-sequences/initializer-list-over-kind.cpp` (ICS-22 ICS-21), `foundations/conversion-sequences/qualification-array-bound.cpp` (ICS-31), `foundations/overload-resolution/address-more-constrained.cpp` (OVL-19), `foundations/overload-resolution/address-no-target-constraints.cpp` (OVL-20), `foundations/overload-resolution/address-no-target-more-constrained.cpp` (OVL-20), `foundations/overload-resolution/tie-conversion-result-noexcept.cpp` (OVL-26), `statements/jumps/goto-past-nontrivial-destructor.cpp` (JMP-6), `statements/jumps/goto-skips-nontrivial-destructor.cpp` (JMP-6), `statements/switch/case-skips-nontrivial-destructor.cpp` (SW-5), `templates/class-template-deduction/list-single-specialization-alias.cpp` (CTAD-18), `templates/constraints/lambda-conversion.cpp` (CNS-18), `templates/constraints/lambda-conversion-implicit.cpp` (CNS-18), `templates/deduction/match-noexcept.cpp` (DED-11), `templates/instantiation/class-inline-static-member-unused.cpp` (INST-5), `templates/ordering/reversed-member-object.cpp` (ORD-7 ORD-6), and the two `run` tests `declarations/namespaces/qualified-definition-inline-member-inside-named.cpp` (NS-15 NS-3) and `templates/instantiation/function-point-constexpr-later.cpp` (INST-32 INST-31).

**pass, now EDG-ALLOWED (15).** Every rule these tests cite is `impl-defined`, `unspecified`, `no-diagnostic`, or `conditionally-supported`, so they still pass. Base `reject`, the front end accepts: `declarations/constexpr/constinit-added-late.cpp` (CINIT-3), the four `declarations/functions/definitions/main-four-params*.cpp` and `main-wrong-first-param*.cpp` tests (FN-6), `foundations/translation/main-wrong-parameter-types-ilp32.cpp` (FN-6), and the four `foundations/translation/limit-*-lp64.cpp` tests (TR-14, TR-15). Base `accept`, the front end rejects: `declarations/attributes/vendor-attribute-changes-layout.cpp` (ATTR-20), the two `expressions/conversions/floating-narrowing-infinity*.cpp` tests (CONV-11), and the two `expressions/primary/char-encoding-*.cpp` tests (PRIM-32).

**EDG-ONLY, now run-unsupported (4).** These `run` tests have an EDG override of `reject`. The front end accepts, and the old harness did not run them. They now run, and the interpreter meets a construct it does not model:
`declarations/storage-linkage/extern-in-block-hidden.cpp` (a block-scope `static` before its declaration ran), `expressions/new-delete/throw-placement-templates-unordered.cpp` (a `new` expression), `statements/range-for/callable-data-members.cpp` (a range-based `for`), `templates/dependent-names/dependent-base-conversion-function.cpp` (a call of a callee it cannot find).

**pass, now pass-partial (4).** `programs/W0/calc.cpp`, `programs/W3/nested.cpp`, and `programs/W4/grid.cpp` stop at the bound of 64 runs. `programs/W5/recursion.cpp` settles a line after two runs. Every explored run gives 0.

**The OPR-7 case.** `classes/operators/subscript-multi-order.cpp` passes before and after, through two changes. The new harness on the old interpreter gives `RUN-OTHER-OUTCOME` for the test. The second explored run evaluates the three arguments of `operator[]` from right to left and gives result 1. The fixed interpreter keeps the order, so the run has no choice point and gives 0.

**New tests.** The 11 tests new since the baseline have the same status under both front ends. Seven pass. Four are missed rejects: `declarations/constexpr/constinit-added-late-thread-local.cpp`, `exceptions/unwinding/placement-new-global-usual-delete-rejected.cpp`, `templates/dependent-names/binary-operator-pack-never-fits.cpp`, and `templates/instantiation/class-point-not-later-type.cpp`.

The front end's answers did not change on any test in this round. Every change in the counts comes from the harness. The one interpreter change shows only under the new harness, on `subscript-multi-order.cpp`.

## 2. What changed

### The harness (`frontend/src/conform.rs`)

- Each test is judged by its base `tidal-expect` line. The `tidal-expect-edg` override names a second answer.
- An answer that matches the base line is `pass`, or `EDG-ONLY` when the EDG override differs.
- An answer that matches only the EDG override is `EDG-AGREES`, a failure. The status is `EDG-ALLOWED`, a pass, when every rule the test cites has one of the kinds `impl-defined`, `unspecified`, `no-diagnostic`, or `conditionally-supported`. The harness reads each rule's kind from the `:kind:` line under its `:::{rule}` header in `../specification` beside the test directory, or in the directory that `--spec DIR` names.
- Three tests have no base line, only compiler lines that differ: `declarations/declarators/array-too-large.cpp`, `expressions/conversions/floating-narrowing-out-of-range.cpp`, and `expressions/multiplicative/floating-overflow-constant.cpp`. Either answer passes them, as before (G135).
- The harness keeps every explored run (decisions 0047, 0048). The first run is judged as before. When the first run passes, each other run is judged by the same expectation, and an outcome that the expectation does not allow gives `RUN-OTHER-OUTCOME`. When the reported outcome is a stop at undefined behavior that the test expects, the other runs do not matter.
- A pass whose exploration is partial gives `pass-partial`. The exploration is partial when the bound of 64 runs stopped it, when a run with more than 4096 choice points settled a line, or when another run is unsupported. The run line says which, for example "explored: 64 runs, partial: the bound of 64 runs stopped the exploration".
- A run that passes with more than one explored run says "explored: N runs" on its run line.
- A test with status `EDG-ONLY` now runs and gets a run status. Before, only `pass` tests did.
- The check table has two more columns, "Agrees with EDG only" and "EDG, allowed". The run table has "Pass, partial" and "Other outcome".
- `-v` lists every test whose status is not exactly `pass`, so `pass-partial` lines appear.

### The interpreter

- `interp/expr.rs`, OPR-7: the flag that keeps the order of an operator's operands is set just before the call's arguments are evaluated. Before, the flag was set before the object expression. A call in the object expression (`pick(t, g)` in the test) took the flag, so the arguments of a multi-argument `operator[]` became a SEQ-1 choice point.
- `interp/expr.rs`, SEQ-1: a call whose arguments have at least two side effects explores every order of its arguments. Each run picks the arguments one at a time, with as many options as arguments are left. Option 0 takes the first argument left, and option `j` the `j`-th from the end, so the second run of a call reverses the order as before. `parameter-destruction-order-three.cpp` now explores 6 runs, and was 2. The bound stays at 64 runs.
- `interp/run.rs`: `run_explored` returns the reason a set is partial (the bound or a settled line) in place of a flag for the bound.
- `main.rs`: `tidal-fe run` prints that reason on its "explored" line, with "the set is partial".

### `tools/frontend_check.py`

- `EDG-ALLOWED` and `pass-partial` are passing statuses. `EDG-AGREES` has rank 0, a failed check. `RUN-OTHER-OUTCOME` has rank 1, a passed check with a failed run.
- The check fails when a test moves to a failing status: from a passing status, and from one failing status to another. A move between two passing statuses is listed and does not fail.
- The baseline keeps its form: a status and a test path, separated by a tab. The reader ignores any later column, so the current file reads as before.

## 3. Items left, with reasons

- The 84 `EDG-AGREES` tests go to the next front-end round (phase 8, item FE).
- The 4 `run-unsupported` tests above join the run constructs that item FE names.
- The harness does not compare a run's set of outcomes with a list of allowed results per `unspecified` rule. Each test gives one expected status for every allowed choice, so every explored outcome must meet that status. A list of allowed results waits for the rules to give one.
- The 22 `unspecified` rules with no choice point in the interpreter are not labelled.
- SEQ-8 still has two options per operator.

## 4. Findings

### G134. A test does not say which of its rules allows EDG's answer

- Page and rules: decision 0047 ("A test's override for a report compiler names the rule that allows the result"); tests `foundations/translation/main-wrong-parameter-types.cpp` (TR-3 FN-6), `templates/instantiation/function-point-constexpr-later.cpp` (INST-32 INST-31), `classes/constructors/parameter-destroyed-by-callee-constant.cpp` (DTOR-7 VC-10).
- Kind: missing.
- What happened: the harness passes an answer that agrees with EDG against the base line only when every rule the test cites allows a second answer. These three tests cite one rule that can allow a second answer and one that cannot. The note of `function-point-constexpr-later.cpp` says that INST-32 allows any result, and the test also cites INST-31, a `required` rule. The note of `main-wrong-parameter-types.cpp` names Q-0024, which FN-6 cites, and the test also cites TR-3. The harness cannot tell which rule decides EDG's answer, so all three are `EDG-AGREES`.
- Suggestion: a header line on an EDG override that names the rule that allows EDG's answer, or says that EDG's answer is a defect. The harness can then use that rule in place of the test's whole rule list.

### G135. Three tests have no base expectation

- Page and rules: MUL-12 and CE-9 (Q-0001), CONV-11, TR-14; tests `expressions/multiplicative/floating-overflow-constant.cpp`, `expressions/conversions/floating-narrowing-out-of-range.cpp`, `declarations/declarators/array-too-large.cpp`.
- Kind: ambiguous.
- What happened: each test has only compiler lines, and the compilers differ, so the harness passes either answer. CONV-11 and TR-14 are `impl-defined`, so either answer fits them. MUL-12 is `undefined`, and CE-9 says that constant evaluation must not evaluate an operation with undefined behavior. By that reading the specification's answer for `floating-overflow-constant.cpp` is reject, and Clang's accept is outside it. MUL-12 is provisional under Q-0001, and the test does not say whether the specification gives one answer here.
- Suggestion: give `floating-overflow-constant.cpp` a base line once Q-0001 settles, or say in its note that the rules allow both answers. The harness then judges it like the other tests.

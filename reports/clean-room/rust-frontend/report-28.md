# Front end, round 28: the T6, GR, and F7 tests, the machine group, and two new marks

Date: 2026-11-11. Decisions 0029, 0035, 0060, 0064, 0065 (phase 17, item FE).
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decisions 0035, 0060, 0064, and 0065, `documentation/exercises.md` section "Front end", `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/DESIGN-rewrite.md` section 5, `frontend/TEMPLATE-MODEL.md`, the marks file `reports/clean-room/rust-frontend/rewrite-marks.tsv`, report 27, `reports/clean-room/roadmap-14/implementer.md` item 3 and `programmer.md` item 3, the front-end ledger, `reports/clean-room/ledger.md`, and the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`. No compiler was run, and no crates were added. The changed files are `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md` (section 6), `reports/clean-room/rust-frontend/conformance.tsv`, `reports/clean-room/rust-frontend/rewrite-marks.tsv`, `reports/clean-room/rust-frontend/ledger.md` (row G183), this report, and 11 files under `frontend/src/`: `cons.rs`, `dcmp.rs`, `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `ics.rs`, `parser.rs`, `templ.rs`, `interp/expr.rs`, and `interp/mod.rs`. `consteval.rs`, `lexer.rs`, `diag.rs`, `main.rs`, and `conform.rs` did not change. No file was added under `frontend/`, and nothing under `frontend/next/` changed.

The reader returned this report as a message, and the integrator saved it here.

The suite grew during the round. At the start, the tree held the T6 tests of commit 3bb416e8 and the GR part 1 tests of commit 1c223631 (7106 tests). Item F7 (b511838d) added 25 tests and changed the text of OBJ-1, OBJ-2, OBJ-3, JMP-6, JMP-7, JMP-9, JMP-14, LOOP-3, RFOR-4, FN-7, FN-12, INST-1, INST-30, CE-3, CE-22, and ATTR-2. So the suite went from 7106 tests to 7131, and this round read the changed text.

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each group of changes, on the live tree, each under the run lock. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, `pass-partial`, or `NO-DIAGNOSTIC-ALLOWED`. A passed check is a full pass or a `run-` status. At the end of the round, `tools/frontend_check.py --update` wrote the new baseline to `conformance.tsv`.

The start counts on the start suite come from the first run of the round, on the front end of HEAD (3bb416e8). The F7 tests arrived after the T6 and GR groups were done. Five of them failed on arrival (section 4). The column "start code on the final suite" adds those five to the start counts. It assumes that the other 20 F7 tests pass on the start code as well, which this round did not measure.

### Counts

| Measure | Start of round, start code (7106 tests) | Start code on the final suite (7131 tests, estimate) | End of round (7131 tests) |
|---|---|---|---|
| Full pass | 6981 | 6981 + 20 | 7039 |
| Check pass | 7006 | 7006 + 20 | 7058 |
| Tests that fail their check (completion line) | 91 | 96 | 64 |
| Of these, marked, with the 36 marks the round started with | 36 | 36 | 36 |
| Of these, counted toward the gate, with the 36 starting marks | 55 | 60 | 28 |
| Of these, marked, with the 38 marks at the end | | | 38 |
| Of these, counted toward the gate, with the 38 marks at the end | | | 26 |
| Program tests that do not run to their result | 6 of 178 | 6 of 178 | 1 of 178 |
| Tests that pass their check and stop in a run | 25 | 25 | 19 |

Status by test at the end (7131): pass 6679, EDG-ONLY 309, EDG-ALLOWED 10, pass-partial 35, NO-DIAGNOSTIC-ALLOWED 6 (all passes); EDG-AGREES 6, FALSE-REJECT 7, MISSED-REJECT 35, unsupported 25, run-unsupported 19. The failing statuses that are not run stops and not `no-diagnostic` cases are the 64 tests of the completion line and the 9 tests out of scope.

Passing rejects whose first error names a cited or linked rule: 2154 of 2782 in report 27, 2176 of 2802 at the end of this round.

### The checks

`python3 -I tools/frontend_check.py`, before `--update`:

```
frontend: 7039/7131 pass, 7058/7131 pass their check, 0 worse, 16 better, 0 moved between passing statuses, 58 new (2 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 64 tests fail their check, of which 36 need the template model (marks file) and 28 count toward the gate of decisions 0062 and 0063; 1/178 program tests do not run to their result; 9 tests out of scope
frontend: 2176/2802 passing rejects name a cited rule (1987) or a linked rule (189) in their first error; `tidal-fe conform` gives the table per page
```

That run read the marks file before this round's two new rows. With them, the line reads 38 marked and 26 toward the gate. The two new tests that do not pass are the two new marks (section 8).

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 193 examples (191 pass, 2 unsupported); 0 worse, 0 better, 22 new (1 not passing), 6 gone
```

Both checks exit 0. One run inside the round reported a test as worse, and the next change repaired it before any other change. The first form of the CE-3 check on pointer arithmetic (section 5) stopped `expressions/additive/pointer-difference-largest-ilp32.cpp`, whose 2 GiB array keeps a value that is not usable as one value. That form was not written to the baseline.

### The rewrite

This round changed no file that the rewrite moved (`lexer.rs`, `diag.rs`, `main.rs`, `conform.rs`), so it made no twin change in `frontend/next/` and did not run `--next`. `consteval.rs` did not change. `interp/expr.rs` and `interp/mod.rs` changed (section 5). Build step 3 copies them, so the copy should start from this round's versions.

## 2. Group 1: the T6 tests

Each T6 rule test and program try passes its check and its run, except the template form of the `static constexpr` member and `r14-fsm-second-try.cpp`, which are marked (section 8).

- **LIST-12, LIST-4, CPT-26, IF-13.** `requirement-too-many-values-nested-aggregate.cpp` and `constexpr-requirement-too-many-values-discards-binding.cpp`. Brace elision into a nested aggregate counted the values, and when values were left after the last member, the front end gave up with an unknown result. Values left over after brace elision are now a LIST-12 error, so the requirement is false and the discarded branch is not checked. The same fault was a missed reject outside templates (a list of three values for a struct of a `short` and an aggregate of one `int`).
- **LIST-4 with conversion functions (G183).** `r14-fields-first-try.cpp` and `r14-fields-second-try.cpp`. These count fields with an object of a class whose conversion function template gives any type. The front end now treats a value of another class that converts to a member's class type as a value that initializes the member whole, and does not elide braces for it. A user-defined conversion to an aggregate class from a conversion function template is now formed (DED-31), and the member then takes the converted value. Section 10 records the doubt about LIST-4's wording.
- **CE-16, CLS-2.** `static-constexpr-member-calls-own-member.cpp`. In a class that is not a template and not inside one, a call in a constant evaluation of a member function whose body waits for the end of the outermost class now names CE-16: the class is incomplete there, so the function has only its declaration.
- **LAM-5, LAM-18.** The four `generic-reference-capture-*.cpp` tests. The body of a generic lambda keeps a dependent call as an unknown expression, so a variable named only in such a call was not captured by `[&]` or `[=]`. The parser now records each variable that an id-expression inside a lambda names in an evaluated operand, and a capture default captures those variables too.
- **CTOR-10, ICS-9.** `closure-argument-converting-constructor-template.cpp` and `closure-lvalue-converting-constructor-template-thunk.cpp`. A closure with no capture has a conversion function to a pointer to function, so the copy-initialization of a class parameter from it took the path for sources with conversion functions, and that path never called a constructor. When the user-defined conversion that wins is a converting constructor, the constructor call is now formed.
- **TPL-11, CTAD-1.** `constant-parameter-class-template-string-literal.cpp`, `r14-keys-first-try.cpp`, and four earlier gate tests (section 5). A constant template parameter whose type is a class template name alone was an unknown type. It is now a placeholder whose argument deduces the class template's arguments, as for a variable copy-initialized from the argument. A literal operator template with such a parameter takes a string literal (OPR-8, OPR-10).
- **Program tries.** `r14-fnref-fifth-try.cpp` and `r14-fnref-sixth-try.cpp` follow from the LAM-5 and CTOR-10 items. `r14-fields-*` and `r14-keys-first-try.cpp` follow from the items above.

## 3. Group 2: the GR tests

Seven of the nine GR part 1 tests passed at the start. `expressions/decltype/member-initializer-base.cpp` and `member-initializer-base-braces.cpp` (DTY-3) were false rejects: the member initializer list took no `decltype_spec` as the name of a base. The `mem_init_id` now reads a `decltype_spec` that gives a class, with parentheses or braces after it.

## 4. Group 3: the F7 tests

- **ATTR-2, ATTR-19 (F7-1).** `maybe-unused-on-type.cpp`. `maybe_unused` after a type specifier applies to the type, and is now an error there.
- **FN-7 (X24-4).** `flow-off-end-goto-out-of-loop-warns.cpp`. A loop whose condition is a constant `true` now completes when a `goto` in its body names a label outside it. A label at the top level of a block is now reached by the statement before it or by any `goto` to it, so control can reach the end after such a label. A block that holds a label at a deeper level still gives no warning.
- **INST-30, CLS-2 (RV10-R-1).** `noexcept-operand-sees-complete-class.cpp`. The operand of a member's `noexcept` waits for the end of the class when it fails at its place. The trial parse made `Holder<Coin>` while `Coin` was incomplete, and that specialization kept the errors. An operand that names the class now waits without a trial parse.
- **CE-3 (Q-0449).** `mutable-member-read-outside.cpp` and `struct-copy-mutable-member-outside.cpp` were EDG-AGREES. In constant evaluation, a read of a `mutable` member of an object that the evaluation did not create, and a trivial copy of a struct with a `mutable` member from such an object, are now CE-3 errors. A union keeps the CE-22 rule of report 27, so `union-copy-inactive-mutable-member-outside.cpp` passes.

The other 20 F7 tests passed on arrival.

## 5. Group 4: the machine and constant evaluation, and other failures

Implementer item 3 asks for the constant-evaluation groups before build step 3 copies `consteval.rs` and `interp/`. Each of the six closes:

| Group | Tests | What changed |
|---|---|---|
| PRIM-27, PRIM-28 | `expressions/primary/fold-expansion.cpp` | A fold over `.*` or `->*` builds the pointer-to-member operator. |
| CE-3 | `foundations/constant-evaluation/read-extended-temporary-not-const.cpp` | The temporary that a `constexpr` reference extends is usable only when its type is `const`. The value of another such temporary is now not usable. |
| CE-3 | `foundations/constant-evaluation/reference-to-local-not-usable.cpp` | Pointer arithmetic over the object of a reference that is not usable names CE-3, when the object has no known bound. |
| LSUP-13 | `foundations/language-support/ordering-defaulted-specified-needs-conversion.cpp` | A defaulted `<=>` whose part gives an ordering class with no declared conversion to the return type is now an LSUP-13 error. `ordering-defaulted-needs-conversion.cpp` now names LSUP-13 in place of DCMP-11. |
| OVL-40 | `overloading/overload-resolution/init-direct-explicit-conversion.cpp` | A direct-initialization of a class takes an explicit conversion function of the source class that gives the class (ICS-45). |
| CTOR-12 | `classes/constructors/base-init-dynamic-cast-early-constant.cpp` | The object under construction, before its bases are initialized, as the operand of `dynamic_cast` is now undefined. |

Other groups:

| Group | Tests | What changed |
|---|---|---|
| TPL-11, CTAD-1 | `templates/declarations/constant-parameter-class-template.cpp`, `templates/class-template-deduction/constant-parameter-guide.cpp`, `templates/class-template-deduction/sites.cpp`, `classes/operators/literal-operator-template-class-parameter.cpp` | Section 2. |
| DCMP-11 | `classes/comparisons/spaceship-member-returns-non-category.cpp` | A defaulted `<=>` with `auto` whose part's `<=>` gives a type that is not a comparison category is deleted, and its use is an error. |
| LAM-6 | `expressions/lambdas/capture-default-in-member-initializer.cpp` | A lambda with a capture default in a default member initializer captures that initializer's `this`. |
| CVF-1, CVF-4, CVF-5 (run stops) | `classes/conversion-functions/call-by-name.cpp`, `conversion-to-void-explicit-call.cpp`, `different-type-not-hidden.cpp` | The parentheses after a conversion-function-id in an expression were read as a function declarator, so `s.operator int()` named the function and did not call it. They are now the call. |
| VIRT-12 (run stop) | `classes/virtual/pure-call-in-ctor-sanitizer.cpp` | A virtual call at run time that reaches a pure virtual function with no definition is now undefined (round 13's pure virtual call). |

## 6. Ledger rows of `reports/clean-room/ledger.md` that this round answers

| Row | Tests | State in this tree |
|---|---|---|
| X24-4 | `declarations/functions/definitions/flow-off-end-goto-out-of-loop-warns.cpp` | pass |
| RV10-R-1 | `templates/instantiation/noexcept-operand-sees-complete-class.cpp` | pass |
| F7-1 | `declarations/attributes/maybe-unused-on-type.cpp` | pass |

## 7. Tests added during the round

58 tests are new against the baseline of report 27: the 33 present at the start of the round (T6, GR part 1, and G181's test), and the 25 of F7. 56 of them pass fully. The two that do not are marked (section 8).

## 8. Decision 0060: the completion test

The completion line of `tools/frontend_check.py`, with the 36 marks the round started with:

```
frontend: completion (decision 0060): 64 tests fail their check, of which 36 need the template model (marks file) and 28 count toward the gate of decisions 0062 and 0063; 1/178 program tests do not run to their result; 9 tests out of scope
```

**The gate's count, as a measure (decision 0064).** With the 36 marks the round started with, 28 failing tests are unmarked, down from 55 on the start suite and an estimated 60 on the final suite. With the 38 marks at the end, 26 are unmarked. No test moved from a passing status to a failing one in the baseline comparison of rounds 27 and 28. No marked test was fixed in the old code.

### Marks added

The marks file gains two rows, and `frontend/TEMPLATE-MODEL.md` section 6 gains the same two (38 tests; 15 in section 3).

| Test | Section | Reason |
|---|---|---|
| `templates/instantiation/class-static-constexpr-member-calls-own-member.cpp` (INST-5) | 3 | The initializer of the `static constexpr` member `max` calls the static member function `compute`. The front end makes the initializer with `Limits<short>`, while the body of `compute` waits for the end of the class, so the evaluation stops. Section 3 makes the initializer at the member's first use, as for the marked `class-static-constexpr-member-initializer-waits.cpp`. Programmer item 3 leaves this form to the rewrite. |
| `programs/roadmap/r14-fsm-second-try.cpp` (INST-5) | 3 | The same cause in a program: the initializer of `table` calls `make_table`. |

No mark was removed. Each of the 36 tests that report 27 marked still fails for the cause that the marks file gives. Lines 5 and 10 of `TEMPLATE-MODEL.md` still say 36 tests, outside section 6, which this reader does not write.

### The 28 tests that count toward the gate

With the 36 starting marks, the 28 tests are 14 missed rejects, 11 unsupported, 1 false reject, and 2 EDG-AGREES. The two new marks are 2 of the unsupported. None of the other 26 is a program test.

| Test | Rules | Status | Cause |
|---|---|---|---|
| `classes/copy-move/trivially-copyable-member-template-selected.cpp` | CPY-27, UNI-11 | EDG-AGREES | The trivially copyable test ignores a member template that overload resolution selects for the copy. |
| `declarations/constexpr/lambda-escalates-runtime.cpp` | CEV-1 | missed reject | No escalation of a lambda's call operator to an immediate function. |
| `declarations/structured-bindings/tuple-member-get-type-parameter-uses-free.cpp` | SB-5 | false reject | The front end uses the member `get`, a template with a type parameter, where SB-5 says the free `get` is used. |
| `expressions/decltype/nested-class-member-evaluated.cpp` | UNEV-1, PRIM-25 | missed reject | No check of a non-static member of an enclosing class named in an evaluated operand of a nested class. |
| `expressions/lambdas/capture-visible-in-trailing-return.cpp` | LAM-18 | unsupported | A capture named in the trailing return type (Q-0092). |
| `expressions/postfix/default-template-argument-incomplete-class.cpp` | PFX-11, CLS-2 | missed reject | No check that a default template argument in a member needs the class complete. |
| `foundations/names/adl-function-template-parameter-adds-nothing.cpp` | NAM-8, NAM-27 | missed reject | Argument-dependent lookup for a function template name adds the namespaces of its parameter types. |
| `overloading/conversion-sequences/overload-set-argument-sequence.cpp` | ICS-47 | unsupported | Overload resolution with an overload set as an argument. |
| `overloading/overload-resolution/address-no-target-auto-template-parameter.cpp` | OVL-20 | missed reject | An overload set as the argument of an `auto` template parameter. |
| `templates/arguments/class-argument-reference-temporary.cpp` | TARG-15 | missed reject | No check that a reference member of a class argument binds to a temporary. |
| `templates/arguments/concept-for-template.cpp` | TARG-7 | missed reject | A concept as the argument of a template template parameter. |
| `templates/arguments/pointer-subobject.cpp` | TARG-14 | unsupported | The address of an array element or a member as a template argument. |
| `templates/class-template-deduction/sites-for-and-condition.cpp` | CTAD-1 | unsupported | Deduction for a range `for` variable and a condition variable. |
| `templates/class-template-deduction/tie-own-constructor-template.cpp` | CTAD-19, CTAD-23 | missed reject | The tie rule between an own constructor template and an inherited constructor. |
| `templates/concepts/concept-id-value.cpp` | CPT-1 | unsupported | A concept-id with a dependent name in its arguments. |
| `templates/concepts/placeholder-parameter-decltype-auto.cpp` | CPT-37 | missed reject | A `decltype(auto)` placeholder parameter with a type constraint. |
| `templates/concepts/placeholder-parameter.cpp` | CPT-20 | unsupported | A constrained placeholder parameter of a generic lambda. |
| `templates/constraints/explicit-instantiation-member-not-satisfied.cpp` | CNS-13 | missed reject | No constraint check on an explicit instantiation of a member. |
| `templates/constraints/friend-distinct-from-namespace.cpp` | CNS-32, CNS-24 | EDG-AGREES | A friend template of a specialization and the namespace template are taken as one template. |
| `templates/constraints/function-type-specialization.cpp` | CNS-30 | unsupported | The type of a specialization of a constrained function template. |
| `templates/declarations/alias-template-self-through-class.cpp` | TPL-31 | missed reject | No check of an alias template that needs itself through a class. |
| `templates/declarations/constant-parameter-no-constexpr-constructor.cpp` | TPL-7 | missed reject | No literal type check on a constant parameter's class type. |
| `templates/ordering/nondeduced-only-parameter.cpp` | ORD-3, ORD-15 | missed reject | Partial ordering with a parameter that appears only in a non-deduced position. |
| `templates/packs/ellipsis-after-parameter.cpp` | PACK-3 | unsupported | `T...` with no pack as a parameter followed by the variable argument part. |
| `templates/packs/expansion-elements.cpp` | PACK-5 | unsupported | An expansion of a template template pack. |
| `templates/specialization/explicit-member-class-template-no-prefix.cpp` | TSP-23 | missed reject | No check of the `template<>` prefixes of an explicit specialization of a member class template. |
| `templates/instantiation/class-static-constexpr-member-calls-own-member.cpp` | INST-5 | unsupported | Marked at the end of the round (above). |
| `programs/roadmap/r14-fsm-second-try.cpp` | INST-5 | unsupported | Marked at the end of the round (above). |

The causes of the first 26 rows are the causes of report 27 section 8. Most of them sit in `templ.rs`, `cons.rs`, `ctad.rs`, and the checks that the rewrite writes again (implementer item 3). SB-5, UNEV-1, LAM-18, and CEV-1 are the exceptions.

Measures for the rewrite: the files over 2000 lines are `expr.rs` (8242), `decl2.rs` (7417), `interp/expr.rs` (4967), `templ.rs` (4389), `decl.rs` (3249), `expr2.rs` (3017), and `parser.rs` (2434), of 53293 lines in all. The first-error rate is 2176 of 2802 passing rejects (78%).

## 9. Run stops

19 tests pass their check and stop in the run, down from 25 at the start. No program test stops in a run. The conversion-function calls by name and the pure virtual call now run (section 5). The 19:

| Test | Rules | Stop |
|---|---|---|
| `classes/comparisons/spaceship-specified-type-synthesized.cpp` | DCMP-4 | an overloaded operator in a synthesized comparison |
| `classes/conversion-functions/deduced-auto.cpp` | CVF-1 | a conversion function with a deduced return type |
| `declarations/initialization/static/reference-constant-initialized.cpp` | SINIT-1 | a reference defined in another unit |
| `declarations/storage-linkage/block-function-declaration.cpp` | LNK-9 | a block-scope static variable used before its declaration ran |
| `exceptions/handlers/dtor-try-block-member-destructor-throws.cpp` | TRY-10 | a destructor function try block |
| `exceptions/handlers/dtor-try-block-rethrows.cpp` | TRY-10 | the same |
| `expressions/new-delete/throw-placement-default-argument.cpp` | NEW-23 | a placement deallocation function with a default argument |
| `expressions/new-delete/throw-placement-templates-unordered.cpp` | NEW-24, DED-39 | placement deallocation function templates |
| `foundations/odr/variable-declaration-forms.cpp` | ODR-15 | a variable defined in another unit |
| `foundations/storage/integer-pointer-no-object-sanitizer.cpp` | UN-7, STG-15 | indirection through a pointer made from an integer |
| `overloading/overload-resolution/deleted-defaulted-move-not-candidate.cpp` | OVL-42 | a class assignment beside a defaulted copy assignment |
| `templates/arguments/constant-overloaded-function.cpp` | TARG-12 | a call through a constant argument chosen from an overload set |
| `templates/deduction/placement-deallocation-match.cpp` | DED-39 | a placement deallocation function template |
| `templates/deduction/placement-deallocation-ordered.cpp` | DED-39, ORD-9 | the same, with ordering |
| `templates/dependent-names/dependent-base-conversion-function.cpp` | DEP-11 | a conversion function of a dependent base |
| `templates/dependent-names/template-keyword-forms.cpp` | DEP-24, DEP-25 | a dependent name |
| `templates/dependent-names/type-only-positions.cpp` | DEP-20 | a dependent name |
| `templates/instantiation/local-entities-per-specialization.cpp` | INST-36 | a call that needs deduction in a local entity |
| `templates/specialization/explicit-class-member-default.cpp` | TSP-22, TSP-8 | a default argument of an explicitly specialized member |

## 10. Findings

**G183 (ambiguous, LIST-4).** LIST-4 says that inner braces are not omitted for a value that can initialize the member directly, and gives an object of the member's type as the example. It does not say whether an object of another class counts when a conversion function, or a conversion function template, gives the member's type. The r14 field-counting tries depend on the answer. Take a struct whose last member is an aggregate, and a value that converts to anything. The count of values that initialize the struct equals its count of members only if such a value initializes that member whole. The front end now counts it as initializing the member whole, which makes both tries run to 0, as their program tests expect. The front-end ledger gains row G183, `open`. A sentence in LIST-4 that names the conversion case, with an example, would close it.

Every other group that this round closed follows from the text, including F7's changes to FN-7, INST-30, CE-3, and ATTR-2 during the round. The groups that stay open have causes in the front end (section 8) or in the template model (the marks file).

Two records sit outside the files this reader writes. `frontend/TEMPLATE-MODEL.md` lines 5 and 10 still give 36 tests. `DESIGN-rewrite.md` section 5 can say that `interp/expr.rs` and `interp/mod.rs` are ready to copy at this round's versions.

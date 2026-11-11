# Front end, round 26: the STG-23 wrong verdict, the T4 and F5 tests, and the failures that the current design can take

Date: 2026-11-11. Decisions 0029, 0035, 0048, 0060, 0062, 0063 (phase 15, item FE).
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decisions 0035, 0060, 0062, and 0063, `documentation/exercises.md` section "Front end", `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md`, the marks file `reports/clean-room/rust-frontend/rewrite-marks.tsv`, report 25, the front-end ledger, `reports/clean-room/ledger.md`, and the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`. No compiler was run, and no crates were added. The changed files are `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md`, `reports/clean-room/rust-frontend/conformance.tsv`, `reports/clean-room/rust-frontend/rewrite-marks.tsv`, this report, and 13 files under `frontend/src/`: `class.rs`, `cons.rs`, `ctad.rs`, `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `layout.rs`, `lexer.rs`, `ovl.rs`, `parser.rs`, `templ.rs`, and `interp/expr.rs`. No file was added under `frontend/`.

The reader returned this report as a message, and the integrator saved it here.

The suite grew during the round. At the start, 51 tests were new against the baseline of report 25 (6938 tests), and 30 of them did not pass. During the round, item RV8 worked in the same tree. It added 18 tests and changed the text of OPR-8, TYN-10, and CTAD-18. So the suite went from 6989 tests at the start of the round to 7007.

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each change, on the live tree, each under `flock build/run_tests.lock`. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, `pass-partial`, or `NO-DIAGNOSTIC-ALLOWED`. A passed check is a full pass or a `run-` status. At the end of the round, `tools/frontend_check.py --update` wrote the new baseline to `conformance.tsv`.

### Counts

| Measure | Baseline of report 25 (6938 tests) | Start of round (6989 tests) | End of round (7007 tests) |
|---|---|---|---|
| Full pass | 6801 | 6822 | 6892 |
| Check pass | 6825 | 6853 | 6921 |
| Tests that fail their check (completion line) | 104 | 127 | 77 |
| Of these, marked for the template model | 30 | 30 | 32 |
| Of these, counted toward the gate | 74 | 97 | 45 |
| Program tests that do not run to their result | 1 of 133 | 7 of 147 | 3 of 147 |

Status by test:

| Status | Baseline of report 25 (6938) | End (7007) |
|---|---|---|
| pass | 6482 | 6559 |
| EDG-ONLY (a pass) | 283 | 290 |
| EDG-ALLOWED (a pass) | 10 | 10 |
| pass-partial (a pass) | 20 | 27 |
| NO-DIAGNOSTIC-ALLOWED (a pass) | 6 | 6 |
| EDG-AGREES (a failure) | 5 | 6 |
| FALSE-REJECT | 7 | 7 |
| MISSED-REJECT | 45 | 37 |
| unsupported | 56 | 36 |
| run-unsupported | 24 | 29 |

The 86 failing statuses at the end are the 77 tests of the completion line and the 9 tests out of scope. At the start of the round, 2 runs stopped at the STG-23 wrong verdict of section 2, and 29 tests stopped in a run.

Passing rejects whose first error names a cited or linked rule: 2115 of 2743 in report 25, 2139 of 2766 at the end of this round.

### The checks

`python3 -I tools/frontend_check.py`, before `--update`:

```
frontend: 6892/7007 pass, 6921/7007 pass their check, 0 worse, 30 better, 0 moved between passing statuses, 69 new (8 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 77 tests fail their check, of which 32 need the template model (marks file) and 45 count toward the gate of decisions 0062 and 0063; 3/147 program tests do not run to their result; 9 tests out of scope
frontend: 2139/2766 passing rejects name a cited rule (1952) or a linked rule (187) in their first error; `tidal-fe conform` gives the table per page
```

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 192 examples (190 pass, 2 unsupported); 0 worse, 0 better, 17 new (1 not passing), 2 gone
```

Both checks exit 0. Two runs inside the round each reported one test as worse, and the next change repaired each before any other change. `expressions/lambdas/capture-outlived-sanitizer.cpp` is in section 4 (LAM-19). `declarations/declarators/multidim-outer-omitted-strings.cpp` became a false reject after the first form of the array brace elision of section 4. The elision now leaves out a list that holds a string literal or an array.

## 2. Group 1: the STG-23 wrong verdict

`foundations/storage/allocated-storage-byte-pointer-class-placement.cpp` and `allocated-storage-byte-pointer-destructor-class-placement.cpp` stopped at STG-23 ("access to an object of scalar type through a glvalue of class 'Cell'"). The program keeps the result of `::operator new` as `unsigned char*`, casts it through `void*` to `Cell*`, and builds element 0 with placement `new`. The cast to `unsigned char*` made the storage an implicit array of `unsigned char`. The later cast to `Cell*` asked for an array of `Cell` at the same place. The front end replaced an implicit array only with an array of an implicit-lifetime type. `Cell` has a constructor, so the cast kept the byte pointer, and the read of `first->tag` read a byte as a `Cell`. STG-28 says that the array of `T` exists also when `T` is not an implicit-lifetime type, and that its elements begin their lifetimes when the program constructs them. Outside constant evaluation, the implicit byte array now gives way to an array of a class type, with each element not yet built. Both tests run to 0.

## 3. Group 2: the new tests of T4 and F5

Each test of this group passes its check, except two: the marked test of section 8 and `classes/operators/literal-operator-template-class-parameter.cpp` (section 8).

- **Literal operators (OPR-8, OPR-10, OPR-16, PRIM-2; Q-0435, Q-0436).** Six F5 tests and `r12-bigint-second-try.cpp`. A literal with a user-defined suffix no longer gives PRIM-2 when no integer type holds its value. PRIM-2 applies only when the numeric form takes the value. The raw form takes the characters. A literal operator template with one pack of `char` and no parameters is now called with the characters of the raw form as explicit template arguments. When the raw form and such a template both exist and no numeric form does, a numeric literal is an OPR-16 error. A string or character literal whose suffix has only a template of the `char` pack form is an OPR-16 error. A template head of a form that OPR-8 does not give is an OPR-10 error. The specialization of a template no longer fails OPR-10 for its empty parameter list.
- **RV8's literal operator tests.** During the round, RV8 added the class-type form to OPR-8, with seven tests. A template whose one parameter is a constant of class type takes a string literal as its template argument when the literal is a valid argument. Otherwise the string form serves. With no string form, the literal is an OPR-16 error. An integer literal whose suffix has only such a template is an OPR-16 error.
- **TYN-4 and ACC-5.** `friend-class-key-enclosing-from-nested.cpp` and `friend-class-key-enclosing-from-nested-template.cpp`. `friend class Queue;` in a class nested in `Queue` declared a new class `Queue` in the nested class. TYN-4 says that the specifier refers to the class that lookup finds. A friend declaration with an elaborated type now uses ordinary lookup, and it declares a class only when lookup finds none.
- **INST-5 and INST-7 (P3-6, P3-7, P3-8).** `member-return-type-waits-for-definition.cpp`, `member-parameter-type-waits-for-definition.cpp`, and `member-constructor-made-at-call.cpp`. The front end made the parameter and return types of each member function complete when it made the member's declaration with the class (FN-12), while the class argument was still incomplete. The body of such a member already waits for a use. The types are now made complete when the body is made. `programs/P3/pipeline-first-try.cpp` passes its check for the same reason, and its run stops at a range `for` (section 9).
- **INST-1 and JMP-3.** `lambda-return-specialization-braced-return.cpp` and `r12-parse-third-try.cpp`. A braced list that initializes an object of a class template specialization did not make the class. The initialization now makes it (INST-1).
- **TPL-6 and TPL-7.** `constant-parameter-specialization-array-member.cpp` and `constant-parameter-specialization-array-member-constructor.cpp`. The structural-type check of a constant parameter of type `Span<2>` read the class as incomplete. The check now makes the specialization first.
- **CTAD-1, CTAD-2, CTAD-6, CTAD-18.** `site-new-no-initializer.cpp`: `new Box` with no initializer now deduces with no arguments. `site-static-member-no-initializer.cpp`, and from report 25's list `site-member.cpp` and `site-return-type.cpp`: a deduced class template type is now an error as the type of a static data member with no initializer, as the type of a non-static member, and as a return type. `initializer-list-guide-empty-list.cpp` and RV8's `initializer-list-guide-empty-list-defaults.cpp`: an empty list skips the initializer-list phase when a guide takes no arguments.
- **ILC-2 and DED-35 (`r12-ring-third-try.cpp`, `r12-ring-fifth-try.cpp`).** `return {this, 0};` to the nested `Iter<false>` stayed unknown in constant evaluation. `Iter` has a constructor template whose first parameter is `const Iter<false>&`. A braced list went to the constructor templates only when each template's first parameter was a template parameter. A class type that is not `std::initializer_list` makes no initializer-list constructor. So the list now reaches the templates, and deduction fails for the template with the wrong number of parameters. Both programs run to 0 in every explored order.
- **RV8's TYN-10 tests.** `forward-class-attributes.cpp` passed on arrival. `elaborated-attribute-not-forward-declaration.cpp`: an attribute or alignment specifier after the class key is now an error when the specifier does not declare the class alone.

The T4 run tests `braced-beside-constructor-template-deduction-fails.cpp`, `braced-beside-constructor-template-no-conversion.cpp`, `assignment-operator-class-parameter.cpp`, and `begin-end-from-base-over-derived.cpp`, and `programs/P3/events-first-try.cpp`, pass their check and stop in the run (section 9).

## 4. Group 3: the other failures that the current design can take

| Group | Tests | What changed |
|---|---|---|
| ICS-45, OVL-15, CAST-19 (Q-0156, Q-0337) | the five `user-defined-explicit*.cpp` tests | A `static_cast` to a class takes an explicit conversion function whose result is the class, through the copy or move constructor. A cast to another type, or to a reference, is now an error when no conversion function serves it, with the limits of ICS-45 on the explicit ones. |
| BF-9 | `bitfield-union-size.cpp`, `bitfield-wide-alignment-128.cpp`, `bitfield-wide-layout.cpp`, `bitfield-wide-layout-ilp32.cpp` | A bit-field wider than its type starts at the next multiple of the alignment of `U`, the largest integer type with at most its width. The bit-field gives the class that alignment and holds its value in its first `8 * sizeof(T)` bits. In a union it starts at byte 0. |
| NEW-1, NEW-2 | `array-paren-init.cpp`, `array-paren-init-narrowing.cpp`, `array-size-from-initializer.cpp`, `forms.cpp` | A parenthesized list for an array `new`, an array `new` whose bound comes from its braced list, and `new auto(e)` and `new auto{e}`. |
| CLS-2, EXS-1, EXS-6 | `noexcept-complete-class.cpp`, `operand-later-member.cpp`, `operand-cycle.cpp`, `operand-cycle-static.cpp` | The operand of a member's `noexcept` that needs a later member is parsed again when the class is complete. A function whose operand names the function itself stays potentially throwing. |
| LIST-6, UNI-7 | `brace-elision.cpp`, `anonymous-union-brace-elision.cpp` | Brace elision for the elements of an array, and into a union, which takes as many values as its first member. |
| DCL-5, CVF-1 | `array-bound-class-conversion.cpp`, `constexpr-conversion.cpp` | An array bound of class type converts by a conversion function. |
| CNS-24, CNS-26, CNS-27 | `lambda-overload.cpp`, `lambda-redeclaration.cpp` | A constraint that contains a lambda expression is equivalent to no other constraint. |
| CTAD-12 | `guide-result-other-template.cpp` | A guide template whose result names another class template. |
| TARG-24 | `argument-makes-parameter-invalid-pointer.cpp` | Arguments that make the type of a later parameter a pointer to a reference. |
| INST-21 (Q-0235) | `explicit-unqualified-outside-namespace-function.cpp` | An explicit instantiation of a function template outside the template's namespace, with a name that is not qualified, also when a using-directive finds the template. |
| DCMP-3 | `template-defaulted.cpp` | A defaulted comparison that is a member template. |
| LAM-19 | `capture-outlived-sanitizer.cpp` | After the `new auto` change, this run reached the closure call and stopped at OBJ-2, which the test does not name. A use of a reference capture whose variable has ended now names LAM-19. The test was `run-unsupported` in the baseline and passes now. |

Section 8 gives the cause of each test that still fails.

## 5. A fault with no test

`operator""_w<'a', 'b'>()` calls a literal operator template by its name with explicit arguments. In constant evaluation it gives the wrong value, and `12_w` gives the right one. No test covers the form. The fault is in the front end's parse of an operator function name followed by a template argument list.

## 6. Ledger rows of `reports/clean-room/ledger.md` that this round answers

The round passes the T4 reductions of the round 12 programmer and the F5 tests that rows P3-6 to P3-10 name for item FE (sections 2 and 3). `pipeline-first-try.cpp` passes its check, and its run stops at a range `for` (section 9).

## 7. Tests added during the round

69 tests are new against the baseline of report 25: 51 present at the start of the round, and 18 that arrived from RV8 during it. 61 of them pass fully. Of the 8 that do not:

- 5 pass their check and stop in a run (section 9).
- `static-inline-member-incomplete-argument.cpp` is marked for the template model (section 8).
- `literal-operator-template-class-parameter.cpp` needs deduction for a constant parameter whose type is a class template name (section 8).
- `classes/initializer-lists/braced-beside-constructor-template-*.cpp` are two of the 5 run stops.

## 8. Decision 0060: the completion test

The completion line of `tools/frontend_check.py`:

```
frontend: completion (decision 0060): 77 tests fail their check, of which 32 need the template model (marks file) and 45 count toward the gate of decisions 0062 and 0063; 3/147 program tests do not run to their result; 9 tests out of scope
```

**The gate of decisions 0062 and 0063 holds.** The gate has three parts:

- At most 50 failing tests that the marks file does not mark. The count is 45.
- No test moved from a passing status to a failing one in two rounds. The baseline comparison reported 0 worse in round 25 and 0 worse at the end of this round. Two tests were worse in a single run inside the round and passed in the next (section 1). Neither was ever written to the baseline as failing.
- A named cause for each remaining test. The table below names one for each of the 45. The marks file names the section of `frontend/TEMPLATE-MODEL.md` for each of the 32 marked tests.

### Marks added

The marks file gains two rows. `frontend/TEMPLATE-MODEL.md` section 6 lists them, and its table of limits counts them.

| Test | Section | Reason |
|---|---|---|
| `templates/instantiation/static-inline-member-incomplete-argument.cpp` | 3 | The class instantiation makes an `inline` static member's initializer, so `Cursor<Derived>` is made while `Derived` is incomplete. Section 3 makes the member's definition at its first use (INST-5, Q-0231). This is the limit of section 3 that six tests of report 25 already have. |
| `classes/copy-move/class-template-ctor-by-value.cpp` | 2 | In the pattern of `Wrapper<T>`, the parameter of `Wrapper(Wrapper)` has type `Ty::Unknown`, so CPY-15 cannot see that the constructor takes the class by value. Section 2 gives a parameter declared with the class's own name the type `CurrentInst`. This is the limit of section 2 that four tests of report 25 already have. |

No mark was removed. Each of the 30 tests that report 25 marked still fails for the cause that the marks file gives.

### The 45 tests that count toward the gate

The 45 tests are 19 missed rejects, 23 unsupported, 1 false reject, and 2 EDG-AGREES. One of them is a program test, `programs/roadmap/r10-tuple-second-try.cpp`. 29 tests stop in a run (`run-unsupported`). They pass their check, so the completion line does not count them.

| Test | Rules | Status | Cause |
|---|---|---|---|
| `classes/comparisons/spaceship-member-returns-non-category.cpp` | DCMP-11 | missed reject | No check that a member `<=>` that a defaulted `<=>` uses returns a comparison category type. |
| `classes/constructors/base-init-dynamic-cast-early-constant.cpp` | CTOR-12 | missed reject | Constant evaluation does not track that a base is not yet constructed when a `dynamic_cast` in a base initializer runs. |
| `classes/copy-move/trivially-copyable-member-template-selected.cpp` | CPY-27, UNI-11 | EDG-AGREES | The trivially copyable test ignores a member template that overload resolution selects for the copy. |
| `classes/operators/literal-operator-template-class-parameter.cpp` | OPR-8, OPR-10 | unsupported | The second template's parameter has type `Text`, a class template name, whose arguments the string literal deduces (TPL-11). The front end does not deduce them. The first template of the test passes. |
| `declarations/auto/copy-list-deduces-initializer-list.cpp` | AUTO-7 | unsupported | `auto x = {...}` gives no `std::initializer_list<E>` type. |
| `declarations/constexpr/lambda-escalates-runtime.cpp` | CEV-1 | missed reject | No escalation of a lambda's call operator to an immediate function. |
| `declarations/structured-bindings/tuple-member-get-type-parameter-uses-free.cpp` | SB-5 | false reject | The front end uses the member `get`, a template with a type parameter, where SB-5 says the free `get` is used. |
| `expressions/conditional/member-pointer-nullptr.cpp` | COND-3 | unsupported | The conditional of a pointer to member and `nullptr` has no type. |
| `expressions/decltype/nested-class-member-evaluated.cpp` | UNEV-1, PRIM-25 | missed reject | No check of a non-static member of an enclosing class named in an evaluated operand of a nested class. |
| `expressions/lambdas/capture-default-in-member-initializer.cpp` | LAM-6 | unsupported | A call of a closure from a default member initializer. |
| `expressions/lambdas/capture-visible-in-trailing-return.cpp` | LAM-18 | unsupported | A capture named in the trailing return type (Q-0092). |
| `expressions/postfix/default-template-argument-incomplete-class.cpp` | PFX-11, CLS-2 | missed reject | No check that a default template argument in a member needs the class complete. |
| `expressions/primary/fold-expansion.cpp` | PRIM-27, PRIM-28 | unsupported | A fold form in constant evaluation. |
| `foundations/constant-evaluation/read-extended-temporary-not-const.cpp` | CE-3 | missed reject | A read of a lifetime-extended temporary that is not `const` is accepted in a constant expression. |
| `foundations/constant-evaluation/reference-to-local-not-usable.cpp` | CE-3 | unsupported | Pointer arithmetic in a constant initializer. |
| `foundations/language-support/ordering-defaulted-specified-needs-conversion.cpp` | LSUP-13 | unsupported | An overloaded operator in constant evaluation. |
| `foundations/names/adl-function-template-parameter-adds-nothing.cpp` | NAM-8, NAM-27 | missed reject | Argument-dependent lookup for a function template name adds the namespaces of its parameter types. |
| `overloading/conversion-sequences/overload-set-argument-sequence.cpp` | ICS-47 | unsupported | Overload resolution with an overload set as an argument. |
| `overloading/overload-resolution/address-no-target-auto-template-parameter.cpp` | OVL-20 | missed reject | An overload set as the argument of an `auto` template parameter. |
| `overloading/overload-resolution/init-direct-explicit-conversion.cpp` | OVL-40 | unsupported | Direct initialization of a class by an explicit conversion function in constant evaluation. |
| `programs/roadmap/r10-tuple-second-try.cpp` | CTAD-29 and others | unsupported | Class template argument deduction beside a constrained constructor template (as `user-guide-beside-constrained-constructor-template.cpp`). |
| `templates/arguments/argument-makes-parameter-invalid.cpp` | TARG-24 | missed reject | The front end keeps the parameter type `typename T::size_type` as an opaque dependent type. Its substitution with `int` cannot tell a failure from an unknown result. |
| `templates/arguments/class-argument-reference-temporary.cpp` | TARG-15 | missed reject | No check that a reference member of a class argument binds to a temporary. |
| `templates/arguments/concept-for-template.cpp` | TARG-7 | missed reject | A concept as the argument of a template template parameter. |
| `templates/arguments/pointer-subobject.cpp` | TARG-14 | unsupported | The address of an array element or a member as a template argument. |
| `templates/class-template-deduction/constant-parameter-guide.cpp` | CTAD-1, TPL-11 | unsupported | Deduction for a constant parameter whose type is a class template name. |
| `templates/class-template-deduction/sites-for-and-condition.cpp` | CTAD-1 | unsupported | Deduction for a range `for` variable and a condition variable. |
| `templates/class-template-deduction/sites.cpp` | CTAD-1 | unsupported | Deduction in a static data member and a constant parameter's type. |
| `templates/class-template-deduction/tie-own-constructor-template.cpp` | CTAD-19, CTAD-23 | missed reject | The tie rule between an own constructor template and an inherited constructor. |
| `templates/class-template-deduction/user-guide-beside-constrained-constructor-template.cpp` | CTAD-29, CTAD-11 | unsupported | A guide from a constrained constructor template keeps its constraint. |
| `templates/concepts/concept-id-value.cpp` | CPT-1 | unsupported | A concept-id with a dependent name in its arguments. |
| `templates/concepts/placeholder-parameter-decltype-auto.cpp` | CPT-37 | missed reject | A `decltype(auto)` placeholder parameter with a type constraint. |
| `templates/concepts/placeholder-parameter.cpp` | CPT-20 | unsupported | A constrained placeholder parameter of a generic lambda. |
| `templates/constraints/explicit-instantiation-member-not-satisfied.cpp` | CNS-13 | missed reject | No constraint check on an explicit instantiation of a member. |
| `templates/constraints/friend-distinct-from-namespace.cpp` | CNS-32, CNS-24 | EDG-AGREES | A friend template of a specialization and the namespace template are taken as one template. |
| `templates/constraints/function-type-specialization.cpp` | CNS-30 | unsupported | The type of a specialization of a constrained function template. |
| `templates/declarations/alias-template-self-through-class.cpp` | TPL-31 | missed reject | No check of an alias template that needs itself through a class. |
| `templates/declarations/constant-parameter-class-template.cpp` | TPL-11, CTAD-1 | unsupported | A constant parameter whose type is a class template name. |
| `templates/declarations/constant-parameter-no-constexpr-constructor.cpp` | TPL-7 | missed reject | No literal type check on a constant parameter's class type. |
| `templates/dependent-names/type-only-positions-more.cpp` | DEP-20 | unsupported | A nested variable template argument of unknown type. |
| `templates/ordering/nondeduced-only-parameter.cpp` | ORD-3, ORD-15 | missed reject | Partial ordering with a parameter that appears only in a non-deduced position. |
| `templates/packs/ellipsis-after-parameter.cpp` | PACK-3 | unsupported | `T...` with no pack as a parameter followed by the variable argument part. |
| `templates/packs/expansion-elements.cpp` | PACK-5 | unsupported | An expansion of a template template pack. |
| `templates/packs/member-template-pack-length.cpp` | PACK-26, PACK-6 | missed reject | No length check of a template template pack that expands an outer pack. |
| `templates/specialization/explicit-member-class-template-no-prefix.cpp` | TSP-23 | missed reject | No check of the `template<>` prefixes of an explicit specialization of a member class template. |

By page:

| Page | Failing tests |
|---|---|
| classes/comparisons | 1 |
| classes/constructors | 1 |
| classes/copy-move | 1 |
| classes/operators | 1 |
| declarations/auto | 1 |
| declarations/constexpr | 1 |
| declarations/structured-bindings | 1 |
| expressions/conditional | 1 |
| expressions/decltype | 1 |
| expressions/lambdas | 2 |
| expressions/postfix | 1 |
| expressions/primary | 1 |
| foundations/constant-evaluation | 2 |
| foundations/language-support | 1 |
| foundations/names | 1 |
| overloading/conversion-sequences | 1 |
| overloading/overload-resolution | 2 |
| programs/roadmap | 1 |
| templates/arguments | 4 |
| templates/class-template-deduction | 5 |
| templates/concepts | 3 |
| templates/constraints | 3 |
| templates/declarations | 3 |
| templates/dependent-names | 1 |
| templates/ordering | 1 |
| templates/packs | 3 |
| templates/specialization | 1 |

Measures for the rewrite: the files over 2000 lines are `expr.rs` (8051), `decl2.rs` (7282), `interp/expr.rs` (4865), `templ.rs` (4360), `decl.rs` (3224), `expr2.rs` (2985), and `parser.rs` (2421), of 52487 lines in all. The first-error rate is 2139 of 2766 passing rejects (77%).

## 9. Run stops

29 tests pass their check and stop in the run. Six of them arrived in this round:

- `programs/P3/pipeline-first-try.cpp` stops at a range `for` (line 107).
- `programs/P3/events-first-try.cpp` stops at a class assignment (line 92).
- `braced-beside-constructor-template-deduction-fails.cpp`, `braced-beside-constructor-template-no-conversion.cpp`, `assignment-operator-class-parameter.cpp`, and `begin-end-from-base-over-derived.cpp` stop in the interpreter.

The other 23 are run stops of report 25.

## 10. Findings

The round records no new finding, and the front-end ledger gains no row. Each group that this round closed follows from the text of the specification, including RV8's changes to OPR-8, TYN-10, and CTAD-18 during the round. The groups that stay open have causes in the front end (section 8) or in the template model (the marks file).

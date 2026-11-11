# Front end, round 25: the T3, F4, RV7, and I7 tests, the TR-13 stop, missed rejects outside templates, and a cause for each failure

Date: 2026-11-11. Decisions 0029, 0035, 0048, 0060, 0061, 0062 (phase 14, item FE).
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decisions 0035, 0060, 0061, and 0062, `documentation/exercises.md` section "Front end", `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md`, report 24, `reports/clean-room/roadmap-11/implementer.md` item 1, `programmer.md` item 2 and section 1.5, `writer.md` item 2, `reports/clean-room/P2/report.md` (P2-9), the front-end ledger, `reports/clean-room/ledger.md`, and the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`. No compiler was run, and no crates were added. The changed files are `frontend/README.md`, `reports/clean-room/rust-frontend/conformance.tsv`, `reports/clean-room/rust-frontend/ledger.md` (rows G164 and G165), this report, and 17 files under `frontend/src/`: `class.rs`, `cons.rs`, `ctad.rs`, `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `ics.rs`, `lexer.rs`, `main.rs`, `ovl.rs`, `parser.rs`, `stmt.rs`, `templ.rs`, `types.rs`, `interp/bytes.rs`, and `interp/expr.rs`. No file was added under `frontend/`.

The reader returned this report as a message, and the integrator saved it here.

The suite grew during the round. At the start, 49 tests were new against the baseline of report 24 (6878 tests), and 29 of them did not pass. During the round, one test arrived from item E (Implement I7, 411ef68e) and 10 from item RV7. So the suite went from 6927 tests at the start of the round to 6938.

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each change, on the live tree, each under `flock build/run_tests.lock`. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, `pass-partial`, or `NO-DIAGNOSTIC-ALLOWED`. A passed check is a full pass or a `run-` status. At the end of the round, `tools/frontend_check.py --update` wrote the new baseline to `conformance.tsv`.

### Counts

| Measure | Baseline of report 24 (6878 tests) | Start of round (6927 tests) | End of round (6938 tests) |
|---|---|---|---|
| Full pass | 6718 | 6738 | 6801 |
| Check pass | 6742 | 6774 | 6825 |
| Tests that fail their check (completion line) | 127 | 144 | 104 |
| Program tests that do not run to their result | 2 of 123 | 4 of 133 | 1 of 133 |

Status by test:

| Status | Baseline of report 24 (6878) | End (6938) |
|---|---|---|
| pass | 6402 | 6482 |
| EDG-ONLY (a pass) | 282 | 283 |
| EDG-ALLOWED (a pass) | 10 | 10 |
| pass-partial (a pass) | 18 | 20 |
| NO-DIAGNOSTIC-ALLOWED (a pass) | 6 | 6 |
| EDG-AGREES (a failure) | 2 | 5 |
| FALSE-REJECT | 8 | 7 |
| MISSED-REJECT | 69 | 45 |
| unsupported | 57 | 56 |
| run-unsupported | 24 | 24 |

At the start of the round, the 144 tests that failed their check were 70 missed rejects, 63 unsupported, 14 false rejects, and 6 EDG-AGREES, less the 9 tests out of scope. The start also had 32 run stops and 4 wrong run results (the storage tests of section 2).

Passing rejects whose first error names a cited or linked rule: 2087 of 2709 in report 24, 2115 of 2743 at the end of this round.

### The checks

`python3 -I tools/frontend_check.py`, before `--update`:

```
frontend: 6801/6938 pass, 6825/6938 pass their check, 0 worse, 29 better, 0 moved between passing statuses, 60 new (6 not passing)
frontend: 21/21 interpreter-only expectations met
frontend: completion (decision 0060): 104 tests fail their check, 1/133 program tests do not run to their result, 9 tests out of scope
frontend: 2115/2743 passing rejects name a cited rule (1930) or a linked rule (185) in their first error; `tidal-fe conform` gives the table per page
```

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 186 examples (184 pass, 2 unsupported); 0 worse, 0 better, 11 new (1 not passing), 2 gone
```

Both checks exit 0. No check in the round reported a test as worse.

## 2. Group 1: the new tests of T3, F4, RV7, and I7, and the TR-13 stop

Each test of this group passes, except the tests in section 7 that are marked as rewrite work.

- **ADD-14, STG-28, STG-29 (Q-0430).** `byte-array-typed-step-distance.cpp`, `byte-array-typed-step-placement.cpp`, `typed-pointer-to-byte-two-past.cpp`, and `allocated-storage-typed-step-after-byte-offset.cpp`. A `T*` that points to a byte of a byte array stepped by one byte. It now steps `sizeof(T)` bytes, as the provisional answer of Q-0430 gives, and a step of more than one element stops the run with ADD-14. Subtraction of two such pointers counts in units of `T`. In storage from an allocation function, a cast to `long*` of a pointer to the byte at the offset of element 2 now gives element 2 of the array of `long` there. The text does not say this (G164). The front end checks each step on its own and keeps no origin, so a second one-element step from the position one past is not caught (README, "Bytes").
- **TR-13 stop of `r10-list-first-try.cpp`, and its two reductions.** `default-argument-converting-constructor-after-derived-pointer.cpp` and `default-argument-converting-constructor-unconstrained-after-derived-pointer.cpp` overflowed the stack on the check, and the program stopped at TR-13. The cause was in deduction. To find a user-defined conversion of a `Circle*` to `Handle<false>`, the front end deduced the converting constructor template `Handle(const Handle<false>&)`. DED-28 checks a parameter that holds no template parameter against its argument, and that check asked for a full conversion sequence of `Circle*` to `const Handle<false>&`. The sequence considered the same constructor template again, with no end. ICS-10 says that the first parameter of a constructor that ICS-4 considers takes only a standard sequence. The deduction of such a constructor template now asks only for a standard sequence there. The program and both reductions run. A cast to `Shape*` hid the fault because the exact match removed the template before deduction.
- **Pointers to members as template arguments (TARG-12, TPL-6, TPL-11, MPTR-3).** The five `constant-parameter-*member*pointer*.cpp` tests stopped in the run. A constant template argument of a pointer-to-member type had no value, so each call stayed unknown. The value is now the member it names.
- **A generic lambda inside an abbreviated function template (DED-34, LAM-15).** `placeholder-parameter-generic-lambda-inside.cpp` and `placeholder-parameter-returns-generic-lambda.cpp`. The `auto` parameter of the lambda took the name of the enclosing template's first invented parameter, so in the specialization the lambda's parameter got the enclosing argument. A lambda's placeholder parameters now have names of their own.
- **A pack of the enclosing class in a member template (INST-5, PACK-1, Q-0429).** `member-template-enclosing-pack-after-deduced.cpp` and `member-template-enclosing-pack-before-deduced.cpp`. In the member template of `Mixer<int, int>`, the parameter `Ts... parts` stayed a function parameter pack whose pattern names no parameter of the member template, and deduction gave up. The pack of the class is known, so the parameter now expands when the member template is declared. The same fix runs the `collect` member of `r11-signal-first-try.cpp`.
- **A functional cast with braces and class template argument deduction in a template (CTAD-1, CTAD-9).** `braced-cast-function-template-return.cpp`, `braced-cast-function-template-variable.cpp`, and `braced-cast-class-template-member.cpp`. In a template's pattern, `Wrapper{v}` with no argument list was a parse error. It now deduces at the instantiation. The same fix runs `r11-expected-first-try.cpp`.
- **The requires clause of a generic lambda (CNS-1, OVL-24, CPT-16).** `template-head-requires-after-head.cpp`, `template-head-trailing-requires.cpp`, and, from report 24's list, `templates/concepts/lambda-clauses.cpp` and `lambda-both-clauses.cpp`. A lambda with a template head and a requires clause stayed unknown. The clause is now checked when a call makes the call operator, and a call operator whose clause fails gives no candidate.
- **LSUP-1 (Q-0431).** `initializer-list-count-before-pointer.cpp`, and from earlier rounds `initializer-list-no-members.cpp`. The shape check now asks for the pointer before the count, and a braced list that needs a `std::initializer_list` of another shape is an LSUP-1 error.
- **I7-17: ILC-3.** `classes/initializer-lists/braced-copy-list-ctor-other-element-type.cpp`. `Bag b{a}` with `a` a `Bag` went to the initializer-list constructor for `int` and stopped. When no initializer-list constructor converts the element to its element type, the second phase of ILC-3 now takes the copy.
- **I7-16: CTAD-9.** `templates/class-template-deduction/aggregate-candidate.cpp`. A designated list such as `Pair c{.first = 1L, .second = 2u}` formed no aggregate candidate. It now forms one, with a parameter of each named member's declared type, in member order.
- **RV7 tests.** DTOR-8 (`classes/constructors/dtor-call-qualified-base.cpp`): `item.Base::~Base()` was a parse error. A qualifier that names the object's class or a base of it now names that class's destructor. JMP-6, LOOP-9 (`statements/loops/jump-into-range-for-body.cpp`): the loop variable of a range `for` now counts as a variable with an initializer, so a jump into the body is rejected. The other 8 RV7 tests passed on arrival.
- **F4 tests.** Of item F4's 19 new tests, 13 pass, among them `initializer-list-count-before-pointer.cpp` (above). The 6 that do not pass are marked as rewrite work in section 7: the two `sizeof-value-dependent-operand-*.cpp` tests (DEP-4) and the four `class-point-*` and `class-static-constexpr-*` tests (INST-33, INST-5).

## 3. Group 2: missed rejects outside templates and other constructs

| Group | Tests | What changed |
|---|---|---|
| PRIM-26 | `number-runs-over-dots.cpp` | A numeric run with two `.` is not one valid literal. |
| ALN-2 | `on-handler-variable.cpp` | No alignment specifier on a handler's variable. |
| DEFARG-4 | `this-in-sizeof-default.cpp` | `this` in a default argument is an error also in an operand that is not evaluated. |
| DCL-19, STG-1 | `array-size-exceeds-size-type.cpp`, `array-4gib-ilp32.cpp` | A bound that does not fit in `std::size_t`, and an array whose size in bytes does not fit. A bound near the largest value no longer collides with the front end's encoding of a bound that names a parameter. |
| RFOR-7 | `postfix-only-iterator.cpp` | An iterator without a valid prefix `++`. |
| AUTO-21, RFOR-1 | `braced-list-mixed-types.cpp` | The elements of a braced range deduce one type. |
| LOG-3, ICS-7 | `operand-ambiguous-conversion.cpp` | A contextual conversion to `bool` that is ambiguous is an error, also in `if` and `?:`. |
| OVL-9, OVL-36 | `builtin-logical-explicit-competes.cpp`, `builtin-logical-explicit-competes-and.cpp` | The built-in candidates of `!`, `&&`, and `||` take an explicit conversion function to `bool` (ICS-45), so they tie with a user-defined operator. |
| CTOR-22, ICS-10 | `one-user-conversion-copy-init.cpp` | Copy-initialization from a value that is not a class converts it by one constructor, whose first parameter takes only a standard sequence. |
| CVF-8 | `new-size-two-integral-conversions.cpp` | The size of a `new` array from a class with two conversion functions to integral types. |
| CVF-9 | `const-object-explicit-cast.cpp` | A `static_cast` of a `const` object by a conversion function that is not `const`. |
| LIST-2, CTOR-21 | `mem-init-braces-narrowing-array.cpp` | A narrowing conversion of an element of an array in braces. The array path did not check narrowing at all. |
| VIRT-5 | `no-unique-final-overrider.cpp` | Two classes override a function of a shared virtual base, and neither overrides the other. |
| ICS-10, OVL-35 | `assignment-left-no-builtin.cpp` | The left operand of a built-in `=` takes no user-defined sequence, and a class without constructors has no assignment from a scalar. |
| UNW-11 | `placement-new-global-usual-delete-rejected.cpp` | A global placement form with one extra size parameter matches the global sized deallocation function. |
| UN-19 | `sizeof-array-parameter-warns.cpp` | A warning for `sizeof` of a parameter declared as an array. |
| UNEV-4 | `side-effect-decltype-warns.cpp`, `side-effect-unevaluated-warns.cpp` | A warning for an increment, a decrement, or an assignment in the operand of `sizeof` or `decltype`. |
| VIRT-13 | `hidden-virtual-warns.cpp` | With `-Wall` or `-Woverloaded-virtual`, a warning for a function that hides a virtual function of a direct base when no function of that name overrides one (G165). |
| TRY-13 | `dtor-try-block-member-use-warns.cpp` | The warning for a member named in a handler of a function try block also covers a destructor. |
| REF-15 | `dangling-new-member-warns.cpp`, `dangling-new-member-warns-default.cpp` | A warning for a reference member of an aggregate that `new` makes, bound to a temporary. |

Section 7 gives the cause of each test that still fails.

## 4. P2-9: the first line when every order agrees

`tidal-fe run` now says on its first line when every explored run returns the first run's result, and then lists the choices. The second line stays:

```
result: 0 in every explored order (8 runs); unspecified choices: SEQ-1 at line 90 (arguments evaluated from left to right)
explored: 8 runs; other outcomes: result 0
```

A partial exploration says "of a partial set" in the parentheses. The conformance harness reads the run's outcome and not this line, so no test changes. `frontend/README.md` gives the form.

## 5. Ledger rows of `reports/clean-room/ledger.md` that this round answers

The round passes the tests of I7-16 and I7-17 (section 2), the T3 reductions of the round 11 programmer, the TR-13 stop that the programmer reduced, and P2-9 (section 4).

## 6. Tests added during the round

60 tests are new against the baseline of report 24: 49 present at the start of the round, and 11 that arrived during it. 54 pass. The 6 that do not pass are the F4 tests of section 2 that need the rewrite's template model.

## 7. Decision 0060: the completion test

The completion line of `tools/frontend_check.py`:

```
frontend: completion (decision 0060): 104 tests fail their check, 1/133 program tests do not run to their result, 9 tests out of scope
```

**The gate of decision 0062 does not hold.** The gate asks for at most 50 tests that fail their check, and the count is 104. The other two parts hold: no test moved from a passing status to a failing one in round 24 or in this round, and each of the 104 tests has a named cause in the table below. 30 of the 104 are marked "rewrite": they need the template model of `frontend/TEMPLATE-MODEL.md`, and the table names the section of that document that passes each. The current design can take the other 74.

The 104 tests are 45 missed rejects, 50 unsupported, 4 false rejects, and 5 EDG-AGREES. The one program test is `programs/roadmap/r10-tuple-second-try.cpp`, which is also in the 104. 24 tests stop in a run (`run-unsupported`). They pass their check, so the completion line does not count them.

| Test | Rules | Status | Cause | Rewrite |
|---|---|---|---|---|
| `classes/comparisons/spaceship-member-returns-non-category.cpp` | DCMP-11 | missed reject | No check that a member `<=>` that a defaulted `<=>` uses returns a comparison category type. | |
| `classes/comparisons/template-defaulted.cpp` | DCMP-3 | missed reject | No check of DCMP-3 on a defaulted comparison of a class template's specialization. | |
| `classes/constructors/base-init-dynamic-cast-early-constant.cpp` | CTOR-12 | missed reject | Constant evaluation does not track that a base is not yet constructed when a `dynamic_cast` in a base initializer runs. | |
| `classes/conversion-functions/constexpr-conversion.cpp` | CVF-1 | unsupported | Constant evaluation of a class object's conversion to an integer in an array bound. | |
| `classes/copy-move/class-template-ctor-by-value.cpp` | CPY-15 | missed reject | No CPY-15 check on a constructor of a class template specialization that takes the class by value. | |
| `classes/copy-move/trivially-copyable-member-template-selected.cpp` | CPY-27, UNI-11 | EDG-AGREES | The trivially copyable test ignores a member template that overload resolution selects for the copy. | |
| `classes/members/noexcept-complete-class.cpp` | CLS-2 | unsupported | The `noexcept` operator on a member declared later in the class. | |
| `classes/unions/anonymous-union-brace-elision.cpp` | UNI-7 | unsupported | Brace elision into an anonymous union member in constant initialization. | |
| `classes/unions/bitfield-union-size.cpp` | BF-9, UNI-1 | unsupported | The layout of a bit-field wider than its type. | |
| `classes/unions/bitfield-wide-alignment-128.cpp` | BF-9 | unsupported | The same. | |
| `classes/unions/bitfield-wide-layout-ilp32.cpp` | BF-9, BF-3 | unsupported | The same. | |
| `classes/unions/bitfield-wide-layout.cpp` | BF-9, BF-3 | unsupported | The same. | |
| `declarations/auto/copy-list-deduces-initializer-list.cpp` | AUTO-7 | unsupported | `auto x = {...}` gives no `std::initializer_list<E>` type. | |
| `declarations/constexpr/lambda-escalates-runtime.cpp` | CEV-1 | missed reject | No escalation of a lambda's call operator to an immediate function. | |
| `declarations/declarators/array-bound-class-conversion.cpp` | DCL-5 | unsupported | Constant evaluation of a class object's conversion in an array bound. | |
| `declarations/exception-specifications/operand-cycle-static.cpp` | EXS-6 | unsupported | The `noexcept` operator on a member whose exception specification is not yet known. | |
| `declarations/exception-specifications/operand-cycle.cpp` | EXS-6 | unsupported | The same. | |
| `declarations/exception-specifications/operand-later-member.cpp` | EXS-1 | unsupported | The same. | |
| `declarations/initialization/list/brace-elision.cpp` | LIST-4 | unsupported | Brace elision in constant evaluation. | |
| `declarations/structured-bindings/tuple-member-get-type-parameter-uses-free.cpp` | SB-5 | false reject | The front end uses the member `get`, a template with a type parameter, where SB-5 says the free `get` is used. | |
| `expressions/conditional/member-pointer-nullptr.cpp` | COND-3 | unsupported | The conditional of a pointer to member and `nullptr` has no type. | |
| `expressions/decltype/nested-class-member-evaluated.cpp` | UNEV-1, PRIM-25 | missed reject | No check of a non-static member of an enclosing class named in an evaluated operand of a nested class. | |
| `expressions/lambdas/capture-default-in-member-initializer.cpp` | LAM-6 | unsupported | A call of a closure from a default member initializer. | |
| `expressions/lambdas/capture-visible-in-trailing-return.cpp` | LAM-18 | unsupported | A capture named in the trailing return type (Q-0092). | |
| `expressions/new-delete/array-paren-init-narrowing.cpp` | NEW-2 | unsupported | An array `new` with parentheses in constant evaluation. | |
| `expressions/new-delete/array-paren-init.cpp` | NEW-2 | unsupported | The same. | |
| `expressions/new-delete/array-size-from-initializer.cpp` | NEW-2 | unsupported | An array `new` whose size comes from the initializer, in constant evaluation. | |
| `expressions/new-delete/forms.cpp` | NEW-1 | unsupported | The `new` forms in constant evaluation. | |
| `expressions/postfix/default-template-argument-incomplete-class.cpp` | PFX-11, CLS-2 | missed reject | No check that a default template argument in a member needs the class complete. | |
| `expressions/primary/fold-expansion.cpp` | PRIM-27, PRIM-28 | unsupported | A fold form in constant evaluation. | |
| `foundations/constant-evaluation/read-extended-temporary-not-const.cpp` | CE-3 | missed reject | A read of a lifetime-extended temporary that is not `const` is accepted in a constant expression. | |
| `foundations/constant-evaluation/reference-to-local-not-usable.cpp` | CE-3 | unsupported | Pointer arithmetic in a constant initializer. | |
| `foundations/language-support/ordering-defaulted-specified-needs-conversion.cpp` | LSUP-13 | unsupported | An overloaded operator in constant evaluation. | |
| `foundations/names/adl-function-template-parameter-adds-nothing.cpp` | NAM-8, NAM-27 | missed reject | Argument-dependent lookup for a function template name adds the namespaces of its parameter types. | |
| `overloading/conversion-sequences/overload-set-argument-sequence.cpp` | ICS-47 | unsupported | Overload resolution with an overload set as an argument. | |
| `overloading/conversion-sequences/user-defined-explicit-reference-other-type.cpp` | ICS-45 | unsupported | A `static_cast` to a reference by an explicit conversion function (Q-0337). | |
| `overloading/conversion-sequences/user-defined-explicit-reference-value.cpp` | ICS-45 | unsupported | The same. | |
| `overloading/conversion-sequences/user-defined-explicit-reference-xvalue.cpp` | ICS-45 | unsupported | The same. | |
| `overloading/conversion-sequences/user-defined-explicit-rvalue-reference-value.cpp` | ICS-45 | unsupported | The same. | |
| `overloading/conversion-sequences/user-defined-explicit.cpp` | ICS-45 | unsupported | The same. | |
| `overloading/overload-resolution/address-no-target-auto-template-parameter.cpp` | OVL-20 | missed reject | An overload set as the argument of an `auto` template parameter. | |
| `overloading/overload-resolution/init-direct-explicit-conversion.cpp` | OVL-40 | unsupported | Direct initialization of a class by an explicit conversion function in constant evaluation. | |
| `programs/roadmap/r10-tuple-second-try.cpp` | CTAD-29 and others | unsupported | Class template argument deduction beside a constrained constructor template (as `user-guide-beside-constrained-constructor-template.cpp`). | |
| `statements/if/constexpr-template-nondependent-call-checked.cpp` | IF-22 | missed reject | The pattern's parse does not check an independent call in a discarded statement. | yes, section 5 |
| `templates/arguments/argument-makes-parameter-invalid-pointer.cpp` | TARG-24 | missed reject | No check that an argument makes the type of a later constant parameter invalid. | |
| `templates/arguments/argument-makes-parameter-invalid.cpp` | TARG-24 | missed reject | The same. | |
| `templates/arguments/class-argument-reference-temporary.cpp` | TARG-15 | missed reject | No check that a reference member of a class argument binds to a temporary. | |
| `templates/arguments/concept-for-template.cpp` | TARG-7 | missed reject | A concept as the argument of a template template parameter. | |
| `templates/arguments/pointer-subobject.cpp` | TARG-14 | unsupported | The address of an array element or a member as a template argument. | |
| `templates/class-template-deduction/constant-parameter-guide.cpp` | CTAD-1, TPL-11 | unsupported | Deduction for a constant parameter whose type is a class template name. | |
| `templates/class-template-deduction/guide-result-other-template.cpp` | CTAD-12 | missed reject | No check that a guide's result names its own template. | |
| `templates/class-template-deduction/site-member.cpp` | CTAD-2 | missed reject | No check of the sites where deduction is not allowed. | |
| `templates/class-template-deduction/site-return-type.cpp` | CTAD-2 | missed reject | The same. | |
| `templates/class-template-deduction/sites-for-and-condition.cpp` | CTAD-1 | unsupported | Deduction for a range `for` variable and a condition variable. | |
| `templates/class-template-deduction/sites.cpp` | CTAD-1 | unsupported | Deduction in a static data member and a constant parameter's type. | |
| `templates/class-template-deduction/tie-own-constructor-template.cpp` | CTAD-19, CTAD-23 | missed reject | The tie rule between an own constructor template and an inherited constructor. | |
| `templates/class-template-deduction/user-guide-beside-constrained-constructor-template.cpp` | CTAD-29, CTAD-11 | unsupported | A guide from a constrained constructor template keeps its constraint. | |
| `templates/concepts/concept-id-value.cpp` | CPT-1 | unsupported | A concept-id with a dependent name in its arguments. | |
| `templates/concepts/placeholder-parameter-decltype-auto.cpp` | CPT-37 | missed reject | A `decltype(auto)` placeholder parameter with a type constraint. | |
| `templates/concepts/placeholder-parameter.cpp` | CPT-20 | unsupported | A constrained placeholder parameter of a generic lambda. | |
| `templates/constraints/explicit-instantiation-member-not-satisfied.cpp` | CNS-13 | missed reject | No constraint check on an explicit instantiation of a member. | |
| `templates/constraints/friend-distinct-from-namespace.cpp` | CNS-32, CNS-24 | EDG-AGREES | A friend template of a specialization and the namespace template are taken as one template. | |
| `templates/constraints/function-type-specialization.cpp` | CNS-30 | unsupported | The type of a specialization of a constrained function template. | |
| `templates/constraints/lambda-overload.cpp` | CNS-24, CNS-27 | missed reject | Clauses that contain lambda expressions compare as equivalent. | |
| `templates/constraints/lambda-redeclaration.cpp` | CNS-26, CNS-27 | missed reject | The same. | |
| `templates/constraints/out-of-class-other-clause.cpp` | CNS-29 | missed reject | The member's trailing clause is not kept for the definition outside the class. | yes, section 5 |
| `templates/constraints/out-of-class-without-clause.cpp` | CNS-29 | missed reject | The same. | yes, section 5 |
| `templates/declarations/alias-template-self-through-class.cpp` | TPL-31 | missed reject | No check of an alias template that needs itself through a class. | |
| `templates/declarations/constant-parameter-class-template.cpp` | TPL-11, CTAD-1 | unsupported | A constant parameter whose type is a class template name. | |
| `templates/declarations/constant-parameter-no-constexpr-constructor.cpp` | TPL-7 | missed reject | No literal type check on a constant parameter's class type. | |
| `templates/deduction/substitution-explicit-specifier.cpp` | DED-24 | unsupported | Substitution into the operand of a conditional `explicit`. | yes, section 4 |
| `templates/deduction/substitution-lambda-body.cpp` | DED-29 | missed reject | A substitution mutes every error, and the lambda body is outside the immediate context. | yes, section 4 |
| `templates/deduction/substitution-lambda-default.cpp` | DED-29 | missed reject | The same, in a template parameter's default. | yes, section 4 |
| `templates/deduction/substitution-member-wrong-kind.cpp` | DED-24 | unsupported | A member of the wrong kind stops the replay. | yes, section 4 |
| `templates/dependent-names/binary-operator-pack-never-fits.cpp` | DEP-37 | missed reject | No check of an operator function that only an empty pack makes valid. | yes, section 5 |
| `templates/dependent-names/current-instantiation-enclosing-base.cpp` | DEP-11 | unsupported | The current instantiation of an enclosing class is an unknown base. | yes, section 2 |
| `templates/dependent-names/member-template-current-instantiation-pointer.cpp` | DEP-24, DEP-2 | false reject | The type of an object of the current instantiation is unknown in a pattern. | yes, section 2 |
| `templates/dependent-names/member-template-current-instantiation-qualifier-dependent-base.cpp` | DEP-24 | missed reject | The same. | yes, section 2 |
| `templates/dependent-names/member-template-non-dependent-base.cpp` | DEP-24 | false reject | The same, through an independent base. | yes, section 2 |
| `templates/dependent-names/member-template-through-current-instantiation-object.cpp` | DEP-24, DEP-2 | false reject | The same. | yes, section 2 |
| `templates/dependent-names/operator-pack-never-fits.cpp` | DEP-37 | missed reject | As `binary-operator-pack-never-fits.cpp`. | yes, section 5 |
| `templates/dependent-names/other-specialization-constant-argument-member-missing.cpp` | DEP-30, DEP-15 | missed reject | A template-id with independent arguments inside its own template is treated as dependent. | yes, section 1 |
| `templates/dependent-names/other-specialization-constant-argument-no-typename.cpp` | DEP-15, DEP-1 | unsupported | The same. | yes, section 1 |
| `templates/dependent-names/sizeof-value-dependent-operand-call-bound-early.cpp` | DEP-4, DEP-5 | EDG-AGREES | No value-dependence mark: `Slot<sizeof(N)>` is treated as dependent, so the call binds late. | yes, sections 1 and 8 |
| `templates/dependent-names/sizeof-value-dependent-operand-no-typename.cpp` | DEP-4 | EDG-AGREES | The same: `Box<sizeof(N)>::type` is read as a member of a dependent specialization. | yes, sections 1 and 8 |
| `templates/dependent-names/type-only-positions-more.cpp` | DEP-20 | unsupported | A nested variable template argument of unknown type. | |
| `templates/instantiation/class-point-not-later-member-type.cpp` | INST-33, INST-30 | missed reject | Lookup in a specialization also sees declarations after its point of instantiation (DEP-9). | yes, section 3 |
| `templates/instantiation/class-point-not-later-type.cpp` | INST-33 | missed reject | The same. | yes, section 3 |
| `templates/instantiation/class-point-not-later.cpp` | INST-32, INST-31 | unsupported | The same, for a static member's initializer. | yes, section 3 |
| `templates/instantiation/class-point-static-const-member.cpp` | INST-33, INST-5 | unsupported | The same, and the initializer is made with the class. | yes, section 3 |
| `templates/instantiation/class-point-static-constexpr-made-at-use.cpp` | INST-33, INST-5, INST-8 | unsupported | A static `constexpr` member's initializer is made with the class, not at its first use. | yes, section 3 |
| `templates/instantiation/class-static-constexpr-member-initializer-waits.cpp` | INST-5 | EDG-AGREES | The same. | yes, section 3 |
| `templates/instantiation/explicit-unqualified-outside-namespace-function.cpp` | INST-21 | missed reject | No check that an explicit instantiation of a function names it in its namespace. | |
| `templates/instantiation/local-class-default-argument.cpp` | INST-12, INST-6 | missed reject | The members of a local class in a specialization are made at their use, not with the function. | yes, section 3 |
| `templates/instantiation/local-class-exception-specification.cpp` | INST-12, INST-6 | missed reject | The same. | yes, section 3 |
| `templates/instantiation/recursion-endless-function.cpp` | INST-15, TR-13 | missed reject | Endless nesting of function bodies made at the end of the unit has no `busy` state. | yes, section 3 |
| `templates/ordering/nondeduced-only-parameter.cpp` | ORD-3, ORD-15 | missed reject | Partial ordering with a parameter that appears only in a non-deduced position. | |
| `templates/packs/ellipsis-after-parameter.cpp` | PACK-3 | unsupported | `T...` with no pack as a parameter followed by the variable argument part. | |
| `templates/packs/expansion-elements.cpp` | PACK-5 | unsupported | An expansion of a template template pack. | |
| `templates/packs/fold-in-outer-pattern.cpp` | PACK-8 | missed reject | The scan of an expansion pattern does not know that a fold uses up its pack. | yes, section 5 |
| `templates/packs/member-template-pack-length.cpp` | PACK-26, PACK-6 | missed reject | No length check of a template template pack that expands an outer pack. | |
| `templates/packs/size-operator-in-outer-pattern.cpp` | PACK-8 | missed reject | As `fold-in-outer-pattern.cpp`, for `sizeof...`. | yes, section 5 |
| `templates/specialization/explicit-member-class-template-no-prefix.cpp` | TSP-23 | missed reject | No check of the `template<>` prefixes of an explicit specialization of a member class template. | |
| `templates/specialization/partial-no-primary-variable.cpp` | TSP-31 | missed reject | A variable partial specialization with no primary takes a path that does not reach the check. | yes, section 3 |

By page:

| Page | Failing tests |
|---|---|
| classes/comparisons | 2 |
| classes/constructors | 1 |
| classes/conversion-functions | 1 |
| classes/copy-move | 2 |
| classes/members | 1 |
| classes/unions | 5 |
| declarations/auto | 1 |
| declarations/constexpr | 1 |
| declarations/declarators | 1 |
| declarations/exception-specifications | 3 |
| declarations/initialization | 1 |
| declarations/structured-bindings | 1 |
| expressions/conditional | 1 |
| expressions/decltype | 1 |
| expressions/lambdas | 2 |
| expressions/new-delete | 4 |
| expressions/postfix | 1 |
| expressions/primary | 1 |
| foundations/constant-evaluation | 2 |
| foundations/language-support | 1 |
| foundations/names | 1 |
| overloading/conversion-sequences | 6 |
| overloading/overload-resolution | 2 |
| programs/roadmap | 1 |
| statements/if | 1 |
| templates/arguments | 5 |
| templates/class-template-deduction | 8 |
| templates/concepts | 3 |
| templates/constraints | 7 |
| templates/declarations | 3 |
| templates/deduction | 4 |
| templates/dependent-names | 12 |
| templates/instantiation | 10 |
| templates/ordering | 1 |
| templates/packs | 5 |
| templates/specialization | 2 |

Measures for the rewrite: the files over 2000 lines are `expr.rs` (7928), `decl2.rs` (7198), `interp/expr.rs` (4860), `templ.rs` (4339), `decl.rs` (3178), `expr2.rs` (2894), and `parser.rs` (2412), of 52045 lines in all. The first-error rate is 2115 of 2743 passing rejects (77%).

## 8. Findings

### G164. STG-28 does not say which element a `T*` from a byte offset points to

- Page and rules: foundations/storage, STG-28, STG-29; expressions/additive, ADD-14; test `foundations/storage/allocated-storage-typed-step-after-byte-offset.cpp`.
- Kind: missing.
- What happened: STG-28 says that a `T*` from the result of an allocation function points to element 0 of the array of `T` there. ADD-14 says that a `T*` into such an array points to an element of it, so ADD-14 does not apply. The test casts a byte pointer `2 * sizeof(long)` bytes into the storage to `long*`, and its note says the result points to element 2. Neither rule says that a cast from a pointer to a byte at an element's offset gives a pointer to that element. STG-29 says a cast of a pointer into a byte array gives a pointer to a byte, and the text does not name the storage of an allocation function as a byte array. A reader of STG-28 alone gets element 0 only from the result itself. The front end follows the test.
- Suggestion: say in STG-28 that a `T*` cast from a pointer to the byte at the start of element `k` of that array points to element `k`, or say which rule decides the case.

### G165. VIRT-13 does not say whether a hidden overload warns when another overload is overridden

- Page and rules: classes/virtual, VIRT-13; test `classes/virtual/hidden-virtual-warns.cpp`.
- Kind: ambiguous.
- What happened: VIRT-13 asks for a warning about "a derived function that hides a virtual function of a base and does not override it". A base can declare two virtual overloads `f(int)` and `f(double)`, and the derived class can override `f(int)` and declare no `f(double)`. The derived `f(int)` hides `f(double)`, and it overrides a function, though not that one. The text does not say whether this case warns. The front end warns only when no derived function of that name overrides a base function, and only with `-Wall` or `-Woverloaded-virtual`, as the flag table gives.
- Suggestion: say whether the warning concerns each hidden base function or each derived function, and add a test for the case with one overload overridden.

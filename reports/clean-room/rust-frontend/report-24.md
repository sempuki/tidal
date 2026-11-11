# Front end, round 24: the T2 and F3 tests, root causes in the unsupported messages, run stops, and missed rejects outside templates

Date: 2026-11-11. Decisions 0029, 0035, 0047, 0048, 0060, 0061 (phase 13, item FE).
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decisions 0035, 0060, and 0061, `documentation/exercises.md` section "Front end", `frontend/README.md`, report 23, `reports/clean-room/roadmap-10/implementer.md` item 2 and `programmer.md` item 2, the front-end ledger, `reports/clean-room/ledger.md`, and the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`. No compiler was run, and no crates were added. The changed files are `frontend/README.md`, `reports/clean-room/rust-frontend/conformance.tsv`, `reports/clean-room/rust-frontend/ledger.md` (rows G161 to G163), this report, and 11 files under `frontend/src/`: `class.rs`, `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `ovl.rs`, `parser.rs`, `stmt.rs`, `templ.rs`, `interp/expr.rs`, and `interp/stmt.rs`. No file was added under `frontend/`.

The reader returned this report as a message, and the integrator saved it here.

The suite grew during the round. At the start, 76 tests were new against the baseline of report 23 (6786 tests), and 33 of them did not pass. During the round, 16 more tests arrived from item E (Implement I6) and item RV6. So the suite went from 6862 tests at the start of the round to 6878.

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each change, on the live tree, each under `flock build/run_tests.lock`. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, `pass-partial`, or `NO-DIAGNOSTIC-ALLOWED`. A passed check is a full pass or a `run-` status. At the end of the round, `tools/frontend_check.py --update` wrote the new baseline to `conformance.tsv`.

### Counts

| Measure | Baseline of report 23 (6786 tests) | Start of round (6862 tests) | After, the same 6862 tests | After, all 6878 tests |
|---|---|---|---|---|
| Full pass | 6582 | 6631 | 6705 | 6718 |
| Check pass | 6622 | 6678 | 6729 | 6742 |

Status by test:

| Status | Start (6862) | After (6878) |
|---|---|---|
| pass | 6327 | 6402 |
| EDG-ONLY (a pass) | 274 | 282 |
| EDG-ALLOWED (a pass) | 10 | 10 |
| pass-partial (a pass) | 14 | 18 |
| NO-DIAGNOSTIC-ALLOWED (a pass) | 6 | 6 |
| EDG-AGREES (a failure) | 3 | 2 |
| FALSE-REJECT | 15 | 8 |
| MISSED-REJECT | 103 | 69 |
| unsupported | 63 | 57 |
| run-unsupported | 44 | 24 |
| RUN-UNEXPECTED-UB | 3 | 0 |

Passing rejects whose first error names a cited or linked rule: 2046 of 2667 at the start of the round, 2087 of 2709 after.

### The checks

`python3 -I tools/frontend_check.py`, before `--update`:

```
frontend: 6718/6878 pass, 6742/6878 pass their check, 0 worse, 54 better, 0 moved between passing statuses, 92 new (10 not passing)
frontend: 20/20 interpreter-only expectations met
frontend: completion (decision 0060): 127 tests fail their check, 2/123 program tests do not run to their result, 9 tests out of scope
frontend: 2087/2709 passing rejects name a cited rule (1902) or a linked rule (185) in their first error; `tidal-fe conform` gives the table per page
```

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 181 examples (179 pass, 2 unsupported); 0 worse, 0 better, 4 new (0 not passing), 0 gone
```

Both checks exit 0. The example `specification/templates/specialization.md:525`, a false reject (TSP-16) at the start of the round, passes now.

## 2. Group 1: the wrong verdicts and the new tests of T2, F3, E, and RV6

Each test of this group passes, except the 10 tests in section 7.

- PACK-14, PACK-13. `base-pack-paren-init-under-derived.cpp` and `base-pack-paren-init-complete-object.cpp`. The mem-initializer-id `Part<K>` was read as a type-id, so `Part<K>(start + 10 * K)` became a function type and the parse failed. The front end then looked up `Part` in the class, found the injected names of both bases and their constructors, and took that as one base named twice. Now the mem-initializer-id is a type name with no declarator. The TARG-11 check of injected names from two bases leaves out the constructors that share the name. The same fix passes `mem-init-other-specialization-own-template.cpp` and `mem-init-two-specializations-one-template.cpp`.
- STG-9, UNI-14, NEW-5. `placement-into-new-array-element-then-delete-array.cpp` and `placement-into-union-member-of-new-array-element.cpp`. A placement `new` with the non-allocating form into an element of storage from `new[]` marked the whole array as made by `new`, so `delete[]` stopped with NEW-5. Now the complete object keeps the way it was made when the new object takes the place of a part of it.
- I5-10: REF-1, ACC-12, CONV-17, DED-15, OVL-37. A reference to a base binds directly by the base relation alone. A reference that binds to an ambiguous base is now a CONV-17 error, and to a base that is not accessible an ACC-12 error, when the binding is made. `lvalue-ref-to-ambiguous-base.cpp`, `lvalue-ref-to-private-base.cpp`, `private-base-with-fallback.cpp`, and `two-equal-bases-with-fallback.cpp` pass, and `lvalue-ref-to-private-base-in-member.cpp` still passes.
- I5-9: DED-24. `qualified-name-in-incomplete-class.cpp`. A qualified name in a class that is declared and not defined is now a certain failure, so substitution fails and the call selects `pick(...)`.
- W20-2: UNI-14, STG-10, UNI-13, ADD-18. Pointer arithmetic in an array member of a union that is not active steps over the positions that the array type gives. UNI-13 now looks at the element class of an array member, so `chain-assign-array-element-nontrivial-constant.cpp` is rejected with UNI-6. The new pointer steps let `pointer-difference-overflow-ilp32.cpp` run its subtraction, so ADD-18 now stops a difference that does not fit in the type of the result. At run time, a construction inside a member of a member union that is not active makes the enclosing members active (G163).
- W20-3: CNS-11, CPY-15. A candidate that is not a template, and that takes every argument with no conversion, now removes the function templates before their constraints are checked. The implicit copy constructor `C(const C&)` counts for a `const C` argument. A copy of a class with a constructor template checks the template's constraints first, unless CNS-11 skips them. So `by-value-constructor-template-constraint-checked.cpp` is rejected, and `by-value-constructor-template-constraint-copies-class.cpp`, `exact-non-template-skips-constraint-check.cpp`, and `default-argument-constrained-converting-constructor.cpp` pass. The last one no longer overflows the stack.
- G157: INST-11. The definition of an object, a `new` expression, and a temporary of a class specialization now make its virtual functions. `virtual-new-expression.cpp` and `virtual-temporary.cpp` pass. The first error names NAM-16, the rule that the body breaks.
- UNI-9 (Q-0424). An anonymous union that holds an unnamed class, or a nested anonymous union, is rejected. `anonymous-union-unnamed-struct-member.cpp` and `anonymous-union-nested-anonymous-union.cpp` pass.
- RFOR-1, DEL-1. `by-value-move-only-elements.cpp`. A deleted or inaccessible constructor that the initialization of the loop variable selects is now an error.
- TSP-15, ORD-14. `partial-array-and-const-array.cpp`. Partial ordering compared the const of `const T[N]` on the array and not on its elements, when both sides were dependent arrays.
- ACC-5, OPR-6. `friend-class-then-operator-calls.cpp`. A friend class declaration made argument-dependent lookup uncertain for the class. Now only a friend function declaration does.
- I6-11: DEP-24, DEP-11. `template-keyword-this-dependent-base.cpp`. `this->template get<3>()` in a class with a dependent base now reads the argument list in the pattern.
- I6-12: OVL-7, NAM-27. `operator-base-class-namespace.cpp`. The early test for a visible operator function now searches the namespaces of the bases.
- RV6 tests. NAM-21 (`definition-trailing-requires-sees-namespace.cpp`): the trailing requires clause of a definition outside its namespace looks up names in that namespace. DED-8, VC-2 (`forwarding-const-array-prvalue.cpp`): a braced prvalue of an array alias is an expression of its type in deduction, and a reference binds it directly. NEW-22 (`aligned-global-delete-sized-form.cpp`): after `delete`, a single object of an over-aligned type uses the global sized aligned form. DTOR-8 (`dtor-call-decltype-reference.cpp`): a `decltype` after `~` that gives a reference is rejected.
- A pattern that names a specialization of the class template whose definition is still being read, as `o.n` with `o` of type `const I<false>&` inside `I`, no longer reports NAM-8. The member access waits for the instantiation.

The ledger rows of `reports/clean-room/ledger.md` that this round answers are I5-9, I5-10, W20-2, W20-3, I6-11, and I6-12, and the rows of the programmer's reductions that section 2 names. The front end passes each of their tests.

## 3. Group 2: root causes behind the unsupported messages

The message "variable template with unknown arguments" now names the first construct that gave an unknown argument, followed by "(in a variable template argument)". A `decltype` of unknown type records its cause: the first unknown part of its operand, the variable whose type its initializer did not give, or a template-id that names a function specialization. The 16 tests of the symptom now report these causes:

| Cause | Tests |
|---|---|
| ambiguous call (overload resolution with `f64` literals) | 3 |
| dependent name | 2 |
| deduction of class template arguments | 2 |
| the type of a variable that its initializer does not give | 2 |
| new expression | 1 |
| call of unknown callee | 1 |
| a template-id that names a function specialization | 1 |
| decltype of an expression of unknown type (the conditional of a pointer to member and `nullptr`) | 1 |
| a type the front end does not know | 1 |
| variable template with unknown arguments (a nested variable template in `sites.cpp` and `type-only-positions-more.cpp`) | 2 |

This round closed these causes:

- `new` in a run: a type in parentheses, `new (int)(4)` and `new (int (*)())`; a braced list with a bound that is not a constant, also with a list longer than the bound (NEW-35, also through a class's allocation function); `std::nothrow` and the nothrow library forms (G162). `parenthesized-type.cpp`, `bad-bound-too-few.cpp`, `bad-bound-nothrow.cpp`, and `nothrow-tag-self-declared.cpp` run.
- Constructor calls in a run: brace elision into an aggregate base or member in the aggregate path; the copy of a class with no user-declared constructor; a braced list with one element of the class and an initializer-list constructor (ILC-3); empty braces with only an initializer-list constructor (ILC-2). `aggregate-with-base.cpp`, `memberwise-order.cpp`, `copy-from-single-same-class.cpp`, and `empty-braces-no-default.cpp` run.

Left: BF-9 layout, pointer arithmetic in a constant past ADD-18, the `noexcept` operator on a later member, a class bound with a conversion function, and the CTAD sites.

## 4. Group 3: run stops that free whole rules

- Jumps into a nested statement (IF-12 twice, SW-1). A `goto` or `case` label now enters a branch of an `if` or the body of a `while`, `do`, or `for`, and the loop goes on after the body (G161). `goto-into-branch.cpp`, `case-into-branch.cpp`, and `case-inside-loop.cpp` run.
- Variables that a jump skips (JMP-16, SW-5). A forward `goto` past a declaration, and a `case` label after a declaration under another label, make the variables with no value. `goto-past-declaration-then-write.cpp` and `jump-past-uninitialized.cpp` run.
- UNI-7. The members of a local anonymous union are variables of the block. `anonymous-union-local.cpp` runs. The run gives each member an object of its own, so the run does not catch a read of the member that is not active at block scope.

Left: overload sets in a run (CVF-1, CVF-4, CVF-5), missing default arguments (TSP-8 for an explicit specialization's member defined outside its class, NEW-23 for a placement allocation function's default), dependent names in a run (DEP-20, DEP-25), unknown callees (TARG-12, DEP-11), and class assignment (OVL-42).

## 5. Group 4: missed rejects outside templates

| Group | Before | After | What changed, or why not |
|---|---|---|---|
| DCMP-12, DCMP-13 | 3 | 0 | The `!=` matches by the adjusted parameter types alone. An explicit object parameter takes part. A using-declaration can name an operator function. |
| DEFARG-13 | 4 | 0 | A later declaration of a function template, or a declaration outside a templated class, adds no default argument. |
| LIST-13, LIST-16 | 4 | 0 | Brace elision into an empty aggregate is an error. A member with no value and an explicit default constructor is an error, also in a designated list. |
| MPTR-10, MPTR-11 | 4 | 0 | An uncalled result of a pointer to member function, an unsafe qualification conversion, a `reinterpret_cast` between data and function members, and a `static_cast` through a private base. |
| CEV-2 | 2 | 0 | The conversion of a consteval function to a pointer, also when a target type selects it. |
| OVL-20 | 4 | 1 | A placeholder variable, an init capture, and an ellipsis argument give an overload set or a function template name no target. Left: the argument of an `auto` template parameter. |
| ACC-15, ACC-18 | 3 | 0 | A friend declaration of an inaccessible member function. A private member of a base named without a qualifier in the derived class. |
| NAM-42 | 2 | 0 | A class template does not share its name with a variable or a function. |
| LSP-7 | 2 | 0 | A C function and a global variable, and a global variable after a C name (Q-0075). |
| LOG-3, OVL-9 | 3 | 3 | Not reached. |
| REF-15 and the `recommended` warnings | not counted again | unchanged | Not reached. |

## 6. Group 5 and items left, with reasons

- `programs/roadmap/r10-list-first-try.cpp` (FALSE-REJECT, TR-13). The instantiation of `cond<0, const Node*, Node*>` nests past the limit. The stop needs the constrained converting constructor `template <bool C = Const> requires C Iter(const Iter<false>&)` together with `const_iterator(head_.next)` in a `const` member. A reduction of a few classes does not show it, so the cause is still open in the front end.
- `programs/roadmap/r10-tuple-second-try.cpp` (unsupported: deduction of class template arguments).
- The DEP-24 tests of Q-0427 (`member-template-through-current-instantiation-object.cpp`, `member-template-current-instantiation-pointer.cpp`, `member-template-non-dependent-base.cpp`, `member-template-current-instantiation-qualifier-dependent-base.cpp`). In a pattern, the type of an object of the current instantiation is unknown, so the front end cannot tell that the member template is found at the definition. This needs the front end's model of the current instantiation, which is rewrite work.
- `other-specialization-constant-argument-*.cpp` (DEP-15, DEP-39) and `substitution-member-wrong-kind.cpp` (DED-24). These also need the front end's model of a specialization.
- The template missed rejects of report 23 section 3 were not targeted.

## 7. Tests added during the round

92 tests are new against the baseline of report 23: 76 present at the start of the round, and 16 that arrived during it. 82 pass. The 10 that do not pass are the two program tests, the six DEP-24 and DEP-15 tests, `substitution-member-wrong-kind.cpp`, and `user-guide-beside-constrained-constructor-template.cpp` (CTAD), all in section 6.

## 8. Findings

### G161. The text does not say how a loop goes on after a jump into its body

- Page and rules: statements/if, IF-12; statements/switch, SW-1; statements/jumps, JMP-6; statements/loops; tests `statements/switch/case-inside-loop.cpp`, `statements/if/goto-into-branch.cpp`.
- Kind: missing.
- What happened: IF-12 lets a `goto` or a `case` label enter a branch, and says the condition is not evaluated. SW-1 says a label can be inside a loop of the switch body. No rule says what happens at the end of the loop body that a jump entered. The front end skips the init statement of a `for`, runs the rest of the body, then the step, and then tests the condition as usual. A `do` loop tests its condition. `case-inside-loop.cpp` expects that reading. The text also does not say that the variables of a skipped init statement exist with no value.
- Suggestion: add to the loops page that a jump into a loop body enters the loop at that point, and that after the body the loop goes on as after a normal pass. Say what JMP-6 makes of a skipped init statement.

### G162. LSUP-6 does not say who defines `std::nothrow`

- Page and rules: foundations/language-support, LSUP-6; expressions/new-delete, NEW-31; tests `foundations/language-support/nothrow-tag-self-declared.cpp`, `expressions/new-delete/bad-bound-nothrow.cpp`.
- Kind: ambiguous.
- What happened: LSUP-6 says the library defines the nothrow forms of the allocation and deallocation functions, and lists `std::nothrow` as an object that a program can declare. A program that declares `extern const std::nothrow_t nothrow;` and uses it needs a definition. The text does not say that the library gives it. The front end makes the object at its first use, as a front-end choice under decision 0035.
- Suggestion: say in LSUP-6 that the library defines `std::nothrow` too, so a program declares it and does not define it.

### G163. UNI-6 and the run test disagree on a construction inside a member that is not active

- Page and rules: classes/unions, UNI-6, UNI-14; Q-0425; test `classes/unions/construct-in-member-of-inactive-member-union-run.cpp`.
- Kind: contradictory.
- What happened: UNI-6 has the kind `undefined`, and its last sentence says that a construction of an object inside a member that is not active uses a member that does not exist. UNI-6 lists the run test in its `:tests:` line. The run test expects the program to run and return 0, and its note says the construction is valid at run time. A reader of UNI-6 predicts undefined behavior. The front end follows the run test: at run time, the construction makes the enclosing union members active. In constant evaluation it still rejects the construction with UNI-6.
- Suggestion: say in UNI-6 which reading holds at run time, or limit the sentence to constant evaluation while Q-0425 is open.

## 9. Decision 0060: the completion test

The completion line of `tools/frontend_check.py`:

```
frontend: completion (decision 0060): 127 tests fail their check, 2/123 program tests do not run to their result, 9 tests out of scope
```

At the start of the round the line read 175 tests and 3 of 123 program tests. The 127 tests are 69 missed rejects, 51 unsupported, 5 false rejects, and 2 EDG-AGREES. The two program tests are in section 6. By page:

| Page | Failing tests |
|---|---|
| classes/comparisons | 2 |
| classes/constructors | 3 |
| classes/conversion-functions | 3 |
| classes/copy-move | 2 |
| classes/members | 1 |
| classes/unions | 5 |
| classes/virtual | 2 |
| declarations/alignment | 1 |
| declarations/auto | 1 |
| declarations/constexpr | 1 |
| declarations/declarators | 2 |
| declarations/exception-specifications | 3 |
| declarations/functions | 1 |
| declarations/initialization | 3 |
| declarations/structured-bindings | 1 |
| exceptions/handlers | 1 |
| exceptions/unwinding | 1 |
| expressions/conditional | 1 |
| expressions/decltype | 3 |
| expressions/lambdas | 2 |
| expressions/logical | 1 |
| expressions/new-delete | 4 |
| expressions/postfix | 1 |
| expressions/primary | 2 |
| expressions/unary | 1 |
| foundations/constant-evaluation | 2 |
| foundations/language-support | 2 |
| foundations/names | 1 |
| foundations/storage | 1 |
| overloading/conversion-sequences | 7 |
| overloading/overload-resolution | 4 |
| programs/roadmap | 2 |
| statements/if | 1 |
| statements/range-for | 2 |
| templates/arguments | 5 |
| templates/class-template-deduction | 9 |
| templates/concepts | 5 |
| templates/constraints | 7 |
| templates/declarations | 3 |
| templates/deduction | 4 |
| templates/dependent-names | 10 |
| templates/instantiation | 6 |
| templates/ordering | 1 |
| templates/packs | 5 |
| templates/specialization | 2 |

24 tests stop in a run (`run-unsupported`). They pass their check, so the completion line does not count them.

Measures for the rewrite (implementer process item 5): the files over 2000 lines are `expr.rs` (7726), `decl2.rs` (7190), `interp/expr.rs` (4815), `templ.rs` (4310), `decl.rs` (3154), `expr2.rs` (2829), and `parser.rs` (2396), of 51375 lines in all. The first-error rate is 2087 of 2709 passing rejects (77%).

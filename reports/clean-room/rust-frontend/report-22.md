# Front end, round 22: the C++23 call forms, the phase 10 rule text, the W18, W19, and I3 findings, and older failures

Date: 2026-11-05. Decisions 0029, 0035, 0047, 0048, 0057 (phase 10, item FE).
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decision 0057, `documentation/exercises.md` section "Front end", the files under `specification/`, `tests/`, `frontend/`, and `tools/`, report 21, `reports/clean-room/roadmap-7/implementer.md` and `programmer-sketches/`, and the reports and reductions of W18, W19, and I3. No compiler was run, and no crates were added. The changed files are `frontend/README.md` and 15 files under `frontend/src/`. No file was added under `frontend/`.

The reader returned this report as a message, and the integrator saved it here.

The suite grew during the round. At the start, 56 tests were new against the baseline of report 21 (6619 tests), and 38 of them did not pass. During the round, 6 more tests arrived under `templates/concepts/`, `templates/constraints/`, and `templates/deduction/`. So the suite went from 6619 tests to 6681.

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each change, on the live tree. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, or `pass-partial`. A passed check is a full pass or a `run-` status. At the end of the round, `tools/frontend_check.py --update` wrote the new baseline to `conformance.tsv`.

### Counts

| Measure | Baseline `conformance.tsv` (6619 tests) | After, same 6619 tests | After, all 6681 tests |
|---|---|---|---|
| Full pass | 6362 | 6398 | 6459 |
| Check pass | 6414 | 6438 | 6499 |

Status by test:

| Status | Before (6619) | After (6681) |
|---|---|---|
| pass | 6089 | 6174 |
| EDG-ONLY (a pass) | 254 | 264 |
| EDG-ALLOWED (a pass) | 10 | 10 |
| pass-partial (a pass) | 9 | 11 |
| EDG-AGREES (a failure) | 2 | 2 |
| FALSE-REJECT | 6 | 4 |
| MISSED-REJECT | 118 | 116 |
| unsupported | 79 | 60 |
| run-unsupported | 52 | 40 |

The after column includes the 62 new tests. 61 of them pass. Passing rejects whose first error names a cited or linked rule: 1992 of 2600 at the start of the round, 2004 of 2609 after.

### The checks

`python3 -I tools/frontend_check.py`, before `--update`:

```
frontend: 6459/6681 pass, 6499/6681 pass their check, 0 worse, 36 better, 0 moved between passing statuses, 62 new (1 not passing)
frontend: 19/19 interpreter-only expectations met
```

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 177 examples (175 pass, 2 unsupported); 0 worse, 0 better, 0 new (0 not passing), 0 gone
```

Both checks exit 0.

### Results by target

**Target 1: the C++23 call forms.** Each test of this target passes.

- CLS-13, DED-8. A member function template whose explicit object parameter has a deduced type deduces it from the object of the call, as the first argument. `this Self&`, `this Self&&`, `this const Self&`, and `this auto&` all work, in a run and in constant evaluation, also with a qualified name (`g.G::get()`). The operator path does the same, so the two-argument `operator[]` with a deduced object works on a const and a non-const object.
- CLS-9, CLS-26. Two declarations whose explicit object parameters have different types, such as `this V&` and `this const V&`, declare two functions.
- OPR-3, CLS-15. A call of a class object takes a member `operator()` template and a static `operator()` template. In a class with conversion functions, the call operator templates join the surrogate candidates.
- LAM-15. A lambda with a template head is a generic lambda. A call deduces its parameters, or names them in `l.operator()<char>()`. `auto...` parameters give the call operator a trailing pack. Inside a template, `f.template operator()<Ts>()` works, also in a fold.
- LAM-16. A generic lambda with an explicit object parameter calls itself through that parameter, in a run and in constant evaluation. The front end gives the call operator its declaration before it parses the body. A capture in such a lambda is read through the explicit object.
- CAST-13. `auto(x)` and `auto{x}` copy `x` with its decayed type, also for a class.
- OVL-18, OVL-20. `&C::f<long>`, `&C::operator()<long>`, `&C::f` with a target type, and `C::f<long>` without `&` name a static member function template's specialization. `&f<int>` with no target type, where `f(T)` and `f(T*)` both match, is ambiguous (Q-0405).
- CONV-8. A zero literal converts to `decltype(nullptr)`, so an operator call that needs the conversion runs.
- LNK-2. A call through a pointer to a function of another unit runs that unit's function, when its parameters and result have scalar types. A captureless lambda of another unit converts and calls the same way.

**Target 2: the phase 10 rule text.**

- ORD-6. For two member templates, the object parameters name the more derived class, so a base member that a using declaration brings in orders against a derived member. The hiding of DER-3 now compares the templates of two specializations, so `A::f(T*)` and `B::f(T)` both stay candidates.
- COND-16. A reference does not bind directly to a class prvalue, also not to the class prvalue result of a conversion function (Q-0401).
- INST-12. A substitution failure in a waiting requires clause makes the constraint not satisfied. A requires expression on the call is false, and the call is an error that names CNS-12.
- BF-1, ALN-8. Explicit conversion functions do not count. Several functions with one target type are allowed, and the one that the conversion selects must be `constexpr`. The width and the operand are converted by that function.
- DCMP-1. A defaulted `==` compares an array of arrays element by element, down to the scalars.
- LSUP-2. A defaulted `<=>` with return type `auto` needs its ordering class declared before the defaulted declaration. A defaulted `<=>` that returns `std::weak_ordering` or `std::partial_ordering` needs `std::strong_ordering`.
- CMP-8, CVF-7, ACC-20, LAM-6. Their new tests passed at the start of the round.

**Target 3: the W18, W19, and I3 findings.**

| Finding | Status |
|---|---|
| W18-1 | Done (target 1, CLS-13). |
| W18-2 | Done (CLS-9). |
| W18-3 | Done. A reference to an array with more const elements, `const T (&)[N]`, deduces from a non-const array (DED-14). |
| W18-4 | Done (OPR-3). |
| W18-5 | Done (OVL-18). |
| W18-6 | Done (LAM-15). |
| W18-7 | Done. A designator that names a member of an anonymous union initializes the union with that member (UNI-7), also in constant evaluation. |
| W18-8 | Done. A lambda that the look-ahead of class template argument deduction parses keeps its closure type, so the deduced class names that type (CTAD-1). |
| W18-9 | The overload-set idiom runs. OVL-27 now counts the call operator that a generic lambda's call makes as a template specialization. The idiom also needs G152. |
| W18-10 | Done. A captureless generic lambda converts to a pointer to function when the specialization for the target's parameters has the target's type (LAM-10). |
| W18-11 | Done (LNK-2). The members of closure types no longer link across units, since each lambda has a type of its own. |
| W19-1 | Done (DCMP-1). |
| W19-3 | Done. `typename decltype(e)::type` parses (DEP-22). |
| W19-4 | Done (LAM-15). |
| I3-14 | Done. A function reference binds to the result of a conversion function (OVL-15). The interpreter gives a function lvalue a place that holds the function, so a reference to a function can be called and its address taken. |
| I3-15 | Done. A one-element braced list of a derived class initializes a base class parameter (ICS-19). |
| I3-16 | Done. A converting constructor whose parameter is a class runs, when the source class has no conversion functions (ICS-3). |
| I3-17 | Done. `static_cast<int&>(s)` binds the reference to the result of a conversion function, with explicit functions allowed (ICS-45). The requires-expression form now passes too. |
| I3-18 | Done. Direct initialization of a reference takes an explicit conversion function that returns the right kind of reference (ICS-45, OVL-15). |

All W18, W19, and I3 reductions pass, and so do the W18 and W19 programs and the three `r7-` program tests.

## 2. Old failures by group

The groups are those of `reports/clean-room/roadmap-7/implementer.md` section 2, item 3. "Before" counts the tests of the group that failed at the start of this round, on the current tree.

| Group | Before | After | What changed, or why not |
|---|---|---|---|
| The `same` idiom with variable templates (unsupported) | 17 | 15 | A pack with explicit elements takes the deduced elements after them (DED-20). The 15 left have unrelated causes: extended floating-point types in conversion sequences, a constrained function's specialization, constant template parameters of class type, and others. |
| Conversion to a function reference, conversion functions in constants | 25 | 11 | I3-14, I3-17, I3-18, OVL-29 (a reference to function prefers its own kind of result), OVL-14 (a conversion function that returns a reference to the class or a derived class). Left: ICS-45 checks inside a requires-expression (5), extended floating-point types (2), `operator auto()` (1), explicit conversion in a class direct initialization (1), a class array bound (2). |
| Concepts in `static_assert` | 3 | 3 | A lambda with a template head and a requires clause, and a placeholder parameter with a type constraint, stay unknown. |
| The `noexcept` operator | 4 | 4 | The operand of a member's `noexcept` that names a later member needs parsing at the end of the class. Not done. |
| `new` in a constant and in a run | 13 | 12 | Not targeted. One test passes after the conversion function changes. |
| Constructor calls in a run | 6 | 4 | An element that is not a list copy-initializes a class element of an array. Left: brace elision into a base, a defaulted copy constructor of a class with bases, two list forms. |
| BF-9 layout of classes with bit-fields | 4 | 4 | Not targeted. |
| Missed rejects: OVL-20 with no target | 5 | 2 | A cast to `void`, an expression statement, and `&f<int>` that leaves a parameter with no target are errors. Left: a constant template argument and an init capture. |
| Missed rejects: ACC-3, ACC-15, ACC-18 | 6 | 6 | Not targeted. |
| Missed rejects: DEFARG-13, LIST-13 and LIST-16, PACK-8, MPTR-11, DCMP-12 and DCMP-13 | 17 | 17 | Not targeted. |
| Missed rejects: the 9 instantiation rules | 9 | 9 | Not targeted. |
| Interpreter panic on `across-units-class-member.cpp` | 1 | 0 | The end of a static object runs in the unit that made it, so its class is looked up in that unit. The test passes. |
| EDG-AGREES (CPY-27, CNS-32) | 2 | 2 | The reasons of report 21 stand. |
| The four older false rejects | 4 | 2 | ODR-15: the redeclaration outside the class of a `constexpr` static member, and a static data member's explicit specialization with no initializer, are no definitions. Left: `tuple-member-get-type-parameter-uses-free.cpp` and `char-named-escape.cpp`. |
| `nondeduced-only-parameter.cpp` (ORD-3) | 1 | 1 | The front end keeps `typename Id<U>::type` as an opaque dependent type, so it cannot see that `U` appears only there. |

Other old tests that pass now: `trivially-copyable-types.cpp` (the bytes of a pointer to member, STG-12), `statements/range-for/braced-list.cpp` (a braced range is a `std::initializer_list` held for the loop), `anonymous-union-designated-member.cpp`, `overload-set-one-function.cpp`, `forwarding-placeholder-pack.cpp`, `generic-function-pointer.cpp`, and the two `explicit-object-recursion` tests.

## 3. Items left, with reasons

- `expressions/postfix/default-template-argument-incomplete-class.cpp` (CLS-2, Q-0406) stays a missed reject. The front end keeps the tokens of a default template argument and parses them at a use, when the class is complete. Checking them at the point of declaration needs a parse that separates dependent defaults from the rest.
- The ICS-45 tests in a requires-expression (`user-defined-explicit*.cpp`): a cast to a reference whose target class has constructors needs the front end to decide whether a temporary can be made, which it does not model for this form.
- `ellipsis-nullptr-becomes-void-pointer.cpp` and `ellipsis-argument-conversions.cpp` include `<cstdarg>`, and the front end reads no standard header.
- `class-prvalue-lvalue-private-copy.cpp` (COND-16) and `implicit-conversion-member-and-new-contexts.cpp` (CONV-20), as in report 21.
- The placement `new` sketch `placement_new_declared_only.cpp` stops at UNI-6. It has no test.

## 4. Tests added during the round

62 tests are new against the baseline: 56 present at the start of the round, and 6 that arrived during it. 61 pass. This one does not:

- MISSED-REJECT: `expressions/postfix/default-template-argument-incomplete-class.cpp` (section 3).

The 6 tests that arrived during the round are `templates/concepts/type-constraint-pack-same-concept-two-declarations.cpp`, `templates/constraints/template-id-in-called-member-body.cpp`, `templates/constraints/template-id-in-uncalled-member-body.cpp`, `templates/constraints/template-template-parameter-own-constraint-dependent.cpp`, `templates/deduction/constraint-failure-in-default-template-argument.cpp`, and `templates/deduction/constraint-failure-in-return-type.cpp`. All pass. For the default template argument test, an error in the substitution of a default template argument now makes deduction fail (DED-24).

## 5. Findings

### G152. The text does not say whether a candidate's body is instantiated when overload resolution only forms the candidate

- Page and rules: templates/deduction, DED-29; expressions/lambdas, LAM-15; overloading/overload-resolution, OVL-27; tests `expressions/lambdas/closure-as-base-overload-set.cpp`, `programs/W18/visitor.cpp`.
- Kind: missing.
- What happened: in `visitor.cpp`, an overload set of closures holds `[](const Circle&)` and a generic lambda whose body reads `t.base`. A call with a `Circle` deduces the generic call operator for `Circle`, and the plain one wins by OVL-27. The call operator has a deduced result type. DED-29 says that an error in a function body that substitution instantiates, for example to find its deduced result type, makes the program not valid. The text does not say whether forming the candidate instantiates that body. If it does, `t.base` with `T = Circle` is an error, and the program is not valid. The test expects a run. LAM-15 says nothing of when the body of a generic lambda's call operator is instantiated. The front end holds the errors of such a body and reports them only when a call selects that call operator. It records the choice in `frontend/README.md`.
- Suggestion: say in DED-29 or LAM-15 when overload resolution needs the deduced result type of a candidate, and whether a candidate that a call does not select has its body instantiated. Add a test where a generic lambda's body is not valid for an argument type and a plain overload wins.

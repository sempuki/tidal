# Front end, round 20: the false VIRT-12 verdict, the phase 8 repairs, the round 6 reductions, and the page examples

Date: 2026-11-04. Decisions 0029, 0035, 0047, 0048, 0056 (phase 9, item FE).
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decision 0056, the files under `specification/`, `tests/`, `frontend/`, and `tools/`, report 19, `reports/clean-room/roadmap-6/implementer.md` (section 1, item 1), `reports/clean-room/roadmap-6/programmer.md` (sections 1.3 to 1.5, item 2), and the "Matrix against review" section of `documentation/decisions/pending.md`. No compiler was run, and no crates were added. The changed files are `frontend/README.md` and 16 files under `frontend/src/`. No file was added.

Clean-room writers committed two changes during the round: `1bf7812` (item RV2) and `6f3d108` (item PU). RV2 added 58 tests, so the suite grew from 6525 tests to 6583. RV2 also changed LSUP-2: a use of `<=>` now needs only the ordering class that it yields (Q-0386). The new tests are judged only as new tests (section 3).

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each change, on the live tree. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, or `pass-partial`. A passed check is a full pass or a `run-` status.

### Counts

| Measure | Baseline `conformance.tsv` (6525 tests) | After, same 6525 tests | After, all 6583 tests |
|---|---|---|---|
| Full pass | 6196 | 6249 | 6282 |
| Check pass | 6269 | 6301 | 6334 |

Status by test:

| Status | Before (6525) | After (6583) |
|---|---|---|
| pass | 5957 | 6026 |
| EDG-ONLY (a pass) | 222 | 237 |
| EDG-ALLOWED (a pass) | 11 | 10 |
| pass-partial (a pass) | 6 | 9 |
| EDG-AGREES (a failure) | 20 | 14 |
| FALSE-REJECT | 12 | 9 |
| MISSED-REJECT | 138 | 140 |
| unsupported | 86 | 86 |
| run-unsupported | 63 | 52 |
| RUN-WRONG-RESULT | 7 | 0 |
| RUN-UNEXPECTED-UB | 3 | 0 |

The after column includes the 58 new tests. 33 of them pass and 25 do not (section 3). Passing rejects whose first error names a cited or linked rule: 1928 of 2523 before, 1952 of 2551 after.

### The checks

`python3 -I tools/frontend_check.py`:

```
frontend: 6282/6583 pass, 6334/6583 pass their check, 0 worse, 53 better, 1 moved between passing statuses, 58 new (25 not passing)
frontend: 19/19 interpreter-only expectations met
```

The one move is `override-independent-base-unused.cpp`, which was `EDG-ALLOWED` and now gives the base answer (`EDG-ONLY`).

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 163 examples (162 pass, 1 unsupported); 0 worse, 6 better, 0 new (0 not passing), 0 gone
```

Both checks exit 0. This round does not change `conformance.tsv` or `examples.tsv`.

### Results by target

**Target 1: the false VIRT-12 verdict.** The delete check took a base's destructor as virtual only if the base declared it virtual. It now walks the bases, so an implicit or declared destructor of a middle class that overrides a virtual destructor counts (VIRT-7). `virtual-dtor-delete-through-middle.cpp`, `virtual-dtor-delete-through-middle-declared.cpp`, and `programs/roadmap/r6-scheduler.cpp` pass.

**Target 2: the phase 8 repair tests.**

- I1 and CLS-35. An empty `[[no_unique_address]]` member of empty class type keeps a class empty. A union's data size is the largest `sizeof` of a member. `empty-class-cases.cpp` and `layout-union-size.cpp` pass. The rule's sentence on the data size of an empty subobject at another offset is not applied (G142).
- G136 and CTOR-17. The front end records a default member initializer that binds a temporary. The error comes from a constructor that uses it:
  - a user constructor without a `mem_init` for the member,
  - an implicit or defaulted default constructor that a default-initialization uses.

  A `std::initializer_list` member set from a braced list counts as well. `reference-default-member-unused.cpp`, `reference-default-member-aggregate-value.cpp`, and the two `initializer-list-*-from-braces.cpp` tests pass.
- G139 and STG-29.
  - A cast into a byte array that is a member now gives a pointer to a byte, as for a complete object.
  - Placement `new` at such a byte finds the element of the member array.
  - Addresses compare by the offset in the complete object.

  Both `member-byte-array-cast-*` tests pass. An access through the cast pointer stops with STG-23 and STG-29, and `programs/W15/optional.cpp` still runs to result 0.
- G141 and RFOR-7. A data member `begin` or `end` of a class type with no `operator()` and no conversion function is an error. A data member that can be called takes the member form in the run. `data-member-begin-not-callable-class.cpp`, `data-member-function-pointers.cpp`, and `callable-data-members.cpp` pass.
- X18. All five X18 tests pass:
  - `override` or `final` twice is an error (VIRT-3).
  - An implicit destructor that overrides a `final` destructor is an error.
  - VIRT-13 warns about an override without `override` or `final` in a class where another override has `override`.
  - In a class template whose bases do not depend on a parameter, a function marked `override` is an error when no virtual function of a base has its name.
- X19: an integer suffix that is not a form of the syntax names a literal operator (OPR-8, OPR-9). With no such operator, the error stays PRIM-16. A hexadecimal, octal, or binary literal with `z` that only `std::size_t` holds has that type. The three X19 tests pass.
- LNK-1: a function whose parameter has a class type of an unnamed namespace links only within its unit. `class-unnamed-namespace-internal.cpp` passes.
- ACC-20: a friend declaration without a template head names one specialization, and TSP-12 applies only with a head. `friend-specialization-only.cpp` passes, and `friend-one-specialization.cpp` still rejects.
- TPL-23: a member defined after a constrained class template must repeat the constraints (CNS-26). `member-outside-clause-missing.cpp` passes.

**Target 3: the round 6 reductions.**

- UNI-13: a chain that starts at the implicit object of a member function activates the union members that it names. `chain-assign-member-function.cpp` and its constant form pass.
- An aggregate element copy-initializes a member whose class has a user-declared constructor, without brace elision. `aggregate-member-converting-constructor.cpp` passes.
- An assignment from an rvalue of another class makes the temporary through the converting constructor, then selects among the declared assignment operators. The copy or move constructor of a class whose only conversion function is an explicit `operator bool` no longer gives up: OVL-40 now looks only at conversion functions that can give the class. `assign-converted-owner.cpp`, `programs/W16/evaluator.cpp`, and `programs/roadmap/r5-evaluator.cpp` run.
- A member of a class template defined after the class is no longer a candidate function of the namespace. `member-operator-outside.cpp` and `programs/W17/units/main.cpp` run. All program tests now run.
- MPTR-8: inside a member template of a class specialization, a name bound to a type that is not a class is an error. `member-template-pointer-to-member-non-class.cpp` passes.
- LSUP-2: the round started with checks for all three ordering classes. RV2 then changed the rule and the tests, and the checks were taken out. The three `ordering-strong-only-*` tests pass under the new text.

**Target 4: the page examples.** All six examples pass:

- `classes/virtual.md`: an implicit destructor that overrides a `final` destructor, from target 2.
- `classes/derived.md`: the front end now checks the access of every chosen constructor, also in a default-initialization. Before, it checked only inherited constructors.
- `declarations/constexpr.md`: a call of a `consteval` constructor outside an immediate context is a constant expression.
- `templates/arguments.md`: a member type named by a qualified name outside its class is checked for access. The check skips these places:
  - the declaration of a member defined after its class,
  - an explicit instantiation or explicit specialization,
  - a name that a using-declaration makes accessible.
- `templates/dependent-names.md`: in a class template with a dependent base, an unqualified name that no searched scope declares is an error at the definition (DEP-12).
- `templates/specialization.md`, TSP-26: an error for `template <>` after the head of an enclosing template, and for fewer `template <>` heads than class template-ids in the qualifier.

`templates/instantiation.md` line 718 stays unsupported.

**Target 5: EDG-AGREES and wrong run results.**

- UNW-1 (pending.md keeps GCC's order). When a destructor throws as a block ends, the other objects of the block still end. A class result that a return statement built ends before a handler of the same function runs. Both UNW-1 tests pass.
- OBJ-11 (pending.md keeps the matrix). A `thread_local` object with dynamic initialization is built at the first use of such a variable of its unit. `frontend/README.md` states the choice.
- LSP-8: a global variable and a C variable of one spelling link as one variable across units. The one declaration of a linkage specification without braces declares a variable as `extern` does. Both LSP-8 tests pass.
- OVL-38: two default arguments compare as expressions, so `5` and `2 + 3` differ and `5` and `0x5` do not.
- LSP-4: a second definition of a C function of one spelling is an error, also in an unnamed namespace.
- DEFARG-10: a `consteval` call in a default argument is not checked at the declaration. A redeclaration of a `consteval` function no longer counts as a `constexpr` mismatch (CX-5).
- IF-25: the initializer of a local reference runs in constant evaluation when it is a constant expression that refers to a variable with static storage duration. `consteval-local-reference-static-object.cpp` (new) passes.
- CTOR-17: an aggregate initialization that leaves a reference member to its default member initializer extends the temporary to the aggregate. `reference-default-member-aggregate-uses-default.cpp` (new) passes.

## 2. Items left, with reasons

- Ten EDG-AGREES tests from before the round remain, with the reasons of report 19: CPY-27 (`trivially-copyable-member-template-selected.cpp`), CTAD-18, CNS-32, CNS-25, DED-11, INST-6, and the three ORD tests. `class-const-and-volatile-lvalues.cpp` (COND-16) also remains.
- Phase 8 repair tests that still fail:
  - `ellipsis-argument-conversions.cpp` (PFX-22): the test includes `<cstdarg>`, and the front end reads no standard header.
  - `implicit-conversion-member-and-new-contexts.cpp` (CONV-20): unsupported.
- DEFARG-10 says that each call that uses the default argument checks the immediate call. The front end does not yet check it at the call.
- `range-for/braced-list.cpp` stays run-unsupported: the run does not lower a braced list as the range.
- 52 run tests are unsupported. The largest groups are `new` forms, constructor calls, and jumps into a nested statement.

## 3. Tests added during the round

RV2 added 58 tests. 33 pass. These 25 do not:

- MISSED-REJECT (16):
  - `friend-member-of-specialization-other-member.cpp`
  - `anonymous-union-two-designators.cpp`
  - three `bitfield-width-class-*.cpp`
  - `abstract-call-argument.cpp`
  - `operand-class-conversion-ambiguous.cpp`
  - `nodiscard-on-type-alias.cpp`
  - three `typedef-unnamed-class-*.cpp`
  - `base-and-member-pointers-constant.cpp`
  - `base-subobject-pointers-constant.cpp`
  - `class-base-const-derived-prvalue-rejected.cpp`
  - `constant-double-to-float-exact.cpp`
  - `constant-float-to-double.cpp`
- EDG-AGREES (4): `template-deduced-auto.cpp`, `capture-default-local-class-default-argument.cpp`, `capture-default-local-class-member.cpp`, `local-class-requires-clause-waits.cpp`.
- FALSE-REJECT (4): `ref-qualified-mixed-different-cv.cpp`, `ellipsis-nullptr-becomes-void-pointer.cpp` (`va_list`), `definition-class-base-sees-namespace.cpp`, `definition-enumeration-body-sees-namespace.cpp`.
- unsupported (1): `class-base-does-not-convert-to-derived.cpp`.

## 4. Findings

### G142. CLS-35 adds a byte to the data size for an empty subobject that a test places beside a member

- Page and rules: classes/members, CLS-35; tests `layout-empty-bases-depths.cpp`, `layout-union-size.cpp`.
- Kind: ambiguous.
- What happened: CLS-35 says that an empty subobject at offset 0 adds nothing to the data size, and that an empty subobject at another offset adds one byte. In `layout-empty-bases-depths.cpp`, `Depths : Red, Blue { int x; }` puts `Red` at 0 and `Blue` at 1. The test expects `x` at offset 0 and a size of 4. If `Blue` adds one byte, the data size so far is 2, and the Padding row starts `x` at 4. The front end applied the sentence, and the test failed. The front end does not apply it.
- Suggestion: say where the extra byte counts. If it counts only for the end of the class (the size), say so, and say that the start of the next member uses the data size without it. Add a test with an empty base at offset 1 and a later `char` member.

### G143. LSP-8 does not say what a run of such a program gives

- Page and rules: declarations/linkage-specifications, LSP-8; tests `c-variable-and-global-variable-two-units.cpp`, `global-variable-and-c-variable-two-units.cpp`.
- Kind: missing.
- What happened: LSP-8 covers a C variable in one unit and a global variable with C++ language linkage and the same spelling in another unit. It says the program is not valid, with no diagnostic required. Both tests expect a run with result 0, which is the result when the two declarations name one variable. The page does not say that a run treats them as one variable. The interpreter now links them, as Clang and GCC do (`frontend/README.md`).
- Suggestion: say in LSP-8 what the matrix targets do, as OBJ-11 states its choice, or say that the run result is the matrix's choice that the tests record.

### G144. VIRT-13 does not say whether a destructor counts as an override for the warning

- Page and rules: classes/virtual, VIRT-13, VIRT-7; test `missing-override-inconsistent-warns.cpp`.
- Kind: ambiguous.
- What happened: VIRT-13 asks for a warning on an override with neither `override` nor `final`, in a class where another override has `override`. By VIRT-7, a declared destructor overrides a virtual destructor of a base. The page does not say whether such a destructor without `override` gets the warning. The front end warns for it, because the text says "an override". No test has a class with a marked override and an unmarked destructor.
- Suggestion: say whether a destructor counts, and add a test with an unmarked destructor beside a marked override.

### G145. OVL-38 does not say whether two default arguments with the same spelling and different entities are the same

- Page and rules: overloading/overload-resolution, OVL-38 (Q-0140); tests `default-from-two-scopes-same-value.cpp`, `default-from-two-scopes-same-literal.cpp`.
- Kind: ambiguous.
- What happened: OVL-38 now says that two default arguments are the same if they are the same expression, and that two literals with the same value are the same. The two declarations are in different namespaces, and a default argument `n` in each namespace can name a different variable `n`. The page does not say whether two such arguments are the same expression. The front end compares the trees, which name the declarations, so it treats them as different.
- Suggestion: say whether "the same expression" means the same tokens or the same entities, and add a test with a name that each namespace declares.

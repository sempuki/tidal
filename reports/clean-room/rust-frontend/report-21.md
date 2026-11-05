# Front end, round 21: the I2 faults, the RV2 tests, the older EDG-AGREES tests, and DEFARG-10 at the call

Date: 2026-11-04. Decisions 0029, 0035, 0047, 0048, 0056 (phase 9, item FE).
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decision 0056, the files under `specification/`, `tests/`, `frontend/`, and `tools/`, report 19, report 20, and `reports/clean-room/I2/report.md` (findings I2-13 to I2-16) with its reductions. No compiler was run, and no crates were added. The changed files are `frontend/README.md` and 12 files under `frontend/src/`. No file was added.

Clean-room writers changed the tree during the round. Commit `d9c18bc` (repairs from I2 and X21) added 20 tests and changed ORD-3, ORD-6, ORD-9, and ORD-17. Commits `d732ac1` and `f2ac11c` added no tests. At the end of the round the working tree held 8 more new tests, 2 changed tests, and changes to 16 specification pages. The new tests are four under `templates/ordering/`, `templates/concepts/nested-requirement-never-satisfied.cpp`, and the three `templates/deduction/substitution-failure-negative-bound*.cpp`. So the suite grew from 6591 tests to 6619. The new tests are judged only as new tests (section 3).

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each change, on the live tree. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, or `pass-partial`. A passed check is a full pass or a `run-` status.

### Counts

| Measure | Baseline `conformance.tsv` (6591 tests) | After, same 6591 tests | After, all 6619 tests |
|---|---|---|---|
| Full pass | 6288 | 6336 | 6362 |
| Check pass | 6340 | 6387 | 6414 |

Status by test:

| Status | Before (6591) | After (6619) |
|---|---|---|
| pass | 6031 | 6089 |
| EDG-ONLY (a pass) | 238 | 254 |
| EDG-ALLOWED (a pass) | 10 | 10 |
| pass-partial (a pass) | 9 | 9 |
| EDG-AGREES (a failure) | 14 | 2 |
| FALSE-REJECT | 10 | 6 |
| MISSED-REJECT | 141 | 118 |
| unsupported | 86 | 79 |
| run-unsupported | 52 | 52 |

The after column includes the 28 new tests. 26 of them pass and 2 do not (section 3). Passing rejects whose first error names a cited or linked rule: 1955 of 2555 before, 1990 of 2593 after.

### The checks

`python3 -I tools/frontend_check.py`:

```
frontend: 6362/6619 pass, 6414/6619 pass their check, 0 worse, 48 better, 0 moved between passing statuses, 28 new (2 not passing)
frontend: 19/19 interpreter-only expectations met
```

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 173 examples (172 pass, 1 unsupported); 0 worse, 0 better, 12 new (0 not passing), 2 gone
```

The 12 new examples and the 2 gone come from the writers' changes to the pages. Both checks exit 0. This round does not change `conformance.tsv` or `examples.tsv`.

### Results by target

**Target 1: the I2 faults.**

- ORD-6, I2-13. The operator path ordered two specializations only when both were members or both were not. A member against a non-member now compares with the member's object parameter. A tie between the two is now an ambiguity. Before, the front end fell back to an older selection that preferred the member. `member-object-rvalue.cpp` passes. The front end now rejects the reduction `r1b-member-lvalue.cpp` as ambiguous, which matches the I2 model.
- ORD-13, I2-14. Constraints decide between two conversion function templates only if the result types are the same, top-level qualifiers included. `constraints-conversion-results-differ.cpp` passes.
- ORD-7, I2-15. A swapped member candidate gets its object parameter first, and then the two parameters are reversed. `reversed-member-object.cpp` passes.
- DED-37, I2-16. A pack expansion whose pattern is more than a bare parameter, such as `A&...`, `A*...`, or `List<A>...`, deduces each element against the pattern. Each pack in the pattern collects its values in order. `ordering/pack-class-arguments.cpp` and all I2 reductions pass.

The writers' tests from `d9c18bc` on the same rules gave four more changes:

- ORD-6 now gives every member an object parameter. For two members of one class, both get one. The kind of reference follows the other member's ref-qualifier. ORD-13 compares the parameter lists with the object parameters (`constraints-member-object.cpp`).
- ORD-9: the two whole function types form one pair, and ORD-2 removes nothing inside them. Two conversion function templates still compare by their result types (ORD-8).
- ORD-17: an invented value of an `auto` parameter has an invented type, which no parameter of a fixed type takes. Constraints of two partial specializations decide only between equivalent heads (`partial-specializations-int-against-constrained-auto.cpp`).
- A `static_cast` to a pointer to function selects from an overload set or a template-id (`address-reference-against-value.cpp`).

**Target 2: the RV2 tests and the two new tests.** 23 of the 25 RV2 tests and both new tests of `b7e0adf` now pass:

- ACC-20: a templated friend `C<T>::f` names the member `f` of each specialization, and no other member. Before, any templated friend opened all access.
- UNI-7, LIST-14: a designator that names a second member of one anonymous union is an error.
- BF-1, BF-2, ALN-8: a width or `alignas` operand of class type needs a literal class with exactly one non-explicit conversion function to an integral or unscoped enumeration type, and that function is `constexpr`.
- VIRT-8: a call of a function with a parameter of abstract class type is an error.
- ATTR-2: `nodiscard` or `noreturn` on a type alias is an error.
- TYN-9: a class named for linkage by a typedef must have no member alias, no static data member, no lambda expression, and no member class that breaks these limits.
- CMP-8: during constant evaluation, a relational comparison of two subobjects of one object is not a constant when the paths split at a base class subobject.
- COND-16, COND-5, COND-3 for two operands of class type:
  - The three steps of COND-16 are implemented, with the table of step 3.
  - These cases are errors: both operands convert, neither converts, or the operands still differ after the conversion.
  - Operands of one type and two categories give a prvalue, which copies the glvalue.
  - An ambiguous conversion sequence of a class operand is an error.
  - 15 COND tests pass, among them 9 that are not RV2 tests.
- TARG-13: a floating argument of another floating-point type is an error.
- CVF-7: a conversion function template whose name has a placeholder is an error.
- LAM-6: a capture default is an error in a lambda that a member declaration or a default argument of a class holds, with no block between. The body of a member function counts as a block.
- INST-12: see G149.
- CLS-26: overloads that differ in cv-qualifiers can mix ref-qualifiers.
- NAM-21: the base clause of a class and the enumerators of an enumeration defined with a qualified name see the qualifier's scope.
- VIRT-13: a destructor without `override` gets no warning (`missing-override-destructor-unmarked-no-warning.cpp`, new in `b7e0adf`).
- DEFARG-10: see target 4 (`default-immediate-call-used-not-constant.cpp`, new in `b7e0adf`).

**Target 3: the older EDG-AGREES tests.** 8 of 10 pass:

- COND-16: `class-const-and-volatile-lvalues.cpp`, from target 2.
- DED-11: a function type keeps `noexcept(B)` with the constant parameter `B`. Deduction gives `B` the argument's `noexcept`, and substitution gives the type its `noexcept`. Such a type is dependent, as DEP-9 needs.
- CNS-25: the front end records the constraints of a template template parameter's head. An argument whose constraints the head does not subsume is an error.
- CTAD-18: through an alias template, one element whose type is a specialization of the class template that the alias names skips the initializer-list phase.
- INST-6: two member partial specializations that become one in a class specialization are an error.
- ORD-6, ORD-7, ORD-13: the three ORD tests, from target 1.

The two left are in section 2.

**Target 4: DEFARG-10 at the call.** A call that uses a default argument now checks each call of an immediate function in that default argument as a constant. The check runs only outside an immediate function context.

**Other.** In explicit template arguments, a pack takes every argument after it, also those at the places of the invented parameters of `auto`. Before, a later argument went to the invented parameter and broke the order. `placeholder-parameter-after-pack.cpp`, `pack-then-deducible.cpp`, and the new example at `templates/arguments.md` line 550 pass.

## 2. Items left, with reasons

- EDG-AGREES, with the reasons of report 19:
  - `trivially-copyable-member-template-selected.cpp` (CPY-27) needs overload resolution of a constructor template inside the implicit copy constructor.
  - `friend-distinct-from-namespace.cpp` (CNS-32) needs the front end to keep a friend template apart from the namespace template of the same name.
- `ellipsis-nullptr-becomes-void-pointer.cpp` (RV2) and `ellipsis-argument-conversions.cpp` (PFX-22) include `<cstdarg>`, and the front end reads no standard header.
- `class-prvalue-lvalue-private-copy.cpp` (COND-16): the front end does not check the access of a copy constructor that a copy-initialization selects. `S t = s;` with a private copy constructor is also accepted.
- `nondeduced-only-parameter.cpp` (ORD-3, new): the front end keeps `typename Id<U>::type` as an opaque dependent type, so it cannot see that `U` appears only there.
- `across-units-class-member.cpp` (new) stays run-unsupported. The run of the two units stops inside the interpreter, because it looks up a class of the second unit among the classes of the first.
- This round did not target these older failures:
  - four FALSE-REJECT tests: `tuple-member-get-type-parameter-uses-free.cpp`, `char-named-escape.cpp`, `constexpr-member-redeclared-outside.cpp`, and `explicit-specialization-without-initializer.cpp`
  - `implicit-conversion-member-and-new-contexts.cpp` (CONV-20, unsupported)
  - `templates/instantiation.md` line 718

## 3. Tests added during the round

28 tests are new: 20 from `d9c18bc` and 8 untracked in the working tree. 26 pass. These 2 do not:

- MISSED-REJECT: `templates/ordering/nondeduced-only-parameter.cpp` (section 2).
- run-unsupported: `declarations/initialization/static/across-units-class-member.cpp` (section 2).

## 4. Findings

### G146. ORD-9 does not say whether `T&&` matches an invented `X&` inside a function type

- Page and rules: templates/ordering, ORD-9, ORD-3; templates/deduction, DED-P3; test `address-reference-kinds.cpp`.
- Kind: missing.
- What happened: the test orders `read(T&)` against `read(T&&)` by their whole function types and expects `read(T&)`. That needs `T&&` in `int(T&&)` to be deduced from the invented `int(X&)`, with `T = X&`. ORD-3 removes the forwarding reference of DED-8. The table of DED-P3 lists `T&&` as a form that gives `T`, and it does not say whether `T&&` matches an lvalue reference type. With exact matching, neither direction succeeds and the address is ambiguous. The front end lets `T&&` take `X&` only inside the whole function types of ORD-9 (`frontend/README.md`).
- Suggestion: say in ORD-9 or DED-P3 whether `T&&` matches an lvalue reference type by reference collapsing, and in which contexts.

### G147. ORD-6 does not say which class the object parameters name for members of two classes

- Page and rules: templates/ordering, ORD-6; tests `member-objects-lvalue-and-none.cpp`, `member-objects-rvalue-and-none.cpp`.
- Kind: missing.
- What happened: ORD-6 now gives each member template an object parameter of type reference to its class. Two member templates can come from different classes, for example a base member that a using-declaration brings into a derived class. Their object parameters then name two classes, so neither is deduced from the other, and neither template is more specialized. The page does not say whether that is the intent. The front end adds the object parameters only when both members belong to one class.
- Suggestion: say which class the object parameter names when the members come from different classes, for example the class of the object argument. Add a test with a base member template that a using-declaration brings in.

### G148. COND-16 does not say whether a reference binds directly to a class prvalue

- Page and rules: expressions/conditional, COND-16 step 2; glossary, "bind directly".
- Kind: ambiguous.
- What happened: step 2 asks whether a `T&&` binds directly to `X` when `Y` is an xvalue. The glossary says a reference binds directly when no temporary object is in between. A class prvalue `X` of `T` or a derived class becomes a temporary object when the reference binds to it. So the text can be read both ways, and the result is an xvalue in one reading and a prvalue in the other. The same holds for a conversion function that returns a class prvalue. The front end counts both as binding directly. No test checks the case.
- Suggestion: say whether binding to a class prvalue, or to the class prvalue result of a conversion function, counts as binding directly. Add a test with a `Derived` prvalue against a `Base` xvalue.

### G149. INST-12 does not say what a failure in a waiting requires clause does

- Page and rules: templates/instantiation, INST-12 (Q-0392); tests `local-class-requires-clause-waits.cpp`, `local-class-requires-clause-checked-at-use.cpp`.
- Kind: missing.
- What happened: the requires clause of a member of a local class is checked when a use needs it. In the waiting test, the clause names `T::value`, and `int` has no such member. The text does not say what a use does with such a clause: whether the substitution failure makes the constraint not satisfied, so the member is not viable, or makes the program not valid. The checked-at-use test has a clause that is false, not one that fails. The front end parses the clause with its errors held back and keeps a clause with an error as a constraint it does not model. A call of such a member stays unsupported.
- Suggestion: say what a substitution failure in the waiting clause gives at the use. Add a test that calls `get` in the waiting test's class.

### G150. CMP-8 does not say whether a member of a base subobject "involves a base class subobject"

- Page and rules: expressions/comparison, CMP-8 (Q-0387); tests `base-subobject-pointers-constant.cpp`, `base-and-member-pointers-constant.cpp`.
- Kind: ambiguous.
- What happened: during constant evaluation, a comparison "that involves a base class subobject" is not a constant. The tests compare a pointer to a base subobject itself. A pointer to a member of a base subobject, compared with a member of the derived class, can be read as involving the base or not. The front end rejects a comparison when the two places split at a base class subobject, so it rejects that case too.
- Suggestion: say whether a pointer into a base subobject counts. Add a constant test that compares `&d.base_member` with `&d.own_member`.

### G151. BF-1 and ALN-8 do not say whether explicit conversion functions count

- Page and rules: classes/unions, BF-1; declarations/alignment, ALN-8; tests `bitfield-width-class-two-conversions.cpp`, `operand-class-conversion-ambiguous.cpp`.
- Kind: missing.
- What happened: the class must have exactly one conversion function that gives an integral or unscoped enumeration type. The rules do not say whether an `explicit` conversion function, or a conversion function template, counts toward "exactly one". The front end leaves out explicit conversion functions and gives up on a class with a conversion function template.
- Suggestion: say which conversion functions count. Add a test with one `explicit operator int` and one `operator long`.

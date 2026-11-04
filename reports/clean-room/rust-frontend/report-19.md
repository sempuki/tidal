# Front end, round 19: the specification's answer on the EDG tests, the phase 8 repairs, and the run constructs

Date: 2026-11-04. Decisions 0029, 0035, 0047, 0048, 0055 (phase 8, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `frontend/`, and `tools/`, report 17, report 18, `reports/clean-room/roadmap-5/implementer.md` (section 1, item 2), and `reports/clean-room/roadmap-5/programmer.md` (sections 1.4 and 6). No compiler was run, and no crates were added. The changed files are `frontend/README.md` and 17 files under `frontend/src/`. No file was added.

A clean-room writer committed four changes during the round: `7a68692` (item RV), `e11f565` and `dad8bf8` (items PQ and TE), and `e4276f1` (exercise I1). The suite grew from 6355 tests at `39b5a6d` to 6416 tests. The 61 new tests are judged only as new tests. Six of them agree with EDG against the base expectation under both front ends (section 3).

## 1. Conformance

### Method

The front end of commit `39b5a6d` was built in a scratch directory. Both front ends ran `conform -v` on one copy of `tests/` and `specification/`. The copy was taken after the last commit of the other writer, so the before and after counts cover the same 6416 tests. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, or `pass-partial`. A passed check is a full pass or a `run-` status.

### Counts

| Measure | Baseline `conformance.tsv` (6355 tests) | Before, same 6416 tests | After, 6416 tests |
|---|---|---|---|
| Full pass | 5987 | 6018 | 6123 |
| Check pass | 6062 | 6097 | 6181 |

Status by test, before and after, on the 6416 tests:

| Status | Before | After |
|---|---|---|
| pass | 5861 | 5891 |
| EDG-ONLY (a pass) | 137 | 219 |
| EDG-ALLOWED (a pass) | 16 | 9 |
| pass-partial (a pass) | 4 | 4 |
| EDG-AGREES (a failure) | 90 | 16 |
| FALSE-REJECT | 10 | 4 |
| MISSED-REJECT | 135 | 131 |
| unsupported | 84 | 84 |
| run-unsupported | 72 | 54 |
| RUN-WRONG-RESULT | 4 | 4 |
| RUN-MISSED-UB | 1 | 0 |
| RUN-UNEXPECTED-UB | 2 | 0 |

Full passes rise by 105 and check passes by 84. The count of passing rejects whose diagnostic names one of the test's rules is 1737, and was 1686.

### The check against the baseline

`python3 -I tools/frontend_check.py` (without `--update`), against the committed baseline (6355 tests), on the live tree:

```
frontend: 6123/6416 pass, 6181/6416 pass their check, 0 worse, 105 better, 7 moved between passing statuses, 61 new (30 not passing)
frontend: 15/15 interpreter-only expectations met
```

The check exits 0. The 7 moves are tests that were `EDG-ALLOWED` and now give the base answer (`EDG-ONLY`). This round does not change `conformance.tsv`.

### Tests that change status

| Change | Tests |
|---|---|
| EDG-AGREES, now EDG-ONLY | 74 |
| run-unsupported, now pass | 17 |
| EDG-ALLOWED, now EDG-ONLY | 7 |
| FALSE-REJECT, now pass | 6 |
| MISSED-REJECT, now pass | 4 |
| RUN-UNEXPECTED-UB, now pass | 2 |
| RUN-MISSED-UB, now pass | 1 |
| run-unsupported, now EDG-ONLY | 1 |

**Target 1: the 84 EDG-AGREES tests.** 74 now give the base answer. 52 were missed rejects and 22 were false rejects. The 10 left are in section 3.

**Target 2: the tests of the repairs and of items R2 and RT.**
- `byte-array-rows-host-objects.cpp` and `byte-array-member-of-array-hosts.cpp` (STG-4, STG-6) pass. The false `undefined` verdict of report 18 is gone.
- `placement-then-cast-read.cpp` (STG-29) stops with STG-23 and STG-29, so its interpreter-only line is met.
- The NAM-21 false rejects pass: `tuple-protocol-page-example.cpp`, `definition-class-body-sees-namespace.cpp`, and `definition-template-member-alias.cpp`.
- `move-assign-only-from-prvalue.cpp` and `return-owner-converts-to-base-owner.cpp` run to result 0.
- `qualified-definition-inline-member-inside-named.cpp` (NS-15), `tuple-index-size-type.cpp` (SB-5), `two-classes-promoted-types.cpp` (COND-11), and `flow-off-end-constant-paths.cpp` (FN-7) pass.

**Target 3: CONV-11.** `floating-narrowing-infinity-run.cpp` and `floating-narrowing-out-of-range-run.cpp` run to result 0. `floating-narrowing-infinity.cpp` and its ILP32 form were `EDG-ALLOWED` and now give the base answer.

**Target 4: run constructs.** These 17 tests now run:
- the seven C library tests: `no-handler-objects-kept.cpp`, `noexcept-boundary-callee-objects-destroyed.cpp`, `noexcept-boundary-own-objects-kept.cpp`, `noexcept-boundary-unmatched-try.cpp`, `class-new-only-global-delete.cpp`, `new-single-object-sized-delete.cpp`, and `new-array-destructor-sized-delete.cpp`,
- the three addresses of an explicit object member function,
- two block-scope `extern` declarations,
- `main-three-params.cpp`,
- the two CONV-11 runs, and the owner and move-assignment programs above.

A third block-scope `extern` test, `extern-in-block-hidden.cpp`, now runs and passes as `EDG-ONLY`.

**Target 5.** VOL-6 and OBJ-12 now stop the run (section 2). No test has an interpreter-only line for them yet, so the counts do not show it.

Four more tests pass along the way: `explicit-object-not-member-pointer.cpp`, `friend-declaration.cpp`, `function-unevaluated-braced.cpp`, and `expansion-to-alias-parameters.cpp`.

## 2. What changed

### The check

- ACC-2: a using-declaration in a class gives the members it names the access of the using-declaration. A member named through it outside that access is an error (`using-reduces-access.cpp`).
- ALN-7: `alignas` on an enumeration is an error. ALN-8 and BF-2: an operand or width of scoped enumeration type is an error.
- CE-19 and EXS-7: the condition of `explicit(...)` and the operand of `noexcept(...)` must not narrow in the conversion to `bool`.
- CLS-14: no default argument on an explicit object parameter.
- VIRT-19: an explicit object member function with the name and parameters of a base virtual function is an error. Before, the front end's override search matched such a function.
- CLS-23: a `static const volatile` integer member takes no initializer in the class.
- CNS-18: a lambda's trailing requires clause is parsed outside templates. A lambda whose constraints are not satisfied has no conversion to a pointer to function.
- CNS-31: a friend with a trailing requires clause must be a definition when it is not a template, or when its constraints use a parameter of the enclosing class template.
- CTOR-17: a mem-initializer or a default member initializer that binds a reference member to a temporary is an error (G136).
- CTOR-18: a default argument of a constructor that makes an object of a nested class with a default member initializer is an error.
- CTOR-24: a class whose data members are all in anonymous unions without default member initializers does not meet CTOR-4.
- DCMP-8: a comparison defaulted after its class that would be deleted is an error.
- DEP-27 and DEP-34: `template` after the qualifier of a declared name, and `template` before a destructor name, are errors.
- DTOR-3: an array `new` of a class whose destructor is deleted or not accessible is an error.
- FN-6 and TR-3: `main` accepts `()`, `(int)`, `(int, char**)`, and `(int, char**, char**)`. These are the lists that Clang accepts. The others are errors (G137).
- FN-7: an `if` with a constant condition takes only the selected branch.
- ICS-21 comes before ICS-22. ICS-31 also compares a pointer to an array of known bound with one of unknown bound.
- INST-5: the errors in the initializer of an `inline` static data member of a class specialization are dropped, as for scoped enumerations.
- INST-7: a `constexpr` function named in a braced initializer of an unevaluated operand is instantiated.
- INST-21, INST-22, and INST-29, for variable templates and class templates:
  - the namespace and the qualified name of the explicit instantiation,
  - the written array bound,
  - an explicit instantiation declaration of a specialization with internal linkage.
- LAM-9, ODR-3: a copy of a whole class object of an enclosing function uses the variable. A constant with a `mutable` member at any depth does not count as a constant for this test. CLS-28: a member function of a local class that binds a reference to such a variable is an error.
- LIST-2: an unscoped enumeration without a fixed type narrows by the values of ENUM-22, and a bit-field narrows by its width.
- LIST-9: parenthesized aggregate initialization with more values than members, where a member is an aggregate, is an error.
- NAM-21: the body of a class defined with a qualified name sees the qualifier's scope. This includes an explicit specialization such as `std::tuple_size<Range>`. The parameters and body of a member of a class template defined outside its class see the pattern class.
- NEW-24, DED-39: a matching deallocation function and a matching template are two matches, so NEW-25 does not apply.
- NS-15: a member of an inline namespace is checked against that inline namespace.
- OVL-19 and OVL-20: constraints remove functions of an overload set before the target type is matched, and also when there is no target type.
- OVL-30: an ellipsis makes two parameter lists differ.
- OVL-38: a call that uses a default argument that two declarations in different scopes give differently is an error (G140).
- PACK-11: an expansion in the argument list of an alias template or a concept that reaches a parameter that is not a pack is an error.
- RFOR-7: a data member `begin` or `end` of a type that cannot be called is an error (G141).
- SB-15 and CPT-37: no constrained `auto` on a structured binding.
- STO-11, UNI-9: an anonymous union in an unnamed namespace needs no `static`.
- TSP-11: a partial specialization that only reorders the parameters is an error.
- `show` writes `noexcept` and `...` in a function type, so `operator Plain()` and `operator Safe()` are two conversion functions (OVL-26).
- SB-5: the tuple protocol passes the index as a `std::size_t` constant.
- COND-11: the operands promote before the built-in candidate is chosen.
- OVL-14, DED-1: copy-initialization from another class can use a specialization of a constructor template.
- CPY-4: the implicit copy assignment is deleted when the class declares a move assignment. An rvalue then selects the declared move assignment.

### The interpreter

- CLS-30: during constant evaluation, the address of a standard-layout object equals the address of its first member and of its base.
- CE-4: inside `std::allocator<T>`, the storage converts only to `T*`.
- CE-24: a `constinit` class object without an initializer runs its default constructor, and the zero does not count.
- CONV-11: a floating conversion above the range gives infinity, at run time and in constant evaluation.
- DTOR-7: a second destruction stops the run, also in constant evaluation and for a destructor of any kind. A class parameter ends through the caller's cleanup list, so a callee's explicit destructor call shows.
- NEW-37, LSUP-6: constant evaluation runs the global nothrow forms as library functions.
- OBJ-1, OBJ-2: a write to a scalar variable inside its own initializer stops the run.
- OBJ-2, CTOR-12: a call of a member function of a member whose lifetime has not begun, from a base's mem-initializer, stops the run.
- REF-5: parenthesized aggregate initialization does not extend a temporary.
- UNI-5, UNI-13: an assignment activates a union member only along the member accesses that the assignment names. A user assignment operator called on an inactive member no longer activates it.
- STG-4, STG-6: the overlap test for objects in byte arrays uses offsets in the complete object. Two rows of an array of byte arrays, or the byte arrays of two elements, now hold separate objects.
- STG-29: a cast of a pointer into a byte array that is a complete object gives a pointer to a byte (G139).
- UNW-3, NEW-22: the global sized deallocation function frees the storage of a single object, and of an array whose element class has a destructor that is not trivial.
- UNW-7: a catch stack records the handlers of each try block and the boundary of each non-throwing function. These are Clang's choices in the UNW-7 table:
  - With no matching handler, no object is destroyed.
  - At a non-throwing boundary, the objects of the called functions are destroyed and the function's own objects are not.
- VOL-6: an access to a volatile object through a glvalue that is not volatile stops the run.
  - The check covers a volatile complete object and a volatile member.
  - It does not run during construction or destruction (G138), and it does not cover the bytes of a volatile member.
  - An implicit member access in a `volatile` member function is now volatile.
- OBJ-12: after `main` returns, a pass through the declaration of a block-scope `static` that has ended stops the run.
- A block-scope `extern` declaration names the variable of the enclosing namespace.
- The address of an explicit object member function is an ordinary pointer to function. A call through it passes the object among the arguments.
- `main` with `(int)` and with `(int, char**, char**)` runs. The environment list is empty.
- The C library list: `_Exit`, `exit`, `abort`, `malloc`, `free`, `puts`, and `putchar`, declared by the program with C language linkage at global scope. `exit` ends the static objects, and `_Exit` ends none. `frontend/README.md` states the list as a front-end choice under decision 0035.

## 3. Items left, with reasons

- Ten of the 84 EDG-AGREES tests remain:
  - `trivially-copyable-member-template-selected.cpp` (CPY-27, UNI-11) needs overload resolution of a constructor template inside the implicit copy constructor.
  - `list-single-specialization-alias.cpp` (CTAD-18) needs the single-element rule through an alias template.
  - `friend-distinct-from-namespace.cpp` (CNS-32) needs the front end to keep a friend template apart from the namespace template of the same name.
  - `template-template-more-constrained.cpp` (CNS-25): the front end does not record the constraints of a template template parameter's head.
  - `match-noexcept.cpp` (DED-11): the front end keeps `noexcept` as a flag and cannot deduce `noexcept(B)`.
  - `function-point-constexpr-later.cpp` (INST-32, INST-31) is the G134 case. The front end takes the first point of instantiation.
  - `member-partial-specializations-collide.cpp` (INST-6) needs a comparison of member partial specializations after substitution.
  - `constraints-conversion-results-differ.cpp`, `member-object-rvalue.cpp`, and `reversed-member-object.cpp` (ORD-6, ORD-7, ORD-13, ORD-15) need partial ordering of conversion function templates, and of the object parameter in reversed candidates.
- Six new tests agree with EDG against the base expectation: `initializer-list-default-member-from-braces.cpp`, `initializer-list-member-from-braces.cpp`, `default-immediate-call-checked-at-call.cpp`, `default-immediate-call-unused-not-checked.cpp`, `c-name-unnamed-namespace-redefined.cpp`, and `class-const-and-volatile-lvalues.cpp`. They arrived after the round started, and the next round can take them.
- `expressions/primary/char-named-escape.cpp` stays a false reject. It needs the Unicode name table.
- 54 run tests are unsupported. The largest groups:
  - `new` forms (8)
  - constructor calls (5)
  - jumps into a nested statement (3)
  - a variable of an enclosing function in a local class (3)
  - a destructor function try block (2)

  `block-function-declaration.cpp` needs a link from a block-scope function declaration to another unit.
- The STG-29 change applies only to a byte array that is a complete object. A member byte array keeps the old behavior: `programs/W15/optional.cpp` passes such a pointer to `std::construct_at`, and with the change the program stops (G139).
- VOL-6 and OBJ-12 have no interpreter-only test. Each of these four tests would gain a `// tidal-interpreter: run-fail` line, and the interpreter meets each one:
  - `declarations/volatile/nonvolatile-access-to-volatile-constant.cpp`
  - `foundations/objects/static-local-after-destruction.cpp`
  - `foundations/objects/static-local-used-by-earlier-static.cpp`
  - `foundations/objects/thread-local-local-after-destruction.cpp`

  The Evidence of VOL-6 and OBJ-12 can then say that only the interpreter stops them.
- `deleted-defaulted-move-not-candidate.cpp` (OVL-42) stays unsupported. The assignment selection gives up on defaulted operators.

## 4. Findings

### G136. CTOR-17 does not say whether an unused default member initializer is an error

- Page and rules: classes/constructors, CTOR-17 (Q-0084); test `reference-default-member-binds-temporary.cpp`.
- Kind: ambiguous.
- What happened: the rule says that a constructor must not bind a reference member to a temporary through a mem-initializer or a default member initializer. A class can have such a default member initializer while each constructor has its own mem-initializer for the member, so no constructor uses it. The page does not say whether that class is valid. The front end rejects at the default member initializer.
- Suggestion: say either that the program is not valid only when a constructor that the program defines or uses takes the default member initializer, or that the default member initializer alone is not valid. Add a test for the case.

### G137. No rule says that a base expectation follows Clang's choice for an implementation-defined rule

- Page and rules: FN-6, TR-3 (Q-0024); UNW-7; CONV-11; tests `main-wrong-parameter-types.cpp`, `main-four-params.cpp`, `noexcept-boundary-unmatched-try.cpp`, `floating-narrowing-infinity.cpp`.
- Kind: missing.
- What happened: FN-6 makes the parameter lists of `main` other than `()` and `(int, char**)` implementation-defined, and its table lists what each compiler accepts. The base expectations follow Clang. `(int)` and `(int, char**, char**)` are accepted, and `(long, char**)` and the four-parameter list are rejected. UNW-7 gives two choices for an unmatched try block in a non-throwing function, and the base expectation is Clang's. To pass, an implementer must read the base lines of the tests and take Clang's column, because the page does not say that the tests follow Clang. TR-3 is a `reject` rule, so the harness counts an answer that follows EDG as a failure for `main-wrong-parameter-types.cpp` (G134).
- Suggestion: state in the specification format, or in each table, that the base expectation of a test for an implementation-defined rule follows Clang on the matrix targets. Or give FN-6 a sentence that names the lists that the tests accept.

### G138. VOL-2 and VOL-6 do not say when a volatile class object stops being volatile

- Page and rules: declarations/volatile, VOL-2, VOL-6; tests `volatile-object-ordinary-ctor.cpp`, `nonvolatile-access-to-volatile-constant.cpp`.
- Kind: missing.
- What happened: VOL-2 says that a volatile class object is volatile after its constructor ends. A destructor runs with a `this` that is not volatile, and it can read the members. The page does not say whether those reads fall under VOL-6. The interpreter exempts both the constructor and the destructor.
- Suggestion: add to VOL-2 that the object is not volatile while its destructor runs, as during construction.

### G139. STG-29 does not say whether a pointer to a byte can be the location of std::construct_at

- Page and rules: foundations/storage, STG-29 (Q-0324); foundations/language-support, LSUP-10; test `placement-then-cast-read.cpp`; program `programs/W15/optional.cpp`.
- Kind: ambiguous.
- What happened: `optional.cpp` calls `std::construct_at(reinterpret_cast<T*>(storage), ...)` on a member byte array. By STG-29 the cast gives a pointer to a byte whose type is `T*`. LSUP-10 describes `construct_at` with a pointer to `T`. The page does not say whether such a pointer to a byte is a valid location. When the interpreter applies STG-29 to member byte arrays, the program stops, so the interpreter applies STG-29 only to byte arrays that are complete objects.
- Suggestion: say in STG-29 that the pointer still gives the address of storage, for example to placement `new` and to `std::construct_at`, and that only an access through it is undefined. Add a run test with a member byte array.

### G140. OVL-38 does not say how two default arguments compare

- Page and rules: overloading/overload-resolution, OVL-38 (Q-0140); tests `default-from-two-scopes.cpp`, `default-from-two-scopes-same.cpp`.
- Kind: ambiguous.
- What happened: the rule makes a call that uses "different default arguments" not valid. The tests use `5` and `7`, and `5` and `5`. The page does not say whether `5` and `2 + 3` count as different: they have the same value and different tokens. The front end compares constant values, and compares the tokens when an argument is not a constant.
- Suggestion: say whether two default arguments are the same when they are the same tokens or when they have the same value. Add a test for the case that the rule decides.

### G141. RFOR-7 names only a data member of type int

- Page and rules: statements/range-for, RFOR-7 (Q-0098); test `data-member-begin-counts.cpp`.
- Kind: missing.
- What happened: the rule says that a data member `begin` or `end` of type `int` makes the loop not valid. A data member can also have a class type with `operator()`, or be a pointer to function, and then the member call is valid. The page does not say how such members count. The front end rejects a data member whose type is not a class and not a pointer to function.
- Suggestion: say that a data member named `begin` or `end` takes the member form, and that the loop is valid only if `range.begin()` is a valid call. Add a test with a callable data member, beside `callable-data-members.cpp`.

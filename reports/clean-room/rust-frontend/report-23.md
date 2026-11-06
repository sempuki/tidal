# Front end, round 23: the two wrong verdicts, the phase 11 and 12 tests, the conditional operator, and the template missed rejects

Date: 2026-11-05. Decisions 0029, 0035, 0047, 0048, 0059 (phase 12, item FE), 0060.
Role: clean-room reader. Inputs: AGENTS.md, CLAUDE.md, decisions 0035, 0059, and 0060, `documentation/exercises.md` section "Front end", `frontend/README.md`, report 22, the front-end ledger, `reports/clean-room/ledger.md`, the files under `specification/`, `tests/`, `frontend/`, and `tools/`, and `reports/clean-room/I4/conditional.py` as a second reading of the conditional operator. No compiler was run, and no crates were added. The changed files are `frontend/README.md`, `reports/clean-room/rust-frontend/conformance.tsv`, `reports/clean-room/rust-frontend/ledger.md`, and 13 files under `frontend/src/`: `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `ics.rs`, `ovl.rs`, `parser.rs`, `special.rs`, `stmt.rs`, `templ.rs`, `interp/expr.rs`, `interp/mod.rs`, and `interp/run.rs`. No file was added under `frontend/`.

The reader returned this report as a message, and the integrator saved it here.

The suite grew during the round. At the start, 92 tests were new against the baseline of report 22 (6681 tests), and 51 of them did not pass. During the round, 13 more tests arrived, among them tests of the RV5 review points. So the suite went from 6773 tests at the start of the round to 6786.

## 1. Conformance

### Method

`python3 -I tools/frontend_check.py` (without `--update`) and `python3 -I tools/check_examples.py --frontend-only` ran after each change, on the live tree. A full pass is `pass`, `EDG-ONLY`, `EDG-ALLOWED`, or `pass-partial`. A passed check is a full pass or a `run-` status. At the end of the round, `tools/frontend_check.py --update` wrote the new baseline to `conformance.tsv`.

### Counts

| Measure | Baseline of report 22 (6681 tests) | Start of round (6773 tests) | After, the same 6681 tests | After, all 6786 tests |
|---|---|---|---|---|
| Full pass | 6459 | 6500 | 6477 | 6582 |
| Check pass | 6499 | 6557 | 6517 | 6622 |

Status by test:

| Status | Start (6773) | After (6786) |
|---|---|---|
| pass | 6209 | 6284 |
| EDG-ONLY (a pass) | 269 | 274 |
| EDG-ALLOWED (a pass) | 10 | 10 |
| pass-partial (a pass) | 12 | 14 |
| EDG-AGREES (a failure) | 6 | 2 |
| FALSE-REJECT | 20 | 4 |
| MISSED-REJECT | 124 | 100 |
| unsupported | 66 | 58 |
| run-unsupported | 52 | 40 |
| RUN-WRONG-RESULT | 2 | 0 |
| RUN-UNEXPECTED-UB | 2 | 0 |
| RUN-OTHER-OUTCOME | 1 | 0 |

Passing rejects whose first error names a cited or linked rule: 2007 of 2616 at the start of the round, 2030 of 2647 after.

### The checks

`python3 -I tools/frontend_check.py`, before `--update`:

```
frontend: 6582/6786 pass, 6622/6786 pass their check, 0 worse, 18 better, 0 moved between passing statuses, 105 new (0 not passing)
frontend: 19/19 interpreter-only expectations met
```

`python3 -I tools/check_examples.py --frontend-only`:

```
frontend: 179 examples (177 pass, 2 unsupported); 0 worse, 0 better, 2 new (0 not passing), 0 gone
```

Both checks exit 0.

### Results by target

**Target 1: the wrong verdicts on valid programs.** Each test of this target passes.

- PRIM-9. `string-literal-pointer-copy-equal.cpp`, `pointer-loop-inside-string-literal.cpp`, and `view-over-string-literal.cpp`. The interpreter kept one object for each set of characters and made the PRIM-9 choice at each comparison of two pointers into that object. So a copy of one pointer could compare unequal to itself, and a loop that stepped a pointer to an end pointer of the same literal ran past the array. Now the first evaluation of a literal makes its object, and each later evaluation of a literal with the same characters makes an object of its own. The choice point is a comparison of pointers into two such objects. Pointers into one object compare by their offsets.
- OBJ-11, SINIT-6. `thread-local-member-starts-unit.cpp`. A `thread_local` static data member with dynamic initialization is now deferred as a `thread_local` variable at namespace scope is, so its first use constructs both variables of the unit.
- REF-1, ICS-11, ICS-46. `pointer-lvalue-binds-more-qualified-pointer.cpp`. An `int*` lvalue binds a `const int* const&` directly. So does a pointer to member for a more qualified pointer to member, and an array of `int*` for an array of `const int* const` (the new test `member-pointer-and-pointer-array-bind-directly.cpp`). The conversion sequence is the identity. ICS-46 prefers the reference whose referenced type has fewer qualifiers below the top level, so `reference-direct-fewer-qualifiers.cpp` still passes.
- TR-17. The run message for a program with no `main` names TR-17.

**Target 2: the phase 11 and phase 12 tests.** All 105 new tests pass.

- The conditional operator, COND-3, COND-5, COND-8, COND-11, and COND-16, is one procedure (`frontend/README.md`, section "The conditional operator"). It replaces the older class steps. The rows of COND-3 before the last run first. COND-16 tries each operand against the other with steps 1, 2, and 3. One conversion is made and the rows run again. Two conversions are an error (COND-5). No conversion leads to the built-in candidates of COND-11. These are tried by overload resolution: every pair of promoted arithmetic types, the pointer, pointer-to-member, and scoped enumeration types that the operands reach, and the composite pointer types. A private or ambiguous base is an error when the conversion is made (ACC-12, CONV-17). A class prvalue result copies or moves each glvalue operand (COND-8), also from a derived class. Every test of `expressions/conditional/` passes except `member-pointer-nullptr.cpp` (section 3). That includes the I4-4 to I4-7 tests and three old tests, as well as `class-prvalue-lvalue-private-copy.cpp` (below).
- ACC-3. The constructor that a copy or a move selects must be accessible. Before this round the front end accepted a copy-initialization from an lvalue whose copy constructor is private (finding G154).
- P1-1, INIT-5. An array of unknown bound of a class with a constructor of its own takes one element for each item, with no brace elision.
- P1-2, ICS-11, ICS-3. A `const` reference to a class binds a temporary made by a converting constructor, when the source class has no conversion functions. A direct initialization with one class argument and a converting constructor chooses that constructor over the implicit copy constructor.
- CPY-21, OVL-14. An assignment from a value that is not a class, or from a closure, goes through a converting constructor or a constructor template.
- DED-1, ILC-2. A braced initializer goes to a constructor template when no constructor takes an initializer list. A constructor template whose parameters need arguments is no candidate for default-initialization, also when it has a pack.
- JMP-10, RET-6. One helper decides when a returned name is an xvalue. It leaves out a `volatile` local object, a reference to an lvalue, an rvalue reference to a `volatile` object, and a structured binding.
- LAM-1, RET-2. A lambda with `-> auto` or `-> decltype(auto)` deduces its result from its body. A placeholder after `->` no longer makes the lambda generic.
- BF-6, CONV-1. A bit-field of an enumeration type promotes as the enumeration does.
- INST-6. A member declared without a function declarator, which gets a function type from the arguments of a class specialization, is an error. This covers the programmer's round 9 program that declares a data member of function type.
- DED-29 (Q-0420). Taking the address of an overload set with a target type instantiates each specialization whose result type comes from its body, also when a plain function matches. An error in that body is an error of the program, also through `static_cast`.
- PACK-13, PACK-14, PACK-17. A pack expansion can be an argument of a mem-initializer, a base initializer named by a template-id (`leaf<I>{v + I}...`), and a parenthesized initializer. A capture `x...` captures each element of the pack, also in a generic lambda.
- NEW-13, NEW-28, NEW-29. A deleted or private aligned allocation form that overload resolution selects is an error, and the expression does not try again without the alignment. A class whose allocation functions accept none of the arguments is an error (OVL-35).
- LSUP-11, CE-9 (Q-0421). In constant evaluation, an assignment to an element of storage from `std::allocator` before `std::construct_at` builds it is not a constant expression.
- ICS-45. An explicit conversion function to a class derived from the target does not serve the copy constructor in a direct initialization.
- OVL-14, CVF-1. A functional cast, a `static_cast`, and a direct initialization of a class from a class with a conversion function to it run.
- The r8 and r9 program tests run.

**Target 3: the template missed rejects.**

| Test | Status |
|---|---|
| `deduction/address-deduced-result-body-instantiated.cpp`, `address-deduced-result-body-static-cast.cpp` (DED-29, Q-0420) | Done (target 2). |
| `deduction/substitution-lambda-body.cpp`, `substitution-lambda-default.cpp` (DED-29) | Left. The front end does not parse the lambda again when it substitutes into the function type (section 3). |
| `declarations/default-repeated-function.cpp` (TPL-16) | Done. A redeclaration of a function template that repeats a default is an error, and a later definition keeps the earlier defaults. |
| `instantiation/class-static-member-function-type.cpp` (INST-6) | Done (target 2). |
| `instantiation/explicit-variable-wrong-type.cpp` (INST-22) | Done. The written type of a variable instantiation must be the specialization's type. |
| `instantiation/explicit-declaration-internal.cpp` (INST-29) | Done for function templates. |
| `instantiation/virtual-object.cpp` (INST-11) | Done. A definition of an object of a class specialization makes its virtual functions at the end of the unit, as the matrix compilers do. `virtual-sizeof.cpp` and `virtual-point-after-class.cpp` still pass. |
| `concepts/virtual-override.cpp` (CPT-15) | Done. A function that overrides a virtual function of a base has no trailing requires clause. |
| `packs/pattern-without-pack.cpp` (PACK-8) | Done for a pattern whose names are all variables that are not packs. |
| `packs/unnamed-pack-in-parentheses.cpp` (PACK-2) | Done. |
| `packs/capture-pack.cpp` | Passes now (target 2). |

The other 40 template missed rejects are in section 3.

**Target 4: the round 8 backlog.** Only the access group moved.

- ACC-3. A function that a target type selects (`target-selects-private.cpp`) and a static member function that a call in a template specialization selects (`template-call-selects-private.cpp`) must be accessible. `enclosing-not-access-nested-function.cpp` passes too.

## 2. Old failures by group

The groups are those of report 22 section 2. "Before" counts the tests of the group that failed at the start of this round.

| Group | Before | After | What changed, or why not |
|---|---|---|---|
| Missed rejects: ACC-3, ACC-15, ACC-18 | 6 | 3 | ACC-3 for a target type and for a static member call. Left: ACC-15 (friends, two tests) and ACC-18. |
| The conditional operator, old tests | 4 | 1 | The new procedure. Left: `member-pointer-nullptr.cpp`. |
| Missed rejects: the 9 instantiation rules | 9 | 5 | INST-22, INST-29, INST-11, INST-6. Left: INST-21, INST-33, two INST-12 local class tests, INST-15. |
| `new` in a constant and in a run | 12 | 10 | Two NEW-28 tests through the allocation selection. |
| The `same` idiom with variable templates, conversion functions, concepts in `static_assert`, the `noexcept` operator, constructor calls in a run, BF-9, OVL-20 with no target, DEFARG-13, LIST-13, LIST-16, MPTR-11, DCMP-12, DCMP-13 | not counted again | unchanged | Not targeted. |
| EDG-AGREES (CPY-27, CNS-32) | 2 | 2 | The reasons of report 21 stand. |
| The four older false rejects | 2 | 2 | `tuple-member-get-type-parameter-uses-free.cpp` and `char-named-escape.cpp`, as in report 22. |
| `nondeduced-only-parameter.cpp` (ORD-3) | 1 | 1 | As in report 22. |

## 3. Items left, with reasons

- `substitution-lambda-body.cpp` and `substitution-lambda-default.cpp` (DED-29). The front end parses the lambda in a function type or a default template argument once, as a pattern. Substitution does not parse the lambda's body again with the arguments bound, so the error in the body never appears. The parse of a lambda body during a substitution now drops the muting of errors (`fn_subst` in `parser.rs`). The change takes effect once substitution parses such a body.
- `out-of-class-without-clause.cpp` and `out-of-class-other-clause.cpp` (CNS-29). For a member of a class template, the check finds no member with the same parameters whose clause tokens it recorded.
- `partial-no-primary-variable.cpp` (TSP-31). The declarator of `size<T*>` with an undeclared `size` does not reach the code for variable partial specializations, where the check now is.
- `partial-after-use.cpp` (TSP-17) is a `no-diagnostic` rule with a Clang override, and it was not targeted.
- `fold-in-outer-pattern.cpp` and `size-operator-in-outer-pattern.cpp` (PACK-8). A fold or `sizeof...` inside an outer pattern consumes its pack, which the lexical check of the pattern does not track.
- `independent-error-unused-call.cpp` (DEP-30) and `constexpr-template-nondependent-call-checked.cpp` (IF-22). The front end does not resolve a call with only non-dependent arguments inside a pattern.
- `member-pointer-nullptr.cpp` (COND-3). The conditional of a pointer to member and `nullptr` is not a class case. The `static_assert` stays unknown, because `decltype` of the expression gives an unknown type.
- `ellipsis-nullptr-becomes-void-pointer.cpp` and `ellipsis-argument-conversions.cpp` include `<cstdarg>`, as in report 22.
- The 13 class template argument deduction and constraint tests (CTAD-2, CTAD-12, CTAD-19, CNS-13, CNS-24, CNS-26) and the 10 dependent-name tests were not targeted.

## 4. Tests added during the round

105 tests are new against the baseline of report 22: 92 present at the start of the round, and 13 that arrived during it. All 105 pass. The 13 that arrived include these tests:

- `declarations/initialization/references/member-pointer-and-pointer-array-bind-directly.cpp`
- `noexcept-function-binds-plain-reference.cpp`
- `classes/members/ref-qualifier-mix-different-constraints.cpp`
- `expressions/new-delete/only-aligned-class-form-ordinary-type.cpp`
- `foundations/storage/class-allocation-function-hides-placement.cpp`
- `foundations/conversions/bit-field-enumeration-promotes-as-enumeration.cpp`
- `overloading/conversion-sequences/explicit-conversion-to-derived-direct-init.cpp`
- `statements/jumps/return-name-limits.cpp`
- `statements/jumps/return-structured-binding-lvalue.cpp`

The ledger rows of `reports/clean-room/ledger.md` that name this round are P1-1, P1-2, and I4-4 to I4-7. The front end passes each test that these rows name. The integrator can move the rows from `planned` to `commit`.

## 5. Findings

### G153. COND-16 and REF-1 disagree on whether a private or ambiguous base binds directly

- Page and rules: expressions/conditional, COND-16; declarations/initialization/references, REF-1; classes/access, ACC-12; foundations/conversions, CONV-17; tests `expressions/conditional/class-private-base-operand.cpp`, `class-ambiguous-base-operand.cpp`.
- Kind: ambiguous.
- What happened: step 1 of COND-16 asks whether a `T&` binds directly to the operand. REF-1 says a reference binds directly to an lvalue of a class derived from the type through an accessible base. On that reading, a `Hidden` lvalue with a private base `Root` does not bind a `Root&` directly. Step 1 then gives no conversion. Step 3's row for a base gives a `Root` prvalue, whose sequence COND-16 forms without access. REF-1 says nothing of an ambiguous base. The test notes say "the conversion counts for COND-16 and then cannot be made", which reads as step 1 counting the base. Both readings reject these two tests. They differ in the category of the converted operand, and in the rule that the error names. The front end takes the base relation without access or ambiguity in steps 1 and 2. It reports ACC-12 or CONV-17 when it makes the conversion.
- Suggestion: say in COND-16 that steps 1 and 2 ask only whether the class of the operand is the class of `T` or derived from it, without access or ambiguity, as step 3 does. Then say that ACC-12 and CONV-17 apply to the conversion that the step finds.

### G154. No test outside the conditional page checks the access of the constructor that a copy selects

- Page and rules: classes/access, ACC-3; expressions/conditional, COND-8; classes/copy-move; test `expressions/conditional/class-prvalue-lvalue-private-copy.cpp`.
- Kind: needs example.
- What happened: before this round the front end accepted `S t = s;` and a by-value argument `g(s)` where the copy constructor of `S` is private. Only `class-prvalue-lvalue-private-copy.cpp`, through COND-8, showed the gap. ACC-3 states the rule for a function that overload resolution selects. A copy-initialization of a class selects its constructor by overload resolution, so ACC-3 covers it, but no example or test shows it.
- Suggestion: add tests where a copy-initialization, a by-value argument, and a return by value select a private copy constructor or a private move constructor. Cite ACC-3 and the copy-move rule that selects the constructor.

### G155. No reject rule names a deleted or inaccessible allocation function

- Page and rules: expressions/new-delete, NEW-29, NEW-25, NEW-13; classes/access, ACC-3; declarations/deleted, DEL-1; tests `expressions/new-delete/aligned-deleted-form-beside-unaligned.cpp`, `aligned-private-form-beside-unaligned.cpp`.
- Kind: missing.
- What happened: NEW-29 says that an allocation function that overload resolution selects accepts the arguments even when it is deleted or not accessible, so the expression is then not valid. NEW-29 has the kind `required`. NEW-25 is the reject rule of the page, and it covers only the deallocation function. The front end reports the error with DEL-1 or ACC-3 as the first rule and NEW-29 as the second.
- Suggestion: give the requirement on the selected allocation function a reject rule. Or extend NEW-25 to say that the allocation function that a `new` expression selects must be accessible and not deleted.

### G156. LSUP-11 names two element forms and leaves the rest open

- Page and rules: foundations/language-support, LSUP-11; foundations/constant-evaluation, CE-9; Q-0421; tests `foundations/storage/allocation-assign-before-construction-int-element.cpp`, `allocation-assign-before-construction-pod-member.cpp`.
- Kind: ambiguous.
- What happened: LSUP-11 says constant evaluation rejects an assignment to an element of storage from `allocate` before `std::construct_at` builds it, "for an element of type `int` or a member of an element of a simple class". The text does not say what holds for an element of another implicit-lifetime type, such as `double` or a pointer. It also does not cover an assignment of a whole element of a simple class. The front end rejects an assignment before construction for every element type in constant evaluation.
- Suggestion: state the rule for every element type. Or list the forms that the probe decided and give the others a question.

### G157. INST-11 does not say when the matrix compilers make a virtual function

- Page and rules: templates/instantiation, INST-11, INST-31, INST-32; tests `templates/instantiation/virtual-object.cpp`, `virtual-sizeof.cpp`, `virtual-point-after-class.cpp`.
- Kind: ambiguous.
- What happened: INST-11 is `unspecified`: a virtual member function that no use names is or is not instantiated. The three tests expect different answers:
  - `virtual-object.cpp` expects a reject when a program defines an object of the specialization.
  - `virtual-sizeof.cpp` expects an accept when only `sizeof` names the class.
  - `virtual-point-after-class.cpp` expects an accept where the body needs a declaration that comes after the object.

  The page does not say which event makes the function on the matrix compilers. The front end makes the virtual functions that are not pure when the program defines an object of the class, at the end of the unit (`frontend/README.md`, the table of matrix choices).
- Suggestion: say in the Evidence of INST-11 which uses make the matrix compilers instantiate the virtual functions: the definition of an object and a constructor call do, and `sizeof` does not. Add a test with a `new` expression and one with a temporary.

## 6. Distance from the completion test

Decision 0060 gives two conditions. The second holds. Every program test under `tests/programs/` runs to its expected result: 111 program tests, 97 with a full exploration and 14 partial. No front-end ledger row marks a test `out of scope`, so the first condition counts every test. 204 of 6786 tests fail their check or their run:

| Status | Tests |
|---|---|
| MISSED-REJECT | 100 |
| unsupported | 58 |
| run-unsupported | 40 |
| FALSE-REJECT | 4 |
| EDG-AGREES | 2 |

By page:

| Page | Failing tests |
|---|---|
| classes/access | 3 |
| classes/comparisons | 6 |
| classes/constructors | 3 |
| classes/conversion-functions | 7 |
| classes/copy-move | 3 |
| classes/derived | 1 |
| classes/initializer-lists | 2 |
| classes/members | 1 |
| classes/unions | 6 |
| classes/virtual | 3 |
| declarations/alignment | 1 |
| declarations/auto | 2 |
| declarations/constexpr | 3 |
| declarations/declarators | 2 |
| declarations/exception-specifications | 3 |
| declarations/functions | 5 |
| declarations/initialization | 9 |
| declarations/inline | 1 |
| declarations/linkage-specifications | 2 |
| declarations/storage-linkage | 1 |
| declarations/structured-bindings | 2 |
| exceptions/handlers | 4 |
| exceptions/unwinding | 1 |
| expressions/additive | 2 |
| expressions/conditional | 1 |
| expressions/decltype | 3 |
| expressions/lambdas | 3 |
| expressions/logical | 1 |
| expressions/member-pointers | 4 |
| expressions/new-delete | 10 |
| expressions/postfix | 3 |
| expressions/primary | 4 |
| expressions/unary | 1 |
| foundations/constant-evaluation | 2 |
| foundations/conversions | 1 |
| foundations/language-support | 4 |
| foundations/names | 3 |
| foundations/odr | 2 |
| foundations/storage | 2 |
| overloading/conversion-sequences | 12 |
| overloading/overload-resolution | 6 |
| statements/if | 3 |
| statements/jumps | 1 |
| statements/range-for | 2 |
| statements/switch | 2 |
| templates/arguments | 6 |
| templates/class-template-deduction | 8 |
| templates/concepts | 5 |
| templates/constraints | 7 |
| templates/declarations | 3 |
| templates/deduction | 5 |
| templates/dependent-names | 10 |
| templates/instantiation | 7 |
| templates/ordering | 1 |
| templates/packs | 5 |
| templates/specialization | 4 |

Six of these tests are outside what the front end reads: the two `<cstdarg>` tests, `char-named-escape.cpp` (a named escape), and the three tests that need an ILP32 bound or a size above the size type. Their rows could take `out of scope` with a reason.

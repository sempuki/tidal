# Front end, round 17

Date: 2026-11-03. Decisions 0029, 0035, 0054 (phase 7, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`, and the earlier reports in `reports/clean-room/rust-frontend/`. No compiler was run, and no crates were added. Only `frontend/src/` changed, and no file was added. The other writer added and changed tests during the round, and the suite grew from 6286 to 6296 tests. New tests are judged only as new tests. Three baseline tests changed their expectation during the round: `declarations/initialization/references/braced-function-noexcept.cpp`, `templates/constraints/redeclare-placeholder.cpp`, and `templates/dependent-names/dependent-base-conversion-function.cpp`. The front end now passes all three.

## 1. Conformance

### Checks

`python3 -I tools/frontend_check.py` (without `--update`), against the baseline `conformance.tsv` (6286 tests):

```
frontend: 6040/6296 pass, 6105/6296 pass their check, 0 worse, 125 better, 10 new (0 not passing)
frontend: 14/14 interpreter-only expectations met
```

The check exits 0.

The check does not catch a change from one failing status to another. So the round-16 front end was kept as a build in a scratch directory, and both front ends ran `conform -v` on the whole suite after each change. Every test whose status changed went to `pass` under the new front end: 98 missed rejects, 10 false rejects, 9 wrong results, 2 missed undefined behaviors, and 11 run-unsupported tests.

Status by test, for the 6286 baseline tests:

| Status | Baseline | After |
|---|---|---|
| pass | 5768 | 5893 |
| differs from EDG only | 137 | 137 |
| FALSE-REJECT | 8 | 1 |
| RUN-WRONG-RESULT | 9 | 0 |
| RUN-MISSED-UB | 2 | 0 |
| MISSED-REJECT | 213 | 117 |
| unsupported | 73 | 73 |
| run-unsupported | 76 | 65 |

Changes by test:

| Change | Tests |
|---|---|
| missed reject, now passes | 94 |
| missed reject, now differs from EDG only | 2 |
| false reject, now passes | 7 |
| wrong run result, now passes | 9 |
| missed undefined behavior, now passes | 2 |
| run-unsupported, now passes | 11 |
| differs from EDG only, now passes | 2 |

The 10 new tests all pass, one of them as "differs from EDG only".

### Program tests

Each program under `tests/programs/` was run with `tidal-fe run` at the end of the round. A W14 program is the files of its directory. "Result 0 (N runs)" means that the run has `unspecified` choice points and that every explored order gave 0.

| Program | Result |
|---|---|
| W0 accounts, owner, shapes | result 0 |
| W0 calc | result 0 (64 runs, the bound) |
| W0 ring | result 0 (2 runs) |
| W1 bitset, checked, fixed, parse | result 0 |
| W1 checksum | result 0 (2 runs) |
| W2 clamp_sort, three_way, validators | result 0 |
| W2 order | result 0 (16 runs) |
| W3 search, tokenize, vending | result 0 |
| W3 nested | result 0 (64 runs, the bound) |
| W4 arena, list, vector | result 0 |
| W4 grid | result 0 (64 runs, the bound) |
| W5 commands, deleted, two_values | result 0 |
| W5 recursion | result 0 (2 runs, about 8 seconds) |
| W6 bindings, config_table, default_members, ref_members | result 0 |
| W7 date, money, rational, small_string | result 0 |
| W8 shared, tagged, unique, vector | result 0 |
| W9 memptr, mixin, visitor | result 0 |
| W9 plugin | result 0 (4 runs) |
| W10 cleanup_guard, error_codes, noexcept_functions, transaction | result 0 |
| W11 consteval_validator, enum_names, lookup_table, math_tests | result 0 |
| W12 dispatcher, sort | result 0 |
| W12 compose | result 0 (8 runs) |
| W12 counter | result 0 (4 runs) |
| W13 aligned, clink, header, registers | result 0 |
| W14 p1_three_units, p2_internal_helpers, p3_shared_constant | result 0 |
| W14 p4_init_order | result 0, with the SINIT-3 order of the units as given |
| W15 largest, optional, typelist | result 0 |
| W15 shapes | result 0 (2 runs) |
| W15 stack | result 0 (4 runs) |

Every program runs to result 0, as in round 16.

## 2. What changed in the front end

### Declarations

- DEFARG-5: a function type in a type-id or a `typedef` must not have a default argument.
- DEFARG-10: in a template, a default argument made only of literals is converted at the declaration.
- DEFARG-11: a friend declaration with a default argument must be a definition and the only declaration.
- DEFARG-12: a default argument outside the class must not make a default, copy, or move constructor.
- NS-15: a qualified definition of a member of an inline namespace goes to that namespace. A qualified definition must be in a namespace that encloses the named one, and not in that namespace.
- NS-14: an alias must not name another namespace than an earlier alias of the same name, and a namespace definition must not use an alias.
- NS-12: a name declared in a namespace and in its inline namespace makes a qualified use ambiguous. NAM-13: qualified lookup follows the using directives of inline namespaces.
- LSP-1, LSP-2, LSP-4, LSP-7, TR-3: the innermost linkage specification gives the language. The C names are kept by spelling. C names with one spelling must agree in kind and type. A C name must not be `main` or the name of a global variable with C++ linkage. A C declaration must not follow a C++ declaration. The form without braces must not declare a `static` entity. The interpreter links a C name by its spelling, also from an unnamed namespace in another unit.
- LNK-10: `static` after a declaration with external linkage. STO-12: the declarations agree on `thread_local`. ODR-1, ODR-15: `extern` with an initializer is a definition.
- INL-2: `inline` on a data member, on a function declared in a block, on a declaration that only defines a class, and on a declaration after the definition.
- AUTO-8 (an array of `auto`), AUTO-21 (elements of different types in `= {...}`), AUTO-22 (declarators that deduce different types for the placeholder).
- ALN-2 (`alignas` on a class weaker than its natural alignment) and ALN-3 for classes.
- TYN-2 (a variable with the name of a typedef of a class), TYN-7 (a class key and a qualified name alone), TYN-9 (a class named for linkage by a typedef with a base, a member function, or a default member initializer), TR-12 (a scoped enumeration without a name).
- TSP-30: an explicit specialization of a scoped member enumeration after a use of the class specialization replaces its enumerators. Unscoped enumerators of a definition with a qualified name go to the class.
- UNI-2, DCL-18 (a written reference to a reference), EXS-3 (an override of a non-throwing function), EXS-7 (`throw` with types), CTOR-14 (delegation cycles).
- CNS-27, CNS-40: a function with a placeholder parameter is a template apart from a template whose head writes the parameter, so a call of both is ambiguous.
- main with a trailing return type `-> int` is accepted.

### Expressions

- CMP-14: a chain through an operator function that returns `bool` warns. The parts of a fold are in parentheses. A warning in a tentative parse or a substitution is dropped and does not make the substitution fail.
- LIST-2 in aggregate initialization of simple aggregates. LIST-14 and ILC-7 for designators.
- BF-4, ASN-10, COND-9: an assignment to a bit-field and a conditional lvalue with a bit-field operand are bit-fields for `&` and `sizeof`.
- PRIM-29 in a template definition: the operator next to `...` is a fold operator, and the operands name a pack.
- VIRT-8 for a temporary of an abstract class. CVF-9 for a const object and a conversion function that is not const. VOL-11. NEW-17 for a scoped enumeration size and a constant size too large for `size_t`. REF-11 for a braced function of another type.
- NAM-16: `p->T::~T()` with `T` a scalar alias. NAM-27: a member template as a template argument adds its class. DEP-12: `operator int()` in a template with a dependent base.
- ODR-3: a function address as a template argument is a use (ODR-9, ODR-11). LAM rules: a lambda reads a `constexpr` variable of the enclosing function without a capture.

### Statements and warnings

- FN-7: control can reach the end of a function that returns a value. The analysis says no when it is not sure.
- SW-7 and SW-8 under `-Wall` or `-Wswitch`. CMP-16 for a comparison of two arrays. COMMA-5 for every built-in comma.
- RFOR-2, RFOR-7: the member form needs both a member `begin` and a member `end`, and otherwise argument-dependent lookup must find both.
- THR-3 for a pointer to an incomplete type. TRY-5 for a pointer to an array of unknown bound and a parameter whose copy constructor is deleted or not accessible.

### Interpreter

- CTOR-12: a call of a member function on an object whose bases are being initialized stops the run. TRY-11: a call of a member function on an object, or a subobject of it, in the handler of its constructor's function try block stops the run.
- TRY-2: a reference to an array or a function never matches. A pointer to a noexcept member function matches a handler without noexcept.
- THR-1: a local rvalue reference to a class object is moved from.
- UNW-2: an aggregate initialization and a lambda capture that throw end the parts built before, with the temporaries that reference members extend.
- UNW-3: a new expression for a type with extended alignment releases its storage through the aligned global form.
- UNW-1: a class result that a return statement built ends when a local destructor throws.
- A function declared in a block calls the function of the enclosing namespace. A lambda or local class reads a constant of the enclosing call's frame.
- CE-17: a variable of a type that is not a literal type no longer stops constant evaluation. CE-2: a destructor that is not constexpr stops it, also with an empty body.

## 3. Items fixed

Priority 1, tests from the phase 7 repairs and the named rules:

- DEFARG-5, DEFARG-10, DEFARG-11, DEFARG-12: `alias-function-type-default.cpp`, `typedef-function-type-default.cpp`, `template-default-independent-unused.cpp`, `friend-default-later-declaration.cpp`, `friend-default-not-definition.cpp`, the three `outside-default-makes-*-constructor.cpp`, and `block-declaration-repeats-outer-default.cpp` (run)
- ALN: `weaker-on-class.cpp`, `class-redeclare-different.cpp`, `class-definition-without-earlier-with.cpp`
- NS-15: `qualified-definition-inline-namespace-member.cpp` (false reject), `qualified-definition-inside-own-namespace.cpp`, `definition-in-unrelated-namespace.cpp`, `storage-linkage/qualified-static-after-extern.cpp`
- CMP-14: `chained-class-bool-warns.cpp`, `chained-fold-not-chain.cpp`, `chained-in-concept-no-warning.cpp` (both false rejects)
- THR-1, TRY-2, TRY-5, UNW-1, UNW-2, UNW-3: `local-rvalue-reference-moved.cpp`, the three `match-*` handler tests, `param-deleted-copy.cpp`, `param-private-copy.cpp`, `param-pointer-unknown-bound-array.cpp`, `aggregate-init-throws-members-destroyed.cpp`, `lambda-capture-throws-captures-destroyed.cpp`, `reference-member-temporary-ended.cpp`, `return-local-destructor-throws-result-destroyed.cpp`, `new-extended-alignment-aligned-delete.cpp`
- NAM-13, NAM-16, NAM-27: `qualified-directive-in-inline-namespace.cpp`, `qualifier-fundamental-alias-destructor.cpp`, `adl-member-template-template-argument.cpp` (all false rejects)
- ODR-3: the five lambda and local-class run tests, `template-argument-address-is-use.cpp`
- TR-3: `main-trailing-return-int.cpp` (false reject), `main-c-linkage.cpp`, `main-c-linkage-in-namespace.cpp`, `main-c-linkage-variable-in-namespace.cpp`
- UW interpreter tests: `base-init-calls-member-early-sanitizer.cpp` (CTOR-12), `ctor-try-block-member-call-sanitizer.cpp` (TRY-11)
- C linkage: `c-linkage-main.cpp`, `c-name-conflict-types.cpp`, `c-name-conflicts-global.cpp`, `c-overload-rejected.cpp`, `c-variable-and-function-conflict.cpp`, `c-variable-named-main.cpp`, `redeclare-linkage-mismatch.cpp`, `single-form-static.cpp`, `c-name-unnamed-namespace-other-unit.cpp` (run), `definitions/c-linkage-main-in-namespace.cpp`

Priority 2: every false reject and wrong result of the baseline except `char-named-escape.cpp`. Also `hidden-friend-block-declaration.cpp`, `local-class-constexpr-double.cpp`, `block-scope-declaration.cpp`, and `member-begin-only-uses-free.cpp` now run.

Priority 3, missed rejects: inline (4), storage and linkage (4), namespaces (3), auto (4), type names (4), unions and bit-fields (8), designators (3), narrowing in aggregates, switch warnings (5), FN-7 (2), CMP-16 (2), COMMA-5, PRIM-29 (2), NEW-17 (2), range-for (2), exception specifications (3), THR-3 (2), VOL-11 (2), CTOR-14 (2), VIRT-8, CVF-9, DCL-18, TR-12, ODR-1.

Changed tests: `braced-function-noexcept.cpp` (REF-11), `redeclare-placeholder.cpp` (CNS-40) and the new `redeclare-placeholder-unconstrained.cpp`, `dependent-base-conversion-function.cpp` and the new `-independent.cpp` form (DEP-12), and the new `explicit-enum-scoped-defined-after-use.cpp`, `explicit-enum-unscoped-declared-after-use.cpp`, and `nonliteral-variable-reached.cpp`.

## 4. Items left, with reasons

- `expressions/primary/char-named-escape.cpp`: it needs the Unicode name table. Not attempted.
- `exceptions/unwinding/array-element-throws-built-destroyed.cpp`: the interpreter does not yet run the constructor call in it.
- 117 missed rejects remain. Many need analyses the front end lacks: access checks after overload resolution (ACC-3, ACC-15, ACC-18, ACC-19), defaulted comparison rules (DCMP-11 to DCMP-13), consteval escalation (CEV-1, CEV-2), the warnings REF-15, UN-19, UNEV-4, VIRT-13, and TRY-13, overload sets with no target (OVL-20), and a group of template checks (CTAD, CNS, TSP, PACK, INST).
- FN-7 says no when the analysis is not sure: after a label, in a discarded `if constexpr` branch, and for a `goto`. A program with such a path and `-Werror` can miss the warning.
- `classes/conversion-functions/const-object-explicit-cast.cpp`: CVF-9 is checked for implicit conversions only, not for `static_cast`.
- The TRY-12 helper `can_complete` treats a `return` as a statement that completes. No test shows a wrong result from it. FN-7 uses a new analysis.

## 5. Findings

### G128. FN-7 does not say what "control can reach the end" means

- Page and rules: declarations/functions/definitions, FN-7; tests `flow-off-end-warns.cpp`, `switch-every-enumerator-no-return.cpp`.
- Kind: missing.
- What happened: the test for a `switch` over every enumerator of a scoped enumeration expects a warning, so the analysis does not use the values an enumeration can hold. The page does not say which conditions count as constant (`while (true)`), whether a call of a `[[noreturn]]` function ends the path, or how labels and `if constexpr` count. The front end treats a constant condition, `throw`, `return`, and a `[[noreturn]]` call as ending the path, and a `switch` without `default` as able to complete.
- Suggestion: list the cases that end a path and say that a `switch` without `default` can complete whatever cases it has.

### G129. CTOR-12 does not name the object of the early member call

- Page and rules: classes/constructors, CTOR-12; tests `base-init-calls-member-early-sanitizer.cpp`, `base-init-calls-member-early-constant.cpp`.
- Kind: ambiguous.
- What happened: the rule says a call of a member function from the `mem_init` of a base, before all bases are initialized, is undefined. The page does not say on which object. A base's own constructor can call the base's member functions, and a call can go to a member of the derived object that is not built yet. The interpreter stops only a call whose object is the object whose bases are being initialized.
- Suggestion: say that the call is on the object under construction, or through a pointer or reference to it, and say whether a call on one of its members is covered here or by OBJ-1.

### G130. UNW-1 leaves out a result built in a try block of the same function

- Page and rules: exceptions/unwinding, UNW-1 (Q-0365); test `return-local-destructor-throws-result-destroyed.cpp`.
- Kind: missing.
- What happened: the test needs the built result to end when a local destructor throws after the return statement. The page does not say what happens when a handler in the same function catches the exception and runs a second return statement. The interpreter ends the result only when the exception leaves the function.
- Suggestion: say when the result ends if the exception is caught inside the function, and add a test.

### G131. CE-17 does not point to CE-2 for the destructor of a non-literal variable

- Page and rules: foundations/constant-evaluation, CE-17, CE-2 (Q-0335); tests `nonliteral-variable-reached.cpp`, `implicit-destructor-member-not-constexpr.cpp`.
- Kind: missing.
- What happened: CE-17 now says a variable of a type that is not a literal type is not forbidden. The second test still needs a reject, because the implicit destructor of the variable's class calls a destructor that is not constexpr. An implementer who drops the CE-17 check loses that reject unless the destructor at the end of the variable's life counts as a call under CE-2, also with an empty body.
- Suggestion: add a sentence to CE-17 saying that the end of such a variable calls its destructor, and that CE-2 applies to that call.

### G132. LSP-7 does not give the order of a C name and a global variable

- Page and rules: declarations/linkage-specifications, LSP-7 (Q-0075); test `c-name-conflicts-global.cpp`.
- Kind: ambiguous.
- What happened: the rule says a C name must not have the spelling of a global variable with C++ linkage. The test declares the global variable first. The page does not say whether a global variable declared after the C name is also not valid, or whether a global function with C++ linkage counts. The front end checks only a C variable declared after a global variable.
- Suggestion: state both orders and say whether functions count, with a test for the other order.

### G133. NS-15 does not say which namespace must enclose a definition of an inline namespace member

- Page and rules: declarations/namespaces, NS-15, NS-3; tests `qualified-definition-inline-namespace-member.cpp`, `qualified-definition-inside-own-namespace.cpp`.
- Kind: ambiguous.
- What happened: `int geo::area()` defines `geo::v2::area`. NS-15 says the definition must be in a namespace that encloses `N`, and not in `N`. The page does not say whether `N` is the named namespace `geo` or the namespace `geo::v2` that declares the member. The two readings differ for a definition `int geo::area()` written inside `geo` itself. The front end applies the check to the named namespace.
- Suggestion: name the namespace that the check uses, and add a test for `geo::area` defined inside `geo`.

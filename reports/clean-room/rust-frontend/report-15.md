# Front end, round 15

Date: 2026-11-03. Decisions 0029, 0035, 0054 (phase 7, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`, the previous reports in `reports/clean-room/rust-frontend/`, and `reports/clean-room/roadmap-4/`. No compiler was run, and no crates were added. Only `frontend/src/` changed, and no file was added. The other agent added and changed tests during the round, and the suite grew from 6099 to 6110 tests. The new tests are judged only as new tests.

## 1. Conformance

### Checks

`python3 -I tools/frontend_check.py` (without `--update`), against the baseline `conformance.tsv` (6099 tests):

```
frontend: 5733/6110 pass, 5813/6110 pass their check, 0 worse, 81 better, 11 new (5 not passing)
frontend: 4/4 interpreter-only expectations met
```

The check exits 0. At the start of the round it reported `5646/6099 pass, 5766/6099 pass their check, 0 worse, 0 better, 0 new (0 not passing)`.

The check does not catch a change from one failing status to another. So the round-14 front end was built from `HEAD` in a scratch directory and run on the same tests after each change. Every test has the same status or a better one under the new front end. The 5 new tests that do not pass give the same result on both front ends. `foundations/storage/base-bytes-shared-with-member.cpp` is `run-unsupported` in the baseline and `RUN-WRONG-RESULT` now. The test changed after the baseline was written, and the round-14 front end gives the same wrong result. The test needs the class layout that this round leaves out.

Status by test, for the 6099 baseline tests:

| Status | Baseline | After |
|---|---|---|
| pass | 5521 | 5601 |
| differs from EDG only | 125 | 126 |
| FALSE-REJECT | 8 | 1 |
| RUN-MISSED-UB | 1 | 0 |
| RUN-WRONG-RESULT | 0 | 1 |
| MISSED-REJECT | 212 | 198 |
| unsupported | 113 | 94 |
| run-unsupported | 119 | 78 |

Changes by test:

| Change | Tests |
|---|---|
| run-unsupported, now passes | 40 |
| unsupported, now passes | 18 |
| unsupported, now differs from EDG only | 1 |
| missed reject, now passes | 14 |
| false reject, now passes | 7 |
| missed undefined behavior, now passes | 1 |

### Program tests

Each program under `tests/programs/` was run with `tidal-fe run` at the end of the round (roadmap-4 implementer process change 6). A W14 program is the files of its directory. "Result 0 (N runs)" means that the run has `unspecified` choice points and that every explored order gave 0.

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
| W5 recursion | unsupported at line 16: step limit of the interpreter exceeded |
| W6 bindings, config_table, default_members, ref_members | result 0 |
| W7 date, money, rational, small_string | result 0 |
| W8 shared, tagged, unique, vector | result 0 |
| W9 memptr, visitor | result 0 |
| W9 mixin | unsupported at line 97: virtual base class |
| W9 plugin | result 0 (4 runs) |
| W10 cleanup_guard, error_codes, noexcept_functions, transaction | result 0 |
| W11 consteval_validator, enum_names, lookup_table, math_tests | result 0 |
| W12 dispatcher, sort | result 0 |
| W12 compose | result 0 (8 runs) |
| W12 counter | result 0 (4 runs) |
| W13 aligned, clink, registers | result 0 |
| W13 header | unsupported at line 31: sizeof of a class with a layout the front end does not compute |
| W14 p1_three_units, p2_internal_helpers, p3_shared_constant | result 0 |
| W14 p4_init_order | result 0, with the SINIT-3 order of the units as given |
| W15 largest, optional, typelist | result 0 |
| W15 shapes | result 0 (2 runs) |
| W15 stack | result 0 (4 runs) |

Of the 16 programs that the task named, 14 now pass. W5 recursion and W13 header are left (section 4). W9 mixin passes its check and stops at a virtual base when it runs.

## 2. What changed in the front end

### Instantiation

- INST-7: the body of a function template specialization is a context of its own. A specialization that an unevaluated operand made, such as `decltype(largest(1, 2))`, now marks the functions that its body calls as used. Before, `largest(3, 9, 4)` stopped at CE-16 because the inner specialization had no body.
- INST-5: the body of a constructor of a class template instance is still parsed with its class. Its diagnostics and the uses that it makes now wait for a use of the constructor. `Optional<Buffer>` no longer reports the deleted copy constructor of `Buffer`, which only the unused copy constructor of `Optional<Buffer>` needs.
- Deduction takes the type of a `reinterpret_cast` operand, which is known.

### Lambdas

- LAM-15: a generic lambda gets a closure type. Its placeholder parameters become invented template parameters, as for an abbreviated function template (DED-34). A call deduces their types and parses the lambda again with those types, which gives a call operator for the call. `-> decltype(auto)` keeps a reference result (RET-2). The closure object of the generic lambda runs the new call operator. The conversion to a pointer to function stays unknown, and a constrained placeholder parameter (CPT-20) leaves the lambda unknown as before.
- DED-29: when the parameters or the result type of a call operator made for a call are not valid, the call has no viable function. This is a failure of substitution where one is in progress.

### Classes and expressions

- UNI-7: an anonymous union in a class is an unnamed member of the class, and the names of its members are in the class scope. A use goes through the unnamed member. A mem-initializer that names a variant member initializes the union with that member active (UNI-5). The class is no longer opaque.
- OPR-14: `x->m` with an overloaded `operator->` calls the operator until the result is a pointer. OPR-3: `x[]`, `x[a, b]`, a static `operator[]`, and one with a default argument call the member `operator[]`.
- OVL-14: the copy and move assignment of a class whose assignment operators all take the class itself are chosen also when the right operand has a conversion function. `p = {r, c}` uses the implicit assignment with copy-list-initialization.
- A list-initialization with one element that is not a list can use a constructor with a standard sequence when the implicit copy or move constructor needs a user-defined one.
- COND-3, COND-11: a derived lvalue and a base lvalue give a base lvalue. A class that converts to the type of the other operand converts to it. Two unrelated classes with one conversion function each to a type that is not a class convert by those functions.
- NEW-15: the operand of `delete` converts by its one conversion function to a pointer to an object type.
- DCL-6: `T a[] = {x, y}` with elements of class `T` has two elements.

### Structured bindings

- SB-5, SB-14, SB-17: the tuple protocol. When `std::tuple_size<E>` is complete and has `value`, the front end makes the hidden variable and, for each element, a hidden reference that `e.get<i>()` or `get<i>(e)` initializes. `decltype` of a name is `std::tuple_element<i, E>::type`. A count mismatch (SB-2), a missing element type, and a `get` that argument-dependent lookup does not find are errors. A range-for statement over structured bindings declares the hidden references too.

### Names

- NAM-39: a class or enumeration can share its name with a data member, a variable, a function, or an enumerator in one scope. An enumerator with the name of its own class is still an error.
- NAM-40: ordinary lookup finds the variable, function, data member, or enumerator. A class key, a name before `::`, and a base-specifier find the class.
- NAM-41, DEP-22: a qualified type name that finds a member which hides the class is not a type.
- DEP-15 (G116): inside a partial specialization, an argument that names a type alias for the corresponding argument of the specialization is equivalent. `Box<pointer>` with `using pointer = T*` names the current instantiation of `Box<T*>`.

### Storage and allocation

- STG-18: an implicitly created object that the program read or wrote can be replaced by an object of an unrelated scalar type. An access to the replacement is undefined. Same-size integer types and byte types do not count as unrelated, and a class that holds the old object as a member is a valid set (STG-8).
- NEW-19, NEW-26, NEW-40, NEW-41: the sized and aligned forms of the global allocation and deallocation functions can be called by name. A deallocation checks its partner (array form, alignment value, size) and a second deallocation. A null pointer does nothing. A declared nothrow form without a definition acts as the library's function.
- NEW-28, NEW-29: a `new` expression for an over-aligned type passes the alignment to a declared function that takes it, and drops it when none does. A `delete` expression for such a type calls an aligned usual deallocation function when one is declared. Storage for an over-aligned type has its alignment.
- ALN-1: `alignas` on a variable, also at block scope, gives the variable's address that alignment. A statement's attributes no longer drop the `alignas` of a block-scope declaration.
- LSP-4: a function with C linkage declared in a namespace runs the definition of that name.

## 3. Items fixed

Priority 1, the program tests:

- False rejects: `W15/largest.cpp` (INST-7), `W15/optional.cpp` (INST-5).
- Unsupported: `W0/calc.cpp` (DCL-6, UNI-7), `W3/nested.cpp` (class assignment from a braced list), `W5/two_values.cpp` and `W6/bindings.cpp` (SB-5), `W12/sort.cpp` and `W12/compose.cpp` (LAM-15). `compose.cpp` differs from EDG only (Q-0354).
- Run-unsupported: `W4/vector.cpp` (NEW-26 by name), `W5/deleted.cpp` (list-initialization), `W8/tagged.cpp` (UNI-7), `W8/unique.cpp` (OPR-14, OVL-14), `W13/aligned.cpp` (NEW-28, ALN-1), `W13/clink.cpp` (LSP-4).

Priority 2:

- `foundations/names/shared-name-class-members.cpp`, `shared-name-enumerator-class.cpp`, `shared-name-variable-found.cpp` (NAM-39, NAM-40)
- `templates/dependent-names/current-instantiation-partial-alias.cpp` (DEP-15)
- `templates/deduction/substitution-lambda-call-fails.cpp` (DED-29, LAM-15)
- `foundations/storage/implicit-objects-no-set-sanitizer.cpp` (STG-18)

Priority 3, and tests that the same changes fixed:

- `expressions/new-delete`: `aligned-fallback-array.cpp`, `aligned-fallback-without-alignment.cpp`, `aligned-forms-by-name.cpp`, `aligned-global-delete.cpp`, `aligned-new-passes-alignment.cpp`, `deallocate-by-name-alignment-mismatch-sanitizer.cpp`, `deallocate-by-name-null.cpp`, `deallocate-by-name-null-nothrow.cpp`, `deallocate-by-name-partner.cpp`, `global-functions-by-name.cpp`, `delete-class-converting-to-pointer.cpp`, `delete-array-class-converting-to-pointer.cpp`
- `foundations/language-support/alignment-enumeration-self-declared.cpp`, `alignment-enumeration-with-enumerator.cpp`
- `declarations/structured-bindings`: the 12 `tuple-*.cpp` tests that did not pass
- `classes/unions`: `anonymous-union-default-member-init.cpp`, `anonymous-union-init.cpp`, `anonymous-union-member.cpp`, `anonymous-union-nontrivial-default.cpp`, and `classes/constructors/const-object-anonymous-union-init.cpp`, `classes/comparisons/eq-anonymous-union-member.cpp`, `classes/members/standard-layout-base-type-anonymous-union.cpp`
- `classes/operators`: `arrow-chain.cpp`, `static-subscript.cpp`, `subscript-default-arg.cpp`, `subscript-multi.cpp`, `subscript-multi-order.cpp`, `subscript-zero-params.cpp`, and `classes/members/static-call-operator.cpp`, `static-subscript-no-parameters.cpp`
- `expressions/conditional`: `class-converts-to-int.cpp`, `derived-base-lvalue.cpp`, `two-classes-convert-to-int.cpp`, `two-classes-convert-to-pointer.cpp`
- `expressions/lambdas/generic.cpp`, `templates/dependent-names/generic-lambda-argument-lookup.cpp`
- `foundations/names`: `shared-name-class-alone.cpp`, `shared-name-enumeration-alone.cpp`, `shared-name-inner-block.cpp`, `shared-name-qualified-not-class.cpp`, and `statements/blocks/class-hidden-by-variable.cpp`, `enumeration-hidden-by-variable.cpp`
- `classes/conversion-functions/delete-class-operand.cpp`, `classes/initializer-lists/explicit-best-match-braced.cpp`, `narrowing-in-ctor-args.cpp`, `declarations/initialization/list/single-same-class-argument.cpp`, `single-same-class-copies.cpp`, `declarations/linkage-specifications/c-name-across-namespaces.cpp`, `statements/blocks/disambiguation-rest-not-declaration.cpp`

## 4. Items left, with reasons

- `W5/recursion.cpp`: `fib(30)` at run time takes more steps than the limit of 20 million. A limit of 200 million did not finish in two minutes, because each explored order of the `+` operands runs the program again. The interpreter needs to be faster, or the exploration needs to stop once a choice point has given the same outcome.
- `W13/header.cpp`: `sizeof` of a class with bit-fields needs the layout of BF-9. Class layout is left for a later round, as the task asked.
- `W9/mixin.cpp` and 9 other run tests stop at a virtual base. The interpreter keeps a base as a part of the object's value at a fixed path. A virtual base needs one subobject that every path reaches, and its place depends on the layout that this round leaves out.
- `expressions/primary/char-named-escape.cpp`: it needs the Unicode name table. Not attempted.
- Eight run tests stop at a `new` expression: placement arguments that need a user-defined conversion, placement deallocation templates, `new (int)(4)` with a parenthesized type, and bound checks with a nothrow form.
- New tests: 5 of the 11 new tests do not pass, with the same status under the round-14 front end. `foundations/overload-resolution/builtin-increment-from-conversion.cpp` is a false reject.

## 5. Findings

### G118. UNI-7 does not say how a class reaches the members of its anonymous union

- Page and rules: classes/unions, UNI-7, UNI-5; programs `W0/calc.cpp`, `W8/tagged.cpp`.
- Kind: missing.
- What happened: UNI-7 says that the members are members of the enclosing scope and variant members of the class. The page does not say that the anonymous union is itself an unnamed member object of the class, with a place among the members. The page also does not say that a mem-initializer can name a variant member, and that the initialization makes that member active. `tagged.cpp` writes `text(s)` in a mem-initializer. The front end models the union as an unnamed member, reaches its members through it, and treats such a mem-initializer as the initialization of the union with that member active.
- Suggestion: say that the anonymous union is an unnamed member of the class in declaration order, and that a mem-initializer or default member initializer of a variant member initializes the union with that member active. Add a run test with a mem-initializer for a member of a class type with a constructor.

### G119. STG-8 and STG-18 do not say when the set of implicit objects is fixed

- Page and rules: foundations/storage, STG-8, STG-18; tests `implicit-objects-no-set-sanitizer.cpp`, `implicit-object-first-use-member.cpp`, `implicit-objects-follow-later-use.cpp`.
- Kind: ambiguous.
- What happened: STG-8 says that the implementation creates the set of objects that gives the program defined behavior, and that a later use can decide the set. An implementer that runs the program sees each use only when it happens. The round-14 front end chose a new set at each cast and carried the bytes over, so an `int` write and a later `float` read of the same storage gave a value. The front end now treats a set as fixed once an object in it is read or written, except where a class of the new type holds the old object as a member. It also leaves out signed and unsigned integers of one size and byte types. These choices come from the tests, not from the rule text.
- Suggestion: state the observable rule. One phrasing is that after the program reads or writes an implicitly created object, an access to the same storage as an object of an unrelated type is undefined, unless an object that the later use names contains the earlier object. List the type pairs that do not count as unrelated, or point to the rule that lists them.

### G120. SB-5 does not give the type of the element index

- Page and rules: declarations/structured-bindings, SB-5, LSUP-3.
- Kind: missing.
- What happened: SB-5 writes `e.get<i>()`, `get<i>(e)`, and `std::tuple_element<i, E>::type`. The page does not say what type `i` has. The type decides which `get` templates match, for example `template <int I>` against `template <std::size_t I>`. The front end converts `i` to the type of the parameter of `std::tuple_element`, and passes `get` an `int` literal, which the template argument converts.
- Suggestion: say that `i` is a constant of type `std::size_t`, and add a test with a member `get` whose parameter has another integer type.

### G121. NEW-28 does not say which aligned deallocation function a `delete` expression prefers

- Page and rules: expressions/new-delete, NEW-28, NEW-22; test `aligned-global-delete.cpp`.
- Kind: missing.
- What happened: NEW-22 prefers the usual form with one parameter. NEW-28 says that the deallocation function gets the alignment. When a program declares both `operator delete(void*, std::align_val_t)` and `operator delete(void*, std::size_t, std::align_val_t)`, the page does not say which one a `delete` of an over-aligned object calls. The test accepts either. The front end calls the form without the size.
- Suggestion: extend NEW-22 to the aligned forms, or say that the choice is unspecified and keep the test as it is.

### G122. COND-11 does not say which built-in candidates a class with several conversion functions gives

- Page and rules: expressions/conditional, COND-3, COND-11.
- Kind: ambiguous.
- What happened: COND-11 says that overload resolution over the built-in candidates picks a conversion for each operand. The page does not list those candidates, and it does not say how to test that one class "converts to" the other. A class with conversion functions to `int` and to `long` needs the candidate list. The front end handles a class with one conversion function to a type that is not a class. For a class that is neither of those, the expression stays unsupported.
- Suggestion: name the candidate set (the built-in candidates of COND-11 for each pair of promoted arithmetic types and for pointer types), and state the "converts to" test as an implicit conversion sequence to the other type.

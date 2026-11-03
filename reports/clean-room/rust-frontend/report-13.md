# Front end, round 13

Date: 2026-11-03. Decisions 0029, 0035, 0051 (phase 6, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `questions/`, `frontend/`, `tools/`, the previous reports in `reports/clean-room/rust-frontend/`, and the Write-suite programs under `reports/clean-room/W7` and `W10`. No compiler was run, and no crates were added. Only `frontend/src/` changed. The other agent added tests during the round, and the suite grew from 5753 to 5879 tests. The new tests are judged only as new tests.

## 1. Conformance

### Checks

`python3 -I tools/frontend_check.py` (without `--update`), against the baseline `conformance.tsv` (5677 tests):

```
frontend: 5420/5879 pass, 5533/5879 pass their check, 0 worse, 46 better, 202 new (31 not passing)
frontend: 6/6 interpreter-only expectations met
```

The check exits 0. At the start of the round it reported `5267/5753 pass, 5377/5753 pass their check, 0 worse, 0 better, 76 new (12 not passing)`.

The check does not catch a change from one failing status to another, so the round-12 front end was also built from `HEAD` in a scratch directory and run on the same tests. Every test has the same status or a better one under the new front end. One test, `templates/ordering/reversed-candidate.cpp`, went from unsupported to a wrong answer partway through the round. ORD-7 fixed it (section 2), and the test passes now. The 31 new tests that do not pass give the same result on both front ends.

Status by test, for the 5677 baseline tests:

| Status | Baseline | After |
|---|---|---|
| pass | 5085 | 5130 |
| differs from EDG only | 118 | 119 |
| FALSE-REJECT | 43 | 28 |
| RUN-WRONG-RESULT | 1 | 1 |
| MISSED-REJECT | 206 | 192 |
| unsupported | 121 | 105 |
| run-unsupported | 103 | 102 |

Changes by test:

| Change | Tests |
|---|---|
| unsupported, now passes | 16 |
| false reject, now passes | 15 |
| missed reject, now passes | 14 |
| run unsupported, now passes | 1 |

`classes/derived/constexpr-dtor-virtual-base.cpp` changed from "pass" to "differs from EDG only". The status counts as a pass.

### Write-suite programs

| Program | Start | After |
|---|---|---|
| W7 `small_string.cpp` | unsupported at line 152 | runs, result 0 |

The other W7 and W10 programs (`date`, `money`, `rational`, `cleanup_guard`, `error_codes`, `noexcept_functions`, `transaction`) still run with result 0.

## 2. What changed in the front end

### Overload resolution

- OVL-27 and OVL-28 in operator overload resolution. A plain function beats a specialization in all cases, not only against a built-in candidate. Of two specializations, the one from the more specialized template wins. The early return to the older selection for a tie between a function and a specialization is gone.
- ORD-7: when a swapped candidate is ordered against a written or rewritten one, its two parameters take part in reverse order. The ordering is keyed by the function and its candidate kind. It is modeled for non-member candidates with two operands.
- The round-12 wrong answers on `eq-across-specializations.cpp` had one cause. When the front end replayed the declaration or the body of a specialization of a friend template defined in a class template, the replay registered the replayed function as a hidden friend of the class. That plain function then won by OVL-27. A replay no longer registers friends.
- OVL-26, Q-0189: a constructor of the target class and a conversion function now compare by their third parts. When the constructor wins a copy-initialization from another class type, the front end builds the constructor call for the interpreter.
- ICS-38: one string literal in braces binds directly to a reference to the literal's own array type, or to an array of unknown bound of its element type. ICS-46 then prefers the known bound. An rvalue reference does not bind to the literal.
- `->*` with a class operand goes through operator overload resolution (OVL-9 has no built-in candidate unless a class operand has a conversion function). SEQ-6 order follows from the call.

### Expressions, statements, and lambdas

- `&Add::operator()` is parsed. Before, the name was read as `operator(`.
- STMT-6: `Name(c)(a);` is a declaration of `c` with the initializer `(a)`.
- JMP-10: in a lambda, a variable of an enclosing function named in a return is not a local of the lambda, so it is not moved. Before, `[&]() -> int& { return value; }` gave REF-8.
- LAM-17: a reference bound directly to an uncaptured local constant, also through a call argument.

### Constant evaluation and the interpreter

- A `constexpr` class object with no initializer runs its default constructor in constant evaluation. Before, such an object was unsupported. The virtual-call and dynamic-cast constant tests pass because of this change.
- LSUP-11, NEW-37, STG-28: during constant evaluation, `::operator new` and `::operator delete` called by name are valid inside the member functions of a class template `allocator` in namespace `std`. A `void*` to `T*` conversion of that storage is valid there (G110). A leak is still CE-7, and a read of an element that was never built is still OBJ-2.
- ENUM-6: a load of an enumeration value outside the range of an enumeration without a fixed type is undefined behavior. `__builtin_memcpy` copies bytes in the interpreter (G112).

### Templates

- DEP-24 in a base list, an alias target, the other type-only positions of DEP-20, and a template template argument (`Holder<T::box>`). The pattern records the fact at the token, and the specialization reports it.
- DEP-26: `object.template field` with no argument list.
- DEP-19: a parameter type `T::type` without `typename` in a function template declared at namespace scope with an unqualified name. The specialization fails (G111).
- DEP-21: `typename` before a name that is not qualified.
- PACK-14: a base-list expansion `Bs...` and a mem-initializer expansion `Bs(args)...` in a specialization give one base and one initializer per element. PACK-15: a mem-initializer expansion that names a data member.
- PACK-16: `alignas(T...)` and `alignas(N...)` take the largest alignment. `alignas` in the head of a class template now applies to its specializations at all, and before it was ignored.
- PACK-21: `using Bs::f...;` and `using Bs::Bs...;`. An element of a type pack before `::` now names its class. In a pattern, the placeholder that DEP-23 declares for a dependent using-declaration no longer conflicts with a member function of the same name.
- INST-5: in a class specialization, the definition of a scoped enumeration member is parsed with its errors dropped (an approximation). In a pattern, an enumerator value that depends on a parameter is no longer unsupported.

### Classes

- DER-11: `static_cast` of a pointer or reference from a virtual base to a derived class.
- DER-12: a `constexpr` or `consteval` constructor or destructor in a class with a virtual base at any depth.
- DER-13: a constructor of a class that is not abstract must name each virtual base that has no default constructor, unless the constructor delegates.
- DER-21: a direct base, or a virtual base at any depth, whose destructor is private and which does not befriend the class makes the class's implicit destructor deleted.

## 3. Items fixed

Priority 1, from the named list:

- W7 `small_string.cpp` line 152, `classes/comparisons/eq-across-specializations.cpp`, `tie-template-before-rewritten.cpp`, `tie-template-before-swapped.cpp`
- `expressions/lambdas/static-call-operator-named.cpp` (new test, passes)
- `foundations/conversion-sequences/list-string-literal-binds-literal.cpp` (ICS-38)
- `foundations/overload-resolution/tie-conversion-result-constructor.cpp` (OVL-26)
- `expressions/sequencing/operand-used-twice-evaluated-once.cpp`
- `expressions/sequencing/overloaded-member-pointer-order.cpp`
- `statements/blocks/disambiguation-declarator-forms.cpp`
- `declarations/enumerations/bytes-out-of-range-sanitizer.cpp`
- `foundations/language-support/allocator-self-declared-constant.cpp`, `foundations/storage/allocation-array-class-elements-constant.cpp`

Priority 2:

- DEP-24: `template-keyword-needed-base.cpp`, `template-keyword-needed-alias-target.cpp`, `template-keyword-needed-template-argument.cpp`
- Other `templates/dependent-names` tests: `template-keyword-no-list.cpp` (DEP-26), `typename-needed-parameter.cpp` (DEP-19), `typename-needs-qualified-name.cpp` (DEP-21)
- LAM-17 for a reference is now modeled. No test covers it yet.
- `classes/derived`: `virtual-base-downcast.cpp`, `constexpr-ctor-virtual-base.cpp`, `most-derived-must-init-virtual-base.cpp`, `virtual-base-private-dtor.cpp`

Priority 3 and other tests that pass now:

- `templates/packs`: `alignment-specifier.cpp`, `alignment-specifier-class.cpp`, `base-list-and-initializers.cpp`, `base-list-order.cpp`, `base-list-duplicate.cpp`, `member-initializer-data-member.cpp`, `member-initializer-data-member-braced.cpp`, `using-declaration-pack.cpp`, `using-declaration-empty-pack.cpp`, `using-declaration-empty-pack-outer-name.cpp`, `using-declaration-empty-pack-only.cpp`, `empty-expansion.cpp`
- `templates/instantiation/class-scoped-enumeration-unused.cpp`
- `templates/ordering/reversed-candidate.cpp`, `reversed-member-object.cpp`
- `templates/class-template-deduction/aggregate-candidate-pack.cpp`
- The constant-default-initialization group: `classes/virtual/call-during-construction-constant.cpp`, `dynamic-cast-constant.cpp`, `virtual-in-constexpr.cpp`, `expressions/constant-expressions/virtual-call.cpp`, `declarations/constexpr/variable-implicit-constructor-member-initializers.cpp`
- `foundations/language-support/ordering-converting-constructor.cpp`, `foundations/overload-resolution/init-constructor-against-conversion.cpp`, `declarations/initialization/references/braced-string-literal-rvalue-ref.cpp`, `classes/operators/arrow-star-order.cpp`

## 4. Items left, with reasons

- Long double (`ieee-types.cpp`, `floating-range*.cpp`, `floating-divide-by-zero-infinity.cpp`) and `char-named-escape.cpp` (Unicode name table): not attempted, as allowed.
- `constructor-ignores-member-destructors.cpp`: unchanged from round 12.
- Packs, not reached: init-capture packs (`init-capture-pack.cpp`, and line 17 of `size-operator.cpp`), a lambda in a pattern (`lambda-in-pattern.cpp`), `function-pack-array-and-unnamed.cpp`, `lengths-differ-substitution.cpp`, `nondeduced-before-end-partial.cpp`.
- Member templates of explicit and partial specializations, not reached: `partial-member-template.cpp`, `member-template-explicit-primary.cpp`, `member-template-explicit-partial.cpp`, `explicit-member-class-member-outside.cpp`.
- Constraints on members, not reached: `member-specialization-by-constraints.cpp`, `member-specialization-first-member.cpp`, `explicit-instantiation-members.cpp`, `mapping-same-arguments.cpp`.
- CTAD, not reached: `aggregate-candidate-arrays.cpp`, `aggregate-pack-length-fewer.cpp`, `list-single-specialization.cpp`, `template-template-parameter-alias.cpp`, and the CTAD sites tests (unsupported).
- Other template false rejects, not reached: `explicit-not-constexpr.cpp` (TSP-25), `declaration-specialization-friend.cpp`, `substitution-lambda-call-fails.cpp`, `requirement-access-concept-in-member.cpp`, `partial-specializations-placeholder.cpp`.
- INST-5 for a scoped enumeration is an approximation. If a later use needs an enumerator whose value has an error, the front end does not report the error.
- ORD-7 is modeled only for non-member candidates with two operands. A member candidate against a swapped one still goes by kind (OVL-32, OVL-33).
- Missed rejects: 192 of the baseline tests remain. In `templates/dependent-names` (about 11), the main groups are DEP-30 and DEP-17 (a missing member of the current instantiation), DEP-37, and DEP-15. In `templates/packs` (6), the groups are PACK-8, PACK-6, PACK-2, and PACK-11.
- 31 of the 202 new tests do not pass. Their status is the same under the round-12 front end. Two of them are a wrong answer or a false reject: `declarations/structured-bindings/bit-field-assign-wraps.cpp` (RUN-WRONG-RESULT) and `declarations/declarators/multidim-outer-omitted-strings.cpp` (FALSE-REJECT). The round did not reach them.

## 5. Findings

### G110. CE-4 rejects the conversion that `std::allocator::allocate` needs

- Page and rules: foundations/constant-evaluation, CE-4; foundations/language-support, LSUP-11; foundations/storage, STG-28; expressions/new-delete, NEW-37.
- Kind: contradiction.
- What happened: CE-4 says that constant evaluation must not convert a pointer to `void` to a pointer to an object type. LSUP-11 allows `::operator new` by name inside the members of `std::allocator`, and its tests write `static_cast<T*>(::operator new(n * sizeof(T)))`. The tests expect acceptance. With CE-4 as written, every such `allocate` fails at the cast. The front end allows the conversion when the operand points to storage from an allocation function called by name.
- Suggestion: add an exception to CE-4 for the result of an allocation function that a member of `std::allocator` calls (LSUP-11), converted to a pointer to the element type (STG-28). Or state in LSUP-11 that the conversion in `allocate` is valid.

### G111. DEP-19 does not say how `f(T::type) { ... }` at namespace scope is read

- Page and rules: templates/dependent-names, DEP-19, DEP-20, DEP-29.
- Kind: ambiguous.
- What happened: DEP-20 does not list the parameters of a function template declared at namespace scope with an unqualified name. By DEP-19, `T::type` there is a value. Then `template <class T> int take(T::type) { return 0; }` reads as a variable template with a parenthesized initializer, followed by a body that the grammar does not allow. That reading would make the template a DEP-29 error at the definition. The test `typename-needed-parameter.cpp` instantiates `take<Has>` and names only DEP-19. The front end reads the declaration as a function and makes each specialization fail.
- Suggestion: say whether the declaration is a syntax error at the definition (DEP-29) or a function whose specialization is not valid (DEP-19). An example would settle it.

### G112. An ENUM-6 test uses a builtin that no page names

- Page and rules: declarations/enumerations, ENUM-6; test `declarations/enumerations/bytes-out-of-range-sanitizer.cpp`.
- Kind: missing.
- What happened: the test copies bytes with `__builtin_memcpy`. No Tidal page declares or describes this name, so a front end built from the pages finds an undeclared function (NAM-8). The front end now models `__builtin_memcpy` from outside knowledge.
- Suggestion: have the test declare its own byte copy (a loop over `unsigned char`), or list the builtins that tests can use and their meaning on a page.

### G113. DER-21 does not say that a friend has access

- Page and rules: classes/derived, DER-21; classes/access.
- Kind: ambiguous.
- What happened: DER-21 says that a private constructor or destructor does not give the derived class access, so the derived special member is deleted. In `virtual-base-private-dtor.cpp`, `A` has a private destructor and befriends `B`. The test needs `B : virtual A` to be valid and `D : B` to have a deleted destructor. Read literally, DER-21 also deletes the destructor of `B`. The front end applies the access rules: a friend has access, and another class does not.
- Suggestion: say that the rule follows the access rules, so a base whose special member is private still gives access to a friend. Add the friend case to the example.

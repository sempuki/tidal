# Front end, round 14

Date: 2026-11-03. Decisions 0029, 0035, 0051 (phase 6, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `questions/`, `frontend/`, `tools/`, the previous reports in `reports/clean-room/rust-frontend/`, and the Write-suite programs under `reports/clean-room/W7` and `W10`. No compiler was run, and no crates were added. Only `frontend/src/` changed, with one new file, `frontend/src/interp/f80.rs`. The other agent added tests during the round, and the suite grew from 5998 to 6026 tests. The new tests are judged only as new tests.

## 1. Conformance

### Checks

`python3 -I tools/frontend_check.py` (without `--update`), against the baseline `conformance.tsv` (5998 tests):

```
frontend: 5590/6026 pass, 5703/6026 pass their check, 0 worse, 46 better, 31 new (11 not passing)
frontend: 4/4 interpreter-only expectations met
```

The check exits 0. At the start of the round it reported `5527/5998 pass, 5642/5998 pass their check, 0 worse, 0 better, 0 new (0 not passing)` and `6/6 interpreter-only expectations met`. The interpreter-only count fell from 6 to 4 because the other agent changed tests during the round, not because a run failed.

The check does not catch a change from one failing status to another. So the round-13 front end was built from `HEAD` in a scratch directory and run on the same tests after each change. Every test has the same status or a better one under the new front end. The 11 new tests that do not pass give the same result on both front ends.

Status by test, for the 5998 baseline tests:

| Status | Baseline | After |
|---|---|---|
| pass | 5404 | 5451 |
| differs from EDG only | 123 | 122 |
| FALSE-REJECT | 30 | 2 |
| RUN-WRONG-RESULT | 3 | 0 |
| MISSED-REJECT | 217 | 205 |
| unsupported | 109 | 106 |
| run-unsupported | 112 | 112 |

Changes by test:

| Change | Tests |
|---|---|
| false reject, now passes | 28 |
| wrong run result, now passes | 3 |
| missed reject, now passes | 12 |
| unsupported, now passes | 3 |
| differs from EDG only, now passes | 1 |

### Write-suite programs

The W7 programs (`date`, `money`, `rational`, `small_string`) and the W10 programs (`cleanup_guard`, `error_codes`, `noexcept_functions`, `transaction`) still run with result 0. The probe programs give the same results as under the round-13 front end.

## 2. What changed in the front end

### Withdrawn rule IDs and limits

- The interpreter's stops and the checks now name live rules. CE-25 is now CE-11, CE-20 is CE-17, CEV-3 is CX-5, UNW-9 is UNW-1, SEQ-3 is SEQ-2, and INST-14 is TR-13. A grep for SEQ-10, CE-29, CE-31, CE-32, CX-6, CEV-4, REF-13, and DCL-10 finds nothing.
- TR-13: `-ftemplate-depth=N` and `--pending_instantiations=N` in a test's flags, or on the `check` command line, set the depth limit. The default stays 256. Both options limit the total depth, which approximates EDG's limit per template.
- STG-1, TR-15: `sizeof` of a class or array whose size, without padding, does not fit in `std::size_t` is an error. The front end uses EDG's limit, which is the largest one. Only `sizeof` is checked.

### Templates

- DEP-29, DEP-19 (G111 repaired): `template <class T> int take(T::type) { ... }` at namespace scope is an error at the definition.
- TPL-38, TPL-20, TPL-22: a function template is identified by its declaration, so a template that differs in its return type, its head, or its parameters is another template. A function template's return type now always takes part in the match.
- DEP-17: `this->name` and `C::name` in the current instantiation must find a member when the class has no dependent base at any depth.
- DEP-15: inside a template, a dependent template-id before `::` names the current instantiation only if it gives the template's own parameters in order. Inside a partial specialization, it must give that partial specialization's argument tokens. Any other such template-id makes its member a value, and a declarator after it is an error.
- CNS-37, TSP-21, TSP-22, TSP-9: `template <> template <class U> ... C<A>::f` redefines the member template of the instance. A trailing clause is matched after substitution, and a type spelled by name now equals a parameter bound to that type in the clause key. The specialization of a static member template is static. An explicit specialization of a member class of an instance replaces the class. The primary member class template can also be given again. A partial specialization of a member template defined outside the class now instantiates.
- CNS-15: an explicit instantiation of a class leaves out the members whose constraints are not satisfied.
- CNS-4: two concept-ids whose arguments the front end cannot map count as one atom when their tokens name the same things.
- CPT-26: access in a requirement is checked where the check happens, also through the replay of a concept.
- ORD-14, DED-12: `auto*` and `auto**` heads are not equivalent, and partial ordering compares their patterns.
- PACK-12: a list with an expansion before its end is a non-deduced position in a partial specialization's match, and the substituted pattern must equal the arguments.
- PACK-6, PACK-27: the error for packs of different lengths is no longer dropped. Substitution then fails. A replay no longer inherits the outer expansion's pack counts.
- PACK-18: init-capture packs `[...y = e]` and `[&...y = x]`. PACK-5: a lambda capture of a pack in a pattern names the element of each copy. A fold whose operand holds a lambda body is recognized.
- DED-32: `friend int f<>(X*);` names the specialization that ordering selects, which gets the friend's access.
- TSP-25: the arguments of a template-id are constant expressions, so a call in them instantiates the function also inside an unevaluated operand.
- Deduction: the cv-qualifiers of an array are those of its elements. A constexpr array has const elements. `const T[N]` deduces `T` from a string literal.
- CTAD: the initializer-list phase of ILC-3, with the CTAD-18 exception for one element of a specialization. CTAD-27 and Q-0276: a pack whose length an earlier element fixes keeps that length, and a base with no element gets an empty list.
- TR-12: when a deferred member body split a `>>`, the inserted token moved the class end and later bodies. Positions are now adjusted.

### Expressions, declarations, and the interpreter

- TYP-10, MUL-20: `long double` is the x87 extended format in a software type (`F80`), with literals, arithmetic, comparisons, conversions, and bytes. Decimal literals can be off by a few units in the last place, and subnormal values are flushed to zero.
- SB-10: an assignment through a binding to a bit-field wraps.
- DCL-6: `char names[][4] = {"ab", "cde"}` has two elements.
- SINIT-7: two static member definitions in a row follow their own order.
- EXS-4, UN-18: the initializer of a new expression is not a temporary, so no destructor of it counts.
- A trivially copyable class argument can be passed to an ellipsis during constant evaluation.

## 3. Items fixed

Priority 1:

- `foundations/translation/limit-depth-flag-lowered.cpp`, `limit-depth-edg-flag-lowered.cpp` (TR-13)
- `foundations/storage/class-size-exceeds-size-type-ilp32.cpp`, and `class-size-exceeds-size-type.cpp` (STG-1)
- `templates/dependent-names/typename-needed-parameter.cpp` (DEP-29)
- `templates/declarations/default-friend-overloads.cpp` (TPL-38)
- The withdrawn IDs (section 2)

Priority 2, the baseline false rejects and wrong results:

- `declarations/structured-bindings/bit-field-assign-wraps.cpp`, `declarations/declarators/multidim-outer-omitted-strings.cpp`, `declarations/initialization/static/static-member-order-not-class-order.cpp`, `declarations/exception-specifications/constructor-ignores-member-destructors.cpp`
- Long double: `expressions/multiplicative/ieee-types.cpp`, `floating-divide-by-zero-infinity.cpp`, `foundations/types/floating-range.cpp`, `floating-range-ilp32.cpp`
- `templates/specialization`: `explicit-not-constexpr.cpp`, `explicit-member-class-member-outside.cpp`, `member-template-explicit-primary.cpp`, `member-template-explicit-partial.cpp`, `partial-member-template.cpp`
- `templates/constraints`: `member-specialization-by-constraints.cpp`, `member-specialization-first-member.cpp`, `explicit-instantiation-members.cpp`, `mapping-same-arguments.cpp`
- `templates/class-template-deduction`: `aggregate-candidate-arrays.cpp`, `aggregate-pack-length-fewer.cpp`, `list-single-specialization.cpp`, `template-template-parameter-alias.cpp`
- `templates/packs`: `function-pack-array-and-unnamed.cpp`, `init-capture-pack.cpp`, `lambda-in-pattern.cpp`, `lengths-differ-substitution.cpp`, `nondeduced-before-end-partial.cpp`, `size-operator.cpp`
- `templates/concepts/requirement-access-concept-in-member.cpp`, `templates/deduction/declaration-specialization-friend.cpp`, `templates/ordering/partial-specializations-placeholder.cpp`

Priority 3, missed rejects and unsupported tests that pass now:

- `templates/dependent-names`: `current-member-missing.cpp`, `this-member-missing.cpp`, `this-member-missing-specialized.cpp` (DEP-17), `other-specialization-needs-typename.cpp`, `partial-primary-form-needs-typename.cpp`, `current-instantiation-constant-member.cpp` (DEP-15)
- `templates/packs/lengths-differ.cpp` (PACK-6)
- `templates/class-template-deduction/aggregate-pack-length-fixed.cpp`, `aggregate-pack-length-conflict.cpp`, `templates/deduction/match-constant-placeholder-pattern.cpp`

## 4. Items left, with reasons

- `expressions/primary/char-named-escape.cpp`: it needs the Unicode name table. Not attempted.
- `templates/deduction/substitution-lambda-call-fails.cpp`: the trailing return type of a generic lambda's call operator is not substituted when the lambda is called, so `[](auto x) -> decltype(x.missing) {...}(1)` is accepted. This needs a substitution step for the lambda's call operator.
- TR-13: the front end counts the total depth, not EDG's depth per template.
- STG-1: only `sizeof` reports a type that is too large. An object definition does not.
- Long double: decimal literals are computed in the extended format, and subnormal values are flushed to zero.
- Missed rejects: 205 baseline tests remain. In `templates/dependent-names` (6), the groups are DEP-37, DEP-12, DEP-30 for calls and base members, and DEP-31. In `templates/packs` (5), the groups are PACK-8 in an outer pattern, PACK-11, PACK-26, and PACK-2.
- New tests: 11 of the 31 do not pass, with the same status under the round-13 front end. Three are false rejects of NAM-39 and NAM-40 (`foundations/names/shared-name-class-members.cpp`, `shared-name-enumerator-class.cpp`, `shared-name-variable-found.cpp`). In these, a class and a variable, function, or enumerator share one name in one scope. The other agent was still writing that group, so the round did not reach it.

## 5. Findings

### G114. The TR-13 option tests do not say which limit a front end applies

- Page and rules: foundations/translation, TR-13; tests `limit-depth-flag-lowered.cpp`, `limit-depth-edg-flag-lowered.cpp`.
- Kind: ambiguous.
- What happened: each test expects a reject from every implementation. For one compiler the reason is the lowered limit, and for the others the reason is an unknown option. A front end that is none of the matrix compilers has to choose. It can reject the option, or apply the option as Clang and GCC do, or apply it as EDG does. The front end applies both options as a limit on the total depth.
- Suggestion: say what an implementation outside the table does with each option, or mark these tests as specific to the compilers that know the option.

### G115. CTAD-27 does not say how a base with no element enters the aggregate candidate

- Page and rules: templates/class-template-deduction, CTAD-9, CTAD-27; Q-0276.
- Kind: missing.
- What happened: CTAD-27 says that a base of the pack with no element "gets an empty list, as in aggregate initialization". The candidate is a function, so its parameters for those bases need arguments. `aggregate-pack-length-conflict.cpp` also needs each later element to deduce the element of the pack in its place. The page does not say that. The front end deduces each later element against the pack element in its position, and it gives the missing parameters defaults.
- Suggestion: state the candidate's parameters when an earlier element fixes the pack's length. Say that the later elements still deduce, position by position, and that the parameters without an element have default arguments.

### G116. DEP-15 does not say how to compare a partial specialization's argument list

- Page and rules: templates/dependent-names, DEP-15.
- Kind: ambiguous.
- What happened: inside a partial specialization, "the template name with that partial specialization's argument list" names the current instantiation. The page does not say whether the lists compare by spelling or by meaning. For example, the page does not say whether an alias for `T*` counts. The front end compares the tokens.
- Suggestion: say that the arguments must be equivalent (TPL-20) and give an example with an alias.

### G117. CPT-26 does not say that a concept-id keeps its first result

- Page and rules: templates/concepts, CPT-26; test `requirement-access-concept-in-member.cpp`.
- Kind: missing.
- What happened: the test's note says that the concept-id is "first checked inside a member" of `Box`, where the private member is accessible. The note suggests that later checks give the same result. CPT-26 says only that access is checked where the check happens. A front end that checks each concept-id again at each use would get another answer at namespace scope. The front end checks access at the point of the first check and keeps the result for each set of arguments.
- Suggestion: state whether a satisfaction result is fixed at the first check of a concept-id with given arguments (Q-0288), and add a test that checks the same concept-id again at namespace scope.

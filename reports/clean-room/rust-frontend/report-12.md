# Front end, round 12

Date: 2026-11-03. Decisions 0029, 0035, 0051 (phase 6, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `questions/`, `frontend/`, `tools/`, the previous reports in `reports/clean-room/rust-frontend/`, and the Write-suite programs under `reports/clean-room/W7` and `W10`. No compiler was run, and no crates were added. Only `frontend/src/` changed. During the round the other agent added tests (the suite grew from 5677 to 5753 tests). The new tests are judged only as new tests.

## 1. Conformance

### Checks

`python3 -I tools/frontend_check.py` (without `--update`), against the baseline `conformance.tsv` (5677 tests):

```
frontend: 5267/5753 pass, 5377/5753 pass their check, 0 worse, 103 better, 76 new (12 not passing)
frontend: 6/6 interpreter-only expectations met
```

The check exits 0. At the start of the round it reported `5101/5677 pass, 5227/5677 pass their check, 0 worse, 0 better`.

Status by test, for the 5677 baseline tests:

| Status | Baseline | After |
|---|---|---|
| pass | 4986 | 5085 |
| differs from EDG only | 115 | 118 |
| FALSE-REJECT | 57 | 43 |
| RUN-WRONG-RESULT | 2 | 1 |
| MISSED-REJECT | 249 | 206 |
| unsupported | 144 | 121 |
| run-unsupported | 124 | 103 |

Changes by test:

| Change | Tests |
|---|---|
| missed reject, now passes | 43 |
| run unsupported, now passes | 22 |
| unsupported, now passes | 23 |
| false reject, now passes | 13 |
| wrong result, now passes | 1 |
| false reject, now run-unsupported | 1 |

Two tests changed from "pass" to "differs from EDG only". Both count as passes.

### Write-suite programs

| Program | Start | After |
|---|---|---|
| W7 `money.cpp` | unsupported (concept) | runs, result 0 |
| W10 `cleanup_guard.cpp` | unsupported (CTAD from a lambda) | runs, result 0 |
| W7 `small_string.cpp` | unsupported (CTAD-11 user guide) | unsupported at line 152 (see section 4) |

The other W programs still run with result 0.

## 2. What changed in the front end

### Templates

- TPL-35, Q-0195: a function template whose parameter after a pack has no default and is not deducible from the function parameters is rejected at the declaration.
- TPL-38, Q-0197: a friend function template defined with a default must be the only declaration of the template. The check covers an earlier namespace declaration and a later one.
- A friend function template defined in its class is now recorded as a template, so operator overload resolution can deduce its specializations.
- Operator overload resolution expands operator function templates for written, rewritten, and swapped candidates (DED-1). A tie between a plain function and a specialization still goes to the older selection, as before. OVL-27 now decides a tie between a built-in candidate and a specialization.
- DEP-24 for qualified names, in an expression (`T::make<int>()`) and after `typename`. DEP-26 for `T::template field` with no argument list. `Holder<T::template box>` no longer gives a false DEP-19.
- DEP-9: a function type inside another type whose `noexcept` operand depends on a parameter is dependent, so `price(p)` waits for the specialization.
- DEP-2: in a pattern, a deduced return type from a dependent return value stays unknown. Before, it became `void`. In a specialization, a call of a member with a deduced return type makes that member's body first (INST-31).
- INST-5: when a class specialization defers its member bodies, it records them all before it parses the constructor bodies. A constructor can then call a constexpr member declared after it.
- CTAD-11: a user guide whose result has expression arguments (`S<M - 1>`) parses the result again after deduction.
- CTAD: in a class template pattern, a copy or move constructor's parameter is read as a reference to the class. Before, the whole deduction gave up.
- A concept-id instantiates its class arguments before the check, so hidden friends of a specialization are found.

### Classes and initialization

- DER-8, DER-15, DER-20, OVL-17, OVL-31: inherited constructors. `using B::B;` gives `D` one constructor for each constructor of `B` that is not a default, copy, or move constructor and that `D` does not hide. The constructor initializes `B` with the arguments, and the other parts get their usual initialization. The inherited constructor keeps the access, `explicit`, `noexcept`, and default arguments of the base constructor. `D` keeps an implicit default constructor and is no longer an aggregate. A call with one argument skips an inherited constructor whose first parameter is a reference to `B`, to `D`, or to a class between them. A derived constructor wins a tie with an inherited one. Two bases that give the selected signature are DER-15. The interpreter does not use an inherited constructor as a copy constructor.
- LSUP-1, ILC-1, ILC-3, ILC-5: a program's own `std::initializer_list` template is used for braced lists. The front end builds an array of the converted elements and calls the constructor that takes the pointer and the count. The array lives as long as the list object. An initializer-list constructor that is not a template is chosen before the constructor templates are expanded. Empty braces take the default constructor.
- SB-1 to SB-4, SB-13, SB-16, RFOR-3: structured bindings for arrays and classes run. A hidden variable holds the initializer, and each name refers to an element or a member. This works in range-for and in lambda captures. SB-13 rejects members split across bases, an anonymous union member, a union, and an inaccessible member. `&b` for a bit-field binding is BF-4.
- BF-12: an unnamed bit-field is not a member for copies, comparisons, and structured bindings.
- CPY-21: assignment of another class type to a class with no declared `operator=` goes through a converting constructor, so a deleted converting constructor template is DEL-1.
- PFX-11: member access into the class being defined is allowed.
- `T(5)` after a type name is an expression and not a declarator (DCL-1). `Money<1> c(Money<2>(5));` was TR-12 before.
- A constructor call with one argument of non-class type no longer gives up when an implicit copy constructor could take the argument through a user-defined conversion.

### Lambdas

- LAM-9: an uncaptured automatic variable, and a variable that an enclosing lambda does not capture. A constant read as a value needs no capture.
- LAM-17: a member used without capturing `this`, the address of an uncaptured constant, a parameter with the name of a capture, and `mutable` with an explicit object parameter.
- LAM-21: a warning for `this` captured only through `[=]`.
- LAM-23: an init capture by copy is `const` in a lambda that is not mutable.
- LAM-24: `this` captured outside a non-static member function. LAM-27: a static lambda with captures.
- LAM-6: a capture by copy of an abstract class. LAM-26: a closure with captures has no default constructor and no copy assignment.

### Constant evaluation and the interpreter

- IF-25, G106: a static local of class type with scalar parts is initialized by constant evaluation when its initializer is a constant expression. Otherwise it runs at run time.
- Q-0193: the temporaries that the reference members of an extended aggregate bind are extended with it (`constexpr const Holder& held = Holder{4};`).
- CE-10: the step limit of constant evaluation is 30 million. The three moderate tests pass, and `long-loop.cpp` and `excessive-call-count.cpp` are still rejected.
- CE-20: a static `const` integer local with a constant initializer is usable.
- CMP-7, CMP-26, CE-14: a global class or array that is not usable in constant expressions keeps its shape, so the addresses of its parts compare during constant evaluation. A read of a part is still CE-3.

### Warnings

- VOL-5 (under `-Wall`, `-Wdeprecated`, or `-Wvolatile`): `++` and `--` on a volatile object, the value of an assignment to a volatile object used in a larger expression, and a volatile structured binding.
- CV-9: a volatile parameter or return type.

## 3. Items fixed (priority 1)

From the named list:

- `templates/declarations/pack-then-not-deducible.cpp`, `pack-then-pack-not-deducible.cpp`, `pack-then-typed-pack-not-deducible.cpp`
- `templates/declarations/default-friend-redeclared.cpp`, `default-friend-declared-before.cpp`
- `statements/if/consteval-static-fallback.cpp`
- The Q-0193 nested read (`read-extended-temporary-nested.cpp`) no longer stops at OBJ-2 in the interpreter.

Baseline false rejects that pass now:

- `classes/unions/bitfield-unnamed-not-member.cpp`, `bitfield-unnamed-value-init-copy.cpp`
- `expressions/comparison/one-past-member-adjacent-constant.cpp`, `pointer-array-member-order.cpp`
- `expressions/constant-expressions/moderate-call-count.cpp`, `moderate-loop.cpp`, `moderate-loops-split.cpp`, `static-local-usable-reached.cpp`
- `expressions/postfix/member-access-class-being-defined.cpp`
- `classes/copy-move/template-ctor-by-value-own-class.cpp`
- `templates/arguments/pointer-arguments.cpp`
- `templates/dependent-names/dependent-type-noexcept.cpp`, `type-dependent-deduced-return.cpp`

`templates/dependent-names/template-keyword-forms.cpp` moved from a false reject to run-unsupported.

The inherited-constructor tests (`classes/derived/inherited-ctor*.cpp`, the OVL-17 and OVL-31 tests, `inheriting-ctor-noexcept.cpp`, and the CTAD inherited-copy tests) pass, except `tie-own-constructor-template.cpp`.

The 43 missed rejects that pass now include 6 `classes/derived`, 13 `expressions/lambdas`, 4 `declarations/structured-bindings`, 5 VOL-5 and CV-9 tests, 5 TPL-35 and TPL-38 tests, 3 `templates/dependent-names`, and 5 OVL-17 and CTAD tests.

## 4. Items left, with reasons

- W7 `small_string.cpp`: line 152 (`SmallString<2>("hi") == word`) is unsupported. The candidates come from the friend templates of two specializations, and one is a written candidate while the other is swapped. In this case the front end produces specializations that it cannot tell apart reliably, so it leaves a tie between a function and a specialization to the older selection, which does not handle it. `classes/comparisons/eq-across-specializations.cpp` and the two `tie-template-before-*` tests stay unsupported for the same reason. Earlier in this round the change gave wrong answers on those three tests. Those answers were removed, and no test gives a wrong answer now.
- Long double (`ieee-types.cpp`, `floating-range*.cpp`, `floating-divide-by-zero-infinity.cpp`): the evaluator uses `f64` (G98).
- `char-named-escape.cpp`: needs a Unicode name table.
- `allocator-self-declared-constant.cpp`, `allocation-array-class-elements-constant.cpp`: allocation in constant evaluation.
- `constructor-ignores-member-destructors.cpp`: a placement `new` with a class `operator new` stays opaque to the `noexcept` operator.
- `list-string-literal-binds-literal.cpp` (ICS-38), `tie-conversion-result-constructor.cpp` (OVL-26), `operand-used-twice-evaluated-once.cpp`, `overloaded-member-pointer-order.cpp`, `disambiguation-declarator-forms.cpp`, `bytes-out-of-range-sanitizer.cpp`: not reached.
- Templates (about 30 tests): packs (alignment specifiers, using-declarations, init-capture packs, a lambda in a pattern, mem-initializers, `sizeof...`), member templates of explicit and partial specializations, constraints on members, and some CTAD sites. Not reached.
- DEP-24 in a base list, an alias target, and a template template argument (`T::box<int>` and `Holder<T::box>`) is still missed. These names go through other type paths.
- LAM-17 for a reference bound to an uncaptured constant is not modeled. Only the address case is.
- The new test `expressions/lambdas/static-call-operator-named.cpp` is a false reject, because `&Add::operator()` is not parsed.
- Missed rejects: 206 of the baseline tests remain. The largest groups are `templates/dependent-names` (about 15), `templates/packs` (10), and `classes/derived` (4, virtual bases).

## 5. Findings

### G107. VOL-5 does not say whether its warnings are on by default

- Page and rules: declarations/volatile, VOL-5. Compare declarations/declarators, CV-9.
- Kind: missing.
- What happened: the evidence for VOL-5 says that Clang and GCC warn "under `-Wall`". The evidence for CV-9 says that the compilers warn by default and also under `-Wall` and `-Wdeprecated`. A front end has to decide whether to emit the VOL-5 warnings without a flag. Under `-Werror`, that choice decides acceptance. The front end now emits VOL-5 only under `-Wall`, `-Wdeprecated`, or `-Wvolatile`, and CV-9 always.
- Suggestion: say in the VOL-5 evidence whether each warning is on by default, and name the flags that turn it on.

### G108. DER-8 does not say which constructors `D` keeps beside the inherited ones

- Page and rules: classes/derived, DER-8, DER-20; classes/constructors, CTOR-2, CTOR-20.
- Kind: missing.
- What happened: DER-8 lists the constructors that are not inherited and says that `D` is not an aggregate. The rule does not say that `D` keeps its implicit default, copy, and move constructors, or that `using B::B;` alone makes `D` a non-aggregate when no constructor is inherited. Two tests need these facts. `inherited-ctor-two-bases-unused.cpp` (`D d;` accepted) needs the implicit default constructor of `D`. `inherited-ctor-not-copy.cpp` (`D d(b);` rejected) needs `D` to be a non-aggregate when it inherits nothing. An inherited constructor also does not suppress the implicit default constructor of `D` the way a constructor declared in `D` does (CTOR-2).
- Suggestion: add to DER-8 that the inherited constructors do not count as constructors declared in `D` for CTOR-2, so `D` keeps its implicit default, copy, and move constructors. Also state that `D` is not an aggregate even when it inherits no constructor.

### G109. TPL-38 does not say when two declarations declare one function template

- Page and rules: templates/declarations, TPL-38; Q-0197.
- Kind: ambiguous.
- What happened: TPL-38 requires that a friend definition with a default is "the only declaration of the function template". The page does not say how to tell that a namespace-scope declaration declares the same template as a friend definition when overloads exist. The front end matches by name, number of function parameters, and number of template parameters.
- Suggestion: refer to the rule that decides when two function template declarations declare the same template (same name, equivalent heads, and equivalent parameter types), or add an example with an overload that is a different template.

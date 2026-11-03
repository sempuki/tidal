# Front end, round 11

Date: 2026-11-03. Decisions 0029, 0035, 0051 (phase 6, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `questions/`, `frontend/`, `tools/`, the previous reports in `reports/clean-room/rust-frontend/`, and the Write-suite programs under `reports/clean-room/W2`, `W6`, `W7`, `W10`, and `W12`. No compiler was run, and no crates were added. Only `frontend/src/` changed. The other agent's commits during this round (up to 5784d69) did not touch `frontend/`.

## 1. Conformance

### Checks

`python3 -I tools/frontend_check.py` (without `--update`), against the baseline `conformance.tsv` (5655 tests):

```
frontend: 5096/5664 pass, 5209/5664 pass their check, 0 worse, 68 better, 10 new (3 not passing)
frontend: 6/6 interpreter-only expectations met
```

The final line is `5096/5664 pass, 5221/5664 pass their check, 0 worse, 68 better, 10 new (3 not passing)`. It exits 0. At the start of the round it reported `5023/5657 pass, 5174/5657 pass their check, 0 worse, 0 better`.

Status by test, for the 5655 baseline tests:

| Status | Baseline | After |
|---|---|---|
| pass | 4910 | 4977 |
| differs from EDG only | 111 | 112 |
| FALSE-REJECT | 79 | 58 |
| RUN-WRONG-RESULT | 12 | 1 |
| RUN-UNEXPECTED-UB | 4 | 0 |
| MISSED-REJECT | 259 | 241 |
| unsupported | 144 | 141 |
| run-unsupported | 135 | 124 |

Changes by test:

| Change | Tests |
|---|---|
| false reject, now passes | 20 |
| missed reject, now passes | 18 |
| run unsupported, now passes | 11 |
| wrong result, now passes | 11 |
| unexpected undefined behavior, now passes | 4 |
| unsupported, now passes | 4 |
| false reject, now unsupported | 1 |

Two tests changed from "pass" to "differs from EDG only". Both count as passes. In both, the check now gives the base expectation and differs from the EDG override.

### Write-suite programs

| Program | Start | After |
|---|---|---|
| W12 `counter.cpp` | rejected, SA-1 | runs, result 0 |
| W7 `date.cpp` | rejected, DCMP-11 | runs, result 0 |
| W7 `rational.cpp` | rejected, CTOR-25 | runs, result 0 |
| W7 `money.cpp` | rejected, DCL-20 | unsupported (concept with a member template constructor) |
| W7 `small_string.cpp` | rejected, CE-13 | unsupported (CTAD-11 user guide) |
| W6 `default_members.cpp` | rejected, PRIM-25 | runs, result 0 |
| W10 `noexcept_functions.cpp` | rejected, SA-1 | runs, result 0 |
| W10 `cleanup_guard.cpp` | rejected, DED-35 | unsupported (CTAD from a lambda) |
| W2 `three_way.cpp` | rejected, CE-2 | runs, result 0 |

No W program gets a wrong answer now.

## 2. What changed in the front end

### Overload resolution and conversions

- OVL-21, Q-0161: `(&f)(a)` with an unqualified name and no parentheses resolves the set as a call by name with no implied object argument. OVL-44 now fires only for a parenthesized name.
- CVF-4: `conv_fns` keeps every conversion function of one class. Before, it kept one per name, so `operator int() const` and `operator int()` collapsed into one function.
- SW-2: a class switch condition converts by the chosen conversion function and is then promoted. Before, it was unsupported.
- ICS-4, ICS-45: an initialization of class `C` from a class whose conversion function returns `C` builds the call. `static_cast` uses `explicit` conversion functions when only a qualification adjustment is left.
- CV-10, Q-0191: the qualification conversion is checked level by level. An added cv-qualifier needs `const` above it, and a dropped array bound needs nothing.
- Copy assignment: a class that declares a copy assignment operator has no implicit one, so the declared operators are the whole candidate set. Before, the front end gave up on any class with constructors.
- Empty braces on a class with no default constructor and no initializer-list constructor are a CTOR-25 error. Constructor templates that need arguments drop out for empty braces. The empty-brace case now resolves to the default constructor.
- Constructors no longer hide the injected class name in an expression. `Q(5)` and `Q{5}` inside a member function of `Q` gave CTOR-25 or TR-12 before this change.
- DCMP-13: for a non-member `==`, the search for a matching `!=` skips friends declared only in a class. For a member `==`, member lookup also finds a base's `!=`.
- `<=>` on two operands of one scoped enumeration gives `strong_ordering` (CMP-12).

### Classes

- CPY-7, DTOR-3: a union's implicit copy, move, and destructor are deleted when a variant member's function is deleted. A call `p->~T()` of a deleted destructor is DEL-1.
- CPY-19: the special members are checked for eligibility. A copy or move function whose constraints fail is left out. At the end of a class definition, the front end evaluates the constraints of its constructors and assignment operators.
- `new T(x)` with a glvalue `x` of class `T` checks the copy or move constructor: CPY-23 and CPY-24 for a deleted one, ACC-3 for one that is not accessible.
- ACC-6: friends are tracked one by one. Each class has its own set of friend classes (by class and by name) and friend functions (by name). A friend template or an untracked form falls back to the old open behavior.
- ACC-2, ACC-4: member access checks the path to a base and the protected-object rule. The check skips a member that a using-declaration names (ACC-9). ACC-13: a using-declaration that names an inaccessible base member is an error.
- PRIM-11, CLS-2: a default member initializer has `this`, so `int area = width * height;` works. Before, it gave PRIM-25.
- CLS-31, ATTR-9: in `sizeof` and the byte layout, an empty `[[no_unique_address]]` member takes no bytes. Two such members of one type get different offsets. The next member can use the tail padding of a member whose class meets the CLS-31 condition. Bases are laid out in the byte layout, with empty bases at offset 0.
- LAM-18: an explicit copy capture is `const` in a lambda that is not `mutable`. Before, only `[=]` made it `const`.
- DCL-20: an array of a class template specialization instantiates the class.
- CINIT-1, CE-24: a `constinit` scalar or array of scalars needs no initializer.
- STMT-8: two block-scope `extern` declarations of one variable are accepted.
- CLS-9 no longer compares functions that differ in `...` or are distinct templates.

### Member pointers

- MPTR-8: a pointer to a member of type `void`.
- MPTR-9: a pointer to a bit-field, or to a member that is not accessible (with ACC-2 or ACC-4).
- MPTR-10: an object of an unrelated class, a call that drops a cv-qualifier, and a ref-qualifier that does not fit the object's category. `const&` is allowed on an rvalue.

### Exceptions and the interpreter

- THR-1: a local is moved only when each try block around the `throw` also contains its declaration. Each function context records the declaration count at each try block. The parser marks a moved operand as an xvalue, so the check side (THR-3) and the interpreter agree.
- TRY-2: handlers match pointers to members by qualification conversion and from `nullptr`. A pointer to a `noexcept` function matches a handler without `noexcept`. Multi-level qualification conversions are checked.
- DTOR-4: in the interpreter, a destructor without a specifier lets an exception out when a member's destructor is `noexcept(false)`.
- UNI-10, UNI-15, UNI-16: the common initial sequence takes members from a single base (CLS-19), and pairs enumerations with one underlying type and nested layout-compatible structs. A read through the other member is exempt from the STG-23 type check (finding G104).
- NEW-24, DED-39, Q-0220: a function and a matching template are two matches, so no deallocation function runs. Two or more templates alone stay unsupported, because ORD-9 is not modeled there.
- STG-8, G101: when a later use replaces an implicit object set, pointers from earlier uses are moved to the object of the new set at the same byte with the same type.
- IF-25, CE-3, SINIT-1: a `const` non-volatile integer or enumeration local, and a static scalar, are initialized by constant evaluation when the initializer is a constant expression. `if consteval` then takes the constant branch.
- PFX-19: a call of a `noexcept` function through a pointer without `noexcept` is valid. This follows the page text after G96.
- Two pointers into one object compare by byte position at run time (CLS-19, CLS-30). Empty bases and the first member then share the object's address.
- A copy constructor is chosen by the source's cv-qualification. `T&` wins over `const T&` for a non-const lvalue (ICS-30). Before, the first constructor found was used.
- Value-initialization of a class with no user-declared constructor that is not an aggregate (`O()`, `O{}`) now runs. An example is a class with a polymorphic base.
- Defaulted comparisons run in constant evaluation (DCMP-1, DCMP-4).
- Unsigned multiplication wraps exactly for 64-bit operands (MUL-8).
- A variable whose type the front end did not work out (for example after a failed CTAD) is unsupported in evaluation. Before, its initializer was evaluated, which gave a false CE-13.

### Other

- EXS-4: the `noexcept` operator counts the copy or move constructor when a class prvalue is made from a glvalue, as in `T(lvalue)`.
- ALN-8: an `alignas` operand of class type converts by its conversion function.
- FN-12 does not apply during signature-only substitution. Deduction with a return type like `Box<F>`, where `F` is a closure, works now.
- DEP-24: inside a template, `<` after a member of an object of unknown type is the less-than operator unless `template` comes before the name.
- LIST-7: brace elision applies only to array or aggregate elements whose value is not an object of the element's class.

## 3. Items fixed (priority 1)

From the named list, these now pass:

- `exceptions/throw/local-outside-try-copied.cpp`
- `exceptions/throw/local-outside-inner-try-copied.cpp`
- `exceptions/throw/parameter-in-try-copied.cpp`
- `exceptions/handlers/match-member-pointer-qualification.cpp`
- `exceptions/handlers/match-noexcept-function-pointer.cpp`
- `exceptions/handlers/match-nullptr-to-member-pointer.cpp`
- `exceptions/unwinding/leaves-destructor-member-throwing-caught.cpp`
- `expressions/new-delete/throw-placement-function-and-template.cpp`
- `expressions/new-delete/throw-placement-ellipsis-no-match.cpp`
- the three `classes/unions/common-initial-sequence-*.cpp` tests
- `classes/copy-move/trivially-copyable-at-least-one.cpp`
- `classes/copy-move/trivially-copyable-constraint-fails.cpp`
- `classes/copy-move/trivially-copyable-deleted-destructor.cpp`
- `classes/copy-move/trivially-copyable-private-copy.cpp`
- `classes/copy-move/trivially-copyable-every-eligible.cpp`
- `classes/copy-move/self-pointer-copy.cpp`
- `classes/members/layout-overlap-member.cpp` and `-ilp32`
- `classes/members/standard-layout-bases-same-address.cpp`
- `classes/unions/union-member-deleted-copy.cpp`
- `foundations/storage/implicit-object-first-use-member.cpp`
- `statements/if/consteval-local-initializers.cpp`
- `statements/switch/condition-two-conversions-same-type.cpp`
- `classes/comparisons/eq-constexpr.cpp`, `spaceship-constexpr.cpp`, `class-template-defaulted.cpp`, and `ne-not-matching-friend.cpp`

These also pass now:

- `foundations/language-support/ordering-complete-declaration.cpp`
- `foundations/overload-resolution/call-through-address-unqualified-argument.cpp`
- `declarations/declarators/qualification-conversion-array.cpp` and `-inner-bound`
- `declarations/alignment/operand-class-converts.cpp`
- `expressions/constant-expressions/constant-initializer-zero-scalar.cpp`
- `expressions/multiplicative/unsigned-wraps-wide.cpp`
- `statements/blocks/redeclare-same-entity-ok.cpp`

In all, 35 of the 95 baseline FALSE-REJECT, RUN-WRONG-RESULT, and RUN-UNEXPECTED-UB tests pass. The others are listed in section 4.

The 18 missed rejects that pass now:

- seven `classes/access` tests,
- seven `expressions/member-pointers` tests,
- `classes/comparisons/ne-matching-base-member.cpp`,
- `classes/unions/union-member-deleted-copy.cpp`,
- `declarations/declarators/pointer-to-void-member.cpp`,
- `expressions/lambdas/const-call-operator.cpp`.

## 4. Items left, with reasons

- `foundations/overload-resolution/call-through-address-unqualified.cpp`: still a false reject, now OVL-36 (ambiguous) in place of OVL-44. By OVL-23 and OVL-25 the call `(&f)()` is a tie between `f() &` and `static f(int = 0)` (finding G103).
- `classes/derived/inherited-ctor-access-in-friend.cpp`: run-unsupported. A class with `using B::B;` is still opaque, because inherited constructors are not modeled.
- `declarations/initialization/references/dangling-new-member-warns-default.cpp`: missed reject. The REF-15 warning for `new Holder{1}` is not modeled.
- `declarations/exception-specifications/constructor-ignores-member-destructors.cpp`: a placement `new` with a class `operator new` stays opaque to the `noexcept` operator.
- Long double (`expressions/multiplicative/ieee-types.cpp`, `foundations/types/floating-range*.cpp`, `floating-divide-by-zero-infinity.cpp`): the evaluator uses `f64`, as G98 records.
- Constant-evaluation pointer comparisons across members (CMP-7, CMP-26): `one-past-member-adjacent-constant.cpp` and `pointer-array-member-order.cpp`.
- `moderate-call-count.cpp`, `moderate-loop.cpp`, and `moderate-loops-split.cpp`: the evaluator's step limit.
- `static-local-usable-reached.cpp`: CE-20 for a usable static local.
- `allocator-self-declared-constant.cpp` and `allocation-array-class-elements-constant.cpp`: allocation in constant evaluation.
- `classes/unions/bitfield-unnamed-*.cpp`: unnamed bit-fields in copies and structured bindings.
- `char-named-escape.cpp`: needs a Unicode name table.
- `list-string-literal-binds-literal.cpp`: ICS-38.
- `tie-conversion-result-constructor.cpp`: OVL-26 between a constructor and a conversion function.
- `disambiguation-declarator-forms.cpp`, `operand-used-twice-evaluated-once.cpp`, and `overloaded-member-pointer-order.cpp`: not reached in this round.
- Templates (36 tests): packs (alignment specifiers, using-declarations, init-capture packs, a lambda in a pattern, mem-initializers), member templates of explicit and partial specializations, CTAD sites, the template keyword forms, and dependent `noexcept`. Not reached in this round. `sites-for-and-condition.cpp` moved from a false reject to unsupported.
- W7 `money.cpp`: a concept over a class with a deleted converting constructor template. W7 `small_string.cpp`: a CTAD-11 user guide. W10 `cleanup_guard.cpp`: CTAD from a lambda argument. All three are unsupported, not wrong.
- Missed rejects: 241 remain. The largest groups are `templates/dependent-names` (20), `expressions/lambdas` (12), `templates/packs` (10), and `classes/derived` (10).

## 5. Findings

### G103. `(&f)()` with `f() &` and `static f(int = 0)` is a tie by the text

- Page and rules: foundations/overload-resolution, OVL-21, OVL-23, OVL-25, OVL-44; Q-0161.
- Kind: contradictory.
- What happened: `call-through-address-unqualified.cpp` expects `(&f)()` to call the static member, and its note says the static member wins. OVL-21 resolves the call with no implied object argument. OVL-23 then says the object parameter of a member takes no part. OVL-25 says parameters with default arguments take no part. So neither candidate has a sequence to compare. No tie-breaker of OVL-26 to OVL-33 applies, because both are plain functions. OVL-36 then makes the call ambiguous, which is what Clang 18 reports. The companion test with an argument, `(&f)(1)`, is decided by the parameter count and passes.
- Suggestion: either state the tie-breaker that prefers the static member in a call with no object, or change the test's expectation to a rejection under OVL-36, with overrides for Clang 22 and EDG.

### G104. STG-23 and a read through the common initial sequence

- Page and rules: foundations/storage, STG-23, STG-14; classes/unions, UNI-10, UNI-15, UNI-16.
- Kind: contradictory.
- What happened: UNI-10 lets a program read a member of the inactive struct when the member lies in the common initial sequence. UNI-16 lets the pair be two different enumerations with one underlying type, or two different layout-compatible structs. STG-23 makes a read through a glvalue of a type that STG-14 does not list undefined, and it names "a class with the same members" as such a type. So `common-initial-sequence-enum.cpp` reads a `Kind` object through a `Shape` glvalue. `common-initial-sequence-nested.cpp` reads a `Point` through a `Pair`. By the text of STG-23, both reads are undefined. The interpreter treats UNI-10 as an exception to STG-23 for the places it remaps.
- Suggestion: add to STG-23, or to UNI-10, that a read that UNI-10 allows is not covered by STG-23.

### G105. DTOR-3 and the variant members of a union

- Page and rules: classes/constructors, DTOR-3; classes/unions, UNI-3; classes/copy-move, CPY-7.
- Kind: ambiguous.
- What happened: DTOR-3 deletes the implicit destructor "if a base or member has a destructor that is deleted or not accessible". UNI-3 covers variant members whose function is non-trivial. A deleted destructor is trivial by DTOR-9, so UNI-3 does not apply to `union U { Pinned x; }` with `~Pinned() = delete`. `trivially-copyable-deleted-destructor.cpp` expects the union's destructor to be deleted. Its copy and move constructors are then deleted too (CPY-7). The text supports this only if a variant member counts as a "member" for DTOR-3. The front end now counts it.
- Suggestion: say in DTOR-3 that a variant member of a union counts.

### G106. IF-25 and a static local of scalar type

- Page and rules: statements/if, IF-25; SINIT-1; CE-3.
- Kind: missing.
- What happened: IF-25 lists a constant-expression initializer of a static variable as a constant evaluation. `consteval-local-initializers.cpp` relies on this for `static int shared = where();`. The rule does not say what happens when the initializer is not a constant expression and `if consteval` takes the run-time branch. An interpreter has to try the constant evaluation first, then fall back to a run-time evaluation. The front end does that for scalars only.
- Suggestion: add an example to IF-25 or SINIT-1 that shows the fallback, and say whether a class-type static with a constant initializer is in scope.

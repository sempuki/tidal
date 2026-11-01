# X5: P9, overload sets as values

Date: 2026-11-01. Decision 0037. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md and decision 0037.
- The Propose sections of `documentation/exercises.md`.
- P9 and section 2 of `reports/clean-room/roadmap/designer.md`, and item 1 of `reports/clean-room/roadmap-2/designer.md`.
- `reports/clean-room/X4/report.md`, as the model for this report's form.
- These specification pages, in full or in the cited rules: overload-resolution, conversion-sequences, names, auto, decltype, lambdas, deduction, ordering (principles), dependent-names, member-pointers.
- The cited rules of these pages: conversions (CONV-8, CONV-13), values (VC-2), postfix (PFX-P3, PFX-5), constexpr (CEV-1, CEV-2), odr (ODR-3, ODR-10), access (ACC-3), return-deduction (RET-1, RET-2), instantiation (INST-31), packs (PACK-23), templates/declarations (TPL-11, TPL-46) and templates/arguments (TARG-12).
- The tests named below, and a scan of all tests for overloaded names used without a call.

Compiler runs: small programs built with g++ and clang++ (`-std=c++23 -pedantic-errors`) under /tmp/claude-1000/x5, to see what programs do today. The results are quoted where they support a claim.

Totals: 41 rule changes (10 new, 16 amended, 15 unchanged but must state their effect). Two existing tests change their expected outcome. 20 findings, X5-1 to X5-20.

## 1. Design

### 1.1 The feature

Today the name of an overload set has a meaning only where a target type picks one function (OVL-18). Without a target, the name is not valid (OVL-20, AUTO-8, DTY-4). So a program cannot pass an overloaded function to generic code:

```cpp
int scale(int);
double scale(double);
template <class F> void each(F f);
each(scale);          // rejected today: F gets no value (DED-19, DED-23)
auto s = scale;       // rejected today (AUTO-8)
```

P9 gives the name a value in exactly these positions. The value is an object of a new class type, the *set closure type*. A call of the object runs overload resolution over the set with the call's arguments.

```cpp
auto s = scale;       // P9: s holds a set closure
s(1);                 // calls scale(int)
s(1.0);               // calls scale(double)
each(scale);          // P9: F is the set closure type
```

### 1.2 Where a name becomes a set closure (new rule OVL-45)

A name denotes an overload set if lookup finds two or more functions, or at least one function template. P9 applies to such a name when all these conditions hold:

- The name is written without `&`. It can be unqualified, qualified, in parentheses, or followed by explicit template arguments.
- No target type exists (OVL-18), or the target is a template parameter that deduction leaves without a value (1.6).
- The steps of OVL-20 do not leave exactly one function.
- Every function in the set is a non-member function or a static member function, or a template of one of these.

The name is then a prvalue of the set closure type of that set. Positions where this arises:
- the initializer of a variable declared with `auto`, `auto&&`, `const auto&`, or `decltype(auto)`,
- the operand of `decltype` and `sizeof`,
- an init capture `[g = f]`,
- a `return` in a function with a deduced return type,
- the argument for a constant template parameter declared with `auto` (TPL-11),
- an argument for a function template parameter under 1.6,
- an expression statement and the operand of a cast to `void`.

These cases keep today's meaning:
- A name with a target type selects one function (OVL-18, OVL-19).
- A name for which OVL-20 leaves one function gives that function. A single plain function still decays to a pointer under `auto` (AUTO-2).
- `&f` with no target stays rejected (OVL-20). `&` asks for an address, and a set has no single address.
- A call `f(a)` is a call, and P9 does not touch it.

These cases stay rejected (new rule OVL-50):
- a set that holds a non-static member function, with an implicit or an explicit object parameter,
- a member access `x.f` that names a member function (VC-2),
- `auto&` with a set name, because the closure is a prvalue (AUTO-18).

### 1.3 The set closure type (new rule OVL-46)

- It is a class with no name and no data members. It is a literal type, so a constant evaluation can make and copy it (CE-23). It is structural, so it can be a constant template argument.
- It can be default-constructed, copied and assigned, as a lambda without captures can (LAM-11).
- Its identity comes from the set. Two names give the same type if they find the same functions by ordinary lookup, carry the same explicit template arguments, and agree on whether argument-dependent lookup applies (1.5). This holds in every translation unit. So `decltype(scale)` names one type wherever the same declarations are visible. A declaration of a new overload between two uses gives the later use a different type.
- It has no `==`, and it has no members that a program can name except the call operator and the conversion of 1.4.

### 1.4 The call and the conversion (new rules OVL-47, OVL-48)

The closure type has a static call operator that is a template. It takes any number of arguments, each as a forwarding reference (DED-8). A call does the following:

1. It forwards each argument with its value category (PACK-23).
2. It resolves a call to the set with those arguments. The candidates are the functions that ordinary lookup found at the name. If the name allowed argument-dependent lookup (1.5), the call adds the functions that lookup finds for the call's argument types.
3. The result has the type and value category of that call, so a function that returns a reference gives a reference.
4. The call operator is `noexcept` if the chosen call is. It is `constexpr` if the chosen function is. If the chosen function is `consteval`, the call operator takes on `consteval` as a template specialization does (CEV-1).

A call that has no viable function, an ambiguous call, a deleted function and an access fault all fail in the call operator's function type. So they are failures of substitution (DED-24), and a requires expression or a constrained template can ask whether a closure accepts some arguments.

This model compiles today as a hand-written class with a static call operator template and a trailing `decltype` return type. Both compilers accept it as a template argument for `template <auto V>`. Both report `false` for a concept that calls it with an argument that no overload takes.

The closure type also has a conversion function template to a pointer to function. For the target type `R (*)(A...)`, it selects a function of the set as OVL-18 and OVL-19 do with that target. If no single function matches, the conversion does not exist. The conversion is a template, so it gives no surrogate candidate (OVL-6), and it serves a user-defined sequence only with an exact match (ICS-5). This follows what both compilers do for a generic lambda without captures: `void (*p)(int) = [](auto x) {};` builds on both.

The forwarding model makes a closure call differ from a direct call in four ways. The proposal accepts these differences and states them:
- **A null pointer constant.** A literal `0` arrives as an `int`, so it no longer converts to a pointer (CONV-8). With `f(int*)` and `f(double)`, the direct call `f(0)` is ambiguous on both compilers, and the forwarding call chooses `f(double)`.
- **A braced list.** It cannot be forwarded (DED-18). Both compilers reject the forwarding call with `{}`.
- **A prvalue argument.** It becomes a temporary and then initializes a by-value parameter by a move. With a deleted move constructor, the direct call builds and the forwarding call fails on both compilers.
- **A bit-field.** It cannot bind a forwarding reference that is not `const` (REF-8).

### 1.5 Lookup (new rules NAM-37 and DEP-39; amended NAM-27 and DEP-2)

- **Ordinary lookup** runs once, at the name. It fixes the candidate set, and the closure type records it (1.3). A later declaration of the same name does not join the set. This matches the binding of an independent name in a template (DEP-5).
- **Argument-dependent lookup** runs at each call of the closure, with that call's argument types. It applies under the conditions that would allow it for a call written at the name (NAM-31, NAM-32):
  - A qualified name or a name in parentheses gives a closure without it.
  - A set found in a block, or a set that ordinary lookup finds as class members, gives a closure without it.
- The call operator is a template, so its call is a dependent call. Argument-dependent lookup sees the declarations before the name and those before the point of instantiation of the call operator specialization (DEP-9, INST-31). Today a generic lambda that wraps a call behaves this way. A call through it finds a function in an argument's namespace that is declared after the lambda, on both compilers. DEP-38 covers a better function in another translation unit.
- **Associated namespaces of the closure type.** A closure passed to an unqualified call adds what the set's functions add. This is the row "a function name with overloads" of NAM-27. So `h(scale)` finds the same functions by argument-dependent lookup as before, whether or not the argument becomes a closure.
- **In a template.** A name whose set depends on a template parameter is type-dependent, for example `T::fn` or `f<T>`. It becomes a closure at the instantiation. A name that does not depend on a parameter becomes a closure at the definition.
- **Access** to the functions of the set is checked from the place of the name. A class can therefore hand out a closure over its private static overloads. The check happens when a call of the closure chooses a function, from the context of the name.

### 1.6 Deduction (new rule DED-40; amended DED-19, DED-23)

Today an argument that names a set gives no value to a parameter whose type is a plain template parameter (DED-19). If no other pair gives `T` a value, deduction fails (DED-23). P9 adds a last step to DED-23:

- A template parameter `T` can have no value from a pair, an explicit argument, or a default. If some pair matched an overload set argument with a parameter whose type is `T` after the adjustments of DED-6 and DED-7, then `T` takes the set closure type of that set. A forwarding reference `T&&` gets the closure type, because the argument is a prvalue.
- The step does not apply to a parameter that is not a plain `T`, such as `T (*)(int)`. So `templates/deduction/overload-set-template.cpp` and `overload-set-two-match.cpp` keep their outcome.
- Defaults come first. Today `template <class F = void (*)(int)> int g(F); g(f)` builds on both compilers with `F` from the default. P9 keeps that.
- Two arguments that name different sets give `T` two values, so deduction fails (DED-21). The same set twice gives one type.
- Each element of a function parameter pack gets its own closure (DED-20).
- The same step serves a generic lambda's call operator (LAM-15), a constrained `auto` parameter (DED-34), and a constructor template inside a user-defined conversion (ICS-4).

### 1.7 Overload resolution in two passes (new rule OVL-49)

The step of 1.6 can make a function template viable that today drops out. That template can then beat the function that wins today. This program builds today and returns 2 on both compilers:

```cpp
void f(int) {} void f(double) {}
template <class F> constexpr int pick(F, int) { return 1; }
constexpr int pick(void (*)(double), long) { return 2; }
static_assert(pick(f, 0) == 2);
```

With the closure, the template ties on the first argument and wins on the second, so the result would become 1. To keep such programs, resolution runs in two passes:

1. Resolution runs with today's rules, and DED-40 is off.
2. Only if the first pass finds no viable function, resolution runs again with DED-40 on.

The two passes apply to every overload resolution, including the resolution of a constructor or a conversion function inside a user-defined sequence. The outer resolution sees only the result. In the second pass, the sequence from an overload set argument to a parameter of the closure type is the identity, with exact-match rank (new rule ICS-47).

### 1.8 Alternatives considered

- **A new closure type for each use**, as each lambda expression has its own type (LAM-2). This is simpler to state. But `decltype(f)` would differ between two uses. An inline function that makes a closure would also have a different type in each translation unit, unless a rule like TPL-46 is added. Rejected.
- **A closure for every set name, also when OVL-20 leaves one function.** This removes the rule that the result depends on how many functions remain. It changes `auto g = f<int>;` and the constrained sets of Q-0142 from a pointer to a closure in valid programs. Rejected.
- **Rank the closure step as a conversion** and drop the two passes. Then `pick(f, 0)` above has crossing sequences and becomes ambiguous, so a valid program becomes invalid. Rejected.
- **Closures over non-static members**, which take the object as the first argument or capture `this`. This is useful, but it needs a decision on the object's value category and on the capture of `this`. Left for a later proposal. OVL-50 keeps the space free.
- **An explicit marker**, such as a keyword before the name. No program changes at all, including the detection programs of 4.3. This is the fallback if the impact in section 4 is judged too large.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. Classes are the specification's rule classes.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| OVL-45 | new | none | required | A name of an overload set with no target, where OVL-20 leaves no single function, is a prvalue of the set closure type (1.2). | none |
| OVL-46 | new | none | required | The set closure type is empty, literal, structural, default-constructible and assignable. Its identity comes from the set, the explicit template arguments and the lookup flag (1.3). | none |
| OVL-47 | new | none | required | The static call operator template states forwarding, lookup, the result type, `noexcept`, `constexpr`, and failures in the function type (1.4, 1.5). | none |
| OVL-48 | new | none | required | The conversion function template to a pointer to function, which selects by OVL-18 and OVL-19 (1.4). | none |
| OVL-49 | new | none | required | Resolution runs in two passes, and DED-40 applies only in the second (1.7). | none |
| OVL-50 | new | none | reject | A set name with a non-static member function, or a member access `x.f`, has no value without a target (1.2). | none |
| DED-40 | new | none | required | A template parameter with no value takes the set closure type of an overload set argument whose parameter is a plain `T` (1.6). | none |
| ICS-47 | new | none | required | Gives the sequence of an argument that names an overload set: the identity for a parameter of the set closure type. It also states today's case, a function-to-pointer step for a pointer-to-function parameter (X5-1). | none |
| NAM-37 | new | none | required | A call of a set closure uses argument-dependent lookup only if the name allowed it (NAM-31, NAM-32) and ordinary lookup found no block or class member declaration. | none |
| DEP-39 | new | none | required | The call of a set closure is a dependent call. Its argument-dependent lookup sees declarations before the name and before the point of instantiation of the call operator. | none |
| OVL-P2 | amended | principle | principle | The context table gains a row for a set name with no target type, citing OVL-45 to OVL-48. | none |
| OVL-18 | amended | required | required | Says that a placeholder type gives no target, and that a function template parameter gives none while deduction has no value. Points to OVL-45. | none |
| OVL-20 | amended | reject | reject | Without a target, the rule rejects only `&f` and a set that OVL-50 excludes. Otherwise OVL-45 gives a closure. | `declarations/auto/overloaded-function.cpp` (reject to accept) |
| OVL-35 | amended | reject | reject | No viable function is reported only after the second pass of OVL-49. | none |
| OVL-39 | amended | required | required | The pairing of a set with each candidate is the first pass. DED-40 adds the closure pairing in the second pass. | none |
| AUTO-P4 | amended | principle | principle | "No overloaded name" becomes "an overloaded name gives its set closure (OVL-45)". | none |
| AUTO-8 | amended | reject | reject | The item "not have the name of an overloaded function" is removed. The other items stay. | `declarations/auto/overloaded-function.cpp` (reject to accept) |
| AUTO-11 | amended | reject | reject | The item "a function name" covers only a name that gives one function. A set name gives a closure prvalue, so `decltype(auto)` deduces the closure type. | none |
| DTY-P1 | amended | principle | principle | An overloaded function name is a prvalue of its set closure type, so the prvalue row of the table applies. | none |
| DTY-1 | amended | required | required | Excludes the name of an overload set from "a name without parentheses". | none |
| DTY-4 | amended | reject | withdrawn | The prohibition goes away. A sentence points to OVL-45 and DTY-2. | `expressions/decltype/overloaded-name.cpp` (reject to accept) |
| DED-19 | amended | required | required | After "otherwise the pair gives no value", adds that DED-40 can still give the closure type in the second pass. | none |
| DED-23 | amended | required | required | Applies DED-40 before "any other template parameter without a value makes deduction fail". | none |
| NAM-27 | amended | required | required | Adds a row for the set closure type, which adds what the type of each function of its set adds. Also says what a function template in the set adds (X5-15). | none |
| ODR-3 | amended | required | required | Making a set closure uses no function. A call of the closure uses the function chosen. The same holds for a set with a target type (X5-8). | none |
| DEP-2 | amended | required | required | A name of an overload set is type-dependent if its lookup or its explicit template arguments depend on a template parameter. | none |
| AUTO-2 | unchanged, state effect | required | required | A single function still gives a pointer. Only a set gives a closure. | none |
| AUTO-18 | unchanged, state effect | reject | reject | `auto& g = f;` stays rejected because the closure is a prvalue. | none |
| RET-1 | unchanged, state effect | required | required | A deduced return type from a set name is the closure type, through the variable rules. | none |
| OVL-25 | unchanged, state effect | required | required | The closure pairing ranks as the identity (ICS-47) inside the second pass. | none |
| OVL-27 | unchanged, state effect | required | required | A template made viable by DED-40 still loses a tie to a plain function in the second pass. | none |
| OVL-37 | unchanged, state effect | reject | reject | At a closure call, the checks on the chosen function are failures in the call operator's function type (OVL-47). So they reach detection as substitution failures. | none |
| ACC-3 | unchanged, state effect | reject | reject | Access for a closure call is checked from the context of the name (X5-7). | none |
| ICS-5 | unchanged, state effect | required | required | The conversion of OVL-48 serves a user-defined sequence only with an exact match. | none |
| LAM-7 | unchanged, state effect | required | required | An init capture of a set name holds a set closure. | none |
| LAM-15 | unchanged, state effect | required | required | A generic lambda called with a set name gets the closure through DED-40. | none |
| DEP-9 | unchanged, state effect | required | required | DEP-39 applies DEP-9 to the closure's call. | none |
| DEP-38 | unchanged, state effect | no-diagnostic | no-diagnostic | Covers a closure call whose argument-dependent lookup in another unit would find a better function. | none |
| ORD-5 | unchanged, state effect | required | required | A closure call orders the templates of the set as a direct call does. ORD-9 orders them for the conversion of OVL-48. | none |
| MPTR-9 | unchanged, state effect | reject | reject | `C::f` without `&` stays rejected for a set with a non-static member (OVL-50). For a set of static members, `C::f` gives a closure (X5-13). | none |
| CEV-2 | unchanged, state effect | reject | reject | The conversion of OVL-48 to a pointer to a `consteval` function counts as taking its address (X5-12). | none |

Counts: 41 changes. 10 new, 16 amended (one of them, DTY-4, withdrawn), 15 unchanged but must state their effect. 0 split. Two existing tests change outcome, both from reject to accept.

A scan of all tests looked for an overloaded name or a function template name in these places: after `=`, inside `decltype(...)` or `sizeof(...)`, as an argument, or in an init capture. Every match outside the two tests above has a target type (OVL-18), uses `&`, or is a call. The tests nearest the boundary keep their outcome:
- `foundations/overload-resolution/address-ambiguous.cpp` and `address-no-target-template.cpp`: they use `&`, so they are still rejected.
- `foundations/overload-resolution/address-no-target-single.cpp`, `address-no-target-constraints.cpp`, `address-no-target-more-constrained.cpp`, `address-no-target-explicit-arguments.cpp`: OVL-20 leaves one function, so the result is unchanged.
- `templates/deduction/overload-set-template.cpp` and `overload-set-two-match.cpp`: the parameter is a function pointer type, not a plain `T`, so DED-40 does not apply.
- `templates/deduction/overload-set-later.cpp`, `overload-set-one-match.cpp`, and the three `overload-set-argument*.cpp` tests: they succeed in the first pass.
- `expressions/member-pointers/form-function-without-amp.cpp` and `address-of-bound-member.cpp`: non-static members, still rejected.
- `declarations/auto/decltype-auto-function.cpp`: a single function, still rejected by AUTO-11.

The two changed tests need new content. A reject test for the remaining AUTO-8 and DTY-4 cases could use a set with a non-static member, under OVL-50.

The proposal needs these new tests, as paths under `tests/foundations/overload-resolution/`:
- `set-value-auto.cpp` (accept; one closure calls two overloads),
- `set-value-decltype-same-type.cpp` (accept; two uses give one type, and a new overload in between gives another),
- `set-value-generic-argument.cpp` (accept; `template <class F>` takes a set),
- `set-value-first-pass-wins.cpp` (accept; the `pick(f, 0)` program of 1.7 still gives 2),
- `set-value-detection.cpp` (accept; a constrained template sees which arguments the closure takes),
- `set-value-adl-later.cpp` (run; the call finds a function declared after the name, in an argument's namespace),
- `set-value-parenthesized-no-adl.cpp` (reject; `(f)` gives a closure without argument-dependent lookup),
- `set-value-nonstatic-member.cpp` (reject; OVL-50),
- `set-value-to-pointer.cpp` (accept; OVL-48),
- `set-value-forwarding-null.cpp` (accept; the closure call with `0` chooses `f(double)` where the direct call is ambiguous),
- `set-value-template-argument.cpp` (accept; `template <auto V>`),
- `set-value-noexcept.cpp` (accept; `noexcept` follows the chosen function),
- `set-value-explicit-arguments.cpp` (accept; `f<int>` with a second template parameter left to each call),
- `set-value-constant-evaluation.cpp` (accept; a closure made and called in a constant evaluation).

Two more go elsewhere: `tests/templates/deduction/overload-set-closure.cpp` and `overload-set-default-first.cpp` (DED-40, and its order against defaults).

## 3. Interactions

**Lookup (NAM-5, NAM-27, NAM-29 to NAM-32, NAM-8).** Ordinary lookup at the name must find at least one function, or the name is not valid (NAM-8). P9 does not apply to a name that only argument-dependent lookup would find, because no set exists to wrap. The rules that turn off argument-dependent lookup for a call also turn it off for the closure. So `(f)` and `N::f` give closures that see only their set. NAM-27 must say what a closure adds as an argument, and the proposal reuses the row for a function name with overloads.

**Overload resolution (OVL-1, OVL-18 to OVL-20, OVL-25 to OVL-39).** The closure call is an ordinary call to the set with forwarded arguments, so every tie-breaker applies. Q-0142 matters more under P9. Where a compiler does not reduce a constrained set (OVL-20 Evidence: GCC 16 reduces for `auto` only), today's difference is accept against reject. Under P9 it becomes a pointer against a closure, which is a silent difference in type. The proposal should ask that Q-0142 be settled first.

**Conversion sequences (ICS-1 to ICS-11, ICS-47).** Tidal gives no sequence for an overload set argument today (X5-1). P9 needs one for its own pairing and for the existing OVL-39 pairing, to state the two passes.

**Deduction (DED-6, DED-7, DED-8, DED-19 to DED-24, DED-34).**
- DED-40 comes after defaults and after every pair, so no deduction that succeeds today changes.
- Failures in the closure call operator's return type are failures of substitution. A template that calls a closure in its function type drops out as it would for any other call (DED-24, DED-29).
- The `noexcept` operand of the call operator is substituted after the choice (DED-27), and an error there makes the program not valid. P9 avoids that error, because a call that fails also fails in the return type, before `noexcept` is substituted.

**Partial ordering (ORD-5, ORD-9).** A closure call orders the templates in its set as a direct call does. The conversion of OVL-48 orders them by function type, as the address of a set does.

**Templates and dependent names (DEP-2, DEP-5, DEP-9, DEP-38, INST-31).** A set name that does not depend on a parameter is bound at the definition. The call operator's argument-dependent lookup follows the two phases. So the result of a closure call can differ between translation units in the same way as any dependent call (DEP-38). A dependent set name such as `T::fn` becomes a closure at each instantiation.

**Placeholder types (AUTO-1 to AUTO-22, RET-1, TPL-11).** Each placeholder deduces from a prvalue of class type:
- `auto`, `auto&&` and `const auto&` all work,
- `auto&` is rejected (AUTO-18),
- `decltype(auto)` gives the closure type,
- a constant template parameter declared with `auto` takes the closure as a structural value.

**`decltype` and unevaluated operands (DTY-1, DTY-2, UNEV-1, UNEV-3, UNEV-5).** `decltype(f)` and `decltype((f))` both give the closure type, because the name is a prvalue. `sizeof(f)` gives the size of an empty class. An expression statement `f;` makes and discards a closure. UNEV-3 then recommends a warning, since the statement has no effect.

**Lambdas (LAM-2, LAM-7, LAM-10, LAM-11, LAM-13, LAM-15).**
- The set closure is not a lambda, and LAM-2 does not apply to it. It behaves like a lambda without captures that has a static call operator template (LAM-13, LAM-15).
- The conversion of OVL-48 mirrors the conversion of a generic lambda to a pointer to function, which Tidal does not state (X5-10).
- A generic lambda called with a set name gets the closure through DED-40. This is the main use: composing functions from overloaded names.

**Pointers to members (MPTR-2, MPTR-9, OVL-44).** Sets with non-static members stay out (OVL-50), so `&C::f` and `C::f` keep today's meaning. A set of static members named `C::f` gets a closure. MPTR-9's wording reads as if `C::f` without `&` is always rejected (X5-13).

**Constant evaluation (CE-23, LAM-12, CEV-1, CEV-2).**
- The closure is a literal type with no state. A constant evaluation can make, copy and call it if the chosen function is `constexpr`.
- A `consteval` function in the set makes the call operator specialization take on `consteval`, as CEV-1 does for a specialization of a template.
- The conversion to a pointer to a `consteval` function counts as taking its address (CEV-2).

**Access and deletion (ACC-3, DEL-1, OVL-37, UNEV-5).** The closure checks access from the context of the name (1.5). A deleted function chosen at a call makes the call fail in the function type, so detection sees it as not callable.

**The one-definition rule (ODR-3, ODR-7, ODR-10).** An inline function that makes a closure has the same closure type in every unit, because the type depends only on the set (OVL-46). Making the closure uses no function. Each call uses the function it chooses.

**Not touched.** These keep today's rules: calls by name, operator expressions, initialization by constructors, and every use with a target type. P9 adds a meaning only where a name of a set has none today.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- For a set `f`, these forms become valid: `auto g = f;`, `decltype(f)`, `sizeof(f)`, `[g = f]`, `return f;` in a function with a deduced return type, `f;`, and `(void)f;`. Both compilers reject each of them today.
- A call that passes a set to a function template parameter of type `T`, `T&&` or `const T&`. Both compilers reject `apply(f)` today.
- A call whose only candidate needs DED-40, even with a fallback candidate `probe(...)`. Both compilers reject `probe(f)` today, because a set argument gives no sequence for `...` either (X5-2).
- A class template whose constructor template takes any callable, initialized from a set, when no other constructor fits.

### 4.2 Undefined becomes defined

None. P9 touches no `undefined` rule.

### 4.3 Valid programs that change meaning

The two passes (1.7) keep every call that resolves today. One class of programs still changes. These programs ask whether a call with a set argument is valid, and today they get "no":
- an expression in a function type, such as `decltype(take(T::fn), true)` in a return type, where today the template drops out and an overload with `...` wins,
- a requires expression or a concept that tests such a call.

Both compilers build these probes today with the answer `false`. Under P9 the answer becomes `true`. A `static_assert(!Passable<S>)` then fails, and code that branches on the answer takes the other branch. The pattern is probably rare: today a set can never be passed, so a program has little reason to ask. This judgment is general experience (X5-18).

A second, smaller class is code that relies on a later overload making `auto g = f;` fail to build, as a guard. Under P9 the declaration builds and `g` changes type from a pointer to a closure.

### 4.4 Valid programs that become invalid

None by Tidal's text, given the two passes. Without them, `pick(f, 0)` in 1.7 would change its result, and a variant with crossing sequences would become ambiguous.

### 4.5 Evidence in Tidal

- **OVL-20 Evidence and Q-0142.** The compilers already disagree on how far OVL-20 reduces a set with constraints and no target. Programs at that edge are not portable today. Under P9 they stay valid on every compiler but get different types.
- **OVL-20 Evidence on G-11.** Every compiler rejects a set of one plain function and one template with `auto`, `decltype`, a cast to `void`, and an expression statement. This confirms that the positions P9 takes are free on all compilers in the matrix.
- **DED-19 tests.** `overload-set-template.cpp` and `overload-set-two-match.cpp` mark the edge of DED-40. A set with a function pointer parameter type still deduces nothing.

## 5. Findings

**X5-1** (missing; foundations/conversion-sequences.md, foundations/overload-resolution.md; ICS-1, ICS-2, OVL-39). No rule gives the sequence of an argument that names an overload set. OVL-39 pairs the set with each candidate, and the note of `overload-set-argument-ambiguous.cpp` says both sequences are the identity. ICS-P2 lists function to pointer as an lvalue transformation, but no rule says it applies to a set. Needed: the kind and rank of that sequence, so that a proposal can compare a new pairing with it. Suggestion: an ICS rule for an overload set argument, citing OVL-18 for the selection.

**X5-2** (missing; foundations/conversion-sequences.md; ICS-6, OVL-39). ICS-6 does not say whether an overload set argument matches `...`. Both compilers reject `g(f)` for `void g(...)` and a set `f`, with "no contextual type". Suggestion: say in ICS-6 or OVL-39 that a set has no ellipsis sequence, with a reject test.

**X5-3** (ambiguous; declarations/auto.md, expressions/decltype.md, foundations/overload-resolution.md; AUTO-8, DTY-4, OVL-20). AUTO-8 and DTY-4 speak of "the name of an overloaded function". OVL-20 also rejects a single function template, and an explicit argument list that leaves a parameter open. It is unclear whether `auto g = tmpl;` breaks AUTO-8 or only OVL-20. The tests cover two plain functions only. Suggestion: AUTO-8 and DTY-4 cite OVL-20 for the cases and use one term, "a name of an overload set", defined in the glossary.

**X5-4** (ambiguous; declarations/auto.md; AUTO-8, AUTO-11). AUTO-8 applies to "a variable declared with `auto`". It does not say whether `decltype(auto)` counts. AUTO-11 rejects "a function name" for `decltype(auto)`, which covers a single function and a set alike. A proposal that changes one case cannot tell which rule to amend. Suggestion: say whether AUTO-8 covers `decltype(auto)`, and split AUTO-11's item into a single function and a set.

**X5-5** (missing; foundations/overload-resolution.md; OVL-18, OVL-20). OVL-20 says "without a target type, for example with `auto` or `decltype`". No rule lists the positions without a target type. The Evidence names a cast to `void` and an expression statement. P9 needed these positions too:
- a deduced return type,
- an init capture,
- `sizeof`,
- an ellipsis argument,
- a deduced function template parameter,
- a constant template parameter declared with `auto`.

Suggestion: a list in OVL-20, which would also serve the index of free space (roadmap section 2).

**X5-6** (ambiguous; foundations/overload-resolution.md; OVL-18, TPL-11, DED-19). OVL-18 says that a template parameter that is not a type gives a target type, and that a function parameter does. A constant parameter declared with `auto` and a function parameter of type `T` give no target. DED-19 states the second case only for deduction. Suggestion: OVL-18 says that the parameter's type must be known, and points to DED-19 and TPL-11.

**X5-7** (missing; classes/access.md; ACC-3, DED-26). ACC-3 checks access "after overload resolution selects a function". It does not say from which context. For a direct call, the context is the call. The place of the name and the place of the call differ for a set closure, for a set with a target type, and for a call inside a template. DED-26 gives "as at the template's declaration" for substitution only. Suggestion: ACC-3 names the context of the check.

**X5-8** (missing; foundations/odr.md; ODR-3, OVL-18). ODR-3 says an evaluated expression that names a function uses it. A name of an overload set names several functions. No rule says that only the function selected by a target type is used. Under P9, the closure selects at each call. Suggestion: a sentence in ODR-3 for a set name.

**X5-9** (missing; templates/dependent-names.md; DEP-2). DEP-2 does not say when the name of an overload set is type-dependent. The cases are a dependent qualifier, such as `T::fn`, and dependent explicit template arguments, such as `f<T>`. A set name has no declared type, so the first item ("declared with a dependent type") does not reach it. Suggestion: an item in DEP-2.

**X5-10** (missing; expressions/lambdas.md; LAM-10, LAM-15). LAM-10 gives the conversion to a pointer to function for a lambda with "the same parameters and return type". It does not cover a generic lambda, whose call operator is a template. Both compilers accept `void (*p)(int) = [](auto x) {};`. P9 models its conversion on that case and cannot cite a rule. Suggestion: a rule for the conversion function template of a generic lambda without captures.

**X5-11** (hard to combine; expressions/lambdas.md, templates/dependent-names.md, templates/instantiation.md; LAM-15, DEP-8, DEP-9, INST-31). A generic lambda at namespace scope is not inside a template, but its call operator is one (LAM-15). No rule says that the two-phase lookup of DEP-9 applies to a call in its body. No rule says where the point of instantiation of its call operator is. Through such a lambda, both compilers find a function in an argument's namespace that is declared after the lambda. P9 relies on this behavior. Suggestion: one sentence in LAM-15 that cites DEP-9 and INST-31.

**X5-12** (ambiguous; declarations/constexpr.md, foundations/overload-resolution.md; CEV-2, OVL-18). CEV-2 limits taking the address of a `consteval` function. It does not say whether selecting one from a set by a target type counts as taking its address. An example is `int (*p)(int) = f;`, where `f(int)` is `consteval` and `f(long)` is not. Suggestion: say that OVL-18 selection is a use of the address under CEV-2.

**X5-13** (ambiguous; expressions/member-pointers.md; MPTR-9, MPTR-2, OVL-18). MPTR-9 lists "`C::f` without `&`" as not valid. Read alone, this covers a static member, but `C::f` names an ordinary function there and works with a target type. The test `form-function-without-amp.cpp` uses a non-static member. Suggestion: limit the item to non-static members.

**X5-14** (missing; foundations/values.md, expressions/postfix.md; VC-2, PFX-P3). VC-2, a `required` rule, says that a member access naming a non-static member function "can only be called". No `reject` rule states the prohibition, so `auto g = x.f;` has no rule to cite. P9 needs one for OVL-50. Suggestion: a reject rule beside PFX-11, with a test.

**X5-15** (ambiguous; foundations/names.md; NAM-27). The row "a function name with overloads: what the type of each function adds" does not say what a function template in the set adds, since a template has no type. Suggestion: say whether a template adds the namespaces of its parameter types that do not depend on its parameters, or only its own namespace, with a test.

**X5-16** (ambiguous; foundations/overload-resolution.md; OVL-39, OVL-P2). OVL-39 says "each function of the set is tried with each candidate", then "one pairing must win (OVL-36)". This reads as if each pairing were a candidate of its own. The test `overload-set-argument-with-other.cpp` also fits a second reading: each candidate picks its function by OVL-18 first, and then the candidates compare. The OVL-P2 table leaves the implied object cell of the row empty. Needed: whether pairings or candidates compete, so that a second pass can be stated. Suggestion: rewrite OVL-39 as "each candidate selects its function by OVL-18 and OVL-19 with its parameter type as the target".

**X5-17** (needs example; templates/packs.md, foundations/conversions.md, templates/deduction.md; PACK-23, DED-8, DED-18, CONV-8, REF-8). No page collects the ways a forwarded call differs from a direct call:
- a zero literal is no longer a null pointer constant,
- a braced list cannot be forwarded,
- a prvalue is moved instead of initializing the parameter,
- a bit-field cannot bind.

The runs in 1.4 show each difference. P9 and any wrapper inherit them. Suggestion: an Examples segment in packs.md beside PACK-23.

**X5-18** (outside knowledge). Three judgments come from general experience, and Tidal gives no data on them:
- that passing an overloaded name to generic code is a common need,
- that detection of "can this set be passed" is rare in real code (4.3),
- that the forwarding model in 1.4 is what users expect of a function object.

The design of the call operator also follows a general habit of writing a generic lambda that forwards to a name. Tidal states the parts (LAM-15, DED-8, PACK-23) but not the pattern.

**X5-19** (outside the spec; D32, Q-0007). Two needs fall outside the spec:
- Implementation cost. The type identity of OVL-46 needs every translation unit to give the same name to a closure type built from the same set. That is a cost in the linkage of names, and Tidal has no model to state it in.
- Version marking. P9 takes the space that AUTO-8 and DTY-4 leave free. A designer cannot tell whether a later version uses that space (X4-12, roadmap-2 item 3).

**X5-20** (needs example; templates/deduction.md; DED-19, DED-23). DED-19 says that the set "resolves after substitution". It does not say that a default template argument can give the value, as in `template <class F = void (*)(int)> int g(F); g(f)`, which both compilers accept. P9 must order its step after defaults, and only the order of DED-23's sentences shows that order. Suggestion: an example or test that passes a set to a parameter with a default type.

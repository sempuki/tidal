# X7: P12, value-specialized parameters

Date: 2026-11-01. Decision 0037. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md and decision 0037.
- The Propose sections of `documentation/exercises.md` (sections 1, 2 and 4).
- P12 and section 2 of `reports/clean-room/roadmap/designer.md`. Item 1 and section 1 of `reports/clean-room/roadmap-2/designer.md`.
- `reports/clean-room/X5/report.md`, as the model for this report's form.
- These pages, in full for their principles and rules: declarations/constexpr, foundations/constant-evaluation, templates/declarations, templates/arguments, templates/deduction (with its Evidence notes), templates/ordering, templates/specialization, foundations/odr, foundations/overload-resolution.
- The cited rules of other pages: IF-13 to IF-24 (statements/if), DCL-P3, DCL-8 and CV-8 (declarators), NAM-P1 and NAM-3 (names), EXS-8, FN-P1 and FN-P2, LAM-1, LAM-10, LAM-15, DEFARG-1, DEFARG-2, CTAD-1, CTAD-5, CTAD-6, CTAD-8, CTAD-11, CTAD-22, CTAD-29, INST-1, INST-31, the INST-14 Evidence line, DEP-2, DEP-4, UNEV-1, CPT-24, LNK-4.
- Glossary entries: constant evaluation, constant parameter, immediate function context, specialization, structural type, template, template argument, discarded statement.
- Tests: `declarations/constexpr/constexpr-parameter.cpp`, the two tests that cite CE-29, and a scan of all tests for `constexpr` in a parameter list.

Compiler runs: small programs built with g++ and clang++ (`-std=c++23 -pedantic-errors`) under /tmp/claude-1000/x7, to see what programs do today. The results are quoted where they support a claim. Probe names (a1, b1, and so on) refer to files in that directory.

Totals: 69 rule changes (11 new, 21 amended, 37 unchanged but must state their effect). One existing test changes its expected outcome, from reject to accept. 12 findings, X7-1 to X7-12.

## 1. Design

### 1.1 The problem

A program can give a function a value that is known at translation in one way only. The value goes in a template argument list, as `get<1>(p)`. Tidal shows four places where that does not work, or works badly:

- **Constructors.** No syntax gives explicit template arguments to a constructor template (templates/deduction, Evidence note F2). A constructor cannot take a constant that its body uses as a constant.
- **Operators.** `t[0]` and `a + b` have no place for a template argument list (OVL-7). An index operator cannot use its index as a constant.
- **Trailing arguments.** Explicit arguments fill the leading template parameters only (TARG-20). A pack in the head takes all of them (TARG-21).
- **`consteval` does not help.** A `consteval` function runs only during constant evaluation, but its parameter is still not a constant. Both compilers reject `A<n>` and `if constexpr (n > 0)` in a `consteval` function body (probes b1, b2). CE-3 says a parameter is usable "only during an evaluation that called the function". The constant expressions that translation needs are not such an evaluation (X7-2).

CX-5 rejects a `constexpr` parameter today. Both compilers reject `void f(constexpr int n);` (probe a1). So the syntax is free.

### 1.2 The feature

A function parameter can be declared `constexpr`. Such a parameter is a *value-specialized parameter*. Its argument must be a constant. Inside the function, the parameter is a constant, as a constant template parameter is. Each distinct value gives a distinct function.

```cpp
struct Pair { int first; double second; };
auto& get(Pair& p, constexpr int i) {
  static_assert(i == 0 || i == 1);
  if constexpr (i == 0) return p.first; else return p.second;
}
Pair p{1, 2.0};
get(p, 1) = 3.0;   // i is 1, and the result is double&
int k = 0;
get(p, k);         // rejected: k is not a constant
```

Today's spelling is `template <int I> auto& get(Pair&)` with `get<1>(p)`. Both compilers build that model (probe b3).

The roadmap called this "constant parameters". The glossary now uses "constant parameter" for a template parameter that takes a value (TPL-6, TARG-12). So the new term is "value-specialized parameter". A function with at least one value-specialized parameter is a *value-specialized function*.

### 1.3 Declarations (new rules CX-7 and CX-8; amended CX-5)

CX-7 allows `constexpr` on a parameter in these declarations:
- a function at namespace scope or in a class, including a member function, a constructor, an operator function, and a friend function,
- a lambda's parameter list,
- a deduction guide (CTAD-11).

The parameter's type must be a structural type (TPL-6), or a placeholder type `auto`, which takes its type from the argument (TPL-11). Top-level `const` is dropped, as TPL-32 does for a constant parameter.

CX-8 lists the places and forms that stay rejected:
- a function declared at block scope, because the function is a template (TPL-2),
- a virtual function, because a member function template must not be virtual (TPL-25),
- `main` (TPL-29), and a function with C linkage (TPL-3),
- a function type that is not part of a function declaration, as in a pointer or reference to function, an alias, or a type in a template argument,
- a handler parameter, a requires expression parameter, and an explicit object parameter,
- a parameter pack, which is left for a later proposal,
- a type that TPL-7 excludes, such as `void`, an rvalue reference, or a class with a private member,
- a storage class specifier on the parameter (TPL-33).

CX-5 keeps its other items. Its parameter item becomes "a parameter must not be `constexpr`, except as CX-7 allows".

### 1.4 The model: a function template (new rule TPL-48)

A value-specialized function is a function template, even with no template head. This is the model of DED-34, where an `auto` parameter makes the function a template.
- Each value-specialized parameter adds an *invented constant parameter* of the parameter's type.
- The invented constant parameters come after the head's parameters and after the invented type parameters of DED-34, in the order of the function parameters.
- A template argument list cannot name an invented constant parameter. Its value comes from the call's argument only (1.5).
- The function type of a specialization lists only the other parameters. The value is part of the specialization's identity, so the value is not passed at run time.
- Two values that are the same by TARG-P4 give one specialization (TARG-19). So `get(p, 1)` and `get(p, 2 - 1)` call one function. Two floating-point values with different representations give two functions.

### 1.5 The call: the value step (new rules DED-40, DED-41, TARG-28, CE-30)

Deduction gains a step between steps 5a and 5b of DED-P2. The step runs after the defaults of the template parameters are substituted, and before the check of DED-28 and the substitution of the function type.

For each value-specialized parameter, in order:
1. The parameter's type is taken after the values found so far are substituted. A `constexpr auto v` parameter has already deduced its type in step 2, as an `auto` parameter does (DED-34).
2. The argument must be a constant expression. It is evaluated alone, at the call, and not inside any evaluation that is running (X7-2). A call has one constant evaluation of each such argument, and every candidate uses that value.
3. The value converts to the parameter's type by the conversions of TARG-12. TARG-13 rejects narrowing and the other excluded conversions. A class type is copy-initialized, as TARG-16 says.
4. The result is the value of the invented constant parameter. Substitution then puts it into the function type, so a trailing return type such as `-> Box<i>` can use it.

If the argument is not a constant expression, or the conversion is not allowed, the template adds no candidate. This is a failure, as DED-5 is for an explicit argument that narrows, and not an error. A call with no other candidate is rejected by DED-35.

Other rules of the step:
- The invented constant parameter takes no value from another pair or from an explicit argument. A use of it in another parameter's type, such as `Box<i> b`, is a non-deduced position (DED-13).
- If the call uses a parameter's default argument, the default is the argument (DED-9). It must be a constant expression in the context of the declaration.
- DED-41: if the evaluation of such an argument exceeds a limit of CE-10, the program is not valid. The template is not quietly dropped. Without this rule, the chosen overload would depend on implementation-defined limits. Both compilers drop a template quietly today when a template argument in the function type exceeds the step limit (probe a4, X7-5).
- CE-30 (no-diagnostic): an argument must not contain an operation that CE-26 lets an implementation miss. Otherwise one compiler sees a constant and another does not, and they choose different functions.

TARG-28 states that the argument of a value-specialized parameter is the template argument for its invented constant parameter. TARG-12 to TARG-16 apply. A string literal is therefore not valid for a `constexpr const char*` parameter (TARG-15), and the candidate drops. A class with a constructor that takes `const char*` accepts one (TARG-16).

When a value-specialized candidate wins, the argument is not evaluated at run time. The other arguments are evaluated as usual. A constant expression changes no object outside its own evaluation (CE-5), so nothing observable is lost.

### 1.6 The body (new rule CX-9; amended CE-3, DEP-2, DEP-4, IF-13)

In the function, the name of a value-specialized parameter is an expression as TPL-9 gives:
- a prvalue for a scalar type, which the program cannot assign to or take the address of (TPL-10),
- an lvalue of type `const C` for a class type `C`, which refers to the one template parameter object for that value (TPL-9),
- an lvalue for a reference type.

The name is usable in constant expressions in the body, the trailing return type, the `noexcept` operand, and a trailing requires clause (CE-3 amended). So it can be an array bound, a case label, a template argument, the condition of `if constexpr`, or an argument of a `consteval` call.

The body is a template body:
- A name of a value-specialized parameter is value-dependent (DEP-4). For `constexpr auto`, it is type-dependent too (DEP-2), as a constant template parameter with a placeholder type is.
- A discarded statement that depends on the value is not instantiated (IF-13). X7-7 asks for the meaning of "in a template".
- Calls in the body use two-phase lookup (DEP-9). Each specialization has its points of instantiation (INST-31).

A recursive call with `n - 1` calls another specialization. INST-14 limits the depth of nesting.

### 1.7 Overloads that differ in constness (new rule OVL-45; amended OVL-P3, OVL-27)

A program can declare `f(int)` and `f(constexpr int)` together. They are two functions: a plain function and a template (TPL-22). TPL-20 is amended so that the positions and types of value-specialized parameters are part of the head's equivalence.

```cpp
int shift(int x, int s);                // checks s at run time
int shift(int x, constexpr int s) {     // checks s during translation
  static_assert(s >= 0 && s < 32);
  return x << s;
}
shift(v, 3);   // second: the sequences tie, and OVL-45 prefers it
shift(v, n);   // first: n is not a constant, so the second adds no candidate
```

Without a new rule, OVL-27 would choose the plain function on a tie. Both compilers prefer a plain function over a template with the same sequences today (probe a6). So the value-specialized overload could never win.

OVL-45 says that `F1` beats `F2` when every sequence ties, and the argument positions where `F1` has a value-specialized parameter are a strict superset of those where `F2` has one. The rule comes after OVL-26 and OVL-29, and before OVL-27. OVL-27 and OVL-P3 are amended to show that order.

The sequence for a value-specialized position is the implicit conversion sequence from the argument to the parameter's type, as for any parameter (OVL-23). So a better conversion still wins first. With `f(long)` and `f(constexpr int)`, the call `f(3)` chooses the second by its identity sequence. The call `f(3L)` chooses the first, because `long` to `long` beats `long` to `int`.

Crossing positions do not decide. With `f(constexpr int, int)` and `f(int, constexpr int)`, the call `f(1, 2)` falls through to partial ordering, finds no order, and is ambiguous (ORD-15).

A trailing requires clause can test the value. `void f(constexpr int n) requires (n > 0)` is not viable for `f(-1)` (OVL-24), and another overload can take that call.

### 1.8 No address and no explicit specialization (amended OVL-18 and DED-30; new rule TSP-33)

- A target type gives no value to an invented constant parameter, because a function type holds no value. So a value-specialized function never matches a target type (OVL-18, DED-30). A set that holds only such functions has no address. DED-35 rejects `&f` with a target, and OVL-20 rejects it without one.
- A lambda with a value-specialized parameter has no conversion to a pointer to function (LAM-10).
- TSP-33 rejects an explicit specialization and an explicit instantiation of a value-specialized function. No syntax can name the value. A program uses `if constexpr` or an overload instead.

### 1.9 The one-definition rule (amended ODR-12)

ODR-12 covers value-specialized functions as function templates. Several units can define one, for example in a header. Each specialization is one entity with one address in the program. A `static` local variable in the body is one object for each value. Probe a7 shows this for the model: `count<1>` and `count<2>` have separate counters on both compilers.

### 1.10 Alternatives considered

- **Explicit arguments for the invented parameters**, so that `get<1>(p)` and `get(p, 1)` both work. DED-34 allows explicit arguments for invented type parameters. But a pack in the head takes all explicit arguments (probe c2, X7-8), and two ways to give one value would need a rule for their agreement. Rejected.
- **Keep the parameter in the function type** and pass the value at run time too. A pointer to such a function would then accept any `int`, and the body would rely on a value that the pointer's caller does not fix. Rejected.
- **A hard error when the argument is not a constant**, even when another candidate exists. That removes the overload on constness, which is a main use. Rejected.
- **Make `consteval` parameters constants.** Each call would then need its own instantiation, which is this proposal with a different keyword. It would also change the meaning of valid `consteval` functions. Rejected.
- **Parameter packs** of value-specialized parameters. These are useful, but they need DED-20's rules for a pack before the end. CX-8 keeps the space free for a later proposal.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| CX-P5 | new | none | principle | A value-specialized parameter makes a function a template over the parameter's value. The argument is a constant, and the parameter is a constant in the body. | none |
| CX-7 | new | none | required | `constexpr` on a parameter of a function, lambda, or deduction guide. Its type is structural or a placeholder (1.3). | none |
| CX-8 | new | none | reject | The places and forms where a value-specialized parameter is not valid (1.3). | none |
| CX-9 | new | none | required | The name in the function is an expression as TPL-9 gives, and it is usable in constant expressions (1.6). | none |
| TPL-48 | new | none | required | A value-specialized function is a function template. Its invented constant parameters, their order, and the function type of a specialization (1.4). | none |
| TARG-28 | new | none | required | The argument of a value-specialized parameter is the template argument of its invented constant parameter. TARG-12 to TARG-16 apply (1.5). | none |
| DED-40 | new | none | required | The value step between steps 5a and 5b. Failure removes the candidate. No other pair and no explicit argument gives the value (1.5). | none |
| DED-41 | new | none | reject | An argument evaluation that exceeds a CE-10 limit makes the program not valid (1.5). | none |
| CE-30 | new | none | no-diagnostic | An argument of a value-specialized parameter must not contain an operation that CE-26 lets an implementation miss (1.5). | none |
| OVL-45 | new | none | required | The tie-breaker that prefers the strict superset of value-specialized positions, placed before OVL-27 (1.7). | none |
| TSP-33 | new | none | reject | No explicit specialization or explicit instantiation of a value-specialized function (1.8). | none |
| CX-5 | amended | reject | reject | The parameter item allows CX-7. The other items stay. | `declarations/constexpr/constexpr-parameter.cpp` (reject to accept) |
| CE-P2 | amended | principle | principle | The list of what an evaluation can use gains value-specialized parameters. | none |
| CE-3 | amended | reject | reject | A value-specialized parameter is usable. An ordinary parameter is never usable in a constant expression that translation needs (X7-2). | none |
| CE-19 | amended | reject | reject | Adds the argument of a value-specialized parameter to the places where a conversion must not narrow. | none |
| TPL-P2 | amended | principle | principle | The kinds table notes that a value-specialized parameter adds an invented constant parameter. | none |
| TPL-20 | amended | reject | reject | The positions and types of value-specialized parameters are part of head equivalence. `f(int)` and `f(constexpr int)` are two functions. | none |
| DED-P2 | amended | principle | principle | The diagram gains the value step between 5a and 5b. | none |
| DED-3 | amended | required | required | A value-specialized parameter always takes part, through DED-40, also when its type holds no template parameter. | none |
| DED-9 | amended | required | required | A used default argument of a value-specialized parameter gives the value. | none |
| DED-13 | amended | required | required | A use of an invented constant parameter in another parameter's type is a non-deduced position. The table of DED-P3 gains a row. | none |
| DED-30 | amended | required | required | A target type gives no value to an invented constant parameter, so the match fails. | none |
| OVL-P3 | amended | principle | principle | The tie-breaker list gains OVL-45 before step 2c. | none |
| OVL-18 | amended | required | required | A value-specialized function never matches a target type. | none |
| OVL-27 | amended | required | required | The rule comes after OVL-45. | none |
| ORD-5 | amended | required | required | A value-specialized parameter takes part by its declared type. | none |
| ORD-13 | amended | required | required | The conditions include the same value-specialized positions. | none |
| ODR-12 | amended | required | required | The list includes value-specialized functions. A static local is one object per specialization. | none |
| DEP-2 | amended | required | required | A `constexpr auto` parameter's name is type-dependent. | none |
| DEP-4 | amended | required | required | A value-specialized parameter's name is value-dependent. | none |
| LAM-10 | amended | required | required | A lambda with a value-specialized parameter has no conversion to a pointer to function. | none |
| IF-13 | amended | required | required | The body of a value-specialized function is "in a template" (X7-7). | none |
| TPL-2 | unchanged, state effect | reject | reject | A value-specialized function at block scope is a template at block scope. | none |
| TPL-3 | unchanged, state effect | reject | reject | No C linkage. | none |
| TPL-6 | unchanged, state effect | required | required | Gives the allowed types. | none |
| TPL-7 | unchanged, state effect | reject | reject | Gives the excluded types, also for a type that a placeholder takes. | none |
| TPL-9 | unchanged, state effect | required | required | Gives the value category, and the one object per class value. | none |
| TPL-10 | unchanged, state effect | reject | reject | No assignment to, and no address of, a scalar parameter. | none |
| TPL-11 | unchanged, state effect | required | required | `constexpr auto` takes its type from the argument. | none |
| TPL-22 | unchanged, state effect | required | required | A plain function and a value-specialized one share a name. | none |
| TPL-25 | unchanged, state effect | reject | reject | A value-specialized member function is not virtual. | none |
| TPL-29 | unchanged, state effect | reject | reject | `main` has no value-specialized parameter. | none |
| DED-24 | unchanged, state effect | required | required | The value is substituted into the function type, and an invalid result removes the candidate. | none |
| DED-34 | unchanged, state effect | required | required | Its invented type parameters come before the invented constant parameters. | none |
| DED-35 | unchanged, state effect | reject | reject | A call whose only candidate needs a constant that the argument is not. | none |
| DED-38 | unchanged, state effect | required | required | Each value gives its own function. | none |
| TARG-13 | unchanged, state effect | reject | reject | Narrowing and excluded conversions remove the candidate. | none |
| TARG-15 | unchanged, state effect | reject | reject | A string literal is not valid for a pointer parameter. | none |
| TARG-16 | unchanged, state effect | required | required | A class parameter is copy-initialized from the argument. | none |
| TARG-19 | unchanged, state effect | required | required | Equal values name one specialization. | none |
| CE-10 | unchanged, state effect | impl-defined | impl-defined | DED-41 turns an exceeded limit into an error at the call. | none |
| CE-11 | unchanged, state effect | required | required | A value-specialized parameter can be an array bound, enumerator value, or case label. | none |
| CE-27 | unchanged, state effect | unspecified | unspecified | An argument whose value depends on an unspecified order can give different specializations on two implementations. | none |
| CEV-1 | unchanged, state effect | reject | reject | A `consteval` call with a value-specialized parameter as its argument is constant. A `constexpr` value-specialized function is "produced from a template" for escalation. | none |
| IF-16 | unchanged, state effect | reject | reject | `if constexpr (n)` is valid. | none |
| OVL-1 | unchanged, state effect | required | required | A value-specialized function adds at most one specialization. | none |
| OVL-10 | unchanged, state effect | required | required | A value-specialized operator is a template, so it removes no built-in candidate. | none |
| OVL-23 | unchanged, state effect | required | required | The sequence of a value-specialized position is the one from the argument to the parameter's type. | none |
| OVL-28 | unchanged, state effect | required | required | Two value-specialized functions with different parameter types are ordered by those types. | none |
| ORD-3 | unchanged, state effect | required | required | An invented constant parameter appears in no compared type, so it needs no value. | none |
| ORD-15 | unchanged, state effect | reject | reject | Crossing value-specialized positions with tied sequences are ambiguous. | none |
| ODR-3 | unchanged, state effect | required | required | The argument is not evaluated at run time. Whether its names are uses is X7-6. | none |
| ODR-10 | unchanged, state effect | no-diagnostic | no-diagnostic | Definitions in several units must match. | none |
| INST-14 | unchanged, state effect | impl-defined | impl-defined | Recursion through `n - 1` is limited by the depth of instantiation. | none |
| INST-31 | unchanged, state effect | required | required | Each specialization has its points of instantiation. | none |
| LAM-15 | unchanged, state effect | required | required | A lambda with a value-specialized parameter has a call operator template, as a generic lambda has. | none |
| CTAD-5 | unchanged, state effect | required | required | A constructor with a value-specialized parameter gives a guide with that parameter. | none |
| CTAD-22 | unchanged, state effect | required | required | OVL-45 decides before CTAD-22 prefers a guide from a constructor that is not a template. | none |
| DEFARG-1 | unchanged, state effect | required | required | A value-specialized parameter can have a default argument, which must be a constant. | none |

Counts: 69 changes. 11 new, 21 amended, 37 unchanged but must state their effect. 0 split, 0 withdrawn. One existing test changes outcome, from reject to accept. The glossary also gains the terms "value-specialized parameter" and "value-specialized function", which have no rule IDs.

A scan of all tests for `constexpr` after `(` or `,` found three files. `if/condition-declaration-constexpr.cpp` and `if/constexpr-with-init.cpp` declare variables in conditions, which CX-7 does not touch. `declarations/constexpr/constexpr-parameter.cpp` is the changed test. CX-5 then needs a new reject test for a form that CX-8 still rejects, such as a pointer to function with a `constexpr` parameter.

The proposal needs these new tests:
- `tests/declarations/constexpr/`:
  - `value-parameter-constant-in-body.cpp` (accept; `static_assert`, array bound, template argument, `if constexpr`),
  - `value-parameter-result-per-value.cpp` (accept; `get(p, 0)` and `get(p, 1)` have different result types),
  - `value-parameter-not-constant.cpp` (reject; a run-time variable and no other candidate),
  - `value-parameter-function-pointer.cpp` (reject; CX-8),
  - `value-parameter-block-scope.cpp` (reject; CX-8 and TPL-2),
  - `value-parameter-virtual.cpp` (reject; CX-8 and TPL-25),
  - `value-parameter-non-structural.cpp` (reject; TPL-7),
  - `value-parameter-consteval-call.cpp` (accept; the contrast to probe b1),
  - `value-parameter-string-literal-pointer.cpp` (reject; TARG-15),
  - `value-parameter-class-from-literal.cpp` (accept; TARG-16),
  - `value-parameter-static-local-per-value.cpp` (run),
  - `value-parameter-no-address.cpp` (reject; OVL-18).
- `tests/templates/deduction/`:
  - `value-parameter-narrowing.cpp` (reject; `300` for `constexpr unsigned char`),
  - `value-parameter-auto.cpp` (accept; `constexpr auto`),
  - `value-parameter-default.cpp` (accept),
  - `value-parameter-step-limit.cpp` (reject; DED-41).
- `tests/foundations/overload-resolution/`:
  - `value-overload-constant-wins.cpp` (accept; OVL-45),
  - `value-overload-runtime-fallback.cpp` (run),
  - `value-overload-better-conversion-first.cpp` (accept; `f(long)` against `f(constexpr int)` with `3L`),
  - `value-overload-crossing-ambiguous.cpp` (reject; ORD-15),
  - `value-overload-requires-value.cpp` (accept; a trailing requires clause on the value).
- One test each in other directories:
  - `tests/templates/specialization/value-parameter-explicit-specialization.cpp` (reject; TSP-33),
  - `tests/foundations/odr/value-parameter-two-units.cpp` (run; ODR-12),
  - `tests/expressions/lambdas/value-parameter-no-conversion.cpp` (reject; LAM-10),
  - `tests/templates/class-template-deduction/value-parameter-guide.cpp` (accept; `Buf(constexpr int n) -> Buf<n>;` makes `Buf b(8);` a `Buf<8>`).

## 3. Interactions

**Constant evaluation (CE-P2, CE-P4, CE-3, CE-5, CE-10, CE-11, CE-16, CE-19, CE-26, CE-27).**
- The argument is evaluated alone at the call. A `constexpr` function that has only a declaration at that point gives no constant (CE-16). So `f(g())` with `g` defined later in the unit quietly chooses the run-time overload, and a call after the definition chooses the value-specialized one. The proposal accepts this. It matches what a template argument does with such a call.
- CE-10 and CE-26 would make the choice of overload depend on the implementation. DED-41 and CE-30 close those gaps.
- CE-27 can give two implementations different values, so they call different specializations. That is already true of any template argument.

**`consteval` and immediate contexts (CEV-1, CEV-2, IF-19, IF-24).** In the body, a call to a `consteval` function with value-specialized arguments is a constant expression, so it needs no immediate function context. A `constexpr` value-specialized function counts as "produced from a template" in CEV-1, so it can escalate. CEV-2 is never reached, because no address exists (1.8).

**Template declarations (TPL-2, TPL-3, TPL-6 to TPL-11, TPL-20, TPL-22, TPL-25, TPL-29, TPL-32, TPL-33).** The proposal reuses the constant parameter's types, value categories and object identity. Every limit on where a template can be declared applies through TPL-48.

**Template arguments (TARG-12 to TARG-16, TARG-19, TARG-20, TARG-P4).** The argument follows TARG-12's conversions, so it is stricter than an ordinary argument. No integer converts to floating point, and nothing narrows. TARG-20 is unchanged: explicit arguments still reach only the head's parameters.

**Deduction (DED-P2, DED-3, DED-5, DED-9, DED-13, DED-21, DED-23, DED-24, DED-28, DED-30, DED-34, DED-35).** The value step sits after the defaults, because a parameter's type can use a defaulted type parameter. It sits before the function type, because the function type can use the value. DED-23 never finds an invented constant parameter without a value, because the argument or its default is always there.

**Partial ordering (ORD-1, ORD-3, ORD-5, ORD-13, ORD-15).** Ordering uses the declared parameter types, including value-specialized ones. Invented constant parameters appear in no type, so ORD-3 gives them no role. OVL-45 runs before OVL-28, so a template with `T` loses to `constexpr auto` at the same position when sequences tie.

**Specialization (TSP-2, TSP-4, TSP-25, TSP-33).** No explicit specialization exists for a value. A function template with a head and a value-specialized parameter also loses its explicit specializations under TSP-33. That keeps TSP-4's matching by function type sound.

**The one-definition rule (ODR-3, ODR-10, ODR-12, ODR-14).** The definition goes in a header like any function template. Two units that call `f(3)` share one specialization.

**Overload resolution (OVL-1, OVL-7, OVL-10, OVL-18, OVL-23, OVL-24, OVL-27, OVL-28, OVL-35, OVL-36).**
- An index operator `operator[](constexpr int i)` lets `t[0]` and `t[1]` return different types. That is a main use.
- Built-in candidates stay, because OVL-10 removes them only for a non-template.
- A call with a set argument (OVL-39) reaches a value-specialized parameter only if the set's selection is a constant pointer to function (TARG-12, last sentence).

**Class template argument deduction (CTAD-5, CTAD-8, CTAD-11, CTAD-17, CTAD-22).** A guide can take a value and make it a class template argument. This answers the gap in templates/deduction Evidence F2 for constructors.

**Lambdas (LAM-1, LAM-10, LAM-15).** `[](constexpr int n) { ... }` has a call operator template. A call `l(3)` replaces the spelling `l.template operator()<3>()`.

**Dependent names and instantiation (DEP-2, DEP-4, DEP-9, INST-14, INST-31, IF-13, IF-22, IF-23).** The body follows every template rule. This includes the check of the parts of a discarded statement that do not depend on the value (IF-22).

**Not touched.** Calls through pointers, virtual dispatch, and all functions without a value-specialized parameter.

## 4. Impact on existing programs

Tidal has no corpus of real programs. The claims below come from the rules and from compiler runs.

### 4.1 Rejected becomes valid

Every declaration with `constexpr` on a function parameter. Both compilers reject it today (probe a1), and CX-5 has a test.

### 4.2 Undefined becomes defined

None. No `undefined` rule changes.

### 4.3 Valid programs that change meaning

None. A value-specialized parameter cannot appear in a valid program today. OVL-45 applies only when a candidate has such a parameter. No detection program can test the declaration, because the declaration is a syntax error and does not take part in substitution.

A library that adds `f(constexpr int)` beside an existing `f(int)` changes which body runs for constant arguments. That is the library's change, as with any new overload. When the two bodies agree, results are the same.

### 4.4 Valid programs that become invalid

None.

### 4.5 Evidence in Tidal

- CX-5 and its test show that the syntax is free.
- The deduction Evidence note F2 and TARG-20 show the gaps that the feature fills.
- Probes a3 and a4 show that a template argument's evaluation failure already selects another overload quietly. P12 would add a common path into that behavior, so DED-41 and CE-30 fix it for the new path. X7-5 asks Tidal to state the existing case.
- Code size grows with each distinct value, as with any template. INST-14's Evidence gives each compiler's depth limit for recursive use.

## 5. Findings

**X7-1** (missing; foundations/constant-evaluation.md, templates/arguments.md; CE-29, TARG-14). TARG-14 and the arguments page Examples cite CE-29 for a reference variable with a constant initializer. The tests `expressions/constant-expressions/reference-constant-initializer.cpp` and `reference-runtime-initializer.cpp` cite it too. No page defines CE-29. CE-3's list of usable variables does not include such a reference, but the accept test relies on it. Suggestion: add CE-29 to constant-evaluation.md and list it in CE-3, or point the citations at an existing rule.

**X7-2** (ambiguous; foundations/constant-evaluation.md, declarations/constexpr.md, statements/if.md; CE-3, CEV-1, IF-16, CE-11). CE-3 says a function parameter "is usable only during an evaluation that called the function". Read alone, this allows `A<n>` and `if constexpr (n > 0)` in a `consteval` function, because every evaluation of that body is one that called the function. Both compilers reject both (probes b1, b2), and the same for a `constexpr` function called in a `static_assert` (probe a2). The missing rule says that the constant expressions translation needs (a template argument, an array bound, an `if constexpr` condition, a `static_assert`, a case label) are evaluated on their own where they appear, never inside a running evaluation. Suggestion: state that in CE-3 or a new rule, with tests b1 and b2. P12 needs the rule to say why it is needed.

**X7-3** (missing; foundations/constant-evaluation.md, declarations/declarators.md, foundations/odr.md, glossary; CE-3, CV-8, ODR-3, CE-29). D28 is still open. "Usable in constant expressions" has three definitions in different words. CE-3 uses "a constant initializer", CV-8 restates it for `const` integers, and ODR-3 defines "a constant variable" as one "that a constant expression initializes". CE-29 is missing (X7-1). A proposal that adds a fourth kind of usable name has no single place to amend. Suggestion: a glossary term and one rule that lists every kind, cited by CE-3, CV-8 and ODR-3.

**X7-4** (missing; foundations/names.md, declarations/functions/definitions.md, declarations/declarators.md; NAM-3, DCL-P3, EXS-8, TPL-20, TPL-22). No rule says when two declarations of a function name in one scope declare one function and when they declare two overloads. NAM-3 assumes "declarations that refer to one entity, such as two declarations of one function". EXS-8 states the `noexcept` case only, and TPL-20 and TPL-22 state the template case. No rule says that the same parameter types give one function, that different ones give overloads, or that a difference in the return type alone is not valid. P12 needs this to state that `f(int)` and `f(constexpr int)` are two functions. Suggestion: a rule on names.md or definitions.md, with tests for each case.

**X7-5** (ambiguous; foundations/constant-evaluation.md, templates/deduction.md; CE-9, CE-10, DED-24). CE-10 says an implementation "rejects" an evaluation that exceeds a limit, and CE-9 is a `reject` rule. In a template argument inside a function type, both compilers treat undefined behavior and an exceeded step limit as substitution failures. The template drops, and another overload wins (probes a3 and a4 both return 2 on both compilers). So the chosen overload depends on implementation-defined limits. DED-24 says "an expression that is not valid", which does not clearly cover a failed evaluation. Suggestion: DED-24 says that a template argument that is not a constant expression is a substitution failure, and an exception or Evidence note covers CE-10.

**X7-6** (missing; foundations/odr.md; ODR-3, ODR-P1, TARG-14). ODR-3 does not say whether a constant template argument uses what it names. `S<&g>` with `g` declared and never defined builds, links and runs on both compilers (probe a5). The ODR-P1 table has no row for a template argument, and "an evaluated expression that takes an address" seems to cover `&g`. P12 needs the answer for its arguments. Suggestion: a row in ODR-P1 and an item in ODR-3.

**X7-7** (ambiguous; statements/if.md, glossary, templates/declarations.md, templates/deduction.md, expressions/lambdas.md; IF-13, IF-22, IF-23, TPL-1, DED-34, LAM-15). IF-13, IF-22 and IF-23 depend on "in a template". The glossary defines a template as "a declaration with a template head". A function with an `auto` parameter (DED-34) and a generic lambda (LAM-15) have no head. Both compilers still treat a generic lambda body at namespace scope as a template body and do not check a discarded branch there (probe c1). P12's functions also have no head. Suggestion: a glossary term for code inside a template that names DED-34, LAM-15 and member templates, used by the IF and DEP rules.

**X7-8** (ambiguous; templates/deduction.md, templates/arguments.md; DED-34, TARG-20, TARG-21). DED-34 says "explicit template arguments can give the invented parameters". When the head ends in a pack, the pack takes every explicit argument (TARG-21), and no explicit argument reaches an invented parameter. `template <class... T> int f(auto x)` called as `f<int, double>(1)` gives 2 on both compilers (probe c2). A designer who adds a second kind of invented parameter has to guess how the order works. Suggestion: DED-34 says that explicit arguments reach the invented parameters only when the head has no pack, with a test.

**X7-9** (missing; foundations/constant-evaluation.md, glossary; CE-P4, CE-11, CE-19, CE-25, IF-16, EXS-1). No rule lists the contexts that need a constant expression. CE-P4 and CE-11 list array bounds, enumerators and case labels. CE-19 adds `explicit`, `noexcept` and template arguments. The glossary entry "constant evaluation" adds `static_assert`. IF-16, CINIT-1 and CEV-1 add their own contexts. P12 adds one more and has to amend four places. This is one of the cross-page inventories of roadmap section 2. Suggestion: one rule that lists every such context, which the other rules cite.

**X7-10** (missing; templates/declarations.md, templates/arguments.md; TPL-9, TARG-16, CE-23). TPL-9 says that one object with static storage duration stands for all equal class arguments. TARG-16 says the argument copy-initializes it. No rule says when the object is initialized, whether its destructor runs, or whether the program can compare its address across two template parameter types. P12 makes such objects common, as the arguments of index operators and guides. Suggestion: a rule beside TPL-9 that states the object's lifetime and its identity.

**X7-11** (outside knowledge). These judgments come from general experience. Tidal gives no data on them:
- that index access with a constant, `t[0]`, and constant-checked arguments such as shift amounts and format strings are common needs,
- that an implementation can tell a failure from an exceeded limit apart from other evaluation failures, which DED-41 needs,
- that one instantiation per value is an acceptable cost.

The model of a function template with invented parameters comes from DED-34 and LAM-15.

**X7-12** (outside the spec; D32, Q-0007). Two needs fall outside the spec:
- Implementation cost. Each value gives an instantiation and a name that every unit must agree on (ODR-12). Tidal has no model of translation cost or of names at link time.
- Version marking. P12 takes the space that CX-5 leaves free. A designer cannot tell whether a later version uses that space (X4-12, roadmap-2 item 3).

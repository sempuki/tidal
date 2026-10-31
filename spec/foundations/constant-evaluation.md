# Constant expressions

A constant expression is an expression that the implementation evaluates
during translation. {term}`Constant evaluation <constant evaluation>`
permits most operations. This page lists the operations that it rejects.

## Principles

:::{principle} CE-P1
:rules: CE-1 CE-17 CE-18 CE-20 CE-23

Constant evaluation is the same program run at translation time
({principle}`TR-P4`). Within one evaluation it can do what a run can do:

- arithmetic,
- loops,
- calls and virtual calls,
- changes to the objects it has made,
- allocation.

A function is not rejected for containing a construct that constant
evaluation cannot handle. It is rejected only when an evaluation reaches
that construct, so a function can serve both modes. A reference whose
object the evaluation does not know is a problem only when the
evaluation reads through it. The objects that an evaluation makes need a
{term}`literal type`, one whose objects the evaluation can build and
destroy.
:::

:::{principle} CE-P2
:rules: CE-2 CE-3 CE-5 CE-7 CE-14 CE-16 CE-21 CE-24

An evaluation is closed. It can use only what translation already knows:

- functions declared `constexpr` or `consteval` and defined before the
  evaluation,
- variables whose values are fixed at translation time because they
  have a {term}`constant initializer`,
- objects that the evaluation itself created.

It can change only its own objects, because anything else exists at run
time. Its result must outlive it, so it cannot be a pointer to an
automatic or temporary object, and storage that the evaluation allocated
must be freed before the evaluation ends. Its result must also be
complete, with a value in every part.
:::

:::{principle} CE-P3
:rules: CE-4 CE-6 CE-8 CE-9 CE-12 CE-13 CE-15 CE-22

An evaluation must give one answer that every implementation can agree
on. So it rejects every operation whose result depends on the machine
or on an order the language does not fix:

- reinterpreting a pointer,
- reading a volatile object,
- reading a union member that is not active,
- comparing the addresses of string literals,
- ordering pointers to unrelated objects,
- any operation that is undefined.

Copying an object reads each of its parts, so a copy of an object with
a part that has no value is undefined too. An exception has no handler
at translation time, so a `throw` that the evaluation reaches ends it.
:::

:::{principle} CE-P4
:rules: CE-11 CE-19 CE-25

Where the language needs a constant to build the program, it needs the
exact value. These places need one:

- an array bound,
- an enumerator,
- a case label,
- the operands of `explicit` and `noexcept`.

A conversion that could lose information is not a constant there. The
condition of `if constexpr` is the one place that only needs a yes or
no.
:::

:::{principle} CE-P5
:rules: CE-10

An evaluation runs on the translator, so it has limits that a run does
not. The depth of recursion and the number of steps that an
implementation permits are its own to set and document.
:::

## Constraints

:::{rule} CE-1
:kind: required
:tests: expressions/constant-expressions/arithmetic-and-calls.cpp expressions/constant-expressions/lambda-call.cpp expressions/constant-expressions/virtual-call.cpp expressions/constant-expressions/try-without-throw.cpp expressions/constant-expressions/new-delete-transient.cpp expressions/constant-expressions/function-pointer-equality.cpp

Constant evaluation can contain these operations:

- arithmetic, loops, and calls to `constexpr` functions,
- changes to objects that the same evaluation created,
- calls to lambdas and to `constexpr` virtual functions,
- `try` blocks, if no exception is thrown,
- `new` and `delete` expressions, as CE-7 describes,
- equality comparison of function pointers.
:::

:::{rule} CE-2
:kind: reject
:tests: expressions/constant-expressions/non-constexpr-call.cpp

During constant evaluation, the program can call only `constexpr` and
`consteval` functions.
:::

:::{rule} CE-3
:kind: reject
:tests: expressions/constant-expressions/read-non-const-variable.cpp expressions/constant-expressions/read-const-runtime-init.cpp expressions/constant-expressions/read-const-constant-init.cpp expressions/constant-expressions/read-parameter-outside.cpp expressions/constant-expressions/const-enum-usable.cpp expressions/constant-expressions/const-volatile-not-usable.cpp

Constant evaluation must not read the value of a variable that is not
usable in constant expressions. A variable is usable if it is
`constexpr`, or if it is a `const` variable of integer or enumeration
type with a {term}`constant initializer` (CV-8). A `const volatile`
variable is not usable. A function parameter is usable only during an
evaluation that called the function.
:::

:::{rule} CE-4
:kind: reject
:tests: expressions/constant-expressions/reinterpret-cast.cpp expressions/constant-expressions/void-pointer-to-object.cpp

Constant evaluation must not contain a `reinterpret_cast`. It must not
convert a pointer to `void` to a pointer to an object type.
:::

:::{rule} CE-5
:kind: reject
:tests: expressions/constant-expressions/modify-outside-object.cpp expressions/constant-expressions/modify-local-object.cpp

Constant evaluation must not change an object that existed before the
evaluation started. It can change objects that it created.
:::

:::{rule} CE-6
:kind: reject
:tests: expressions/constant-expressions/inactive-union-member.cpp expressions/constant-expressions/active-union-member.cpp

Constant evaluation must not read a union member that is not the active
member. An assignment to a member makes that member active.
:::

:::{rule} CE-7
:kind: reject
:tests: expressions/constant-expressions/allocation-not-freed.cpp expressions/constant-expressions/allocation-in-result.cpp expressions/constant-expressions/delete-mismatch.cpp expressions/constant-expressions/double-delete.cpp

Storage that a `new` expression gets during constant evaluation must be
freed in the same evaluation. These conditions apply:

- The evaluation must free the storage exactly once.
- `delete[]` must free storage from `new[]`, and `delete` must free
  storage from `new`.
- The result of the evaluation must not point to that storage.
:::

:::{rule} CE-8
:kind: reject
:tests: expressions/constant-expressions/throw-evaluated.cpp expressions/constant-expressions/throw-not-evaluated.cpp

Constant evaluation must not evaluate a throw-expression. A
throw-expression on a path that the evaluation does not take is valid.
:::

:::{rule} CE-9
:kind: reject
:tests: expressions/constant-expressions/undefined-behavior.cpp expressions/constant-expressions/undefined-behavior-dangling.cpp

Constant evaluation must not evaluate an operation that has undefined
behavior. Each rule of class `undefined` gives such an operation. The
Evidence segment lists these rules and the compilers that reject each
one during constant evaluation.
:::

:::{rule} CE-11
:kind: required
:tests: expressions/constant-expressions/integral-constant-array-bound.cpp

These need a constant expression of integer or enumeration type:

- an array bound,
- an enumerator value,
- a case label.
:::

:::{rule} CE-19
:kind: reject
:tests: expressions/constant-expressions/converted-constant-narrowing.cpp expressions/constant-expressions/explicit-condition-narrowing.cpp

Where a constant converts to another type for a use in CE-11, the
conversion must not narrow. The condition of `if constexpr` is an
exception (IF-16). This also applies to:

- the condition of `explicit(...)`,
- the operand of a `noexcept` specifier (EXS-1),
- a non-type template argument *(templates referenced)*.
:::

:::{rule} CE-25
:kind: reject
:tests: expressions/constant-expressions/array-bound-floating.cpp expressions/constant-expressions/array-bound-not-constant.cpp expressions/constant-expressions/enumerator-floating.cpp expressions/constant-expressions/case-label-floating.cpp expressions/constant-expressions/case-label-not-constant.cpp

An array bound, an enumerator value, and a case label must be constant
expressions (CE-11). A value of floating-point type, or a value that is
not known during translation, is not valid there.
:::

:::{rule} CE-12
:kind: reject
:tests: expressions/constant-expressions/volatile-read.cpp

Constant evaluation must not read an object through a `volatile` lvalue.
:::

:::{rule} CE-13
:kind: reject
:tests: expressions/constant-expressions/string-literal-identity.cpp

Constant evaluation must not compare the addresses of two string literals
for equality. It is unspecified if two equal literals share storage.
:::

:::{rule} CE-14
:kind: reject
:tests: expressions/constant-expressions/result-points-to-local.cpp expressions/constant-expressions/result-points-to-static.cpp expressions/constant-expressions/result-refers-to-temporary.cpp expressions/constant-expressions/result-refers-to-static-temporary.cpp

A pointer or reference that is the value of a constant expression must
not refer to an object with automatic storage duration or to a temporary
object. It can refer to an object with static storage duration. A
temporary bound to a reference at namespace scope lives until the
program ends (REF-4) and counts as static, so such a reference can be
a constant.
:::

:::{rule} CE-15
:kind: reject
:tests: expressions/constant-expressions/relational-unrelated-pointers.cpp

Constant evaluation must not compare pointers to unrelated objects with
`<`, `>`, `<=`, or `>=`, because the result is unspecified.
:::

:::{rule} CE-16
:kind: reject
:tests: expressions/constant-expressions/call-before-definition.cpp

If a `constexpr` function has only a declaration at the point where
constant evaluation calls it, the call is not a constant expression. A
definition later in the translation unit does not help.
:::

:::{rule} CE-17
:kind: required
:tests: expressions/constant-expressions/static-local-not-reached.cpp expressions/constant-expressions/goto-not-reached.cpp expressions/constant-expressions/nonliteral-variable-not-reached.cpp expressions/constant-expressions/literal-not-reached.cpp

A `constexpr` function can contain these:

- a `goto`,
- a `static` or `thread_local` variable,
- a variable of a type that is not a {term}`literal type`.

CE-20 rejects a call only if the evaluation reaches one of them.
:::

:::{rule} CE-20
:kind: reject
:tests: expressions/constant-expressions/static-local-reached.cpp expressions/constant-expressions/goto-reached.cpp

Constant evaluation must not reach any of these (CE-17):

- a `goto`,
- a `static` or `thread_local` variable,
- a variable of a type that is not a {term}`literal type`.
:::

:::{rule} CE-21
:kind: reject
:tests: expressions/constant-expressions/result-unset-member.cpp expressions/constant-expressions/result-unset-array-element.cpp expressions/constant-expressions/result-every-part-set.cpp expressions/constant-expressions/result-union-no-active-member.cpp expressions/constant-expressions/result-union-one-active-member.cpp

The value of a constant expression of class or array type must give a
value to each of its scalar parts. These conditions apply:

- Each element of an array member is a part.
- In a union, only the active member is a part. A union with no active
  member is valid ({question}`Q-0117`).
- The result of a {term}`constant initializer` must meet the same
  condition.
:::

:::{rule} CE-22
:kind: reject
:tests: expressions/constant-expressions/copy-unset-member.cpp expressions/constant-expressions/copy-unset-array-element.cpp expressions/constant-expressions/unset-member-not-copied.cpp expressions/constant-expressions/copy-after-every-part-set.cpp

During constant evaluation, an implicit copy or move function
({rule}`CPY-8`) reads each scalar part of the source object. A
user-provided copy or move function reads only what its body reads.
Constant evaluation must not use an implicit copy or move function on
an object that has a scalar part with no value (OBJ-2), including the
move of a returned local variable ({rule}`CPY-12`).
:::

:::{rule} CE-23
:kind: required
:tests: expressions/constant-expressions/literal-destructor.cpp expressions/constant-expressions/literal-member.cpp expressions/constant-expressions/literal-base.cpp expressions/constant-expressions/literal-volatile-member.cpp expressions/constant-expressions/literal-union-volatile-member.cpp expressions/constant-expressions/literal-union-all-volatile.cpp expressions/constant-expressions/literal-constexpr-destructor.cpp expressions/constant-expressions/literal-closure.cpp expressions/constant-expressions/literal-not-reached.cpp

A *literal type* is one of these:

- a scalar type or a reference type,
- an array of a literal type,
- the closure type of a lambda whose captures have literal types,
- a class whose destructor is implicit, defaulted, or `constexpr`, and
  whose bases and non-static data members have literal types that are
  not `volatile`. A union can have `volatile` members if at least one
  member is not `volatile`.
:::

:::{rule} CE-24
:kind: required
:tests: expressions/constant-expressions/constant-initializer-usable.cpp expressions/constant-expressions/constant-initializer-not-constant.cpp

A *constant initializer* is an initializer whose whole initialization of
the variable, with each conversion and constructor call in it, is a
constant expression. For a variable with no initializer, its default
initialization is the initialization that this rule and CE-21 judge.
:::

:::{rule} CE-18
:kind: required
:tests: expressions/constant-expressions/unknown-reference-not-read.cpp

A reference whose object is not known during constant evaluation can
still occur in a constant expression if the evaluation does not read
through it. An example is a call of a `constexpr` member function that
does not use the object.
:::

## Implementation Requirements

:::{rule} CE-10
:kind: impl-defined
:tests: expressions/constant-expressions/deep-recursion.cpp expressions/constant-expressions/moderate-recursion.cpp expressions/constant-expressions/long-loop.cpp

An implementation sets limits on the depth of recursion and on the number
of steps in one constant evaluation. It rejects an evaluation that exceeds
a limit. The limits are implementation-defined. Every matrix compiler
accepts a recursion depth of 400, and rejects a depth of 100000 and a
loop of 100000000 steps.
:::

## Examples

```cpp
constexpr int square(int v) { return v * v; }
static_assert(square(4) == 16);       // fine

int plain();
constexpr int f() { return plain(); }
static_assert(f() == 1);              // rejected: plain is not constexpr

constexpr int g() {
  int* p = new int(5);
  int v = *p;
  delete p;                           // freed in the same evaluation
  return v;
}
static_assert(g() == 5);

constexpr int h(int v) {
  if (v < 0) throw v;
  return v;
}
static_assert(h(1) == 1);             // fine: the throw is not evaluated
static_assert(h(-1) == 0);            // rejected

constexpr int k(int v) {
  constexpr int w = v;                // rejected: a new evaluation starts
  return w;                           // here, and v is not known in it
}
constexpr const int& t = 4;           // fine: the temporary is static
void use() {
  constexpr const int& u = 4;         // rejected: the temporary is local
}
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 61 of 67 tests.

- CE-10: all three implementations reject a recursion depth of 100000 and
  a loop of 100000000 steps by default. All accept a depth of 400.

  - Clang stops at a depth of 512 calls and at its step limit
    (`-fconstexpr-depth`, `-fconstexpr-steps`).
  - GCC stops at a depth of 512 and at 262144 loop iterations
    (`-fconstexpr-depth`, `-fconstexpr-loop-limit`,
    `-fconstexpr-ops-limit`).
- CE-12: Clang 18 also reports the `constexpr` function as never constant
  (compare {question}`Q-0020`).
- CE-13: Clang rejects the comparison of two string literal addresses.
  GCC and EDG accept it and fold it to a value ({question}`Q-0033`).
- CE-11: EDG accepts `explicit(2)` and converts the condition to `bool`
  ({question}`Q-0078`).
- CE-18: Clang 18 rejects the use of the reference. Clang 22, GCC, and
  EDG accept it ({question}`Q-0090`).
- CE-21: every compiler rejects a constant whose value has a scalar part
  with no value, for `constexpr` and `constinit` alike.
- CE-21: a union with no active member is a valid constant for Clang
  and EDG. GCC rejects it ({question}`Q-0117`).
- CE-22: Clang and EDG reject a copy of an object that has a part with
  no value, and name the part in a note. GCC 14 and 16 accept the copy
  ({question}`Q-0115`).
- CE-23: Clang and GCC reject a `constexpr` variable of a type that is
  not literal and name the condition that fails. EDG rejects the same
  programs, reporting the call of a destructor that is not `constexpr`
  or a type that is not literal. CE-23 lists only conditions that a test
  can observe.

### Undefined rules

The table lists each rule of class `undefined` and the compilers whose
constant evaluation rejects it (CE-9). It is generated from the test
expectations, which the matrix checks. Each test of a rule that expects
a rejection from some compiler is one case. Each entry is one of these:

- "rejects": the compiler rejects every case.
- "accepts": the compiler rejects no case.
- "some cases": the compiler rejects some cases and not others.

Where the versions of a compiler differ, the entry names each version.
"no test" means that the rule has no case.

:::{tidal-undefined-index}
:::

Inputs: clang-sema-expr, clang-diag-sema.

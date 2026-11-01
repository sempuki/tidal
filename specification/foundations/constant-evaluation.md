# Constant expressions

A constant expression is an expression that the implementation evaluates
during translation. {term}`Constant evaluation <constant evaluation>`
permits most operations. This page lists the operations that constant
evaluation rejects.

## Principles

:::{principle} CE-P1
:rules: CE-1 CE-17 CE-18 CE-20 CE-23

Constant evaluation is the same program run at translation time
({principle}`TR-P4`). Within one evaluation it can do what a run can do:

- arithmetic,
- loops,
- calls and virtual calls,
- changes to the objects that the evaluation has made,
- allocation.

Constant evaluation rejects a construct that it cannot handle only when
an evaluation reaches that construct. So a function can contain such a
construct and serve both modes. A reference whose object the evaluation
does not know is a problem only when the evaluation reads through the
reference. The objects that an evaluation makes need a
{term}`literal type`, one whose objects the evaluation can build and
destroy.
:::

:::{principle} CE-P2
:rules: CE-2 CE-3 CE-5 CE-7 CE-14 CE-16 CE-21 CE-24 CE-29

An evaluation is closed and can use only what translation already
knows:

- functions declared `constexpr` or `consteval` and defined before the
  evaluation,
- variables whose values are fixed at translation time because they
  have a {term}`constant initializer`,
- `const` temporaries that such a variable keeps alive,
- objects that the evaluation itself created.

The evaluation can change only its own objects, because anything else
exists at run time. The result must outlive the evaluation, so the
result cannot be a pointer to an automatic or temporary object, and
storage that the evaluation allocated must be freed before the
evaluation ends. The result must also be complete, with a value in
every part.
:::

:::{principle} CE-P3
:rules: CE-4 CE-6 CE-8 CE-9 CE-12 CE-13 CE-15 CE-22 CE-27 CE-28
:exceptions: CE-26

An evaluation must not depend on the machine or on where objects are
placed in memory. So an evaluation rejects these operations:

- reinterpreting a pointer,
- reading a volatile object,
- reading a union member that is not active,
- comparing the addresses of string literals,
- ordering pointers to unrelated objects,
- dividing a floating-point value by zero, which gives an infinity or a
  NaN only on some machines,
- any operation that is undefined.

Copying an object reads each of its parts, so a copy of an object with
a part that has no value is undefined too. An exception has no handler
at translation time, so a `throw` that the evaluation reaches ends the
evaluation.

Where the language leaves an order of evaluation to the implementation,
an evaluation picks an order, as a run does. The order of an evaluation
need not match the order of a run. So two implementations can give
different values to one constant, and so can one implementation at
translation time and at run time. An implementation need not detect two
unsequenced accesses to one object, which are undefined.
:::

:::{principle} CE-P4
:rules: CE-11 CE-19 CE-25

Where the language needs a constant to build the program, it needs the
exact value. These places need an exact value:

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

An evaluation runs on the translator, so an evaluation has limits that
a run does not. Each implementation sets and documents its own limits on
the depth of recursion and on the number of steps.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   constant_expr: `conditional_expr`
```

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
:tests: expressions/constant-expressions/read-non-const-variable.cpp expressions/constant-expressions/read-const-runtime-init.cpp expressions/constant-expressions/read-const-constant-init.cpp expressions/constant-expressions/read-parameter-outside.cpp expressions/constant-expressions/const-enum-usable.cpp expressions/constant-expressions/const-volatile-not-usable.cpp expressions/constant-expressions/read-extended-temporary-not-const.cpp expressions/constant-expressions/read-extended-temporary-class.cpp expressions/constant-expressions/read-extended-temporary-nested.cpp

Constant evaluation must not read the value of a variable that is not
usable in constant expressions. A variable is usable if it is
`constexpr`, or if it is a `const` variable of integer or enumeration
type with a {term}`constant initializer` (CV-8). A `const volatile`
variable is not usable. A function parameter is usable only during an
evaluation that called the function. A temporary that the initializer of
a `constexpr` reference extends (CE-14) is usable if the type of the
temporary is `const` ({question}`Q-0193`).
:::

:::{rule} CE-29
:kind: reject
:tests: expressions/constant-expressions/reference-runtime-initializer.cpp expressions/constant-expressions/reference-constant-initializer.cpp

A reference variable with a {term}`constant initializer` is usable in
constant expressions, also if the reference is not `constexpr`.
Constant evaluation must not use a reference variable to name its
object if the reference is not usable and the evaluation did not
initialize it. {rule}`CE-18` gives the case of a reference that the
evaluation does not read through.
:::

:::{rule} CE-4
:kind: reject
:tests: expressions/constant-expressions/reinterpret-cast.cpp expressions/constant-expressions/void-pointer-to-object.cpp

Constant evaluation must not contain a `reinterpret_cast` or convert a
pointer to `void` to a pointer to an object type.
:::

:::{rule} CE-5
:kind: reject
:tests: expressions/constant-expressions/modify-outside-object.cpp expressions/constant-expressions/modify-local-object.cpp

Constant evaluation must not change an object that existed before the
evaluation started. The evaluation can change the objects that it
created.
:::

:::{rule} CE-6
:kind: reject
:tests: expressions/constant-expressions/inactive-union-member.cpp expressions/constant-expressions/active-union-member.cpp

Constant evaluation must not read a union member that is not the active
member ({rule}`UNI-6`). An assignment to a member makes that member
active ({rule}`UNI-5`).
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
:tests: expressions/constant-expressions/throw-evaluated.cpp expressions/constant-expressions/throw-not-evaluated.cpp exceptions/throw/constant-evaluation-throw-rejected.cpp exceptions/throw/constant-evaluation-throw-not-taken.cpp classes/virtual/dynamic-cast-reference-fails-constant.cpp

Constant evaluation must not evaluate a throw-expression, or a failed
`dynamic_cast` of a reference, which throws ({rule}`VIRT-10`). A
throw-expression on a path that the evaluation does not take is valid.
:::

:::{rule} CE-9
:kind: reject
:tests: expressions/constant-expressions/undefined-behavior.cpp expressions/constant-expressions/undefined-behavior-dangling.cpp

Constant evaluation must not evaluate an operation that has undefined
behavior. Each rule of class `undefined` gives such an operation. CE-26
gives the operations that an implementation need not detect. The
Evidence segment lists the `undefined` rules and the compilers that
reject each rule during constant evaluation.
:::

:::{rule} CE-11
:kind: required
:tests: expressions/constant-expressions/integral-constant-array-bound.cpp

Each of the following needs a constant expression of integer or
enumeration type:

- an array bound,
- an enumerator value,
- a case label.
:::

:::{rule} CE-19
:kind: reject
:tests: expressions/constant-expressions/converted-constant-narrowing.cpp expressions/constant-expressions/explicit-condition-narrowing.cpp

Where a constant converts to another type for a use in CE-11, the
conversion must not narrow. The condition of `if constexpr` is an
exception (IF-16). The same condition applies to:

- the condition of `explicit(...)`,
- the operand of a `noexcept` specifier (EXS-1),
- a template argument for a constant parameter ({rule}`TARG-13`).
:::

:::{rule} CE-25
:kind: reject
:tests: expressions/constant-expressions/array-bound-floating.cpp expressions/constant-expressions/array-bound-not-constant.cpp expressions/constant-expressions/enumerator-floating.cpp expressions/constant-expressions/case-label-floating.cpp expressions/constant-expressions/case-label-not-constant.cpp

These must be constant expressions (CE-11):

- an array bound, which also follows {rule}`DCL-19`,
- an enumerator value,
- a case label, which also follows {rule}`SW-3`.

A value of floating-point type, or a value that is not known during
translation, is not valid in these places.
:::

:::{rule} CE-12
:kind: reject
:tests: expressions/constant-expressions/volatile-read.cpp

Constant evaluation must not read an object through a `volatile` lvalue.
:::

:::{rule} CE-13
:kind: reject
:tests: expressions/constant-expressions/string-literal-identity.cpp

Constant evaluation must not compare the addresses of two string
literals for equality. Two equal literals can share storage or not, and
the choice is unspecified.
:::

:::{rule} CE-14
:kind: reject
:tests: expressions/constant-expressions/result-points-to-local.cpp expressions/constant-expressions/result-points-to-static.cpp expressions/constant-expressions/result-refers-to-temporary.cpp expressions/constant-expressions/result-refers-to-static-temporary.cpp expressions/constant-expressions/extended-temporary-through-cast.cpp expressions/constant-expressions/extended-temporary-static-local.cpp expressions/constant-expressions/extended-temporary-member.cpp expressions/constant-expressions/extended-temporary-nested.cpp expressions/constant-expressions/read-extended-temporary-class.cpp expressions/constant-expressions/read-extended-temporary-nested.cpp

A pointer or reference that is the value of a constant expression must
not refer to an object with automatic storage duration or to a temporary
object. The pointer or reference can refer to an object with static
storage duration. A temporary that a reference with static storage
duration extends, by any form of binding in {rule}`REF-4`, counts as
static ({question}`Q-0193`).
:::

:::{rule} CE-15
:kind: reject
:tests: expressions/constant-expressions/relational-unrelated-pointers.cpp

Constant evaluation must not compare pointers to unrelated objects with
`<`, `>`, `<=`, or `>=`, because the result is unspecified.
:::

:::{rule} CE-28
:kind: reject
:tests: expressions/multiplicative/floating-divide-by-zero.cpp expressions/multiplicative/floating-zero-by-zero.cpp

Constant evaluation must not evaluate a floating-point `/` whose right
operand is zero. The rule holds also for a type that follows IEEE 754,
where the division has a result at run time ({rule}`MUL-11`).
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

A `constexpr` function can contain these constructs:

- a `goto`,
- a `static` or `thread_local` variable,
- a variable of a type that is not a {term}`literal type`.

CE-20 rejects a call only if the evaluation reaches one of these
constructs.
:::

:::{rule} CE-20
:kind: reject
:tests: expressions/constant-expressions/static-local-reached.cpp expressions/constant-expressions/goto-reached.cpp expressions/constant-expressions/static-local-not-usable-reached.cpp expressions/constant-expressions/static-local-usable-reached.cpp

Constant evaluation must not reach any of these constructs (CE-17):

- a `goto`,
- a `static` or `thread_local` variable that is not usable in constant
  expressions (CE-3),
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
- For a variable with static storage, the zero that {rule}`STO-3`
  writes first does not count. A part that the initialization leaves
  without a value has no value.
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

A *literal type* is one of these types:

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
:tests: expressions/constant-expressions/constant-initializer-usable.cpp expressions/constant-expressions/constant-initializer-not-constant.cpp expressions/constant-expressions/constant-initializer-zero-scalar.cpp expressions/constant-expressions/constant-initializer-zero-trivial-class.cpp

A *constant initializer* is an initializer whose whole initialization of
the variable, with each conversion and constructor call in it, is a
constant expression. For a variable with no initializer, the default
initialization of the variable is the initialization that this rule and
CE-21 judge. A default initialization that calls no constructor, or only
a {term}`trivial` one, leaves the zero of {rule}`STO-3` as a constant
initializer ({question}`Q-0192`).
:::

:::{rule} CE-18
:kind: required
:tests: expressions/constant-expressions/unknown-reference-not-read.cpp

A reference whose object is not known during constant evaluation can
still occur in a constant expression if the evaluation does not read
through the reference. An example is a call of a `constexpr` member
function that does not use the object.
:::

## Implementation Requirements

:::{rule} CE-10
:kind: impl-defined
:tests: expressions/constant-expressions/deep-recursion.cpp expressions/constant-expressions/moderate-recursion.cpp expressions/constant-expressions/long-loop.cpp

An implementation sets limits on the depth of recursion and on the
number of steps in one constant evaluation. The implementation rejects
an evaluation that exceeds a limit. The limits are
implementation-defined. Every matrix compiler accepts a recursion depth
of 400, and rejects a depth of 100000 and a loop of 100000000 steps.
:::

:::{rule} CE-26
:kind: no-diagnostic
:tests: expressions/constant-expressions/unsequenced-not-detected.cpp

Constant evaluation must not evaluate two {term}`unsequenced` accesses
to one memory location where one access writes ({rule}`SEQ-4`,
{rule}`MUL-14`). An implementation need not diagnose such a pair.
:::

:::{rule} CE-27
:kind: unspecified
:tests: expressions/constant-expressions/order-may-differ-from-run.cpp

Where the order of evaluation is unspecified ({rule}`SEQ-1`,
{rule}`SEQ-8`, {rule}`ASN-11`), constant evaluation also uses an
unspecified order. The order need not be the one that the same
implementation uses at run time for the same expression.
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
constexpr int&& m = 4;                // fine: the temporary is static
static_assert(m == 4);                // rejected: the temporary is not const
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 79
of 92 tests.

- CE-10: all three implementations reject a recursion depth of 100000
  and a loop of 100000000 steps by default. All accept a depth of 400.

  - Clang stops at a depth of 512 calls and at its step limit
    (`-fconstexpr-depth`, `-fconstexpr-steps`).
  - GCC stops at a depth of 512 and at 262144 loop iterations
    (`-fconstexpr-depth`, `-fconstexpr-loop-limit`,
    `-fconstexpr-ops-limit`).
- CE-12: Clang 18 also reports the `constexpr` function as never
  constant (compare {question}`Q-0020`).
- CE-13: Clang rejects the comparison of two string literal addresses.
  GCC and EDG accept it and fold it to a value ({question}`Q-0033`).
- CE-11: EDG accepts `explicit(2)` and converts the condition to `bool`
  ({question}`Q-0078`).
- CE-18: Clang 18 rejects the use of the reference. Clang 22, GCC, and
  EDG accept it ({question}`Q-0090`).
- CE-29: every compiler accepts `&alias` in a constant expression and
  `alias` as a template argument, where `alias` is a reference to a
  global with a constant initializer and is not `constexpr`. Every
  compiler rejects a reference whose initializer calls a function that
  is not `constexpr`.
- CE-20: every compiler accepts an evaluation that reaches a `static`
  or `thread_local` variable that is usable in constant expressions:
  a `constexpr` variable, or a `const` integer with a constant
  initializer. Every compiler rejects one that reaches a `static const
  double`.
- CE-21: every compiler rejects a constant whose value has a scalar part
  with no value, for `constexpr` and `constinit` alike.
- CE-21: a union with no active member is a valid constant for Clang
  and EDG. GCC rejects it ({question}`Q-0117`).
- CE-22: Clang and EDG reject a copy of an object that has a part with
  no value, and name the part in a note. GCC 14 and 16 accept the copy
  ({question}`Q-0115`). At run time the same copy is a read under
  {rule}`OBJ-2`.
- CE-28: Clang and GCC reject `1.0 / 0.0` and `0.0 / 0.0` and note a
  division by zero, although both give an infinity or a NaN at run time
  ({question}`Q-0004`). EDG 7.0 accepts both ({question}`Q-0002`).
- CE-3 and CE-14: the compilers split on temporaries that a namespace
  `constexpr` reference extends ({question}`Q-0193`).
  - EDG rejects the reference when the temporary is extended through a
    member access or through a reference member of a braced aggregate.
    Clang and GCC accept both references.
  - GCC rejects a read of a `const` temporary of class type. Clang and
    EDG accept the read. Every compiler accepts the read of a `const`
    scalar temporary and rejects the read of a temporary that is not
    `const`.
- CE-24: every compiler accepts `constinit` on a scalar or an array of
  scalars with no initializer. For a class with a trivial default
  constructor, GCC and EDG accept it, and Clang rejects it
  ({question}`Q-0192`).
- CE-26: all eight compilers accept constant evaluation of
  `i++ + i++`, `i + i++`, and `i++ * i`. CE-26 is class `no-diagnostic`
  because no compiler detects the pair. So the table below shows "no
  test" for SEQ-4 and MUL-14.
- CE-27: Clang and EDG evaluate the arguments of a call left to right in
  constant evaluation. GCC also evaluates them left to right in constant
  evaluation, and right to left at run time. CE-27 allows both results.
- CE-23: Clang and GCC reject a `constexpr` variable of a type that is
  not literal and name the condition that fails. EDG rejects the same
  programs, reporting the call of a destructor that is not `constexpr`
  or a type that is not literal. CE-23 lists only conditions that a test
  can observe.

### Undefined rules

The table lists each rule of class `undefined` and the compilers whose
constant evaluation rejects it (CE-9). The table is generated from the
test expectations, which the matrix checks. Each test of a rule that
expects a rejection from some compiler is one case. Each entry is one of
these:

- "rejects": the compiler rejects every case.
- "accepts": the compiler rejects no case.
- "some cases": the compiler rejects some cases and not others.

Where the versions of a compiler differ, the entry names each version.
"no test" means that the rule has no case.

:::{tidal-undefined-index}
:::

Inputs: clang-sema-expr, clang-diag-sema.

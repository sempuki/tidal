# Constant expressions

A constant expression is an expression that the implementation evaluates
during translation. {term}`Constant evaluation <constant evaluation>`
permits most operations. This page lists the operations that constant
evaluation rejects.

## Principles

:::{principle} CE-P1
:rules: CE-1 CE-17 CE-18 CE-23

Constant evaluation is the same program run at translation time
({principle}`TR-P4`). Within one evaluation it can do what a run can do:

- arithmetic,
- loops,
- calls and virtual calls,
- changes to the objects that the evaluation has made,
- allocation.

Constant evaluation rejects a construct that it cannot handle only when
an evaluation reaches that construct. So a function can contain such a
construct and serve both modes. A `constexpr` function that no
evaluation can complete is valid too ({rule}`CX-3`), and only a call
that an evaluation reaches is rejected. A reference whose object the evaluation
does not know is a problem only when the evaluation reads through the
reference. Only a `constexpr` variable needs a {term}`literal type`
({rule}`CX-1`), because its value outlives the evaluation. A variable or
a temporary that lives only inside the evaluation can have any type,
and the evaluation fails only if it reaches an operation that it cannot
do, such as a read of a volatile member or a call of a function that is
not `constexpr` ({question}`Q-0335`).
:::

:::{principle} CE-P2
:rules: CE-2 CE-3 CE-5 CE-7 CE-14 CE-16 CE-21 CE-24

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
result cannot be a pointer to an automatic or {term}`temporary object`, and
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

A read of a scalar that has no value is rejected, also where a run
permits the read, as in a copy into `unsigned char`. Copying an object
reads each of its parts, so a copy of an object with a part that has no
value is rejected too. An exception has no {term}`handler` at translation time,
so a `throw` that the evaluation reaches ends the evaluation. For the
same reason an evaluation never reaches a point where the program would
terminate.

Where the language leaves an order of evaluation to the implementation,
an evaluation picks an order, as a run does. The order of an evaluation
need not match the order of a run. So two implementations can give
different values to one constant, and so can one implementation at
translation time and at run time. An implementation need not detect two
unsequenced accesses to one object, which are undefined.
:::

:::{principle} CE-P4
:rules: CE-11 CE-19 CE-30

Where the language needs a constant to build the program, it needs the
exact value. In most such places, a {term}`narrowing conversion` is not
valid. A condition that only needs a yes or no permits such a
conversion:

| Place | Value | Narrowing conversion | Rules |
|---|---|---|---|
| an array bound | integer or enumeration | rejected | CE-11, CE-19, {rule}`DCL-19` |
| an enumerator value | integer or enumeration | rejected | CE-11, CE-19, {rule}`ENUM-17` |
| a case label | integer or enumeration | rejected | CE-11, CE-19, {rule}`SW-3` |
| a constant {term}`template argument` | the parameter's type | rejected | CE-19, {rule}`TARG-13` |
| the operand of `explicit(...)` | `bool` | rejected | CE-19 |
| the operand of a `noexcept` specifier | `bool` | rejected | CE-19, {rule}`EXS-1` |
| the condition of `if constexpr` | `bool` | permitted | {rule}`IF-16` |
| the condition of `static_assert` | `bool` | permitted | {rule}`SA-3` |
| the operand of `alignas` | integer or unscoped enumeration, zero or a power of two | | {rule}`ALN-2`, {rule}`ALN-8` |
| the width of a bit-field | integer or unscoped enumeration | | {rule}`BF-1`, {rule}`BF-2` |
| the initializer of a `constexpr` variable | the variable's type | as in any initialization | {rule}`CX-1` |
| the initializer of a `constinit` variable | the variable's type | as in any initialization | {rule}`CINIT-1` |
| a call to a `consteval` function, outside an {term}`immediate function context` | the function's return type | as in any call | {rule}`CEV-1` |

Each of these places starts a new evaluation, also inside a function
that an evaluation runs. So the parameters of that function are unknown
there (CE-30).
:::

:::{principle} CE-P5
:rules: CE-10

An evaluation runs on the translator, so an evaluation has limits that
a run does not. Each implementation sets and documents its own limits on
the depth of recursion and on the number of steps. These limits are
among the limits of translation that {principle}`TR-P6` lists.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   constant_expr: `conditional_expr`
```

## Constraints

:::{rule} CE-1
:kind: required
:tests: foundations/constant-evaluation/arithmetic-and-calls.cpp foundations/constant-evaluation/lambda-call.cpp foundations/constant-evaluation/virtual-call.cpp foundations/constant-evaluation/try-without-throw.cpp foundations/constant-evaluation/new-delete-transient.cpp foundations/constant-evaluation/function-pointer-equality.cpp foundations/constant-evaluation/call-through-function-pointer.cpp expressions/member-pointers/compare-equal-constant.cpp

Constant evaluation can contain these operations:

- arithmetic, loops, and calls to `constexpr` functions, also through a
  function pointer,
- changes to objects that the same evaluation created,
- calls to lambdas and to `constexpr` virtual functions,
- `try` blocks, if no exception is thrown,
- `new` and `delete` expressions, as CE-7 describes,
- equality comparison of function pointers and of pointers to members.
:::

:::{rule} CE-2
:kind: reject
:tests: foundations/constant-evaluation/non-constexpr-call.cpp foundations/constant-evaluation/implicit-constructor-calls-constexpr.cpp foundations/constant-evaluation/implicit-constructor-member-not-constexpr.cpp foundations/constant-evaluation/defaulted-constructor-member-not-constexpr.cpp foundations/constant-evaluation/implicit-destructor-member-not-constexpr.cpp foundations/constant-evaluation/nonliteral-variable-destructor-called.cpp foundations/constant-evaluation/nonliteral-temporary-destructor-called.cpp foundations/constant-evaluation/implicit-assignment-constant.cpp foundations/constant-evaluation/implicit-assignment-member-not-constexpr.cpp foundations/constant-evaluation/implicit-constructor-virtual-base.cpp foundations/constant-evaluation/defaulted-constructor-virtual-base.cpp foundations/constant-evaluation/implicit-copy-virtual-base.cpp foundations/constant-evaluation/virtual-base-path-not-reached.cpp foundations/constant-evaluation/call-through-function-pointer-not-constexpr.cpp classes/virtual/virtual-call-override-not-constexpr.cpp declarations/constexpr/function-not-constant-call.cpp

During constant evaluation, the program can call only `constexpr` and
`consteval` functions. An implicit or defaulted constructor,
destructor, or assignment operator counts as `constexpr`, except a
constructor or destructor of a class with a {term}`virtual base`
({rule}`DER-12`). Each function that it calls for a base or a member
must also obey this rule.
:::

:::{rule} CE-3
:kind: reject
:tests: foundations/constant-evaluation/read-non-const-variable.cpp foundations/constant-evaluation/read-const-runtime-init.cpp foundations/constant-evaluation/read-const-constant-init.cpp foundations/constant-evaluation/read-parameter-outside.cpp foundations/constant-evaluation/const-enum-usable.cpp foundations/constant-evaluation/const-volatile-not-usable.cpp foundations/constant-evaluation/read-extended-temporary-not-const.cpp foundations/constant-evaluation/read-extended-temporary-class.cpp foundations/constant-evaluation/read-extended-temporary-nested.cpp foundations/constant-evaluation/reference-reads-constant-object.cpp foundations/constant-evaluation/parameter-if-constexpr-consteval.cpp foundations/constant-evaluation/parameter-nested-call.cpp foundations/constant-evaluation/parameter-static-assert.cpp foundations/constant-evaluation/parameter-template-argument-consteval.cpp foundations/constant-evaluation/reference-runtime-initializer.cpp foundations/constant-evaluation/reference-constant-initializer.cpp foundations/constant-evaluation/reference-to-local-not-usable.cpp

Constant evaluation must not do either of these with a variable that
the evaluation did not create, unless the variable is
{term}`usable in constant expressions`:

- read the value of the variable,
- use a reference variable to name its object.

These variables are usable:

- a `constexpr` variable,
- a `const` variable of integer or enumeration type with a
  {term}`constant initializer` (CV-8), if the variable is not also
  `volatile`,
- a reference variable with a constant initializer, also if the
  reference is not `constexpr`. A reference bound to an object with
  automatic storage, or to a temporary that does not get static
  storage, has no constant initializer, because a constant expression
  cannot refer to such an object (CE-14),
- a `const` temporary that the initializer of a `constexpr` reference
  extends (CE-14, {question}`Q-0193`).

A function parameter is usable only during an evaluation that called
the function, and {rule}`CE-30` gives the places where a new
evaluation starts. {rule}`CE-18` gives the case of a reference that
the evaluation does not read through.
:::

:::{rule} CE-30
:kind: reject
:tests: foundations/constant-evaluation/read-parameter-outside.cpp foundations/constant-evaluation/parameter-template-argument-consteval.cpp foundations/constant-evaluation/parameter-if-constexpr-consteval.cpp foundations/constant-evaluation/parameter-static-assert.cpp foundations/constant-evaluation/parameter-nested-call.cpp foundations/constant-evaluation/try-around-constexpr-variable.cpp foundations/constant-evaluation/try-around-consteval-call.cpp

Each place in the table of {principle}`CE-P4` starts its own constant
evaluation where the place occurs. This holds also in the body of a
`constexpr` or `consteval` function. The new evaluation must not read a
parameter of that function (CE-3), because no call of the function is
part of the new evaluation. A try block around the place is not part of
the new evaluation either, so a throw in the new evaluation is rejected
by {rule}`CE-8`.
:::

:::{rule} CE-4
:kind: reject
:tests: foundations/constant-evaluation/reinterpret-cast.cpp foundations/constant-evaluation/void-pointer-to-object.cpp foundations/language-support/allocator-self-declared-constant.cpp foundations/constant-evaluation/allocator-result-other-type.cpp expressions/casts/static-cast-void-pointer-round-trip-constant.cpp

Constant evaluation must not contain a `reinterpret_cast` or convert a
pointer to `void` to a pointer to an object type. Inside a member of
`std::allocator<T>`, the result of a call of `::operator new` by name
({rule}`LSUP-11`) can be converted to `T*` ({rule}`STG-28`,
{question}`Q-0349`).
:::

:::{rule} CE-5
:kind: reject
:tests: foundations/constant-evaluation/modify-outside-object.cpp foundations/constant-evaluation/modify-local-object.cpp foundations/language-support/construct-at-object-outside-evaluation.cpp

Constant evaluation must not change an object that existed before the
evaluation started. The evaluation can change the objects that it
created.
:::

:::{rule} CE-6
:kind: reject
:tests: foundations/constant-evaluation/inactive-union-member.cpp foundations/constant-evaluation/active-union-member.cpp classes/unions/construct-activates-constant.cpp classes/unions/construct-in-member-of-inactive-member-union-constant.cpp

Constant evaluation must not read a union member that is not the active
member ({rule}`UNI-6`). Each of these makes a member active:

- an assignment to the member ({rule}`UNI-5`),
- the creation of an object of the member's type at the address of the
  member ({rule}`UNI-14`).
:::

:::{rule} CE-7
:kind: reject
:tests: foundations/constant-evaluation/allocation-not-freed.cpp foundations/constant-evaluation/allocation-in-result.cpp foundations/constant-evaluation/delete-mismatch.cpp foundations/constant-evaluation/double-delete.cpp

Storage that a `new` expression gets during constant evaluation must be
freed in the same evaluation. These conditions apply:

- The evaluation must free the storage exactly once.
- `delete[]` must free storage from `new[]`, and `delete` must free
  storage from `new`.
- The result of the evaluation must not point to that storage.
:::

:::{rule} CE-8
:kind: reject
:tests: foundations/constant-evaluation/throw-evaluated.cpp foundations/constant-evaluation/throw-not-evaluated.cpp exceptions/throw/constant-evaluation-throw-rejected.cpp exceptions/throw/constant-evaluation-throw-not-taken.cpp classes/virtual/dynamic-cast-reference-fails-constant.cpp foundations/constant-evaluation/termination-noexcept-rejected.cpp foundations/constant-evaluation/termination-rethrow-rejected.cpp foundations/constant-evaluation/rethrow-not-reached.cpp declarations/constexpr/consteval-converting-constructor-throws.cpp expressions/new-delete/bad-bound-constant-negative.cpp expressions/new-delete/bad-bound-constant-too-few.cpp foundations/constant-evaluation/try-around-constexpr-variable.cpp foundations/constant-evaluation/try-around-consteval-call.cpp foundations/constant-evaluation/termination-not-reached.cpp

Constant evaluation must not evaluate any of these operations, each of
which throws:

- a throw-expression, also a rethrow,
- a failed `dynamic_cast` of a reference ({rule}`VIRT-10`),
- a `new` expression for an array with an erroneous bound
  ({rule}`NEW-39`).

A throw-expression on a path that the evaluation does not take is
valid. A rethrow that the evaluation reaches is rejected. No exception
is ever handled during translation, so the rethrow is the case where a
run would terminate ({rule}`UNW-5`). Each point where the program
terminates follows a throw, so constant evaluation never reaches such a
point.
:::

:::{rule} CE-9
:kind: reject
:tests: foundations/constant-evaluation/undefined-behavior.cpp foundations/constant-evaluation/undefined-behavior-dangling.cpp expressions/postfix/member-call-null-pointer-constant.cpp templates/deduction/substitution-failure-undefined-operation.cpp classes/constructors/reference-default-member-read-constant.cpp foundations/storage/allocation-assign-before-construction-int-element.cpp foundations/storage/allocation-assign-before-construction-pod-member.cpp foundations/storage/allocation-assign-before-construction-double-element.cpp foundations/storage/allocation-assign-before-construction-user-assignment.cpp

Constant evaluation must not evaluate an operation that has undefined
behavior. Each rule of class `undefined` gives such an operation. CE-26
gives the operations that an implementation need not detect. A read of
an object after its lifetime ends ({rule}`OBJ-2`) is not one of those
operations, so constant evaluation rejects it. The read is rejected
also through a pointer to a local of a function that has returned, or
through a reference member that a default member initializer bound to a
temporary ({rule}`CTOR-17`). The
Evidence segment lists the `undefined` rules and the compilers that
reject each rule during constant evaluation.
:::

:::{rule} CE-11
:kind: reject
:tests: foundations/constant-evaluation/integral-constant-array-bound.cpp foundations/constant-evaluation/array-bound-floating.cpp foundations/constant-evaluation/array-bound-not-constant.cpp foundations/constant-evaluation/enumerator-floating.cpp foundations/constant-evaluation/case-label-floating.cpp foundations/constant-evaluation/case-label-not-constant.cpp

Each of these must be a constant expression of integer or enumeration
type:

- an array bound, which also follows {rule}`DCL-19`,
- an enumerator value,
- a case label, which also follows {rule}`SW-3`.

A value of floating-point type, or a value that is not known during
translation, is not valid in these places.
:::

:::{rule} CE-19
:kind: reject
:tests: foundations/constant-evaluation/converted-constant-narrowing.cpp foundations/constant-evaluation/explicit-condition-narrowing.cpp

Where a constant converts to another type for a use in CE-11, the
conversion must not be a {term}`narrowing conversion`. The condition of `if constexpr` is an
exception (IF-16). The same condition applies to:

- the condition of `explicit(...)`,
- the operand of a `noexcept` specifier (EXS-1),
- a template argument for a {term}`constant parameter` ({rule}`TARG-13`).
:::

:::{rule} CE-12
:kind: reject
:tests: foundations/constant-evaluation/volatile-read.cpp foundations/constant-evaluation/volatile-reference-read.cpp

Constant evaluation must not read an object through a `volatile` lvalue.
:::

:::{rule} CE-13
:kind: reject
:tests: foundations/constant-evaluation/string-literal-identity.cpp foundations/constant-evaluation/string-literal-identity-variables.cpp

Constant evaluation must not compare the addresses of two string
literals for equality. Two equal literals can share storage or not, and
the choice is unspecified.
:::

:::{rule} CE-14
:kind: reject
:tests: foundations/constant-evaluation/result-points-to-local.cpp foundations/constant-evaluation/result-points-to-static.cpp foundations/constant-evaluation/result-refers-to-temporary.cpp foundations/constant-evaluation/result-refers-to-static-temporary.cpp foundations/constant-evaluation/extended-temporary-through-cast.cpp foundations/constant-evaluation/extended-temporary-static-local.cpp foundations/constant-evaluation/extended-temporary-member.cpp foundations/constant-evaluation/extended-temporary-nested.cpp foundations/constant-evaluation/read-extended-temporary-class.cpp foundations/constant-evaluation/read-extended-temporary-nested.cpp

A pointer or reference that is the value of a constant expression must
not refer to an object with {term}`automatic storage` duration or to a temporary
object. The pointer or reference can refer to an object with static
{term}`storage duration`. A temporary that a reference with static storage
duration extends, by any form of binding in {rule}`REF-4`, counts as
static ({question}`Q-0193`).
:::

:::{rule} CE-15
:kind: reject
:tests: foundations/constant-evaluation/relational-unrelated-pointers.cpp foundations/constant-evaluation/relational-unrelated-array-elements.cpp

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
:tests: foundations/constant-evaluation/call-before-definition.cpp foundations/constant-evaluation/member-call-before-definition.cpp

If a `constexpr` function has only a declaration at the point where
constant evaluation calls it, the call is not a constant expression. A
definition later in the {term}`translation unit` does not help.
:::

:::{rule} CE-17
:kind: reject
:tests: foundations/constant-evaluation/static-local-not-reached.cpp foundations/constant-evaluation/goto-not-reached.cpp foundations/constant-evaluation/nonliteral-variable-not-reached.cpp foundations/constant-evaluation/literal-not-reached.cpp foundations/constant-evaluation/static-local-reached.cpp foundations/constant-evaluation/goto-reached.cpp foundations/constant-evaluation/static-local-not-usable-reached.cpp foundations/constant-evaluation/static-local-usable-reached.cpp declarations/constexpr/function-static-local-evaluated.cpp foundations/constant-evaluation/literal-temporary-volatile-member.cpp foundations/constant-evaluation/nonliteral-variable-reached.cpp foundations/constant-evaluation/nonliteral-temporary-in-function.cpp foundations/constant-evaluation/nonliteral-variable-destructor-called.cpp foundations/constant-evaluation/nonliteral-temporary-destructor-called.cpp

Constant evaluation must not reach any of these constructs:

- a `goto`,
- a `static` or `thread_local` variable that is not usable in constant
  expressions (CE-3).

A variable or a temporary of a type that is not a {term}`literal type`
is not one of these constructs ({question}`Q-0335`). The end of such a
variable or temporary calls its destructor, and CE-2 applies to that
call, also for a destructor with an empty body.

A `constexpr` function can contain these constructs on a path that the
evaluation does not take.
:::

:::{rule} CE-21
:kind: reject
:tests: foundations/constant-evaluation/result-unset-member.cpp foundations/constant-evaluation/result-unset-array-element.cpp foundations/constant-evaluation/result-every-part-set.cpp foundations/constant-evaluation/result-returned-local-unset-element.cpp foundations/constant-evaluation/result-returned-local-every-element.cpp foundations/constant-evaluation/read-returned-local-unset-element.cpp foundations/constant-evaluation/result-union-no-active-member.cpp foundations/constant-evaluation/result-union-one-active-member.cpp foundations/constant-evaluation/constant-initializer-zero-trivial-class.cpp foundations/constant-evaluation/constant-initializer-trivial-class-block-static.cpp foundations/constant-evaluation/constant-initializer-trivial-class-array.cpp

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
:tests: foundations/constant-evaluation/copy-unset-member.cpp foundations/constant-evaluation/copy-unset-array-element.cpp foundations/constant-evaluation/unset-member-not-copied.cpp foundations/constant-evaluation/copy-after-every-part-set.cpp foundations/constant-evaluation/result-returned-local-unset-element.cpp declarations/initialization/forms/copy-indeterminate-unsigned-char.cpp foundations/constant-evaluation/read-returned-local-unset-element.cpp foundations/constant-evaluation/return-local-unset-member.cpp foundations/constant-evaluation/return-prvalue-unset-member-read.cpp

Constant evaluation must not read a scalar object that has no value
({rule}`OBJ-2`). The rule also applies where a run-time read is not
undefined, such as a copy into an `unsigned char` ({rule}`INIT-1`,
{question}`Q-0039`). An implicit copy or move function
({rule}`CPY-8`) reads each scalar part of its source, also in the move
of a returned local variable ({rule}`CPY-12`), and a user-provided one
reads only what its body reads.
:::

:::{rule} CE-23
:kind: required
:tests: foundations/constant-evaluation/literal-destructor.cpp foundations/constant-evaluation/literal-member.cpp foundations/constant-evaluation/literal-base.cpp foundations/constant-evaluation/literal-volatile-member.cpp foundations/constant-evaluation/literal-union-volatile-member.cpp foundations/constant-evaluation/literal-union-all-volatile.cpp foundations/constant-evaluation/literal-constexpr-destructor.cpp foundations/constant-evaluation/literal-closure.cpp foundations/constant-evaluation/literal-not-reached.cpp foundations/constant-evaluation/literal-one-constexpr-constructor.cpp foundations/constant-evaluation/literal-temporary-volatile-member.cpp

A *literal type* is one of these types:

- a {term}`scalar type` or a reference type,
- an array of a literal type,
- the closure type of a lambda whose captures have literal types,
- a class whose destructor is implicit, defaulted, or `constexpr`, and
  whose bases and non-static data members have literal types that are
  not `volatile`. A union can have `volatile` members if at least one
  member is not `volatile`. The class must also be an aggregate, or have
  a `constexpr` constructor other than a copy or move constructor.

{rule}`TPL-7` rejects a constant template parameter of a class that
fails the last condition.
:::

:::{rule} CE-24
:kind: required
:tests: foundations/constant-evaluation/constant-initializer-usable.cpp foundations/constant-evaluation/constant-initializer-not-constant.cpp foundations/constant-evaluation/constant-initializer-zero-scalar.cpp foundations/constant-evaluation/constant-initializer-zero-trivial-class.cpp foundations/constant-evaluation/constant-initializer-trivial-class-block-static.cpp foundations/constant-evaluation/constant-initializer-trivial-class-array.cpp foundations/constant-evaluation/constant-initializer-trivial-empty-class.cpp

A *constant initializer* is an initializer whose whole initialization of
the variable, with each conversion and constructor call in it, is a
constant expression. For a variable with no initializer, this rule and
CE-21 judge the {term}`default initialization` ({rule}`INIT-9`):

- A scalar or an array of scalars gets no initialization. The zero of
  {rule}`STO-3` is then a constant initializer.
- A class or an array of a class calls the {term}`default constructor`, also a
  {term}`trivial` one. The zero does not count, so a trivial
  constructor leaves each scalar member without a value
  ({question}`Q-0192`).
:::

:::{rule} CE-18
:kind: required
:tests: foundations/constant-evaluation/unknown-reference-not-read.cpp foundations/constant-evaluation/unknown-reference-static-member.cpp

A reference whose object is not known during constant evaluation can
still occur in a constant expression if the evaluation does not read
through the reference. An example is a call of a `constexpr` member
function that does not use the object. A call of a static member
function through the reference, as in `s.size()`, does not use the
object either.
:::

:::{rule} CE-20
:kind: withdrawn

This rule merged into {rule}`CE-17`.
:::

:::{rule} CE-25
:kind: withdrawn

This rule merged into {rule}`CE-11`.
:::

:::{rule} CE-29
:kind: withdrawn

This rule merged into {rule}`CE-3`.
:::

:::{rule} CE-31
:kind: withdrawn

This rule merged into {rule}`CE-8`.
:::

:::{rule} CE-32
:kind: withdrawn

This rule merged into {rule}`CE-22`.
:::

## Implementation Requirements

:::{rule} CE-10
:kind: impl-defined
:tests: foundations/constant-evaluation/deep-recursion.cpp foundations/constant-evaluation/moderate-recursion.cpp foundations/constant-evaluation/long-loop.cpp foundations/constant-evaluation/moderate-loop.cpp foundations/constant-evaluation/moderate-loops-split.cpp foundations/constant-evaluation/moderate-call-count.cpp foundations/constant-evaluation/excessive-call-count.cpp templates/deduction/substitution-failure-evaluation-limit.cpp

An implementation sets limits on the depth of recursion and on the
number of steps in one constant evaluation. The implementation rejects
an evaluation that exceeds a limit. The limits are
implementation-defined, and every matrix compiler has limits between
these figures:

| Measure | Accepted | Rejected |
|---|---|---|
| depth of recursion | 400 | 100000 |
| iterations of one loop | 200000 | 100000000 |
| function calls in one evaluation, in total | 150000 | 300000000 |
:::

:::{rule} CE-26
:kind: no-diagnostic
:tests: foundations/constant-evaluation/unsequenced-not-detected.cpp foundations/constant-evaluation/unsequenced-assignments-not-detected.cpp

Constant evaluation must not evaluate two {term}`unsequenced` accesses
to one {term}`memory location` where one access writes ({rule}`SEQ-4`,
{rule}`MUL-14`). An implementation need not diagnose such a pair.
:::

:::{rule} CE-27
:kind: unspecified
:tests: foundations/constant-evaluation/order-may-differ-from-run.cpp foundations/constant-evaluation/order-of-operands-may-differ-from-run.cpp

Where the order of evaluation is unspecified ({rule}`SEQ-1`,
{rule}`SEQ-8`, {rule}`ASN-11`), constant evaluation also uses an
unspecified order. The order need not be the one that the same
implementation uses at run time for the same expression.
:::

## Examples

% tidal-example: reject
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

constexpr int fail() { throw 1; }
constexpr int guarded() {
  try {
    constexpr int w = fail();         // rejected: the try block is not part
    return w;                         // of the new evaluation (CE-30)
  } catch (...) { return 0; }
}
constexpr const int& t = 4;           // fine: the temporary is static
void use() {
  constexpr const int& u = 4;         // rejected: the temporary is local
}
constexpr int&& m = 4;                // fine: the temporary is static
static_assert(m == 4);                // rejected: the temporary is not const

struct Table { int v[4]; };
constexpr Table make() {
  Table t;
  for (int i = 0; i < 3; ++i) t.v[i] = i + 1;
  return t;                           // a move of t (CE-22)
}
constexpr Table bad = make();         // rejected: v[3] has no value
struct Filled { int v[4] = {}; };     // every element starts at 0, so a
                                      // builder can set only some

struct Pair { int first; int second; };
struct Empty {};
constinit int counter;                // fine: the zero is the value
constinit Pair pair;                  // rejected: the default constructor
                                      // gives first and second no value
constinit Empty empty;                // fine: Empty has no scalar part

constexpr int pass_on(bool rethrow) {
  if (rethrow) throw;                 // a rethrow with no exception
  return 1;
}
static_assert(pass_on(false) == 1);   // fine: the rethrow is not reached
static_assert(pass_on(true) == 1);    // rejected: the rethrow is reached

constexpr const int* dangling() { int x = 1; return &x; }
static_assert(*dangling() == 1);      // rejected: x ended with the call

struct Box {
  constexpr int size() const { return 4; }
  static constexpr int count() { return 2; }
};
int sizes(Box& b) {                   // the object of b is unknown
  constexpr int n = b.size();         // fine: size reads nothing of b
  constexpr int c = b.count();        // fine: count is static (CE-18)
  return n + c;
}

int local() {
  constexpr int table[] = {1, 2};     // automatic storage
  auto&& view = table;                // no constant initializer (CE-3)
  constexpr int second = view[1];     // rejected: view is not usable
  return second;
}

constexpr const int* kept() {
  static const int value = 3;         // usable: a const int with a
  return &value;                      // constant initializer (CE-3)
}
template <const int* P> struct Ptr {};
Ptr<kept()> ptr;                      // fine
constexpr int count() {
  static int calls = 0;               // not usable: not const
  return ++calls;
}
static_assert(count() == 1);          // rejected: the evaluation reaches calls (CE-17)

union Some { volatile int a; int b; };          // literal: b is not volatile
union All { volatile int a; volatile long b; }; // not a literal type
constexpr Some some{.b = 1};          // fine
constexpr All all{.a = 1};            // rejected: All is not literal (CE-23)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 133
of 163 tests.

### Divergences

- {question}`Q-0002` (CE-28; compiler defect): whether EDG's acceptance
  of a floating division by zero in constant evaluation changes CE-28.
  CE-28 rejects the division, as Clang and GCC do. EDG 7.0 accepts it.
- {question}`Q-0004` (CE-28; settled): whether a floating division by
  zero is implementation-defined on IEEE 754 targets. It is at run
  time, and constant evaluation rejects it. Only EDG differs, under
  Q-0002.
- {question}`Q-0020` (CE-12; open): how a test records a compiler that
  does not implement a feature yet. An override names the question.
  Clang 18 also rejects the definition of a `constexpr` function that
  can never give a constant, also when no evaluation calls it
  ({rule}`CX-3`).
- {question}`Q-0033` (CE-13; open): whether constant evaluation may
  compare the addresses of two string literals. CE-13 rejects the
  comparison, as Clang does. GCC and EDG fold some or all such
  comparisons.
- {question}`Q-0078` (CE-11; settled): whether the operand of `noexcept` or
  `explicit` can narrow when it converts to `bool`. It cannot, as Clang
  and GCC say. EDG 7.0 accepts `explicit(2)`.
- {question}`Q-0090` (CE-18; settled): whether an evaluation can use a
  reference whose referent is unknown when it reads nothing through it.
  It can. Only Clang 18 differs, for a call of a static member function
  and of a non-static one that does not use the object.
- {question}`Q-0115` (CE-21, CE-22; compiler defect): whether GCC's
  acceptance of a copy of an object with a part that has no value
  changes CE-22. The copy is rejected, as Clang and EDG do. GCC 14 and
  16 accept the copy and read the part as 0.
- {question}`Q-0117` (CE-21; open): whether a union with no active
  member is a valid constant. It is, as Clang and EDG say. GCC rejects
  it.
- {question}`Q-0192` (CE-24; open): whether a class with a trivial
  default constructor and no initializer has a constant initializer. A
  class with a scalar member has none, as Clang says. GCC and EDG
  accept the variable.
- {question}`Q-0193` (CE-3, CE-14; open): whether a constant can refer
  to, and read, every temporary that a namespace reference extends. It
  can, and a read needs a `const` type, as Clang says. EDG rejects two
  forms of the reference. GCC rejects a read of a class temporary, also
  through its reference member. In `read-extended-temporary-nested.cpp`,
  EDG rejects the declaration and GCC rejects the read.
- {question}`Q-0318` (UN-7, PFX-21; open): whether constant evaluation
  must reject a member function call through a null pointer. It must,
  as Clang and GCC do. EDG accepts the call.
- {question}`Q-0335` (CE-17; open): whether a temporary that a constant
  evaluation makes needs a literal type. It does not, as Clang 22 says.
  GCC rejects such a temporary, EDG rejects one in a `static_assert`
  condition, and Clang 18 also rejects a local variable.
- {question}`Q-0349` (CE-4; open): whether a member of
  `std::allocator<T>` can convert the result of `::operator new` to a
  pointer to a type other than `T`. It cannot, as Clang says. GCC and
  EDG accept the conversion.
- {question}`Q-0421` (LSUP-11, CE-9; open): whether constant evaluation
  accepts an assignment to an element of storage from `std::allocator`
  before `std::construct_at` builds it. It rejects the assignment, as
  Clang does. GCC and EDG accept it.
- {question}`Q-0425` (CE-6, UNI-14; open):
  whether building an object inside a union member that is not
  active, as a member of a member union or an element of an array
  member, makes the member active. It does not, so constant evaluation
  rejects the construction, as Clang and GCC say. EDG accepts it.

### Split tests

Each row is a test that carries an override for a baseline compiler. The
Expected column gives the outcome that the rules predict, and each
family column gives the outcome that the test expects on that family.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `allocator-result-other-type.cpp` | CE-4 | reject | reject | accept | accept | {question}`Q-0349` |
| `classes/unions/construct-in-member-of-inactive-member-union-constant.cpp` | UNI-14, UNI-6, CE-6 | reject | reject | reject | accept | {question}`Q-0425` |
| `constant-initializer-trivial-class-array.cpp` | CE-24, CE-21, CINIT-1 | reject | reject | run | accept | {question}`Q-0192` |
| `constant-initializer-trivial-class-block-static.cpp` | CE-24, CE-21, CINIT-1 | reject | reject | run | accept | {question}`Q-0192` |
| `constant-initializer-zero-trivial-class.cpp` | CE-24, CE-21, CINIT-1 | reject | reject | run | accept | {question}`Q-0192` |
| `copy-unset-array-element.cpp` | CE-22, OBJ-2 | reject | reject | accept | reject | {question}`Q-0115` |
| `copy-unset-member.cpp` | CE-22, OBJ-2 | reject | reject | accept | reject | {question}`Q-0115` |
| `explicit-condition-narrowing.cpp` | CE-19 | reject | reject | reject | accept | {question}`Q-0078` |
| `expressions/multiplicative/floating-divide-by-zero.cpp` | CE-28 | reject | reject | reject | accept | {question}`Q-0002` |
| `expressions/multiplicative/floating-zero-by-zero.cpp` | CE-28 | reject | reject | reject | accept | {question}`Q-0002` |
| `expressions/postfix/member-call-null-pointer-constant.cpp` | PFX-21, UN-7, CE-9 | reject | reject | reject | accept | {question}`Q-0318` |
| `extended-temporary-member.cpp` | CE-14 | accept | accept | accept | reject | {question}`Q-0193` |
| `extended-temporary-nested.cpp` | CE-14 | accept | accept | accept | reject | {question}`Q-0193` |
| `foundations/storage/allocation-assign-before-construction-double-element.cpp` | LSUP-11, CE-9 | reject | reject | accept | accept | {question}`Q-0421` |
| `foundations/storage/allocation-assign-before-construction-int-element.cpp` | LSUP-11, CE-9 | reject | reject | accept | accept | {question}`Q-0421` |
| `foundations/storage/allocation-assign-before-construction-pod-member.cpp` | LSUP-11, CE-9 | reject | reject | accept | accept | {question}`Q-0421` |
| `foundations/storage/allocation-assign-before-construction-user-assignment.cpp` | LSUP-11, CE-9 | reject | reject | accept | accept | {question}`Q-0421` |
| `literal-temporary-volatile-member.cpp` | CE-17, CE-23 | accept | Clang 22: accept; Clang 18: reject | reject | reject | {question}`Q-0335` |
| `nonliteral-temporary-in-function.cpp` | CE-17 | accept | Clang 22: accept; Clang 18: reject | reject | accept | {question}`Q-0335` |
| `nonliteral-variable-reached.cpp` | CE-17 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0335` |
| `read-extended-temporary-class.cpp` | CE-3, CE-14 | accept | accept | reject | accept | {question}`Q-0193` |
| `read-extended-temporary-nested.cpp` | CE-3, CE-14 | accept | accept | reject | reject | {question}`Q-0193` |
| `read-returned-local-unset-element.cpp` | CE-22, CE-21 | reject | reject | accept | reject | {question}`Q-0115` |
| `result-returned-local-unset-element.cpp` | CE-21, CE-22 | reject | reject | accept | reject | {question}`Q-0115` |
| `result-union-no-active-member.cpp` | CE-21 | accept | accept | reject | accept | {question}`Q-0117` |
| `return-local-unset-member.cpp` | CE-22, CPY-12 | reject | reject | accept | reject | {question}`Q-0115` |
| `string-literal-identity-variables.cpp` | CE-13 | reject | reject | accept | accept | {question}`Q-0033` |
| `string-literal-identity.cpp` | CE-13 | reject | reject | accept | accept | {question}`Q-0033` |
| `unknown-reference-not-read.cpp` | CE-18 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0090` |
| `unknown-reference-static-member.cpp` | CE-18 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0090` |

### Notes

- CE-9: the gap review of the text phase 8 wrote on expressions, foundations,
  and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) read the sentence on a read
  after a lifetime ends as possibly naming an operation that need not be
  detected. CE-9 now says that such a read is not one of the operations
  of CE-26, so constant evaluation rejects it, as every compiler does.
- CE-10: all three implementations reject a recursion depth of 100000
  and a loop of 100000000 steps by default. All accept a depth of 400,
  four loops of 100000 iterations in one evaluation, and 150049 calls
  (`moderate-loops-split.cpp`, `moderate-call-count.cpp`).
  In a template argument of a function template's signature, a failed
  evaluation is a {term}`substitution failure` ({rule}`DED-24`). So the limits
  can decide which overload a call selects.

  - Clang stops at a depth of 512 calls and at its step limit
    (`-fconstexpr-depth`, `-fconstexpr-steps`).
  - GCC stops at a depth of 512, at 262144 iterations of one loop,
    and at 33554432 operations in one evaluation
    (`-fconstexpr-depth`, `-fconstexpr-loop-limit`,
    `-fconstexpr-ops-limit`). The loop limit counts each loop on its
    own, so four loops of 100000 iterations pass.
  - Clang counts every step of the evaluation together. Clang 18 and 22
    accept one loop of 1000000 iterations and reject one of 2000000.
    EDG accepts one of 500000 and rejects one of 1000000. A naive
    recursive Fibonacci function that makes 1028457 calls fails on
    every Clang version, and one that makes 635621 calls fails on Clang
    22 at `-O2`. EDG accepts 635621 calls and rejects 1028457. Every
    compiler accepts 392835 calls.
  - The limit on total work is reached long before the depth limit. A
    recursion of depth 40 that makes 331 million calls fails on every
    compiler.
- CE-28, CE-13: every test of the two rules needs an override for EDG,
  because EDG accepts each division and folds each comparison that the
  probes tried.
- CE-3: every compiler accepts `&alias` in a constant expression and
  `alias` as a template argument, where `alias` is a reference to a
  global with a constant initializer and is not `constexpr`. Every
  compiler rejects a reference whose initializer calls a function that
  is not `constexpr`.
- CE-17, CE-2: every compiler rejects the evaluation when a variable or
  a temporary of a class with a destructor that is not `constexpr`
  ends, also when the destructor's body is empty. Clang 22 and EDG
  name the destructor call. GCC names the call for a variable, and for
  a temporary names the destructor as the reason that the class is not
  a literal type. Clang 18 rejects the type that is not a literal type.
- CE-30: Clang and GCC reject a throw in a `constexpr` variable's
  initializer inside a try block of a `constexpr` function, and a throw
  in a `consteval` call inside a run-time try block.
- CE-17: every compiler accepts an evaluation that reaches a `static`
  or `thread_local` variable that is usable in constant expressions:
  a `constexpr` variable, or a `const` integer with a constant
  initializer. Every compiler rejects one that reaches a `static const
  double`.
- CE-21: every compiler rejects a constant whose value has a scalar part
  with no value, for `constexpr` and `constinit` alike, when a
  constructor leaves the part without a value.
- CE-26: all eight compilers accept constant evaluation of
  `i++ + i++`, `i + i++`, `i++ * i`, `(i = 1) + (i = 2)`, and
  `i - ++i`. CE-26 is class `no-diagnostic` because no compiler detects
  the pair. So the table below shows "no test" for SEQ-4 and MUL-14.
- CE-22: every compiler rejects a copy of an `unsigned char` that has
  no value into another `unsigned char`
  (`copy-indeterminate-unsigned-char.cpp`). The "some cases" entries of
  INIT-1 and OBJ-2 below name the tests that a compiler accepts. Each
  other test of the two rules is rejected.
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
- "some cases": the compiler rejects some cases and not others. The
  entry names the tests that the compiler accepts.

Where the versions of a compiler differ, the entry names each version.
"no test" means that the rule has no case.

:::{tidal-undefined-index}
:::

Inputs: clang-sema-expr, clang-diag-sema.

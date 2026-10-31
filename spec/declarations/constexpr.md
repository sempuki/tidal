# The constexpr, consteval, and constinit specifiers

These specifiers control when evaluation occurs.

- `constexpr` on a variable makes the variable a constant. On a
  function, `constexpr` lets {term}`constant evaluation` call the
  function.
- `consteval` makes a function an *immediate function*. Each call must
  be a constant expression.
- `constinit` requires that a variable with static or thread
  {term}`storage duration` has a constant initializer.

Rules are prefixed `CX` for `constexpr`, `CEV` for `consteval`, and
`CINIT` for `constinit`.

## Principles

:::{principle} CX-P1
:rules: CX-3 CX-6 CX-4 CX-5

`constexpr` on a function is a permission. It lets constant evaluation
call the function ({principle}`TR-P4`), and the same function still
runs at run time with the same meaning. The body can contain operations
that constant evaluation cannot do, as long as no constant evaluation
reaches them. Because every translation unit that evaluates the
function needs its body, the function is `inline`. The specifier
applies to functions and variables only, and all declarations of a
function must agree on it.
:::

:::{principle} CX-P2
:rules: CX-1 CX-2

`constexpr` on a variable makes the variable a constant:

- the variable is `const`,
- its initializer is a constant expression,
- its type can be evaluated at translation time.

A constant that is a pointer or reference must refer to a function or
an object with static storage duration, never an automatic variable.
The target must exist at translation time and still exist at run time.
:::

:::{principle} CX-P3
:rules: CEV-1 CEV-2 CEV-3 CEV-4

`consteval` on a function is a demand. Every call is evaluated at
translation time, so a call with arguments that are not constant is an
error unless the call sits inside another function that constant
evaluation is already running. A function that makes such a call becomes
`consteval` itself when the language allows, and otherwise the call is
rejected. The function has no run-time existence, so its address can be
taken only where constant evaluation is in progress. A `consteval`
function is `inline` for the same reason as a `constexpr` function.
:::

:::{principle} CX-P4
:rules: CINIT-1 CINIT-2

`constinit` is about initialization only. The variable must live for the
whole program or the whole thread, and its initializer must be a
constant, so that the variable already has its value before any other
static object can look at the variable. The specifier does not make the
variable `const` and asks nothing of the destructor. A variable is
either `constinit` or `constexpr`, and every declaration of it says
which.
:::

## Constraints

:::{rule} CX-1
:kind: reject
:tests: declarations/constexpr/variable-const.cpp declarations/constexpr/variable-not-constant.cpp declarations/constexpr/variable-no-initializer.cpp declarations/constexpr/variable-non-literal.cpp declarations/constexpr/variable-assign.cpp declarations/constexpr/constexpr-ref-temporary-namespace.cpp declarations/constexpr/constexpr-ref-temporary-block.cpp declarations/constexpr/variable-default-constructor.cpp declarations/constexpr/variable-default-initialized-no-value.cpp expressions/constant-expressions/literal-destructor.cpp expressions/constant-expressions/literal-member.cpp expressions/constant-expressions/literal-base.cpp expressions/constant-expressions/literal-volatile-member.cpp expressions/constant-expressions/literal-union-all-volatile.cpp declarations/constexpr/variable-implicit-constructor-member-initializers.cpp declarations/constexpr/variable-implicit-constructor-member-unset.cpp

A `constexpr` variable is `const`, and it must obey these rules:

- The variable must have an initializer, unless it has class type and a
  `constexpr` default constructor gives every member a value. An
  implicit default constructor does so when CTOR-4 holds for the class.
- Its initialization must be a constant expression.
- Its type must be a {term}`literal type` (CE-23).
- A `constexpr` reference in a block, without `static`, must not bind
  to a temporary.

A `constexpr` reference at namespace scope can bind to a temporary,
because that temporary has static storage.
:::

:::{rule} CX-2
:kind: reject
:tests: declarations/constexpr/reference-to-static.cpp declarations/constexpr/reference-to-automatic.cpp declarations/constexpr/pointer-to-automatic.cpp declarations/constexpr/pointer-one-past-end.cpp declarations/constexpr/static-local-to-static-local.cpp

A `constexpr` reference, or a `constexpr` pointer that is not null, must
refer to one of these:

- a function,
- an object with static {term}`storage duration`,
- for a pointer only, the place one past the end of such an object.

A static local variable can refer to another static local variable. A
`constexpr` pointer or reference must not refer to an automatic
variable.
:::

:::{rule} CX-3
:kind: required
:tests: declarations/constexpr/function-compile-and-run.cpp declarations/constexpr/function-never-constant.cpp declarations/constexpr/function-loops-and-locals.cpp declarations/constexpr/function-static-local-unevaluated.cpp declarations/constexpr/function-static-constexpr-local.cpp declarations/constexpr/function-goto-unevaluated.cpp declarations/constexpr/function-non-literal-param.cpp declarations/constexpr/function-non-literal-local-unevaluated.cpp declarations/constexpr/non-literal-return.cpp

A program can call a `constexpr` function during {term}`constant
evaluation` and at run time. The body can contain the operations that
CX-6 lists on paths that the call does not evaluate. A function that
never gives a constant expression is valid.

The return type and the parameter types of a `constexpr` function need
not be literal types.
:::

:::{rule} CX-6
:kind: reject
:tests: declarations/constexpr/function-not-constant-call.cpp declarations/constexpr/function-static-local-evaluated.cpp

During {term}`constant evaluation`, a call to a `constexpr` function
(CX-3) must not evaluate an operation that constant evaluation does not
permit. Such operations include:

- a call to a function that is not `constexpr`,
- the definition of a `static` or `thread_local` variable that is not
  `constexpr`,
- a `goto`.
:::

:::{rule} CX-4
:kind: required
:tests: declarations/constexpr/function-implicitly-inline.cpp

A `constexpr` function is `inline` (LNK-4).
:::

:::{rule} CX-5
:kind: reject
:tests: declarations/constexpr/constexpr-class-only.cpp declarations/constexpr/constexpr-redeclaration-mismatch.cpp declarations/constexpr/constexpr-parameter.cpp

Only variables and functions can be `constexpr`. A parameter must not be
`constexpr`, and neither can a declaration that only defines a class.
All declarations of a function must agree on `constexpr`.
:::

:::{rule} CEV-1
:kind: reject
:tests: declarations/constexpr/consteval-call-constant.cpp declarations/constexpr/consteval-call-runtime-arg.cpp declarations/constexpr/consteval-in-consteval.cpp declarations/constexpr/consteval-ub-rejected.cpp declarations/constexpr/immediate-escalation.cpp declarations/constexpr/lambda-escalates-constant.cpp declarations/constexpr/lambda-escalates-runtime.cpp declarations/constexpr/consteval-returns-allocation.cpp declarations/constexpr/plain-constexpr-no-escalation.cpp

Each call to a `consteval` function must be a constant expression. This
rule does not apply to a call in an {term}`immediate function context`,
such as the body of another `consteval` function.

These functions take on `consteval` from their bodies:

- a `constexpr` function produced from a template *(templates
  referenced)*,
- a lambda without `consteval` on it,
- a special member function that the class defaults.

If such a function calls a `consteval` function and the arguments are
not constant, the calling function is treated as `consteval` too, and
its own calls are then checked the same way.

The result of a `consteval` call must be usable in a constant
expression. A pointer to storage that the call allocated and did not
release is not.
:::

:::{rule} CEV-2
:kind: reject
:tests: declarations/constexpr/consteval-address-taken.cpp declarations/constexpr/consteval-address-in-immediate.cpp

A program can take the address of a `consteval` function only in an
immediate function context.
:::

:::{rule} CEV-3
:kind: reject
:tests: declarations/constexpr/consteval-variable.cpp declarations/constexpr/consteval-and-constexpr.cpp declarations/constexpr/consteval-redeclare-constexpr.cpp

`consteval` applies only to functions. A declaration must not have both
`consteval` and `constexpr`. All declarations of a function must agree
on `consteval`.
:::

:::{rule} CINIT-1
:kind: reject
:tests: declarations/constexpr/constinit-static.cpp declarations/constexpr/constinit-not-const.cpp declarations/constexpr/constinit-dynamic.cpp declarations/constexpr/constinit-nonconstexpr-dtor.cpp expressions/constant-expressions/constant-initializer-not-constant.cpp

A `constinit` variable must have a {term}`constant initializer`.
`constinit` does not make the variable `const`, so the program can
change the variable. `constinit` does not require a `constexpr`
destructor.
:::

:::{rule} CINIT-2
:kind: reject
:tests: declarations/constexpr/constinit-automatic.cpp declarations/constexpr/constinit-block-static.cpp declarations/constexpr/constinit-with-constexpr.cpp declarations/constexpr/constinit-extern-declaration.cpp declarations/constexpr/constinit-on-function.cpp declarations/constexpr/constinit-added-late.cpp declarations/constexpr/constinit-on-declaration-only.cpp

A `constinit` variable must have static or thread {term}`storage
duration`. A declaration must not have both `constinit` and `constexpr`.
If a declaration of a variable has `constinit`, the initializing
declaration must also have it ({question}`Q-0025`).
:::

:::{rule} CEV-4
:kind: required
:tests: declarations/constexpr/consteval-inline.cpp

A `consteval` function is `inline` (LNK-4).
:::

## Examples

```cpp
constexpr int limit = 3;          // const int
int buffer[limit];

constexpr int square(int v) { return v * v; }
static_assert(square(4) == 16);   // constant evaluation
int at_run_time(int n) { return square(n); }

consteval int cube(int v) { return v * v * v; }
constexpr int a = cube(2);        // fine
int b(int n) { return cube(n); }  // rejected: n is not a constant

constinit int counter = 5;        // constant initializer, not const
void bump() { ++counter; }
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 58
of 63 tests.

- CX-3: Clang 18 rejects a `constexpr` function that never gives a
  constant, and a non-literal parameter type. Newer versions accept both
  ({question}`Q-0020`).
- CINIT-2: Clang rejects `constinit` added after the initializing
  declaration. GCC and EDG accept the late `constinit`. Clang
  (pedantic) and EDG reject `constinit` that is only on an earlier
  declaration. GCC accepts the earlier `constinit`
  ({question}`Q-0025`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls.

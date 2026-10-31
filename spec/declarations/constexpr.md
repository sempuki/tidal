# The constexpr, consteval, and constinit specifiers

These specifiers control when evaluation occurs.

- `constexpr` on a variable makes it a constant. On a function, it lets
  {term}`constant evaluation` call the function.
- `consteval` makes a function an *immediate function*. Each call must be
  a constant expression.
- `constinit` requires that a variable with static or thread
  {term}`storage duration` has a constant initializer.

Rules are prefixed `CX` for `constexpr`, `CEV` for `consteval`, and
`CINIT` for `constinit`.

## Principles

:::{principle} CX-P1
:rules: CX-3 CX-4 CX-5

`constexpr` on a function is a permission, not a demand. It lets
constant evaluation call the function ({principle}`TR-P4`), and the same
function still runs at run time with the same meaning. The body can
contain operations that constant evaluation cannot do, as long as no
constant evaluation reaches them. Because every translation unit that
evaluates the function needs its body, the function is `inline`. The
specifier applies to functions and variables only, and all declarations
of a function must agree on it.
:::

:::{principle} CX-P2
:rules: CX-1 CX-2

`constexpr` on a variable makes it a constant: it is `const`, its
initializer is a constant expression, and its type can be evaluated at
translation time. A constant that is a pointer or reference must refer
to something that exists at translation time and still exists at run
time, which means a function or an object with static storage
duration, never an automatic variable.
:::

:::{principle} CX-P3
:rules: CEV-1 CEV-2 CEV-3 CEV-4

`consteval` on a function is a demand. Every call is evaluated at
translation time, so a call with arguments that are not constant is an
error unless it sits inside another function that constant evaluation
is already running. A function that makes such a call becomes
`consteval` itself when the language allows, and otherwise the call is
rejected. The function has no run-time existence, so its address can be
taken only where constant evaluation is in progress. It is `inline` for
the same reason as a `constexpr` function.
:::

:::{principle} CX-P4
:rules: CINIT-1 CINIT-2

`constinit` is about initialization only. The variable must live for
the whole program or the whole thread, and its initializer must be a
constant, so that it already has its value before any other static
object can look at it. The specifier does not make the variable `const`
and asks nothing of the destructor. A variable is either `constinit` or
`constexpr`, and every declaration of it says which.
:::

## Constraints

:::{rule} CX-1
:kind: reject
:tests: declarations/constexpr/variable-const.cpp declarations/constexpr/variable-not-constant.cpp declarations/constexpr/variable-no-initializer.cpp declarations/constexpr/variable-non-literal.cpp declarations/constexpr/variable-assign.cpp declarations/constexpr/constexpr-ref-temporary-namespace.cpp declarations/constexpr/constexpr-ref-temporary-block.cpp declarations/constexpr/variable-default-constructor.cpp declarations/constexpr/variable-default-initialized-no-value.cpp

A `constexpr` variable is `const`. It must be initialized, and the
initialization must be a constant expression. A variable of class type
can have no initializer if a `constexpr` default constructor gives every
member a value. Its type must be a literal type
*(referenced)*. A `constexpr` reference at namespace scope can bind to a
temporary, because that temporary has static storage. In a block
without `static`, it cannot.
:::

:::{rule} CX-2
:kind: reject
:tests: declarations/constexpr/reference-to-static.cpp declarations/constexpr/reference-to-automatic.cpp declarations/constexpr/pointer-to-automatic.cpp declarations/constexpr/pointer-one-past-end.cpp declarations/constexpr/static-local-to-static-local.cpp

A `constexpr` reference, or a `constexpr` pointer that is not null, must
refer to a function or to an object with static {term}`storage duration`. A
pointer can also point one past the end of such an object. A static local
variable can refer to another static local variable. A `constexpr`
pointer or reference must not refer to an automatic variable.
:::

:::{rule} CX-3
:kind: required
:tests: declarations/constexpr/function-compile-and-run.cpp declarations/constexpr/function-not-constant-call.cpp declarations/constexpr/function-never-constant.cpp declarations/constexpr/function-loops-and-locals.cpp declarations/constexpr/function-static-local-unevaluated.cpp declarations/constexpr/function-static-local-evaluated.cpp declarations/constexpr/function-static-constexpr-local.cpp declarations/constexpr/function-goto-unevaluated.cpp declarations/constexpr/function-non-literal-param.cpp declarations/constexpr/function-non-literal-local-unevaluated.cpp declarations/constexpr/non-literal-return.cpp

A program can call a `constexpr` function during {term}`constant
evaluation` and at run time. During constant evaluation, the call must
not evaluate an operation that constant evaluation does not permit. Such
operations include:

- a call to a function that is not `constexpr`,
- the definition of a `static` or `thread_local` variable that is not
  `constexpr`,
- a `goto`.

The body can contain these operations on paths that the call does not
evaluate. A function that never gives a constant expression is valid.

The return type and the parameter types of a `constexpr` function need
not be literal types.
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
`constexpr`, and neither can a declaration that only defines a class. All
declarations of a function must agree on `constexpr`.
:::

:::{rule} CEV-1
:kind: reject
:tests: declarations/constexpr/consteval-call-constant.cpp declarations/constexpr/consteval-call-runtime-arg.cpp declarations/constexpr/consteval-in-consteval.cpp declarations/constexpr/consteval-ub-rejected.cpp declarations/constexpr/immediate-escalation.cpp declarations/constexpr/lambda-escalates-constant.cpp declarations/constexpr/lambda-escalates-runtime.cpp declarations/constexpr/consteval-returns-allocation.cpp declarations/constexpr/plain-constexpr-no-escalation.cpp

Each call to a `consteval` function must be a constant expression. This
does not apply to a call in an {term}`immediate function context`, such
as the body of another `consteval` function.

Some functions take on `consteval` from their bodies. If such a function
calls a `consteval` function and the arguments are not constant, the
calling function is treated as `consteval` too, and its own calls are
then checked the same way. These functions are:

- a `constexpr` function produced from a template *(templates
  referenced)*,
- a lambda without `consteval` on it,
- a special member function that the class defaults.

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
:tests: declarations/constexpr/constinit-static.cpp declarations/constexpr/constinit-not-const.cpp declarations/constexpr/constinit-dynamic.cpp declarations/constexpr/constinit-nonconstexpr-dtor.cpp

A `constinit` variable must have a constant initializer. `constinit` does
not make the variable `const`, so the program can change it. It does not require a `constexpr`
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 47 of 52 tests.

- CX-3: Clang 18 rejects a `constexpr` function that never gives a
  constant, and a non-literal parameter type. Newer versions accept both
  ({question}`Q-0020`).
- CINIT-2: Clang rejects `constinit` added after the initializing
  declaration. GCC and EDG accept it. Clang (pedantic) and EDG reject
  `constinit` that is only on an earlier declaration. GCC accepts it
  ({question}`Q-0025`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls.

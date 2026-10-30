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

## Legality Rules

:::{rule} CX-1
:kind: reject
:tests: declarations/constexpr/variable-const.cpp declarations/constexpr/variable-not-constant.cpp declarations/constexpr/variable-no-initializer.cpp declarations/constexpr/variable-non-literal.cpp declarations/constexpr/variable-assign.cpp

A `constexpr` variable is `const`. It must have an initializer, and the
initializer must be a constant expression. Its type must be a literal type
*(referenced)*.
:::

:::{rule} CX-2
:kind: reject
:tests: declarations/constexpr/reference-to-static.cpp declarations/constexpr/reference-to-automatic.cpp declarations/constexpr/pointer-to-automatic.cpp

A `constexpr` reference, or a `constexpr` pointer that is not null,
must refer to an object or function with static {term}`storage duration`.
:::

:::{rule} CX-3
:kind: required
:tests: declarations/constexpr/function-compile-and-run.cpp declarations/constexpr/function-not-constant-call.cpp declarations/constexpr/function-never-constant.cpp declarations/constexpr/function-loops-and-locals.cpp declarations/constexpr/function-static-local-unevaluated.cpp declarations/constexpr/function-static-local-evaluated.cpp declarations/constexpr/function-static-constexpr-local.cpp declarations/constexpr/function-goto-unevaluated.cpp declarations/constexpr/function-non-literal-param.cpp declarations/constexpr/function-non-literal-local-unevaluated.cpp

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
:tests: declarations/constexpr/consteval-call-constant.cpp declarations/constexpr/consteval-call-runtime-arg.cpp declarations/constexpr/consteval-in-consteval.cpp declarations/constexpr/consteval-ub-rejected.cpp

Each call to a `consteval` function must be a constant expression. This
does not apply to a call in an {term}`immediate function context`, such
as the body of another `consteval` function.
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
:tests: declarations/constexpr/constinit-static.cpp declarations/constexpr/constinit-not-const.cpp declarations/constexpr/constinit-dynamic.cpp

A `constinit` variable must have a constant initializer. `constinit` does
not make the variable `const`, so the program can change it.
:::

:::{rule} CINIT-2
:kind: reject
:tests: declarations/constexpr/constinit-automatic.cpp declarations/constexpr/constinit-block-static.cpp declarations/constexpr/constinit-with-constexpr.cpp declarations/constexpr/constinit-extern-declaration.cpp declarations/constexpr/constinit-on-function.cpp declarations/constexpr/constinit-added-late.cpp declarations/constexpr/constinit-on-declaration-only.cpp

A `constinit` variable must have static or thread {term}`storage
duration`. A declaration must not have both `constinit` and `constexpr`.
If a declaration of a variable has `constinit`, the initializing
declaration must also have it ({question}`Q-0025`).
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 37 of
41 tests.

- CX-3: Clang 18 rejects a `constexpr` function that never gives a
  constant, and a non-literal parameter type. Newer versions accept both
  ({question}`Q-0020`).
- CINIT-2: Clang rejects `constinit` added after the initializing
  declaration. GCC and EDG accept it. Clang (pedantic) and EDG reject
  `constinit` that is only on an earlier declaration. GCC accepts it
  ({question}`Q-0025`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, gcc-cp-decl,
edg-decls.

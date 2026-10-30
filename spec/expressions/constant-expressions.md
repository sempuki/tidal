# Constant expressions

A constant expression is an expression that the implementation evaluates
during translation. {term}`Constant evaluation <constant evaluation>`
permits most operations. This page lists the operations that it rejects.

## Legality Rules

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
:tests: expressions/constant-expressions/read-non-const-variable.cpp expressions/constant-expressions/read-const-runtime-init.cpp expressions/constant-expressions/read-const-constant-init.cpp expressions/constant-expressions/read-parameter-outside.cpp

Constant evaluation must not read the value of a variable that is not
usable in constant expressions. A variable is usable if it is `constexpr`,
or if it is a `const` integer variable with a constant initializer (CV-8).
A function parameter is usable only during an evaluation that called the
function.
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
behavior. MUL-9, ADD-10, REF-5, INIT-1, and CV-5 give such operations.
:::

:::{rule} CE-11
:kind: required
:tests: expressions/constant-expressions/converted-constant-narrowing.cpp expressions/constant-expressions/integral-constant-array-bound.cpp

An array bound, an enumerator value, and a case label need a constant
expression of integer or enumeration type. Where a constant converts to
another type for such a use, the conversion must not narrow. The condition
of `if constexpr` is an exception (IF-16).
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

## Implementation Requirements

:::{rule} CE-10
:kind: impl-defined
:tests: expressions/constant-expressions/deep-recursion.cpp expressions/constant-expressions/moderate-recursion.cpp expressions/constant-expressions/long-loop.cpp

An implementation sets limits on the depth of recursion and on the number
of steps in one constant evaluation. It rejects an evaluation that exceeds
a limit. The limits are implementation-defined.
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
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 31 of
32 tests.

- CE-10: all three implementations reject a recursion depth of 100000 and
  a loop of 100000000 steps by default. Clang stops at a depth of 512
  calls and at its step limit (`-fconstexpr-depth`, `-fconstexpr-steps`).
  GCC stops at a depth of 512 and at 262144 loop iterations
  (`-fconstexpr-depth`, `-fconstexpr-loop-limit`, `-fconstexpr-ops-limit`).
  All accept a depth of 400.
- CE-12: Clang 18 also reports the `constexpr` function as never constant
  (compare {question}`Q-0020`).
- CE-13: Clang rejects the comparison of two string literal addresses.
  GCC and EDG accept it and fold it to a value ({question}`Q-0033`).

Inputs: clang-sema-expr, clang-diag-sema, gcc-cp-typeck, gcc-cp-semantics.

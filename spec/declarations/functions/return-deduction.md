# Return type deduction

A function with `auto` or `decltype(auto)` as its return type gets its
return type from its `return` statements.

## Legality Rules

:::{rule} RET-1
:kind: required
:tests: declarations/functions/return-deduction/basic.cpp declarations/functions/return-deduction/conflicting-returns.cpp declarations/functions/return-deduction/void-and-value.cpp declarations/functions/return-deduction/braced-return.cpp

Each `return` statement gives a type, as for a variable that the
returned expression initializes. All `return` statements must give the
same type. A function with no `return` statement, or with only
`return;`, returns `void`. A `return` statement must not return a
braced list.
:::

:::{rule} RET-2
:kind: required
:tests: declarations/functions/return-deduction/auto-strips.cpp declarations/functions/return-deduction/decltype-auto.cpp declarations/functions/return-deduction/trailing-auto.cpp

The rules for variables apply to the return type. `auto` removes
references and top-level qualifiers (AUTO-1). `auto&` keeps them
(AUTO-3). `decltype(auto)` uses `decltype` of the returned expression
(AUTO-10).
:::

:::{rule} RET-3
:kind: reject
:tests: declarations/functions/return-deduction/use-before-definition.cpp declarations/functions/return-deduction/use-after-definition.cpp declarations/functions/return-deduction/recursion-after-return.cpp declarations/functions/return-deduction/recursion-before-return.cpp

A program must not use the function before its return type is known. A
call before the definition is not valid. A recursive call is valid after
a `return` statement that gives the type.
:::

:::{rule} RET-4
:kind: reject
:tests: declarations/functions/return-deduction/redeclare-with-type.cpp declarations/functions/return-deduction/redeclare-with-auto.cpp

Each declaration of the function must use the same placeholder as its
return type.
:::

## Examples

```cpp
auto a() { return 1; }              // int
auto b(bool c) {
  if (c) return 1;
  return 2.0;                       // rejected: int, then double
}

int x;
decltype(auto) c() { return (x); }  // int&

constexpr auto fact(int n) {
  if (n <= 1) return 1;             // the type is now int
  return n * fact(n - 1);           // fine
}
```

## Evidence

All seven compilers agree on all 13 tests. The results are in
`tests/RESULTS.md`.

Inputs: clang-sema-stmt, clang-sema-decl, clang-diag-sema, gcc-cp-typeck,
gcc-cp-decl, edg-func-def.

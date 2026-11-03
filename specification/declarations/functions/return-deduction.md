# Return type deduction

A function with `auto` or `decltype(auto)` as its return type gets its
return type from its `return` statements.

## Principles

:::{principle} RET-P1
:rules: RET-1 RET-7 RET-2 RET-6

A deduced return type is found as if each `return` statement initialized
a variable declared with the placeholder, and all statements must agree.
The rules for variables then carry over:

- `auto` drops references and qualifiers,
- `auto&` keeps them,
- `decltype(auto)` takes the exact type.

`decltype(auto)` can deduce a reference to a local variable, or to a
temporary that a call in the `return` statement creates. Each is
already gone when the caller uses the reference. A plain `auto` return
gives a copy and avoids the problem.
:::

:::{principle} RET-P2
:rules: RET-3 RET-4 RET-5

The type is known only after a `return` statement has been seen, so a
call before the definition, or a recursion before a `return`, has no
type to use. Every declaration must write the placeholder, because the
deduced type is a result and not a declaration. A virtual function
cannot deduce, because its callers never see its definition.
:::

## Constraints

:::{rule} RET-1
:kind: required
:tests: declarations/functions/return-deduction/basic.cpp declarations/functions/return-deduction/discarded-return-not-deduced.cpp declarations/functions/return-deduction/void-call-return.cpp

Each `return` statement gives a type, as for a variable that the
returned expression initializes. A function with no `return` statement,
or with only `return;`, returns `void`. A `return` of an expression of
type `void`, such as a call of a `void` function, also gives `void`. A
`return` statement in a {term}`discarded statement` does not take part,
also outside a template. RET-7 lists what is not valid.
:::

:::{rule} RET-7
:kind: reject
:tests: declarations/functions/return-deduction/conflicting-returns.cpp declarations/functions/return-deduction/void-and-value.cpp declarations/functions/return-deduction/braced-return.cpp

All `return` statements that RET-1 counts must give the same type. A
`return` statement must not return a braced list.
:::

:::{rule} RET-2
:kind: required
:tests: declarations/functions/return-deduction/auto-strips.cpp declarations/functions/return-deduction/decltype-auto.cpp declarations/functions/return-deduction/trailing-auto.cpp

The rules for variables apply to the return type:

| Placeholder | Return type | Rule |
|---|---|---|
| `auto` | Removes references and top-level qualifiers | AUTO-1 |
| `auto&` | Keeps references and top-level qualifiers | AUTO-3 |
| `decltype(auto)` | Uses `decltype` of the returned expression | AUTO-10 |
:::

:::{rule} RET-3
:kind: reject
:tests: declarations/functions/return-deduction/use-before-definition.cpp declarations/functions/return-deduction/use-after-definition.cpp declarations/functions/return-deduction/recursion-after-return.cpp declarations/functions/return-deduction/recursion-before-return.cpp declarations/functions/return-deduction/auto-ref-no-return.cpp declarations/functions/return-deduction/auto-ref-return-local.cpp declarations/functions/return-deduction/member-calls-later-deduced.cpp

A program must not use the function before its return type is known.
These rules also apply:

- A call before the definition is not valid, also in the body of a
  member function that calls a later member of its class.
- A recursive call is valid after a `return` statement that gives the
  type.
- A function with a reference or pointer placeholder return type must
  have a `return` statement with an expression.
- A deduced lvalue reference cannot be initialized from the name of a
  local variable with {term}`automatic storage`, because a `return`
  treats that name as an xvalue (CPY-11). A `static` local variable is
  an lvalue.
:::

:::{rule} RET-4
:kind: reject
:tests: declarations/functions/return-deduction/redeclare-with-type.cpp declarations/functions/return-deduction/redeclare-with-auto.cpp declarations/functions/return-deduction/redeclare-after-definition.cpp

Each declaration of the function must use the same placeholder as its
return type. A later declaration must not write the deduced type in
place of the placeholder.
:::

:::{rule} RET-5
:kind: reject
:tests: declarations/functions/return-deduction/virtual-auto.cpp

A virtual function must not have a deduced return type ({rule}`VIRT-2`).
:::

## Undefined Behavior

:::{rule} RET-6
:kind: undefined
:tests: declarations/functions/return-deduction/decltype-auto-dangling.cpp declarations/functions/return-deduction/decltype-auto-paren-local-xvalue.cpp declarations/functions/return-deduction/decltype-auto-paren-local-sanitizer.cpp declarations/functions/return-deduction/trailing-auto.cpp

If a `decltype(auto)` function returns a parenthesized local variable
with {term}`automatic storage`, the return type is an rvalue reference
to it (CPY-11). For a `static` local or a global variable, the return
type is an lvalue reference. A use of the rvalue reference is
undefined, and {term}`constant evaluation` rejects the use ({rule}`CE-9`).
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
decltype(auto) d() {
  static int s;
  return (s);                       // int&: s is static
}
decltype(auto) e() {
  int n = 1;
  return (n);                       // int&&: a use is undefined
}

constexpr auto fact(int n) {
  if (n <= 1) return 1;             // the type is now int
  return n * fact(n - 1);           // fine
}
constexpr auto fib(int n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2);
}
static_assert(fib(40) == 102334155);  // rejected: 331 million calls
                                      // exceed the step limit (CE-10)

int make();
decltype(auto) pass(auto&& v) { return static_cast<decltype(v)&&>(v); }
decltype(auto) wrap() { return pass(make()); }  // int&&: the temporary from
                                                // make() ends with the return
auto copy() { return pass(make()); }            // int: a copy, safe

auto split(int total) {             // two results, with no type outside
  struct Parts { int whole, rest; };
  return Parts{total / 10, total % 10};
}
auto [w, r] = split(47);            // 4 and 7 (SB-3)
```

## Evidence

All eight compilers agree on all 23 tests. The results are in
`tests/RESULTS.md`.

- RET-6: the address sanitizer stops the use of the result on GCC and
  Clang. Clang reports a use after return. GCC gives a null reference
  for the address of the local variable, and the read faults.

Inputs: clang-sema-stmt, clang-sema-decl, clang-diag-sema, edg-func-def.

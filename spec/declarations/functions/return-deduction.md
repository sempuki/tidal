# Return type deduction

A function with `auto` or `decltype(auto)` as its return type gets its
return type from its `return` statements.

## Principles

:::{principle} RET-P1
:rules: RET-1 RET-2 RET-6

A deduced return type is found as if each `return` statement initialized
a variable declared with the placeholder, and all statements must
agree. The rules for variables then carry over: `auto` drops references
and qualifiers, `auto&` keeps them, and `decltype(auto)` takes the exact
type. That last form can deduce a reference to a local variable, which
is already gone when the caller uses it.
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
:tests: declarations/functions/return-deduction/basic.cpp declarations/functions/return-deduction/conflicting-returns.cpp declarations/functions/return-deduction/void-and-value.cpp declarations/functions/return-deduction/braced-return.cpp declarations/functions/return-deduction/discarded-return-not-deduced.cpp declarations/functions/return-deduction/void-call-return.cpp

Each `return` statement gives a type, as for a variable that the
returned expression initializes. All `return` statements must give the
same type. A function with no `return` statement, or with only
`return;`, returns `void`. A `return` of an expression of type `void`,
such as a call of a `void` function, also gives `void`. A `return` statement must not return a
braced list. A `return` statement in a {term}`discarded statement` does
not take part, also outside a template.
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
:tests: declarations/functions/return-deduction/use-before-definition.cpp declarations/functions/return-deduction/use-after-definition.cpp declarations/functions/return-deduction/recursion-after-return.cpp declarations/functions/return-deduction/recursion-before-return.cpp declarations/functions/return-deduction/auto-ref-no-return.cpp declarations/functions/return-deduction/auto-ref-return-local.cpp

A program must not use the function before its return type is known. A
call before the definition is not valid. A recursive call is valid after
a `return` statement that gives the type. A function with a reference or
pointer placeholder return type must have a `return` statement with an
expression. A deduced lvalue reference cannot be initialized from the
name of a local variable, because a `return` treats that name as an
xvalue (CPY-11).
:::

:::{rule} RET-4
:kind: reject
:tests: declarations/functions/return-deduction/redeclare-with-type.cpp declarations/functions/return-deduction/redeclare-with-auto.cpp declarations/functions/return-deduction/redeclare-after-definition.cpp

Each declaration of the function must use the same placeholder as its
return type. A later declaration must not write the deduced type in place
of the placeholder.
:::

:::{rule} RET-5
:kind: reject
:tests: declarations/functions/return-deduction/virtual-auto.cpp

A virtual function must not have a deduced return type *(virtual
functions referenced)*.
:::

## Undefined Behavior

:::{rule} RET-6
:kind: undefined
:tests: declarations/functions/return-deduction/decltype-auto-dangling.cpp declarations/functions/return-deduction/decltype-auto-paren-local-xvalue.cpp

If a function with `decltype(auto)` returns a parenthesized local
variable, its return type is an rvalue reference to that variable, because a
`return` treats the name of a local variable as an xvalue (CPY-11). For
a global variable, the result is an lvalue reference. A use
of the result is undefined, because the variable no longer exists.
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

All seven compilers agree on all 20 tests. The results are in
`tests/RESULTS.md`.

Inputs: clang-sema-stmt, clang-sema-decl, clang-diag-sema, edg-func-def.

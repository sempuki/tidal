# Lambda expressions

A lambda expression creates an object of a unique class type, the
*closure type*. A program can call the object like a function. The object
can capture variables from the enclosing scope.

## Syntax

```{eval-rst}
.. productionlist:: expr_lambda
   lambda: "[" [`captures`] "]" [`params`] [`specifiers`] ["->" `type`] `compound_statement`
   captures: `capture_default` | `capture_default` "," `capture_list` | `capture_list`
   capture_default: "=" | "&"
   capture: `name` | "&" `name` | `name` "=" `expression` | "&" `name` "=" `expression`
          : "this" | "*" "this"
   specifiers: ["mutable" | "static"] ["constexpr" | "consteval"] ["noexcept"]
```

## Static Semantics

:::{rule} LAM-1
:kind: required
:tests: expressions/lambdas/default-arguments.cpp expressions/lambdas/immediately-invoked.cpp expressions/lambdas/return-deduction.cpp expressions/lambdas/syntax-forms.cpp

A lambda can have parameters, default arguments, and a trailing return
type. Without a trailing return type, the return type comes from its
`return` statements, as for a function with an `auto` return type. A
program can call a lambda in the expression that creates it.
:::

:::{rule} LAM-2
:kind: required
:tests: expressions/lambdas/unique-type.cpp

Each lambda expression has its own closure type. Two lambda expressions
never have the same type.
:::

:::{rule} LAM-3
:kind: required
:tests: expressions/lambdas/const-call-operator.cpp expressions/lambdas/mutable.cpp

The call operator is `const` unless the lambda is `mutable`. Without
`mutable`, the body must not change a variable captured by copy.
:::

:::{rule} LAM-5
:kind: required
:tests: expressions/lambdas/default-capture-mixed.cpp expressions/lambdas/default-captures.cpp

`[=]` captures by copy each local variable that the body uses. `[&]`
captures them by reference. An explicit capture after a default uses the
other mode.
:::

:::{rule} LAM-6
:kind: reject
:tests: expressions/lambdas/capture-abstract.cpp expressions/lambdas/capture-copy-same-as-default.cpp expressions/lambdas/capture-default-at-namespace.cpp expressions/lambdas/capture-global-rejected.cpp expressions/lambdas/capture-repeated.cpp expressions/lambdas/capture-same-as-default.cpp

A capture list must obey these rules:

- Each name occurs at most once.
- An explicit capture must not use the same mode as the default.
- Only a variable with {term}`automatic storage` can be captured.
- A lambda outside a function must not have a capture default.
- A variable of abstract class type *(referenced)* cannot be captured by
  copy.
:::

:::{rule} LAM-7
:kind: required
:tests: expressions/lambdas/capture-structured-binding.cpp expressions/lambdas/init-capture.cpp

An init capture `[x = e]` declares a new member `x` that `e` initializes.
`[&r = v]` declares a reference to `v`. A structured binding can be
captured.
:::

:::{rule} LAM-8
:kind: required
:tests: expressions/lambdas/capture-this.cpp expressions/lambdas/this-outside-member.cpp

In a member function, `[this]` captures the object by reference, and
`[*this]` captures a copy of it. A lambda outside a member function must
not capture `this`.
:::

:::{rule} LAM-9
:kind: reject
:tests: expressions/lambdas/constant-needs-no-capture.cpp expressions/lambdas/nested-capture-ok.cpp expressions/lambdas/nested-capture.cpp expressions/lambdas/static-and-global-no-capture.cpp expressions/lambdas/uncaptured-variable.cpp

A lambda must capture each local automatic variable that its body uses.
For a nested lambda, each enclosing lambda must capture it too. A constant
that the body only reads as a value needs no capture. Static and global
variables need no capture.
:::

:::{rule} LAM-10
:kind: required
:tests: expressions/lambdas/capturing-no-function-pointer.cpp expressions/lambdas/function-pointer.cpp

A lambda without captures converts to a pointer to a function with the
same parameters and return type. A lambda with captures does not.
:::

:::{rule} LAM-11
:kind: required
:tests: expressions/lambdas/capturing-not-default-constructible.cpp expressions/lambdas/default-constructible.cpp

The closure type of a lambda without captures can be default-constructed
and assigned. The closure type of a lambda with captures cannot be
default-constructed.
:::

:::{rule} LAM-12
:kind: required
:tests: expressions/lambdas/constexpr-implicit.cpp

The call operator is `constexpr` when its body permits it. A program can
call such a lambda during {term}`constant evaluation`.
:::

:::{rule} LAM-13
:kind: required
:tests: expressions/lambdas/static-call-operator.cpp expressions/lambdas/static-with-capture.cpp

A lambda declared `static` has a static call operator. It must not have
captures.
:::

:::{rule} LAM-14
:kind: required
:tests: expressions/lambdas/unevaluated-operand.cpp

A lambda can occur in an operand that is not evaluated, such as the
operand of `decltype` or `sizeof`.
:::

:::{rule} LAM-15
:kind: required
:tests: expressions/lambdas/generic.cpp

A parameter of type `auto` makes the call operator a template
*(referenced)*. Each call deduces the parameter type.
:::

:::{rule} LAM-16
:kind: required
:tests: expressions/lambdas/explicit-object-recursion.cpp

A lambda can have an explicit object parameter `this auto self`. The body
can call `self` to call the lambda again.
:::

## Dynamic Semantics

:::{rule} LAM-4
:kind: required
:tests: expressions/lambdas/capture-by-copy-at-creation.cpp expressions/lambdas/capture-by-reference.cpp

A capture by copy copies the variable when the lambda expression is
evaluated. A capture by reference refers to the variable itself, so later
changes to the variable are visible.
:::

## Examples

```cpp
int n = 1;
auto by_copy = [n] { return n; };
auto by_ref  = [&n] { return n; };
n = 5;
by_copy();                        // 1
by_ref();                         // 5

auto count = [k = 0]() mutable { return ++k; };
int (*p)(int) = [](int x) { return x + 1; };
auto fact = [](this auto self, int v) -> int {
  return v <= 1 ? 1 : v * self(v - 1);
};
```

## Evidence

The results are in `tests/RESULTS.md`.

- LAM-6: EDG accepts a capture default on a lambda outside a function.
  Clang and GCC reject it ({question}`Q-0037`).
- LAM-13 and LAM-16: all seven compilers accept static lambdas and
  explicit object parameters in C++23 mode.

Inputs: clang-sema-expr, clang-diag-sema.

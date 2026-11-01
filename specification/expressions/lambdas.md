# Lambda expressions

A lambda expression creates an object of a unique class type, the
*closure type*. A program can call the object like a function. The
object can capture variables from the enclosing scope.

## Principles

:::{principle} LAM-P1
:rules: LAM-1 LAM-2 LAM-3 LAM-12 LAM-13 LAM-15 LAM-16 LAM-14 LAM-22 LAM-23 LAM-27

A lambda expression is a class written in place. The expression makes a
closure type that no other expression shares, with a call operator
whose parameters and body are the ones written. The call operator can
do everything that a class member function can do:

- a trailing return type or a deduced one,
- `constexpr` where the body permits it,
- `static` where there is nothing to capture,
- a template parameter from `auto`,
- an explicit object parameter.

The call operator is `const` unless the lambda says `mutable`, because a
copy of the enclosing variables should not change under a caller's
hands. A lambda is an expression, so it can stand where no evaluation
happens.
:::

:::{principle} LAM-P2
:rules: LAM-5 LAM-7 LAM-8 LAM-4 LAM-28 LAM-18 LAM-24

The closure reaches the automatic variables of the enclosing function
through its captures.

| Capture | What the closure holds |
|---|---|
| By copy | A member that the enclosing variable initializes when the lambda expression is evaluated. Later changes to the variable are invisible. |
| By reference | An alias of the variable itself |
| Init capture | A member with any initializer |
| `this` | A pointer |
| `*this` | A copy of the object |

The evaluation of the lambda expression initializes the captures. The
explicit captures go in the order of the capture list. The captures
exist before the call operator is declared, so the parameter list and
return type of the call operator can see the captures.
:::

:::{principle} LAM-P3
:rules: LAM-9 LAM-6 LAM-17

A closure must capture what its body uses and nothing it cannot. Only a
variable with automatic storage needs capturing, because statics and
globals are reachable from anywhere, and a constant read as a value is
not used as an object. Every such use must be covered, through each
enclosing lambda in turn. An implementation rejects the following:

- a capture list that names a variable twice,
- a capture list that repeats the default,
- a capture list that captures what has no automatic storage,
- a body that reaches a member or a local object without a capture
  that holds it.
:::

:::{principle} LAM-P4
:rules: LAM-10 LAM-11 LAM-25 LAM-26

A closure with no captures carries no state, so the closure is as good
as a function. A closure with captures is an object with state.

| Closure | Converts to a pointer to function | Type can be default-constructed and assigned |
|---|---|---|
| No captures | Yes | Yes |
| With captures | No | No. The closure must be made from the lambda expression. |
:::

:::{principle} LAM-P5
:rules: LAM-19 LAM-20 LAM-21

A reference capture is an alias, so the capture dangles when the
variable ends, and using the capture then is undefined. The layout of a
closure is the implementation's, since a program names the members of a
closure only through the captures. An implicit capture of `this` through
`[=]` hides a pointer behind a copy default, so an implementation should
warn about the implicit capture.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   lambda_expr: "[" [`captures`] "]" [`lambda_template`]
              :    ["(" `parameters` ")"] {`lambda_specifier`}
              :    [`noexcept_specifier`] ["->" `type_id`] [`requires_clause`]
              :    `compound_stmt`
   lambda_template: "<" `template_params` ">" [`requires_clause`] {`attribute_spec`}
   captures: `capture_default` {"," `capture`}
           : | `capture` {"," `capture`}
   capture_default: "=" | "&"
   capture: ["&"] `identifier` ["..."]
          : | ["&"] ["..."] `identifier` `init`
          : | "this" | "*" "this"
   lambda_specifier: "mutable" | "static" | "constexpr" | "consteval"
```

A `capture` with `...` captures a pack ({rule}`PACK-17`,
{rule}`PACK-18`).

## Constraints

:::{rule} LAM-1
:kind: required
:tests: expressions/lambdas/default-arguments.cpp expressions/lambdas/immediately-invoked.cpp expressions/lambdas/syntax-forms.cpp

A lambda can have these:

- parameters,
- default arguments,
- a trailing return type.

Without a trailing return type, the return type comes from the `return`
statements of the lambda, as for a function with an `auto` return type.
A program can call a lambda in the expression that creates it.
:::

:::{rule} LAM-22
:kind: reject
:tests: expressions/lambdas/return-deduction.cpp

Without a trailing return type, all `return` statements of a lambda must
give the same type, as for a function with an `auto` return type
(LAM-1).
:::

:::{rule} LAM-2
:kind: required
:tests: expressions/lambdas/unique-type.cpp

Each lambda expression has its own closure type. Two lambda expressions
never have the same type.
:::

:::{rule} LAM-3
:kind: required
:tests: expressions/lambdas/mutable.cpp expressions/lambdas/modify-through-reference-capture.cpp

The call operator is `const` unless the lambda is `mutable`. A variable
captured by reference can be changed also without `mutable`.
:::

:::{rule} LAM-23
:kind: reject
:tests: expressions/lambdas/const-call-operator.cpp

Without `mutable`, the body must not change a variable captured by copy
(LAM-3).
:::

:::{rule} LAM-5
:kind: required
:tests: expressions/lambdas/default-capture-mixed.cpp expressions/lambdas/default-captures.cpp

`[=]` captures by copy each local variable that the body uses. `[&]`
captures the same variables by reference. An explicit capture after a
default uses the other mode.
:::

:::{rule} LAM-6
:kind: reject
:tests: expressions/lambdas/capture-abstract.cpp expressions/lambdas/capture-copy-same-as-default.cpp expressions/lambdas/capture-default-at-namespace.cpp expressions/lambdas/capture-global-rejected.cpp expressions/lambdas/capture-repeated.cpp expressions/lambdas/capture-same-as-default.cpp expressions/lambdas/reference-default-explicit-this.cpp

A capture list must obey these rules:

- Each name occurs at most once.
- An explicit capture must not use the same mode as the default. As
  exceptions, `[&, this]`, `[=, this]`, and `[=, *this]` are valid.
- Only a variable with {term}`automatic storage` can be captured.
- A lambda outside a function must not have a capture default, except in
  a default member initializer.
- A variable of abstract class type ({rule}`VIRT-8`) cannot be captured
  by copy.
:::

:::{rule} LAM-7
:kind: required
:tests: expressions/lambdas/capture-structured-binding.cpp expressions/lambdas/init-capture.cpp expressions/lambdas/explicit-capture-order.cpp expressions/lambdas/namespace-init-capture.cpp expressions/lambdas/init-capture-outer-lookup.cpp expressions/sequencing/init-capture-order.cpp expressions/lambdas/init-capture-extends-temporary.cpp

An init capture `[x = e]` declares a new member `x` that `e`
initializes, and `[&r = v]` declares a reference to `v`. These rules
apply:

- Init captures are initialized in order with the other explicit
  captures (LAM-28).
- The temporaries of the initializer end with the full expression that
  contains the lambda expression ({rule}`VC-9`).
- An init capture by reference that binds to a temporary makes the
  temporary live as long as the closure object.
- A structured binding can be captured.
- A lambda at namespace scope can have an init capture.
- Name lookup for the initializer uses the enclosing scope, so the
  initializer can use an outer variable with the same name as the
  capture.
:::

:::{rule} LAM-8
:kind: required
:tests: expressions/lambdas/capture-this.cpp expressions/lambdas/default-member-initializer-captures-this.cpp

In a member function, `[this]` captures the object by reference, and
`[*this]` captures a copy of it. A lambda in a default member
initializer can capture `this` too.
:::

:::{rule} LAM-24
:kind: reject
:tests: expressions/lambdas/this-outside-member.cpp

The lambdas that LAM-8 gives are the only ones that can capture `this`.
Any other lambda outside a member function must not capture `this`.
:::

:::{rule} LAM-9
:kind: reject
:tests: expressions/lambdas/constant-needs-no-capture.cpp expressions/lambdas/nested-capture-ok.cpp expressions/lambdas/nested-capture.cpp expressions/lambdas/static-and-global-no-capture.cpp expressions/lambdas/uncaptured-variable.cpp expressions/lambdas/constant-value-no-capture.cpp

A lambda must capture each local automatic variable that its body uses.
For a nested lambda, each enclosing lambda must capture the variable
too. A constant that the body only reads as a value needs no capture.
Static and global variables need no capture.
:::

:::{rule} LAM-10
:kind: required
:tests: expressions/lambdas/function-pointer.cpp expressions/lambdas/noexcept-function-pointer.cpp

A lambda without captures converts to a pointer to a function with the
same parameters and return type. For a `noexcept` lambda, the pointer is
a pointer to a `noexcept` function.
:::

:::{rule} LAM-25
:kind: reject
:tests: expressions/lambdas/capturing-no-function-pointer.cpp

A lambda with captures does not convert to a pointer to a function
(LAM-10).
:::

:::{rule} LAM-11
:kind: required
:tests: expressions/lambdas/default-constructible.cpp

The closure type of a lambda without captures can be default-constructed
and assigned.
:::

:::{rule} LAM-26
:kind: reject
:tests: expressions/lambdas/capturing-not-default-constructible.cpp expressions/lambdas/closure-copy-assign-deleted.cpp

The closure type of a lambda with captures cannot be default-constructed
or assigned (LAM-11).
:::

:::{rule} LAM-12
:kind: required
:tests: expressions/lambdas/constexpr-implicit.cpp

The call operator is `constexpr` when the body permits `constexpr`. A
program can call such a lambda during {term}`constant evaluation`.
:::

:::{rule} LAM-13
:kind: required
:tests: expressions/lambdas/static-call-operator.cpp

A lambda declared `static` has a static call operator. The closure
also converts to a pointer to function ({rule}`LAM-10`). A call of the
closure object chooses the static call operator over the surrogate
candidate from that conversion ({rule}`OVL-25`).
:::

:::{rule} LAM-27
:kind: reject
:tests: expressions/lambdas/static-with-capture.cpp

A lambda declared `static` must not have captures (LAM-13).
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
({rule}`DED-34`). Each call deduces the parameter type.
:::

:::{rule} LAM-16
:kind: required
:tests: expressions/lambdas/explicit-object-recursion.cpp

A lambda can have an explicit object parameter `this auto self`. The
body can call `self` to call the lambda again.
:::

:::{rule} LAM-17
:kind: reject
:tests: expressions/lambdas/default-argument-capture.cpp expressions/lambdas/parameter-same-as-capture.cpp expressions/lambdas/capture-data-member-by-name.cpp expressions/lambdas/member-use-without-this.cpp expressions/lambdas/constant-address-needs-capture.cpp expressions/lambdas/explicit-object-mutable.cpp

A lambda must not:

- capture anything if it is in a default argument,
- have a parameter with the name of an explicit capture,
- capture a non-static data member by its name; it needs `this` or an
  init capture,
- use a member of the current object without capturing `this`, directly
  or through a capture default,
- take the address of a local constant, or bind a reference to the
  constant, without capturing the constant,
- be `mutable` and have an explicit object parameter.
:::

:::{rule} LAM-18
:kind: required
:tests: expressions/lambdas/capture-visible-in-trailing-return.cpp

The captures of a lambda are visible in its parameter list and trailing
return type. In a lambda that is not `mutable`, `decltype((x))` for a
copy capture `x` is a reference to `const`.
:::

## Runtime Behavior

:::{rule} LAM-4
:kind: required
:tests: expressions/lambdas/capture-by-copy-at-creation.cpp expressions/lambdas/capture-by-reference.cpp expressions/lambdas/copy-capture-array-and-reference.cpp

A capture by copy copies the variable when the lambda expression is
evaluated. A capture by reference refers to the variable itself, so
later changes to the variable are visible. A capture by copy copies
these:

- each element, for an array,
- the object that the reference refers to, for a reference.
:::

:::{rule} LAM-28
:kind: required
:tests: expressions/lambdas/explicit-capture-order.cpp

The explicit captures by copy and the init captures are initialized in
the order of the capture list. LAM-20 gives the order of the implicit
captures.
:::

## Undefined Behavior

:::{rule} LAM-19
:kind: undefined
:tests: expressions/lambdas/dangling-reference-capture-constant.cpp expressions/lambdas/capture-outlived-sanitizer.cpp

If a lambda that captured a variable by reference is called after the
life of the variable ends, and the body uses the capture, the behavior
is undefined.
:::

## Implementation Requirements

:::{rule} LAM-20
:kind: unspecified
:tests: expressions/lambdas/closure-size-unspecified.cpp expressions/lambdas/implicit-capture-order.cpp

These properties of a closure type are unspecified:

- the order of its members,
- its size,
- whether a capture by reference adds a member,
- the order in which the implicit captures are initialized, among
  themselves and relative to the explicit captures.
:::

## Recommended Practice

:::{rule} LAM-21
:kind: recommended
:tests: expressions/lambdas/implicit-this-capture-warns.cpp

An implementation should warn when `[=]` captures `this` implicitly. Such a
capture is deprecated.
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 58
of 60 tests.

- LAM-6: EDG accepts a capture default on a lambda outside a function.
  Clang and GCC reject it ({question}`Q-0037`).
- LAM-13 and LAM-16: all seven compilers accept static lambdas and
  explicit object parameters in C++23 mode.
- LAM-13: every compiler accepts a call of a closure object with a
  static call operator, although the closure also converts to a pointer
  to function. OVL-25 gives the reason.
- LAM-20: every compiler initializes the explicit captures first. The
  implicit captures follow, in the order of their first use in the body.
  LAM-20 allows this order.
- LAM-18: GCC 16 and EDG find the capture in the trailing return type.
  Clang and GCC 14 find the outer variable ({question}`Q-0092`).

Inputs: clang-sema-expr, clang-diag-sema.

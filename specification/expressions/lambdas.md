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

The evaluation of the lambda expression initializes the captures, in an
order that the implementation chooses. Every compiler of the matrix
initializes the explicit captures in the order of the capture list. A trailing
return type comes after the parameters and can see the captures. The
parameters are declared as the parameters of any function are, so a
name in the parameter list means the enclosing variable.
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
  that holds it,
- a capture default on a lambda that is not inside a block and is not
  in a default member initializer.

A capture default asks for captures from the block that encloses the
lambda, or from the object of a default member initializer through
`this`. A lambda at namespace scope has neither. Neither has a lambda in
a default argument, or in a member declaration of a local class outside
any function body, even though a function encloses the class. So a
capture default in those places is rejected even when the body uses
nothing that needs a capture.
:::

:::{principle} LAM-P4
:rules: LAM-10 LAM-11 LAM-25 LAM-26 LAM-29

A closure with no captures carries no state, so the closure is as good
as a function. A closure with captures is an object with state.

| Closure | Converts to a pointer to function | Type can be default-constructed and assigned | Can be copied and moved |
|---|---|---|---|
| No captures | Yes | Yes | Yes |
| With captures | No | No. The closure must be made from the lambda expression. | Yes, if each capture can |

Each closure has its own type ({rule}`LAM-2`). A program without the
library keeps closures of different types together through a common
base with a virtual call operator, or through function pointers for
closures without captures.
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
:tests: expressions/lambdas/return-deduction.cpp expressions/lambdas/return-deduction-void-and-value.cpp

Without a trailing return type, all `return` statements of a lambda must
give the same type, as for a function with an `auto` return type
(LAM-1).
:::

:::{rule} LAM-2
:kind: required
:tests: expressions/lambdas/unique-type.cpp expressions/lambdas/distinct-types-overload.cpp

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
:tests: expressions/lambdas/const-call-operator.cpp expressions/lambdas/const-call-operator-init-capture.cpp

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
:tests: expressions/lambdas/capture-abstract.cpp expressions/lambdas/capture-copy-same-as-default.cpp expressions/lambdas/capture-default-at-namespace.cpp expressions/lambdas/capture-global-rejected.cpp expressions/lambdas/capture-repeated.cpp expressions/lambdas/capture-same-as-default.cpp expressions/lambdas/reference-default-explicit-this.cpp expressions/lambdas/capture-default-in-member-initializer.cpp expressions/lambdas/capture-default-local-class-member.cpp expressions/lambdas/capture-default-local-class-default-argument.cpp

A capture list must obey these rules:

- Each name occurs at most once.
- An explicit capture must not use the same mode as the default. As
  exceptions, `[&, this]`, `[=, this]`, and `[=, *this]` are valid.
- Only a variable with {term}`automatic storage` can be captured.
- A lambda that is not inside a block must not have a capture default,
  except in a default member initializer. A default argument, and a
  member declaration of a local class outside a function body, are not
  inside a block.
- A variable of {term}`abstract class` type ({rule}`VIRT-8`) cannot be captured
  by copy.
:::

:::{rule} LAM-7
:kind: required
:tests: expressions/lambdas/capture-structured-binding.cpp expressions/lambdas/init-capture.cpp expressions/lambdas/explicit-capture-order.cpp expressions/lambdas/namespace-init-capture.cpp expressions/lambdas/init-capture-outer-lookup.cpp foundations/evaluation-order/init-capture-order.cpp expressions/lambdas/init-capture-extends-temporary.cpp

An init capture `[x = e]` declares a new member `x` that `e`
initializes, and `[&r = v]` declares a reference to `v`. These rules
apply:

- Init captures are initialized together with the other explicit
  captures, in the order of LAM-28.
- The temporaries of the initializer end with the {term}`full expression` that
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
:tests: expressions/lambdas/this-outside-member.cpp expressions/lambdas/this-in-static-member.cpp

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
:tests: expressions/lambdas/function-pointer.cpp expressions/lambdas/noexcept-function-pointer.cpp expressions/lambdas/generic-function-pointer.cpp expressions/lambdas/function-pointer-constant.cpp expressions/lambdas/unary-plus-function-pointer.cpp

A lambda without captures converts to a pointer to a function with the
same parameters and return type. For a `noexcept` lambda, the pointer is
a pointer to a `noexcept` function. A generic lambda without captures
converts to a pointer to function of any type that {term}`deduction` from the
call operator template can match, as for a target type
({rule}`OVL-18`). Constant evaluation can use the conversion, and a call
through the pointer if the call operator is `constexpr` ({rule}`LAM-12`).
:::

:::{rule} LAM-25
:kind: reject
:tests: expressions/lambdas/capturing-no-function-pointer.cpp expressions/lambdas/capturing-no-function-pointer-argument.cpp

A lambda with captures does not convert to a pointer to a function
(LAM-10).
:::

:::{rule} LAM-11
:kind: required
:tests: expressions/lambdas/default-constructible.cpp expressions/lambdas/default-constructed-call.cpp

The closure type of a lambda without captures can be default-constructed
and assigned.
:::

:::{rule} LAM-29
:kind: required
:tests: expressions/lambdas/closure-copy-keeps-state.cpp expressions/lambdas/closure-copy-array-capture.cpp

A closure type can be copied and moved if each of its captures can. The
copy starts with the state of the original.
:::

:::{rule} LAM-26
:kind: reject
:tests: expressions/lambdas/capturing-not-default-constructible.cpp expressions/lambdas/closure-copy-assign-deleted.cpp

The closure type of a lambda with captures cannot be default-constructed
or assigned (LAM-11).
:::

:::{rule} LAM-12
:kind: required
:tests: expressions/lambdas/constexpr-implicit.cpp expressions/lambdas/function-pointer-constant.cpp

The call operator is `constexpr` when the body permits `constexpr`. A
program can call such a lambda during {term}`constant evaluation`.
:::

:::{rule} LAM-13
:kind: required
:tests: expressions/lambdas/static-call-operator.cpp expressions/lambdas/static-call-operator-named.cpp

A lambda declared `static` has a static call operator. The closure
also converts to a pointer to function ({rule}`LAM-10`). A call of the
closure object chooses the static call operator over the {term}`surrogate candidate`
from that conversion ({rule}`OVL-25`).
:::

:::{rule} LAM-27
:kind: reject
:tests: expressions/lambdas/static-with-capture.cpp expressions/lambdas/static-with-capture-default.cpp

A lambda declared `static` must not have captures (LAM-13).
:::

:::{rule} LAM-14
:kind: required
:tests: expressions/lambdas/unevaluated-operand.cpp expressions/lambdas/unevaluated-noexcept.cpp

A lambda can occur in an operand that is not evaluated, such as the
operand of `decltype` or `sizeof`.
:::

:::{rule} LAM-15
:kind: required
:tests: expressions/lambdas/generic.cpp templates/dependent-names/generic-lambda-argument-lookup.cpp expressions/lambdas/template-head.cpp expressions/lambdas/generic-pack.cpp

A parameter of type `auto` makes the call operator a template
({rule}`DED-34`). Each call deduces the parameter type. `auto...`
gives the call operator a pack ({rule}`PACK-1`). A template head after
the captures, as in `[]<class T>(T x) { ... }`, makes the call operator
a template with those parameters. A call deduces them as for a function
template, or names them in `l.operator()<int>()`. The body is a
{term}`templated context` also at namespace scope, so its dependent
calls look up names as {rule}`DEP-9` gives, at the points of
{rule}`INST-31`.
:::

:::{rule} LAM-16
:kind: required
:tests: expressions/lambdas/explicit-object-recursion.cpp expressions/lambdas/explicit-object-recursion-run.cpp expressions/lambdas/explicit-object-recursion-constant.cpp

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
:tests: expressions/lambdas/capture-visible-in-trailing-return.cpp expressions/decltype/captured-in-lambda.cpp expressions/lambdas/explicit-capture-unevaluated-member.cpp expressions/lambdas/capture-not-visible-in-parameter-list.cpp

The captures of a lambda are visible in its trailing return type and its
body. In the parameter list, a name refers to the enclosing variable,
also in a parameter type and a default argument ({question}`Q-0092`). In
a lambda that is not `mutable`, `decltype((x))` for a
copy capture `x` is a reference to `const`. The same holds for a name
that a copy default would capture. The rule applies also if the lambda
names `x` only in unevaluated operands. Such a use makes no implicit
capture, and an explicit capture still makes a member.
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
:tests: expressions/lambdas/closure-size-unspecified.cpp expressions/lambdas/closure-alignment-unspecified.cpp expressions/lambdas/implicit-capture-order.cpp expressions/lambdas/closure-trivially-copyable-unspecified.cpp

These properties of a closure type are unspecified:

- the order of its members, also for the explicit captures by copy,
- its size and its alignment,
- whether a capture by reference adds a member,
- whether the closure type is {term}`trivially copyable`
  ({rule}`CPY-19`) or {term}`standard-layout` ({rule}`CLS-19`),
- the order in which the implicit captures are initialized, among
  themselves and relative to the explicit captures.

A capture by copy adds a member of the captured type. So the closure is
at least as large as each such member.
:::

:::{rule} LAM-28
:kind: unspecified
:tests: expressions/lambdas/explicit-capture-order.cpp expressions/lambdas/init-capture-order.cpp foundations/evaluation-order/init-capture-order.cpp

The order in which the explicit captures by copy and the init captures
are initialized is unspecified, as is the order of the members that
hold them ({rule}`LAM-20`). On the matrix, every compiler initializes
them in the order of the capture list. LAM-20 gives the order of the
implicit captures.
:::

## Recommended Practice

:::{rule} LAM-21
:kind: recommended
:tests: expressions/lambdas/implicit-this-capture-warns.cpp expressions/lambdas/implicit-this-capture-call-warns.cpp

An implementation should warn when `[=]` captures `this` implicitly. Such a
capture is deprecated.
:::

## Examples

% tidal-example: fragment
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
auto q = +[](int x) { return x * 2; };   // int (*)(int): + applies to
                                         // the converted pointer
```

Closures of different types in one table, without the library. A
holder template derives from a base with a virtual call operator
({rule}`VIRT-1`):

```cpp
struct Handler {
  virtual int operator()(int) = 0;
  virtual ~Handler() = default;
};
template <class F> struct Holder : Handler {
  F f;
  Holder(F g) : f(g) {}
  int operator()(int v) override { return f(v); }
};
template <class F> Handler* hold(F f) { return new Holder<F>(f); }

int dispatch(int base) {
  Handler* table[2] = {hold([](int v) { return v + 1; }),
                       hold([base](int v) { return v + base; })};
  int r = (*table[1])(5);                // base + 5
  for (Handler* h : table) delete h;
  return r;
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 85
of 89 tests.

### Divergences

- {question}`Q-0037` (LAM-6; settled on LAM-P3): whether a lambda
  outside a block can have a capture default. It cannot, at namespace
  scope, in a default argument, or in a member declaration of a local
  class, as Clang and GCC say. EDG accepts all three.
- {question}`Q-0092` (LAM-18; open): whether a name in a lambda's
  trailing return type finds a capture. It does, as GCC 16 and EDG say.
  Clang and GCC 14 find the outer variable. In the parameter list,
  every compiler finds the outer variable, so `decltype((x))` there is
  `int&`.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `capture-default-at-namespace.cpp` | LAM-6 | reject | reject | reject | accept | {question}`Q-0037` |
| `capture-default-local-class-default-argument.cpp` | LAM-6 | reject | reject | reject | accept | {question}`Q-0037` |
| `capture-default-local-class-member.cpp` | LAM-6 | reject | reject | reject | accept | {question}`Q-0037` |
| `capture-visible-in-trailing-return.cpp` | LAM-18 | accept | reject | GCC 16: accept; GCC 14: reject | accept | {question}`Q-0092` |

### Notes

- LAM-6: the gap review of the text phase 8 wrote on expressions
  (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`)
  reads the limit as a block around the lambda, so a lambda in a member
  declaration or a default argument of a local class has none.
- LAM-13 and LAM-16: all seven compilers accept static lambdas and
  explicit object parameters in C++23 mode.
- LAM-13: every compiler accepts a call of a closure object with a
  static call operator, although the closure also converts to a pointer
  to function. OVL-25 gives the reason.
- LAM-28: the gap review of the text phase 8 wrote on expressions,
  foundations, and templates
  (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`)
  reads the order of the closure's members, and so the order of their
  initialization, as unspecified, for explicit captures and init
  captures alike. LAM-28 is `unspecified`, and UNW-2 no longer promises
  the order of the capture list. Every compiler of the matrix uses that
  order (`explicit-capture-order.cpp`, `init-capture-order.cpp`).
- LAM-20: every compiler initializes the explicit captures first. The
  implicit captures follow, in the order of their first use in the body.
  LAM-20 allows this order.
- LAM-19: at run time, `-fsanitize=address` in Clang and GCC stops a
  call that reads a reference capture after the variable's scope ends.
  The report names a stack use after scope
  (`capture-outlived-sanitizer.cpp`).
- LAM-20: on every compiler, a closure that copies an `int` is at least
  as large as an `int`, as the copy member requires.
- LAM-20: on every compiler, a closure that copies a `double` and a
  `char` has the size and alignment of a struct with those two members
  (`closure-alignment-unspecified.cpp`). The alignment bullet and the
  order of the members for explicit captures follow the gap review of
  item O (`reports/review/2026-11-03-gap-review-item-o.md`).
- LAM-20: on every compiler, a closure that captures an `int` by copy
  or by reference has trivial copy and move constructors and a trivial
  destructor, so a union can hold it and copy it. The builtin queries
  of Clang, GCC, and EDG call each closure in the grid
  `trivially-copyable.json` trivially copyable. In the grid
  `standard-layout.json`, they call a closure with a copy capture
  standard-layout and a closure with a reference capture not
  standard-layout (inventory item TYPE-closure).
- LAM-21: GCC warns under `-Wdeprecated` and Clang under
  `-Wdeprecated-this-capture`. Both warnings are on by default in C++20
  and later, and `-pedantic-errors` does not make them errors. With
  `-Werror`, alone or with `-Wall -Wextra`, both compilers reject the
  program. EDG does not accept these flags and skips the test.
- LAM-18: Clang warns under `-Wall` (`-Wunused-lambda-capture`) about an
  explicit capture that the body names only in unevaluated operands.
  GCC does not warn. On every compiler, the closure is at least as
  large as the captured object.

Inputs: clang-sema-expr, clang-diag-sema.

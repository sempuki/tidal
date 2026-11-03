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
:rules: CX-3 CX-4 CX-5

`constexpr` on a function is a permission. It lets constant evaluation
call the function ({principle}`TR-P4`), and the same function still
runs at run time with the same meaning. The body can contain operations
that constant evaluation cannot do, as long as no constant evaluation
reaches them ({rule}`CE-17`). Because every {term}`translation unit` that
evaluates the function needs its body, the function is `inline`. The
specifier applies to functions and variables only, and all
declarations of a function must agree on it.
:::

:::{principle} CX-P2
:rules: CX-1 CX-2

`constexpr` on a variable makes the variable a constant:

- the variable is `const`,
- its initializer is a constant expression,
- its type can be evaluated at translation time.

A pointer or reference in a constant, also one in a member, must refer
to a function or an object with static storage duration, never an
automatic variable or a temporary that lives only as long as one.
The target must exist at translation time and still exist at run time.
:::

:::{principle} CX-P3
:rules: CEV-1 CEV-2 CX-4 CX-5

`consteval` on a function is a demand. Every call is evaluated at
translation time, so a call with arguments that are not constant is an
error unless the call sits inside another function that constant
evaluation is already running. A function that makes such a call becomes
`consteval` itself when the language allows, and otherwise the call is
rejected. The function has no run-time existence, so its address can be
taken only where constant evaluation is in progress. `consteval`
applies to functions only, and a `consteval` function is `inline` for
the same reason as a `constexpr` function.
:::

:::{principle} CX-P4
:rules: CINIT-1 CINIT-2

`constinit` is about initialization only. The variable must live for the
whole program or the whole thread, and its initializer must be a
constant, so that the variable already has its value before any other
static object can look at the variable. The specifier does not make the
variable `const` and asks nothing of the destructor. A declaration can
have `constinit` or `constexpr`, and at most one of them.

The declaration that initializes a `constinit` variable carries
`constinit`, so the check happens where the initializer is. An earlier
declaration, such as an `extern` declaration in a header, can carry
`constinit` or leave it out.
:::

## Constraints

:::{rule} CX-1
:kind: reject
:tests: declarations/constexpr/variable-const.cpp declarations/constexpr/variable-not-constant.cpp declarations/constexpr/variable-no-initializer.cpp declarations/constexpr/variable-non-literal.cpp declarations/constexpr/variable-assign.cpp declarations/constexpr/constexpr-ref-temporary-namespace.cpp declarations/constexpr/constexpr-ref-temporary-block.cpp declarations/constexpr/variable-default-constructor.cpp declarations/constexpr/variable-default-initialized-no-value.cpp expressions/constant-expressions/literal-destructor.cpp expressions/constant-expressions/literal-member.cpp expressions/constant-expressions/literal-base.cpp expressions/constant-expressions/literal-volatile-member.cpp expressions/constant-expressions/literal-union-all-volatile.cpp declarations/constexpr/variable-implicit-constructor-member-initializers.cpp declarations/constexpr/variable-implicit-constructor-member-unset.cpp declarations/constexpr/aggregate-reference-member-temporary-block.cpp

A `constexpr` variable is `const`, and it must obey these rules:

- The variable must have an initializer, unless it has class type and a
  `constexpr` {term}`default constructor` gives every member a value. An
  implicit default constructor does so when CTOR-4 holds for the class.
- Its initialization must be a constant expression.
- Its type must be a {term}`literal type` (CE-23).
- A `constexpr` reference in a block, without `static`, must not bind
  to a temporary (CX-2).

A `constexpr` reference at namespace scope can bind to a temporary,
because that temporary has static storage.
:::

:::{rule} CX-2
:kind: reject
:tests: declarations/constexpr/reference-to-static.cpp declarations/constexpr/reference-to-automatic.cpp declarations/constexpr/pointer-to-automatic.cpp declarations/constexpr/pointer-one-past-end.cpp declarations/constexpr/static-local-to-static-local.cpp declarations/constexpr/aggregate-reference-member-temporary-block.cpp declarations/constexpr/aggregate-reference-member-temporary-namespace.cpp declarations/constexpr/pointer-to-string-literal.cpp

A `constexpr` variable holds references and pointers in itself and in its
members. Each such reference, and each such pointer that is not null,
must refer to one of these:

- a function,
- an object with static {term}`storage duration`, such as a string
  literal ({rule}`PRIM-7`),
- for a pointer only, the place one past the end of such an object.

A temporary that the initializer extends ({rule}`REF-4`) has the storage
duration of the variable. So a `constexpr` variable in a block without
`static` must not refer to an automatic variable or to such a temporary.
:::

:::{rule} CX-3
:kind: required
:tests: declarations/constexpr/function-compile-and-run.cpp declarations/constexpr/function-never-constant.cpp declarations/constexpr/function-loops-and-locals.cpp declarations/constexpr/function-static-local-unevaluated.cpp declarations/constexpr/function-static-constexpr-local.cpp declarations/constexpr/function-goto-unevaluated.cpp declarations/constexpr/function-non-literal-param.cpp declarations/constexpr/function-non-literal-local-unevaluated.cpp declarations/constexpr/non-literal-return.cpp

A program can call a `constexpr` function during {term}`constant
evaluation` and at run time. The body can contain the operations that
{rule}`CE-2` and {rule}`CE-17` reject, on paths that the call does not
evaluate. A function that never gives a constant expression is valid.

The return type and the parameter types of a `constexpr` function need
not be literal types.
:::

:::{rule} CX-4
:kind: required
:tests: declarations/constexpr/function-implicitly-inline.cpp declarations/constexpr/consteval-inline.cpp

A `constexpr` or `consteval` function is `inline` (LNK-4).
:::

:::{rule} CX-5
:kind: reject
:tests: declarations/constexpr/constexpr-class-only.cpp declarations/constexpr/constexpr-redeclaration-mismatch.cpp declarations/constexpr/constexpr-parameter.cpp declarations/constexpr/consteval-variable.cpp declarations/constexpr/consteval-and-constexpr.cpp declarations/constexpr/consteval-redeclare-constexpr.cpp

`constexpr` and `consteval` can appear only on these declarations:

| Specifier | Can appear on |
|---|---|
| `constexpr` | a function, or a variable that is not a parameter |
| `consteval` | a function |

A declaration must not have both specifiers. All declarations of a
function must agree on each of the two specifiers.
:::

:::{rule} CEV-1
:kind: reject
:tests: declarations/constexpr/consteval-call-constant.cpp declarations/constexpr/consteval-call-runtime-arg.cpp declarations/constexpr/consteval-in-consteval.cpp declarations/constexpr/consteval-ub-rejected.cpp declarations/constexpr/immediate-escalation.cpp declarations/constexpr/lambda-escalates-constant.cpp declarations/constexpr/lambda-escalates-runtime.cpp declarations/constexpr/consteval-returns-allocation.cpp declarations/constexpr/plain-constexpr-no-escalation.cpp declarations/constexpr/consteval-converting-constructor.cpp declarations/constexpr/consteval-converting-constructor-throws.cpp

Each call to a `consteval` function must be a constant expression. A
`consteval` constructor counts, also when an implicit conversion calls
it. This rule does not apply to a call in an {term}`immediate function context`,
such as the body of another `consteval` function.

These functions take on `consteval` from their bodies:

- a `constexpr` function produced from a template
  ({doc}`/templates/instantiation`),
- a lambda without `consteval` on it,
- a {term}`special member function` that the class defaults.

If such a function calls a `consteval` function and the arguments are
not constant, the calling function is treated as `consteval` too, and
its own calls are then checked the same way.

The result of a `consteval` call must be usable in a constant
expression. A pointer to storage that the call allocated and did not
release is not.
:::

:::{rule} CEV-2
:kind: reject
:tests: declarations/constexpr/consteval-address-taken.cpp declarations/constexpr/consteval-address-in-immediate.cpp declarations/constexpr/consteval-selected-by-target.cpp declarations/constexpr/consteval-not-selected-by-target.cpp

A program can take the address of a `consteval` function only in an
immediate function context. A target type that selects a `consteval`
function from an {term}`overload set` ({rule}`OVL-18`) takes its address.
:::

:::{rule} CINIT-1
:kind: reject
:tests: declarations/constexpr/constinit-static.cpp declarations/constexpr/constinit-not-const.cpp declarations/constexpr/constinit-dynamic.cpp declarations/constexpr/constinit-nonconstexpr-dtor.cpp expressions/constant-expressions/constant-initializer-not-constant.cpp expressions/constant-expressions/constant-initializer-zero-trivial-class.cpp expressions/constant-expressions/constant-initializer-trivial-class-block-static.cpp expressions/constant-expressions/constant-initializer-trivial-class-array.cpp

A `constinit` variable must have a {term}`constant initializer`. For a
variable with no initializer, {rule}`CE-24` and {rule}`CE-21` say when
the {term}`default initialization` is one. `constinit` does not make the
variable `const`, so the program can
change the variable. `constinit` does not require a `constexpr`
destructor.
:::

:::{rule} CINIT-2
:kind: reject
:tests: declarations/constexpr/constinit-automatic.cpp declarations/constexpr/constinit-block-static.cpp declarations/constexpr/constinit-with-constexpr.cpp declarations/constexpr/constinit-extern-declaration.cpp declarations/constexpr/constinit-only-on-definition.cpp declarations/constexpr/constinit-on-function.cpp declarations/constexpr/constinit-added-late.cpp declarations/constexpr/constinit-on-declaration-only.cpp

`constinit` applies only to variables, and a `constinit` variable must
have static or thread {term}`storage duration`. A declaration must not
have both `constinit` and `constexpr`.
If a declaration of a variable has `constinit`, the initializing
declaration must also have it ({question}`Q-0025`).
:::

:::{rule} CX-6
:kind: withdrawn

This rule merged into {rule}`CE-2` and {rule}`CE-17`.
:::

:::{rule} CEV-3
:kind: withdrawn

This rule merged into {rule}`CX-5`.
:::

:::{rule} CEV-4
:kind: withdrawn

This rule merged into {rule}`CX-4`.
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

// header
extern constinit int first_ticket;  // constinit here is optional
// one source file
constinit int first_ticket = 100;   // constinit here is required
```

A table built during translation. A function cannot return an array, so
the table is a struct with an array member. The default member
initializer gives every element a value ({rule}`CE-21`):

```cpp
struct Squares { int v[16] = {}; };
constexpr Squares build() {
  Squares s;
  for (int i = 0; i < 16; ++i) s.v[i] = i * i;
  return s;
}
constexpr Squares squares = build();
static_assert(squares.v[5] == 25);
```

A `consteval` constructor checks its argument where the program converts
it. A `throw` on bad input makes the call not constant, so the
implementation rejects the declaration ({rule}`CE-8`):

```cpp
struct Identifier {
  const char* text;
  consteval Identifier(const char* s) : text(s) {
    if (s[0] >= '0' && s[0] <= '9') throw "starts with a digit";
  }
};
Identifier good = "counter";        // fine
Identifier bad = "2x";              // rejected
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 64
of 72 tests.

- {rule}`CE-17`: every compiler accepts a constant evaluation that
  reaches the definition of a `static const int` with a constant
  initializer (`static-local-usable-reached.cpp`). Every
  compiler rejects one that reaches a `static int`
  (`function-static-local-evaluated.cpp`).
- CX-3: Clang 18 rejects a `constexpr` function that never gives a
  constant, a non-literal parameter type, and a non-literal return
  type. Clang 22 accepts all three ({question}`Q-0020`).
- CEV-1: all eight compilers, Clang 18 and GCC 14 included, make a
  lambda and a `constexpr` function from a template `consteval` when
  the body calls a `consteval` function with arguments that are not
  constant.
- CINIT-2: Clang rejects `constinit` added after the initializing
  declaration. GCC and EDG accept the late `constinit`. Clang
  (pedantic) and EDG reject `constinit` that is only on an earlier
  declaration. GCC accepts the earlier `constinit`
  ({question}`Q-0025`).
- CX-2: all compilers reject a `constexpr` aggregate in a block whose
  reference member binds to a temporary, and accept the same aggregate
  at namespace scope. A `constexpr` reference variable that binds a
  temporary aggregate is a different case, which {rule}`CE-14` covers
  ({question}`Q-0193`). Q-0193 stays open under decision 0051, because
  on 2026-11-03 the newest versions still split on it.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls.

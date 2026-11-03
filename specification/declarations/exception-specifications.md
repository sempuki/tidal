# Exception specifications

A `noexcept` specifier states that a function does not let an exception
leave it. The specifier is part of the function type. This page covers:

- the forms of the specifier,
- its place in function and pointer types,
- overriding,
- the functions that are `noexcept` without the keyword,
- operands that depend on template parameters.

The rules for exception handling give the behavior when an exception
does leave a `noexcept` function ({rule}`THR-9`). DCL-14 and CONV-10
give the rules for redeclarations and pointer conversions.

Depends on: declarators, standard conversions, virtual functions,
constructors and destructors, constant expressions.

## Principles

:::{principle} EXS-P1
:rules: EXS-1 EXS-7 EXS-2 EXS-8

A `noexcept` specifier is part of the function's type, so it travels
with the type through:

- type aliases,
- pointers,
- references,
- `decltype`.

So a call through a pointer to a non-throwing function is itself
non-throwing, and two functions that differ only in the specifier are
the same function declared twice. The specifier is a constant, decided
at translation time from an expression that can see the parameters and
the whole class.
:::

:::{principle} EXS-P2
:rules: EXS-3

A caller that holds a pointer to a base relies on the base's promise. So
an override can make the promise but cannot take it back. A non-throwing
virtual function can be overridden only by a non-throwing function, and
a destructor is no exception.
:::

:::{principle} EXS-P3
:rules: EXS-4

Only the program knows the body of a function that the program writes. A
function that the implementation writes gets its specifier from the
functions that it calls. A destructor gets its specifier from the
destructors of its parts, also when the program writes its body.

| Function | Specifier when the declaration says nothing |
|---|---|
| Written by the program, other than a destructor | Potentially throwing |
| A destructor, also one that the program writes | Non-throwing when the destructors of all bases and members are non-throwing ({rule}`DTOR-4`) |
| A defaulted special member or comparison | Non-throwing when every part that the function touches is non-throwing |
| A deallocation function | Non-throwing |

:::

:::{principle} EXS-P4
:rules: EXS-5 EXS-6

An implementation can accept the old `throw()` form as `noexcept`. An
operand that refers to the function that the specifier qualifies is a
cycle that an implementation need not detect.
:::

:::{principle} EXS-P5
:rules: EXS-9 EXS-10

A template can make the specifier depend on its template arguments, so
that each {term}`specialization` makes its own promise. A copy of a
container, for example, can be non-throwing exactly when the copy of an
element is. The implementation works out the specifier of a
specialization only when a use needs it. So a class template can
declare a member whose operand is not valid for some arguments, and the
program stays valid while no use needs that specifier.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   noexcept_specifier: "noexcept" ["(" `constant_expr` ")"]
```

The specifier follows the parameter list and the qualifiers of a
function declarator. `noexcept` alone means `noexcept(true)`. A function
with `noexcept(true)` is a *non-throwing* function.

## Constraints

:::{rule} EXS-1
:kind: required
:tests: declarations/exception-specifications/forms.cpp declarations/exception-specifications/operand-later-member.cpp declarations/exception-specifications/operand-uses-parameters.cpp

The operand of `noexcept` is a constant expression that converts to
`bool`. In a member function, the operand can name members declared
later in the class. The operand can also use the parameters of the
function. EXS-7 lists what is not valid.
:::

:::{rule} EXS-7
:kind: reject
:tests: declarations/exception-specifications/narrowing-operand.cpp declarations/exception-specifications/non-constant.cpp declarations/exception-specifications/pointer-operand.cpp declarations/exception-specifications/throw-type-removed.cpp

The operand of `noexcept` (EXS-1) must be a constant expression. The
conversion of the operand to `bool` must not narrow
({question}`Q-0078`). A list of types in the form `throw(T)` is not
valid.
:::

:::{rule} EXS-2
:kind: required
:tests: declarations/exception-specifications/alias-noexcept-function.cpp declarations/exception-specifications/call-through-noexcept-pointer.cpp declarations/exception-specifications/decltype-keeps-noexcept.cpp declarations/exception-specifications/in-type.cpp declarations/exception-specifications/member-function-noexcept.cpp declarations/exception-specifications/nested-indirection.cpp declarations/exception-specifications/parameter-noexcept-overloads.cpp declarations/exception-specifications/typedef-noexcept.cpp

The `noexcept` specifier is part of a function type. The specifier is
kept in:

- type aliases and `typedef` names,
- pointers to functions, at any depth, and pointers to member functions,
- the type that `decltype` gives.

A call through a pointer to a non-throwing function is non-throwing.
Parameter types that differ in `noexcept` are different types, so
functions with those parameter types overload. For a `noexcept`
argument, the overload that keeps `noexcept` is better
({rule}`ICS-26`). EXS-8 lists what is not valid.
:::

:::{rule} EXS-8
:kind: reject
:tests: declarations/exception-specifications/noexcept-in-function-redeclare-later.cpp declarations/exception-specifications/overload-on-noexcept.cpp

Two declarations that differ only in `noexcept` are not overloads
(EXS-2), and the second one is not valid.
:::

:::{rule} EXS-3
:kind: reject
:tests: declarations/exception-specifications/override-dtor-noexcept-false.cpp declarations/exception-specifications/override-may-add-noexcept.cpp declarations/exception-specifications/override-must-be-noexcept.cpp

A function that overrides a non-throwing virtual function must be
non-throwing. The rule applies to destructors too. An override can be
non-throwing when the overridden function is not.
:::

:::{rule} EXS-4
:kind: required
:tests: declarations/exception-specifications/constexpr-not-implied.cpp declarations/exception-specifications/defaulted-explicit-noexcept.cpp declarations/exception-specifications/defaulted-follows-members.cpp declarations/exception-specifications/delete-operator-noexcept.cpp declarations/exception-specifications/implicit-follows-members.cpp declarations/exception-specifications/implicit-special-members.cpp declarations/exception-specifications/inheriting-ctor-noexcept.cpp declarations/exception-specifications/lambda-not-noexcept.cpp declarations/exception-specifications/user-function-not-noexcept.cpp declarations/exception-specifications/delete-class-follows-destructor.cpp declarations/exception-specifications/class-delete-operator-noexcept.cpp declarations/exception-specifications/default-member-init-throwing.cpp declarations/exception-specifications/defaulted-comparison-noexcept.cpp declarations/exception-specifications/defaulted-comparison-member-throwing.cpp declarations/exception-specifications/constructor-ignores-member-destructors.cpp

A function without a `noexcept` specifier is not non-throwing, with
these exceptions:

- An implicit or defaulted special member function, or a defaulted
  comparison, is non-throwing if each function that it calls for bases
  and members is non-throwing ({question}`Q-0095` for comparisons). A
  default member initializer that the function uses counts too. A
  constructor does not count the destructors that it runs after a
  throw.
- A deallocation function `operator delete` is non-throwing
  ({question}`Q-0094`).
- A destructor follows DTOR-4.
- An inherited constructor has the specifier of the base constructor.
- A `delete` expression is non-throwing if the destructor that it calls
  is non-throwing.

A defaulted function can state its own specifier. `constexpr` does not
make a function non-throwing, and a lambda is non-throwing only with
`noexcept`.
:::

:::{rule} EXS-9
:kind: required
:tests: declarations/exception-specifications/dependent-operand-per-specialization.cpp declarations/exception-specifications/dependent-operand-not-needed.cpp

In a template, the operand of `noexcept` can depend on the template
parameters. Each {term}`specialization` gets its specifier from the
operand with its own arguments ({principle}`TSP-P1`). The operand is
evaluated only when a use needs the specifier, such as a call, an
address, or the `noexcept` operator ({rule}`UN-18`).
:::

:::{rule} EXS-10
:kind: reject
:tests: declarations/exception-specifications/dependent-operand-needed-by-call.cpp declarations/exception-specifications/dependent-operand-needed-by-noexcept.cpp

When a use needs the specifier of a specialization (EXS-9), the operand
with the arguments of the specialization must be valid. The operand
must also obey EXS-7.
:::

## Implementation Requirements

:::{rule} EXS-5
:kind: impl-defined
:tests: declarations/exception-specifications/throw-empty-removed.cpp

Acceptance of `throw()` with an empty list is implementation-defined
({question}`Q-0077`). If an implementation accepts `throw()`, the form
means `noexcept`.
:::

:::{rule} EXS-6
:kind: no-diagnostic
:tests: declarations/exception-specifications/operand-cycle.cpp

The operand of a `noexcept` specifier must not depend on the specifier
itself. An implementation need not diagnose such a dependency
({question}`Q-0079`).
:::

## Examples

```cpp
void log(const char*) noexcept;
void (*safe)(const char*) noexcept = log;   // valid
void (*any)(const char*) = log;             // valid: noexcept dropped
safe = any;                                  // rejected (CONV-10)

struct Buffer {
  Buffer(Buffer&&) noexcept;                 // moves without throwing
  Buffer& operator=(Buffer&&) noexcept(sizeof(int) == 4);
};

struct Base { virtual void close() noexcept; };
struct File : Base { void close() override; };  // rejected: throwing override
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 35
of 41 tests.

- EXS-7: EDG accepts `noexcept(2)` and a pointer operand. Clang and GCC
  reject them ({question}`Q-0078`).
- EXS-4: GCC treats a class-specific `operator delete` without a
  specifier as potentially throwing. Clang and EDG treat it as
  non-throwing ({question}`Q-0094`). EDG treats a defaulted `==` as
  non-throwing even if a member `==` is potentially throwing. Clang and
  GCC treat it as potentially throwing ({question}`Q-0095`).
- EXS-9 and EXS-10: every compiler accepts a specialization whose
  operand is not valid while no use needs the specifier. Every compiler
  rejects a call or a `noexcept` operator that needs the specifier. {rule}`INST-7`
  says when the body of a specialization is instantiated.
- EXS-5 is implementation-defined because the compilers split on
  `throw()`. Clang and GCC accept the form as `noexcept`, and EDG
  rejects it ({question}`Q-0077`).
- EXS-6 needs no diagnostic because GCC and EDG accept an operand that
  depends on the function itself, and Clang rejects it
  ({question}`Q-0079`). The test records both outcomes.
- EXS-4: Clang, GCC, and EDG give `true` for `noexcept` of a placement
  `new` of a class whose members have `noexcept` constructors and
  potentially throwing destructors.
- Each split above holds on the newest compiler of each family in the
  matrix: GCC 16, Clang 22, and EDG 7.0. The older GCC 14 and Clang 18
  give the same results as the newer versions.

Inputs: clang-sema-declcxx, clang-sema-type, clang-diag-sema, edg-decls.

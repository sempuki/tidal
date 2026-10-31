# Exception specifications

A `noexcept` specifier states that a function does not let an exception
leave it. It is part of the function type. This page covers:

- the forms of the specifier,
- its place in function and pointer types,
- overriding,
- the functions that are `noexcept` without the keyword.

What happens when an exception does leave a `noexcept` function belongs
with exception handling ({rule}`THR-9`). DCL-14 and CONV-10 give the rules
for redeclarations and pointer conversions.

Depends on: declarators, standard conversions, virtual functions,
constructors and destructors, constant expressions.

## Principles

:::{principle} EXS-P1
:rules: EXS-1 EXS-7 EXS-2 EXS-8

A `noexcept` specifier is part of the function's type, and the type is
what travels through:

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

A caller that holds a pointer to a base relies on the base's promise.
So an override can make the promise but cannot take it back. A
non-throwing virtual function can be overridden only by a non-throwing
function, and a destructor is no exception.
:::

:::{principle} EXS-P3
:rules: EXS-4

A function the program writes is potentially throwing unless it says
otherwise, because only the program knows its body. A function the
implementation writes gets its specifier from the functions it calls:
a defaulted special member or comparison is non-throwing when every
part it touches is, and a deallocation function is non-throwing by
default.
:::

:::{principle} EXS-P4
:rules: EXS-5 EXS-6

Two points are left open. An implementation can accept the old `throw()`
form as `noexcept`. And an operand that refers to the function it
qualifies is a cycle that an implementation need not detect.
:::

## Syntax

```{eval-rst}
.. productionlist:: exs
   noexcept_specifier: "noexcept" ["(" `constant_expression` ")"]
```

The specifier follows the parameter list and the qualifiers of a function
declarator. `noexcept` alone means `noexcept(true)`. A function with
`noexcept(true)` is a *non-throwing* function.

## Constraints

:::{rule} EXS-1
:kind: required
:tests: declarations/exception-specifications/forms.cpp declarations/exception-specifications/operand-later-member.cpp declarations/exception-specifications/operand-uses-parameters.cpp

The operand of `noexcept` is a constant expression that converts to
`bool`. In a member function, the operand can name members declared
later in the class. It can also use the parameters of the function.
EXS-7 lists what is not valid.
:::

:::{rule} EXS-7
:kind: reject
:tests: declarations/exception-specifications/narrowing-operand.cpp declarations/exception-specifications/non-constant.cpp declarations/exception-specifications/pointer-operand.cpp declarations/exception-specifications/throw-type-removed.cpp

The operand of `noexcept` (EXS-1) must be a constant expression. Its
conversion to `bool` must not narrow ({question}`Q-0078`). A list of
types in the form `throw(T)` is not valid.
:::

:::{rule} EXS-2
:kind: required
:tests: declarations/exception-specifications/alias-noexcept-function.cpp declarations/exception-specifications/call-through-noexcept-pointer.cpp declarations/exception-specifications/decltype-keeps-noexcept.cpp declarations/exception-specifications/in-type.cpp declarations/exception-specifications/member-function-noexcept.cpp declarations/exception-specifications/nested-indirection.cpp declarations/exception-specifications/parameter-noexcept-overloads.cpp declarations/exception-specifications/typedef-noexcept.cpp

The `noexcept` specifier is part of a function type. It is kept in:

- type aliases and `typedef` names,
- pointers to functions, at any depth, and pointers to member functions,
- the type that `decltype` gives.

A call through a pointer to a non-throwing function is non-throwing.
Parameter types that differ in `noexcept` are different types, so
functions with them overload. EXS-8 lists what is not valid.
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
non-throwing. This includes destructors. An override can be non-throwing
when the overridden function is not.
:::

:::{rule} EXS-4
:kind: required
:tests: declarations/exception-specifications/constexpr-not-implied.cpp declarations/exception-specifications/defaulted-explicit-noexcept.cpp declarations/exception-specifications/defaulted-follows-members.cpp declarations/exception-specifications/delete-operator-noexcept.cpp declarations/exception-specifications/implicit-follows-members.cpp declarations/exception-specifications/implicit-special-members.cpp declarations/exception-specifications/inheriting-ctor-noexcept.cpp declarations/exception-specifications/lambda-not-noexcept.cpp declarations/exception-specifications/user-function-not-noexcept.cpp declarations/exception-specifications/delete-class-follows-destructor.cpp declarations/exception-specifications/class-delete-operator-noexcept.cpp declarations/exception-specifications/default-member-init-throwing.cpp declarations/exception-specifications/defaulted-comparison-noexcept.cpp declarations/exception-specifications/defaulted-comparison-member-throwing.cpp

A function without a `noexcept` specifier is not non-throwing, with these
exceptions:

- An implicit or defaulted special member function, or a defaulted
  comparison, is non-throwing if each function that it calls for bases
  and members is non-throwing ({question}`Q-0095` for comparisons). A
  default member initializer that it uses counts too.
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

## Implementation Requirements

:::{rule} EXS-5
:kind: impl-defined
:tests: declarations/exception-specifications/throw-empty-removed.cpp

It is implementation-defined if `throw()` with an empty list is accepted
({question}`Q-0077`). If it is, it means `noexcept`.
:::

:::{rule} EXS-6
:kind: no-diagnostic
:tests: declarations/exception-specifications/operand-cycle.cpp

The operand of a `noexcept` specifier must not depend on the specifier
itself. An implementation need not diagnose it ({question}`Q-0079`).
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 30 of
36 tests.

- EXS-1: EDG accepts `noexcept(2)` and a pointer operand. Clang and GCC
  reject them ({question}`Q-0078`).
- EXS-5: Clang and GCC accept `throw()`. EDG rejects it
  ({question}`Q-0077`).
- EXS-4: GCC treats a class-specific `operator delete` without a
  specifier as potentially throwing. EDG treats a defaulted `==` as
  non-throwing even if a member `==` is potentially throwing
  ({question}`Q-0094`, {question}`Q-0095`).
- EXS-6: Clang rejects an operand that depends on the function itself.
  GCC and EDG accept it ({question}`Q-0079`).

Inputs: clang-sema-declcxx, clang-sema-type, clang-diag-sema, edg-decls.

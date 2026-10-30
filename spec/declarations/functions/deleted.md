# Deleted and defaulted functions

`= delete` as the body of a function makes each use of the function an
error. `= default` asks for the implicit definition of a special member
function *(classes referenced)*.

## Legality Rules

:::{rule} DEL-1
:kind: reject
:tests: declarations/functions/deleted/call-deleted.cpp declarations/functions/deleted/declare-deleted-unused.cpp declarations/functions/deleted/address-of-deleted.cpp declarations/functions/deleted/deleted-overload-selected.cpp declarations/functions/deleted/deleted-overload-not-selected.cpp declarations/functions/deleted/deleted-unevaluated.cpp

A program must not refer to a deleted function. This applies also in an
operand that is not evaluated, such as `decltype` or `sizeof`. A
deleted function takes part in overload resolution *(referenced)*. If
overload resolution selects it, the program is not valid.
:::

:::{rule} DEL-2
:kind: reject
:tests: declarations/functions/deleted/delete-not-first.cpp declarations/functions/deleted/delete-then-define.cpp declarations/functions/deleted/deleted-main.cpp

`= delete` must be on the first declaration of a function. A deleted
function must not have another definition. `main` must not be deleted
({question}`Q-0023`).
:::

:::{rule} DEL-3
:kind: reject
:tests: declarations/functions/deleted/deleted-with-message.cpp

`= delete` must not have a reason in parentheses. A later language
version permits it ({question}`Q-0007`).
:::

:::{rule} DEL-4
:kind: reject
:tests: declarations/functions/deleted/default-non-member.cpp declarations/functions/deleted/defaulted-friend-comparison.cpp

`= default` is permitted only on special member functions and
comparison operators *(classes referenced)*. A defaulted comparison
operator can be a friend that the class declares. Other functions that are
not class members must not use `= default`.
:::

:::{rule} DEL-5
:kind: required
:tests: declarations/functions/deleted/deleted-in-two-units.cpp

A deleted function is `inline`. Several translation units can contain
the same deleted definition.
:::

## Examples

```cpp
void f(int);
void f(double) = delete;
f(1);        // calls f(int)
f(1.5);      // rejected: selects the deleted f(double)

void g() = delete;
using T = decltype(g());   // rejected: refers to g

void h();
void h() = delete;         // rejected: not the first declaration
```

## Evidence

The results are in `tests/RESULTS.md`.

- DEL-2: GCC 14 accepts a deleted `main`. GCC 16, Clang, and EDG reject
  it ({question}`Q-0023`).
- DEL-3: Clang and GCC accept a reason as a newer-standard extension.
  They reject it under `-pedantic-errors`.

Inputs: clang-sema-declcxx, clang-diag-sema, gcc-cp-decl, edg-func-def.

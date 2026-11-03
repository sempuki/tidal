# Deleted and defaulted functions

`= delete` as the body of a function makes each use of the function an
error. `= default` asks for the implicit definition of a special member
function ({rule}`CTOR-2`, {rule}`CPY-4`, {rule}`DTOR-2`).

## Principles

:::{principle} DEL-P1
:rules: DEL-1 DEL-2 DEL-3 DEL-5

A deleted function is declared so that it can be chosen and then
refused. The deleted function takes part in overload resolution like any
function, and a use of the function is an error, even in an operand that
is never evaluated. The deletion is on the first declaration, so no
earlier code could use the function, and the deletion is the function's
only definition.
:::

:::{principle} DEL-P2
:rules: DEL-4

`= default` asks the implementation for its own version of a function.
Only the special members and the comparison operators can be defaulted,
because the implementation knows how to write them.
:::

## Constraints

:::{rule} DEL-1
:kind: reject
:tests: declarations/functions/deleted/call-deleted.cpp declarations/functions/deleted/declare-deleted-unused.cpp declarations/functions/deleted/address-of-deleted.cpp declarations/functions/deleted/deleted-overload-selected.cpp declarations/functions/deleted/deleted-overload-not-selected.cpp declarations/functions/deleted/deleted-unevaluated.cpp foundations/overload-resolution/chosen-deleted.cpp foundations/overload-resolution/address-of-deleted-chosen.cpp foundations/conversion-sequences/chosen-deleted-constructor.cpp foundations/conversion-sequences/chosen-deleted-conversion-function.cpp

A program must not refer to a deleted function, also in an operand that
is not evaluated, such as `decltype` or `sizeof`. A deleted function
takes part in overload resolution ({rule}`OVL-25`).
If overload resolution selects a deleted function, the program is not
valid. This rule is a specific case of {rule}`OVL-37`.
:::

:::{rule} DEL-2
:kind: reject
:tests: declarations/functions/deleted/delete-not-first.cpp declarations/functions/deleted/delete-then-define.cpp declarations/functions/deleted/deleted-main.cpp declarations/inline/deleted-function-inline.cpp

`= delete` must be on the first declaration of a function. A later
declaration without a body is valid, and the function stays deleted. A
deleted function must not have another definition, and `main` must not
be deleted ({question}`Q-0023`).
:::

:::{rule} DEL-3
:kind: reject
:tests: declarations/functions/deleted/deleted-with-message.cpp

`= delete` must not have a reason in parentheses. A later language
version permits a reason ({question}`Q-0007`).
:::

:::{rule} DEL-4
:kind: reject
:tests: declarations/functions/deleted/default-non-member.cpp declarations/functions/deleted/defaulted-friend-comparison.cpp

`= default` is permitted only on special member functions and comparison
operators ({rule}`DCMP-3`). A defaulted comparison operator can be a
friend that the class declares. Other functions that are not class
members must not use `= default`.
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
  a deleted `main` ({question}`Q-0023`).
- DEL-3: Clang and GCC accept a reason as a newer-standard extension,
  but reject a reason under `-pedantic-errors`.

Inputs: clang-sema-declcxx, clang-diag-sema, edg-func-def.

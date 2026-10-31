# Q-0138: Does the kind of a function reference break a tie between two conversion functions?

**Status:** provisional
**Affects:** OVL-29, `tests/foundations/overload-resolution/tie-function-reference-kind.cpp`

## Context

A class has two conversion functions to the same function type `F`. One
returns `F&` and the other returns `F&&`. A reference to `F` binds
directly to the result of either one, and both sequences are the
identity.

| Compiler | `F& r = Source{};` | `F&& r = Source{};` |
|---|---|---|
| GCC 14, GCC 16 | the `F&` conversion | the `F&&` conversion |
| EDG 7.0 | the `F&` conversion | the `F&&` conversion |
| Clang 18, Clang 22 | the `F&` conversion | ambiguous |

## Evidence

`tests/foundations/overload-resolution/tie-function-reference-kind.cpp`.
Item RK-4 of the overload resolution inventory names this tie-breaker,
and gap G-6 says that compilers can ignore it.

## Options

1. The conversion that returns the target's kind of reference wins
   (GCC, EDG).
2. No tie-breaker applies, and the rvalue reference binding is
   ambiguous (Clang).

## Provisional answer

Option 1. Two of three implementations agree, and the inventory
describes the tie-breaker.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Q-0374: Is a call of a member function on a member that is not yet initialized undefined, when the function reads nothing?

**Status:** provisional
**Affects:** CTOR-12, OBJ-2, `tests/classes/constructors/base-init-calls-member-of-member-constant.cpp`

## Context

`struct D : A { Helper h; constexpr D() : A(h.one()) {} };` calls
`Helper::one` on `h` before `h` is initialized. The function returns a
constant and reads no member of `h`.

| Compiler | `constexpr D d;` |
|---|---|
| Clang 18, 22, host | rejects ("member call on object outside its lifetime") |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

No compiler rejects the call when the function belongs to `D` itself
and reads nothing, from the `mem_init` of a base.

## Evidence

The test in Affects, and probes of calls through `this`, through a
pointer to the object, on a member, and of a static member function.

## Options

1. The call is undefined, because the lifetime of `h` has not begun.
   Constant evaluation rejects the call (Clang).
2. The call is defined while the function reads or writes nothing of
   `h` (GCC, EDG).

## Provisional answer

The call is undefined (option 1). OBJ-1 starts the lifetime of a class
object when its initialization completes, and a member function works
on its object. Clang names this reason.

Kept open on 2026-11-03 under decision 0054, because no review has read the provisional answer. GCC 16 and EDG 7.0 also differ from the answer.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

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

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer, and CTOR-12 changed in the commit that read the review (decision 0055). The gap review of the rules that phase 7 rewrote supports the answer with no doubt.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) agrees with option 1. The member's lifetime has not begun, because its initialization, even one that does nothing, has not happened yet. The reference lists a call of a non-static member function through a glvalue to such an object as undefined, and the member is not an object under construction. Undefined behavior ends constant evaluation, as Clang finds.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

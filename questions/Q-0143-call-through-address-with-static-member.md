# Q-0143: Can a call through the address of an overload set choose a static member?

**Status:** provisional
**Affects:** OVL-21, `tests/foundations/overload-resolution/call-through-address-static-member.cpp`

## Context

A class has a non-static `reset()` and a static `reset(int)`. A member
function calls `(&Item::reset)(1)`.

| Compiler | Result |
|---|---|
| Clang 22 | accepts and calls the static member |
| Clang 18 | rejects: the overloaded function cannot be resolved |
| GCC 14, GCC 16 | rejects: no contextual type information |
| EDG 7.0 | stops with an internal error |

For free functions, `(&pick)(1)` works on every compiler
(`call-through-address.cpp`).

## Evidence

`tests/foundations/overload-resolution/call-through-address-static-member.cpp`.
Item AO-1 of the overload resolution inventory, and gap G-2, which says
that support for this form may be partial.

## Options

1. Resolution runs on the set with no implied object argument, and the
   static member wins (Clang 22).
2. The form is not valid for a set of members (Clang 18, GCC).

## Provisional answer

Yes; Clang 22 agrees, Clang 18 and GCC differ, EDG has an internal error (option 1). The inventory describes the call this way, the newest Clang
implements it, and EDG's internal error is a defect.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer, and the rule records one differing family. GCC 16 rejects the call, and EDG 7.0 stops with an internal error.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. Overload resolution runs on the set with no implied object argument, and the program is not valid only when the chosen function is an implicit object member function. Clang 22 follows this reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-10-31

The source-reader gap review of overload resolution (`reports/review/2026-10-31-gap-review-overload-resolution.md`) reads the address of the plain qualified name as allowed for a set that holds both a static member and members with an implicit object parameter, and the call then chooses the static member. With a parenthesized qualified name or an unqualified name, the address of such members cannot be taken, so the expression is an error. The reading supports the provisional answer for the plain qualified name, and the review points to this question for the splits.

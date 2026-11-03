# Q-0318: Must constant evaluation reject a member function call through a null pointer?

**Status:** provisional
**Affects:** PFX-21, CE-9, `tests/expressions/postfix/member-call-null-pointer-constant.cpp`

## Context

A `constexpr` function calls `p->one()` where `p` is a null pointer to
a class, and `one` reads no member. A `static_assert` evaluates the
function.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects: member call on dereferenced null pointer |
| GCC 16, 14 | rejects: dereferencing a null pointer |
| EDG 7.0 | accepts |

At run time, UBSan on Clang and GCC reports "member call on null
pointer" for the same call.

## Evidence

The test above and
`tests/expressions/postfix/member-call-null-pointer-sanitizer.cpp`.

## Options

1. The call is undefined (PFX-21), so constant evaluation rejects it
   (CE-9). EDG misses the case.
2. The call is valid when the function does not touch the object.

## Provisional answer

Yes, undefined (PFX-21); EDG accepts (option 1). Clang and GCC reject the call in constant evaluation, and both
sanitizers report it at run time. The test expects a rejection and
records EDG's result with an override.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the call, and EDG 7.0 accepts it, so two families disagree. A probe where the member function reads a member is rejected by every compiler. CE-P3 supports the provisional answer, because constant evaluation rejects any operation that is undefined (CE-9, PFX-21). The front end's interpreter rejects the call with PFX-21.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

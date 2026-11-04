# Q-0187: In a call with no object, does a member's object parameter take part in overload resolution?

**Status:** provisional
**Affects:** OVL-23, OVL-25, OVL-3, `tests/overloading/overload-resolution/no-object-ref-qualified-member.cpp`

## Context

A call by name in a static member function can find members with an
implicit object parameter. The call has no object (OVL-3). A class has
`static void put(long)` and `void put(int) &&`, and a static member
function calls `put(1)`.

| Compiler | Object of the call | Result |
|---|---|---|
| Clang 18, Clang 22 | none; the object parameter takes no part | `put(int) &&` wins, and the call is rejected for lack of an object |
| GCC 14, GCC 16 | an lvalue of the class | `put(int) &&` is not viable, and `put(long)` wins |
| EDG 7.0 | none; the object parameter takes no part | `put(int) &&` wins, and the call is rejected for lack of an object |

For `void k(int) &` and `void k(int) &&` with no object, EDG reports an
ambiguous call. Clang and GCC report a call without an object. GCC names
`k(int) &`.

## Evidence

`tests/overloading/overload-resolution/no-object-ref-qualified-member.cpp`.
The same split appears for `S::put(1)` at namespace scope: GCC accepts
the call, and Clang and EDG reject it.

## Options

1. The object parameter takes no part, whatever its cv-qualifiers and
   ref-qualifier (Clang, EDG).
2. The call has an lvalue object of the class, so the qualifiers can
   make a member not viable (GCC).

## Provisional answer

No; it takes no part; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree. The answer also matches
ICS-14, where the object slot of a static member takes no part.

The question stays open under decision 0051 (2026-11-03). GCC 16 gives the call an lvalue object, and Clang 22 and EDG 7.0 do not, so two families disagree. A probe of 2026-11-03 confirms the split for `S::put(1)` at namespace scope. The provisional answer follows OVL-P1, which checks for an object in step 4, after the best function is chosen. A made-up object in step 2 would remove a member before that check.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. A call with no usable object gets a made-up object of the class. The reference states in a note that this object never makes a function viable or not viable, and that the call is not valid when a non-static member wins. The normative text gives the made-up object no value category, so most of the support comes from the note.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

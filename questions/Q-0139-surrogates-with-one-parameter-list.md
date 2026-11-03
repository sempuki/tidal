# Q-0139: Is a call ambiguous when two surrogates differ only in the qualifiers of their conversion functions?

**Status:** settled
**Affects:** OVL-6, OVL-36, `tests/foundations/overload-resolution/surrogate-same-parameters.cpp`

## Context

A class has `operator Fn()` and `operator Fn() const`, where `Fn` is a
pointer to function. Each gives a surrogate candidate with the same
parameter list. A call on an object of the class that is not `const`
must choose one surrogate.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | ambiguous |
| EDG 7.0 | ambiguous |
| GCC 14, GCC 16 | accepts |

The object's sequences use different conversion functions, so they tie
(ICS-25). With conversions to two different function pointer
types, every compiler reports the tie (object-call grid, columns
"surrogate / const surrogate").

## Evidence

`tests/foundations/overload-resolution/surrogate-same-parameters.cpp`.
Item AF-7 of the overload resolution inventory.

## Options

1. The call is ambiguous (Clang, EDG).
2. The surrogate whose conversion function binds the object better
   wins (GCC).

## Answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the answer follows
the sequence rules of ICS-25 without an exception for surrogates.

The question stays open under decision 0051 (2026-11-03). GCC 16 accepts the call, and Clang 22 and EDG 7.0 find it ambiguous, so two families disagree. The provisional answer follows ICS-25, step 3 of ICS-P5. Two user-defined sequences that use different conversion functions tie. The same tie decides two surrogates to function pointer types with different return types, and there every compiler agrees (object-call grid, run again on 2026-11-03).

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) supports this answer with no doubt, and OVL-P3 gives the same answer: the two surrogates tie on every argument under ICS-P5, no tie-breaker of step 2 applies, and neither function is better. GCC 16 still differs. Tidal records GCC's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. The reference allows two surrogates whose declarations are the same, so that overload resolution cannot separate them, and makes such a call ambiguous unless a third candidate is better. The reference does not link the object's conversion for a surrogate to the conversion function the surrogate comes from. The two surrogates tie on every argument in either reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

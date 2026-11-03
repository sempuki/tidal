# Q-0144: Does a const object get a surrogate from a conversion function that is not const?

**Status:** settled
**Affects:** OVL-6, `tests/foundations/overload-resolution/surrogate-qualifiers.cpp`

## Context

A class has `operator Fn()`, which is not `const`, and a `const` call
operator that takes `long`. A `const` object of the class is called with
an `int`.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | the call operator |
| EDG 7.0 | the call operator |
| GCC 14, GCC 16 | ambiguous between the call operator and the surrogate |

With no call operator, GCC chooses the surrogate and then rejects the
`const` object for the conversion function.

## Evidence

`tests/foundations/overload-resolution/surrogate-qualifiers.cpp`, and
the rows "const S lvalue" of the object-call grid. Items CC-8 and gap
G-10 of the overload resolution inventory.

## Options

1. A conversion function gives a surrogate only if its cv-qualifiers
   include those of the object (Clang, EDG).
2. Every conversion function gives a surrogate (GCC).

## Answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the inventory states
the condition.

The question stays open under decision 0051 (2026-11-03). GCC 16 keeps the surrogate, and Clang 22 and EDG 7.0 do not, so two families disagree. The provisional answer follows ICS-13 and OVL-23. The object of a surrogate converts by the conversion function, whose implicit object parameter is an lvalue reference to the class without `const`. A `const` object cannot bind that reference (ICS-12), so the surrogate has no sequence for the object. GCC itself rejects the `const` object for the conversion function when the surrogate is the only candidate.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) supports this answer with no doubt, and OVL-P2 gives the same answer: the object meets the object parameter of the conversion function (ICS-13), and a `const` object has no sequence to a parameter without `const`. GCC 16 still differs. Tidal records GCC's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. A conversion function gives a surrogate only when its cv-qualifiers are the same as the object's or add to them. A conversion function that is not `const` gives no surrogate for a `const` object.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

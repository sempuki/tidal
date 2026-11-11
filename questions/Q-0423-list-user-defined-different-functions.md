# Q-0423: Which element's conversion does a list sequence hold when the elements do not all use one conversion function?

**Status:** provisional
**Affects:** ICS-16, ICS-17, ICS-25, `tests/overloading/conversion-sequences/list-user-defined-first-element-int.cpp`, `tests/overloading/conversion-sequences/list-user-defined-first-element-long.cpp`, `tests/overloading/conversion-sequences/array-list-user-defined-first-element.cpp`
**Source:** Q-0419, which held this case until decision 0061 item QR2 moved it here, with the probe grid `reports/clean-room/roadmap-9/writer-probes/i3-13-list-worst-element.json` that a clean-room writer ran for I3-13.
**Waits for:** a review (silent)

## Context

A braced list initializes a parameter of type `std::initializer_list<E>`
or a reference to an array of `E`. Its sequence is the worst element
conversion (ICS-16, ICS-17). When that kind is a user-defined sequence
and the elements do not all use the same conversion function, the list
holds more than one user-defined sequence, or one user-defined sequence
beside standard ones. ICS-25 compares two user-defined sequences only
when they use the same function.

Class `P` converts to `int` and class `Q` converts to `long`. Two
overloads take `std::initializer_list<int>` and
`std::initializer_list<long>`, or `const int (&)[2]` and
`const long (&)[2]`.

| Call | Clang 18 and 22 | EDG 7.0 | GCC 14 and 16 |
|---|---|---|---|
| `{P{}, Q{}}` | the `int` form | the `int` form | ambiguous |
| `{Q{}, P{}}` | the `long` form | the `long` form | ambiguous |
| `{P{}, 0}` | the `int` form | the `int` form | ambiguous |
| `{Q{}, 0}`, arrays | the `long` form | the `long` form | ambiguous |

Clang and EDG take the conversion of the first element that has a
user-defined one. GCC rejects every call.

## Evidence

The three tests in **Affects**. Each expects the choice of Clang and
EDG and records GCC's rejection as an override.

## Options

1. The list sequence holds the user-defined sequence of the first
   element that has one (Clang, EDG).
2. Two list sequences whose worst kind is user-defined are
   indistinguishable, so the call is ambiguous (GCC).
3. The list sequence holds the worst of the user-defined element
   sequences by some other order, such as the last element.

## Provisional answer

The list sequence holds the user-defined sequence of the first element
that has one (option 1). Two of three families agree, and the answer
gives ICS-25 one conversion function to compare.

Kept open on 2026-11-10 under decision 0054, because the review finds the reference silent on this case.

## Opinions

### Gap review, 2026-11-05 (phase 12 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 12 wrote on expressions, overloading, and templates (`reports/review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md`) read this case as part of Q-0419 (RV5-B-8). When the elements use different functions, the reference names the worst conversion and never says which of two user-defined sequences through different functions is worse, so the choice of the first element is Tidal's own.

# Q-0419: When every element of a braced list converts by the same conversion function, do two list sequences compare by their third parts?

**Status:** settled
**Affects:** ICS-16, ICS-25, `tests/overloading/conversion-sequences/list-user-defined-same-function.cpp`
**Source:** I3 finding I3-13 (`reports/clean-room/I3/report.md`), from the probe grid `reports/clean-room/roadmap-9/writer-probes/i3-13-list-worst-element.json` that a clean-room writer ran for it.

## Context

A braced list initializes a parameter of type `std::initializer_list<E>`
or a reference to an array of `E`. Its sequence is the worst element
conversion (ICS-16, ICS-17). When that kind is a user-defined sequence,
ICS-25 compares two such list sequences by their conversion functions
and third parts. The text did not say which element's conversion the
list sequence holds.

Class `P` converts to `int` and class `Q` converts to `long`. Two
overloads take `std::initializer_list<int>` and
`std::initializer_list<long>`, or `const int (&)[2]` and
`const long (&)[2]`.

| Call | Clang 18 and 22 | EDG 7.0 | GCC 14 and 16 |
|---|---|---|---|
| `{P{}, P{}}` | the `int` form | the `int` form | ambiguous |
| `{P{}, Q{}}` | the `int` form | the `int` form | ambiguous |
| `{Q{}, P{}}` | the `long` form | the `long` form | ambiguous |
| `{P{}, 0}` | the `int` form | the `int` form | ambiguous |

Clang and EDG take the conversion of the first element that has a
user-defined one. GCC rejects every call in the grid, also the calls
where every element uses the same conversion function.

This question holds the first row, where every element uses the same
conversion function. The other rows, where the elements do not all use
one conversion function, are Q-0423.

## Evidence

The test in **Affects** expects the choice of Clang and EDG and
records GCC's rejection as an override.

## Options

1. The list sequence holds the user-defined sequence of the first
   element that has one (Clang, EDG).
2. Two list sequences whose worst kind is user-defined are
   indistinguishable, so the call is ambiguous (GCC).
3. The list sequence holds the worst of the user-defined element
   sequences by some other order, such as the last element.

## Answer

Yes. Both list sequences use the one conversion function, so ICS-25
compares their third parts, as step 3 of ICS-P5 says. Option 1 gives
this answer for the first row, and option 3 gives it too. Clang and EDG
choose the `int` form, and GCC calls the call ambiguous.

Settled on 2026-11-10 under decision 0054. The gap review of 2026-11-05 supports the answer for the case where every element uses the same conversion function, and ICS-P5 gives the same answer. GCC 14 and 16 differ, and the test keeps their override. The case where the elements use different conversion functions moved to Q-0423 (decision 0061, item QR2).

Kept open on 2026-11-05 under decision 0054, because the review reads the mixed lists as a choice the reference does not make (item RV5).

Read again on 2026-11-12 (decision 0066, item RV12). The gap review of the phase 18 text reads ICS-16's second paragraph as a choice that the reference leaves open, and names this question and Q-0423 as its holders (RV12-13). The comparison of two sequences through the same conversion function supports the settled answer, which stands.

## Opinions

### Gap review, 2026-11-05 (phase 12 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 12 wrote on expressions, overloading, and templates (`reports/review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV5-B-8). When every element uses the same conversion function, the reference decides by the third parts and supports the answer, and GCC's ambiguity reads against it. When the elements use different functions, the reference names the worst conversion and never says which of two user-defined sequences through different functions is worse, so the choice of the first element is Tidal's own.

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) reads ICS-16 (RV12-13). The first paragraph agrees. When the worst element conversion is user-defined, the reference does not say which user-defined sequence the list holds. It compares two user-defined sequences only when they use the same conversion function, and then by the standard conversion after the function. That comparison gives this question's answer for elements that use one conversion function. The choice for elements that use different functions is Tidal's own, and Q-0423 holds it.

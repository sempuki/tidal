# Q-0454: Is a braced integer constant a valid template argument for a floating-point parameter?

**Status:** provisional
**Affects:** TARG-13, TARG-16, `tests/templates/arguments/braced-integer-for-floating.cpp`
**Source:** RV11-R-7 (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`), with the round 15 writer grid `reports/clean-room/roadmap-15/writer-probes/rv11-rows.json`.
**Waits for:** a principle (TARG-P3)

## Context

A class template has a constant parameter of type `double`. The
argument is the `int` constant `1`, written in braces:

```cpp
template <double D> struct Rate { static constexpr double value = D; };
static_assert(Rate<{1}>::value == 1.0);
```

| Form | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `Rate<{1}>` | accepts | rejects, "conversion from 'int' to 'double' in a converted constant expression" | rejects, "is incompatible with template parameter of type" |
| `Rate<1>` | rejects | rejects | rejects |
| `{3}` and `{}` for an `int` parameter | accepts | accepts | accepts |
| `{3.5}` for an `int` parameter, `{300}` for a `char` parameter | rejects | rejects | rejects |

## Evidence

The test in **Affects** and the grid in **Source**. TARG-13 forbids a
conversion between an integer type and a floating-point type in a
constant argument, and every compiler rejects `Rate<1>`
(`constant-integer-to-floating.cpp`).

## Options

1. The braced value converts as the same value without braces, so the
   argument is not valid (GCC, EDG).
2. The braces make the argument a list initialization of the
   parameter, which allows the exact conversion of `1` to `double`
   (Clang).

## Provisional answer

No; the program is rejected (option 1). TARG-16 says that the value in
a braced list for a scalar parameter converts as TARG-13 says, and
TARG-13 forbids the integer to floating-point conversion. GCC 16 and
EDG 7.0 agree, and Clang 22 alone differs. Every compiler also rejects
the narrowing forms in braces, so the braces add no conversion in those
forms.

Opened on 2026-11-11 (decision 0066, item F8). No review has read the
question. The read of the RV10 repair diff names the scalar case as a
gap in Tidal's text (RV11-R-7) and gives a template argument no braced
form at all (RV10-6), which `pending.md` row 4 holds as the item
"TARG-16".

Kept open on 2026-11-12 under decision 0054 (decision 0066, item RV12). The gap review of the phase 18 text gives the first reading of the question and supports the answer with no doubt (RV12-10). The rejection holds whether or not a template argument can be braced. Clang 22 alone differs. No principle gives the answer. TARG-P3 says that the conversion to the parameter's type keeps the value, and `1` keeps its value as `1.0`, so option 2 also fits TARG-P3's model. TARG-P3 also names TARG-13 in `:exceptions:`. So TARG-P3 fails step 3 of the principle test. The **Waits for:** line names a principle (TARG-P3), was a first review. The review reads the braced form itself against the C++23 text (RV10-6), and {question}`Q-0456` now holds that premise.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) gives the first reading of the question: supports with no doubt (option 1, RV12-10). The reference's template argument has no braced form, so `Rate<{1}>` is not valid on that ground alone, as RV10-6 found. Under Tidal's reading the braced value converts as the same value without braces would. The conversion of a constant template argument does not allow an integer to become a floating-point value, so that reading rejects the program too. The rejection holds under either reading, and Clang's acceptance does not conform.

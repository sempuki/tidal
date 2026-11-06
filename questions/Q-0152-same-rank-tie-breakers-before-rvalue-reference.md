# Q-0152: Do the same-rank tie-breakers come before the rvalue-reference preference, and does that preference come before the qualification step?

**Status:** provisional
**Affects:** ICS-28, ICS-31, ICS-32 to ICS-36, ICS-P5, `tests/overloading/conversion-sequences/tie-breaker-before-rvalue-reference.cpp`, `tests/overloading/conversion-sequences/fixed-enumeration-before-rvalue-reference.cpp`, `tests/overloading/conversion-sequences/floating-before-rvalue-reference.cpp`, `tests/overloading/conversion-sequences/rvalue-reference-before-qualification.cpp`
**Waits for:** a review (doubt); newest families that agree

## Context

Each case has two functions. One takes a `const` lvalue reference, and
the other takes an rvalue reference. The argument is a prvalue, so both
references bind to it or to a temporary, and both sequences have the
same rank. ICS-28 prefers the rvalue reference. A same-rank tie-breaker
or ICS-31 prefers the other function.

| Argument | Functions | Other rule | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|---|---|
| `low`, of `enum Level : short` | `const short&`, `int&&` | ICS-34 | `short` | `int&&` | `int&&` |
| `&n`, an `int*` | `void* const&`, `bool&&` | ICS-32 | `void*` | `void*` | `bool&&` |
| `Derived{}` | `const Middle&`, `Base&&` | ICS-36 | `Middle` | `Middle` | `Base&&` |
| `&d`, a `Derived*` | `Base* const&`, `void*&&` | ICS-33 | `Base*` | `Base*` | `void*&&` |
| `&d`, a `Derived*` | `Middle* const&`, `Base*&&` | ICS-36 | `Middle*` | `Middle*` | `Base*&&` |
| `&Base::x` | `int Middle::* const&`, `int Derived::*&&` | ICS-36 | `int Middle::*` | `int Middle::*` | `int Derived::*&&` |
| `1.0f64` | `const double&`, `long double&&` | ICS-35 | no `f64` suffix | `double` | `long double&&` |
| `&n`, an `int*` | `const int* const&`, `const volatile int*&&` | ICS-31 | `const volatile int*&&` | `const int*` | `const volatile int*&&` |

## Evidence

The four tests in **Affects**, and the order grid in `tests/overloading/conversion-sequences/grids/tables.md`
(`tests/overloading/conversion-sequences/grids/order.json`). A source
reader's gap review (items O1 and O2 of
`reports/review/2026-10-31-gap-review-conversion-sequences.md`) reports
that the same-rank tie-breakers belong to the rank comparison, before
the reference rules, and that the rvalue-reference preference comes
before the qualification step.

## Options

1. The same-rank tie-breakers come before ICS-28, and ICS-28 comes
   before ICS-31 (Clang on every row, GCC on the pointer, class, and
   floating rows, EDG on the last row).
2. ICS-28 comes before the same-rank tie-breakers and before ICS-31
   (EDG on the tie-breaker rows).
3. GCC's mix: the tie-breakers come first except ICS-34, and ICS-31
   comes before ICS-28.

## Provisional answer

Yes to both; Clang agrees, GCC and EDG differ in part (option 1). Two of three implementations agree on the pointer, class, and
member pointer rows, and on the last row. Clang follows one order on
every row, and the gap review reports the same order. The tie-breakers
then belong to the rank step, which is where ICS-27 already puts them.

The question stays open under decision 0051 (2026-11-03). The order grid, run again on 2026-11-03, gives the table above. Clang 22 follows option 1 on every row. GCC 16 applies ICS-28 before ICS-34, and ICS-31 before ICS-28. EDG 7.0 applies ICS-28 before each same-rank tie-breaker. So two families disagree on each part. For the first part, the provisional answer follows ICS-27 and ICS-P5. The same-rank tie-breakers refine the rank, so they belong to the rank step, and the reference steps 4d to 4g come after the conversions are compared. For the second part, every compiler applies ICS-28 before ICS-30 (`rvalue-reference-before-cv.cpp`), and the answer keeps the two cv-qualifier steps, ICS-31 and ICS-30, after ICS-28. Items O1 and O2 of the gap review give the same order.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Noted on 2026-11-05 under decision 0058, item PK2. A test whose notes name the question has two newest families that differ from the answer, so the question also waits for newest families that agree (`pending.md` row 1). The check of `tools/question_index.py` found the case.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. The same-rank tie-breakers belong to the rank step, which comes before the reference steps, and the rvalue-reference preference comes before the step that compares qualification conversions. Each row of the table follows option 1. The review has a small doubt about whether the reference steps cover bindings that make a temporary. The reference speaks of sequences that include a reference binding, and compilers apply the steps to these bindings.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

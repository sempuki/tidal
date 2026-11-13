# Q-0457: In copy initialization, is a deduction guide viable when its parameter needs a user-defined conversion of the argument?

**Status:** provisional
**Affects:** CTAD-17, `tests/templates/class-template-deduction/copy-init-guide-user-conversion.cpp`
**Source:** RV8-B-5 (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`), and RV12-18 (`reports/review/2026-11-12-gap-review-phase-18.md`, Part B).
**Waits for:** the maintainer (pending.md row 4)

## Context

A class template has a constructor from `const Conv&`, and a user
guide takes an `int`. The class `Conv` converts to `int`. A variable
is copy-initialized from a `Conv` with `=` and no braces:

```cpp
struct Conv { constexpr operator int() const { return 1; } };
template <class T> struct W { constexpr W(const Conv&) {} };
W(int) -> W<long>;
W w = Conv{};
```

The user guide is viable only through the conversion function of
`Conv`. CTAD-17's second item says that the limit of OVL-14 and ICS-10
to a standard sequence does not apply to the guides. The limit applies
to the constructors of `W<long>` that the initialization then chooses,
and `W(const Conv&)` takes the `Conv` directly.

| Form | Clang 18 | Clang 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|---|
| `W w = Conv{};` with the guide `W(int)` | rejects, "no viable constructor or deduction guide" | accepts, `W<long>` | accepts, `W<long>` | accepts, `W<long>` |

## Evidence

The test in **Affects** expects acceptance and records Clang 18's
rejection as an override. The override's note names
{question}`Q-0020`, the question of how a test records a feature that a
compiler does not implement.

## Options

1. The guide is viable, and the limit to a standard sequence applies
   only to the constructors that the initialization chooses after
   deduction (Clang 22, GCC, EDG).
2. The guides act as constructors in copy initialization, so the limit
   on a user-defined sequence for the first parameter reaches the
   guide, and no guide is viable (Clang 18, the gap reviews of the
   phase 15 and phase 18 text).

## Provisional answer

The guide is viable (option 1). Every newest family accepts the
program, and CTAD-17 keeps their answer. The `pending.md` row 4 item
"CTAD-17" holds the split.

Opened on 2026-11-12 (decision 0066, item RV12). Part B of the gap
review of the phase 18 text asks for a question on RV8-B-5 (RV12-18),
because the review reads Clang 18's answer as the conforming one, and
Q-0020 records Clang 18's result as a missing feature. No principle
gives the answer. CTAD-P3 says that the guides compete as the
constructors of a class would, and it does not say whether the limit on
the first parameter reaches them, so both options fit CTAD-P3's model.

Rule text on 2026-11-12 (decision 0066, item RV12): CTAD-17's item on the limit to a standard sequence, the CTAD-17 Note, the Divergences list, the split test table, and the note of `copy-init-guide-user-conversion.cpp` name this question in place of Q-0020 (RV12-18). The `pending.md` row 4 item is now "CTAD-17, Q-0457". The answer and the **Waits for:** line do not change.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads CTAD-17's second item against the C++23 text (option 2, RV8-B-5). The reference forms the guides as the constructors of a notional class and runs the usual rules of initialization on them. In copy initialization from a value of another type, those rules bar a user-defined sequence for the first parameter of a candidate constructor, and the reference states no exception for the guides. Read as written, the guide that needs the conversion function is not viable, which is Clang 18's answer. The review notes that the choice of a guide runs no conversion, which may explain the newer compilers, and that the reference does not make that distinction.

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) answers yes to a question for RV8-B-5 in Part B. Clang 18 differs from the newer compilers, and the earlier review reads the C++23 text as Clang 18 does. Q-0020 holds Clang 18's override as a missing feature, and that record does not fit a compiler whose answer the review reads as conforming.

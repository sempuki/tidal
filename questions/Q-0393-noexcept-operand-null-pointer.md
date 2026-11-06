# Q-0393: Can the operand of a `noexcept` specifier be a null pointer constant of the null pointer type?

**Status:** provisional
**Affects:** EXS-7, EXS-P1, `tests/declarations/exception-specifications/null-pointer-operand.cpp`, `tests/declarations/exception-specifications/null-pointer-constant-operand.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md, the EXS-P1 and EXS-7 section
**Waits for:** the maintainer (pending.md row 4); a review (against)

## Context

EXS-7 asks that the operand of a `noexcept` specifier convert to
`bool` without a narrowing conversion. The null pointer literal
converts to `false`, and that conversion is not a narrowing
conversion. The gap review of the text phase 8 wrote reads the C++23
text as allowing only a short list of conversions to `bool` in this
place, and a conversion from the null pointer type is not on the list.

```cpp
void f() noexcept(nullptr);
constexpr decltype(nullptr) none = nullptr;
void g() noexcept(none);
```

| Program | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `noexcept(nullptr)` | rejects | accepts | accepts |
| `noexcept(none)` | rejects | accepts | accepts |

The operand of `explicit(...)` splits the same way.

## Evidence

The two tests in **Affects**. Each expects acceptance and records
Clang's rejection as an override.

## Options

1. The operand is valid, because its conversion to `bool` does not
   narrow (GCC, EDG).
2. The operand is not valid, because the conversion from the null
   pointer type is not one that this place allows (Clang, and the
   review's reading).

## Provisional answer

The operand is valid and gives a potentially throwing function
(option 1). GCC 16 and EDG 7.0 agree, and EXS-P1 asks only that the
conversion not narrow. Tidal keeps the matrix's answer against the
review (`documentation/decisions/pending.md`, "Matrix against
review").

Kept open on 2026-11-04 under decision 0054, because the gap review reads against the answer, and Clang 22 differs from it.

Kept open on 2026-11-05 under decision 0054, because the review reads against the answer, and the question waits on the maintainer (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) supports option 2. In its reading, the operand must become a `bool` constant through a fixed list of conversions: user conversions, the lvalue, array, and function conversions, qualification and function pointer conversions, integral promotions, and integral conversions that do not narrow. A conversion from the null pointer type to `bool` is not on the list, so the operand is not valid, although the conversion is not a narrowing conversion.

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: reads against. The operand must be a constant converted to bool using only a fixed list of conversions. The conversion from the null pointer type to bool is not on that list, so option 2 holds.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

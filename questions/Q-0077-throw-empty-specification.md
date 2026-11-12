# Q-0077: Is throw() a valid exception specification?

**Status:** provisional
**Affects:** EXS-5, `tests/declarations/exception-specifications/throw-empty-removed.cpp`
**Waits for:** the maintainer (pending.md row 4)

## Context

`void f() throw();` uses the old form of an exception specification. Clang and GCC accept the form under `-pedantic-errors` and treat it as `noexcept`. EDG 7.0 rejects it. All three reject `throw(int)`.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

## Options

1. It is implementation-defined if `throw()` is accepted. If `throw()` is accepted, `throw()` means `noexcept`.
2. `throw()` is not valid.
3. `throw()` is valid and means `noexcept`.

## Provisional answer

Implementation-defined; where `throw()` is accepted, it means `noexcept` (option 1). Two implementations accept `throw()` and one rejects it.

Kept on 2026-11-03. The review reads option 2, which only EDG 7.0 follows. Clang 22 and GCC 16 accept `throw()` under `-pedantic-errors`, and EXS-P4 gives the answer: an implementation can accept the old form as `noexcept`. Tidal keeps option 1 until the maintainer decides. The question is in row 4 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

Kept open on 2026-11-04 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

Kept in `pending.md` row 4 on 2026-11-11 (decision 0065, item RV10). The gap review of the phase 17 text reads against the answer again (RV10-2). `exception_spec` follows EXS-5 and derives `throw()`. Clang 22 and GCC 16 accept `throw()`, and EDG 7.0 rejects it, as before.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2. The grammar of an exception specification has only the `noexcept` forms, because the empty `throw()` form was removed in C++20. So `throw()` is a syntax error and needs a diagnostic, which EDG gives. Clang and GCC accept it silently in their strictest modes, which is a defect. A warning would be enough to conform.

### Gap review, 2026-11-11 (phase 17 text)

The source-reader gap review of the phase 17 text (`reports/review/2026-11-11-gap-review-phase-17.md`) reads the question at `exception_spec`: reads against (option 2, RV10-2). The exception specification of the reference has only the two `noexcept` forms. The empty `throw()` form was removed in an earlier revision, and the reference lists the removal among its changes. So a program with `throw()` is not valid, and a compiler that accepts it must give a diagnostic. The production derives one form more than the reference. The production and its paragraph follow EXS-5, so the text is consistent with Tidal's own answer. The review gives the reading of the review of 2026-11-03.

### Read of the RV10 repair diff, 2026-11-11

The source-reader read of the RV10 repair diff (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`) reads the Q-0077 line of the exception specifications page and the dated note of decision 0065, item RV10: agrees as a record. Both state RV10-2 correctly, and the repair diff leaves RV10-2 as it was. The read gives no new reading of the question's answer.

# Q-0155: When does one braced string literal have a sequence to a reference to a character array?

**Status:** provisional
**Affects:** ICS-38, `tests/overloading/conversion-sequences/list-string-literal-utf8.cpp`, `tests/overloading/conversion-sequences/list-string-literal-too-short.cpp`, `tests/overloading/conversion-sequences/list-string-literal-bound.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences
**Waits for:** the maintainer (pending.md row 4)

## Context

ICS-38 gives one string literal in braces the identity to a reference to
a character array that the literal can initialize (LIST-6, LIST-15).
Three cases split the compilers.

| Case | Functions | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|---|
| `{u8"ab"}` | `const char (&)[3]`, a class from `const char8_t*` | the array | the class | the array |
| `{"ab"}` | `const char (&)[2]`, `const char*` | tie | `const char*` | `const char*` |
| `{"ab"}` | `const char (&)[]`, `const char (&)[3]` | tie | `[3]` | `[3]` |

## Evidence

The three tests in **Affects**. A source reader's gap review (item S3)
reports that the source treats the braced string literal as its own
case, checked before the array rule.

## Options

1. The literal has a sequence exactly when it can initialize the array.
   A UTF-8 literal initializes a `char` array, a literal that does not
   fit gives no sequence, and ICS-24 prefers the known bound (Clang on
   the first case, GCC and EDG on the others).
2. Each compiler's own answer.

## Provisional answer

When the literal can initialize the array; GCC differs on UTF-8, Clang on size and bound (option 1). Two of three implementations agree on each case, and the
answer follows the initialization rules LIST-6 and LIST-15.

Kept on 2026-11-03. The review reads the literal that does not fit as a tie, with doubt, as Clang 22 does. The answer keeps option 1, which GCC 16 and EDG 7.0 follow and which the table of REF-P3 states: a string literal of another array type makes a temporary only if it can initialize the array, and has no sequence otherwise. The question stays open under decision 0054, because the review reads with doubt.

Kept open on 2026-11-04 under decision 0054, because the gap review reads with doubt.

Kept open on 2026-11-11 (decision 0066, item DQ). The second review reads the literal that does not fit against the answer (DQ-7), as the review of 2026-11-03 did, and it supports the UTF-8 case and the known bound. In `list-string-literal-too-short.cpp` Clang 22 follows the review, and GCC 16 and EDG 7.0 keep the answer. The question waits with `pending.md` row 4 for that case, and the tests keep their expectations.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees for the UTF-8 case and for the known bound, and reads the literal that does not fit against the answer, with doubt. A UTF-8 literal suits a `char` array, so the array is chosen. For two arrays of one element type, the known bound wins. The sequence rule checks only the literal's character kind, not the bound, so a literal too long for the array still has the identity, and the pointer candidate ties with it (Clang's result). The doubt is whether a literal that does not fit counts as suitable.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: against the answer for the literal that does not fit, and supports the UTF-8 case and the known bound (DQ-7). The list rule gives the identity when the one element is a string literal of a kind that suits the array. The suitable kinds come from the rule on character arrays, which matches the literal's kind to the element type and allows a UTF-8 literal for a `char` array. The check on the bound is a separate sentence of that rule, so a literal too long for the array still has the identity. The pointer candidate adds an array-to-pointer step, which the comparison leaves out, so the two sequences tie. That is Clang's result, and it reads against option 1 for `list-string-literal-too-short.cpp`. The known bound wins by the list tie-breaker for arrays of one element type.

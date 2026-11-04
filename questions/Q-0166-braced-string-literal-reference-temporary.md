# Q-0166: Can one braced string literal initialize the temporary array of a reference?

**Status:** settled
**Affects:** LIST-6, REF-14, `tests/declarations/initialization/list/string-literal-reference-temporary.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

A string literal is an lvalue array of `const` characters. When the
reference has that array type, REF-6 binds the reference to the literal
directly. Otherwise REF-14 makes a temporary array from the braced
list, and LIST-6 lets the literal initialize the temporary.

| Declaration | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `const char (&same)[3]{"ab"};` (direct) | accepts | accepts | accepts |
| `const unsigned char (&high)[3]{"ab"};` | temporary | rejects | temporary |
| `const signed char (&low)[3]{"ab"};` | temporary | rejects | temporary |
| `const char (&room)[4]{"ab"};` | temporary | rejects | temporary |
| `const char (&utf)[3]{u8"ab"};` | temporary | rejects | temporary |
| `unsigned char (&bad)[3]{"ab"};` | rejects | rejects | rejects |

A grid of the five literal kinds and seven character types gives the
same pairs for an array variable on every compiler. Only the reference
case splits.

## Evidence

The test in **Affects**. GCC's message is "invalid conversion from
brace-enclosed initializer list to const unsigned char[3]". Q-0155
records the related split for overload resolution: GCC gives a braced
UTF-8 literal no sequence to a `char` array.

## Options

1. The literal initializes the temporary under LIST-6, as for an array
   variable (Clang, EDG).
2. A braced string literal binds a reference only directly (GCC).

## Answer

Yes, under LIST-6; Clang and EDG agree, GCC rejects (option 1). Two of three implementations agree, and the result follows
LIST-6 and REF-14 without a special case.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and REF-P3 gives the same answer: a string literal of another array type makes a temporary when it can initialize the array. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. An array of another character type or another bound is not related to the literal's type, so a temporary is made by copy list initialization. One braced string literal can then initialize a character array of a fitting kind, also a `char` array from a UTF-8 literal. GCC's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

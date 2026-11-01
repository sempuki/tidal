# Q-0166: Can one braced string literal initialize the temporary array of a reference?

**Status:** provisional
**Affects:** LIST-6, REF-14, `tests/declarations/initialization/list/string-literal-reference-temporary.cpp`

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

## Provisional answer

Option 1. Two of three implementations agree, and the result follows
LIST-6 and REF-14 without a special case.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

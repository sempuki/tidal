# Q-0155: When does one braced string literal have a sequence to a reference to a character array?

**Status:** provisional
**Affects:** ICS-38, `tests/foundations/conversion-sequences/list-string-literal-utf8.cpp`, `tests/foundations/conversion-sequences/list-string-literal-too-short.cpp`, `tests/foundations/conversion-sequences/list-string-literal-bound.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

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

Option 1. Two of three implementations agree on each case, and the
answer follows the initialization rules LIST-6 and LIST-15.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

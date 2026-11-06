# Q-0359: Is a read of a volatile object's bytes through `unsigned char` that is not volatile an access under VOL-6?

**Status:** provisional
**Affects:** VOL-6, STG-12, `tests/declarations/volatile/byte-copy-through-plain-bytes.cpp`, `tests/classes/copy-move/trivially-copyable-volatile-member.cpp`
**Waits for:** a review (doubt)

## Context

A class with a `volatile int` member is trivially copyable
(CPY-27), so STG-12 lets a program copy its bytes. VOL-6 makes
an access to a volatile object through a glvalue that is not volatile
undefined. A byte copy through `unsigned char*` reads the bytes of the
volatile member through such a glvalue.

| Compiler | Byte copy through `unsigned char*` | Byte copy through `volatile unsigned char*` |
|---|---|---|
| Clang 18, Clang 22 | runs, gives the values | runs, gives the values |
| GCC 14, GCC 16 | runs, gives the values | runs, gives the values |
| EDG 7.0 | compiles | compiles |

No sanitizer reports either copy. The class is not a literal type,
because of its volatile member, so constant evaluation cannot copy it.

## Evidence

`tests/declarations/volatile/byte-copy-through-plain-bytes.cpp` and
`tests/classes/copy-move/trivially-copyable-volatile-member.cpp`.
Probes by the clean-room writer: Clang and GCC reject a `constexpr`
object of the class as not literal.

## Options

1. A byte glvalue that is not volatile is a glvalue that is not
   volatile, so VOL-6 applies, and the copy must use volatile byte
   glvalues.
2. A byte glvalue reads the object representation, which is outside
   VOL-6, so the plain copy is valid.

## Provisional answer

Yes; the plain byte copy is undefined, and a copy uses volatile byte glvalues (option 1). VOL-6 names no exception for byte types, and
option 1 keeps both STG-12 and VOL-6 as written. No implementation
shows a difference.

Kept open on 2026-11-03 under decision 0054, because the review reads the reference with doubt.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: supports with a doubt. The reference makes an access to an object defined volatile through a glvalue that is not volatile undefined. Whether a byte read of the object representation counts as such an access is not spelled out.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as unclear, leaning to option 1. The reference makes undefined an access to an object defined with a volatile type through a glvalue that is not volatile, and it has no exception for byte types. The reference does not say whether a member counts as defined with a volatile type, or whether the bytes of the object representation count as volatile.

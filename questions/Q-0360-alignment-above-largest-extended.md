# Q-0360: Does an implementation reject an `alignas` above its largest extended alignment?

**Status:** provisional
**Affects:** ALN-5, `tests/declarations/alignment/extended-beyond-largest.cpp`, `tests/declarations/alignment/extended-beyond-largest-ilp32.cpp`

## Context

`struct alignas(1 << 29) Huge { char c; };` asks for an alignment of
2^29. Each compiler in the matrix supports a largest extended alignment
(ALN-5): 2^28 for Clang and GCC, and 2^15 for EDG 7.0.

| Compiler | `alignas(1 << 29)` | `alignof(Huge)` |
|---|---|---|
| GCC 16, GCC 14 | rejects ("exceeds maximum") | none |
| EDG 7.0 | rejects ("invalid alignment value") | none |
| Clang 22, Clang 18 | accepts | 1 |

Clang gives the same result for each power of two from 2^29 to 2^32,
and rejects 2^33. EDG rejects each power of two from 2^16. The results
are the same on x86-64 and with `-m32`.

## Evidence

The tests in **Affects**, with an override for Clang.

## Options

1. An implementation rejects an alignment that it does not support. GCC
   and EDG conform. Clang accepts the declaration and drops the
   alignment.
2. The program is valid, and an alignment that the implementation does
   not support has an implementation-defined effect. Each compiler
   conforms.

## Provisional answer

The implementation rejects the alignment (option 1). Two of three
families reject it, and a dropped alignment breaks ALN-4 for each object
of the class.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. ALN-P3 says that the largest alignment depends on the target, and no principle says that an implementation rejects an alignment above it. Clang 22 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ALN-P3 failed step 3, because it said only that the largest alignment depends on the target. ALN-P3 now says that an implementation rejects an `alignas` above its largest alignment, which rules out option 2. The existing tests check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ALN-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. An operand that gives an extended alignment that the implementation does not support in that context makes the program not valid. Which extended alignments are supported is implementation-defined. A diagnostic is required, so Clang's silent acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

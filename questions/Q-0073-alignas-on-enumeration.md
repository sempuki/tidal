# Q-0073: Can alignas apply to an enumeration?

**Status:** provisional
**Affects:** ALN-7, `tests/declarations/alignment/enum.cpp`

## Context

`enum alignas(8) E { a };` puts `alignas` on an enumeration. Clang 18 and 22 and GCC 16 reject the declaration. EDG 7.0 and GCC 14 accept it. The host GCC 16.2 and Clang 22.1 reject it too.

GCC changed between versions 14 and 16, so GCC 14's acceptance looks
like a defect. EDG 7.0 is the newest EDG in the matrix, and the split
with EDG stays.

## Evidence

`tests/declarations/alignment/enum.cpp`, with overrides for EDG and
GCC 14.

## Options

1. `alignas` must not apply to an enumeration.
2. `alignas` on an enumeration is valid and sets the alignment of the
   enumeration.

## Provisional answer

No; `alignas` must not apply to an enumeration (option 1). Clang and the newer GCC agree. The provisional answer has the weakest support of the group, and an opinion is welcome.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. ALN-P1 says what `alignas` does to a type's alignment, and no principle says which declarations can carry it. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ALN-P1 failed step 3, because no principle said which declarations can carry `alignas`. ALN-P1 now gives a table of the declarations, with an enumeration among those that reject `alignas`. The existing tests check each row. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ALN-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The entities that accept an alignment specifier are variables, data members other than bit-fields, and classes. An enumeration is not a class and is not in that list.

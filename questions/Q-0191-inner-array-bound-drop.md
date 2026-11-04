# Q-0191: Does an array bound dropped at an inner pointer level need `const` above it?

**Status:** provisional
**Affects:** CV-10, `tests/declarations/declarators/qualification-conversion-array-inner-bound.cpp`

## Context

`int (**)[3]` converts to `int (**)[]`. The bound is dropped at the
second level, and the first level has no `const`. With `const` at the
first level, every compiler accepts the conversion.

| Compiler | `int (**)[3]` to `int (**)[]` |
|---|---|
| Clang 18, Clang 22 | rejected |
| GCC 14, GCC 16 | accepted |
| EDG 7.0 | accepted |

Clang treats the dropped bound as it treats added `const`. Through the
converted pointer, a program can store a pointer to an array of another
bound in the original `int (*)[3]`.

## Evidence

`tests/declarations/declarators/qualification-conversion-array-inner-bound.cpp`.

## Options

1. A dropped bound needs no `const` at the levels above (GCC, EDG).
2. A dropped bound needs `const` at each level above, except the
   outermost (Clang).

## Provisional answer

No; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree.

Kept on 2026-11-03. The review reads option 2, which only Clang follows. GCC 16 and EDG 7.0 accept the conversion, and DCL-P4 does not say why `const` must be added at the higher levels, so no principle on the page decides whether a dropped bound counts. Tidal keeps option 1 until the maintainer decides. The question is in row 4 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2. The combined type of a qualification conversion is built level by level, and at any level where it differs from an input, by a qualifier or by an unknown bound where the input has a bound, `const` is added at every level between the outermost and that level. The conversion is valid only if the combined type is the target type. So `int (**)[3]` to `int (**)[]` needs `const` on the middle level. Clang follows this reading. GCC's and EDG's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

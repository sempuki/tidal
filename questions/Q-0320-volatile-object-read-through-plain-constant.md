# Q-0320: Must constant evaluation reject a read of a volatile object through a glvalue that is not volatile?

**Status:** provisional
**Affects:** VOL-6, VOL-8, `tests/declarations/volatile/constant-read-through-plain.cpp`
**Waits for:** newest families that agree

## Context

A `constexpr` function declares a local `volatile int`, casts its
address to `int*` with `const_cast`, and reads the object through the
`int*`. A `constexpr` variable takes the function's result.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects: read of volatile object is not allowed in a constant expression |
| GCC 16, 14 | accepts |
| EDG 7.0 | accepts |

Every compiler rejects a read through a volatile glvalue in the same
place (`constant-read-rejected.cpp`, VOL-8). No sanitizer of GCC or
Clang reports the read at run time.

## Evidence

The test above, and `nonvolatile-access-to-volatile-constant.cpp`.

## Options

1. Reject. The read is undefined (VOL-6), and constant evaluation
   rejects undefined behavior. The object is volatile, so VOL-8 also
   applies.
2. Accept. Constant evaluation looks only at the type of the glvalue,
   which is not volatile.

## Provisional answer

Yes, undefined (VOL-6); GCC and EDG accept (option 1). VOL-6 makes the read undefined, and constant evaluation
rejects an expression whose evaluation has undefined behavior. Clang
follows option 1. The test records GCC and EDG as overrides.

The question stays open under decision 0051 (2026-11-03). Clang 22 rejects the read, and GCC 16 and EDG 7.0 accept it, so two families disagree. The provisional answer follows the one family. CE-P3 lists the read of a volatile object among the operations that an evaluation rejects, and the object here is volatile (VOL-6, VOL-8). Every compiler rejects a read through a volatile glvalue in the same place. The front end's interpreter rejects the read with CE-12.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer in at least one form, and the rule records one differing family.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1 through two separate rules. An access to a volatile object through a glvalue that is not volatile is undefined. Separately, a constant evaluation can read an object whose lifetime began in the evaluation only when the glvalue and the object are both not volatile. Here the object is volatile, so the read is not allowed apart from the first rule. Option 2 needs the check to look at the glvalue alone, and the reference names the object.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

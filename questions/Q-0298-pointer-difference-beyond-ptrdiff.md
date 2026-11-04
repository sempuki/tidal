# Q-0298: How do implementations treat a pointer difference that does not fit its type?

**Status:** settled
**Affects:** ADD-18, TR-14, `tests/expressions/additive/pointer-difference-overflow-ilp32.cpp`, `tests/expressions/additive/pointer-difference-largest-ilp32.cpp`

## Context

The difference of two pointers has a signed type that the
implementation chooses (ADD-12). With `-m32` the type is `int`, so an
array of `char` with more than `INT_MAX` elements can hold two pointers
whose difference does not fit.

| Case, with `-m32` | Clang 22, 18 | EDG 7.0 | GCC 16, 14 |
|---|---|---|---|
| declare `extern char big[0x80000000u]` | accepts | accepts | rejects: "exceeds maximum object size" |
| `&big[0x7fffffff] - &big[0]` in a constant | accepts, `INT_MAX` | accepts, `INT_MAX` | needs the array |
| `&big[0x80000000u] - &big[0]` in a constant | rejects: "value 2147483648 is outside the range" | rejects: "integer operation result is out of range" | needs the array |
| `&big[0] - &big[0x80000000u]`, which is `INT_MIN` and fits | accepts | rejects | needs the array |
| `&big[0x7fffffff] + 1`, with no difference | accepts | rejects | needs the array |

Clang rejects only a difference that does not fit. EDG rejects every
position past `INT_MAX` in constant evaluation, also where no difference
is taken. GCC keeps every object at or below `INT_MAX` bytes on this
target, so a difference of two pointers into one array always fits. On
x86-64 the type is `long`. Clang and GCC reject an array of `char` with more
than `LONG_MAX` elements there, and EDG accepts it.

## Evidence

The tests in Affects, and probes of the cases in the table with `-m32`
on every compiler in the matrix that takes the flag. Programs with
`-m32` do not link on the matrix hosts, so the matrix has no run-time
result.

## Options

1. A difference that does not fit its type is undefined (ADD-18).
   GCC's object limit is an implementation limit (TR-14). EDG's
   rejection of positions past `INT_MAX` is a missing feature of its
   constant evaluator, and a possible defect.
2. The same as option 1, and an implementation must also keep each
   object small enough that every difference fits, as GCC does.

## Answer

Undefined (ADD-18); EDG rejects positions past the largest difference (option 1). Clang and EDG reject the overflow during constant evaluation,
and GCC rejects the array that would allow it. No implementation gives a
difference that does not fit. Option 2 would make Clang and EDG
nonconforming for accepting the array.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and ADD-P3 gives the same answer: a count that does not fit the type has no value. EDG 7.0 differs on positions past the largest difference, and the tests keep their override for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. A difference outside the range of the result type is undefined. A position inside the array, or one past its end, is a valid operand of addition, so EDG's rejection of a position past the largest difference follows from no rule.

# Q-0039: Is copying an uninitialized `unsigned char` undefined?

**Status:** provisional
**Affects:** INIT-1, `tests/declarations/initialization/forms/copy-indeterminate-unsigned-char.cpp`, `copy-indeterminate-unsigned-char-run.cpp`

## Context

A gap review summary says that copying an uninitialized `unsigned char` (or `std::byte`) into another object of the same kind is not undefined; the copy just holds an unknown value. During constant evaluation, every compiler in the matrix rejects the copy. At run time no compiler shows a difference, so the test can't tell the two answers apart.

## Options

1. Not undefined at run time for `unsigned char` and `std::byte`; rejected during constant evaluation, as every compiler does.
2. Undefined for every type, as INIT-1 says now.

## Provisional answer

Not undefined at run time for `unsigned char` and `std::byte`, and rejected during constant evaluation (option 1). The constant-evaluation behavior is observed, and the run-time exception comes from the review summary. A test that can observe the run-time difference would settle the question.

## Opinions

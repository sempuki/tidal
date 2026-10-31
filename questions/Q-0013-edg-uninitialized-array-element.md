# Q-0013: Does EDG accepting a read of an uninitialized array element during constant evaluation change INIT-1?

**Status:** compiler defect
**Affects:** INIT-1, `tests/declarations/initialization/forms/read-uninitialized-array.cpp`

## Context

Reading an automatic variable that was never given a value is rejected
during constant evaluation by every compiler in the matrix. For an array
whose element was never written, Clang and GCC reject the read and EDG 7.0
accepts it. EDG rejects the same read of a scalar
(`read-uninitialized.cpp`).

## Evidence

`tests/RESULTS.md`, rows `read-uninitialized.cpp` and
`read-uninitialized-array.cpp`.

## Options

1. Keep INIT-1 `undefined` for arrays and scalars alike, and report EDG's
   behavior as a possible defect.
2. Treat array elements differently from scalars.

## Provisional answer

Option 1. EDG treats the scalar and array cases differently, and the other
implementations treat them the same.

## Opinions

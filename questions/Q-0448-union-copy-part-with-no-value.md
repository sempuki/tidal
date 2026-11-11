# Q-0448: After a trivial copy of a union in constant evaluation, does a part that had no value in the source have a value in the copy?

**Status:** provisional
**Affects:** CE-22, CE-P3, `tests/foundations/constant-evaluation/union-copy-part-with-no-value.cpp`, `tests/foundations/constant-evaluation/union-copy-part-with-no-value-read.cpp`
**Source:** X25-5 (`reports/clean-room/X25/report.md`, probes `a3.cpp` and `a3c.cpp`), rerun on the matrix in `reports/clean-room/roadmap-13/writer-probes/x25-constant.json` and `x25-rules.json`, and by the item F6 writer with `tools/probe_grid.py`.
**Waits for:** a first review

## Context

A union has an `int` array as its active member, and only the first
element has a value. During constant evaluation, the program copies
the union with its trivial copy constructor and then reads an element
of the copy.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| the copy, a move, and a copy assignment, then a read of the first element | accepts | accepts | accepts |
| the copy, then a read of the element with no value | rejects | rejects | accepts |
| the same with a class member `Point` whose `y` has no value, then a read of `y` | rejects | rejects | accepts |
| a copy assignment into a target whose element has a value, then a read of that element | rejects | rejects | rejects |
| the same copy of a struct that is not a union | rejects | accepts | rejects |

The last row is the case of Q-0115. The copy of a struct reads its
parts, and the copy of a union does not on any compiler.

## Evidence

The tests in **Affects** and the probes in **Source**.

## Options

1. The copy reads no part, and a part with no value in the source has
   no value in the copy. A later read of the part is rejected (Clang,
   GCC).
2. The copy gives the part a value, and a later read is valid (EDG for
   the copy constructor; no compiler for the copy assignment).

## Provisional answer

The part has no value in the copy, and a read of it is rejected
(option 1). CE-22 says so. Clang and GCC agree, and EDG alone differs.
EDG also rejects the read after a copy assignment, so EDG's result
depends on the form of the copy.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Q-0132: Does a braced list have a conversion sequence to a class parameter when the chosen constructor is `explicit`?

**Status:** provisional
**Affects:** ICS-8, ICS-19, `tests/foundations/conversion-sequences/chosen-explicit-copy-list.cpp`

## Context

A function parameter is copy-initialized from its argument. For a
braced list and a class parameter, the list chooses a constructor
(ILC-3). ILC-3 says that `explicit` constructors are candidates in copy
list initialization, and ILC-8 rejects the program when the best one is
`explicit`.

The test has two functions. One takes a class `Strict` whose only
constructor `Strict(int, int)` is `explicit`. The other takes a class
`Loose` with `Loose(long, long)`. A second argument makes the first
function better if `{1, 2}` has a sequence to `Strict`.

| Compiler | Result |
|---|---|
| GCC 14, GCC 16 | chooses the function for `Strict`, then rejects the call ("would use explicit constructor") |
| Clang 18, Clang 22 | chooses the function for `Loose` and accepts |
| EDG 7.0 | chooses the function for `Loose` and accepts |

With only the function for `Strict`, every compiler rejects the call.

## Evidence

`tests/foundations/conversion-sequences/chosen-explicit-copy-list.cpp`.
The probe compiled the test with `-std=c++23 -pedantic-errors` (Clang,
GCC) and `--c++23 --strict` (EDG).

## Options

1. The sequence exists. The `explicit` constructor takes part as ILC-3
   says, and the call fails only if its function is chosen (GCC).
2. No sequence exists. An `explicit` constructor gives no sequence for a
   copy-initialized parameter, so the other function is chosen (Clang,
   EDG).

## Provisional answer

Option 1. The answer follows ILC-3, which keeps `explicit` constructors
as candidates in copy list initialization, and the model of ICS-8, where
faults of the chosen function do not stop a sequence from forming.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

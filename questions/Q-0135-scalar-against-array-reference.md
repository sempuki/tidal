# Q-0135: Does a braced list prefer a scalar parameter to a reference to an array when both sequences are the identity?

**Status:** provisional
**Affects:** ICS-17, `tests/foundations/conversion-sequences/list-array-against-scalar.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

The argument is `{1}`. One function takes `int`, and the other takes
`int (&&)[2]`. The sequence to `int` is the identity (ICS-15). The
sequence to the array is the identity for each element (ICS-17). No
tie-breaker on the page names this pair.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | tie |
| EDG 7.0 | tie |
| GCC 14, GCC 16 | prefers `int` |

The lists grid shows the same split for `int (&&)[]`.

## Evidence

`tests/foundations/conversion-sequences/list-array-against-scalar.cpp`,
and the "one {1}" rows of the lists grid on the page.

## Options

1. The two sequences tie (Clang, EDG).
2. The sequence to the scalar is better (GCC). The page then needs a
   tie-breaker for this case.

## Provisional answer

No, they tie; GCC differs (option 1). Two of three implementations agree, and the page's rules give
a tie with no further rule.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

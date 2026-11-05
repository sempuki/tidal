# Q-0380: What type does a hexadecimal literal with a `z` suffix get when only `std::size_t` can hold it?

**Status:** provisional
**Affects:** PRIM-1, PRIM-P1, `tests/expressions/primary/int-size-suffix-hex-unsigned.cpp`
**Source:** finding X19-4 of `reports/clean-room/X19/report.md`

## Context

PRIM-1 gives a literal with a `z` suffix the signed type that matches
`std::size_t`. For the other suffixes, an octal, hexadecimal, or binary
literal can take the unsigned type of the same rank when the signed
type cannot hold the value. The finding asked for a row of the PRIM-P1
table for `z`, and a probe of the row found a split on x86-64:

| Literal | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `0xFFFF'FFFF'FFFF'FFFFz` | `std::size_t` | `long`, with no diagnostic | `std::size_t` |
| `18446744073709551615z` | rejected | rejected | rejected |

## Evidence

The test in Affects. Clang and EDG accept it. GCC fails its first
assertion.

## Options

1. The literal has the type `std::size_t` (Clang, EDG). The list of a
   `z` literal in a base other than decimal follows the pattern of the
   other suffixes: the signed type, then its unsigned type.
2. The literal has the signed type, and its value changes (GCC).
3. The literal is not valid, as for a decimal literal.

## Provisional answer

`std::size_t` (option 1). Two of three families agree, and the answer
keeps the pattern of the PRIM-P1 table, where a literal in a base
other than decimal can take the unsigned type of its rank. Option 2
changes the value of a literal in silence, which PRIM-P1 rules out for
every other literal.

Kept open on 2026-11-04 under decision 0054, because no review has read the question. Item RV2 of decision 0056 reads it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

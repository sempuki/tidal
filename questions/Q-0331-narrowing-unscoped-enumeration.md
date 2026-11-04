# Q-0331: Which values does an unscoped enumeration without a fixed type have for narrowing?

**Status:** provisional
**Affects:** LIST-2, `tests/declarations/initialization/list/narrowing-unscoped-enum-values-fit.cpp`

## Context

LIST-2 makes a conversion from an integer source narrowing when the
target type cannot hold all the values of the source. An unscoped
enumeration without a fixed type counts as an integer source. Two sets
of values are possible for such an enumeration:

- the values of the smallest set of bits that holds every enumerator,
  which ENUM-22 gives,
- every value of the underlying type that the implementation picks.

For `enum E { A = 0, B = 100 };`, the first set is 0 to 127, and
`unsigned char` holds it. The second set is every `int` or `unsigned
int` value, and `unsigned char` does not hold it.

GCC 14 and 16 accept `unsigned char c{e};` for this enumeration. GCC
also accepts an enumeration with the enumerators 0 and 255, and rejects
one with 0 and 200 for `signed char` and one with -1 and 100 for
`unsigned char`. So GCC uses the ENUM-22 set. Clang 18 and 22 and EDG 7.0
reject each of these programs. They use the values of the underlying
type.

## Evidence

- `tests/declarations/initialization/list/narrowing-unscoped-enum-values-fit.cpp`
- `tests/declarations/initialization/list/narrowing-unscoped-enum.cpp`:
  every compiler rejects an enumeration with the enumerators 0 and 1000
  for `unsigned char`, which both sets predict.
- Clang: "non-constant-expression cannot be narrowed from type 'E' to
  'unsigned char'". EDG: "invalid narrowing conversion from "E" to
  "unsigned char"".

## Options

1. The values that ENUM-22 gives. GCC conforms, and Clang and EDG reject
   valid programs. The rule agrees with ENUM-22, under which a value
   outside the set cannot occur in a valid program.
2. Every value of the underlying type. Clang and EDG conform, and GCC
   accepts invalid programs. The result then depends on the underlying
   type, which ENUM-5 leaves to the implementation.

## Provisional answer

The values that ENUM-22 gives (option 1). An enumeration without a fixed
type holds only those values (ENUM-22), so a conversion of such a value
to a type that holds all of them loses nothing. Option 1 also makes the
result independent of the underlying type that the implementation
picks.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer, and the rule records one differing family. Clang 22 and EDG 7.0 use the values of the underlying type.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. Narrowing asks whether the target holds all the values of the source type. An enumeration without a fixed type has the values of the smallest bit width that holds every enumerator. GCC follows this reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

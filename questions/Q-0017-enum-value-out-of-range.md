# Q-0017: Is converting a value outside an enumeration's range undefined?

**Status:** provisional
**Affects:** ENUM-6, `tests/declarations/enumerations/cast-out-of-range.cpp`

## Context

For `enum E { A = 1, B = 4 };`, which has no fixed underlying type, Clang
computes a range of values from the enumerators ("valid range of values
[0, 7]") and rejects `static_cast<E>(8)` during constant evaluation. GCC
16 and 14 and EDG 7.0 accept it. All compilers accept `static_cast<E>(7)`,
and any value for an enumeration with a fixed underlying type.

## Evidence

`tests/RESULTS.md`, rows `cast-in-range.cpp`, `cast-out-of-range.cpp`, and
`cast-fixed-any-value.cpp`. Clang 22's note: "integer value 8 is outside
the valid range of values [0, 7] for the enumeration type 'E'".

## Options

1. `undefined` for values outside the smallest range of bits that holds
   all enumerators. Clang diagnoses it; GCC and EDG don't have to.
2. `required`: any value of the underlying type is allowed, as GCC and EDG
   behave.

## Provisional answer

Option 1. An implementation can only diagnose this during constant
evaluation if the operation is invalid, and an implementation that treats
it as valid (option 2) would forbid Clang's diagnostic.

## Opinions

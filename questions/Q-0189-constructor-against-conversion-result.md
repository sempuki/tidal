# Q-0189: Does the result tie-breaker compare a constructor with a conversion function?

**Status:** provisional
**Affects:** OVL-26, `tests/foundations/overload-resolution/tie-conversion-result-constructor.cpp`

## Context

`Target` has a constructor from `const Source&`. `Source` has a
conversion function to `Derived`, a class derived from `Target`. A
`Target` is copy-initialized from a `Source` lvalue. Both candidates bind
`const Source&` to the lvalue, so the argument sequences tie. The
constructor's result is a `Target`. The conversion function's result
needs a derived-to-base conversion.

| Compiler | Copy initialization | Argument of a function |
|---|---|---|
| Clang 18, Clang 22 | ambiguous | ambiguous |
| GCC 14, GCC 16 | the constructor | the constructor |
| EDG 7.0 | the constructor | the constructor |

When both results are a `Target`, every compiler finds the two
ambiguous.

## Evidence

`tests/foundations/overload-resolution/tie-conversion-result-constructor.cpp`
and `init-constructor-conversion-tie.cpp`.

## Options

1. The result tie-breaker compares a constructor, whose result has the
   identity, with a conversion function (GCC, EDG).
2. The result tie-breaker compares only two conversion functions
   (Clang).

## Provisional answer

Yes; the constructor's result has the identity; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

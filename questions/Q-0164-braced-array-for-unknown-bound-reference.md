# Q-0164: Does one braced array lvalue bind a reference to an array of unknown bound directly?

**Status:** provisional
**Affects:** REF-6, `tests/declarations/initialization/references/braced-list-array-unknown-bound.cpp`

## Context

The array `store` has type `int[3]`. The declaration is
`int (&whole)[]{store};`. After `=`, every compiler binds `int (&)[]`
to `store` (REF-1).

| Declaration | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `int (&whole)[]{store};` | binds to `store` | makes a temporary `int[1]` from the list and rejects it | binds to `store` |
| `const int (&whole)[]{store};` | binds to `store` | rejects, as above | binds to `store` |
| `const char (&text)[]{"ab"};` | accepts | accepts | accepts |

For a reference to an array of known bound, every compiler binds one
braced array lvalue of that type directly (REF-6).

## Evidence

The test in **Affects**. GCC's message is "invalid conversion from
brace-enclosed initializer list to int[1]". EDG also rejects a
constant expression that names an element through any `const int (&)[]`
bound to a `constexpr` array, with or without braces, so the test checks
the binding with a non-`const` reference.

## Options

1. Bind directly, as after `=` (Clang, EDG). REF-6 treats an array of
   known bound as the referenced type of an array of unknown bound.
2. Make a temporary from the list (GCC). The list then holds an array
   where an element is expected, and the program is rejected.

## Provisional answer

Option 1. Two of three implementations agree, and the result matches
the binding after `=` and the binding to an array of known bound.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

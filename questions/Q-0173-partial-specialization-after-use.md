# Q-0173: Must an implementation diagnose a partial specialization declared after a use that it would match?

**Status:** provisional
**Affects:** TSP-17

## Context

A program completes `Kind<int*>` and then declares the partial
specialization `Kind<T*>`, which matches `int*`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, and the earlier use keeps the primary template |
| GCC 14, 16, host | rejects |
| EDG 7.0 | rejects |

For an explicit specialization after a use, all compilers reject the
program (TSP-6).

## Evidence

`tests/templates/specialization/partial-after-use.cpp`, with an override
for Clang.

## Options

1. The program is not valid, and an implementation need not diagnose it
   (TSP-17 as `no-diagnostic`).
2. The program is not valid, and the implementation must diagnose it
   (`reject`, as GCC and EDG do).
3. The program is valid.

## Provisional answer

No, no diagnostic required; GCC and EDG reject, Clang accepts (option 1). GCC and EDG show that the order is a fault. Clang shows that
a mature implementation does not check it, and a use in another
translation unit cannot be checked at all.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

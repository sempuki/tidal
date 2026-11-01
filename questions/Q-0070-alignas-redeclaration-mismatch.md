# Q-0070: Must all declarations of a variable agree on alignas?

**Status:** provisional
**Affects:** ALN-3, the four tests under Evidence

## Context

A variable is declared with one `alignas` and defined with a different one, or declared with `alignas` and defined without it. Clang 18 and 22 and EDG 7.0 reject both. GCC 14 and 16 accept both.

GCC also accepts two declarations of one class with different `alignas`,
and a class definition without the `alignas` of an earlier declaration.
Clang and EDG reject both.

## Evidence

`tests/declarations/alignment/redeclare-different.cpp`,
`definition-without-earlier-with.cpp`, `class-redeclare-different.cpp`,
and `class-definition-without-earlier-with.cpp`, each with an override
for GCC.

## Options

1. If a declaration has `alignas`, the definition must have the same alignment.
2. Declarations can differ.

## Provisional answer

Yes; if a declaration has `alignas`, the definition must have the same alignment (option 1). Clang and EDG agree.

## Opinions

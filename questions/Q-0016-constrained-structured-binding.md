# Q-0016: May a structured binding use a constrained `auto`?

**Status:** compiler defect
**Affects:** SB-7, `tests/declarations/structured-bindings/constrained.cpp`

## Context

`Any auto [x, y] = a;`, where `Any` is a concept satisfied by every type,
is rejected by Clang 22 and 18 ("cannot be declared with constrained
'auto'") and GCC 16 and 14 ("cannot have constrained 'auto' type"). EDG 7.0
accepts it.

## Evidence

`tests/RESULTS.md`, row `constrained.cpp`.

## Options

1. Rejected. EDG's acceptance is a possible defect.
2. Allowed, with the constraint checked against the type of the hidden
   variable.

## Provisional answer

Option 1. Two of three implementations reject it with a diagnostic
written specifically for this case.

## Opinions

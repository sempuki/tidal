# Q-0172: Must an implementation reject a partial specialization of a function template that no call uses?

**Status:** provisional
**Affects:** TSP-7

## Context

`template <class T> int pick<T, int>(T, int)` declares a partial
specialization of a function template.

| Program | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| the declaration only | rejects | rejects | accepts |
| the declaration and a call of `pick` | rejects | rejects | rejects at the call |

## Evidence

`tests/templates/specialization/function-partial.cpp` (an override for
EDG) and `tests/templates/specialization/function-partial-used.cpp`.

## Options

1. The declaration is not valid, and the implementation diagnoses it
   (Clang, GCC).
2. The implementation need not diagnose it until a use (EDG).

## Provisional answer

Option 1. Clang and GCC reject the declaration itself, and EDG rejects
it as soon as the template is used. TSP-7 stays a `reject` rule.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

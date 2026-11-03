# Q-0220: Does a placement deallocation function win over a template that also matches?

**Status:** provisional
**Affects:** NEW-23, `tests/expressions/new-delete/throw-placement-function-and-template.cpp`, `tests/expressions/new-delete/throw-placement-function-before-template-instantiation.cpp`

## Context

A class has the placement allocation function
`operator new(size, Arena*)`. The class also has the deallocation
function `operator delete(void*, Arena*)` and the template
`operator delete(void*, T*)`. Both match the allocation function, the
template with `T = Arena`. The constructor throws.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | calls the function |
| EDG 7.0 | chooses the function |
| Clang 18, 22, host | calls no deallocation function, and warns that two match |

EDG has no code generation. Two probes show its choice. EDG rejects
the program when the function is deleted. EDG accepts the program when
the body of the template holds a failing `static_assert`.

## Evidence

`tests/expressions/new-delete/throw-placement-function-and-template.cpp`,
with an override for Clang, and
`tests/expressions/new-delete/throw-placement-function-before-template-instantiation.cpp`.

## Options

1. The function wins over the template, as in overload resolution
   (GCC, EDG).
2. Two matches give no deallocation function (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang calls none (option 1). GCC and EDG agree. The answer also agrees with
Q-0219, where an order between the matches chooses one.

The question stays open under decision 0051 (2026-11-03). GCC 16 calls the function, EDG 7.0 chooses it, and Clang 22 calls no deallocation function, so two families disagree. OVL-27 supports the provisional answer. In overload resolution, a function that is not a template specialization wins over a specialization, and NEW-23 applies the same preference to the match.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

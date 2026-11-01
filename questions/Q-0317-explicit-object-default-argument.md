# Q-0317: Can an explicit object parameter have a default argument?

**Status:** provisional
**Affects:** CLS-14, `tests/classes/members/explicit-object-default-argument.cpp`

## Context

A member function declares its explicit object parameter as
`this Gauge self = Gauge{}`.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects |
| GCC 16, 14 | rejects |
| EDG 7.0 | accepts |

The object of a call always initializes the parameter (CLS-13), so a
default argument for it can never be used.

## Evidence

The test above. Clang says that the explicit object parameter cannot
have a default argument, and GCC says the same in other words.

## Options

1. The default argument is not valid.
2. The default argument is valid and has no effect.

## Provisional answer

Option 1. Clang and GCC reject the declaration, and the default
argument could never take effect. The test expects a rejection and
records EDG's result with an override.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Q-0007: Which language version does Tidal track, and how are newer features handled?

**Status:** provisional
**Affects:** charter, decision 0002, `tests/statements/if/condition-structured-binding.cpp`

## Context

Tidal's baseline is C++23. Compilers already implement some newer features
and accept them in C++23 mode with a warning (for example, a structured
binding as an `if` condition). Under `-pedantic-errors` the compilers
reject these features.

## Options

1. Track C++23 only. Newer features count as rejected in strict mode, as
   the tests do now.
2. Track the newest version implemented by all baseline compilers, and
   mark rules with the version they apply to.

## Provisional answer

Option 1 for the proof of concept. Option 2 is needed before Tidal can
follow the language forward.

## Opinions

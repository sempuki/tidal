# Q-0066: Must an implementation diagnose a class member that changes the meaning of a name?

**Status:** provisional
**Affects:** TYN-5, `tests/declarations/type-names/alias-shadow-changes-meaning.cpp`, `tests/declarations/type-names/function-name-changes-meaning.cpp`

## Context

A class uses a name from an enclosing scope, for example a type alias, in
a member declaration. A later member of the class declares the same name
with a different meaning. GCC rejects the class. Clang and EDG accept it.

## Options

1. The class is not valid, and an implementation need not diagnose it
   (`no-diagnostic`).
2. Implementations must reject the class.
3. The class is valid.

## Provisional answer

Option 1. GCC diagnoses the class. Clang and EDG still conform.

## Opinions

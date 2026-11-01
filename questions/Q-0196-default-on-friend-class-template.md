# Q-0196: Can a friend declaration of a class template give a default template argument?

**Status:** provisional
**Affects:** TPL-37

## Context

A class declares `template <class T = int> friend struct Box;` for a
class template `Box` declared before it.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: default argument on a friend template |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: default argument on a friend |

All compilers reject a default on a friend function template
declaration that is not a definition.

## Evidence

`tests/templates/declarations/default-friend-class.cpp`, with an
override for GCC, and `default-friend-declaration.cpp`.

## Options

1. A friend declaration of a class template must not give a default
   (Clang, EDG).
2. The default is valid (GCC).

## Provisional answer

Option 1. Clang and EDG agree, and the rule then matches the rule for
friend function template declarations, which all compilers apply.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

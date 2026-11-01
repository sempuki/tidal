# Q-0199: Can a member enumeration of a class template be defined after the class body?

**Status:** provisional
**Affects:** TPL-42

## Context

A class template declares `enum class Kind : T;` and `enum Plain : int;`
in its body, and defines both after the body with the class's template
head.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | rejects under `-pedantic-errors`: the definition is an enumeration template |
| EDG 7.0 | accepts |

GCC accepts the program without `-pedantic-errors`. All compilers accept
a member class defined after the body.

## Evidence

`tests/templates/declarations/member-enum-outside.cpp`, with an
override for GCC, and `member-class-outside.cpp`.

## Options

1. The definition is valid (Clang, EDG).
2. The definition is not valid in strict mode (GCC).

## Provisional answer

Yes; Clang and EDG accept, GCC rejects under -pedantic-errors (option 1). Clang and EDG agree, and GCC treats the form as an
extension, so GCC gives it the same meaning.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

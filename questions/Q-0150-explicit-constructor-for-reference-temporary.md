# Q-0150: Can a braced list for a reference use an explicit constructor to make the temporary?

**Status:** provisional
**Affects:** REF-11, REF-14, `tests/declarations/initialization/references/braced-list-explicit-constructor.cpp`

## Context

A braced list for a reference to a class makes a temporary of the class
(REF-14). The compilers differ when the only constructor is `explicit`
and the list has braces with no `=`, as in `const Meters& length{3}`:

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | accepts |
| GCC 14, GCC 16 | rejects ("would use explicit constructor") |
| EDG 7.0 | rejects ("copy-list-initialization cannot use a constructor marked `explicit`") |

With `=`, as in `const Meters& length = {3}`, all compilers reject the
list.

## Evidence

`braced-list-explicit-constructor.cpp`.

## Options

1. The temporary is copy-initialized from the list, so an `explicit`
   constructor is not valid (GCC, EDG).
2. The temporary is initialized as the braces of the reference are, so
   braces without `=` can use an `explicit` constructor (Clang).

## Provisional answer

Option 1. Two of the three compilers reject the list, and the result
then does not depend on the presence of `=`.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

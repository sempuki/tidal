# Q-0198: Can a friend function in a class template have the name of a template parameter?

**Status:** provisional
**Affects:** TPL-48, TPL-18

## Context

A class template `template <class T> struct Holder` declares
`friend void T();`, and the program uses `Holder<int>`. A second form
defines the friend in the class: `friend void T(Holder) {}`.

| Compiler | Friend declaration | Friend definition |
|---|---|---|
| Clang 18, 22, host | accepts | accepts |
| GCC 14, 16, host | accepts | rejects: shadows template parameter |
| EDG 7.0 | rejects at the use of `Holder<int>`: the parameter is redeclared | rejects at the use |

Without a use of `Holder<int>`, all compilers accept the declaration.

## Evidence

`tests/templates/declarations/parameter-friend-name.cpp`, with an
override for EDG. The friend definition has no test.

## Options

1. A friend function declaration with the name is valid, because the
   friend binds no name in the template's scope (Clang, GCC).
2. The friend redeclares the parameter and is not valid (EDG).

## Provisional answer

Yes for the friend declaration (option 1), where Clang and GCC agree. The
friend definition stays open: Clang accepts it, and GCC and EDG reject
it. Tidal states no rule for the definition.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

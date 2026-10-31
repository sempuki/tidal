# Q-0126: When does a union or an anonymous union give its members a value for a const object without an initializer?

**Status:** provisional
**Affects:** CTOR-4, CTOR-24, `tests/classes/constructors/const-object-union-no-init.cpp`, `tests/classes/constructors/const-object-union-member-init.cpp`, `tests/classes/constructors/const-object-anonymous-union-init.cpp`, `tests/classes/constructors/const-object-anonymous-union-no-init.cpp`, `tests/classes/constructors/const-object-only-anonymous-unions.cpp`

## Context

CTOR-4 rejects a `const` object without an initializer unless its class
gives each member a value. Only one member of a union can have a value.
The compilers decide the union cases in different ways:

| Case | Clang 22 | Clang 18 | GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|---|---|
| Union, no default member initializer | rejects | rejects | rejects | rejects |
| Union, one variant member with a default member initializer | accepts | rejects | rejects | accepts |
| Class with another member and an anonymous union that has such a member | accepts | rejects | accepts | accepts |
| Class with another member and an anonymous union that has none | rejects | rejects | accepts | rejects |
| Class whose data members are all anonymous unions without one | rejects | rejects | accepts | accepts |

Clang 22 applies one model: a union, named or anonymous, gives a value
if one variant member has a default member initializer. GCC never
accepts a named union and never checks an anonymous union. EDG follows
the Clang 22 model, except that EDG accepts a class whose data members
are all in anonymous unions with no default member initializer.

## Evidence

The five tests above. The probes compiled each case with
`-std=c++23 -pedantic-errors` (Clang, GCC) and `--c++23 --strict` (EDG).

## Options

1. Follow the majority in each case. The fifth row is then valid, and
   the rule needs a sentence for that row.
2. Follow the Clang 22 model in every case. The fifth row is then not
   valid, and the rule needs no extra sentence.

## Provisional answer

Option 1, because Tidal follows the majority where the compilers
disagree. In the fifth row, GCC and EDG accept by different models.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

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
if any variant member carries a default member initializer. GCC never
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

A union, named or anonymous, gives a value when one variant member carries a default member initializer, and a class of only anonymous unions without one is not valid (option 2). Clang 22 follows this model in every row. GCC and EDG accept the fifth row.

Changed on 2026-11-03 for the fifth row, to option 2. The review reads option 2, Clang 22 rejects, and CTOR-P2 gives the same answer: a `const` object with no initializer needs a class that gives every part a value, and in the fifth row no part gets one. CTOR-24 no longer has a sentence for that row, and `const-object-only-anonymous-unions.cpp` expects a rejection, with overrides for GCC and EDG. Rows one to four keep their answers. The question stays open under decision 0054, because two newest families differ from the answer in the fifth row, and GCC differs in the second row.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer in the fifth row.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with rows one to four and reads option 2 for the fifth row. A `const` object of a union with a data member needs exactly one variant member with a default member initializer. In a class, each anonymous union with a data member needs one too. A class whose data members are all in anonymous unions without one fails that condition, and nothing exempts a class with no other data members. Clang 22 follows this reading. GCC and EDG accept the fifth row.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

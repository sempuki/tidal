# Q-0192: Does a class with a trivial default constructor and no initializer have a constant initializer?

**Status:** provisional
**Affects:** CE-24, CE-21, CINIT-1, `tests/foundations/constant-evaluation/constant-initializer-zero-trivial-class.cpp`, `tests/foundations/constant-evaluation/constant-initializer-trivial-class-block-static.cpp`, `tests/foundations/constant-evaluation/constant-initializer-trivial-class-array.cpp`, `tests/foundations/constant-evaluation/constant-initializer-trivial-empty-class.cpp`

## Context

A variable with static storage holds zero before any other
initialization (STO-3). For a variable with no initializer, CE-24 judges
the default initialization. CE-21 says that the zero does not give a
part a value.

The compilers agree on these cases:

| Variable at namespace scope | Clang | GCC | EDG |
|---|---|---|---|
| `constinit int counter;` | accepts | accepts | accepts |
| `constinit int values[3];` | accepts | accepts | accepts |
| `constinit` class whose `constexpr` constructor leaves a member unset | rejects | rejects | rejects |
| `constinit` empty class with a trivial default constructor | accepts | accepts | accepts |

The compilers split on one case. Take
`struct Pair { int first; int second; }; constinit Pair pair;`. The
default constructor of `Pair` is trivial. GCC and EDG accept the
variable, and the members read as zero at run time. Clang 18 and Clang
22 reject the variable and say that it has no constant initializer. A
`static constinit` variable in a block and a `constinit` array of
`Pair` give the same split.

## Evidence

- `tests/foundations/constant-evaluation/constant-initializer-zero-scalar.cpp`
- `tests/foundations/constant-evaluation/constant-initializer-zero-trivial-class.cpp`,
  with overrides for GCC and EDG
- `tests/foundations/constant-evaluation/constant-initializer-trivial-class-block-static.cpp`,
  with overrides for GCC and EDG
- `tests/foundations/constant-evaluation/constant-initializer-trivial-class-array.cpp`,
  with overrides for GCC and EDG
- `tests/foundations/constant-evaluation/constant-initializer-trivial-empty-class.cpp`
- `tests/foundations/constant-evaluation/result-unset-member.cpp`
- The gap review of question cluster 4
  (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads
  the reference.

## Options

1. A default initialization that calls no constructor, or only a
   trivial one, leaves the zero, and the zero is a constant initializer.
   A class with a trivial default constructor then behaves as a scalar
   does. Clang differs.
2. Only a scalar or an array of scalars keeps the zero as a constant
   initializer. A class must get a value for each member from its
   initialization. GCC and EDG differ.

## Provisional answer

No; the default constructor runs, the zero does not count, and a class with a scalar member has no constant initializer (option 2). Clang rejects the variable.

Changed on 2026-11-03. The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 2, with doubt. Tidal's own rules give the same answer without the exception that option 1 added to CE-24. INIT-9 makes the default initialization of a class a call of the default constructor, and of a scalar nothing. CE-21 says that the zero of STO-3 does not give a part a value. CE-P2 asks for a value in every part of the result. So the trivial constructor leaves the members of `Pair` without a value, and the variable has no constant initializer under CINIT-1. A scalar gets no initialization, so its zero stays the constant initializer. An empty class has no scalar part, and every compiler accepts it, which fits the constructor call of option 2. CE-24 now states option 2. `constant-initializer-zero-trivial-class.cpp` expects rejection, with overrides for GCC and EDG. The new tests `constant-initializer-trivial-class-block-static.cpp` and `constant-initializer-trivial-class-array.cpp` cover a `static` variable in a block and an array of `Pair`. The new test `constant-initializer-trivial-empty-class.cpp` covers the empty class.

The question stays open under decision 0051 (2026-11-03). GCC 16 and EDG 7.0 accept the `constinit` class with a trivial default constructor, at namespace scope, as a `static` variable in a block, and as an array, and Clang 22 rejects all three, so two families disagree. The review's doubt is whether a call of a trivial constructor, which does nothing, counts as an initialization. `tools/frontend_check.py` reports that the front end passes the four tests.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 2, with doubt. Default initialization of a class always calls a constructor, a trivial one included, and only a type that is neither a class nor an array gets no initialization. The trivial constructor leaves the members without a value, so the variable fails the checks on a constant value. The constructor call is then dynamic initialization, and a `constinit` variable with dynamic initialization is not valid, as Clang finds. A scalar or an array of scalars gets no initialization, so the variable has no dynamic initialization, and the reference accepts it. The doubt is whether a call of a trivial constructor counts as dynamic initialization. GCC and EDG appear to treat the call as no initialization.

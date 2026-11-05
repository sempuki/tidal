# Q-0180: Which dangling references should an implementation warn about?

**Status:** provisional
**Affects:** REF-15, `tests/declarations/initialization/references/dangling-new-member-warns.cpp`

## Context

REF-15 recommends a warning when a reference outlives the temporary that
it binds to. The compilers warn about different cases.

| Program | Clang 18 | Clang 22, host | GCC 14 | GCC 16, host |
|---|---|---|---|---|
| `new Holder{1}`, with a reference member | warns (`-Wdangling-field`) | warns | no warning | no warning |
| `Holder h(1);`, with a reference member | no warning | warns (`-Wdangling`) | no warning | no warning |
| `const int& w = pass(1);` | no warning | no warning | warns under `-Wall` (`-Wdangling-reference`) | no warning |
| `const int& b = Box{2}.get();` | no warning | no warning | warns under `-Wall` | no warning |

The Clang warnings are on by default. GCC 16 no longer warns for the
last two cases, where GCC 14 does.

## Evidence

Probes on the six Clang and GCC compilers.

Under `-Wall -Wextra`, Clang 22 and GCC 16 give these results:

| Case | Clang 22 | GCC 16 |
|---|---|---|
| an aggregate initialized with parentheses, whose reference member binds a temporary (REF-5) | warns (`-Wdangling`) | no warning |
| a reference bound to the returned reference of a call whose argument is a temporary (REF-5) | no warning | warns (`-Wdangling-reference`) |
| a reference member of an object from `new`, bound to a temporary (REF-5) | warns (`-Wdangling-field`) | no warning |
| a constructor that binds a reference member to a temporary (CTOR-17) | rejects | warns (`-Wextra`) |

`-Wdangling-field` is on by default, so `-Werror` alone makes the
warning an error. GCC 14 and GCC 16 give no warning for the `new` case,
also under `-Wall`.

The two tables disagree on GCC 16 for a reference bound to the result
of a call on a temporary. A probe on 2026-11-04 with the host GCC
16.2.1 and `-Wall` gives `-Wdangling-reference` for both
`const int& w = pass(1);` and `const int& b = Box{2}.get();`, where
`pass` takes and returns `const int&` and `get` returns a reference to
a member. The second table matches that probe. The first table's rows
for GCC 16 may come from other forms of `pass` and `Box`, and they
need a new probe.

## Options

1. A recommended rule in general terms, with a test for the case that
   one compiler family warns about in every version (the `new` case).
2. No recommended rule until the compilers agree on a case.

## Provisional answer

A general recommended rule, tested on the `new` case (option 1). A recommended rule allows an implementation that does not
warn, and the `new` case is stable across the Clang versions.

Kept open on 2026-11-03 under decision 0054, because the review finds the reference silent, so it does not support the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) finds the reference silent. It recommends no warning for dangling references. Notes point out that a parenthesized aggregate with a reference member and a `new` expression can leave a dangling reference. The review calls the choice a matter of quality of implementation.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

# Q-0066: Must an implementation diagnose a class member that changes the meaning of a name?

**Status:** settled
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

## Answer

No; the class is not valid, and the error is `no-diagnostic` (option 1). GCC diagnoses the class. Clang and EDG still conform.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and TYN-P4 gives the same answer: a member that changes the meaning of a name after a use makes the class not valid, and an implementation need not notice. The rule needs no diagnostic, so GCC 16's rejection and the acceptance by Clang 22 and EDG 7.0 all conform, and the tests keep their override for GCC.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A search in a class that gives a different result from the same search in the completed class makes the program not valid, with no diagnostic required. A name used before a later member declares the same name is such a case, and GCC's diagnostic is allowed.

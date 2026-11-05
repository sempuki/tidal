# Q-0055: Is a defaulted copy or move constructor with a different parameter type deleted or invalid?

**Status:** settled
**Affects:** CPY-14, `tests/classes/copy-move/defaulted-volatile-param.cpp`, `tests/classes/copy-move/defaulted-move-const-param.cpp`

## Context

A class can default a copy constructor with a `const volatile` reference
parameter, or a move constructor with a `const` rvalue reference
parameter. These types differ from the types of the implicit functions.
GCC 16, Clang 18 and 22, and EDG 7.0 accept the declaration and define the
function as deleted. A use of the function is rejected. GCC 14 rejects the
declaration itself.

## Options

1. The function is deleted. Only a use is rejected.
2. The declaration is not valid.

## Answer

Deleted; only a use is rejected (option 1). The newer GCC agrees with Clang and EDG, so the behavior of
GCC 14 is treated as an older implementation choice.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and CPY-P3 gives the same answer: a function that the class defaults in its definition follows the deletion rule also when its signature differs from the implicit one, so the problem appears only at a use. CPY-P3 states the general model here, and CPY-14 states the case. No newest family differs. GCC 14 rejects the declaration, and the tests keep their overrides for GCC 14.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CPY-P3 passes. CPY-P3 says that a function the class defaults in its definition is deleted when it cannot work, also when its signature differs from the implicit one. So the declaration is valid and only a use is rejected, and option 2 does not fit the model. The question stays settled.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A defaulted special member may differ from the implicit one only in a few listed ways, and a parameter type outside that list makes a function defaulted on its first declaration deleted. A copy constructor that takes a reference to `const volatile` class, and a move constructor that takes a `const` rvalue reference, are still copy and move constructors.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) names the question as settled and gives no new reading. The question stays settled.

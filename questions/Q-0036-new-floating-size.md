# Q-0036: May the size in an array `new` have floating-point type?

**Status:** settled
**Affects:** NEW-2, `tests/expressions/new-delete/array-non-integral-size.cpp`, `tests/expressions/new-delete/array-floating-size-exact.cpp`

## Context

`new int[1.5]` is accepted by Clang 22 and 18, with a `-Wliteral-conversion` warning, and by EDG 7.0. Both convert the size to 1. GCC 16 and 14 reject it ("must have integral or enumeration type").

## Options

1. Accepted: the size converts to the type of `sizeof`, as any value converts.
2. Rejected: only integer and enumeration sizes are allowed.

## Answer

Yes; converted to the type of `sizeof` (option 1). Two of three implementations accept the floating-point size.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. NEW-P1 says that an array can have a size known only at run time, and it does not say which types the size can have. GCC 16 rejects the floating-point size.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). NEW-P1 failed step 3, because it did not say which types the size can have. NEW-P1 now says that the size converts to the type of `sizeof` by an implicit conversion, so a floating-point value gives a size, which rules out option 2. The existing tests check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses NEW-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied NEW-P1 to the question's program, `new int[1.5]` (`array-floating-size-exact.cpp`, `array-non-integral-size.cpp`), without the rule text: the size converts to the type of `sizeof` by an implicit conversion, so a floating size is accepted. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and NEW-P1 gives the same answer under the test of the style guide (section 3). GCC 16 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The first array size in a `new` expression converts implicitly to the type of `sizeof`. A floating-point value converts by the ordinary floating-integral conversion, so the form is valid. An older rule limited the size to integral and enumeration types, and GCC still applies that rule.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The reference converts the array size to the size type by an implicit conversion, so a floating value and a class with a suitable conversion function both work, and a scoped enumeration does not.

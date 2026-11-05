# Q-0353: Does EDG losing the member that a converted member pointer names during constant evaluation change MPTR-4?

**Status:** settled
**Affects:** MPTR-3, MPTR-4, `tests/programs/W9/memptr.cpp`, `tests/programs/W9/mixin.cpp`

## Context

MPTR-4 converts a pointer to a member of a base `B` to a pointer to a
member of a derived class `D`. The converted pointer names the same
member. During constant evaluation EDG 7.0 loses the member in two
cases:

| Case | Clang 18, Clang 22, GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|
| A pointer to a member function of `B`, read from a `constexpr` array, converted to a pointer to member of `D`, and called on a `D` | calls the function | the call does not change the object |
| A pointer to a data member of `B`, converted to a pointer to member of `D` by the `return` of a `constexpr` function, and used to assign to a `D` | assigns the member | the member keeps its value |

EDG gives the expected result when the pointer comes from a `constexpr`
variable, or when the conversion and the use are in one function.
Clang and GCC give the same results at run time.

## Evidence

`tests/programs/W9/memptr.cpp` and `tests/programs/W9/mixin.cpp`, the
W9 Write programs, with overrides for EDG. Probes by the clean-room
writer on the matrix containers gave the table above.

## Options

1. A converted member pointer names the same member wherever its value
   goes (MPTR-4), and constant evaluation follows run time (CE-P1).
   EDG's result is a defect.
2. A converted member pointer that passes through an object or a
   function result has no reliable value during constant evaluation.

## Answer

No; the converted pointer names the member, and EDG differs (option 1).
MPTR-4 states no condition on where the pointer's value comes from, and
Clang and GCC agree at run time and in constant evaluation.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. MPTR-P3 allows the conversion and does not say that the value survives a pass through an object or a function result. CE-P1, on the constant evaluation page, gives the answer. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). MPTR-P3 failed step 3, because MPTR-P3 did not say that a converted value survives a pass through an object or a function result. CE-P1 gives the model, and the member pointer page did not cite CE-P1. Item PQ widened MPTR-P3 to state the case and to cite CE-P1. The new test `tests/expressions/member-pointers/converted-through-object-constant.cpp` checks the case under MPTR-4 in a rule test, with an override for EDG 7.0, because only program tests checked it before. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses MPTR-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied MPTR-P3 to the question's program, a converted pointer to member that passes through an array or a function result in constant evaluation (`memptr.cpp`, `mixin.cpp`), without the rule text: the converted value names the same member wherever it goes. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and MPTR-P3 gives the same answer under the test of the style guide (section 3). EDG 7.0 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Converting a pointer to a member of a base to a pointer to a member of a derived class keeps the member that the pointer refers to. A `constexpr` function gives the same result as an equivalent ordinary function, apart from copy elision, and nothing lets the value change when it passes through an object or a return. EDG's results are a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The reference has no rule that changes a converted member pointer that passes through an object or a function result.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

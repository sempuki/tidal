# Q-0048: If `std::tuple_size<E>` is complete but has no `value`, do structured bindings use the data members?

**Status:** provisional
**Affects:** SB-5, `tests/declarations/structured-bindings/tuple-size-without-value.cpp`

## Context

Clang and GCC fall back to binding the data members of `E` when `std::tuple_size<E>` is a complete type without a `value` member. EDG 7.0 rejects the declaration.

## Options

1. Fall back to the data members (SB-3).
2. Reject the declaration.

## Provisional answer

Yes; the bindings fall back to the data members (SB-3) (option 1). Clang and GCC, in both versions, fall back.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. SB-P2 gives the order of the tests, and it does not say what happens when `std::tuple_size<E>` is complete and has no `value` member. EDG 7.0 rejects the declaration.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). SB-P2 failed step 3, because test 2 of its table said only "a type with a tuple size". Test 2 now needs a complete `std::tuple_size` with a `value` member, and SB-P2 says that a class whose `std::tuple_size` has no `value` member goes on to test 3, the data members. The review reads with no doubt, and EDG 7.0 is the one newest family that differs. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses SB-P2.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied SB-P2 to the program of `tuple-size-without-value.cpp` without the rule text and reached the provisional answer: the trait class is complete and has no `value` member, so test 3 applies and the names bind to the data members.

Kept open on 2026-11-04 under decisions 0054 and 0055, because SB-5, the rule the question affects, changed in this commit: only a member function template named `get` selects the member form. The question waits for the next use of the changed text.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The tuple protocol applies only when `std::tuple_size<E>` is complete and has a member named `value`. Without that member, the bindings use the data members.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The reference uses the tuple protocol only when the trait class is complete and has a member named `value`.

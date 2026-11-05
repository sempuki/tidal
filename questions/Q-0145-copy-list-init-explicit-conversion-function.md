# Q-0145: Can copy-list-initialization use an explicit conversion function for the copy constructor's parameter?

**Status:** settled
**Affects:** OVL-40, `tests/overloading/overload-resolution/init-copy-list-explicit-conversion.cpp`

## Context

`Source` has an `explicit` conversion function to `Target`. `Target` has
a default constructor and the implicit copy and move constructors. The
program writes `Target t = {s};`.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | accepts |
| GCC 14, GCC 16 | rejects |
| EDG 7.0 | rejects |

In direct-initialization, `Target t(s);` and `Target t{s};`, every
compiler uses the explicit conversion function (initialization grid).

## Evidence

`tests/overloading/overload-resolution/init-copy-list-explicit-conversion.cpp`,
and the row "copy list init" of the initialization grid. Item CI-4 of
the overload resolution inventory names direct-initialization only.

## Options

1. Copy-list-initialization takes only conversion functions that are
   not `explicit` (GCC, EDG).
2. The explicit conversion function can make the temporary (Clang).

## Answer

No; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and the inventory names
direct-initialization only.

The question stays open under decision 0051 (2026-11-03). Clang 22 accepts, and GCC 16 and EDG 7.0 reject, so two families disagree. The provisional answer follows ICS-45, which gives `explicit` conversion functions to direct initialization only. Copy-list-initialization is a form of copy initialization, and every compiler keeps `explicit` conversion functions out of copy initialization without braces (`init-copy-explicit-conversion.cpp`).

Kept open on 2026-11-03 under decision 0054, because no principle gives the answer. OVL-P2 and ICS-P1 do not say which initializations can use an `explicit` conversion function, and only ICS-45 and OVL-40 do.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). OVL-P2 and ICS-P1 failed step 3 of the test, because neither said which initializations can use an `explicit` conversion function. CVF-P2 says so, and the overloading page did not cite CVF-P2. Item PQ widened OVL-P2. OVL-P2 now cites CVF-P2 and says that copy-list-initialization, a form of copy initialization, cannot use the function to make the temporary for a copy or move constructor. OVL-40 and `init-copy-list-explicit-conversion.cpp` check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses OVL-P2. Clang 22 is the one newest family that differs.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies OVL-P2 to the program of `init-copy-list-explicit-conversion.cpp` without the rule text and reaches the provisional answer: copy-list-initialization is a form of copy initialization, so the `explicit` conversion function cannot make the temporary for the copy constructor's parameter, no constructor is viable, and the program is rejected.

Settled on 2026-11-04 under decision 0054. The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) supports this answer with no doubt, and OVL-P2 gives the same answer by the test of the style guide (section 3). OVL-P2 did not change in the commit that recorded the use. Clang 22 still differs, and the test keeps its override for it.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. The braced list chooses a constructor of the target, and the copy and move constructors take a reference parameter that is copy-initialized from the source. `explicit` conversion functions take part only in a direct initialization of a reference, or in making the temporary for a constructor's reference parameter during a direct initialization of the class. Copy-list-initialization is neither case.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. An `explicit` conversion function can serve there only in a direct initialization of the class.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

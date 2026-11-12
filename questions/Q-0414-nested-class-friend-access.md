# Q-0414: Does a friend function that a nested class declares have access to the private members of the enclosing class?

**Status:** provisional
**Affects:** ACC-10, ACC-5, `tests/classes/access/nested-class-friend-reads-enclosing.cpp`
**Source:** roadmap round 8 programmer reduction `nested_friend_reads_enclosing.cpp` (`reports/clean-room/roadmap-8/programmer.md`, section 1.5), from the probe that a clean-room writer ran for it.
**Waits for:** a review (doubt)

## Context

Class `Outer` has a private member and a nested class `Inner`. `Inner` defines a friend function `peek` that reads the private member of an `Outer` object. A nested class has the access of a member of the enclosing class (ACC-10), and a friend has the access of the class that befriends it (ACC-5).

| Implementation | Result |
|---|---|
| Clang 22 | accepts |
| GCC 16 | accepts |
| EDG 7.0 | rejects the access |

## Evidence

The test in **Affects**, with an override for EDG.

## Options

1. The friend has the nested class's access, and the program is valid (Clang, GCC).
2. The friend has no access to the enclosing class (EDG).

## Provisional answer

The friend has the nested class's access (option 1). Two of three families agree, and ACC-5 with ACC-10 gives that answer.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 10 wrote, item RV4).

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-26), and EDG 7.0 alone differs. The doubt is whether the friend's body, which lies inside the nested class's definition, counts as lying in a direct member of the enclosing class.

## Opinions

### Gap review, 2026-11-05 (phase 10 text, classes, declarations, foundations, overloading, and statements)

The source-reader gap review of the text phase 10 wrote on classes, declarations, foundations, overloading, and statements (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) reads the question: supports with a doubt. A private member is reachable from code inside a direct member of the class, and the friend's body is written inside the nested class, which is a member. The reference also says friendship does not pass along, and it does not say whether a friend defined inside a nested class counts as code inside that member.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-26). A private member is accessible at a point that lies in a direct member or a friend of its class. A nested class is a member of the enclosing class and has the access that any member has, and each member of a class gets every access that its class has. The friend function is a member of neither class and a friend only of the nested class, and friendship does not pass along. The doubt is now one phrase. The test holds only if the body of the friend, which lies within the nested class's definition, counts as lying in a direct member of the enclosing class. The reference does not say whether the phrase means the place in the text or the entity.

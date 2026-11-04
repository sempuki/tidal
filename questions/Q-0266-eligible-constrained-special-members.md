# Q-0266: Do the constraints of special member functions decide which ones make a class trivial?

**Status:** provisional
**Affects:** CNS-21, `tests/templates/constraints/eligible-default.cpp`, `tests/templates/constraints/eligible-copy.cpp`, `tests/templates/constraints/eligible-more-constrained.cpp`, `tests/templates/constraints/eligible-unsatisfied-more-constrained.cpp`

## Context

A class template declares a defaulted constructor and a
user-provided constructor of the same kind, with different
constraints. For the arguments used, only the defaulted one is
satisfied, or the defaulted one is more constrained. A union with a
member of that class needs the member's constructor to be trivial.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts: the constructor is trivial |
| GCC 14, 16, host | accepts: the constructor is trivial |
| EDG 7.0 | rejects: the union's constructor is deleted |

## Evidence

The four tests above. In `eligible-unsatisfied-more-constrained.cpp` the user-provided constructor is more constrained but not satisfied, and EDG again treats the default constructor as not trivial. With destructors, every compiler chooses the trivial one (`destructor-trivial.cpp`).

No test shows the kinds of the third condition of CNS-21 apart from another split. A class with a defaulted copy constructor and a user-provided one with another first parameter type, or two copy assignment operators with different ref-qualifiers, makes a union member that Clang copies and GCC and EDG do not. The split appears without constraints too.

## Options

1. Only eligible special members count (Clang, GCC). The others can
   never be called for those arguments.
2. Every declared special member counts (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and EDG already selects destructors by their constraints.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 treat the defaulted constructor as the trivial one, and EDG 7.0 deletes the union's constructor, so two families disagree. CNS-P3 supports the provisional answer. A special member function whose constraints fail is not eligible (CNS-21), and EDG 7.0 already chooses destructors by their constraints (CNS-19). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1 for the copy constructor and has no clear reading for the default constructor. CNS-P3 keeps option 1 for both, because a special member whose constraints fail is not eligible.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1 for the copy constructor and gives no clear reading for the default constructor. A special member is eligible when it is not deleted, its constraints hold, and no rival of the same kind is more constrained. For the copy constructor of a union, the reference looks at the constructor that overload resolution selects for each variant member, so the more constrained trivial one counts. For the default constructor of a union, the text asks only whether a member class has a default constructor that is not trivial, with no mention of eligibility. Read literally, that text gives EDG's result. The intent of eligibility points to option 1. The review notes that a later language version reworks the default constructor of a union, outside the reference.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

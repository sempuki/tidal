# Q-0395: Does a member `get` that is not a function template select the member form of the tuple protocol?

**Status:** settled
**Affects:** SB-5, `tests/declarations/structured-bindings/tuple-member-variable-template-get-uses-free.cpp`, `tests/declarations/structured-bindings/tuple-member-class-template-get-uses-free.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md, the SB-5 row

## Context

SB-5 uses the member form `e.get<i>()` when member lookup of `get`
finds a template whose head starts with a value. The gap review of the
text phase 8 wrote reads the C++23 text as counting only a member
function template. A class `Pair` with the tuple protocol has a member
variable template or a member class template named `get`, with a head
`template <std::size_t I>`, and its namespace has a free function
template `get`.

| Member `get` | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| a static member variable template | free `get`, accepts | member form, rejects: "is not a function template" | free `get`, accepts |
| a member class template | free `get`, accepts | member form, rejects: "is not a function template" | free `get`, accepts |

## Evidence

The two tests in **Affects**. Each expects acceptance and records GCC's
rejection as an override.

## Options

1. Only a member function template selects the member form. Otherwise
   the binding calls the free `get` (Clang, EDG, and the review's
   reading).
2. Any member template whose head starts with a value selects the
   member form (GCC).

## Answer

Only a member function template selects the member form (option 1).
Clang 22 and EDG 7.0 agree, and the gap review reads the C++23 text the
same way.

Kept open on 2026-11-04 under decision 0054, because SB-5 changed in this commit (decision 0055), and GCC 16 differs from the answer.

Kept open on 2026-11-05 under decision 0054, because the use of SB-P2 did not give the provisional answer, so the principle needs a repair and a new use (gap review of the text phase 9 wrote, item RV3).

Used on 2026-11-05 in batch PU3 (`reports/clean-room/PU3/report.md`). The reader applied SB-P2 to the question's two programs and rejected both, because SB-P2 said "a member `get` template" and a nested class template and a variable template are member templates (finding PU3-1). SB-P2 now names a member function template and says that the other two do not count.

Kept open on 2026-11-05 under decisions 0054 and 0055, because SB-P2 changed in this commit after finding PU3-1. The question waits for the next use of SB-P2.

Settled on 2026-11-05 under decision 0054. The gap review of the text phase 10 wrote (item RV4) supports the answer with no doubt, and its use of the principle on the question's programs gives the provisional answer (decision 0056). At most one newest family differs in each form, and each test keeps its override.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) supports option 1. In its reading, the member form needs a lookup in the class that finds a function template whose first template parameter takes a value. A member class template or variable template of the same name does not count. SB-5 said "member template", which the review found too wide.

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: supports with no doubt. The reference chooses the member form only when the search for get in the class finds a member function template that takes a value as its leading parameter. A member variable template or class template does not count. used SB-P2 on ``declarations/structured-bindings/tuple-member-variable-template-get-uses-free.cpp`, `tuple-member-class-template-get-uses-free.cpp``, without the rule text. The model's answer: The principle says the elements come from get, and does not say when the member form is chosen. No answer. Matches the provisional answer: no (not decided). Reference verdict: supports the provisional answer with no doubt.

### Gap review, 2026-11-05 (phase 10 text, classes, declarations, foundations, overloading, and statements)

The source-reader gap review of the text phase 10 wrote on classes, declarations, foundations, overloading, and statements (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) used SB-P2 on ``tuple-member-variable-template-get-uses-free.cpp``, without the rule text. The model's answer: The class has the tuple protocol. The member named get is a variable template, which does not count, so each element comes from the free get found by argument-dependent lookup. The bindings are 1 and 2, the assertion holds, and the program is accepted. Matches the provisional answer: yes. Reference verdict: supports with no doubt. used SB-P2 on ``tuple-member-class-template-get-uses-free.cpp``, without the rule text. The model's answer: The member named get is a class template, which does not count, so each element comes from the free get. The bindings are 1 and 2, and the program is accepted. Matches the provisional answer: yes. Reference verdict: supports with no doubt.

# Q-0117: Is a union with no active member a valid constant?

**Status:** provisional
**Affects:** CE-21, TARG-19, `tests/foundations/constant-evaluation/result-union-no-active-member.cpp`, `tests/templates/arguments/same-specialization-union-empty.cpp`
**Waits for:** a review (doubt)

## Context

A union's `constexpr` default constructor initializes no member, so no
member is active. A `constexpr` variable of that union type has no
member with a value. CE-21 requires a value in each scalar part of a
constant, and in a union only the active member is a part.

Clang and EDG accept the variable. GCC 14 and 16 reject it and say
that the constructor leaves the variable incompletely initialized.
Every compiler accepts a constant union whose one active member has a
value.

The same split decides whether such a union can be a template argument.
A program uses `Pick<Choice{}>` twice, where `Choice` is a union whose
`constexpr` default constructor activates no member, and compares the
two types.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts; the types are the same |
| GCC 14, 16, host | rejects: `Choice()` is not a constant expression |
| EDG 7.0 | accepts; the types are the same |

All compilers accept unions with an active member as arguments and
compare them by the active member and its value
(`same-specialization-kinds.cpp`). Q-0202 asked the template argument
case and is a duplicate of this question.

## Evidence

`tests/RESULTS.md`, rows `result-union-no-active-member.cpp` and
`result-union-one-active-member.cpp`, and
`tests/templates/arguments/same-specialization-union-empty.cpp`, with an
override for GCC.

## Options

1. A union with no active member has no parts that need a value, so such
   a union is a valid constant and a valid template argument. Two such
   values of one union type are the same argument by the table of
   TARG-P4. GCC differs.
2. A constant union must have an active member with a value. Clang and
   EDG differ.

## Provisional answer

Yes, and also a valid template argument (CE-21, TARG-19); GCC rejects it (option 1). Two of three implementations accept the variable, and option 1
follows from CE-21 counting only the active member. The status is not
"compiler defect", because option 2 is a consistent reading too.

The question stays open under decision 0051 (2026-11-03). Clang 22 and EDG 7.0 accept the union with no active member, as a constant and as a template argument, and GCC 16 rejects it, so two families disagree. CE-P2 supports the provisional answer. A result must have a value in every part, and in a union only the active member is a part (CE-21). The front end's interpreter accepts both tests.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1, with doubt.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1, with doubt. A union with no active member has no member whose lifetime has begun, so the checks on the value of a constant find no fault. The reference also counts two unions that both have no active member as equivalent template arguments, which only makes sense if such a union can be a constant. The doubt is that the check on subobjects does not say plainly that an inactive member is skipped. A reader who counts the inactive member as a subobject with no value would reach GCC's result.

# Q-0449: Can constant evaluation copy a union whose active member is `mutable` from a union that the evaluation did not create?

**Status:** provisional
**Affects:** CE-22, `tests/foundations/constant-evaluation/union-copy-mutable-member-outside.cpp`, `tests/foundations/constant-evaluation/union-copy-mutable-member-assignment-outside.cpp`, `tests/foundations/constant-evaluation/union-copy-mutable-member-inside.cpp`
**Source:** point RV9-10 of the gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`), with the writer probe `reports/clean-room/roadmap-13/writer-probes/rv9-points.json`.
**Waits for:** newest families that agree; a principle (CE-P3)

## Context

A union has a `mutable` member. A `constexpr` variable of the union
type makes that member active, and a constant evaluation copies the
variable with the union's implicit copy function:

```cpp
union Cell { mutable int count; int fixed; };
constexpr Cell shared{1};
constexpr Cell copy = shared;
```

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| the copy constructor, from `shared` | rejects: read of a `mutable` member | rejects: refers to `mutable` subobjects | accepts |
| the copy assignment into a local union, from `shared` | rejects: read of a `mutable` member | accepts | accepts |
| the copy constructor and the copy assignment, from a local union of the same evaluation | accepts | accepts | accepts |
| the copy constructor, from a `constexpr` union whose active member is the member that is not `mutable` | rejects | rejects | accepts |
| the copy constructor of a struct with a `mutable` member, from a `constexpr` variable | rejects | rejects | accepts |
| a read of the `mutable` member of `shared` | rejects | rejects | accepts |

CE-22 says that a trivial copy of a union copies its active member and
reads none of its parts (Q-0448). No rule of the constant-evaluation
page names a `mutable` member.

## Evidence

The tests in **Affects** and the probe in **Source**.

## Options

1. The copy, by the constructor or by the assignment, is not valid when
   the source's active member is `mutable` and the evaluation did not
   create the source. A `mutable` member that is not active does not
   count (Clang for each form with the active member).
2. The copy is not valid when the union has any `mutable` member and
   the evaluation did not create the source (Clang and GCC for the
   constructor).
3. The copy is valid (EDG).

## Provisional answer

The copy is not valid when the source's active member is `mutable` and
the evaluation did not create the source, for the constructor and for
the assignment (option 1). CE-22 says so, as the RV9-10 reading of the
review gives. Clang agrees in both forms. GCC accepts the assignment,
and EDG accepts both, so two newest families stand against the
assignment form. Clang and GCC also reject the copy when the `mutable`
member is not active. CE-22 sets no limit there, and no test holds
that form.

Opened on 2026-11-11 (decision 0064, item RV9). No principle gives the answer. CE-P3 says that a trivial copy or move of a union reads no part, and it says nothing of a `mutable` member. A reader of CE-P3 alone reaches option 3, so CE-P3 fails step 3 of the principle test.

## Opinions

### Gap review, 2026-11-11 (phase 16 text)

The source-reader gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`) gives the case (RV9-10). Besides the copy of the active member that CE-22 states, the review reads one more limit on a union's implicit copy or move constructor or assignment in constant evaluation: the call is not constant when the union's active member is `mutable`, unless the union's lifetime began within the evaluation. No rule of the page names the case.

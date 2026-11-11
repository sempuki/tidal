# Q-0449: Can constant evaluation copy a union whose active member is `mutable` from a union that the evaluation did not create?

**Status:** provisional
**Affects:** CE-22, CE-3, `tests/foundations/constant-evaluation/union-copy-mutable-member-outside.cpp`, `tests/foundations/constant-evaluation/union-copy-mutable-member-assignment-outside.cpp`, `tests/foundations/constant-evaluation/union-copy-mutable-member-inside.cpp`, `tests/foundations/constant-evaluation/union-copy-inactive-mutable-member-outside.cpp`, `tests/foundations/constant-evaluation/struct-copy-mutable-member-outside.cpp`, `tests/foundations/constant-evaluation/mutable-member-read-outside.cpp`
**Source:** point RV9-10 of the gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`), with the writer probe `reports/clean-room/roadmap-13/writer-probes/rv9-points.json`.
**Waits for:** a review (doubt); newest families that agree; a principle (CE-P3)

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

Kept open on 2026-11-11 under decision 0054 (decision 0064, item RV9). The read of the RV9 repair diff gives the first reading and supports the answer with a doubt (RV10-R-4). For the copy assignment, the reference does not say whether the union whose lifetime must begin in the evaluation is the source or the target. CE-22 reads the source, and GCC's acceptance fits the target. GCC 16 and EDG 7.0 still differ in the assignment form, and CE-P3 still fails step 3 of the principle test. The read also finds the copy valid when the `mutable` member is not active. Clang and GCC read against in that form, and CE-22 sets no limit there, as the sentence above says.

Rule text on 2026-11-11 (decision 0065, item F7): tests hold the three forms of the table that no test held. CE-3 says that a `mutable` member of a usable variable is not usable, as the read of the RV9 repair diff gives. `mutable-member-read-outside.cpp` and `struct-copy-mutable-member-outside.cpp` expect the rejection, with EDG as an override. CE-22 says that a `mutable` member that is not active sets no limit, and `union-copy-inactive-mutable-member-outside.cpp` expects acceptance, with Clang and GCC as overrides. Two newest families stand against that form, as against the assignment form. The answer and the **Waits for:** line do not change.

## Opinions

### Gap review, 2026-11-11 (phase 16 text)

The source-reader gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`) gives the case (RV9-10). Besides the copy of the active member that CE-22 states, the review reads one more limit on a union's implicit copy or move constructor or assignment in constant evaluation: the call is not constant when the union's active member is `mutable`, unless the union's lifetime began within the evaluation. No rule of the page names the case.

### Read of the RV9 repair diff, 2026-11-11

The source-reader read of the RV9 repair diff (`reports/review/2026-11-11-read-of-rv9-repair-diff.md`) gives the first reading of the question: supports with a doubt (option 1). The limit concerns the union's active member, and only when that member is `mutable`. So option 2, which counts any `mutable` member, reads against. The limit does not apply when the union's lifetime began in the same evaluation. For the copy constructor, the limit can only concern the source, because the target always begins in the evaluation. The doubt is RV10-R-4. For the copy assignment, the reference names neither the source nor the target. CE-22 and `union-copy-mutable-member-assignment-outside.cpp` read the source. The limit keeps a copy from reading a part of a constant that constant evaluation cannot read, and a reading of the source fits that purpose. GCC's acceptance of the assignment fits a reading of the target, and the local target began in the evaluation. The read finds that the CE-22 sentence, the Q-0449 line, and the constructor row agree, and that the assignment row agrees with the doubt. The gap review's entry above states RV9-10 correctly. The read also gives the forms of the table that no test holds:

- A copy from a `constexpr` union whose active member is not `mutable` is valid, because constant evaluation can read the active member. Clang and GCC read against, and EDG agrees.
- The implicit copy of a struct reads each member, and a `mutable` part of a constant variable cannot be used in a constant expression. So the copy of a struct with a `mutable` member from a `constexpr` variable is not constant. Clang and GCC agree, and EDG reads against.
- A read of the `mutable` member of `shared` follows the same rule. Clang and GCC agree, and EDG reads against.

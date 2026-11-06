# Q-0392: Can a member function of a local class in a function template have a requires clause?

**Status:** settled
**Affects:** INST-12, INST-P1, `tests/templates/instantiation/local-class-requires-clause-waits.cpp`, `tests/templates/instantiation/local-class-requires-clause-checked-at-use.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md, the INST-P1 row

## Context

INST-12 instantiates a local class of a function template with the
function, member function bodies and default arguments included. The
gap review of the text phase 8 wrote on templates reads the
constraints of the local class as an exception: they are not
instantiated with the class. Probes of a member function with a
requires clause that names the function template's parameter:

```cpp
template <class T> int make() {
  struct Local {
    int get() requires (T::value > 0) { return 1; }
    int other() { return 2; }
  };
  return Local().other();
}
```

| Case | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| The requires clause cannot be checked for `int`, and nothing calls `get` | accepts and runs | accepts and runs | rejects the requires clause |
| The requires clause is false for `int`, and the function calls `get` | rejects the call | rejects the call | rejects the requires clause |

EDG says that a requires clause is not allowed there, because the
function is not templated.

## Evidence

The two tests in **Affects**. The first expects the program to run and
records EDG's rejection as an override.

## Options

1. The member function is templated, so it can have a requires clause,
   and the clause waits for a use of the member (Clang, GCC, and the
   review's reading of the exception).
2. A member of a local class cannot have a requires clause (EDG).

## Answer

Yes; the requires clause is valid and waits for a use of the member
(option 1). Clang and GCC agree, and the review reads the constraints
of a local class as an exception to its eager instantiation.

Kept open on 2026-11-04 under decision 0054, because INST-P1 and INST-12 changed in the commit that opened the question, and no review has read the question.

Clarified on 2026-11-05 after finding G149 of the front end (`reports/clean-room/rust-frontend/report-21.md`). INST-12 now says that a substitution failure in the waiting requires clause makes the constraint not satisfied at the use. Clang 22 and GCC 16 give `false` for a requires expression that names the call, and reject the call. EDG 7.0 rejects the requires clause itself, as in the other tests of this question. The new tests are `tests/templates/instantiation/local-class-requires-clause-substitution-failure.cpp` and `tests/templates/instantiation/local-class-requires-clause-substitution-failure-call.cpp`.

Settled on 2026-11-05 under decision 0054. The gap review of the text phase 9 wrote (item RV3) supports the answer with no doubt. At most one newest family differs in each form, and each test keeps its override.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) reads a local class and its members as instantiated with the enclosing function, with their default arguments, exception specifications, and default member initializers, and reads the type constraints and requires clauses of the local class as left out of that. The review does not discuss EDG's rejection of the requires clause.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the question: supports with no doubt. The reference names a member function of a local class as a case where the requires clause is not instantiated with the function. The member is templated, so the clause is allowed.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

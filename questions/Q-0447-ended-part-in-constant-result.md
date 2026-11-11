# Q-0447: Does a part that an explicit destructor call ended count as a part with no value in the result of a constant expression?

**Status:** provisional
**Affects:** CE-21, `tests/foundations/constant-evaluation/result-ended-element.cpp`, `tests/foundations/constant-evaluation/result-union-ended-element.cpp`
**Source:** X25-7 (`reports/clean-room/X25/report.md`, probe `a10.cpp`), rerun on the matrix in `reports/clean-room/roadmap-13/writer-probes/x25-rules.json`, and by the item F6 writer with `tools/probe_grid.py`.
**Waits for:** a review (doubt); newest families that agree

## Context

A class or a union has an `int` array member. The constructor, or a
`constexpr` function that returns the object, ends one element with a
pseudo-destructor call. A `constexpr` variable then holds the object.

| Form | Clang 18 and 22 | GCC 14 | GCC 16 | EDG 7.0 |
|---|---|---|---|---|
| a struct, the element ended in the constructor | rejects | accepts | rejects | rejects |
| a union, the element ended in the constructor | rejects | accepts | rejects | rejects |
| a struct or a union, the element ended in a function that returns the object | rejects | accepts | rejects | rejects |
| a struct whose element of class type ends by its destructor call | rejects | accepts | rejects | rejects |
| a struct, the `int` element ended and then assigned 3 | rejects | accepts | accepts | accepts |

In the last row, Clang rejects the assignment to the element after its
end. The other compilers read the assignment as a new value. No rule
of the constant-evaluation page names the form, and no test holds it.

## Evidence

The tests in **Affects** and the probes in **Source**.

## Options

1. A part whose lifetime ended has no value, so the constant is not
   valid (Clang, GCC 16, EDG).
2. The part keeps its last value (GCC 14).

## Provisional answer

The ended part has no value, and the constant is not valid (option 1).
CE-21 says so. Every newest compiler agrees, and GCC 14 alone differs.

Kept open on 2026-11-11 under decision 0054 (decision 0064, item RV9). The gap review of the phase 16 text supports the answer with a doubt (RV9-9), and every newest compiler agrees. The review also reads the form of the table's last row: an assignment to the `int` element after its end uses the element outside its lifetime, which is undefined, so the evaluation is not constant. OBJ-2 gives the same reading, as Clang does. GCC 16 and EDG 7.0 accept the form, so two newest families differ in that form.

## Opinions

### Gap review, 2026-11-11 (phase 16 text)

The source-reader gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`) reads the question: supports with a doubt (RV9-9). The value of a constant needs each scalar part to have a value. A pseudo-destructor call in an evaluation ends the scalar's lifetime, and an object's properties hold only during its lifetime, so the ended part has no value that the constant could accept (option 1). The doubt is that the reference reaches the result only through that general statement on lifetimes, and does not say outright what value an ended scalar has. An element of class type that a destructor call ended follows the same reading. The review reads the last row of the table as decided: the assignment after the element's end is a use of the object outside its lifetime, no operation in the row creates a new object, and the undefined use makes the evaluation not constant. Clang's rejection matches, and GCC 16 and EDG read against.

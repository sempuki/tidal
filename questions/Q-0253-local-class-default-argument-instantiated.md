# Q-0253: Is the default argument of a local class's member function instantiated with the enclosing function?

**Status:** settled
**Affects:** INST-12, `tests/templates/instantiation/local-class-default-argument.cpp`

## Context

A function template declares a local class with a member function
`get(int y = T::nothing)`. No call uses the default argument, and the
function is instantiated with `T = int`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects |
| EDG 7.0 | rejects |
| GCC 14, 16, host | accepts |

Every compiler instantiates the default member initializers, the
exception specifications, and the local enumerations of the function.

## Evidence

The test above and the tests `local-class-member-initializer.cpp`,
`local-class-exception-specification.cpp`, and `local-enumeration.cpp`.
Item 29 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says that the default
arguments of a local class are instantiated with the function.

## Options

1. The default argument is instantiated with the function (Clang, EDG).
2. The default argument waits for a call that uses it (GCC).

## Answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the gap review agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. INST-P1 and INST-P2 let default arguments wait for a use and do not name local classes.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). INST-P1 and INST-P2 failed step 3, because they let default arguments wait for a use and did not name local classes. Item PQ widened INST-P1. A local class is part of its function, and the instantiation of the function is the use that needs the local class, so every part of the local class is made with the function, default arguments among them. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses INST-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied INST-P1 to the question's program, a default argument of a member function of a local class in a function template (`local-class-default-argument.cpp`), without the rule text: the local class is instantiated whole with its function, default arguments included, which is the provisional answer.

Kept open on 2026-11-04 under decision 0055, because INST-P1 and INST-12 changed in this commit. The review reads the constraints of a local class as left out of its instantiation, and INST-P1 now says that the requires clause of a member waits (Q-0392). The question waits for the next use of INST-P1. GCC 16 is the one newest family that differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied INST-P1, without the rule text, to the program of `local-class-default-argument.cpp`. The answer names INST-P1 and gives the outcome that the test expects, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and INST-P1 gives the same answer by the test of the style guide (section 3). GCC 16 still differs, and the test keeps its override.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A local class and its members, with their default arguments, exception specifications, and member initializers, are never instantiated on their own, and their checks happen when the enclosing entity is instantiated. The statement is a note, but it describes the effect of the binding rules. GCC's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. A local class and its members are instantiated with the enclosing function, with their default arguments, exception specifications, and default member initializers. The review adds that the type constraints and requires clauses of the local class are left out of that.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

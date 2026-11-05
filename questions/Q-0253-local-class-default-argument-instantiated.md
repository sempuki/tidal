# Q-0253: Is the default argument of a local class's member function instantiated with the enclosing function?

**Status:** provisional
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

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the gap review agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. INST-P1 and INST-P2 let default arguments wait for a use and do not name local classes.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). INST-P1 and INST-P2 failed step 3, because they let default arguments wait for a use and did not name local classes. Item PQ widened INST-P1. A local class is part of its function, and the instantiation of the function is the use that needs the local class, so every part of the local class is made with the function, default arguments among them. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses INST-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A local class and its members, with their default arguments, exception specifications, and member initializers, are never instantiated on their own, and their checks happen when the enclosing entity is instantiated. The statement is a note, but it describes the effect of the binding rules. GCC's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

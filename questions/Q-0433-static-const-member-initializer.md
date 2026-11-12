# Q-0433: Is the initializer in the class of a `static const` member made with the class?

**Status:** provisional
**Affects:** INST-5, INST-33, INST-P2, `tests/templates/instantiation/class-static-const-member-initializer-made.cpp`, `tests/templates/instantiation/class-point-static-const-member.cpp`
**Source:** the I6 report (`reports/clean-room/I6/report.md`, I6-15), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/i6-6-15-clean.json`.
**Waits for:** a review (doubt)

## Context

A class template declares a `static const` data member of integer or
enumeration type, with an initializer in the class body, and the member
is neither `inline` nor `constexpr`. The initializer is not valid for
some arguments. A use of the class that does not name the member, such
as an object or `sizeof`, instantiates the class.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `static const int`, `long`, `bool`, or an enumeration; an object or `sizeof` of the class | rejects | accepts | rejects |
| the same, a pointer to the class only | accepts | accepts | accepts |
| `static inline const int` or `static constexpr int`; an object or `sizeof` | accepts | accepts | rejects |
| `static const int` whose initializer needs a function declared after the class's point of instantiation, read after the declaration | rejects | accepts | rejects |

So Clang and EDG make this initializer with the class, and GCC waits for
a use of the member. For an `inline` or `constexpr` member, Clang and
GCC wait, and EDG makes the initializer with the class
(Q-0231).

## Evidence

The tests in **Affects**, and the grids
`tests/templates/instantiation/grids/static-const-member.json` and
`static-member-point.json`.

## Options

1. The initializer in the class of a `static const` member that is
   neither `inline` nor `constexpr` is made with the class (Clang, EDG).
2. The initializer waits for a use of the member, as for every other
   `static` data member (GCC).

## Provisional answer

The initializer is made with the class (option 1). Clang 22 and EDG 7.0
agree, and GCC alone waits. INST-5 lists the case as an exception, and
the INST-P2 table gives it a row.

Kept open on 2026-11-11 under decision 0054 (item RV7). The review supports the answer with a doubt, and only GCC differs.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-27), and GCC 16 alone differs. The doubt is one sentence on the made member, which GCC can read as delaying the initializer.

## Opinions

### Gap review, 2026-11-11 (phase 14 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 14 wrote on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV7-B-14). The instantiation of the class makes the declarations of its static data members and not their definitions. For a `static const` member of integer or enumeration type that is neither `inline` nor `constexpr`, the declaration in the class is not a definition and carries the initializer. So the initializer comes with the declaration, and the class makes it (option 1). The doubt is a general sentence that delays the initialization of a static data member, with its side effects, until a use needs the definition. GCC can read that sentence as delaying the initializer itself. The review reads it as a statement about run-time effects.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-27). The instantiation of a class makes the declarations of its static data members and not their definitions. For a `static const` member of integral type that is neither `inline` nor `constexpr`, the declaration in the class holds the initializer and is not a definition. So the class makes the initializer, which is option 1. The doubt is now one sentence. A member is made when a use needs its definition, and the initialization of a static data member, with its side effects, waits for such a use. That sentence is tied to the definition, and for this member the initializer sits in the declaration. GCC can still read the sentence as delaying the initializer, and the reference does not rule that reading out.

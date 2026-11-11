# Q-0433: Is the initializer in the class of a `static const` member made with the class?

**Status:** provisional
**Affects:** INST-5, INST-33, INST-P2, `tests/templates/instantiation/class-static-const-member-initializer-made.cpp`, `tests/templates/instantiation/class-point-static-const-member.cpp`
**Source:** the I6 report (`reports/clean-room/I6/report.md`, I6-15), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/i6-6-15-clean.json`.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

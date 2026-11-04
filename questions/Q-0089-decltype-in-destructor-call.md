# Q-0089: Can decltype name the class in an explicit destructor call?

**Status:** settled
**Affects:** DTY-3, `tests/expressions/decltype/destructor-call.cpp`

## Context

A program calls a destructor explicitly and writes the class
name as `decltype` of the object, as in `t.~decltype(t)()`. Clang and EDG
accept the call. GCC rejects it and expects a class name after `~`.

## Options

1. `decltype` can name the class in an explicit destructor call.
2. Only a class name or a type alias can follow `~`.

## Answer

Yes; `decltype` can name the class in an explicit destructor call (option 1). Clang and EDG agree, and DTY-3 lets
`decltype` occur where a type name can occur.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and DTY-P2 gives the same answer: the result of `decltype` is a type, so it can stand wherever a type name can stand. A type alias can follow `~` in a destructor call, so `decltype` can too. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The grammar of an unqualified name has a form with a tilde and a `decltype` specifier, and that form names the destructor of the type that the specifier gives. After `.` or `->`, a tilde can only start a destructor name. GCC's rejection is a defect.

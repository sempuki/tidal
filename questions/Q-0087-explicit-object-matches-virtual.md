# Q-0087: Can an explicit object member function have the name and parameters of a base virtual function?

**Status:** provisional
**Affects:** VIRT-19, `tests/classes/virtual/explicit-object-overrides.cpp`

## Context

A derived class declares a member function with an explicit object parameter. The name and other parameters of that function match a virtual function of a base. Clang and GCC reject the declaration. EDG 7.0 accepts it.

## Options

1. Not valid.
2. Valid, and the function does not override.

## Provisional answer

No; the declaration is not valid (option 1). Clang and GCC agree.

Kept on 2026-11-03. The review reads the C++23 text as option 2, which EDG 7.0 follows, and a later change as option 1, which Clang 22 and GCC 16 follow. Whether Tidal follows such a change is Q-0007, so the answer waits on the maintainer. The question is in rows 4 and 7 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads option 2 in the C++23 text and notes that a later change gives option 1. In the C++23 text a derived function overrides only when the two declarations correspond, and the object parameters of the test differ, so the function does not override, is not virtual, and hides the base function. That is EDG's result. A later change ignores the object parameter when the derived function has an explicit object parameter. The function then overrides, which is not allowed. Clang and GCC follow that change.

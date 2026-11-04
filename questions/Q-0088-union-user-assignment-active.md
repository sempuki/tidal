# Q-0088: Does assignment through a user-provided operator activate an inactive union member during constant evaluation?

**Status:** provisional
**Affects:** UNI-6, `tests/classes/unions/user-assignment-not-activating-constant.cpp`

## Context

A union member has a class type with a user-provided assignment operator. Constant evaluation assigns to that member while another member is active. Clang and GCC reject the evaluation. EDG 7.0 accepts it.

## Options

1. The member does not become active, so the access is undefined and constant evaluation rejects it.
2. The assignment makes the member active.

## Provisional answer

No; the access is undefined, so constant evaluation rejects it (option 1). Clang and GCC agree.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. UNI-P1 says that a plain assignment to another member makes that member active, and does not say whether an assignment through a user-provided operator counts. EDG 7.0 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. Only an assignment through the built-in operator or a trivial assignment operator changes the active member. A user-provided operator is a member call on an object whose lifetime has not begun, which is undefined.

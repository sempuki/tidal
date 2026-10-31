# Q-0088: Does assignment through a user-provided operator activate an inactive union member during constant evaluation?

**Status:** provisional
**Affects:** UNI-6, `tests/classes/unions/user-assignment-not-activating-constant.cpp`

## Context

A union member has a class type with a user-provided assignment operator. Constant evaluation assigns to that member while another member is active. Clang and GCC reject the evaluation. EDG 7.0 accepts it.

## Options

1. The member does not become active, so the access is undefined and constant evaluation rejects it.
2. The assignment makes the member active.

## Provisional answer

Option 1. Clang and GCC agree.

## Opinions

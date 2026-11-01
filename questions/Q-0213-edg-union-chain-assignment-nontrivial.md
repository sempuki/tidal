# Q-0213: Does EDG accepting an assignment into a union member that has a default constructor that is not trivial change UNI-13?

**Status:** compiler defect
**Affects:** UNI-6, UNI-13, `tests/classes/unions/chain-assign-nontrivial-constant.cpp`

## Context

A union has an active `int` member and a member of class `N`. `N` has a
default member initializer, so its default constructor is not trivial.
A constant evaluation assigns to `w.n.x`, a part of the member that is
not active.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: assignment to a member of a union with another active member |
| GCC 14, 16, host | rejects: the access names a member other than the active one |
| EDG 7.0 | accepts |

Every compiler accepts the same assignment when the member's default
constructor is trivial (`tests/classes/unions/chain-assign-activates.cpp`).

## Evidence

`tests/classes/unions/chain-assign-nontrivial-constant.cpp`, with an
override for EDG.

## Options

1. The assignment makes a member active only if the member's default
   constructor is trivial. Otherwise the assignment uses an object that
   does not exist, and the behavior is undefined (UNI-6). EDG's
   acceptance is a defect.
2. The assignment makes every member in the chain active.

## Provisional answer

No; Clang and GCC reject, likely EDG defect (option 1). Clang and GCC agree. A member whose constructor does work
cannot begin its lifetime without that work.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

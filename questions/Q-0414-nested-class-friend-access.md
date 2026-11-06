# Q-0414: Does a friend function that a nested class declares have access to the private members of the enclosing class?

**Status:** provisional
**Affects:** ACC-10, ACC-5, `tests/classes/access/nested-class-friend-reads-enclosing.cpp`
**Source:** roadmap round 8 programmer reduction `nested_friend_reads_enclosing.cpp` (`reports/clean-room/roadmap-8/programmer.md`, section 1.5), from the probe that a clean-room writer ran for it.
**Waits for:** a first review

## Context

Class `Outer` has a private member and a nested class `Inner`. `Inner` defines a friend function `peek` that reads the private member of an `Outer` object. A nested class has the access of a member of the enclosing class (ACC-10), and a friend has the access of the class that befriends it (ACC-5).

| Implementation | Result |
|---|---|
| Clang 22 | accepts |
| GCC 16 | accepts |
| EDG 7.0 | rejects the access |

## Evidence

The test in **Affects**, with an override for EDG.

## Options

1. The friend has the nested class's access, and the program is valid (Clang, GCC).
2. The friend has no access to the enclosing class (EDG).

## Provisional answer

The friend has the nested class's access (option 1). Two of three families agree, and ACC-5 with ACC-10 gives that answer.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

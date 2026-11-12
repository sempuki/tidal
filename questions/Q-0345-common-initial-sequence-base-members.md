# Q-0345: Do the members that a base declares count in the common initial sequence of a struct?

**Status:** provisional
**Affects:** UNI-15, UNI-16, `tests/classes/unions/common-initial-sequence-inherited.cpp`, `tests/classes/members/grids/layout-compatible.json`
**Waits for:** a principle (UNI-P6)

## Context

`struct Base { int id; int size; }; struct Named : Base {};` and
`struct Direct { int id; int size; };`. `Named` declares no members, and
its base declares them all, so `Named` is standard-layout (CLS-19).

UNI-15 takes the members of `Named` to be `id` and `size` of `Base`.
Then the common initial sequence of `Named` and `Direct` holds both
members, and the two structs are layout-compatible (UNI-16).

| Check | Result |
|---|---|
| Builtin query of Clang 22 and GCC (grid row "members from a base") | layout-compatible |
| Run test: a union holds a `Named`, and the program reads `id` and `size` through `Direct` | the values of the `Named` object, on every compiler |
| Clang 18, EDG 7.0 | no such query |

The source-reader gap review of item O
(`reports/review/2026-11-03-gap-review-item-o.md`) says the reference
does not state whether the members of a struct include those that its
base declares. A run test cannot show the answer, because the read
through `Direct` gives the expected value on every compiler either way.

## Evidence

The grid row and the run test in **Affects**.

## Options

1. The members that a base declares count, so `Named` and `Direct`
   have a common initial sequence of two members (Clang 22 and GCC
   queries).
2. Only the members that the struct itself declares count, so the
   common initial sequence of `Named` and `Direct` is empty.

## Provisional answer

Yes; the members that a base declares count (option 1). The builtin queries of Clang 22 and GCC agree, and the
class-properties inventory (item LC-inherited-members) describes the
same reading. A struct whose members all come from one base has the
layout of that base, so the reading changes no address.

Kept open on 2026-11-03 under decision 0054, because the review reads with doubt.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with no doubt, and no newest family differs. No principle gives the answer. UNI-P6 pairs the non-static data members of the two structs in declaration order, and it does not say whether the members of a base count. Each option fits the model, so UNI-P6 fails step 3 of the principle test.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1, with doubt. A derived class has the members of its bases among its own, so the common initial sequence takes the base's members. The doubt is that the sequence is defined in declaration order, and the C++23 text does not say how that order runs across a base and a derived class. When all the members are in one base, the order does not matter.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports, and the doubt is resolved. A standard-layout class has all its data members first declared in one class, either the class itself or one base. So the order across a base and a derived class never arises for the classes that have a common initial sequence, and the members of a base count as members of the derived class.

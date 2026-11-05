# Q-0391: Does a friend template that names a member of a class template befriend only that member, in every specialization?

**Status:** settled
**Affects:** ACC-20, `tests/classes/access/friend-member-of-explicit-specialization.cpp`, `tests/classes/access/friend-member-of-specialization-other-member.cpp`
**Source:** the ACC-20 note of `reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`

## Context

A friend declaration with a template head can name a member of a class
template through a qualifier such as `Box<T>::`:

```cpp
template <class T> struct Box { int peek(); int other(); };
class Safe {
  int secret = 7;
  template <class T> friend int Box<T>::peek();
};
```

The member `peek` of each specialization of `Box` is then a friend of
`Safe`. Every compiler accepts a use of `secret` from `Box<T>::peek`
(`tests/classes/access/friend-member-of-every-specialization.cpp`). The
compilers differ in two forms.

| Form | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| A use from `peek` of an explicit specialization `Box<char>` | accepts | accepts | rejects: `secret` is inaccessible |
| A use from `Box<T>::other`, which is not a friend | accepts | rejects | rejects |

Clang warns at the friend declaration that it does not support a
dependent qualifier there, and that it turns off access checks for
`Safe`. So Clang accepts every use of a private member of `Safe`.

## Evidence

The two tests in **Affects**. The first expects the program to run and
records EDG's rejection. The second expects a rejection and records
Clang's acceptance.

## Options

1. The named member of every specialization, explicit specializations
   included, is a friend, and no other member is (GCC; Clang for the
   first form, EDG for the second).
2. Only the members of specializations made from the template are
   friends, and an explicit specialization's member is not (EDG).

## Answer

The named member of every specialization is a friend, and no other
member is (option 1). The gap review of the text phase 8 wrote reads
the reference this way, explicit specializations included. In each
form, one newest family differs.

Kept open on 2026-11-04 under decision 0054, because the question and
the ACC-20 text are new in this commit and wait for a use.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied ACC-P3, without the rule text, to the programs of `friend-member-of-explicit-specialization.cpp`, `friend-member-of-specialization-other-member.cpp`, and `friend-member-of-every-specialization.cpp`. Each answer names ACC-P3 and gives the outcome that the tests expect. The reader marked the answer for `friend-member-of-explicit-specialization.cpp` doubtful (finding PU1-6), because ACC-P3 does not say whether the member that an explicit specialization declares anew is "that member".

Kept open on 2026-11-04 under decision 0054, because the use in batch PU1 reaches the answer for the explicit specialization form with doubt (finding PU1-6). ACC-P3 does not decide that form, so the use does not pass step 3 of the test of the style guide. In each form one newest family differs.

Kept open on 2026-11-04 under decision 0055, because ACC-P3 changed in this commit after finding PU1-6. ACC-P3 now says that befriending a member of a class template befriends no other member, and that the member which an explicit specialization declares with the same name and type is a friend too. The question waits for the next use of ACC-P3. In each form one newest family differs.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied ACC-P3, without the rule text, to the programs of `friend-member-of-specialization-other-member.cpp`, `friend-member-of-every-specialization.cpp`, and `friend-member-of-explicit-specialization.cpp`. Each answer names ACC-P3 and gives the outcome that the tests expect, with no doubt. The answer for `friend-member-of-explicit-specialization.cpp` takes the sentence of ACC-P3 on the member that an explicit specialization declares with the same name and type. The use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU2 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and ACC-P3 gives the same answer by the test of the style guide (section 3). In each form one newest family still differs (EDG 7.0 for the explicit specialization, Clang 22 for the other member), and each test keeps its override.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) finds that ACC-20 leaves out this form. A friend declaration with a template head can name a member function or a member class of a specialization of a class template, written with a qualifier that ends in a template name with arguments, and the qualifier must let each template parameter be deduced. The matching member of every specialization, explicit specializations included, is then a friend.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

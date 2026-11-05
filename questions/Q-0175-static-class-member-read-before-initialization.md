# Q-0175: Can a dynamic initializer read the zero in a member of a later class-type variable?

**Status:** provisional
**Affects:** SINIT-2, SINIT-3, OBJ-1, OBJ-2

## Context

STO-3 sets the storage of every static variable to zero before other
initialization. SINIT-2 lets a dynamic initializer read an earlier
scalar variable of the same translation unit before that variable's own
initializer runs, and the read gives zero.

A class-type variable is different. OBJ-1 starts the lifetime of a class
object when its constructor completes. Before that, a member holds the
zero in its storage, but the object has not started. The question is
whether a dynamic initializer that reads such a member gets zero or has
undefined behavior.

| Implementation | Read at run time | Same read in constant evaluation |
|---|---|---|
| Clang 22, `-O0` and `-O2` | 0 | rejected, "read of object outside its lifetime" |
| GCC 16, `-O0` and `-O2` | 0 | rejected, "accessing uninitialized member" |
| EDG 7.0 | (no code) | rejected, "access to uninitialized object" |

The address and undefined behavior sanitizers report nothing at run
time. A later aggregate with no constructor, which a dynamic
initializer gives its value, gives the same run-time results.

## Evidence

- `tests/declarations/initialization/static/member-read-before-constructor-constant.cpp`:
  a `constinit` object whose constructor reads a member before the
  member's constructor runs. Every compiler rejects the constant.
- The run-time probe reads `p.x` from an earlier initializer, where `p`
  is a later namespace-scope `Point` with a constructor. Clang and GCC
  give 0. The probe is not a test, because Tidal does not test a run of
  undefined behavior for its result.

Constant evaluation cannot show the scalar case, because a constant
cannot read a variable whose value comes from a dynamic initializer. So
the rejections do not separate the scalar case from the class case.

## Options

1. A member of a class object has no value until the initialization of
   the object gives it one. A read before then is undefined (OBJ-2). The
   zero of STO-3 is visible only through a variable of scalar type.
2. The zero counts as a value for every object with static storage. A
   read of a member before the constructor runs gives 0.

## Provisional answer

No; the read is undefined (OBJ-2) (option 1). The option follows the lifetime model of OBJ-1. Constant
evaluation in every compiler treats the member as having no value. The
run-time results of option 2 lie within what OBJ-2 allows.

Kept open on 2026-11-03 under decision 0054, because the review reads the reference as unclear. OBJ-1 now starts the lifetime of a class object when its initialization completes, which is the model of option 1.

Kept open on 2026-11-04 under decision 0055, because SINIT-P2 changed in this commit: it now says that a read of a member of another unit's class object, before the object's initialization gives the member a value, is undefined, and SINIT-3 cites the question. The question waits for the next use of SINIT-P2.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied SINIT-P2, without the rule text, to the programs of `member-read-before-constructor-constant.cpp` and `across-units-class-member.cpp`. Each answer names SINIT-P2 and gives the outcome that the tests expect. `constinit` rejects a constructor that reads a member with no value, and the program of two units runs. The use supports the answer.

Kept open on 2026-11-04 under decision 0054, because the latest gap review (2026-11-03) reads the reference as unclear. The question waits for a review that reads option 1 with no doubt.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as unclear. The rule on lifetimes supports option 1. A class object with a dynamic initializer has not begun its lifetime before the initializer completes, and a note to that rule names this case. A read of a member then is outside the lifetime. A note on static initialization says that an earlier initializer can see the zero of a later object, and that note fits option 2. Read strictly, the rule on lifetimes also makes the scalar read of SINIT-2 undefined, so the review finds the reference inconsistent. The review knows of an open core issue on the point after C++23.

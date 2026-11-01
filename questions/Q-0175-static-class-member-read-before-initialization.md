# Q-0175: Can a dynamic initializer read the zero in a member of a later class-type variable?

**Status:** provisional
**Affects:** SINIT-2, OBJ-1, OBJ-2

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

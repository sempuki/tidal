# Q-0115: Does GCC accepting a copy of an object with an unset part during constant evaluation change CE-22?

**Status:** compiler defect
**Affects:** CE-22, CE-21, CE-32, `tests/expressions/constant-expressions/copy-unset-member.cpp`, `tests/expressions/constant-expressions/copy-unset-array-element.cpp`, `tests/expressions/constant-expressions/result-returned-local-unset-element.cpp`, `tests/expressions/constant-expressions/read-returned-local-unset-element.cpp`

## Context

A class has a member with no initializer, and the constructor of the
class does not set the member. A `constexpr` function makes an object of
the class, copies the object, and returns a member that has a value.
Clang and EDG reject the evaluation and say that the copy reads the
member with no value. GCC 14 and 16 accept the evaluation. The same
holds for an array member with elements that have no value (W0 F2).

Every compiler rejects a read of the unset member itself (OBJ-2) and a
constant whose constructor leaves a part unset (CE-21).

A builder function that returns a local with an unset array element
shows the same split (Write report W11). Clang and EDG reject the
`constexpr` variable that the result initializes. GCC 14 and 16 accept
the variable, and a `static_assert` that reads the unset element as 0
passes on GCC.

## Evidence

`tests/RESULTS.md`, rows `copy-unset-member.cpp`,
`copy-unset-array-element.cpp`, `result-returned-local-unset-element.cpp`,
and `read-returned-local-unset-element.cpp`. Clang notes that the subobject is not
initialized in the call to the copy constructor. EDG notes an access to
an uninitialized object.

## Options

1. A copy reads each scalar part (CPY-8), so copying an unset part is a
   read of a value that does not exist and is undefined (OBJ-2).
   Constant evaluation rejects the copy. GCC's acceptance is a likely
   defect.
2. A copy of an unset part is not a read and gives an unset part in the
   copy. Constant evaluation accepts the copy, and only a later read is
   rejected.

## Provisional answer

No; likely GCC defect (option 1). Two of the three implementations reject the copy, and Tidal's
rules already describe an implicit copy as a copy of each part. Option 2
would need a new kind of value that a copy can carry without a read.

## Opinions

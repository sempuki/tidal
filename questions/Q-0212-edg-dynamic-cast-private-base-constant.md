# Q-0212: Does EDG letting a `dynamic_cast` from a private base succeed during constant evaluation change VIRT-10?

**Status:** compiler defect
**Affects:** VIRT-10, `tests/classes/virtual/dynamic-cast-private-base-constant.cpp`

## Context

A class `D` has a private base `A` and a public base `B`. A member
function of `D` returns a pointer to its `A` subobject. A `dynamic_cast`
of that pointer to `D*`, or to `B*`, follows the check of VIRT-10.

| Compiler | Run time | Constant evaluation |
|---|---|---|
| Clang 18, 22, host | null pointer | null pointer |
| GCC 14, 16, host | null pointer | null pointer |
| EDG 7.0 | no code generation | the cast succeeds |

EDG gives a null pointer in one probed case. In that case `D` has a
public base `A` and a private base `B`, and the cast goes from the `B`
subobject to `D*`.

## Evidence

`tests/classes/virtual/dynamic-cast-private-base-constant.cpp`, with an
override for EDG. The run tests `dynamic-cast-private-base.cpp`,
`dynamic-cast-cross-from-private-base.cpp`, and
`dynamic-cast-cross-to-private-base.cpp` give a null pointer on every
Clang and GCC configuration.

## Options

1. The check needs the operand's subobject to be a public base of the
   target object or of the most derived object (VIRT-10). EDG's success
   is a defect.
2. Constant evaluation ignores access in the check.

## Provisional answer

Option 1. Clang and GCC agree at run time and in constant evaluation,
and the result of a constant must match the result at run time.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

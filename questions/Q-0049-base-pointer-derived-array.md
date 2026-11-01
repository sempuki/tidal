# Q-0049: Is arithmetic on a base class pointer into an array of a derived class undefined?

**Status:** provisional
**Affects:** ADD-14, ADD-P6, `tests/expressions/additive/base-pointer-derived-array.cpp`, `tests/expressions/additive/subtract-base-pointers-derived-array.cpp`, `tests/expressions/additive/base-pointer-one-past-subobject.cpp`, `tests/expressions/additive/base-pointer-beyond-subobject.cpp`

## Context

A `B*` can point to the `B` subobject of an element of an array of `D`,
where `D` derives from `B`. Arithmetic on the `B*` counts in units of
`B`, and the array elements are `D` objects. During constant
evaluation the compilers give these results:

| Case | Clang 22, 18 | EDG 7.0 | GCC 16, 14 |
|---|---|---|---|
| `p + 1`, with no read | accepts | accepts | accepts |
| `(p + 1) - p` | accepts, `1` | accepts, `1` | accepts, `1` |
| read through `p + 1` | rejects, "past the end of object" | rejects | accepts |
| `p + 2`, with no read | rejects, "element 2 of non-array object" | rejects, "invalid arithmetic on non-array pointer" | accepts |
| `B*` to `arr[1]` minus `B*` to `arr[0]` | rejects, "not elements of the same array" | rejects | accepts |

When `D` adds a member, GCC gives `2` for the last difference, which is
the distance in bytes divided by the size of `B`. When `D` adds no
member, GCC gives `1`.

Clang and EDG treat the `B` subobject as a single object, which counts
as an array of one element (ADD-6). Every implementation accepts the
position one past that object. GCC computes with addresses.

## Evidence

The tests in Affects, and probes of the cases in the table on every
compiler in the matrix. GCC 16 behaves as GCC 14.

## Options

1. Undefined: the element type of the pointer must match the array's
   element type for any arithmetic. Every implementation then misses a
   diagnostic for `p + 1`.
2. The subobject is an array of one element. Moving the pointer outside
   the positions `p` and `p + 1`, a read through `p + 1`, and a
   difference of pointers into different elements are undefined. GCC
   misses these diagnostics.
3. Only a read through the pointer is undefined.

## Provisional answer

Option 2. Clang and EDG agree on every case, and every implementation
accepts `p + 1` without a read. ADD-14 states option 2.

## Opinions

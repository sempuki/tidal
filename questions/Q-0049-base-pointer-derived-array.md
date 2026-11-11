# Q-0049: Is arithmetic on a base class pointer into an array of a derived class undefined?

**Status:** settled
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

## Answer

The subobject acts as an array of one element, so moving the pointer outside `p` and `p + 1`, a read through `p + 1`, and a difference of pointers into different elements are undefined (option 2). Clang and EDG agree on every case, and every implementation
accepts `p + 1` without a read. ADD-14 states option 2.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and ADD-P6 gives the same answer: a pointer of another type, such as a base class pointer into an array of a derived class, sees only its own object, as an array of one. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

Kept settled on 2026-11-11 (decision 0062, item RV7). A review of 2026-11-11 reads ADD-14's array of one against the reference, and every compiler accepts `p + 1` with no read, in constant evaluation too. Tidal keeps the answer, and `pending.md` row 4 holds the case as the item "ADD-14, ADD-P6".

Kept settled on 2026-11-11 (decision 0063, item RV8). The read of the RV7 repair diff finds that the reference agrees with this answer (RV8-R-9). A pointer to a base class subobject counts the subobject as the one element of an array of the pointer's own type, so the move to one past is defined and a move of two is undefined. The note of 2026-11-11 under item RV7 does not hold. The `pending.md` row 4 item that named this question now holds the case of a pointer to the matching signed or unsigned integer type (Q-0442), and Q-0430 holds the byte case.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 2. For pointer arithmetic, a lone object counts as the single element of a one-element array. Adding anything other than zero or one to a pointer to a base subobject is undefined. A difference needs two elements of one array, and the two base subobjects are not elements of one array. The position one past the end points to no object, so a read through it is undefined.

### Gap review, 2026-11-11 (phase 14 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 14 wrote on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`) names the question in its reading of ADD-14 (RV7-B-3). Addition and subtraction on a pointer are undefined when the pointed-to type and the element type of the array are not similar. The review reads ADD-14's sentence that counts the object as an array of one for a pointer of another type against that rule, because the sentence allows the move to one past. That reading matches option 1 of the question. It differs from the settled answer and from the gap review of 2026-11-03.

### Read of the RV7 repair diff, 2026-11-11

The source-reader read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`) reads the RV7 repair of the additive page. RV7-B-3's reading is too wide for this question's case: the move one past a base class subobject agrees with the reference, so ADD-14 and this answer agree with it, and the row 4 item "ADD-14, ADD-P6" records no disagreement there (RV8-R-9). The disagreement that remains is in the byte case (Q-0430) and in a signed or unsigned integer of the same width.

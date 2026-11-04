# Q-0355: Does EDG calling a base's function for a virtual call made inside an intermediate class during constant evaluation change VIRT-1?

**Status:** provisional
**Affects:** VIRT-1, CE-1, `tests/programs/W9/visitor.cpp`

## Context

Class `V` declares a virtual function `f`. Class `W` derives from `V`
and overrides `f`. Class `C` derives from `W` and does not override
`f`, so `W::f` is the final overrider of `f` in `C`. An object of type
`C` is created during constant evaluation, and a virtual call of `f`
through a `V&` reaches `W::f`. Inside `W::f`, a second virtual call of
`f` goes through `*this`, converted to `V&`.

| Compiler | The second call |
|---|---|
| Clang 18, Clang 22 | calls `W::f` |
| GCC 14, GCC 16 | calls `W::f` |
| EDG 7.0 | calls `V::f`. When `V::f` is pure, EDG says that `V::f` is not defined and rejects the constant. |

EDG calls `W::f` when the object's type is `W` itself. Clang and GCC
give the same result at run time.

## Evidence

`tests/programs/W9/visitor.cpp`, the W9 Write program, with an override
for EDG. In that program a walker class visits a tree and a class
derived from the walker counts its leaves. Probes by the clean-room
writer on the matrix containers gave the table above.

## Options

1. A virtual call during constant evaluation calls the final overrider
   for the complete object (VIRT-1, CE-1). EDG's result is a defect.
2. A virtual call made from a member function of an intermediate class
   uses that class's view of the object.

## Provisional answer

No; the call reaches `W::f`, and EDG differs (option 1). VIRT-1 resolves
a virtual call by the final overrider for the class of the complete
object, and constant evaluation runs the same program as run time
(CE-P1). Clang and GCC agree at run time and in constant evaluation.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
